use std::fmt;

use crate::diagnostic::Diagnostics;

#[derive(Debug)]
#[non_exhaustive]
pub enum Error {
    ParseErrors(Diagnostics),
    FullMoonError(Vec<(String, full_moon::Error)>),
}

impl fmt::Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::ParseErrors(parse_error) => write!(formatter, "{}", parse_error),
            Error::FullMoonError(full_moon_errors) => {
                let text = full_moon_errors
                    .iter()
                    .map(|(s, e)| format!("Full-Moon: {}\n    in {}", e, s))
                    .collect::<Vec<String>>()
                    .join("\n");

                write!(formatter, "{}", text)
            }
        }
    }
}

impl std::error::Error for Error {}
