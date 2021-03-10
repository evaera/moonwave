use crate::{
    diagnostic::Diagnostics,
    doc_comment::DocComment,
    tags::{CustomTag, Tag},
};
use serde::Serialize;

use super::DocEntryParseArguments;

/// A DocEntry for a function or method.
#[derive(Debug, PartialEq, Serialize)]
pub struct TypeDocEntry<'a> {
    pub name: String,
    pub desc: String,
    pub tags: Vec<CustomTag<'a>>,

    pub private: bool,
    pub ignore: bool,

    #[serde(skip)]
    pub source: &'a DocComment,

    #[serde(skip)]
    pub within: String,
}

impl<'a> TypeDocEntry<'a> {
    pub(super) fn parse(args: DocEntryParseArguments<'a>) -> Result<Self, Diagnostics> {
        let DocEntryParseArguments {
            name,
            desc,
            within,
            tags,
            source,
        } = args;

        let mut doc_entry = Self {
            name,
            desc,
            source,
            within: within.unwrap(),
            tags: Vec::new(),
            private: false,
            ignore: false,
        };

        let mut unused_tags = Vec::new();

        for tag in tags {
            match tag {
                Tag::Custom(custom_tag) => doc_entry.tags.push(custom_tag),

                Tag::Private(_) => doc_entry.private = true,
                Tag::Ignore(_) => doc_entry.ignore = true,

                _ => unused_tags.push(tag),
            }
        }

        if !unused_tags.is_empty() {
            let mut diagnostics = Vec::new();
            for tag in unused_tags {
                diagnostics.push(tag.diagnostic("This tag is unused by type doc entries."));
            }

            return Err(Diagnostics::from(diagnostics));
        }

        Ok(doc_entry)
    }
}
