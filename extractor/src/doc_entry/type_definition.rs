use crate::{
    diagnostic::Diagnostics,
    doc_comment::{DocComment, OutputSource},
    serde_util::is_false,
    tags::{CustomTag, DeprecatedTag, ExternalTag, FieldTag, Tag},
};
use serde::Serialize;

use super::DocEntryParseArguments;

#[derive(Debug, PartialEq, Serialize)]
pub struct Field {
    pub name: String,
    pub lua_type: String,
    pub desc: String,
}

impl<'a> From<FieldTag<'a>> for Field {
    fn from(field_tag: FieldTag<'a>) -> Self {
        Self {
            name: field_tag.name.as_str().to_owned(),
            lua_type: field_tag.lua_type.as_str().to_owned(),
            desc: field_tag.desc.as_str().to_owned(),
        }
    }
}

/// A DocEntry for a function or method.
#[derive(Debug, PartialEq, Serialize)]
pub struct TypeDocEntry<'a> {
    pub name: String,
    pub desc: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub lua_type: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub fields: Vec<Field>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<CustomTag<'a>>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub external_types: Vec<ExternalTag<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub since: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deprecated: Option<DeprecatedTag<'a>>,
    #[serde(skip_serializing_if = "is_false")]
    pub private: bool,
    #[serde(skip_serializing_if = "is_false")]
    pub unreleased: bool,
    #[serde(skip_serializing_if = "is_false")]
    pub ignore: bool,

    #[serde(rename = "source")]
    pub output_source: OutputSource,

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
            lua_type: None,
            since: None,
            deprecated: None,
            fields: Vec::new(),
            within: within.unwrap(),
            tags: Vec::new(),
            external_types: Vec::new(),
            private: false,
            unreleased: false,
            ignore: false,
            output_source: source.output_source.clone(),
        };

        let mut diagnostics = Vec::new();

        for tag in tags {
            match tag {
                Tag::Type(type_tag) => {
                    doc_entry.lua_type = Some(type_tag.lua_type.as_str().to_owned())
                }

                Tag::Field(field_tag) => doc_entry.fields.push(field_tag.into()),

                Tag::Deprecated(deprecated_tag) => doc_entry.deprecated = Some(deprecated_tag),
                Tag::Since(since_tag) => doc_entry.since = Some(since_tag.version.to_string()),
                Tag::Custom(custom_tag) => doc_entry.tags.push(custom_tag),
                Tag::External(external_tag) => doc_entry.external_types.push(external_tag),
                Tag::Include(include_tag) => {
                    match fs_err::read_to_string(include_tag.path.as_str()) {
                        Ok(text) => {
                            doc_entry.desc.push_str(&text);
                            doc_entry.desc.push('\n');
                        },
                        Err(e) => diagnostics.push(include_tag.path.diagnostic(format!("Unable to read file. Reason: {}", e)))
                    }
                },

                Tag::Private(_) => doc_entry.private = true,
                Tag::Unreleased(_) => doc_entry.unreleased = true,
                Tag::Ignore(_) => doc_entry.ignore = true,

                _ => diagnostics.push(tag.diagnostic("This tag is unused by type doc entries.")),
            }
        }

        if !diagnostics.is_empty() {
            return Err(Diagnostics::from(diagnostics));
        }

        Ok(doc_entry)
    }
}
