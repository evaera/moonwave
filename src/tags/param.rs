use serde::Serialize;

use crate::{diagnostic::Diagnostic, span::Span};

// TODO: Instead of Option<String>, just give empty string
#[derive(Debug, PartialEq, Serialize)]
pub struct ParamTag<'a> {
    pub name: Span<'a>,
    pub desc: Option<Span<'a>>,
    pub lua_type: Span<'a>,
    #[serde(skip)]
    pub source: Span<'a>,
}

impl<'a> ParamTag<'a> {
    pub fn parse(span: Span<'a>) -> Result<Self, Diagnostic> {
        let mut pieces = span.splitn(2, "--");
        let name_and_maybe_type: Span<'_> = pieces.next().unwrap().trim();
        let desc = pieces.next().map(|desc| desc.trim());

        let mut pieces = name_and_maybe_type.splitn(2, " ");
        let name = pieces.next().unwrap().trim();
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
        assert_yaml_snapshot!(value);

        assert_eq!(
            value,
            ParamTag {
                name: Span::dummy("COOL_NAME"),
                desc: Some(Span::dummy("HEY! This is a sweet description")),
                lua_type: Span::dummy("foo"),
                source
            }
        )
    }

    #[test]
    fn lovecraftian_type() {
        let source = Span::dummy("foo Roact.Element<{ oh_no: string -> coroutine }> -- I'm sorry.");
        let value = ParamTag::parse(source).unwrap();
        assert_yaml_snapshot!(value);

        assert_eq!(
            value,
            ParamTag {
                name: Span::dummy("foo"),
                lua_type: Span::dummy("Roact.Element<{ oh_no: string -> coroutine }>"),
                desc: Some(Span::dummy("I'm sorry.")),
                source,
            }
        );
    }

    #[test]
    fn no_type() {
        let source = Span::dummy("coffee -- Ever heard of FlowJS?");
        let value = ParamTag::parse(source);
        assert_yaml_snapshot!(value);

        assert!(value.is_err());
        assert_eq!(value.unwrap_err().text, "Param type is required");
    }

    #[test]
    fn no_description() {
        let source = Span::dummy("coffee tasty");
        let value = ParamTag::parse(source).unwrap();
        assert_yaml_snapshot!(value);

        assert_eq!(
            value,
            ParamTag {
                name: Span::dummy("coffee"),
                lua_type: Span::dummy("tasty"),
                desc: None,
                source,
            }
        );
    }

    #[test]
    fn no_description_nor_type() {
        let source = Span::dummy("coffee");
        let value = ParamTag::parse(source);
        assert_yaml_snapshot!(value);

        assert!(value.is_err());
        assert_eq!(value.unwrap_err().text, "Param type is required");
    }

    #[test]
    fn snapshot() {
        assert_yaml_snapshot!(ParamTag::parse(Span::dummy("coffee")))
    }
}
