use crate::{diagnostic::Diagnostic, span::Span};
use serde::Serialize;
use std::convert::TryFrom;

mod kind;
mod marker;
mod param;
mod return_tag;
mod validation;
mod within;

pub use kind::KindTag;
pub use marker::MarkerTag;
pub use param::ParamTag;
pub use return_tag::ReturnTag;
pub use validation::validate_tags;
pub use within::WithinTag;

pub use self::kind::KindTagType;
use self::marker::MarkerTagType;

#[allow(unused)]
#[derive(Debug, PartialEq, Hash, Eq)]
pub enum TagType {
    Param,
    Property,
    Function,
    Class,
    Within,
    Type,
    Unreleased,
    Server,
    Client,
    Private,
    Ignore,
    Yields,
    ReadOnly,
    Return,
    // Unimplemented
    Deprecated,
    Since,
    Tag,
    Error,
    Field,
    External,
    Link,
    Interface,
    Enum,
}

#[derive(Debug, PartialEq, Serialize)]
pub enum Tag<'a> {
    Param(ParamTag<'a>),
    Return(ReturnTag<'a>),
    Kind(KindTag<'a>),
    Within(WithinTag<'a>),
    Marker(MarkerTag<'a>),
}

impl<'a> Tag<'a> {
    pub fn diagnostic(&self, text: &str) -> Diagnostic {
        match self {
            Tag::Param(tag) => tag.source.diagnostic(text),
            Tag::Return(tag) => tag.source.diagnostic(text),
            Tag::Kind(tag) => tag.source.diagnostic(text),
            Tag::Within(tag) => tag.source.diagnostic(text),
            Tag::Marker(tag) => tag.source.diagnostic(text),
        }
    }

    pub fn tag_type(&self) -> TagType {
        match self {
            Tag::Param(_) => TagType::Param,
            Tag::Return(_) => TagType::Return,
            Tag::Kind(KindTag { kind_type, .. }) => kind_type.tag_type(),
            Tag::Within(_) => TagType::Within,
            Tag::Marker(MarkerTag { marker_type, .. }) => marker_type.tag_type(),
        }
    }

    /// Replaces the source span with a new span for error reporting clarity
    pub fn blame(&mut self, span: Span<'a>) {
        match self {
            Tag::Param(tag) => tag.source.replace(span),
            Tag::Return(tag) => tag.source.replace(span),
            Tag::Kind(tag) => tag.source.replace(span),
            Tag::Within(tag) => tag.source.replace(span),
            Tag::Marker(tag) => tag.source.replace(span),
        }
    }
}

impl<'a> TryFrom<Span<'a>> for Tag<'a> {
    type Error = Diagnostic;

    fn try_from(text: Span<'a>) -> Result<Self, Diagnostic> {
        let mut pieces = text.splitn(2, " ");

        let tag_name = pieces.next().unwrap().trim();

        let mut parsed_tag = match tag_name.as_str() {
            "@unreleased" => MarkerTag::parse(MarkerTagType::Unreleased).map(Tag::Marker),
            "@server" => MarkerTag::parse(MarkerTagType::Server).map(Tag::Marker),
            "@client" => MarkerTag::parse(MarkerTagType::Client).map(Tag::Marker),
            "@private" => MarkerTag::parse(MarkerTagType::Private).map(Tag::Marker),
            "@ignore" => MarkerTag::parse(MarkerTagType::Ignore).map(Tag::Marker),
            "@yields" => MarkerTag::parse(MarkerTagType::Yields).map(Tag::Marker),
            "@readonly" => MarkerTag::parse(MarkerTagType::ReadOnly).map(Tag::Marker),

            _ => {
                let tag_text = match pieces.next().map(Span::trim) {
                    Some(span) => span,
                    None => return Err(text.diagnostic("This tag requires text following it")),
                };

                match tag_name.as_str() {
                    "@param" => ParamTag::parse(tag_text).map(Tag::Param),
                    "@return" => ReturnTag::parse(tag_text).map(Tag::Return),
                    "@within" => WithinTag::parse(tag_text).map(Tag::Within),
                    "@prop" => KindTag::parse(tag_text, KindTagType::Property).map(Tag::Kind),
                    "@type" => KindTag::parse(tag_text, KindTagType::Type).map(Tag::Kind),
                    "@class" => KindTag::parse(tag_text, KindTagType::Class).map(Tag::Kind),
                    "@function" => KindTag::parse(tag_text, KindTagType::Function).map(Tag::Kind),
                    _ => Err(text.diagnostic("Unknown tag")),
                }
            }
        }?;

        parsed_tag.blame(text);

        Ok(parsed_tag)
    }
}
