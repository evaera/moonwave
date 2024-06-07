use serde::Serialize;

use crate::{diagnostic::Diagnostic, span::Span};

#[derive(Debug, PartialEq, Serialize)]
pub struct ExternalTag<'a> {
    pub name: Span<'a>,
    pub url: Span<'a>,
    #[serde(skip)]
    pub source: Span<'a>,
}

impl<'a> ExternalTag<'a> {
    pub fn parse(span: Span<'a>) -> Result<Self, Diagnostic> {
        let mut pieces = span.splitn(2, " ");
        let name = pieces.next().unwrap().trim();

        let url = pieces
            .next()
            .map(Span::trim)
            .ok_or_else(|| span.diagnostic("A url is required"))?;

        Ok(Self {
            name,
            url,
            source: span,
        })
    }
}
