use std::borrow::Cow;

use full_moon::tokenizer::{Token, TokenType};

/// A class for representing an unparsed doc comment from Lua.
#[derive(Debug)]
pub struct DocComment {
    pub comment: String,
    pub file_id: usize,
    pub start: usize,
}

impl DocComment {
    pub fn new(token: Cow<Token>, file_id: usize) -> Self {
        match token.token_type() {
            TokenType::MultiLineComment { comment, .. } => Self {
                comment: comment.to_string(),
                file_id,
                start: token.start_position().bytes() + 5, // 5, because --[=[
            },
            _ => unreachable!(),
        }
    }
}
