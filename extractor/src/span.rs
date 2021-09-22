use std::{fmt, ops::Deref};

use crate::{diagnostic::Diagnostic, doc_comment::DocComment};

#[derive(Debug, Copy, Clone, Default)]
pub struct Span<'a> {
    source: &'a str,
    pub start: usize,
    pub len: usize,
    pub file_id: usize,
    pub source_offset: usize,
}

impl Span<'static> {
    pub fn empty(file_id: usize) -> Self {
        Span {
            source: "",
            file_id,
            ..Default::default()
        }
    }
}

impl<'a> Span<'a> {
    pub fn dummy(source: &'a str) -> Self {
        Self {
            source,
            len: source.len(),
            ..Default::default()
        }
    }

    pub fn slice(&self, start: usize, len: usize) -> Self {
        Self {
            start: self.start + start,
            len,
            ..*self
        }
    }

    pub fn as_str(&self) -> &'a str {
        &self.source[self.start..self.start + self.len]
    }

    pub fn lines(self) -> impl Iterator<Item = Span<'a>> {
        self.as_str().lines().map(move |line| self.from_slice(line))
    }

    pub fn splitn(self, n: usize, pat: &'static str) -> impl Iterator<Item = Span<'a>> {
        self.as_str()
            .splitn(n, pat)
            .map(move |piece: &'a str| self.from_slice(piece))
    }

    pub fn trim(self) -> Self {
        self.from_slice(self.as_str().trim())
    }

    pub fn strip_prefix(self, prefix: &str) -> Option<Self> {
        Some(self.from_slice(self.as_str().strip_prefix(prefix)?))
    }

    pub fn diagnostic<S: Into<String>>(self, text: S) -> Diagnostic {
        Diagnostic::from_span(text, self)
    }

    pub fn replace(&mut self, span: Self) {
        *self = span // Is this legal?
    }

    fn from_slice(&self, text: &'a str) -> Self {
        let start = text.as_ptr() as usize - self.source.as_ptr() as usize;

        Span {
            start,
            len: text.len(),
            ..*self
        }
    }
}

impl PartialEq for Span<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.as_str() == other.as_str()
    }
}

impl fmt::Display for Span<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl serde::Serialize for Span<'_> {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl Deref for Span<'_> {
    type Target = str;
    fn deref(&self) -> &str {
        self.as_str()
    }
}

impl<'a> From<&'a DocComment> for Span<'a> {
    fn from(doc: &'a DocComment) -> Self {
        Span {
            source: &doc.comment,
            len: doc.comment.len(),
            file_id: doc.file_id,
            source_offset: doc.start,
            ..Default::default()
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn correct_deref() {
        let text = "abcdef";
        let span = Span::dummy(text);

        let deref_check: &str = &span;
        assert_eq!(deref_check, text);

        let slice = span.slice(1, 3);
        let deref_slice_check: &str = &slice;
        assert_eq!(deref_slice_check, "bcd");

        let slice = slice.slice(1, 2);
        let deref_slice_check: &str = &slice;
        assert_eq!(deref_slice_check, "cd");
    }

    #[test]
    fn lines() {
        let text = "hello\nworld!\nipsum";
        let span = Span::dummy(text);

        let lines: Vec<_> = span.lines().map(|line| line.as_str()).collect();
        assert_eq!(lines, &["hello", "world!", "ipsum"]);
    }

    #[test]
    fn trim() {
        let text = "    hello       ";
        let span = Span::dummy(text);

        assert_eq!(span.trim().as_str(), "hello");
    }
}
