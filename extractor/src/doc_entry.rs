use serde::Serialize;
use std::convert::TryFrom;

use crate::{
    diagnostic::{Diagnostic, Diagnostics},
    doc_comment::DocComment,
    span::Span,
    tags::{validate_tags, Tag},
};
use full_moon::{
    ast::{self, punctuated::Punctuated, Stmt},
    node::Node,
    tokenizer,
};

mod class;
mod function;
mod property;
mod type_definition;

pub use class::ClassDocEntry;
pub use function::{FunctionDocEntry, FunctionType};
pub use property::PropertyDocEntry;
pub use type_definition::TypeDocEntry;

use self::function::FunctionSource;

/// Enum used when determining the type of the DocEntry during parsing
#[derive(Debug, PartialEq)]
enum DocEntryKind {
    Function {
        within: String,
        name: String,
        function_type: FunctionType,
        function_source: Option<FunctionSource>,
    },
    Property {
        within: String,
        name: String,
        lua_type: Option<String>,
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
                    function_type: function_tag.function_type.clone(),
                    within: get_within_tag(tags, tag)?,
                    function_source: None,
                }));
            }
            Tag::Property(property_tag) => {
                return Ok(Some(DocEntryKind::Property {
                    name: property_tag.name.as_str().to_owned(),
                    within: get_within_tag(tags, tag)?,
                    lua_type: None,
                }))
            }
            Tag::Type(type_tag) => {
                return Ok(Some(DocEntryKind::Type {
                    name: type_tag.name.as_str().to_owned(),
                    within: get_within_tag(tags, tag)?,
                }))
            }
            Tag::Interface(interface_tag) => {
                return Ok(Some(DocEntryKind::Type {
                    name: interface_tag.name.as_str().to_owned(),
                    within: get_within_tag(tags, tag)?,
                }))
            }
            _ => (),
        }
    }

    Ok(None)
}

fn assert_token_sequence_is_single<T>(
    sequence: &Punctuated<T>,
    doc_comment: &DocComment,
    token_kind: &str,
) -> Result<(), Diagnostic>
where
    T: Node,
{
    if sequence.len() > 1 {
        if let Some((start, end)) = sequence.last().unwrap().range() {
            return Err(Diagnostic {
                additional_diagnostics: vec![Diagnostic {
                    text: format!("Additional {token_kind} here"),
                    start: start.bytes(),
                    len: end.bytes() - start.bytes(),
                    file_id: doc_comment.file_id,
                    additional_diagnostics: vec![],
                }],
                ..doc_comment.diagnostic(format!(
                    "Assignments cannot have more than one {token_kind}"
                ))
            });
        }
    } else if sequence.is_empty() {
        return Err(doc_comment.diagnostic(format!("Assignments must have one {token_kind}")));
    }

    Ok(())
}

