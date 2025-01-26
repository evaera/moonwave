use crate::{
    diagnostic::{Diagnostic, Diagnostics},
    doc_comment::{DocComment, OutputSource},
    serde_util::is_false,
    tags::{CustomTag, ExternalTag, FieldTag, Tag},
};
use full_moon::{ast::{luau::{
    GenericDeclaration, GenericDeclarationParameter, GenericParameterInfo, IndexedTypeInfo, TypeArgument, TypeField, TypeFieldKey, TypeInfo
}, punctuated::Punctuated}, node::Node, tokenizer::{TokenReference, TokenType}};
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

fn gen_param_info_to_string(gen_param_info: &GenericParameterInfo) -> Option<String> {
    match gen_param_info {
        GenericParameterInfo::Name(name) => {
            Some(name.token().to_string())
        }
        GenericParameterInfo::Variadic { name, ellipsis } => {
            Some(format!(
                "{}{}",
                name.token(),
                ellipsis.token()
            ))
        }
        _ => None,
    }
}

fn gen_decl_param_to_string(gen_decl_param: &GenericDeclarationParameter) -> Option<String> {
    let parameter_string = match gen_param_info_to_string(gen_decl_param.parameter()) {
        Some(string ) => string,
        None => return None,
    };
    let equals_string = optional_token_to_string(gen_decl_param.equals());
    let type_string = match gen_decl_param.default_type() {
        Some(parameter) => match type_info_to_string(parameter) {
            Some(string) => string,
            None => return None,
        },
        None => String::new(),
    };
    Some(format!(
        "{}{}{}",
        parameter_string,
        equals_string,
        type_string
    ))
}

fn punctuated_generics_to_string(punctuated: &Punctuated<GenericDeclarationParameter>) -> Option<String> {
    let mut string = String::new();

    for generic in punctuated {
        let generic_string = match gen_decl_param_to_string(generic) {
            Some(string) => string,
            None => return None,
        };
        string.push_str(generic_string.as_str())
    }

    Some(string)
}

fn gen_decl_to_string(gen_decl: &GenericDeclaration) -> Option<String> {
    let (start, end) = gen_decl.arrows().tokens();
    let generics_string = match punctuated_generics_to_string(gen_decl.generics()) {
        Some(string) => string,
        None => return None
    };
    Some(format!(
        "{}{}{}",
        start.token(),
        generics_string,
        end.token()
    ))
}

fn punctuated_type_argument_to_string(punctuated: &Punctuated<TypeArgument>) -> Option<String> {
    let mut string = String::new();

    for pair in punctuated.pairs() {
        let type_string = match type_argument_to_string(pair.value()) {
            Some(string) => string,
            None => return None,
        };
        string.push_str(type_string.as_str());
        string.push_str(optional_token_to_string(pair.punctuation()).as_str());
    }

    Some(string)
}

fn punctuated_type_field_to_string(punctuated: &Punctuated<TypeField>) -> Option<String> {
    let mut string = String::new();

    for pair in punctuated.pairs() {
        let type_string = match type_field_to_string(pair.value()) {
            Some(string) => string,
            None => return None,
        };
        string.push_str(type_string.as_str());
        string.push_str(optional_token_to_string(pair.punctuation()).as_str());
    }

    Some(string)
}

fn punctuated_type_info_to_string(punctuated: &Punctuated<TypeInfo>) -> Option<String> {
    let mut string = String::new();

    for pair in punctuated.pairs() {
        let type_string = match type_info_to_string(pair.value()) {
            Some(string) => string,
            None => return None,
        };
        string.push_str(type_string.as_str());
        string.push_str(optional_token_to_string(pair.punctuation()).as_str());
    }

    Some(string)
}

/// Converts an IndexedTypeInfo to a String representation, excluding trivia.
fn indexed_type_info_to_string(indexed_type_info: &IndexedTypeInfo) -> Option<String> {
    match indexed_type_info {
        IndexedTypeInfo::Basic(basic) => {
            Some(basic.token().to_string())
        }
        IndexedTypeInfo::Generic { base, arrows, generics } => {
            let (start, end) = arrows.tokens();
            let generics_string = match punctuated_type_info_to_string(generics) {
                Some(string) => string,
                None => return None,
            };
            Some(format!(
                "{}{}{}{}",
                base.token(),
                start.token(),
                generics_string,
                end.token()
            ))
        }
        _ => None
    }
}

/// Converts an optional TokenReference to a String representation, excluding trivia.
fn optional_token_to_string(token: Option<&TokenReference>) -> String {
    match token {
        Some(token) => token.token().to_string(),
        None => String::new(),
    }
}

/// Converts a TypeArgument to a String representation, excluding trivia.
fn type_argument_to_string(type_argument: &TypeArgument) -> Option<String> {
    let name_string = match type_argument.name() {
        Some((identifier, colon)) => {
            format!(
                "{}{}",
                identifier.token(),
                colon.token()
            )
        },
        None => String::new(),
    };
    let type_string = match type_info_to_string(type_argument.type_info()) {
        Some(string) => string,
        None => return None,
    };
    Some(format!(
        "{}{}",
        name_string,
        type_string
    ))
}

/// Converts a TypeField to a String representation, excluding trivia.
fn type_field_to_string(type_field: &TypeField) -> Option<String> {
    let access = optional_token_to_string(type_field.access());
    let key = match type_field_key_to_string(type_field.key()) {
        Some(string) => string,
        None => return None,
    };
    let value = match type_info_to_string(type_field.value()) {
        Some(string) => string,
        None => return None,
    };
    Some(format!(
        "{}{}{}{}",
        access,
        key,
        type_field.colon_token().token(),
        value
    ))
}

