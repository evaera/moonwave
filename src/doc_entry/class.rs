use crate::{
    diagnostic::Diagnostics,
    doc_comment::DocComment,
    tags::{CustomTag, MarkerTag, Tag},
};
use serde::Serialize;

use super::DocEntryParseArguments;

/// A DocEntry for a class which contains functions, properties, and types
#[derive(Debug, PartialEq, Serialize)]
pub struct ClassDocEntry<'a> {
    pub name: String,
    pub desc: String,
    pub markers: Vec<MarkerTag<'a>>,
    pub tags: Vec<CustomTag<'a>>,
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

        let mut doc_entry = Self {
            name,
            desc,
            source,
            markers: Vec::new(),
            tags: Vec::new(),
        };

        let mut unused_tags = Vec::new();

        for tag in tags {
            match tag {
                Tag::Marker(tag) => doc_entry.markers.push(tag),
                Tag::Custom(tag) => doc_entry.tags.push(tag),
                _ => unused_tags.push(tag),
            }
        }

        if !unused_tags.is_empty() {
            let mut diagnostics = Vec::new();
            for tag in unused_tags {
                diagnostics.push(tag.diagnostic("This tag is unused by class doc entries."));
            }

            return Err(Diagnostics::from(diagnostics));
        }

        Ok(doc_entry)
    }
}
