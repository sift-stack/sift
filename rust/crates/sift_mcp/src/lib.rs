use anyhow::{Context, Result};
use clap::{crate_name, crate_version};
use rmcp::{ServiceExt, transport::stdio};
use sift_rs::{Credentials, SiftChannelBuilder};

pub(crate) mod server;
use server::SiftMcpServer;

pub mod tool;

mod error;

pub async fn run(credentials: Credentials, use_tls: bool) -> Result<()> {
    let channel = SiftChannelBuilder::new(credentials)
        .use_tls(use_tls)
        .user_agent(format!("{}/{}", crate_name!(), crate_version!()))
        .build()
        .context("failed to build gRPC channel to connect to Sift")?;

    let service = SiftMcpServer::new(channel)
        .serve(stdio())
        .await
        .context("failed to start MCP server")?;

    service
        .waiting()
        .await
        .context("MCP server terminated unexpectedly")?;

    Ok(())
}
