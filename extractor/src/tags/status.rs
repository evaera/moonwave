use serde::Serialize;

use crate::{diagnostic::Diagnostic, span::Span};

#[derive(Debug, PartialEq, Serialize)]
pub struct DeprecatedTag<'a> {
    pub version: Option<Span<'a>>,
    pub desc: Option<Span<'a>>,
    #[serde(skip)]
    pub source: Span<'a>,
}

impl<'a> DeprecatedTag<'a> {
    pub fn parse(span: Span<'a>) -> Result<Self, Diagnostic> {
        let mut pieces = span.splitn(2, "--");
        let version: Span<'_> = pieces.next().unwrap().trim();

        let version = if version.is_empty() {
            None
        } else {
            Some(version)
        };

        let desc = pieces.next().map(|span| span.trim());

        Ok(Self {
            version,
            desc,
            source: span,
        })
    }
}

#[derive(Debug, PartialEq, Serialize)]
pub struct SinceTag<'a> {
    pub version: Span<'a>,
    #[serde(skip)]
    pub source: Span<'a>,
}

impl<'a> SinceTag<'a> {
    pub fn parse(span: Span<'a>) -> Result<Self, Diagnostic> {
        Ok(Self {
            version: span,
            source: span,
        })
    }
}

#[cfg(test)]
mod tests {
    use insta::assert_yaml_snapshot;

    use super::*;

    #[test]
    fn deprecated() {
        let source =
            Span::dummy("v5.3 -- This is very deprecated. Never use this. Keep scrollin'.");

        let value = DeprecatedTag::parse(source);

        assert_yaml_snapshot!(value, @r###"
        ---
        Ok:
          version: v5.3
          desc: "This is very deprecated. Never use this. Keep scrollin'."
        "###);
    }

    #[test]
    fn since() {
        let source = Span::dummy("v5.14");

        let value = SinceTag::parse(source);

        assert_yaml_snapshot!(value, @r###"
        ---
        Ok:
          version: v5.14
        "###);
    }
}
