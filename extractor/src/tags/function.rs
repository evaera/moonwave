use serde::Serialize;

use crate::{diagnostic::Diagnostic, span::Span};

#[derive(Debug, PartialEq, Serialize)]
pub struct FunctionTag<'a> {
    pub name: Span<'a>,
    #[serde(skip)]
    pub source: Span<'a>,
}

impl<'a> FunctionTag<'a> {
    pub fn parse(text: Span<'a>) -> Result<Self, Diagnostic> {
        Ok(Self {
            name: text,
            source: text,
        })
    }
}
