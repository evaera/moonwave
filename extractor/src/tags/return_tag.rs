use serde::Serialize;

use crate::{diagnostic::Diagnostic, span::Span};

#[derive(Debug, PartialEq, Serialize)]
pub struct ReturnTag<'a> {
    pub desc: Span<'a>,
    pub lua_type: Span<'a>,
    #[serde(skip)]
    pub source: Span<'a>,
}

impl<'a> ReturnTag<'a> {
    pub fn parse(span: Span<'a>) -> Result<Self, Diagnostic> {
        let mut pieces = span.splitn(2, "--");
        let lua_type: Span<'_> = pieces.next().unwrap().trim();

        if lua_type.is_empty() {
            return Err(span.diagnostic("Return type is required"));
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
mod test {
    use insta::assert_yaml_snapshot;

    use super::*;

    #[test]
    fn everything_sandwich() {
        let source = Span::dummy("foo -- HEY! This is a sweet description");

        let value = ReturnTag::parse(source).unwrap();
        assert_yaml_snapshot!(value, @r###"
        ---
        desc: HEY! This is a sweet description
        lua_type: foo
        "###);
    }

    #[test]
    fn lovecraftian_type() {
        let source = Span::dummy("Roact.Element<{ oh_no: string -> coroutine }> -- I'm sorry.");
        let value = ReturnTag::parse(source).unwrap();
        assert_yaml_snapshot!(value, @r###"
        ---
        desc: "I'm sorry."
        lua_type: "Roact.Element<{ oh_no: string -> coroutine }>"
        "###);
    }

    #[test]
    fn no_description() {
        let source = Span::dummy("tasty");
        let value = ReturnTag::parse(source).unwrap();
        assert_yaml_snapshot!(value, @r###"
        ---
        desc: ""
        lua_type: tasty
        "###);
    }

    #[test]
    fn just_description() {
        let source = Span::dummy("-- What am I even documenting here?");
        let value = ReturnTag::parse(source);
        assert_yaml_snapshot!(value, @r###"
        ---
        Err:
          text: Return type is required
          start: 0
          len: 35
          file_id: 0
          additional_diagnostics: []
        "###);
    }

    #[test]
    fn snapshot() {
        assert_yaml_snapshot!(ReturnTag::parse(Span::dummy("coffee")), @r###"
        ---
        Ok:
          desc: ""
          lua_type: coffee
        "###)
    }
}
