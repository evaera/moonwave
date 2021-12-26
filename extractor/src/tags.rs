use crate::{diagnostic::Diagnostic, doc_entry::FunctionType, span::Span};
use serde::Serialize;
use std::convert::TryFrom;

mod class;
mod custom;
mod error;
mod field;
mod function;
mod index;
mod interface;
mod marker;
mod param;
mod property;
mod return_tag;
mod status;
mod type_tag;
mod validation;
mod within;

pub use class::ClassTag;
pub use custom::CustomTag;
pub use error::ErrorTag;
pub use field::FieldTag;
pub use function::FunctionTag;
pub use index::IndexTag;
pub use interface::InterfaceTag;
pub use marker::{
    ClientTag, IgnoreTag, PluginTag, PrivateTag, ReadOnlyTag, ServerTag, UnreleasedTag, YieldsTag,
};
pub use param::ParamTag;
pub use property::PropertyTag;
pub use return_tag::ReturnTag;
pub use status::{DeprecatedTag, SinceTag};
pub use type_tag::TypeTag;
pub use validation::validate_tags;
pub use within::WithinTag;

macro_rules! define_tags {
    ( $( $variant_name:ident($struct_name:ident),)* ) => {
        #[derive(Debug, PartialEq, Serialize)]
        pub enum Tag<'a> {
            $( $variant_name($struct_name<'a>), )*
        }

        impl<'a> Tag<'a> {
            pub fn diagnostic(&self, text: &str) -> Diagnostic {
                match self {
                    $( Tag::$variant_name(tag) => tag.source.diagnostic(text), )*
                }
            }

            pub fn tag_type(&self) -> TagType {
                match self {
                    $( Tag::$variant_name(_) => TagType::$variant_name, )*
                }
            }

            /// Replaces the source span with a new span for error reporting clarity
            pub fn blame(&mut self, span: Span<'a>) {
                match self {
                    $( Tag::$variant_name(tag) => tag.source.replace(span), )*
                }
            }
        }

        #[allow(unused)]
        #[derive(Debug, PartialEq, Hash, Eq)]
        pub enum TagType {
            $( $variant_name, )*
        }
    };
}

define_tags! {
    Param(ParamTag),
    Function(FunctionTag),
    Property(PropertyTag),
    Class(ClassTag),
    Within(WithinTag),
    Type(TypeTag),
    Interface(InterfaceTag),
    Field(FieldTag),
    Unreleased(UnreleasedTag),
    Server(ServerTag),
    Client(ClientTag),
    Plugin(PluginTag),
    Private(PrivateTag),
    Ignore(IgnoreTag),
    Yields(YieldsTag),
    ReadOnly(ReadOnlyTag),
    Return(ReturnTag),
    Deprecated(DeprecatedTag),
    Since(SinceTag),
    Custom(CustomTag),
    Error(ErrorTag),
    Index(IndexTag),

    // Unimplemented:
    // External,
    // Link,
    // Enum,
}

impl<'a> TryFrom<Span<'a>> for Tag<'a> {
    type Error = Diagnostic;

    fn try_from(text: Span<'a>) -> Result<Self, Diagnostic> {
        if text.starts_with('.') {
            let mut parsed_tag = FieldTag::parse(text.slice(1, text.len() - 1)).map(Tag::Field)?;
            parsed_tag.blame(text);

            return Ok(parsed_tag);
        }

        let mut pieces = text.splitn(2, " ");

        let tag_name = pieces.next().unwrap().trim();

        let mut tag_text = || {
            pieces
                .next()
                .map(Span::trim)
                .ok_or_else(|| text.diagnostic("This tag requires text following it"))
        };

        let mut parsed_tag = match tag_name.as_str() {
            "@server" => ServerTag::parse().map(Tag::Server),
            "@client" => ClientTag::parse().map(Tag::Client),
            "@plugin" => PluginTag::parse().map(Tag::Plugin),
            "@private" => PrivateTag::parse().map(Tag::Private),
            "@ignore" => IgnoreTag::parse().map(Tag::Ignore),
            "@yields" => YieldsTag::parse().map(Tag::Yields),
            "@readonly" => ReadOnlyTag::parse().map(Tag::ReadOnly),
            "@unreleased" => UnreleasedTag::parse().map(Tag::Unreleased),

            "@param" => ParamTag::parse(tag_text()?).map(Tag::Param),
            "@return" => ReturnTag::parse(tag_text()?).map(Tag::Return),
            "@within" => WithinTag::parse(tag_text()?).map(Tag::Within),
            "@__index" => IndexTag::parse(tag_text()?).map(Tag::Index),
            "@type" => TypeTag::parse(tag_text()?).map(Tag::Type),
            "@interface" => InterfaceTag::parse(tag_text()?).map(Tag::Interface),
            "@field" => FieldTag::parse(tag_text()?).map(Tag::Field),
            "@prop" => PropertyTag::parse(tag_text()?).map(Tag::Property),
            "@class" => ClassTag::parse(tag_text()?).map(Tag::Class),
            "@function" => FunctionTag::parse(tag_text()?, FunctionType::Static).map(Tag::Function),
            "@method" => FunctionTag::parse(tag_text()?, FunctionType::Method).map(Tag::Function),
            "@deprecated" => DeprecatedTag::parse(tag_text()?).map(Tag::Deprecated),
            "@since" => SinceTag::parse(tag_text()?).map(Tag::Since),
            "@tag" => CustomTag::parse(tag_text()?).map(Tag::Custom),
            "@error" => ErrorTag::parse(tag_text()?).map(Tag::Error),
            _ => Err(text.diagnostic("Unknown tag")),
        }?;

        parsed_tag.blame(text);

        Ok(parsed_tag)
    }
}
