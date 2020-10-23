use std::error;
use std::fmt;

#[derive(Debug)]
pub struct ParseError {
    text: String,
}

impl ParseError {
    pub fn new<S: Into<String>>(text: S) -> Self {
        Self { text: text.into() }
    }
}

impl error::Error for ParseError {}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.text)
    }
}
#[derive(Debug)]
pub struct ParseErrors {
    errors: Vec<ParseError>,
}

impl From<Vec<ParseError>> for ParseErrors {
    fn from(errors: Vec<ParseError>) -> Self {
        Self { errors }
    }
}

impl error::Error for ParseErrors {}
impl fmt::Display for ParseErrors {
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