fn determine_kind_assignment(
    name: String,
    within: String,
    expression: &ast::Expression,
    doc_comment: &DocComment,
) -> Result<DocEntryKind, Diagnostic> {
    match expression {
        ast::Expression::Function(function_box) => {
            let function_body = &function_box.1;

            Ok(DocEntryKind::Function {
                name,
                within,
                function_type: FunctionType::Static,
                function_source: Some(function_body.clone().into()),
            })
        }
        ast::Expression::TypeAssertion { type_assertion, .. } => {
            let type_info = type_assertion.cast_to();
            match type_info {
                ast::luau::TypeInfo::Callback { .. } => {
                    Ok(DocEntryKind::Function {
                        name,
                        within,
                        function_type: FunctionType::Static,
                        function_source: match type_info.try_into() {
                            Ok(function_source) => Some(function_source),
                            Err(index) => return Err(doc_comment.diagnostic(format!(
                                "Type assertion must contain names for all parameters; type #{} is missing a name.",
                                index + 1
                            )))}
                    })
                }
                other => {
                    Ok(DocEntryKind::Property {
                        name,
                        within,
                        lua_type: Some(other.to_string()),
                    })
                }
            }
        }
        _ => {
            let lua_type = match expression {
                ast::Expression::InterpolatedString(_) => Some("string".to_string()),
                ast::Expression::TableConstructor(table) => Some(table.to_string()),
                ast::Expression::Number(_) => Some("number".to_string()),
                ast::Expression::String(_) => Some("string".to_string()),
                ast::Expression::Symbol(symbol) => match symbol.token().token_type() {
                    tokenizer::TokenType::Symbol { symbol } => match symbol {
                        tokenizer::Symbol::True => Some("boolean".to_string()),
                        tokenizer::Symbol::False => Some("boolean".to_string()),
                        tokenizer::Symbol::Nil => Some("nil".to_string()),
                        tokenizer::Symbol::Ellipsis => None,
                        _ => unreachable!(),
                    },
                    _ => unreachable!(),
                },
                _ => None,
            };

            let lua_type = match lua_type {
                Some(lua_type) => Some(lua_type.to_owned()),
                None => return Err(doc_comment.diagnostic("Expression must be a function, a string, a table, a number, `true`, `false`, `nil`, or a type assertion."))
            };

            Ok(DocEntryKind::Property {
                name,
                within,
                lua_type,
            })
        }
    }
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

    let within_tag = tags
        .iter()
        .find(|tag| matches!(tag, Tag::Within(_)))
        .map(|tag| {
            if let Tag::Within(within) = tag {
                within
            } else {
                unreachable!();
            }
        });

    match stmt {
        Some(Stmt::LocalFunction(function)) => {
            let name = function.name().to_string();

            let within = if let Some(within) = within_tag {
                within.name.as_str().to_owned()
            } else {
                return Err(doc_comment.diagnostic("Function requires @within tag"));
            };

            Ok(DocEntryKind::Function {
                name,
                within,
                function_type: FunctionType::Static,
                function_source: Some(function.body().clone().into()),
            })
        }
        Some(Stmt::FunctionDeclaration(function)) => match function.name().method_name() {
            Some(method_name) => {
                let within = if let Some(within) = within_tag {
                    within.name.as_str().to_owned()
                } else {
                    function.name().names().to_string()
                };

                Ok(DocEntryKind::Function {
                    name: method_name.to_string(),
                    within,
                    function_type: FunctionType::Method,
                    function_source: Some(function.body().clone().into()),
                })
            }
            None => {
                let mut names: Vec<_> = function.name().names().iter().collect();

                let function_name = names.pop().unwrap().to_string();

                let within = if let Some(within) = within_tag {
                    within.name.as_str().to_owned()
                } else if !names.is_empty() {
                    names
                        .into_iter()
                        .map(|token| token.to_string())
                        .collect::<Vec<_>>()
                        .join(".")
                } else {
                    return Err(doc_comment.diagnostic("Function requires @within tag"));
                };

                Ok(DocEntryKind::Function {
                    name: function_name,
                    within,
                    function_type: FunctionType::Static,
                    function_source: Some(function.body().clone().into()),
                })
            }
        },
        Some(Stmt::LocalAssignment(assignment)) => {
            let expressions = assignment.expressions();
            let variables = assignment.names();

            assert_token_sequence_is_single(variables, doc_comment, "variable")?;
            assert_token_sequence_is_single(expressions, doc_comment, "expression")?;

            let expression = expressions.into_iter().next().unwrap();

            let name = variables.first().unwrap().value().token().to_string();

            let within = if let Some(within) = within_tag {
                within.name.as_str().to_owned()
            } else {
                return Err(doc_comment.diagnostic("Local assignment requires @within tag."));
            };

            determine_kind_assignment(name, within, expression, doc_comment)
        }
        Some(Stmt::Assignment(assignment)) => {
            let expressions = assignment.expressions();
            let variables = assignment.variables();

            assert_token_sequence_is_single(variables, doc_comment, "variable")?;
            assert_token_sequence_is_single(expressions, doc_comment, "expression")?;

            let expression = expressions.into_iter().next().unwrap();

            let mut parts = match variables.into_iter().next().unwrap() {
                ast::Var::Name(token) => vec![token.token().to_string()],
                ast::Var::Expression(var_expression) => {
                    let mut parts = Vec::new();

                    match var_expression.prefix() {
                        ast::Prefix::Name(token) => parts.push(token.token().to_string()),
                        _ => (),
                    };

                    for suffix in var_expression.suffixes() {
                        match suffix {
                            ast::Suffix::Index(index) => match index {
                                ast::Index::Brackets {
                                    expression: ast::Expression::String(string),
                                    ..
                                } => parts.push(string.token().to_string()),
                                ast::Index::Dot { name, .. } => {
                                    parts.push(name.token().to_string())
                                }
                                _ => (),
                            },
                            _ => (),
                        }
                    }

                    parts
                }
                _ => Vec::new(),
            };

            let name = parts.pop();

            let within = if let Some(within) = within_tag {
                within.name.as_str().to_owned()
            } else if !parts.is_empty() {
                parts.join(".")
            } else {
                return Err(
                    doc_comment.diagnostic("Assignment requires @within tag or a qualified name.")
                );
            };

            let name = match name {
                Some(name) => name,
                None => {
                    return Err(doc_comment.diagnostic(
                        "Explicitly specify a kind tag, like @function, @prop, or @class.",
                    ))
                }
            };

            determine_kind_assignment(name, within, expression, doc_comment)
        }

        _ => Err(doc_comment
            .diagnostic("Explicitly specify a kind tag, like @function, @prop, or @class.")),
    }
}

