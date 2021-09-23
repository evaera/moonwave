use std::collections::BTreeSet;

use crate::{
    diagnostic::Diagnostics,
    doc_comment::{DocComment, OutputSource},
    realm::Realm,
    serde_util::is_false,
    tags::{CustomTag, DeprecatedTag, Tag},
};
use serde::Serialize;

use super::DocEntryParseArguments;

/// A DocEntry for a class which contains functions, properties, and types
#[derive(Debug, PartialEq, Serialize)]
pub struct ClassDocEntry<'a> {
    pub name: String,
    pub desc: String,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<CustomTag<'a>>,
    #[serde(skip_serializing_if = "BTreeSet::is_empty")]
    pub realm: BTreeSet<Realm>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deprecated: Option<DeprecatedTag<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub since: Option<String>,
    #[serde(skip_serializing_if = "is_false")]
    pub private: bool,
    #[serde(skip_serializing_if = "is_false")]
    pub unreleased: bool,
    #[serde(skip_serializing_if = "is_false")]
    pub ignore: bool,

    #[serde(rename = "source")]
    pub output_source: OutputSource,

    #[serde(skip)]
    pub __index: String,

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
            tags: Vec::new(),
            realm: BTreeSet::new(),
            private: false,
            unreleased: false,
            ignore: false,
            deprecated: None,
            since: None,
            output_source: source.output_source.clone(),
            __index: "__index".to_owned(),
        };

        let mut unused_tags = Vec::new();

        for tag in tags {
            match tag {
                Tag::Custom(tag) => doc_entry.tags.push(tag),
                Tag::Deprecated(deprecated_tag) => doc_entry.deprecated = Some(deprecated_tag),
                Tag::Since(since_tag) => doc_entry.since = Some(since_tag.version.to_string()),
                Tag::Index(index_tag) => doc_entry.__index = index_tag.name.to_string(),

                Tag::Private(_) => doc_entry.private = true,
                Tag::Unreleased(_) => doc_entry.unreleased = true,
                Tag::Ignore(_) => doc_entry.ignore = true,

                Tag::Server(_) => {
                    doc_entry.realm.insert(Realm::Server);
                }
                Tag::Client(_) => {
                    doc_entry.realm.insert(Realm::Client);
                }
                Tag::Plugin(_) => {
                    doc_entry.realm.insert(Realm::Plugin);
                }
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
