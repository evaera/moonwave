use crate::{
    diagnostic::Diagnostics,
    doc_comment::{DocComment, OutputSource},
    serde_util::is_false,
    tags::{CustomTag, ExternalTag, FieldTag, Tag},
};
use full_moon::ast::luau::TypeInfo;
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
    #[serde(skip_serializing_if = "is_false")]
    pub private: bool,
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
    pub(super) fn parse(
        args: DocEntryParseArguments<'a>,
        type_info: Option<TypeInfo>,
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
            lua_type: None,
            fields: Vec::new(),
            within: within.unwrap(),
            tags: Vec::new(),
            external_types: Vec::new(),
            private: false,
            ignore: false,
            output_source: source.output_source.clone(),
        };

        let mut unused_tags = Vec::new();

        for tag in tags {
            match tag {
                Tag::Type(type_tag) => {
                    if let Some(explicit_lua_type) = type_tag.lua_type {
                        doc_entry.lua_type = Some(explicit_lua_type.as_str().to_owned())
                    }
                }

                Tag::Field(field_tag) => doc_entry.fields.push(field_tag.into()),

                Tag::Custom(custom_tag) => doc_entry.tags.push(custom_tag),
                Tag::External(external_tag) => doc_entry.external_types.push(external_tag),

                Tag::Private(_) => doc_entry.private = true,
                Tag::Ignore(_) => doc_entry.ignore = true,

                _ => unused_tags.push(tag),
            }
        }

        if doc_entry.lua_type.is_none() && type_info.is_some() {
            doc_entry.lua_type = Some(type_info.unwrap().to_string())
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
