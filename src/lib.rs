use codespan_reporting::{
    diagnostic::Diagnostic as CodeSpanDiagnostic,
    files::SimpleFiles,
    term::{
        self,
        termcolor::{ColorChoice, StandardStream},
    },
};

use std::{fs, path::Path};
use walkdir::{self, WalkDir};

mod cli;
mod diagnostic;
mod doc_comment;
mod doc_entry;
mod error;
pub mod source_file;
mod span;
mod tags;

pub use cli::*;

use error::Error;
use source_file::SourceFile;

pub fn generate_docs_from_path(path: &Path) -> Result<(), Error> {
    let mut codespan_files = SimpleFiles::new();
    let mut file_ids: Vec<usize> = Vec::new();

    let walker = WalkDir::new(path).follow_links(true).into_iter();
    for entry in walker
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .filter(|e| e.file_name().to_string_lossy().ends_with(".lua"))
    {
        let path = entry.path();
        let contents = fs::read_to_string(path).map_err(Error::ReadError)?;

        let file_id = codespan_files.add(path.to_string_lossy().to_string(), contents);

        file_ids.push(file_id);
    }

    let mut source_files: Vec<SourceFile> = Vec::new();

    for file_id in file_ids {
        let source = codespan_files.get(file_id).unwrap().source();

        source_files.push(SourceFile::from_str(source, file_id)?);
    }

    let (entries, errors): (Vec<_>, Vec<_>) = source_files
        .iter()
        .map(SourceFile::parse)
        .partition(Result::is_ok);

    let errors: Vec<_> = errors.into_iter().map(Result::unwrap_err).collect();

    let writer = StandardStream::stderr(ColorChoice::Always);
    let config = codespan_reporting::term::Config::default();

    for error in errors {
        match error {
            Error::ParseErrors(diagnostics) => {
                for diagnostic in diagnostics.into_iter() {
                    term::emit(
                        &mut writer.lock(),
                        &config,
                        &codespan_files,
                        &CodeSpanDiagnostic::from(diagnostic),
                    )
                    .unwrap()
                }
            }
            Error::ReadError(error) => eprintln!("{}", error),
            Error::FullMoonError(error) => eprintln!("{}", error),
        }
    }

    let entries: Vec<_> = entries.into_iter().map(Result::unwrap).flatten().collect();

    dbg!(entries);

    Ok(())
}