impl<'a> DocEntry<'a> {
    pub fn parse(doc_comment: &'a DocComment) -> Result<(DocEntry<'a>, Vec<Tag<'a>>), Diagnostics> {
        let stmt = doc_comment.stmt.as_ref();

        let span: Span<'a> = doc_comment.into();

        let mut lines = span.lines();

        let first_line = lines.next();

        let mut indentation = None;

        if let Some(first_line) = first_line {
            if first_line.starts_with("---") {
                if first_line.len() == 3 {
                    return Err(Diagnostics::from(vec![span.diagnostic(
                        "The first line of a doc comment must have text after the triple dash",
                    )]));
                }

                indentation = Some(&first_line.as_str()[..4]);
            } else if first_line.contains(|char: char| !char.is_whitespace()) {
                return Err(Diagnostics::from(vec![
                    span.diagnostic("There must be a new line after --[=[")
                ]));
            }
        }

        let indentation = indentation.unwrap_or_else(|| {
            lines
                .find(|span| span.contains(|char: char| !char.is_whitespace()))
                .map(|span| span.as_str())
                .and_then(|str| {
                    let first_non_whitespace = str.find(|char: char| !char.is_whitespace())?;

                    Some(&str[..first_non_whitespace])
                })
                .unwrap_or("")
        });

        if !span.lines().all(|span| {
            span.is_empty()
                || span.starts_with(indentation)
                || span.as_str() == "---"
                || span.chars().all(char::is_whitespace)
        }) {
            return Err(Diagnostics::from(vec![span.diagnostic(
                "This doc comment has mixed indentation. \
                All lines within the doc comment must start with the same indentation. \
                Try using your editor's \"Convert Indentation to Tabs\" code action.",
            )]));
        }

        let (tag_lines, desc_lines): (Vec<Span>, Vec<Span>) = span
            .lines()
            .filter(|span| span.as_str() != "---")
            .map(|span| span.strip_prefix(indentation).unwrap_or(span))
            .partition(|line| line.starts_with(&['@', '.'][..]));

        let mut desc_lines = desc_lines
            .into_iter()
            .skip_while(|line| line.is_empty())
            .map(|span| span.as_str())
            .collect::<Vec<_>>();

        while let Some(line) = desc_lines.last() {
            if !line.is_empty() {
                break;
            }

            desc_lines.pop();
        }

        let desc = desc_lines.join("\n");

        let (tags, errors): (Vec<_>, Vec<_>) = tag_lines
            .into_iter()
            .map(Tag::try_from)
            .partition(Result::is_ok);

        let mut tags: Vec<_> = tags.into_iter().map(Result::unwrap).collect();
        let mut errors: Vec<_> = errors.into_iter().map(Result::unwrap_err).collect();

        let all_tags: Vec<Tag> = tags.clone();

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
                Tag::Function(_) | Tag::Within(_) | Tag::Class(_) | Tag::Interface(_)
            )
        });

        Ok(match kind {
            DocEntryKind::Function {
                within,
                name,
                function_type,
                function_source,
            } => (
                DocEntry::Function(FunctionDocEntry::parse(
                    DocEntryParseArguments {
                        within: Some(within),
                        name,
                        desc,
                        tags,
                        source: doc_comment,
                    },
                    function_type,
                    function_source,
                )?),
                all_tags,
            ),
            DocEntryKind::Property {
                within,
                name,
                lua_type,
            } => (
                DocEntry::Property(PropertyDocEntry::parse(
                    DocEntryParseArguments {
                        within: Some(within),
                        name,
                        desc,
                        tags,
                        source: doc_comment,
                    },
                    lua_type,
                )?),
                all_tags,
            ),
            DocEntryKind::Type { within, name } => (
                DocEntry::Type(TypeDocEntry::parse(DocEntryParseArguments {
                    within: Some(within),
                    name,
                    desc,
                    tags,
                    source: doc_comment,
                })?),
                all_tags,
            ),
            DocEntryKind::Class { name } => (
                DocEntry::Class(ClassDocEntry::parse(DocEntryParseArguments {
                    within: None,
                    name,
                    desc,
                    tags,
                    source: doc_comment,
                })?),
                all_tags,
            ),
        })
    }
}
