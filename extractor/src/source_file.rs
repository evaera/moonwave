use std::borrow::Cow;

use crate::{diagnostic::Diagnostics, doc_comment::DocComment, doc_entry::DocEntry, error::Error};
use full_moon::{
    self,
    ast::Stmt,
    node::Node,
    tokenizer::{Token, TokenType},
};

#[derive(Debug)]
pub struct SourceFile<'a> {
    doc_comments: Vec<(DocComment, Option<Stmt<'a>>)>,
    file_id: usize,
}

impl<'a> SourceFile<'a> {
    pub fn from_str(source: &'a str, file_id: usize, relative_path: String) -> Result<Self, Error> {
        let ast = full_moon::parse(source).map_err(|e| Error::FullMoonError(e.to_string()))?;

        struct Collector<'a, 'b> {
            buffer: Vec<Token<'a>>,
            last_line: usize,
            file_id: usize,
            relative_path: &'b str,
        }

        impl<'a, 'b> Collector<'a, 'b> {
            fn new(file_id: usize, relative_path: &'b str) -> Self {
                Self {
                    buffer: Vec::new(),
                    file_id,
                    last_line: 0,
                    relative_path,
                }
            }

            fn scan(&mut self, token: Cow<Token<'a>>) -> Option<DocComment> {
                match token.token_type() {
                    TokenType::MultiLineComment { blocks: 1, comment } => {
                        self.clear();

                        Some(DocComment::new(
                            comment.to_string(),
                            token.start_position().unwrap().bytes() + "--[=[".len(),
                            token.end_position().unwrap().line() + 1,
                            self.file_id,
                            self.relative_path.to_owned(),
                        ))
                    }
                    TokenType::SingleLineComment { comment } => {
                        if let Some(comment) = comment.strip_prefix('-') {
                            if comment.len() > 1 {
                                if let Some(first_non_whitespace) =
                                    comment.find(|char: char| !char.is_whitespace())
                                {
                                    // Compatibility: Drop lines like `---@module <path>` used
                                    // for Roblox LSP comments (#39)
                                    let tag_body = &comment[first_non_whitespace..];

                                    if tag_body.starts_with("@module") {
                                        return None;
                                    }
                                }
                            }

                            self.buffer.push(token.into_owned());
                        } else {
                            return self.flush();
                        }

                        None
                    }
                    TokenType::Whitespace { .. } => {
                        if token.start_position().unwrap().line() == self.last_line {
                            return self.flush();
                        }

                        self.last_line = token.start_position().unwrap().line();

                        None
                    }
                    _ => None,
                }
            }

            fn clear(&mut self) {
                self.buffer.clear();
            }

            fn flush(&mut self) -> Option<DocComment> {
                if self.buffer.is_empty() {
                    return None;
                }

                let comment = self
                    .buffer
                    .iter()
                    .map(|token| match token.token_type() {
                        TokenType::SingleLineComment { comment } => {
                            format!("--{}", comment)
                        }
                        _ => unreachable!(),
                    })
                    .collect::<Vec<_>>()
                    .join("\n");

                let doc_comment = Some(DocComment::new(
                    comment,
                    self.buffer.first().unwrap().start_position().bytes(),
                    self.buffer.last().unwrap().end_position().line() + 1,
                    self.file_id,
                    self.relative_path.to_owned(),
                ));

                self.clear();

                doc_comment
            }
        }

        let mut collector = Collector::new(file_id, &relative_path);

        let mut doc_comments: Vec<_> = ast
            .nodes()
            .iter_stmts()
            .map(|stmt| {
                let mut comments = stmt
                    .surrounding_trivia()
                    .0
                    .into_iter()
                    .filter_map(|token| {
                        collector
                            .scan(token)
                            .map(|comment| (comment, Some(stmt.clone())))
                    })
                    .collect::<Vec<_>>();

                if let Some(doc_comment) = collector.flush() {
                    comments.push((doc_comment, Some(stmt.clone())))
                }

                comments
            })
            .flatten()
            .collect();

        let mut collector = Collector::new(file_id, &relative_path);

        doc_comments.extend(
            ast.eof()
                .surrounding_trivia()
                .0
                .into_iter()
                .filter_map(|token| collector.scan(token).map(|comment| (comment, None))),
        );

        if let Some(doc_comment) = collector.flush() {
            doc_comments.push((doc_comment, None))
        }

        Ok(Self {
            doc_comments,
            file_id,
        })
    }

    pub fn parse(&'a self) -> Result<Vec<DocEntry>, Error> {
        let doc_entries: Vec<Result<DocEntry, Diagnostics>> = self
            .doc_comments
            .iter()
            .map(|c| DocEntry::parse(&c.0, c.1.as_ref()))
            .collect();

        let (doc_entries, errors): (Vec<_>, Vec<_>) =
            doc_entries.into_iter().partition(Result::is_ok);
        let doc_entries: Vec<_> = doc_entries.into_iter().map(Result::unwrap).collect();
        let errors: Diagnostics = errors
            .into_iter()
            .map(Result::unwrap_err)
            .map(Diagnostics::into_iter)
            .flatten()
            .collect::<Vec<_>>()
            .into();

        if errors.is_empty() {
            Ok(doc_entries)
        } else {
            Err(Error::ParseErrors(errors))
        }
    }
}
