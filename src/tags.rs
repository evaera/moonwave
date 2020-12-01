use crate::{diagnostic::Diagnostic, span::Span};
use serde::Serialize;
use std::convert::TryFrom;

mod kind;
mod param;
mod validation;
mod within;

pub use kind::KindTag;
pub use param::ParamTag;
pub use validation::validate_tags;
pub use within::WithinTag;

pub use self::kind::KindTagType;

#[allow(unused)]
#[derive(Debug, PartialEq, Hash, Eq)]
pub enum TagType {
    Param,
    Property,
    Function,
    Class,
    Within,
    Type,
    // Unimplemented
    Return,
    Tag,
    Deprecated,
    Since,
    Unreleased,
    Server,
    Client,
    Private,
    Ignore,
    Error,
    Yields,
    ReadOnly,
    Field,
    External,
    Link,
    Interface,
    Enum,
}

#[derive(Debug, PartialEq, Serialize)]
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

    pub fn tag_type(&self) -> TagType {
        match self {
            Tag::Param(_) => TagType::Param,
            Tag::Kind(KindTag {
                kind_type: KindTagType::Function,
                ..
            }) => TagType::Function,
            Tag::Kind(KindTag {
                kind_type: KindTagType::Property,
                ..
            }) => TagType::Property,
            Tag::Kind(KindTag {
                kind_type: KindTagType::Class,
                ..
            }) => TagType::Property,
            Tag::Kind(KindTag {
                kind_type: KindTagType::Type,
                ..
            }) => TagType::Type,
            Tag::Within(_) => TagType::Within,
        }
    }

    /// Replaces the source span with a new span for error reporting clarity
    pub fn blame(&mut self, span: Span<'a>) {
        match self {
            Tag::Param(tag) => tag.source.replace(span),
            Tag::Kind(tag) => tag.source.replace(span),
            Tag::Within(tag) => tag.source.replace(span),
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

        let mut parsed_tag = match tag_name.as_str() {
            "@param" => ParamTag::try_from(tag_text).map(Tag::Param),
            "@within" => WithinTag::try_from(tag_text).map(Tag::Within),
            "@prop" => KindTag::parse(tag_text, KindTagType::Property).map(Tag::Kind),
            "@class" => KindTag::parse(tag_text, KindTagType::Class).map(Tag::Kind),
            "@function" => KindTag::parse(tag_text, KindTagType::Function).map(Tag::Kind),
            _ => Err(text.diagnostic("Unknown tag")),
        }?;

        parsed_tag.blame(text);

        Ok(parsed_tag)
    }
}
