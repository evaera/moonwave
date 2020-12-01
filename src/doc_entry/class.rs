use crate::{diagnostic::Diagnostics, tags::Tag};
use serde::Serialize;

use super::DocEntryParseArguments;

/// A DocEntry for a class which contains functions, properties, and types
#[derive(Debug, PartialEq, Serialize)]
pub struct ClassDocEntry<'a> {
    pub name: String,
    pub desc: String,
    blah: Tag<'a>,
}

impl<'a> ClassDocEntry<'a> {
    pub(super) fn parse(_args: DocEntryParseArguments) -> Result<Self, Diagnostics> {
        unimplemented!();
    }
}
