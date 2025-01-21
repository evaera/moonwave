use crate::{
    diagnostic::{Diagnostic, Diagnostics},
    doc_comment::{DocComment, OutputSource},
    serde_util::is_false,
    tags::{CustomTag, ExternalTag, FieldTag, Tag},
};
use full_moon::{ast::luau::{TypeFieldKey, TypeInfo}, node::Node, tokenizer::TokenType};
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

        let type_info_exists = type_info.is_some();
        if let Some(type_info) = type_info {
            match type_info {
                TypeInfo::Table { fields, .. } => {
                    for pair in fields.pairs() {
                        let field = pair.value();
                        
                        let name = match field.key() {
                            TypeFieldKey::IndexSignature { brackets, inner } => {
                                let (start, end) = brackets.tokens();
                                format!("{}{}{}", start.token(), inner, end.token())
                            }
                            TypeFieldKey::Name(token_reference) => {
                                token_reference.token().to_string()
                            }
                            _ => continue
                        };

                        let punctuated_trivia = if let Some(punctuated) = pair.punctuation() {
                            vec![
                                punctuated.leading_trivia().collect::<Vec<_>>(),
                                punctuated.trailing_trivia().collect::<Vec<_>>()
                            ]
                            .into_iter()
                            .flatten()
                            .collect()
                        } else {
                            Vec::new()
                        };

                        let (leading_trivia, trailing_trivia) = field.surrounding_trivia();
                        let desc = leading_trivia
                            .iter()
                            .chain(trailing_trivia.iter())
                            .chain(punctuated_trivia.iter())
                            .find_map(|token| match token.token_type() {
                                TokenType::SingleLineComment { comment } => {
                                    if comment.starts_with("-") {
                                        let string = comment
                                            .strip_prefix("-")
                                            .unwrap()
                                            .trim()
                                            .to_string();

                                        Some(string)
                                    } else {
                                        None
                                    }
                                }
                                _ => None
                            })
                            .unwrap_or_else(String::new);

                        doc_entry.fields.push(Field {
                            name,
                            lua_type: field.value().to_string(),
                            desc,
                        });
                    }
                },
                _ => doc_entry.lua_type = Some(type_info.to_string().trim().to_string())
            }
        }

        let mut unused_tags = Vec::new();

        for tag in tags {
            match tag {
                Tag::Type(type_tag) => {
                    doc_entry.lua_type = Some(type_tag.lua_type.as_str().to_owned());
                }

                Tag::Field(field_tag) => {
                    if type_info_exists {
                        if let Some(found) = doc_entry.fields.iter_mut().find(|existing_field| {
                            field_tag.name.as_str() == existing_field.name
                        }) {
                            found.lua_type = field_tag.lua_type.to_string();

                            if !field_tag.desc.is_empty() {
                                found.desc = field_tag.desc.to_string();
                            }
                        } else {
                            return Err(Diagnostics::from(vec![Diagnostic::from_span(
                                format!(
                                    "Field \"{}\" does not actually exist in interface",
                                    field_tag.name
                                ),
                                field_tag.name,
                            )]));
                        }
                    } else {
                        doc_entry.fields.push(field_tag.into())
                    }
                },

                Tag::Custom(custom_tag) => doc_entry.tags.push(custom_tag),
                Tag::External(external_tag) => doc_entry.external_types.push(external_tag),

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
