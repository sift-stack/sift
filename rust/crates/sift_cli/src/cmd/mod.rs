use crate::{Clargs, Color, ConfigSubCmds, SubCmds, GetSubCmds};
use anyhow::{Context, Result, format_err};
use crossterm::style::force_color_output;
use sift_connect::{SiftChannelBuilder, SiftChannel, Credentials};
use std::{
    env,
    io::{self, IsTerminal},
};

mod completions;
mod config;
mod get;
mod utils;

/// Executes the CLI based on [Clargs].
pub fn run(args: Clargs) -> Result<()> {
    let Clargs { profile, disable_tls, color, .. } = args;

    let stdout = io::stdout();
    set_colorization(&stdout, color);

    match args.subcommands {
        SubCmds::Completions { shell } => completions::run(stdout, shell),
        SubCmds::Config { subcommand } => match subcommand {
            ConfigSubCmds::Show => config::show::run::<_, String>(stdout, None),
            ConfigSubCmds::Create {
                name,
                force,
                uri,
                apikey,
            } => config::create::run::<_, String>(stdout, None, name, uri, apikey, force),
        },
        SubCmds::Get { subcommand } => match subcommand {
            GetSubCmds::Assets { filter, pagination } => run_future(
                profile,
                disable_tls,
                async |channel| get::assets::run(stdout, filter, pagination, channel).await,
            ),
            GetSubCmds::Channels { asset, filter, pagination } => run_future(
                profile,
                disable_tls,
                async |channel| get::channels::run(stdout, asset, filter, pagination, channel).await,
            )
        }
    }
}

/// Runs a future to completion on a single threaded runtime.
pub fn run_future<F>(profile: Option<String>, disable_tls: bool, op: F) -> Result<()>
where F: AsyncFnOnce(SiftChannel) -> Result<()>,
{
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .context("failed to initialize Tokio runtime")?;

    let creds = Credentials::Profile(profile.clone());

    runtime.block_on(async move {
        let channel = SiftChannelBuilder::new(creds)
            .use_tls(!disable_tls)
            .build()
            .map_err(|e| format_err!("{e}"))?;

        op(channel).await
    })
}

/// Sets output colorization based on `color` parameter.
pub fn set_colorization(stdout: &io::Stdout, color: Color) {
    match color {
        Color::Auto => {
            if env::var("NO_COLOR").is_ok_and(|nc| !nc.is_empty()) || !stdout.is_terminal() {
                force_color_output(false)
            } else {
                force_color_output(true)
            }
        }
        Color::Never => force_color_output(false),
        Color::Always => (),
    }
}
