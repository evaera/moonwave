use libmoonwave::{generate_docs_from_path, Args, Subcommand};
use std::path::Path;
use structopt::StructOpt;

fn main() {
    let args = Args::from_args();

    match args.subcommand {
        Subcommand::Build => {
            let path = Path::new("test-input");
            match generate_docs_from_path(&path) {
                Ok(_) => return,
                Err(error) => eprintln!("{}", error),
            };

            std::process::exit(1);
        }
    }
}
