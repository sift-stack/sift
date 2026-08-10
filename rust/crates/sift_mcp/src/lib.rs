use anyhow::{Context, Result};
use clap::{crate_name, crate_version};
use rmcp::{ServiceExt, transport::stdio};
use serde::Serialize;
use sift_rs::{Credentials, SiftChannelBuilder};
use tokio::sync::watch;

mod server;
use server::SiftMcpServer;

mod startup;
pub use startup::report_startup_error;

mod error;
mod policy;
mod prompt;
mod service;
mod tool;

#[derive(Clone, Debug, Serialize)]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum UpdateCheck {
    Checking {
        current_version: String,
    },
    Current {
        current_version: String,
        latest_version: String,
    },
    UpdateAvailable {
        current_version: String,
        latest_version: String,
        install_command: String,
        message: String,
    },
    Unavailable {
        current_version: String,
        message: String,
    },
}

impl UpdateCheck {
    pub fn message(&self) -> String {
        match self {
            Self::Checking { current_version } => {
                format!("The update check for sift-cli {current_version} is not complete.")
            }
            Self::Current {
                current_version,
                latest_version,
            } => format!("sift-cli {current_version} is current; latest is {latest_version}."),
            Self::UpdateAvailable { message, .. } | Self::Unavailable { message, .. } => {
                message.clone()
            }
        }
    }

    pub(crate) fn is_checking(&self) -> bool {
        matches!(self, Self::Checking { .. })
    }

    pub(crate) fn update_message(&self) -> Option<&str> {
        match self {
            Self::UpdateAvailable { message, .. } => Some(message),
            _ => None,
        }
    }
}

pub type UpdateCheckReceiver = watch::Receiver<UpdateCheck>;

pub async fn run(
    credentials: Credentials,
    use_tls: bool,
    app_uri: String,
    allow_create: bool,
    allow_destructive: bool,
    cli_version: String,
    update_check: Option<UpdateCheckReceiver>,
) -> Result<()> {
    let channel = SiftChannelBuilder::new(credentials)
        .use_tls(use_tls)
        .user_agent(format!("{}/{}", crate_name!(), crate_version!()))
        .build()
        .context("failed to build gRPC channel to connect to Sift")?;

    let service = SiftMcpServer::new_with_update_check(
        channel,
        app_uri,
        allow_create,
        allow_destructive,
        cli_version,
        update_check,
    )
    .serve(stdio())
    .await
    .context("failed to start MCP server")?;

    service
        .waiting()
        .await
        .context("MCP server terminated unexpectedly")?;

    Ok(())
}
