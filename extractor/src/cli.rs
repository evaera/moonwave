use std::path::PathBuf;

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
