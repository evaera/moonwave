use crate::{diagnostic::Diagnostic, span::Span};
use serde::Serialize;
use std::convert::TryFrom;

mod custom;
mod kind;
mod marker;
mod param;
mod return_tag;
mod status;
mod validation;
mod within;

pub use custom::CustomTag;
pub use kind::{KindTag, KindTagType};
pub use marker::{MarkerTag, MarkerTagType};
pub use param::ParamTag;
pub use return_tag::ReturnTag;
pub use status::{DeprecatedTag, SinceTag};
pub use validation::validate_tags;
pub use within::WithinTag;

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
    Deprecated,
    Since,
    Custom,
    // Unimplemented
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
    Deprecated(DeprecatedTag<'a>),
    Since(SinceTag<'a>),
    Custom(CustomTag<'a>),
}

impl<'a> Tag<'a> {
    pub fn diagnostic(&self, text: &str) -> Diagnostic {
        match self {
            Tag::Param(tag) => tag.source.diagnostic(text),
            Tag::Return(tag) => tag.source.diagnostic(text),
            Tag::Kind(tag) => tag.source.diagnostic(text),
            Tag::Within(tag) => tag.source.diagnostic(text),
            Tag::Marker(tag) => tag.source.diagnostic(text),
            Tag::Deprecated(tag) => tag.source.diagnostic(text),
            Tag::Since(tag) => tag.source.diagnostic(text),
            Tag::Custom(tag) => tag.source.diagnostic(text),
        }
    }

    pub fn tag_type(&self) -> TagType {
        match self {
            Tag::Param(_) => TagType::Param,
            Tag::Return(_) => TagType::Return,
            Tag::Kind(KindTag { kind_type, .. }) => kind_type.tag_type(),
            Tag::Within(_) => TagType::Within,
            Tag::Marker(MarkerTag { marker_type, .. }) => marker_type.tag_type(),
            Tag::Deprecated(_) => TagType::Deprecated,
            Tag::Since(_) => TagType::Since,
            Tag::Custom(_) => TagType::Custom,
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
            Tag::Deprecated(tag) => tag.source.replace(span),
            Tag::Since(tag) => tag.source.replace(span),
            Tag::Custom(tag) => tag.source.replace(span),
        }
    }
}

impl<'a> TryFrom<Span<'a>> for Tag<'a> {
    type Error = Diagnostic;

    fn try_from(text: Span<'a>) -> Result<Self, Diagnostic> {
        let mut pieces = text.splitn(2, " ");

        let tag_name = pieces.next().unwrap().trim();

        let mut parsed_tag = match tag_name.as_str() {
            "@server" => MarkerTag::parse(MarkerTagType::Server).map(Tag::Marker),
            "@client" => MarkerTag::parse(MarkerTagType::Client).map(Tag::Marker),
            "@private" => MarkerTag::parse(MarkerTagType::Private).map(Tag::Marker),
            "@ignore" => MarkerTag::parse(MarkerTagType::Ignore).map(Tag::Marker),
            "@yields" => MarkerTag::parse(MarkerTagType::Yields).map(Tag::Marker),
            "@readonly" => MarkerTag::parse(MarkerTagType::ReadOnly).map(Tag::Marker),
            "@unreleased" => MarkerTag::parse(MarkerTagType::Unreleased).map(Tag::Marker),

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
                    "@deprecated" => DeprecatedTag::parse(tag_text).map(Tag::Deprecated),
                    "@since" => SinceTag::parse(tag_text).map(Tag::Since),
                    "@tag" => CustomTag::parse(tag_text).map(Tag::Custom),
                    _ => Err(text.diagnostic("Unknown tag")),
                }
            }
        }?;

        parsed_tag.blame(text);

        Ok(parsed_tag)
    }
}
