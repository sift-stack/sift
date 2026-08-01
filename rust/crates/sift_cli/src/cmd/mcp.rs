use std::process::ExitCode;

use anyhow::Result;
use sift_rs::Credentials;

use crate::cli::McpArgs;
use crate::cmd::Context;
use crate::util::app_uri::resolve_app_uri;

pub async fn run(ctx: Context, args: McpArgs) -> Result<ExitCode> {
    let app_uri = resolve_app_uri(ctx.app_uri.as_deref(), &ctx.rest_uri);
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