/// Converts a TypeFieldKey to a String representation, excluding trivia.
fn type_field_key_to_string(field_key: &TypeFieldKey) -> Option<String> {
    match field_key {
        TypeFieldKey::IndexSignature { brackets, inner } => {
            let (start, end) = brackets.tokens();
            Some(format!("{}{}{}", start.token(), inner, end.token()))
        }
        TypeFieldKey::Name(token_reference) => {
            Some(token_reference.token().to_string())
        }
        _ => None
    }
}

/// Converts a TypeInfo to a String representation, excluding trivia.
fn type_info_to_string(type_info: &TypeInfo) -> Option<String> {
    match type_info {
        TypeInfo::Array { braces, access, type_info } => {
            let (start, end) = braces.tokens();
            let access_string = optional_token_to_string(access.as_ref());
            Some(format!(
                "{}{}{}{}",
                start.token(),
                access_string,
                type_info,
                end.token()
            ))
        }
        TypeInfo::Basic(basic) => {
            Some(basic.token().to_string())
        }
        TypeInfo::String(string) => {
            Some(string.token().to_string())
        }
        TypeInfo::Boolean(boolean) => {
            Some(boolean.token().to_string())
        }
        TypeInfo::Callback { generics, parentheses, arguments, arrow, return_type } => {
            let generics_string = match generics {
                Some(generics) => {
                    match gen_decl_to_string(generics) {
                        Some(string) => string,
                        None => return None,
                    }
                },
                None => String::new(),
            };
            let (start, end) = parentheses.tokens();
            let arguments_string = match punctuated_type_argument_to_string(arguments) {
                Some(string) => string,
                None => return None,
            };
            let return_type_string = match type_info_to_string(return_type) {
                Some(string) => string,
                None => return None,
            };
            Some(format!(
                "{}{}{}{}{}{}",
                generics_string,
                start.token(),
                arguments_string,
                end.token(),
                arrow.token(),
                return_type_string
            ))
        }
        TypeInfo::Generic { base, arrows, generics } => {
            let (start, end) = arrows.tokens();
            let generics_string = match punctuated_type_info_to_string(generics) {
                Some(string) => string,
                None => return None,
            };
            Some(format!(
                "{}{}{}{}",
                base.token(),
                start.token(),
                generics_string,
                end.token()
            ))
        }
        TypeInfo::GenericPack { name, ellipsis } => {
            Some(format!(
                "{}{}",
                name.token(),
                ellipsis.token()
            ))
        }
        TypeInfo::Intersection(intersection) => {
            let leading_string = optional_token_to_string(intersection.leading());
            let types_string = match punctuated_type_info_to_string(intersection.types()) {
                Some(string) => string,
                None => return None,
            };
            Some(format!(
                "{}{}",
                leading_string,
                types_string
            ))
        }
        TypeInfo::Module { module, punctuation, type_info } => {
            let module_index_string = match indexed_type_info_to_string(type_info.as_ref()) {
                Some(string) => string,
                None => return None,
            };
            Some(format!(
                "{}{}{}",
                module.token(),
                punctuation.token(),
                module_index_string
            ))
        }
        TypeInfo::Optional { base, question_mark } => {
            let base_string = match type_info_to_string(base.as_ref()) {
                Some(string) => string,
                None => return None
            };
            Some(format!(
                "{}{}",
                base_string,
                question_mark.token()
            ))
        }
        TypeInfo::Table { braces, fields } => {
            let (start, end) = braces.tokens();
            let fields_string = match punctuated_type_field_to_string(fields) {
                Some(string) => string,
                None => return None,
            };
            Some(format!(
                "{}{}{}",
                start.token(),
                fields_string,
                end.token()
            ))
        }
        TypeInfo::Typeof { typeof_token, parentheses, inner } => {
            let (start, end) = parentheses.tokens();
            Some(format!(
                "{}{}{}{}",
                typeof_token.token(),
                start.token(),
                inner.to_string(),
                end.token()
            ))
        }
        TypeInfo::Tuple { parentheses, types } => {
            let (start, end) = parentheses.tokens();
            let types_string = match punctuated_type_info_to_string(types) {
                Some(string) => string,
                None => return None,
            };
            Some(format!(
                "{}{}{}",
                start.token(),
                types_string,
                end.token()
            ))
        }
        TypeInfo::Union(union) => {
            let leading_string = optional_token_to_string(union.leading());
            let types_string = match punctuated_type_info_to_string(union.types()) {
                Some(string) => string,
                None => return None,
            };
            Some(format!(
                "{}{}",
                leading_string,
                types_string
            ))
        }
        TypeInfo::Variadic { ellipsis, type_info } => {
            let type_string = match type_info_to_string(type_info.as_ref()) {
                Some(string) => string,
                None => return None
            };
            Some(format!(
                "{}{}",
                ellipsis.token(),
                type_string
            ))
        }
        TypeInfo::VariadicPack { ellipsis, name } => {
            Some(format!(
                "{}{}",
                ellipsis.token(),
                name.token()
            ))
        }
        _ => None
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
                        
                        let name = match type_field_key_to_string(field.key()) {
                            Some(name) => name,
                            None => continue,
                        };
                        
                        let lua_type = match type_info_to_string(field.value()) {
                            Some(lua_type) => lua_type,
                            None => continue
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
                            lua_type,
                            desc,
                        });
                    }
                },
                _ => doc_entry.lua_type = match type_info_to_string(&type_info) {
                    Some(string) => Some(string),
                    None => Some(type_info.to_string().trim().to_string()),
                }
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
                            if !field_tag.lua_type.is_empty() {
                                found.lua_type = field_tag.lua_type.to_string();
                            }

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
                        if field_tag.lua_type.is_empty() {
                            field_tag.source.diagnostic("Field type is required when missing type info");
                        }

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
