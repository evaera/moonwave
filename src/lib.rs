use codespan_reporting::{
    diagnostic::Diagnostic as CodeSpanDiagnostic,
    files::SimpleFiles,
    term::{
        self,
        termcolor::{ColorChoice, StandardStream},
    },
};

use diagnostic::{Diagnostic, Diagnostics};
use doc_entry::{DocEntry, FunctionDocEntry, PropertyDocEntry, TypeDocEntry};
use serde::Serialize;
use std::{collections::HashMap, fs, io, path::Path};
use walkdir::{self, WalkDir};

mod cli;
mod diagnostic;
mod doc_comment;
mod doc_entry;
pub mod error;
pub mod source_file;
mod span;
mod tags;

pub use cli::*;

use error::Error;
use source_file::SourceFile;

#[derive(Debug, Serialize, Default)]
struct OutputClass<'a> {
    name: String,
    desc: String,
    functions: Vec<FunctionDocEntry<'a>>,
    properties: Vec<PropertyDocEntry<'a>>,
    types: Vec<TypeDocEntry<'a>>,
}

pub fn generate_docs_from_path(path: &Path) -> anyhow::Result<()> {
    let (codespan_files, file_ids) = find_files(path)?;

    let mut errors: Vec<Error> = Vec::new();
    let mut source_files: Vec<SourceFile> = Vec::new();

    for file_id in file_ids {
        let source = codespan_files.get(file_id).unwrap().source();

        match SourceFile::from_str(source, file_id) {
            Ok(source_file) => source_files.push(source_file),
            Err(error) => errors.push(error),
        }
    }

    let (entries, source_file_errors): (Vec<_>, Vec<_>) = source_files
        .iter()
        .map(SourceFile::parse)
        .partition(Result::is_ok);

    errors.extend(source_file_errors.into_iter().map(Result::unwrap_err));

    let entries: Vec<_> = entries.into_iter().map(Result::unwrap).flatten().collect();

    match into_classes(entries) {
        Ok(classes) => {
            if errors.is_empty() {
                println!("{}", serde_json::to_string_pretty(&classes)?);
            }
        }
        Err(diagnostics) => errors.push(Error::ParseErrors(diagnostics)),
    }

    if !errors.is_empty() {
        report_errors(errors, &codespan_files);
    }

    Ok(())
}

fn into_classes<'a>(entries: Vec<DocEntry<'a>>) -> Result<Vec<OutputClass<'a>>, Diagnostics> {
    let mut map: HashMap<String, OutputClass<'a>> = HashMap::new();

    for entry in &entries {
        if let DocEntry::Class(class) = entry {
            map.insert(
                class.name.to_owned(),
                OutputClass {
                    name: class.name.to_owned(),
                    desc: class.desc.to_owned(),
                    ..Default::default()
                },
            );
        }
    }

    let mut diagnostics: Vec<Diagnostic> = Vec::new();

    for entry in entries {
        match entry {
            DocEntry::Function(entry) => {
                if let Some(class) = map.get_mut(&entry.within) {
                    class.functions.push(entry)
                } else {
                    diagnostics.push(entry.source.diagnostic(format!(
                        "This function's parent class \"{}\" is missing a doc entry",
                        &entry.within
                    )))
                }
            }
            DocEntry::Property(_) => {}
            DocEntry::Type(_) => {}
            DocEntry::Class(_) => {}
        }
    }

    if diagnostics.is_empty() {
        Ok(map.into_iter().map(|(_, value)| value).collect())
    } else {
        Err(Diagnostics::from(diagnostics))
    }
}

fn find_files(path: &Path) -> Result<(SimpleFiles<String, String>, Vec<usize>), io::Error> {
    let mut codespan_files = SimpleFiles::new();
    let mut file_ids: Vec<usize> = Vec::new();

    let walker = WalkDir::new(path).follow_links(true).into_iter();
    for entry in walker
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .filter(|e| e.file_name().to_string_lossy().ends_with(".lua"))
    {
        let path = entry.path();
        let contents = fs::read_to_string(path)?;

        let file_id = codespan_files.add(path.to_string_lossy().to_string(), contents);

        file_ids.push(file_id);
    }

    Ok((codespan_files, file_ids))
}

fn report_errors(errors: Vec<Error>, codespan_files: &SimpleFiles<String, String>) {
    let writer = StandardStream::stderr(ColorChoice::Always);
    let config = codespan_reporting::term::Config::default();

    for error in errors {
        match error {
            Error::ParseErrors(diagnostics) => {
                for diagnostic in diagnostics.into_iter() {
                    term::emit(
                        &mut writer.lock(),
                        &config,
                        codespan_files,
                        &CodeSpanDiagnostic::from(diagnostic),
                    )
                    .unwrap()
                }
            }
            Error::FullMoonError(error) => eprintln!("{}", error),
        }
    }
}
