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
    /// Extracts doc comments from the given files
    Extract(ExtractSubcommand),
}

#[derive(Debug, StructOpt)]
pub struct ExtractSubcommand {
    pub input_path: Option<PathBuf>,
}
