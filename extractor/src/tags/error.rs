use serde::Serialize;

use crate::{diagnostic::Diagnostic, span::Span};

#[derive(Debug, PartialEq, Serialize)]
pub struct ErrorTag<'a> {
    pub lua_type: Span<'a>,
    pub desc: Span<'a>,
    #[serde(skip)]
    pub source: Span<'a>,
}

impl<'a> ErrorTag<'a> {
    pub fn parse(span: Span<'a>) -> Result<Self, Diagnostic> {
        let mut pieces = span.splitn(2, "--");
        let lua_type: Span<'_> = pieces.next().unwrap().trim();

        if lua_type.is_empty() {
            return Err(span.diagnostic("Error type is required"));
        }

        let desc = pieces
            .next()
            .map(|desc| desc.trim())
            .unwrap_or_else(|| Span::empty(span.file_id));

        Ok(Self {
            desc,
            lua_type,
            source: span,
        })
    }
}

#[cfg(test)]
mod tests {
    use insta::assert_yaml_snapshot;

    use super::*;

    #[test]
    fn error_tag() {
        let source = Span::dummy(r#""very bad error" -- Very bad error "#);

        let value = ErrorTag::parse(source).unwrap();

        assert_yaml_snapshot!(value, @r###"
        ---
        lua_type: "\"very bad error\""
        desc: Very bad error
        "###);
    }

    #[test]
    fn no_desc() {
        let source = Span::dummy(r#""very bad error" "#);
        let value = ErrorTag::parse(source).unwrap();

        assert_yaml_snapshot!(value, @r###"
        ---
        lua_type: "\"very bad error\""
        desc: ""
        "###);
    }
    #[test]
    fn nothing() {
        let source = Span::dummy("");

        let value = ErrorTag::parse(source);

        assert_yaml_snapshot!(value, @r###"
        ---
        Err:
          text: Error type is required
          start: 0
          len: 0
          file_id: 0
          additional_diagnostics: []
        "###);
    }
}
