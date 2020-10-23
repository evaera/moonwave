use crate::{
    parse_error::{ParseError, ParseErrors},
    tags::TagType,
};
use full_moon::ast::Stmt;
use std::convert::TryFrom;

#[derive(Debug)]
pub enum DocEntryKind {
    Method {
        name: String,
        within: String,
    },
    Function {
        name: String,
        within: Option<String>,
    },
    _Class {
        name: String,
    },
}

impl<'a> TryFrom<&'a Stmt<'a>> for DocEntryKind {
    type Error = anyhow::Error;
    fn try_from(stmt: &Stmt) -> anyhow::Result<Self> {
        match stmt {
            Stmt::FunctionDeclaration(function) => match function.name().method_name() {
                Some(method_name) => Ok(Self::Method {
                    name: method_name.to_string(),
                    within: function.name().names().to_string(),
                }),
                None => {
                    let mut names: Vec<_> = function.name().names().iter().collect();

                    Ok(Self::Function {
                        name: names.pop().unwrap().to_string(),
                        within: (!names.is_empty()).then(|| {
                            names
                                .into_iter()
                                .map(|token| token.to_string())
                                .collect::<Vec<_>>()
                                .join(".")
                        }),
                    })
                }
            },

            Stmt::LocalAssignment(_) => unimplemented!(),
            Stmt::LocalFunction(_) => unimplemented!(),
            _ => Err(anyhow::Error::msg("Invalid statement")),
        }
    }
}
#[derive(Debug)]
pub struct DocEntry {
    kind: DocEntryKind,
    desc: String,
    tags: Vec<TagType>,
}

fn get_kind_from_tags(_tags: &[TagType]) -> Result<DocEntryKind, ParseError> {
    unimplemented!();
}

impl DocEntry {
    pub fn parse(text: String, kind: Option<DocEntryKind>) -> Result<Self, ParseErrors> {
        let (tag_lines, desc_lines): (Vec<&str>, Vec<&str>) = text
            .lines()
            .map(str::trim)
            .filter(|line| !line.is_empty())
            .partition(|line| line.starts_with('@'));

        let desc = desc_lines.join("\n");

        let (tags, errors): (Vec<_>, Vec<_>) = tag_lines
            .iter()
            .map(|line| TagType::parse(line))
            .partition(Result::is_ok);

        let tags: Vec<_> = tags.into_iter().map(Result::unwrap).collect();
        let errors: Vec<_> = errors.into_iter().map(Result::unwrap_err).collect();

        if !errors.is_empty() {
            return Err(ParseErrors::from(errors));
        }

        let kind = match kind.map_or_else(|| get_kind_from_tags(&tags), Ok) {
            Ok(kind) => kind,
            Err(error) => return Err(ParseErrors::from(vec![error])),
        };

        let kind = match kind {
            DocEntryKind::Function { within: None, .. } => unimplemented!(),
            _ => kind,
        };

        Ok(Self { kind, desc, tags })
    }
}
