use crate::{diagnostic::Diagnostic, span::Span};
use std::convert::TryFrom;

mod kind;
mod param;
mod within;

pub use kind::KindTag;
pub use param::ParamTag;
pub use within::WithinTag;

pub use self::kind::KindTagType;

#[derive(Debug, PartialEq)]
pub enum Tag<'a> {
    Param(ParamTag<'a>),
    Kind(KindTag<'a>),
    Within(WithinTag<'a>),
}

impl<'a> Tag<'a> {
    pub fn diagnostic(&self, text: &str) -> Diagnostic {
        match self {
            Tag::Param(tag) => tag.source.diagnostic(text),
            Tag::Kind(tag) => tag.source.diagnostic(text),
            Tag::Within(tag) => tag.source.diagnostic(text),
        }
    }
}

impl<'a> TryFrom<Span<'a>> for Tag<'a> {
    type Error = Diagnostic;

    fn try_from(text: Span<'a>) -> Result<Self, Diagnostic> {
        let mut pieces = text.splitn(2, " ");

        let tag_name = pieces.next().unwrap().trim();

        // TODO: insert tags with no stuff here

        let tag_text = match pieces.next().map(Span::trim) {
            Some(span) => span,
            None => return Err(text.diagnostic("This tag requires text following it")),
        };

        match tag_name.as_str() {
            "@param" => ParamTag::try_from(tag_text).map(Tag::Param),
            "@within" => WithinTag::try_from(tag_text).map(Tag::Within),
            "@prop" => KindTag::parse(tag_text, KindTagType::Property).map(Tag::Kind),
            "@class" => KindTag::parse(tag_text, KindTagType::Class).map(Tag::Kind),
            "@function" => KindTag::parse(tag_text, KindTagType::Function).map(Tag::Kind),
            _ => Err(text.diagnostic("Unknown tag")),
        }
    }
}
