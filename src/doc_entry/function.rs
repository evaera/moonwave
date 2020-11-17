use crate::{
    diagnostic::Diagnostics,
    tags::{ParamTag, Tag},
};

use super::DocEntryParseArguments;

/// Used to separate functions (called with a dot) from methods (called with a colon)
#[derive(Debug, PartialEq)]
pub enum FunctionType {
    Method,
    Function,
}

/// A DocEntry for a function or method.
#[derive(Debug, PartialEq)]
pub struct FunctionDocEntry<'a> {
    name: String,
    desc: String,
    within: String,
    params: Vec<ParamTag<'a>>,
    function_type: FunctionType,
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
