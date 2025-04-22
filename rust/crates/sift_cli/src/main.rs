use clap::Parser;
use std::process::ExitCode;

/// Defines the CLI and its arguments
pub(crate) mod args;
use args::*;

/// The actual CLI runner that executes [args::Clargs]
pub(crate) mod cmd;

fn main() -> ExitCode {
    let clargs = Clargs::parse();

    match cmd::run(clargs) {
        Ok(_) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("{err:?}");
            ExitCode::FAILURE
        }
    }
}
