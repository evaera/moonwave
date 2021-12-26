use full_moon::ast::Stmt;
use serde::{Deserialize, Serialize};

use crate::diagnostic::Diagnostic;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct OutputSource {
    pub line: usize,

    #[serde(rename = "path")]
    pub relative_path: String,
}

/// A class for representing an unparsed doc comment from Lua.
#[derive(Debug, PartialEq)]
pub struct DocComment {
    pub comment: String,
    pub file_id: usize,
    pub start: usize,
    pub output_source: OutputSource,
    pub stmt: Option<Stmt>,
}

impl DocComment {
    pub fn new(
        comment: String,
        start_position: usize,
        target_line: usize,
        file_id: usize,
        relative_path: String,
        stmt: Option<Stmt>,
    ) -> Self {
        Self {
            comment,
            file_id,
            start: start_position,
            output_source: OutputSource {
                line: target_line,
                relative_path,
            },
            stmt,
        }
    }

    pub fn diagnostic<S: Into<String>>(&self, text: S) -> Diagnostic {
        Diagnostic::from_doc_comment(text, self)
    }
}
