use std::process::ExitCode;

use anyhow::Result;
use sift_rs::Credentials;

use crate::cmd::Context;

pub async fn run(ctx: Context) -> Result<ExitCode> {
    let credentials = Credentials::Config {
        uri: ctx.grpc_uri,
        apikey: ctx.api_key,
    };
    match sift_mcp::run(credentials, !ctx.disable_tls).await {
        Ok(_) => Ok(ExitCode::SUCCESS),
        Err(err) => Err(err),
    }
}
