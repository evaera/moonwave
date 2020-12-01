use crate::{
    diagnostic::Diagnostics,
    doc_comment::DocComment,
    tags::{ParamTag, Tag},
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
    pub function_type: FunctionType,
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
            source,
        })
    }
}
