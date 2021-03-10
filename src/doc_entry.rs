use serde::Serialize;
use std::convert::TryFrom;

use crate::{
    diagnostic::{Diagnostic, Diagnostics},
    doc_comment::DocComment,
    span::Span,
    tags::{validate_tags, Tag},
};
use full_moon::ast::Stmt;

mod class;
mod function;
mod property;
mod type_definition;

pub use class::ClassDocEntry;
pub use function::{FunctionDocEntry, FunctionType};
pub use property::PropertyDocEntry;
pub use type_definition::TypeDocEntry;

/// Enum used when determining the type of the DocEntry during parsing
#[derive(Debug, PartialEq)]
enum DocEntryKind {
    Function {
        within: String,
        name: String,
        function_type: FunctionType,
    },
    Property {
        within: String,
        name: String,
    },
    Type {
        within: String,
        name: String,
    },
    Class {
        name: String,
    },
}

/// An enum of all possible DocEntries
#[derive(Debug, Serialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum DocEntry<'a> {
    Function(FunctionDocEntry<'a>),
    Property(PropertyDocEntry<'a>),
    Class(ClassDocEntry<'a>),
    Type(TypeDocEntry<'a>),
}

#[derive(Debug)]
struct DocEntryParseArguments<'a> {
    name: String,
    tags: Vec<Tag<'a>>,
    desc: String,
    within: Option<String>,
    source: &'a DocComment,
}

fn get_within_tag<'a>(tags: &'a [Tag], kind_tag: &Tag) -> Result<String, Diagnostic> {
    for tag in tags {
        if let Tag::Within(within_tag) = tag {
            return Ok(within_tag.name.as_str().to_owned());
        }
    }

    Err(kind_tag.diagnostic("Must specify containing class with @within tag"))
}

fn get_explicit_kind(tags: &[Tag]) -> Result<Option<DocEntryKind>, Diagnostic> {
    for tag in tags {
        match tag {
            Tag::Class(class_tag) => {
                return Ok(Some(DocEntryKind::Class {
                    name: class_tag.name.as_str().to_owned(),
                }))
            }
            Tag::Function(function_tag) => {
                return Ok(Some(DocEntryKind::Function {
                    name: function_tag.name.as_str().to_owned(),
                    function_type: FunctionType::Static,
                    within: get_within_tag(tags, tag)?,
                }));
            }
            Tag::Property(property_tag) => {
                return Ok(Some(DocEntryKind::Property {
                    name: property_tag.name.as_str().to_owned(),
                    within: get_within_tag(tags, tag)?,
                }))
            }
            Tag::Type(type_tag) => {
                return Ok(Some(DocEntryKind::Type {
                    name: type_tag.name.as_str().to_owned(),
                    within: get_within_tag(tags, tag)?,
                }))
            }
            // TODO: interface
            _ => (),
        }
    }

    Ok(None)
}

fn determine_kind(
    doc_comment: &DocComment,
    stmt: Option<&Stmt>,
    tags: &[Tag],
) -> Result<DocEntryKind, Diagnostic> {
    let explicit_kind = get_explicit_kind(tags)?;

    if let Some(kind) = explicit_kind {
        return Ok(kind);
    }

    match stmt {
        Some(Stmt::FunctionDeclaration(function)) => match function.name().method_name() {
            Some(method_name) => Ok(DocEntryKind::Function {
                name: method_name.to_string(),
                within: function.name().names().to_string(),
                function_type: FunctionType::Method,
            }),
            None => {
                let mut names: Vec<_> = function.name().names().iter().collect();

                let function_name = names.pop().unwrap().to_string();

                if names.is_empty() {
                    return Err(doc_comment.diagnostic("Function requires @within tag"));
                }

                Ok(DocEntryKind::Function {
                    name: function_name,
                    within: names
                        .into_iter()
                        .map(|token| token.to_string())
                        .collect::<Vec<_>>()
                        .join("."),
                    function_type: FunctionType::Static,
                })
            }
        },

        _ => Err(doc_comment
            .diagnostic("Explicitly specify a kind tag, like @function, @prop, or @class.")),
    }
}

impl<'a> DocEntry<'a> {
    pub fn parse(
        doc_comment: &'a DocComment,
        stmt: Option<&Stmt<'a>>,
    ) -> Result<DocEntry<'a>, Diagnostics> {
        let span: Span<'a> = doc_comment.into();

        let (tag_lines, desc_lines): (Vec<Span>, Vec<Span>) = span
            .lines()
            .map(Span::trim)
            .filter(|line| !line.is_empty())
            .partition(|line| line.starts_with('@'));

        let desc = desc_lines
            .iter()
            .map(|span| span.as_str())
            .collect::<Vec<_>>()
            .join("\n");

        let (tags, errors): (Vec<_>, Vec<_>) = tag_lines
            .into_iter()
            .map(Tag::try_from)
            .partition(Result::is_ok);

        let mut tags: Vec<_> = tags.into_iter().map(Result::unwrap).collect();
        let mut errors: Vec<_> = errors.into_iter().map(Result::unwrap_err).collect();

        errors.extend(validate_tags(&tags));

        if !errors.is_empty() {
            return Err(Diagnostics::from(errors));
        }

        let kind =
            determine_kind(doc_comment, stmt, &tags).map_err(|err| Diagnostics::from(vec![err]))?;

        // Sift out the kind/within tags because those are only used for determining the kind
        tags.retain(|t| {
            !matches!(
                t,
                Tag::Function(_) | Tag::Within(_) | Tag::Class(_) | Tag::Property(_) | Tag::Type(_)
            )
        });

        Ok(match kind {
            DocEntryKind::Function {
                within,
                name,
                function_type,
            } => DocEntry::Function(FunctionDocEntry::parse(
                DocEntryParseArguments {
                    within: Some(within),
                    name,
                    desc,
                    tags,
                    source: doc_comment,
                },
                function_type,
            )?),
            DocEntryKind::Property { within, name } => {
                DocEntry::Property(PropertyDocEntry::parse(DocEntryParseArguments {
                    within: Some(within),
                    name,
                    desc,
                    tags,
                    source: doc_comment,
                })?)
            }
            DocEntryKind::Type { within, name } => {
                DocEntry::Type(TypeDocEntry::parse(DocEntryParseArguments {
                    within: Some(within),
                    name,
                    desc,
                    tags,
                    source: doc_comment,
                })?)
            }
            DocEntryKind::Class { name } => {
                DocEntry::Class(ClassDocEntry::parse(DocEntryParseArguments {
                    within: None,
                    name,
                    desc,
                    tags,
                    source: doc_comment,
                })?)
            }
        })
    }
}
