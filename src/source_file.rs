use crate::{diagnostic::Diagnostics, doc_comment::DocComment, doc_entry::DocEntry, error::Error};
use full_moon::{self, ast::Stmt, node::Node, tokenizer::TokenType};

#[derive(Debug)]
pub struct SourceFile<'a> {
    doc_comments: Vec<(DocComment, Option<Stmt<'a>>)>,
    file_id: usize,
}

impl<'a> SourceFile<'a> {
    pub fn from_str(source: &'a str, file_id: usize, relative_path: String) -> Result<Self, Error> {
        let ast = full_moon::parse(source).map_err(|e| Error::FullMoonError(e.to_string()))?;

        let mut doc_comments: Vec<_> = ast
            .nodes()
            .iter_stmts()
            .map(|stmt| {
                stmt.surrounding_trivia()
                    .0
                    .into_iter()
                    .filter_map(|token| match token.token_type() {
                        TokenType::MultiLineComment { blocks: 1, .. } => Some((
                            DocComment::new(token, file_id, relative_path.clone()),
                            Some(stmt.clone()),
                        )),
                        _ => None,
                    })
                    .collect::<Vec<_>>()
            })
            .flatten()
            .collect();

        doc_comments.extend(
            ast.eof()
                .surrounding_trivia()
                .0
                .into_iter()
                .filter_map(|token| match token.token_type() {
                    TokenType::MultiLineComment { blocks: 1, .. } => {
                        Some((DocComment::new(token, file_id, relative_path.clone()), None))
                    }
                    _ => None,
                }),
        );

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
