use serde::Serialize;

use crate::{diagnostic::Diagnostic, span::Span};

#[derive(Debug, PartialEq, Serialize)]
pub struct IndexTag<'a> {
    pub name: Span<'a>,
    #[serde(skip)]
    pub source: Span<'a>,
}

impl<'a> IndexTag<'a> {
    pub fn parse(span: Span<'a>) -> Result<Self, Diagnostic> {
        Ok(Self {
            name: span,
            source: span,
        })
    }
}
