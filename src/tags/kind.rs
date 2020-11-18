use crate::{diagnostic::Diagnostic, span::Span};

#[derive(Debug, PartialEq)]
pub enum KindTagType {
    Function,
    Property,
    Class,
}

#[derive(Debug, PartialEq)]
pub struct KindTag<'a> {
    pub name: Span<'a>,
    pub kind_type: KindTagType,
    pub source: Span<'a>,
}

impl<'a> KindTag<'a> {
    pub fn parse(text: Span<'a>, tag_type: KindTagType) -> Result<Self, Diagnostic> {
        if text.is_empty() {
            return Err(text.diagnostic("This kind tag has stuff after it"));
        }

        Ok(Self {
            name: text,
            kind_type: tag_type,
            source: text,
        })
    }
}
