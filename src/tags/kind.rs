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
        ), @r###"
        ---
        Ok:
          name: hey there
          kind_type: Function
        "###);

        assert_yaml_snapshot!(KindTag::parse(
            Span::dummy("hey there"),
            KindTagType::Property
        ), @r###"
        ---
        Ok:
          name: hey there
          kind_type: Property
        "###);

        assert_yaml_snapshot!(KindTag::parse(
            Span::dummy("This is a class"),
            KindTagType::Class
        ), @r###"
        ---
        Ok:
          name: This is a class
          kind_type: Class
        "###);

        assert_yaml_snapshot!(KindTag::parse(Span::dummy(""), KindTagType::Class), @r###"
        ---
        Err:
          text: This kind tag has stuff after it
          start: 0
          len: 0
          file_id: 0
          additional_diagnostics: []
        "###);
    }
}
