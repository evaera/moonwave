use crate::{diagnostic::Diagnostic, span::Span};
use serde::Serialize;

macro_rules! define_marker_tag {
    ( $struct_name:ident ) => {
        #[derive(Debug, PartialEq, Serialize)]
        pub struct $struct_name<'a> {
            #[serde(skip)]
            pub source: Span<'a>,
        }

        impl<'a> $struct_name<'a> {
            pub fn parse() -> Result<Self, Diagnostic> {
                Ok(Self {
                    source: Span::dummy(""),
                })
            }
        }
    };
}

define_marker_tag!(ServerTag);
define_marker_tag!(PluginTag);
define_marker_tag!(ClientTag);
define_marker_tag!(PrivateTag);
define_marker_tag!(IgnoreTag);
define_marker_tag!(YieldsTag);
define_marker_tag!(ReadOnlyTag);
define_marker_tag!(UnreleasedTag);

#[cfg(test)]
mod test {
    use insta::assert_yaml_snapshot;

    use super::*;

    #[test]
    fn snapshot() {
        assert_yaml_snapshot!(ServerTag::parse(), @r###"
        ---
        Ok: {}
        "###);

        assert_yaml_snapshot!(ServerTag::parse(), @r###"
        ---
        Ok: {}
        "###);
    }
}
