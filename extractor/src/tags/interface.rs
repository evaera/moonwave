use serde::Serialize;

use crate::{diagnostic::Diagnostic, span::Span};

#[derive(Debug, PartialEq, Serialize)]
pub struct InterfaceTag<'a> {
    pub name: Span<'a>,
    #[serde(skip)]
    pub source: Span<'a>,
}

impl<'a> InterfaceTag<'a> {
    pub fn parse(text: Span<'a>) -> Result<Self, Diagnostic> {
        Ok(Self {
            name: text,
            source: text,
        })
    }
}
