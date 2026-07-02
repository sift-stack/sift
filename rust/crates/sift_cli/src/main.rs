use anyhow::{Context as AnyhowContext, Result};
use crossterm::style::Stylize;
use std::{future::Future, process::ExitCode};
use tokio::runtime;

mod cli;
use cli::{Cmd, ConfigCmd};

mod cmd;
use cmd::Context;

mod util;
use util::tty::Output;

use clap::{CommandFactory, Parser};

use crate::cli::InstallCmd;

const BIN_NAME: &str = "sift-cli";

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

#[allow(dead_code)]
fn run_future_mt<F>(fut: F) -> Result<ExitCode>
where
    F: Future<Output = Result<ExitCode>> + 'static,
{
    let runtime = runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .context("failed to initialize Tokio runtime")?;

    runtime.block_on(fut)
}

fn run(clargs: cli::Args) -> Result<ExitCode> {
    if clargs.version {
        return run_future(cmd::version::run());
    }

    let Some(cmd) = clargs.cmd else {
        cli::Args::command().print_help()?;
        return Ok(ExitCode::SUCCESS);
    };

    // These commands don't require `Context`
    match cmd {
        Cmd::Config(cmd) => match cmd {
            ConfigCmd::Show => return cmd::config::show(),
            ConfigCmd::Create => return cmd::config::create(),
            ConfigCmd::Where => return cmd::config::config_where(),
            ConfigCmd::Update(args) => return cmd::config::update(clargs.profile, args),
        },
        Cmd::Doc(args) => return run_future(cmd::doc::serve(args)),
        Cmd::Install(cmd) => match cmd {
            InstallCmd::Completions(cmd) => match cmd {
                cli::CompletionsCmd::Print(args) => return cmd::install::completions::print(args),
                cli::CompletionsCmd::Update => return cmd::install::completions::update(),
            },
            InstallCmd::AgentSkills(args) => return cmd::install::agent::skills(args),
        },
        _ => (),
    }

    let ctx = Context::new(clargs.profile.clone(), clargs.disable_tls)?;

    // Mcp Server
    if let Cmd::Mcp = cmd {
        return run_future_mt(cmd::mcp::run(ctx));
    }

    let profile = clargs
        .profile
        .as_ref()
        .map_or_else(|| "default".to_string().cyan(), |s| s.clone().cyan());

    Output::new()
        .line(format!("{} profile '{profile}'", "Using".green()))
        .print();

    // These commands require `Context`
    match cmd {
        Cmd::Import(cmd) => match cmd {
            cli::ImportCmd::Csv(args) => run_future(cmd::import::csv::run(ctx, args)),
            cli::ImportCmd::Parquet(cmd) => match cmd {
                cli::ImportParquetCmd::FlatDataset(args) => {
                    run_future(cmd::import::parquet::flat_dataset::run(ctx, args))
                }
                cli::ImportParquetCmd::ChannelPerRow(args) => run_future(
                    cmd::import::parquet::channel_per_row_dataset::run(ctx, args),
                ),
            },
            cli::ImportCmd::Tdms(args) => {
                run_future(cmd::import::tdms::detect_tdms_config::run(ctx, args))
            }
            cli::ImportCmd::Hdf5(args) => run_future(cmd::import::hdf5::import::run(ctx, args)),
            cli::ImportCmd::Ulog(args) => run_future(cmd::import::ulog::import::run(ctx, args)),
            cli::ImportCmd::Backup(args) => match args.cmd {
                Some(cli::BackupCmd::Ls(ls_args)) => {
                    run_future(cmd::import::backup::ls(ctx, ls_args))
                }
                None => run_future(cmd::import::backup::run(ctx, args.import_args)),
            },
        },
        Cmd::Export(cmd) => match cmd {
            cli::ExportCmd::Run(args) => run_future(cmd::export::run(ctx, args)),
            cli::ExportCmd::Asset(args) => run_future(cmd::export::asset(ctx, args)),
        },
        Cmd::Ping => run_future(cmd::ping::run(ctx)),
        _ => Ok(ExitCode::SUCCESS),
    }
}
