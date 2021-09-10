use serde::Serialize;

use crate::{diagnostic::Diagnostic, span::Span};

#[derive(Debug, PartialEq, Serialize)]
pub struct ClassTag<'a> {
    pub name: Span<'a>,
    #[serde(skip)]
    pub source: Span<'a>,
}

impl<'a> ClassTag<'a> {
    pub fn parse(text: Span<'a>) -> Result<Self, Diagnostic> {
        Ok(Self {
            name: text,
            source: text,
        })
    }
}
