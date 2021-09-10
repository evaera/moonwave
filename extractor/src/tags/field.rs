use serde::Serialize;

use crate::{diagnostic::Diagnostic, span::Span};

#[derive(Debug, PartialEq, Serialize)]
pub struct FieldTag<'a> {
    pub name: Span<'a>,
    pub desc: Span<'a>,
    pub lua_type: Span<'a>,
    #[serde(skip)]
    pub source: Span<'a>,
}

impl<'a> FieldTag<'a> {
    pub fn parse(span: Span<'a>) -> Result<Self, Diagnostic> {
        let mut pieces = span.splitn(2, "--");
        let name_and_maybe_type: Span<'_> = pieces.next().unwrap().trim();
        let desc = pieces
            .next()
            .map(|desc| desc.trim())
            .unwrap_or_else(|| Span::empty(span.file_id));

        let mut pieces = name_and_maybe_type.splitn(2, " ");
        let name = pieces.next().unwrap().trim();

        if name.is_empty() {
            return Err(span.diagnostic("Field name is required"));
        }

        let lua_type = pieces
            .next()
            .map(|name| name.trim())
            .ok_or_else(|| span.diagnostic("Field type is required"))?;

        Ok(Self {
            name,
            desc,
            lua_type,
            source: span,
        })
    }
}
