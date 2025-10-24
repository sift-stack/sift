use anyhow::{Context as AnyhowContext, Result};
use crossterm::style::Stylize;
use std::process::ExitCode;
use tokio::runtime;

mod cli;
use cli::{Cmd, ConfigCmd};

mod cmd;
use cmd::Context;

mod util;
use util::tty::Output;

use clap::Parser;

fn main() -> ExitCode {
    let args = cli::Args::parse();

    match run(args) {
        Err(err) => {
            Output::new().line(format!("{err:?}")).eprint();
            ExitCode::FAILURE
        }
        Ok(exit_code) => exit_code,
    }
}

fn run_future<F>(fut: F) -> Result<ExitCode>
where
    F: Future<Output = Result<ExitCode>> + 'static,
{
    let runtime = runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .context("failed to initialize Tokio runtime")?;

    runtime.block_on(fut)
}

fn run(clargs: cli::Args) -> Result<ExitCode> {
    // These commands don't require `Context`
    match clargs.cmd {
        Cmd::Config(cmd) => match cmd {
            ConfigCmd::Show => return cmd::config::show(),
            ConfigCmd::Create => return cmd::config::create(),
            ConfigCmd::Where => return cmd::config::config_where(),
            ConfigCmd::Update(args) => return cmd::config::update(clargs.profile, args),
        },
        Cmd::Completions(cmd) => match cmd {
            cli::CompletionsCmd::Print(args) => return cmd::completions::print(args),
            cli::CompletionsCmd::Update => return cmd::completions::update(),
        },
        _ => (),
    }

    let profile = clargs
        .profile
        .as_ref()
        .map_or_else(|| "default".to_string().cyan(), |s| s.clone().cyan());
    let ctx = Context::new(clargs.profile, clargs.disable_tls)?;

    Output::new()
        .line(format!("{} profile '{profile}'", "Using".green()))
        .print();

    // These commands require `Context`
    match clargs.cmd {
        Cmd::Import(cmd) => match cmd {
            cli::ImportCmd::Csv(args) => run_future(cmd::import::csv::run(ctx, args)),
            cli::ImportCmd::Parquet(cmd) => match cmd {
                cli::ImportParquetCmd::FlatDataset(args) => {
                    run_future(cmd::import::parquet::flat_dataset::run(ctx, args))
                }
            },
        },
        Cmd::Export(cmd) => match cmd {
            cli::ExportCmd::Run(args) => run_future(cmd::export::run(ctx, args)),
            cli::ExportCmd::Asset(args) => run_future(cmd::export::asset(ctx, args)),
        },
        _ => Ok(ExitCode::SUCCESS),
    }
}
