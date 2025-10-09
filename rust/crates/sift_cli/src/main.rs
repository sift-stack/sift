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
    if let Cmd::Config(subcmd) = clargs.cmd {
        return match subcmd {
            ConfigCmd::Show => cmd::config::show(),
            ConfigCmd::Create => cmd::config::create(),
            ConfigCmd::Where => cmd::config::config_where(),
            ConfigCmd::Update(args) => cmd::config::update(clargs.profile, args),
        };
    }
    let profile = clargs
        .profile
        .as_ref()
        .map_or_else(|| "default".to_string().green(), |s| s.clone().green());
    let ctx = Context::new(clargs.profile, clargs.disable_tls)?;

    Output::new()
        .line(format!("Using profile '{profile}'"))
        .print();

    match clargs.cmd {
        Cmd::Import(args) => match args {
            cli::ImportCmd::Csv(args) => run_future(cmd::csv::import(ctx, args)),
        },
        _ => Ok(ExitCode::SUCCESS),
    }
}
