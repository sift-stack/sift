use std::{
    io::{self, IsTerminal},
    process::ExitCode,
};

use anyhow::{Error, Result};
use sift_rs::Credentials;
use tracing_subscriber::EnvFilter;

use crate::cli::McpArgs;
use crate::cmd::Context;
use crate::util::tty::Output;

pub async fn report_startup_error(error: Error) -> Result<ExitCode> {
    let message = format!("{error:#}");
    Output::new().line(&message).eprint();
    if io::stdin().is_terminal() {
        return Ok(ExitCode::FAILURE);
    }
    sift_mcp::report_startup_error(message).await?;
    Ok(ExitCode::FAILURE)
}

pub async fn run(ctx: Context, args: McpArgs, app_uri: String) -> Result<ExitCode> {
    // stdout carries the MCP protocol, so every diagnostic goes to stderr,
    // which MCP clients capture in their logs. Without a subscriber the
    // server runs silent and an unexpected death leaves no evidence behind.
    tracing_subscriber::fmt()
        .with_writer(std::io::stderr)
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .init();

    tracing::info!(
        version = env!("CARGO_PKG_VERSION"),
        uri = %ctx.grpc_uri,
        "starting Sift MCP server"
    );

    let credentials = Credentials::Config {
        uri: ctx.grpc_uri,
        apikey: ctx.api_key,
    };
    match sift_mcp::run(
        credentials,
        !ctx.disable_tls,
        app_uri,
        args.allow_destructive,
    )
    .await
    {
        Ok(_) => {
            tracing::info!("Sift MCP server exited cleanly");
            Ok(ExitCode::SUCCESS)
        }
        Err(err) => {
            tracing::error!(error = ?err, "Sift MCP server terminated");
            Err(err)
        }
    }
}
