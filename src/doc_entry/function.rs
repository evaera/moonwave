use crate::{
    diagnostic::Diagnostics,
    doc_comment::DocComment,
    tags::{CustomTag, DeprecatedTag, ErrorTag, MarkerTag, ParamTag, ReturnTag, Tag},
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
    pub within: String,
    pub params: Vec<ParamTag<'a>>,
    pub returns: Vec<ReturnTag<'a>>,
    pub markers: Vec<MarkerTag<'a>>,
    pub tags: Vec<CustomTag<'a>>,
    pub errors: Vec<ErrorTag<'a>>,
    pub function_type: FunctionType,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub since: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub deprecated: Option<DeprecatedTag<'a>>,

    #[serde(skip)]
    pub source: &'a DocComment,
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
            markers: Vec::new(),
            tags: Vec::new(),
            errors: Vec::new(),
        };

        let mut unused_tags = Vec::new();

        for tag in tags {
            match tag {
                Tag::Param(param) => doc_entry.params.push(param),
                Tag::Return(return_tag) => doc_entry.returns.push(return_tag),
                Tag::Marker(marker) => doc_entry.markers.push(marker),
                Tag::Deprecated(deprecated_tag) => doc_entry.deprecated = Some(deprecated_tag),
                Tag::Since(since_tag) => doc_entry.since = Some(since_tag.version.to_string()),
                Tag::Custom(custom_tag) => doc_entry.tags.push(custom_tag),
                Tag::Error(error_tag) => doc_entry.errors.push(error_tag),
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
