use std::collections::BTreeSet;

use crate::{
    diagnostic::{Diagnostic, Diagnostics},
    doc_comment::{DocComment, OutputSource},
    realm::Realm,
    serde_util::is_false,
    tags::{CustomTag, DeprecatedTag, ErrorTag, ParamTag, ReturnTag, Tag},
};
use full_moon::ast::{types::TypeInfo::Tuple, FunctionBody};
use serde::Serialize;

use super::DocEntryParseArguments;

/// Used to separate functions (called with a dot) from methods (called with a colon)
#[derive(Debug, PartialEq, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub enum FunctionType {
    Method,
    Static,
}

#[derive(Debug, PartialEq, Serialize, Clone)]
pub struct FunctionSource {
    params: Vec<FunctionParam>,
    returns: Vec<FunctionReturn>,
}

impl From<FunctionBody> for FunctionSource {
    fn from(func: FunctionBody) -> Self {
        let mut params = Vec::new();

        let params_and_types = func.parameters().into_iter().zip(func.type_specifiers());
        for (parameter, type_specifier) in params_and_types {
            let source_param = FunctionParam {
                name: match parameter {
                    full_moon::ast::Parameter::Ellipse(_) => "...".to_owned(),
                    full_moon::ast::Parameter::Name(token) => {
                        if let full_moon::tokenizer::TokenType::Identifier { identifier } =
                            token.token_type()
                        {
                            identifier.to_string()
                        } else {
                            unreachable!()
                        }
                    }
                    _ => {
                        unreachable!()
                    }
                },
                desc: "".to_string(),
                lua_type: type_specifier
                    .map(|type_specifier| type_specifier.type_info().to_string())
                    .unwrap_or_else(String::new),
            };

            params.push(source_param);
        }

        let returns = match func.return_type() {
            Some(return_type) => {
                let info = return_type.type_info();

                match info {
                    Tuple { types, .. } => types
                        .into_iter()
                        .map(|ty| FunctionReturn {
                            lua_type: ty.to_string(),
                            desc: String::new(),
                        })
                        .collect::<Vec<_>>(),
                    _ => vec![FunctionReturn {
                        lua_type: info.to_string(),
                        desc: String::new(),
                    }],
                }
            }
            None => Vec::new(),
        };

        FunctionSource { params, returns }
    }
}

#[derive(Debug, PartialEq, Serialize, Clone)]
pub struct FunctionParam {
    name: String,
    desc: String,
    lua_type: String,
}

impl<'a> From<ParamTag<'a>> for FunctionParam {
    fn from(tag: ParamTag) -> Self {
        Self {
            name: tag.name.to_string(),
            desc: tag.desc.to_string(),
            lua_type: tag.lua_type.to_string(),
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Clone)]
pub struct FunctionReturn {
    desc: String,
    lua_type: String,
}

impl<'a> From<ReturnTag<'a>> for FunctionReturn {
    fn from(tag: ReturnTag) -> Self {
        Self {
            desc: tag.desc.to_string(),
            lua_type: tag.lua_type.to_string(),
        }
    }
}

/// A DocEntry for a function or method.
#[derive(Debug, PartialEq, Serialize)]
pub struct FunctionDocEntry<'a> {
    pub name: String,
    pub desc: String,
    pub params: Vec<FunctionParam>,
    pub returns: Vec<FunctionReturn>,
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
        function_source: Option<FunctionSource>,
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

        let source_exists = if let Some(function_source) = function_source {
            for param in function_source.params {
                doc_entry.params.push(param);
            }

            for ret in function_source.returns {
                doc_entry.returns.push(ret)
            }

            true
        } else {
            false
        };

        let mut return_cleared = false;
        for tag in tags {
            match tag {
                Tag::Param(param) => {
                    if source_exists {
                        if let Some(found) = doc_entry.params.iter_mut().find(|existing_param| {
                            param.name.as_str().replace('?', "") == existing_param.name
                        }) {
                            found.desc = param.desc.to_string();

                            if !param.lua_type.is_empty() {
                                found.lua_type = param.lua_type.to_string();
                            }

                            // Special case for params ending with ?
                            // Luau doesn't actually allow this syntax but users use it
                            if param.name.ends_with('?') && !found.name.ends_with('?') {
                                found.name = format!("{}?", found.name);
                            }
                        } else {
                            return Err(Diagnostics::from(vec![Diagnostic::from_span(
                                format!(
                                    "Param \"{}\" does not actually exist in function",
                                    param.name
                                ),
                                param.name,
                            )]));
                        }
                    } else {
                        doc_entry.params.push(param.into());
                    }
                }
                Tag::Return(return_tag) => {
                    if source_exists && !return_cleared {
                        doc_entry.returns.clear();
                        return_cleared = true;
                    }

                    doc_entry.returns.push(return_tag.into());
                }
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

        let mut diagnostics = Vec::new();
        for param in doc_entry.params.iter() {
            if param.lua_type.is_empty() {
                diagnostics.push(Diagnostic::from_doc_comment(
                    format!("Function parameter \"{}\" has no type. Document with @param or insert Luau type annotation", param.name),
                    source,
                ))
            }
        }

        if !diagnostics.is_empty() {
            return Err(Diagnostics::from(diagnostics));
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
