use anyhow::Context;
use cli::{report_errors, Args, Subcommand};
use libmoonwave::{find_files, generate_docs_from_sources, parse_source_files_at_path};
use std::env::current_dir;
use structopt::StructOpt;

mod cli;

fn run(args: Args) -> anyhow::Result<()> {
    match args.subcommand {
        Subcommand::Extract(subcommand) => {
            let path = match subcommand.input_path {
                Some(path) => path,
                None => current_dir()?,
            };

            let base_path = match subcommand.base_path {
                Some(path) => path,
                None => path.clone(),
            };

            let (codespan_files, files) =
                find_files(&path).context("failed to find source files")?;

            let result = parse_source_files_at_path(&codespan_files, &files, &base_path);
            let result = if let Ok(source_files) = &result {
                generate_docs_from_sources(source_files)
            } else {
                Err(result.unwrap_err())
            };

            match result {
                Ok(classes) => println!(
                    "{}",
                    serde_json::to_string_pretty(&classes)
                        .context("failed to serialize classes to JSON")?
                ),
                Err(errors) => {
                    let count_errors = errors.len();
                    report_errors(errors, &codespan_files);
                    if count_errors == 1 {
                        anyhow::bail!("aborting due to diagnostic error");
                    } else {
                        anyhow::bail!("aborting due to {} diagnostic errors", count_errors);
                    }
                }
            }
        }
    }

    Ok(())
}

fn main() {
    let args = Args::from_args();

    if let Err(error) = run(args) {
        eprintln!("error: {}", error);
        std::process::exit(1);
    }
}
