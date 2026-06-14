use std::path::PathBuf;

use codespan_reporting::{
    diagnostic::Diagnostic as CodeSpanDiagnostic,
    term::{
        self,
        termcolor::{ColorChoice, StandardStream},
    },
};
use libmoonwave::{error::Error, CodespanFiles};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(about = env!("CARGO_PKG_DESCRIPTION"))]
pub struct Args {
    #[structopt(subcommand)]
    pub subcommand: Subcommand,
}

#[derive(Debug, StructOpt)]
pub enum Subcommand {
    Extract(ExtractSubcommand),
}

/// Extracts doc comments from the given files
#[derive(Debug, StructOpt)]
pub struct ExtractSubcommand {
    pub input_path: Option<PathBuf>,

    /// The base path that source paths in the output will be relative to.
    /// If unspecified, the input path is used.
    #[structopt(long = "base", short = "b")]
    pub base_path: Option<PathBuf>,
}

pub fn report_errors(errors: Vec<Error>, codespan_files: &CodespanFiles) {
    let writer = StandardStream::stderr(ColorChoice::Auto);
    let config = codespan_reporting::term::Config {
        end_context_lines: usize::MAX,
        ..Default::default()
    };

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
            err => eprintln!("{err}"),
        }
    }
}
