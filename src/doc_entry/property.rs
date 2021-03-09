use crate::{
    diagnostic::Diagnostics,
    doc_comment::DocComment,
    tags::{CustomTag, Tag},
};
use serde::Serialize;

use super::DocEntryParseArguments;

/// A DocEntry for a property of a class
#[derive(Debug, PartialEq, Serialize)]
pub struct PropertyDocEntry<'a> {
    pub name: String,
    pub desc: String,
    pub lua_type: String,
    pub within: String,
    pub tags: Vec<CustomTag<'a>>,
    #[serde(skip)]
    pub source: &'a DocComment,
}

impl<'a> PropertyDocEntry<'a> {
    pub(super) fn parse(_args: DocEntryParseArguments) -> Result<Self, Diagnostics> {
        todo!();
    }
}
