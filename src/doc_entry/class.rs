use crate::{
    diagnostic::Diagnostics,
    doc_comment::DocComment,
    tags::{MarkerTag, Tag},
};
use serde::Serialize;

use super::DocEntryParseArguments;

/// A DocEntry for a class which contains functions, properties, and types
#[derive(Debug, PartialEq, Serialize)]
pub struct ClassDocEntry<'a> {
    pub name: String,
    pub desc: String,
    pub markers: Vec<MarkerTag<'a>>,
    #[serde(skip)]
    pub source: &'a DocComment,
}

impl<'a> ClassDocEntry<'a> {
    pub(super) fn parse(args: DocEntryParseArguments<'a>) -> Result<Self, Diagnostics> {
        let DocEntryParseArguments {
            name,
            desc,
            within: _,
            tags,
            source,
        } = args;

        let mut markers = Vec::new();
        let mut unused_tags = Vec::new();

        for tag in tags {
            match tag {
                Tag::Marker(marker) => markers.push(marker),
                _ => unused_tags.push(tag),
            }
        }

        Ok(Self {
            name,
            desc,
            markers,
            source,
        })
    }
}
