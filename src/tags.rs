use crate::parse_error::ParseError;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub enum Tag {
    Param(ParamTag),
    Kind(KindTag),
    Within(WithinTag),
}

// TODO: Instead of Option<String>, just give empty string
#[derive(Debug, PartialEq)]
pub struct ParamTag {
    pub name: String,
    pub desc: Option<String>,
    pub lua_type: Option<String>,
}

impl FromStr for ParamTag {
    type Err = ParseError;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let mut pieces = text.splitn(2, "--");
        let name_and_maybe_type = pieces.next().unwrap().trim();
        let desc = pieces.next().map(|desc| desc.trim().to_owned());

        let mut pieces = name_and_maybe_type.splitn(2, ' ');
        let name = pieces.next().unwrap().trim().to_owned();
        let lua_type = pieces.next().map(|name| name.trim().to_owned());

        Ok(Self {
            name,
            desc,
            lua_type,
        })
    }
}

#[derive(Debug, PartialEq)]
pub struct WithinTag {
    pub name: String,
}

impl FromStr for WithinTag {
    type Err = ParseError;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        if text.is_empty() {
            return Err(ParseError::new("This tag has stuff after it"));
        }

        Ok(Self {
            name: text.to_owned(),
        })
    }
}

#[derive(Debug, PartialEq)]
pub enum KindTagType {
    Function,
    Property,
    Class,
}

#[derive(Debug, PartialEq)]
pub struct KindTag {
    pub name: String,
    pub tag_type: KindTagType,
}

impl KindTag {
    fn parse(text: &str, tag_type: KindTagType) -> Result<Self, ParseError> {
        if text.is_empty() {
            return Err(ParseError::new("This tag has stuff after it"));
        }

        Ok(Self {
            name: text.to_owned(),
            tag_type,
        })
    }
}

impl FromStr for Tag {
    type Err = ParseError;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let mut pieces = text.splitn(2, ' ');

        let tag_name = pieces.next().unwrap().trim();
        let tag_text = pieces.next().map(str::trim).unwrap_or("");

        match tag_name {
            "@param" => tag_text.parse::<ParamTag>().map(Tag::Param),
            "@within" => tag_text.parse::<WithinTag>().map(Tag::Within),
            "@prop" => KindTag::parse(tag_text, KindTagType::Property).map(Tag::Kind),
            "@class" => KindTag::parse(tag_text, KindTagType::Class).map(Tag::Kind),
            "@function" => KindTag::parse(tag_text, KindTagType::Function).map(Tag::Kind),
            _ => Err(ParseError::new("Unknown tag")),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn everything_sandwich() {
        let value =
            str::parse::<Tag>("@param COOL_NAME foo -- HEY! This is a sweet description").unwrap();

        assert_eq!(
            value,
            Tag::Param(ParamTag {
                name: String::from("COOL_NAME"),
                lua_type: Some(String::from("foo")),
                desc: Some(String::from("HEY! This is a sweet description")),
            })
        );
    }

    #[test]
    fn lovecraftian_type() {
        let value = str::parse::<Tag>(
            "@param foo Roact.Element<{ ohno: string -> coroutine }> -- I'm sorry.",
        )
        .unwrap();

        assert_eq!(
            value,
            Tag::Param(ParamTag {
                name: String::from("foo"),
                lua_type: Some(String::from("Roact.Element<{ ohno: string -> coroutine }>")),
                desc: Some(String::from("I'm sorry.")),
            })
        );
    }

    #[test]
    fn no_type() {
        let value = str::parse::<Tag>("@param coffee -- Ever heard of FlowJS?").unwrap();

        assert_eq!(
            value,
            Tag::Param(ParamTag {
                name: String::from("coffee"),
                lua_type: None,
                desc: Some(String::from("Ever heard of FlowJS?")),
            })
        );
    }

    #[test]
    fn no_description() {
        let value = str::parse::<Tag>("@param coffee tasty").unwrap();

        assert_eq!(
            value,
            Tag::Param(ParamTag {
                name: String::from("coffee"),
                lua_type: Some(String::from("tasty")),
                desc: None,
            })
        );
    }

    #[test]
    fn no_description_nor_type() {
        let value = str::parse::<Tag>("@param coffee").unwrap();

        assert_eq!(
            value,
            Tag::Param(ParamTag {
                name: String::from("coffee"),
                lua_type: None,
                desc: None,
            })
        );
    }
}
