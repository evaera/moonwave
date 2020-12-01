use std::fmt;

use crate::diagnostic::Diagnostics;

#[derive(Debug)]
#[non_exhaustive]
pub enum Error {
    ParseErrors(Diagnostics),
    FullMoonError(String),
}

impl fmt::Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::ParseErrors(parse_error) => write!(formatter, "{}", parse_error),
            Error::FullMoonError(full_moon_error) => write!(formatter, "{}", full_moon_error),
        }
    }
}

impl std::error::Error for Error {}
