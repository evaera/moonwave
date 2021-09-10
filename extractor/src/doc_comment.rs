use std::borrow::Cow;

use full_moon::tokenizer::{Token, TokenType};
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
}

impl DocComment {
    pub fn new(token: Cow<Token>, file_id: usize, relative_path: String) -> Self {
        match token.token_type() {
            TokenType::MultiLineComment { comment, .. } => Self {
                comment: comment.to_string(),
                file_id,
                start: token.start_position().bytes() + "--[=[".len(),
                output_source: OutputSource {
                    line: token.end_position().line() + 1,
                    relative_path,
                },
            },
            _ => unreachable!(),
        }
    }

    pub fn diagnostic<S: Into<String>>(&self, text: S) -> Diagnostic {
        Diagnostic::from_doc_comment(text, self)
    }
}
