use crate::{
    parse_error::{ParseError, ParseErrors},
    tags::{KindTag, KindTagType, ParamTag, Tag, WithinTag},
};
use full_moon::ast::Stmt;

/// Used to separate functions (called with a dot) from methods (called with a colon)
#[derive(Debug, PartialEq)]
pub enum FunctionType {
    Method,
    Function,
}

/// A DocEntry for a function or method.
#[derive(Debug, PartialEq)]
pub struct FunctionDocEntry {
    name: String,
    desc: String,
    within: String,
    params: Vec<ParamTag>,
    function_type: FunctionType,
}

impl FunctionDocEntry {
    fn parse(
        args: DocEntryParseArguments,
        function_type: FunctionType,
    ) -> Result<Self, ParseErrors> {
        let DocEntryParseArguments {
            name,
            desc,
            within,
            tags,
        } = args;

        let within = within.unwrap();
        let mut params = Vec::new();

        for tag in tags {
            match tag {
                Tag::Param(param) => params.push(param),
                Tag::Kind(_) => unreachable!(),
                Tag::Within(_) => unreachable!(),
            }
        }

        Ok(Self {
            name,
            desc,
            params,
            function_type,
            within,
        })
    }
}

/// A DocEntry for a property of a class
#[derive(Debug, PartialEq)]
pub struct PropertyDocEntry {
    name: String,
    desc: String,
    lua_type: String,
}

impl PropertyDocEntry {
    fn parse(args: DocEntryParseArguments) -> Result<Self, ParseErrors> {
        unimplemented!()
    }
}

/// A DocEntry for a class which contains functions, properties, and types
#[derive(Debug, PartialEq)]
pub struct ClassDocEntry {
    name: String,
    desc: String,
}

impl ClassDocEntry {
    fn parse(args: DocEntryParseArguments) -> Result<Self, ParseErrors> {
        unimplemented!()
    }
}

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
    Class {
        name: String,
    },
}

/// An enum of all possible DocEntries
#[derive(Debug)]
pub enum DocEntry {
    Function(FunctionDocEntry),
    Property(PropertyDocEntry),
    Class(ClassDocEntry),
}

#[derive(Debug)]
struct DocEntryParseArguments {
    name: String,
    tags: Vec<Tag>,
    desc: String,
    within: Option<String>,
}

fn get_explicit_kind(tags: &[Tag]) -> Result<Option<DocEntryKind>, ParseError> {
    let kind_tags = tags
        .iter()
        .filter(|t| matches!(**t, Tag::Kind(_)))
        .collect::<Vec<_>>();

    let within_tags = tags
        .iter()
        .filter(|t| matches!(**t, Tag::Within(_)))
        .collect::<Vec<_>>();

    if within_tags.len() > 1 {
        return Err(ParseError::new("Only one within tag is allowed"));
    }

    if kind_tags.is_empty() {
        return Ok(None);
    }

    if kind_tags.len() > 1 {
        return Err(ParseError::new("Only one kind tag is allowed"));
    }

    let the_kind_tag = kind_tags[0];

    match the_kind_tag {
        Tag::Kind(KindTag {
            tag_type: KindTagType::Class,
            name,
        }) => {
            if !within_tags.is_empty() {
                return Err(ParseError::new("Within tag is incompatible with class tag"));
            }

            Ok(Some(DocEntryKind::Class {
                name: name.to_owned(),
            }))
        }
        Tag::Kind(KindTag { tag_type, name }) => {
            if within_tags.is_empty() {
                return Err(ParseError::new(
                    "Must include within tag when using a kind tag",
                ));
            }

            let name = name.to_owned();
            let within = match within_tags[0] {
                Tag::Within(WithinTag { name }) => name.to_owned(),
                _ => unreachable!(),
            };

            match tag_type {
                KindTagType::Function => Ok(Some(DocEntryKind::Function {
                    name,
                    within,
                    function_type: FunctionType::Function,
                })),
                KindTagType::Property => Ok(Some(DocEntryKind::Property { name, within })),
                _ => panic!("Unhandled tag type {:?}", tag_type),
            }
        }
        _ => unreachable!(),
    }
}

fn determine_kind(stmt: &Stmt, tags: &[Tag]) -> Result<DocEntryKind, ParseError> {
    let explicit_kind = get_explicit_kind(tags)?;

    if let Some(kind) = explicit_kind {
        return Ok(kind);
    }

    match stmt {
        Stmt::FunctionDeclaration(function) => match function.name().method_name() {
            Some(method_name) => Ok(DocEntryKind::Function {
                name: method_name.to_string(),
                within: function.name().names().to_string(),
                function_type: FunctionType::Method,
            }),
            None => {
                let mut names: Vec<_> = function.name().names().iter().collect();

                let function_name = names.pop().unwrap().to_string();

                if names.is_empty() {
                    return Err(ParseError::new("Function requires @within tag"));
                }

                Ok(DocEntryKind::Function {
                    name: function_name,
                    within: names
                        .into_iter()
                        .map(|token| token.to_string())
                        .collect::<Vec<_>>()
                        .join("."),
                    function_type: FunctionType::Function,
                })
            }
        },

        Stmt::LocalAssignment(_) => unimplemented!(),
        Stmt::LocalFunction(_) => unimplemented!(),
        _ => Err(ParseError::new("Undeterminable doc entry")),
    }
}

impl DocEntry {
    pub fn parse(text: String, stmt: &Stmt) -> Result<Self, ParseErrors> {
        let (tag_lines, desc_lines): (Vec<&str>, Vec<&str>) = text
            .lines()
            .map(str::trim)
            .filter(|line| !line.is_empty())
            .partition(|line| line.starts_with('@'));

        let desc = desc_lines.join("\n");

        let (tags, errors): (Vec<_>, Vec<_>) = tag_lines
            .iter()
            .map(|line| line.parse::<Tag>())
            .partition(Result::is_ok);

        let mut tags: Vec<_> = tags.into_iter().map(Result::unwrap).collect();
        let errors: Vec<_> = errors.into_iter().map(Result::unwrap_err).collect();

        if !errors.is_empty() {
            return Err(ParseErrors::from(errors));
        }

        let kind = determine_kind(stmt, &tags).map_err(|err| ParseErrors::from(vec![err]))?;

        // Sift out the kind/within tags because those are only used for determining the kind
        tags.retain(|t| !matches!(t, Tag::Kind(_) | Tag::Within(_)));

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
                },
                function_type,
            )?),
            DocEntryKind::Property { within, name } => {
                DocEntry::Property(PropertyDocEntry::parse(DocEntryParseArguments {
                    within: Some(within),
                    name,
                    desc,
                    tags,
                })?)
            }
            DocEntryKind::Class { name } => {
                DocEntry::Class(ClassDocEntry::parse(DocEntryParseArguments {
                    within: None,
                    name,
                    desc,
                    tags,
                })?)
            }
        })
    }
}
