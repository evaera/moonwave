use serde::Serialize;

use crate::{diagnostic::Diagnostic, span::Span};

#[derive(Debug, PartialEq, Serialize)]
pub struct TypeTag<'a> {
    pub name: Span<'a>,
    pub lua_type: Span<'a>,
    #[serde(skip)]
    pub source: Span<'a>,
}

impl<'a> TypeTag<'a> {
    pub fn parse(span: Span<'a>) -> Result<Self, Diagnostic> {
        let mut pieces = span.splitn(2, " ");
        let name = pieces.next().unwrap().trim();

        let lua_type = pieces
            .next()
            .map(Span::trim)
            .ok_or_else(|| span.diagnostic("Property type is required"))?;

        Ok(Self {
            name,
            lua_type,
            source: span,
        })
    }
}
