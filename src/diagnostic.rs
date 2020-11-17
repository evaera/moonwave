use std::{error, fmt};

use codespan_reporting::diagnostic::{Diagnostic as CodeSpanDiagnostic, Label};

use crate::{doc_comment::DocComment, span::Span};

#[derive(Debug)]
pub struct Diagnostic {
    text: String,
    start: usize,
    len: usize,
    file_id: usize,
}

impl Diagnostic {
    pub fn from_span<S: Into<String>>(text: S, span: Span) -> Self {
        Self {
            text: text.into(),
            start: span.start + span.source_offset,
            len: span.len,
            file_id: span.file_id,
        }
    }

    pub fn from_doc_comment<S: Into<String>>(text: S, doc_comment: &DocComment) -> Self {
        Self {
            text: text.into(),
            start: doc_comment.start,
            len: 1, // This is arbitrary
            file_id: doc_comment.file_id,
        }
    }
}

impl error::Error for Diagnostic {}

impl fmt::Display for Diagnostic {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.text)
    }
}

impl From<Diagnostic> for CodeSpanDiagnostic<usize> {
    fn from(diagnostic: Diagnostic) -> Self {
        CodeSpanDiagnostic::error()
            .with_message(&diagnostic.text)
            .with_labels(vec![Label::primary(
                diagnostic.file_id,
                diagnostic.start..(diagnostic.start + diagnostic.len),
            )
            .with_message(&diagnostic.text)])
    }
}

#[derive(Debug)]
pub struct Diagnostics {
    errors: Vec<Diagnostic>,
}

impl Diagnostics {
    pub fn into_iter(self) -> impl Iterator<Item = Diagnostic> {
        self.errors.into_iter()
    }

    pub fn is_empty(&self) -> bool {
        self.errors.is_empty()
    }
}

impl From<Vec<Diagnostic>> for Diagnostics {
    fn from(errors: Vec<Diagnostic>) -> Self {
        Self { errors }
    }
}

impl error::Error for Diagnostics {}
impl fmt::Display for Diagnostics {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let text = self
            .errors
            .iter()
            .map(|e| e.to_string())
            .collect::<Vec<String>>()
            .join("\n");

        write!(f, "{}", text)
    }
}
