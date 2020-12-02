use crate::{diagnostic::Diagnostics, doc_comment::DocComment, tags::Tag};
use serde::Serialize;

use super::DocEntryParseArguments;

/// A DocEntry for a property of a class
#[derive(Debug, PartialEq, Serialize)]
pub struct PropertyDocEntry<'a> {
    pub name: String,
    pub desc: String,
    pub lua_type: String,
    pub within: String,
    #[serde(skip)]
    pub source: &'a DocComment,
    blah: Tag<'a>,
}

impl<'a> PropertyDocEntry<'a> {
    pub(super) fn parse(_args: DocEntryParseArguments) -> Result<Self, Diagnostics> {
        unimplemented!()
    }
}
