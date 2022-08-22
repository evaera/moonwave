use crate::{diagnostic::Diagnostics, doc_comment::DocComment, doc_entry::DocEntry, error::Error};
use full_moon::{
    self,
    ast::{LastStmt, Stmt},
    node::Node,
    tokenizer::{Token, TokenReference, TokenType},
    visitors::Visitor,
};

#[derive(Debug)]
pub struct SourceFile {
    doc_comments: Vec<DocComment>,
    file_id: usize,
}

impl<'a> SourceFile {
    pub fn from_str(source: &'a str, file_id: usize, relative_path: String) -> Result<Self, Error> {
        let ast = full_moon::parse(source).map_err(|e| Error::FullMoonError(e.to_string()))?;

        struct Collector<'b> {
            buffer: Vec<(Token, Option<Stmt>)>,
            last_line: usize,
            file_id: usize,
            relative_path: &'b str,
            doc_comments: Vec<DocComment>,
        }

        impl<'b> Collector<'b> {
            fn new(file_id: usize, relative_path: &'b str) -> Self {
                Self {
                    buffer: Vec::new(),
                    file_id,
                    last_line: 0,
                    relative_path,
                    doc_comments: Vec::new(),
                }
            }

            fn scan(&mut self, token: Token, stmt: Option<Stmt>) {
                match token.token_type() {
                    TokenType::MultiLineComment { blocks: 1, comment } => {
                        self.last_line = token.end_position().line();
                        self.clear();

                        self.doc_comments.push(DocComment::new(
                            comment.to_string(),
                            token.start_position().bytes() + "--[=[".len(),
                            token.end_position().line() + 1,
                            self.file_id,
                            self.relative_path.to_owned(),
                            stmt,
                        ));
                    }
                    TokenType::SingleLineComment { comment } => {
                        self.last_line = token.start_position().line();

                        if let Some(comment) = comment.strip_prefix('-') {
                            if comment.trim().chars().all(|char| char == '-') {
                                // Comment is all -------
                                return;
                            }

                            if comment.len() > 1 {
                                if let Some(first_non_whitespace) =
                                    comment.find(|char: char| !char.is_whitespace())
                                {
                                    // Compatibility: Drop lines like `---@module <path>` used
                                    // for Roblox LSP comments (#39)
                                    let tag_body = &comment[first_non_whitespace..];

                                    if tag_body.starts_with("@module") {
                                        return;
                                    }
                                }
                            }

                            self.buffer.push((token, stmt));
                        } else if let Some(doc_comment) = self.flush() {
                            self.doc_comments.push(doc_comment);
                        }
                    }
                    TokenType::Whitespace { .. } => {
                        let line = token.start_position().line();
                        let is_consecutive_newline = line > self.last_line;

                        self.last_line = line;

                        if is_consecutive_newline {
                            if let Some(doc_comment) = self.flush() {
                                self.doc_comments.push(doc_comment);
                            }
                        }
                    }
                    _ => {}
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
                    .map(|(token, _)| match token.token_type() {
                        TokenType::SingleLineComment { comment } => {
                            format!("--{}", comment)
                        }
                        _ => unreachable!(),
                    })
                    .collect::<Vec<_>>()
                    .join("\n");

                let doc_comment = Some(DocComment::new(
                    comment,
                    self.buffer.first().unwrap().0.start_position().bytes(),
                    self.buffer.last().unwrap().0.end_position().line() + 1,
                    self.file_id,
                    self.relative_path.to_owned(),
                    self.buffer.last().unwrap().1.as_ref().cloned(),
                ));

                self.clear();

                doc_comment
            }

            fn finish(mut self) -> Vec<DocComment> {
                if let Some(doc_comment) = self.flush() {
                    self.doc_comments.push(doc_comment);
                }

                self.doc_comments
            }
        }

        impl Visitor for Collector<'_> {
            fn visit_stmt(&mut self, stmt: &Stmt) {
                let surrounding_trivia = stmt.surrounding_trivia().0;
                for trivia in surrounding_trivia {
                    self.scan(trivia.clone(), Some(stmt.clone()));
                }
            }

            fn visit_last_stmt(&mut self, stmt: &LastStmt) {
                let stmt = stmt.clone();
                let surrounding_trivia = stmt.surrounding_trivia().0;
                for trivia in surrounding_trivia {
                    self.scan(trivia.clone(), None);
                }
            }

            fn visit_eof(&mut self, stmt: &TokenReference) {
                let surrounding_trivia = stmt.surrounding_trivia().0;
                for trivia in surrounding_trivia {
                    self.scan(trivia.clone(), None);
                }
            }
        }

        let mut collector = Collector::new(file_id, &relative_path);

        collector.visit_ast(&ast);

        let doc_comments = collector.finish();

        Ok(Self {
            doc_comments,
            file_id,
        })
    }

    pub fn parse(&'a self) -> Result<Vec<DocEntry>, Error> {
        let (doc_entries, errors): (Vec<_>, Vec<_>) = self
            .doc_comments
            .iter()
            .map(DocEntry::parse)
            .partition(Result::is_ok);
        let doc_entries: Vec<_> = doc_entries.into_iter().map(Result::unwrap).collect();
        let errors: Diagnostics = errors
            .into_iter()
            .map(Result::unwrap_err)
            .flat_map(Diagnostics::into_iter)
            .collect::<Vec<_>>()
            .into();

        if errors.is_empty() {
            Ok(doc_entries)
        } else {
            Err(Error::ParseErrors(errors))
        }
    }
}
