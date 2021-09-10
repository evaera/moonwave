use serde::Serialize;

use crate::{diagnostic::Diagnostic, span::Span};

#[derive(Debug, PartialEq, Serialize)]
pub struct ParamTag<'a> {
    pub name: Span<'a>,
    pub desc: Span<'a>,
    pub lua_type: Span<'a>,
    #[serde(skip)]
    pub source: Span<'a>,
}

impl<'a> ParamTag<'a> {
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
            return Err(span.diagnostic("Param name is required"));
        }

        let lua_type = pieces
            .next()
            .map(|name| name.trim())
            .ok_or_else(|| span.diagnostic("Param type is required"))?;

        Ok(Self {
            name,
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
        let source = Span::dummy("COOL_NAME foo -- HEY! This is a sweet description");

        let value = ParamTag::parse(source).unwrap();
        assert_yaml_snapshot!(value, @r###"
        ---
        name: COOL_NAME
        desc: HEY! This is a sweet description
        lua_type: foo
        "###);
    }

    #[test]
    fn lovecraftian_type() {
        let source = Span::dummy("foo Roact.Element<{ oh_no: string -> coroutine }> -- I'm sorry.");
        let value = ParamTag::parse(source).unwrap();
        assert_yaml_snapshot!(value, @r###"
        ---
        name: foo
        desc: "I'm sorry."
        lua_type: "Roact.Element<{ oh_no: string -> coroutine }>"
        "###);
    }

    #[test]
    fn no_type() {
        let source = Span::dummy("coffee -- Ever heard of FlowJS?");
        let value = ParamTag::parse(source);
        assert_yaml_snapshot!(value, @r###"
        ---
        Err:
          text: Param type is required
          start: 0
          len: 31
          file_id: 0
          additional_diagnostics: []
        "###);
    }

    #[test]
    fn no_description() {
        let source = Span::dummy("coffee tasty");
        let value = ParamTag::parse(source).unwrap();
        assert_yaml_snapshot!(value, @r###"
        ---
        name: coffee
        desc: ""
        lua_type: tasty
        "###);
    }

    #[test]
    fn no_description_nor_type() {
        let source = Span::dummy("coffee");
        let value = ParamTag::parse(source);
        assert_yaml_snapshot!(value, @r###"
        ---
        Err:
          text: Param type is required
          start: 0
          len: 6
          file_id: 0
          additional_diagnostics: []
        "###);
    }

    #[test]
    fn empty_type() {
        let source = Span::dummy("a  -- HEY! This is a sweet description");

        let value = ParamTag::parse(source);
        assert_yaml_snapshot!(value, @r###"
        ---
        Err:
          text: Param type is required
          start: 0
          len: 38
          file_id: 0
          additional_diagnostics: []
        "###);
    }

    #[test]
    fn snapshot() {
        assert_yaml_snapshot!(ParamTag::parse(Span::dummy("coffee")), @r###"
        ---
        Err:
          text: Param type is required
          start: 0
          len: 6
          file_id: 0
          additional_diagnostics: []
        "###)
    }
}
