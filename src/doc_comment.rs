use std::borrow::Cow;

use full_moon::{
    ast::Stmt,
    tokenizer::{Token, TokenType},
};

/// A class for representing an unparsed doc comment from Lua.
pub struct DocComment<'a> {
    pub comment: String,
    pub file_id: usize,
    pub bytes_start_position: usize,
    pub attached_stmt: &'a Stmt<'a>,
}

impl<'a> DocComment<'a> {
    pub fn new(token: Cow<Token>, stmt: &'a Stmt<'a>, file_id: usize) -> Self {
        match token.token_type() {
            TokenType::MultiLineComment { comment, .. } => Self {
                comment: comment.to_string(),
                file_id,
                bytes_start_position: token.start_position().bytes(),
                attached_stmt: stmt,
            },
            _ => unreachable!(),
        }
    }
}
