use crate::{diagnostic::Diagnostic, span::Span};
use serde::Serialize;

use super::TagType;

#[derive(Debug, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum MarkerTagType {
    Unreleased,
    Server,
    Client,
    Private,
    Ignore,
    Yields,
    ReadOnly,
}

impl MarkerTagType {
    pub fn tag_type(&self) -> TagType {
        match self {
            MarkerTagType::Unreleased => TagType::Unreleased,
            MarkerTagType::Server => TagType::Server,
            MarkerTagType::Client => TagType::Client,
            MarkerTagType::Private => TagType::Private,
            MarkerTagType::Ignore => TagType::Ignore,
            MarkerTagType::Yields => TagType::Yields,
            MarkerTagType::ReadOnly => TagType::ReadOnly,
        }
    }
}

#[derive(Debug, PartialEq, Serialize)]
#[serde(transparent)]
pub struct MarkerTag<'a> {
    pub marker_type: MarkerTagType,
    #[serde(skip)]
    pub source: Span<'a>,
}

impl<'a> MarkerTag<'a> {
    pub fn parse(tag_type: MarkerTagType) -> Result<Self, Diagnostic> {
        Ok(Self {
            marker_type: tag_type,
            source: Span::dummy(""),
        })
    }
}

#[cfg(test)]
mod test {
    use insta::assert_yaml_snapshot;

    use super::*;

    #[test]
    fn snapshot() {
        assert_yaml_snapshot!(MarkerTag::parse(MarkerTagType::Unreleased));
        assert_yaml_snapshot!(MarkerTag::parse(MarkerTagType::Unreleased));
    }
}
