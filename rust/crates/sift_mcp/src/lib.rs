use anyhow::{Context, Result};
use clap::{crate_name, crate_version};
use rmcp::{ServiceExt, transport::stdio};
use sift_rs::{Credentials, SiftChannelBuilder};

mod server;
use server::SiftMcpServer;

mod startup;
pub use startup::report_startup_error;

mod error;
mod policy;
mod prompt;
mod service;
mod tool;

pub async fn run(
    credentials: Credentials,
    use_tls: bool,
    app_uri: String,
    allow_create: bool,
    allow_destructive: bool,
) -> Result<()> {
    let channel = SiftChannelBuilder::new(credentials)
        .use_tls(use_tls)
        .user_agent(format!("{}/{}", crate_name!(), crate_version!()))
        .build()
        .context("failed to build gRPC channel to connect to Sift")?;

    let service = SiftMcpServer::new(channel, app_uri, allow_create, allow_destructive)
        .serve(stdio())
        .await
        .context("failed to start MCP server")?;

    service
        .waiting()
        .await
        .context("MCP server terminated unexpectedly")?;

    Ok(())
}
