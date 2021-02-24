use serde::Serialize;

use crate::{diagnostic::Diagnostic, span::Span};

#[derive(Debug, PartialEq, Serialize)]
#[serde(transparent)]
pub struct CustomTag<'a> {
    pub name: Span<'a>,
    #[serde(skip)]
    pub source: Span<'a>,
}

impl<'a> CustomTag<'a> {
    pub fn parse(name: Span<'a>) -> Result<Self, Diagnostic> {
        Ok(Self { name, source: name })
    }
}

#[cfg(test)]
mod tests {
    use insta::assert_yaml_snapshot;

    use super::*;

    #[test]
    fn custom_tag() {
        let source = Span::dummy("example");

        let value = CustomTag::parse(source).unwrap();

        assert_yaml_snapshot!(value, @r###"
        ---
        example
        "###);
    }
}
