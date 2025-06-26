use serde::Serialize;

use crate::{diagnostic::Diagnostic, span::Span};

#[derive(Debug, PartialEq, Serialize, Clone)]
pub struct IncludeTag<'a> {
	pub path: Span<'a>,
	#[serde(skip)]
	pub source: Span<'a>,
}

impl<'a> IncludeTag<'a> {
	pub fn parse(span: Span<'a>) -> Result<Self, Diagnostic> {
		if span.is_empty() {
			return Err(span.diagnostic("This tag has stuff after it"));
		}

		Ok(Self {
			path: span,
			source: span,
		})
	}
}