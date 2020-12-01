use std::{error, fmt};

use crate::{doc_comment::DocComment, span::Span};
use codespan_reporting::diagnostic::{Diagnostic as CodeSpanDiagnostic, Label};
use serde::Serialize;

#[derive(Debug, Default, Serialize)]
pub struct Diagnostic {
    pub text: String,
    start: usize,
    len: usize,
    file_id: usize,
    additional_diagnostics: Vec<Diagnostic>,
}

impl Diagnostic {
    pub fn from_span<S: Into<String>>(text: S, span: Span) -> Self {
        Self {
            text: text.into(),
            start: span.start + span.source_offset,
            len: span.len,
            file_id: span.file_id,
            ..Default::default()
        }
    }

    pub fn from_doc_comment<S: Into<String>>(text: S, doc_comment: &DocComment) -> Self {
        Self {
            text: text.into(),
            start: doc_comment.start,
            len: doc_comment.comment.len(),
            file_id: doc_comment.file_id,
            ..Default::default()
        }
    }

    pub fn attach_diagnostic(&mut self, diagnostic: Diagnostic) -> &Self {
        self.additional_diagnostics.push(diagnostic);

        self
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
        let mut labels = vec![Label::primary(
            diagnostic.file_id,
            diagnostic.start..(diagnostic.start + diagnostic.len),
        )
        .with_message(&diagnostic.text)];

        for additional_diagnostic in diagnostic.additional_diagnostics {
            labels.push(
                Label::secondary(
                    additional_diagnostic.file_id,
                    additional_diagnostic.start
                        ..(additional_diagnostic.start + additional_diagnostic.len),
                )
                .with_message(&additional_diagnostic.text),
            )
        }

        CodeSpanDiagnostic::error()
            .with_message(&diagnostic.text)
            .with_labels(labels)
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
