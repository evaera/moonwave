use serde::Serialize;

use crate::{diagnostic::Diagnostic, doc_entry::FunctionType, span::Span};

#[derive(Debug, PartialEq, Serialize)]
pub struct FunctionTag<'a> {
    pub name: Span<'a>,
    pub function_type: FunctionType,
    #[serde(skip)]
    pub source: Span<'a>,
}

impl<'a> FunctionTag<'a> {
    pub fn parse(text: Span<'a>, function_type: FunctionType) -> Result<Self, Diagnostic> {
        Ok(Self {
            name: text,
            source: text,
            function_type,
        })
    }
}
