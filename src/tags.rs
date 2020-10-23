use crate::parse_error::ParseError;
use regex::Regex;

#[derive(Debug)]
pub enum TagType {
    Param {
        name: String,
        desc: Option<String>,
        lua_type: Option<String>,
    },
}

impl TagType {
    pub fn parse(text: &str) -> Result<Self, ParseError> {
        let first_space = text.find(' ');

        let mut tag_name = match first_space {
            Some(index) => &text[..index],
            None => text,
        };

        let tag_text = match first_space {
            Some(index) => &text[index + 1..],
            None => "",
        };

        if tag_name.starts_with('@') {
            tag_name = &tag_name[1..]
        }

        match tag_name {
            "param" => {
                let mut double_hyphen_split = tag_text.split("--");
                let first_half = double_hyphen_split
                    .next()
                    .ok_or_else(|| ParseError::new("No first half"))?;
                let desc = double_hyphen_split.next().map(|str| str.trim().to_owned());

                // TODO: Switch to regex with named capture because this is awful
                let first_half_space = first_half.find(' ');
                let name = first_half_space
                    .map(|i| &first_half[..i])
                    .unwrap_or(first_half)
                    .trim()
                    .to_owned();

                let lua_type = first_half_space
                    .map(|i| &first_half[i..])
                    .map(|str| str.trim().to_owned());

                Ok(Self::Param {
                    name,
                    desc,
                    lua_type,
                })
            }
            _ => Err(ParseError::new(format!("Invalid tag: {}", text))),
        }
    }
}
