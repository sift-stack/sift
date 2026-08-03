use std::{
    io::{self, IsTerminal},
    process::ExitCode,
};

use anyhow::{Error, Result};
use sift_rs::Credentials;

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
        Ok(_) => Ok(ExitCode::SUCCESS),
        Err(err) => Err(err),
    }
}
