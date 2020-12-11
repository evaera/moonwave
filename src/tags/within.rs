use serde::Serialize;

use crate::{diagnostic::Diagnostic, span::Span};

#[derive(Debug, PartialEq, Serialize)]
pub struct WithinTag<'a> {
    pub name: Span<'a>,
    #[serde(skip)]
    pub source: Span<'a>,
}

impl<'a> WithinTag<'a> {
    pub fn parse(span: Span<'a>) -> Result<Self, Diagnostic> {
        if span.is_empty() {
            return Err(span.diagnostic("This tag has stuff after it"));
        }

        Ok(Self {
            name: span,
            source: span,
        })
    }
}

#[cfg(test)]
mod test {
    use insta::assert_yaml_snapshot;

    use super::*;

    #[test]
    fn snapshot() {
        assert_yaml_snapshot!(WithinTag::parse(Span::dummy("hey there")), @r###"
        ---
        Ok:
          name: hey there
        "###);

        assert_yaml_snapshot!(WithinTag::parse(Span::dummy("")), @r###"
        ---
        Err:
          text: This tag has stuff after it
          start: 0
          len: 0
          file_id: 0
          additional_diagnostics: []
        "###);
    }
}
