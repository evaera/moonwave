use std::collections::BTreeSet;

use crate::{
    diagnostic::Diagnostics,
    doc_comment::{DocComment, OutputSource},
    realm::Realm,
    serde_util::is_false,
    tags::{CustomTag, DeprecatedTag, ErrorTag, ParamTag, ReturnTag, Tag},
};
use serde::Serialize;

use super::DocEntryParseArguments;

/// Used to separate functions (called with a dot) from methods (called with a colon)
#[derive(Debug, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum FunctionType {
    Method,
    Static,
}

/// A DocEntry for a function or method.
#[derive(Debug, PartialEq, Serialize)]
pub struct FunctionDocEntry<'a> {
    pub name: String,
    pub desc: String,
    pub params: Vec<ParamTag<'a>>,
    pub returns: Vec<ReturnTag<'a>>,
    pub function_type: FunctionType,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<CustomTag<'a>>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub errors: Vec<ErrorTag<'a>>,
    #[serde(skip_serializing_if = "BTreeSet::is_empty")]
    pub realm: BTreeSet<Realm>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub since: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deprecated: Option<DeprecatedTag<'a>>,
    #[serde(skip_serializing_if = "is_false")]
    pub private: bool,
    #[serde(skip_serializing_if = "is_false")]
    pub unreleased: bool,
    #[serde(skip_serializing_if = "is_false")]
    pub yields: bool,
    #[serde(skip_serializing_if = "is_false")]
    pub ignore: bool,

    #[serde(rename = "source")]
    pub output_source: OutputSource,

    #[serde(skip)]
    pub source: &'a DocComment,
    #[serde(skip)]
    pub within: String,
}

impl<'a> FunctionDocEntry<'a> {
    pub(super) fn parse(
        args: DocEntryParseArguments<'a>,
        function_type: FunctionType,
    ) -> Result<Self, Diagnostics> {
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
            function_type,
            since: None,
            deprecated: None,
            within: within.unwrap(),
            params: Vec::new(),
            returns: Vec::new(),
            tags: Vec::new(),
            errors: Vec::new(),
            realm: BTreeSet::new(),
            private: false,
            unreleased: false,
            yields: false,
            ignore: false,
            output_source: source.output_source.clone(),
        };

        let mut unused_tags = Vec::new();

        for tag in tags {
            match tag {
                Tag::Param(param) => doc_entry.params.push(param),
                Tag::Return(return_tag) => doc_entry.returns.push(return_tag),
                Tag::Deprecated(deprecated_tag) => doc_entry.deprecated = Some(deprecated_tag),
                Tag::Since(since_tag) => doc_entry.since = Some(since_tag.version.to_string()),
                Tag::Custom(custom_tag) => doc_entry.tags.push(custom_tag),
                Tag::Error(error_tag) => doc_entry.errors.push(error_tag),

                Tag::Private(_) => doc_entry.private = true,
                Tag::Unreleased(_) => doc_entry.unreleased = true,
                Tag::Yields(_) => doc_entry.yields = true,
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
                diagnostics.push(tag.diagnostic("This tag is unused by function doc entries."));
            }

            return Err(Diagnostics::from(diagnostics));
        }

        Ok(doc_entry)
    }
}
