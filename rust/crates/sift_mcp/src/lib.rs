use anyhow::{Context, Result};
use clap::{crate_name, crate_version};
use rmcp::{ServiceExt, transport::stdio};
use serde::Serialize;
use sift_rs::{Credentials, SiftChannelBuilder};
use tokio::sync::watch;

mod client_event;
pub use client_event::ClientEventConfig;

mod feature_flags;
pub use feature_flags::FeatureFlags;
pub use service::remote_files::RestConfig;

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
) -> Result<()> {
    run_with_update_check(
        credentials,
        use_tls,
        app_uri,
        allow_create,
        allow_destructive,
        crate_version!().to_string(),
        None,
    )
    .await
}

pub async fn run_with_update_check(
    credentials: Credentials,
    use_tls: bool,
    app_uri: String,
    allow_create: bool,
    allow_destructive: bool,
    cli_version: String,
    update_check: Option<UpdateCheckReceiver>,
) -> Result<()> {
    run_with_client_events(
        credentials,
        use_tls,
        app_uri,
        allow_create,
        allow_destructive,
        cli_version,
        update_check,
        None,
        FeatureFlags::default(),
        None,
    )
    .await
}

/// Runs the server, reporting anonymous tool-call events only when a client
/// event config is supplied. `None` leaves the server silent, which is what
/// `sift-cli mcp --disable-nonessential-traffic` passes.
#[allow(clippy::too_many_arguments)]
pub async fn run_with_client_events(
    credentials: Credentials,
    use_tls: bool,
    app_uri: String,
    allow_create: bool,
    allow_destructive: bool,
    cli_version: String,
    update_check: Option<UpdateCheckReceiver>,
    client_event_config: Option<ClientEventConfig>,
    feature_flags: FeatureFlags,
    rest_config: Option<RestConfig>,
) -> Result<()> {
    let client_event_reporter =
        client_event::ClientEventReporter::from_config(client_event_config, &cli_version);
    run_server(
        credentials,
        use_tls,
        RunConfig {
            app_uri,
            allow_create,
            allow_destructive,
            cli_version,
            update_check,
            client_event_reporter,
            feature_flags,
            rest_config,
        },
    )
    .await
}

struct RunConfig {
    app_uri: String,
    allow_create: bool,
    allow_destructive: bool,
    cli_version: String,
    update_check: Option<UpdateCheckReceiver>,
    client_event_reporter: client_event::ClientEventReporter,
    feature_flags: FeatureFlags,
    rest_config: Option<RestConfig>,
}

async fn run_server(credentials: Credentials, use_tls: bool, config: RunConfig) -> Result<()> {
    let channel = SiftChannelBuilder::new(credentials)
        .use_tls(use_tls)
        .user_agent(format!("{}/{}", crate_name!(), crate_version!()))
        .build()
        .context("failed to build gRPC channel to connect to Sift")?;

    let service = SiftMcpServer::new_with_client_events(
        channel,
        config.app_uri,
        config.allow_create,
        config.allow_destructive,
        config.cli_version,
        config.update_check,
        config.client_event_reporter,
        config.feature_flags,
        config.rest_config,
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

#[cfg(test)]
mod tests {
    use std::future::Future;

    use sift_rs::Credentials;

    fn accepts_legacy_run<F, Fut>(_run: F)
    where
        F: Fn(Credentials, bool, String, bool, bool) -> Fut,
        Fut: Future<Output = anyhow::Result<()>>,
    {
    }

    #[test]
    fn public_run_keeps_its_legacy_signature() {
        accepts_legacy_run(super::run);
    }
}
