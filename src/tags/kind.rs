use crate::{diagnostic::Diagnostic, span::Span};
use serde::Serialize;

use super::TagType;

#[derive(Debug, PartialEq, Serialize)]
pub enum KindTagType {
    Function,
    Property,
    Class,
    Type,
    #[allow(unused)]
    External,
}

impl KindTagType {
    pub fn tag_type(&self) -> TagType {
        match self {
            KindTagType::Function => TagType::Function,
            KindTagType::Property => TagType::Property,
            KindTagType::Class => TagType::Class,
            KindTagType::Type => TagType::Type,
            KindTagType::External => TagType::External,
        }
    }
}

#[derive(Debug, PartialEq, Serialize)]
pub struct KindTag<'a> {
    pub name: Span<'a>,
    pub kind_type: KindTagType,
    #[serde(skip)]
    pub source: Span<'a>,
}

impl<'a> KindTag<'a> {
    pub fn parse(text: Span<'a>, tag_type: KindTagType) -> Result<Self, Diagnostic> {
        if text.is_empty() {
            return Err(text.diagnostic("This kind tag has stuff after it"));
        }

        Ok(Self {
            name: text,
            kind_type: tag_type,
            source: text,
        })
    }
}

#[cfg(test)]
mod test {
    use insta::assert_yaml_snapshot;

    use super::*;

    #[test]
    fn snapshot() {
        assert_yaml_snapshot!(KindTag::parse(
            Span::dummy("hey there"),
            KindTagType::Function
        ));
        assert_yaml_snapshot!(KindTag::parse(
            Span::dummy("hey there"),
            KindTagType::Property
        ));
        assert_yaml_snapshot!(KindTag::parse(
            Span::dummy("This is a class"),
            KindTagType::Class
        ));

        assert_yaml_snapshot!(KindTag::parse(Span::dummy(""), KindTagType::Class));
    }
}
