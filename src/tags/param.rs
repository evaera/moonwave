use std::convert::TryFrom;

use crate::{diagnostic::Diagnostic, span::Span};

// TODO: Instead of Option<String>, just give empty string
#[derive(Debug, PartialEq)]
pub struct ParamTag<'a> {
    pub name: Span<'a>,
    pub desc: Option<Span<'a>>,
    pub lua_type: Option<Span<'a>>,
    pub source: Span<'a>,
}

impl<'a> TryFrom<Span<'a>> for ParamTag<'a> {
    type Error = Diagnostic;

    fn try_from(span: Span<'a>) -> Result<Self, Self::Error> {
        let mut pieces = span.splitn(2, "--");
        let name_and_maybe_type: Span<'a> = pieces.next().unwrap().trim();
        let desc = pieces.next().map(|desc| desc.trim());

        let mut pieces = name_and_maybe_type.splitn(2, " ");
        let name = pieces.next().unwrap().trim();
        let lua_type = pieces.next().map(|name| name.trim());

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
    use super::*;

    #[test]
    fn everything_sandwich() {
        let value = ParamTag::try_from(Span::from_source(
            "@param COOL_NAME foo -- HEY! This is a sweet description",
            0,
        ))
        .unwrap();

        assert_eq!(value.name.as_str(), "COOL_NAME");
        assert_eq!(value.lua_type.map(|t| t.as_str()), Some("foo"));
        assert_eq!(
            value.desc.map(|t| t.as_str()),
            Some("HEY! This is a sweet description")
        );
    }

    // #[test]
    // fn lovecraftian_type() {
    //     let value = str::parse::<Tag>(
    //         "@param foo Roact.Element<{ ohno: string -> coroutine }> -- I'm sorry.",
    //     )
    //     .unwrap();

    //     assert_eq!(
    //         value,
    //         Tag::Param(ParamTag {
    //             name: String::from("foo"),
    //             lua_type: Some(String::from("Roact.Element<{ ohno: string -> coroutine }>")),
    //             desc: Some(String::from("I'm sorry.")),
    //         })
    //     );
    // }

    // #[test]
    // fn no_type() {
    //     let value = str::parse::<Tag>("@param coffee -- Ever heard of FlowJS?").unwrap();

    //     assert_eq!(
    //         value,
    //         Tag::Param(ParamTag {
    //             name: String::from("coffee"),
    //             lua_type: None,
    //             desc: Some(String::from("Ever heard of FlowJS?")),
    //         })
    //     );
    // }

    // #[test]
    // fn no_description() {
    //     let value = str::parse::<Tag>("@param coffee tasty").unwrap();

    //     assert_eq!(
    //         value,
    //         Tag::Param(ParamTag {
    //             name: String::from("coffee"),
    //             lua_type: Some(String::from("tasty")),
    //             desc: None,
    //         })
    //     );
    // }

    // #[test]
    // fn no_description_nor_type() {
    //     let value = str::parse::<Tag>("@param coffee").unwrap();

    //     assert_eq!(
    //         value,
    //         Tag::Param(ParamTag {
    //             name: String::from("coffee"),
    //             lua_type: None,
    //             desc: None,
    //         })
    //     );
    // }
}
