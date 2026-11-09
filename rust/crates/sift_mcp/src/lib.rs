use anyhow::{Context, Result};
use clap::crate_version;
use rmcp::{ServiceExt, transport::stdio};
use serde::Serialize;
use sift_rs::{Credentials, SiftChannelBuilder};
use tokio::sync::watch;

mod client_event;
pub use client_event::ClientEventConfig;

mod feature_flags;
pub use feature_flags::FeatureFlags;
pub use service::declarative::DeclarativeService;
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

/// Names this server in the User-Agent of every request it sends to Sift,
/// which is how Sift attributes the activity.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ClientName {
    #[default]
    SiftMcp,
    /// The server running inside a Sift chat pod.
    Chat,
}

impl ClientName {
    pub(crate) fn as_str(self) -> &'static str {
        match self {
            Self::SiftMcp => "sift_mcp",
            Self::Chat => "chat",
        }
    }
}

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
        Vec::new(),
        ClientName::default(),
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
    ignored_tools: Vec<String>,
    client_name: ClientName,
) -> Result<()> {
    let client_event_reporter = client_event::ClientEventReporter::from_config(
        client_event_config,
        client_name,
        &cli_version,
    );
    let rest_config = rest_config.map(|config| RestConfig {
        client_name,
        ..config
    });
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
            ignored_tools,
            client_name,
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
    ignored_tools: Vec<String>,
    client_name: ClientName,
}

async fn run_server(credentials: Credentials, use_tls: bool, config: RunConfig) -> Result<()> {
    let channel = SiftChannelBuilder::new(credentials)
        .use_tls(use_tls)
        .user_agent(grpc_user_agent(config.client_name))
        .build()
        .context("failed to build gRPC channel to connect to Sift")?;

    let has_rest_config = config.rest_config.is_some();
    let server = SiftMcpServer::new_with_client_events(
        channel,
        config.app_uri,
        config.allow_create,
        config.allow_destructive,
        config.cli_version,
        config.update_check,
        config.client_event_reporter,
        config.feature_flags,
        config.rest_config,
        config.ignored_tools,
    )?;
    let _schema_download =
        ChartSchemaDownload(start_chart_schema_download(&server, has_rest_config));
    let service = server
        .serve(stdio())
        .await
        .context("failed to start MCP server")?;
    service
        .waiting()
        .await
        .context("MCP server terminated unexpectedly")?;
    Ok(())
}

// A canceled server future must not leave a detached download owning its cache.
struct ChartSchemaDownload(Option<tokio::task::JoinHandle<()>>);

impl Drop for ChartSchemaDownload {
    fn drop(&mut self) {
        if let Some(task) = &self.0 {
            task.abort();
        }
    }
}

/// Refresh the machine-local schema cache if needed without delaying MCP
/// initialization or unrelated tools. A failed refresh is reported by the
/// schema tool, which retries on demand.
fn start_chart_schema_download(
    server: &SiftMcpServer,
    has_rest_config: bool,
) -> Option<tokio::task::JoinHandle<()>> {
    if !has_rest_config || !server.tool_router.has_route("get_chart_schema") {
        return None;
    }
    let declarative = server.declarative_service.clone();
    Some(tokio::spawn(async move {
        if let Err(error) = declarative.chart_schema().await {
            eprintln!("Could not refresh the cached chart schema: {error:#}");
        }
    }))
}

fn grpc_user_agent(client_name: ClientName) -> String {
    format!("{}/{}", client_name.as_str(), crate_version!())
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

    async fn server_with_schema_endpoint(rest_uri: String) -> super::SiftMcpServer {
        let (client, _server) = tokio::io::duplex(1024);
        let channel = sift_test_util::grpc::memory_sift_channel(client).await;
        let mut server =
            super::SiftMcpServer::new(channel, "https://app.example.test".into(), false, false);
        server.declarative_service = super::DeclarativeService::new(
            Some(super::RestConfig::new(rest_uri, "test-key".into())),
            "test",
        )
        .with_retry(crate::service::declarative::test::fast_retry());
        server
    }

    #[tokio::test]
    async fn startup_download_populates_the_cache_file_before_any_schema_call() {
        use crate::service::declarative::test::{
            Reply, chart_schema_responses, start_sequence_server,
        };
        let (rest_uri, http) = start_sequence_server(
            chart_schema_responses()
                .into_iter()
                .map(Reply::Respond)
                .collect(),
        )
        .await;
        let server = server_with_schema_endpoint(rest_uri).await;
        let task = super::start_chart_schema_download(&server, true).unwrap();
        task.await.unwrap();
        assert_eq!(http.await.unwrap().len(), 2);
        // The mock is now closed, so this can only succeed from the cache file.
        let snapshot = server.declarative_service.chart_schema().await.unwrap();
        assert!(snapshot.path.is_absolute());
        assert!(snapshot.path.is_file());
    }

    #[tokio::test]
    async fn startup_download_is_skipped_without_rest_or_when_schema_tool_is_ignored() {
        let mut server = server_with_schema_endpoint("http://127.0.0.1:1".into()).await;
        assert!(super::start_chart_schema_download(&server, false).is_none());
        server.tool_router.remove_route("get_chart_schema");
        assert!(super::start_chart_schema_download(&server, true).is_none());
    }

    #[tokio::test]
    async fn unavailable_schema_does_not_fail_startup() {
        use crate::service::declarative::test::{Reply, http_response, start_sequence_server};
        let (rest_uri, http) =
            start_sequence_server(vec![Reply::Respond(http_response("404 Not Found", ""))]).await;
        let server = server_with_schema_endpoint(rest_uri).await;
        super::start_chart_schema_download(&server, true)
            .unwrap()
            .await
            .expect("schema failure must not fail the background task or MCP startup");
        assert_eq!(http.await.unwrap().len(), 1);
    }

    #[tokio::test]
    async fn stalled_schema_download_does_not_block_mcp_initialization_or_other_tools() {
        use crate::service::declarative::test::{Reply, start_sequence_server};
        use rmcp::ServiceExt;
        use std::time::Duration;
        use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};

        let (rest_uri, http) = start_sequence_server(vec![Reply::Hang]).await;
        let server = server_with_schema_endpoint(rest_uri).await;
        let download = super::start_chart_schema_download(&server, true).unwrap();
        // Observe the startup fetch without making a schema tool call.
        assert_eq!(
            tokio::time::timeout(Duration::from_secs(2), http)
                .await
                .unwrap()
                .unwrap()
                .len(),
            1
        );
        let (client, transport) = tokio::io::duplex(65536);
        let serving = tokio::spawn(async move {
            let service = server.serve(transport).await.unwrap();
            service.waiting().await.unwrap();
        });
        let (reader, mut writer) = tokio::io::split(client);
        let mut lines = BufReader::new(reader).lines();
        let requests = [
            serde_json::json!({"jsonrpc":"2.0","id":1,"method":"initialize","params":{
                "protocolVersion":"2024-11-05","capabilities":{},
                "clientInfo":{"name":"schema-startup-test","version":"1"}
            }}),
            serde_json::json!({"jsonrpc":"2.0","id":2,"method":"tools/list","params":{}}),
            serde_json::json!({"jsonrpc":"2.0","id":3,"method":"tools/call","params":{
                "name":"explore_url","arguments":{"run_ids":["test-run"]}
            }}),
        ];
        for request in requests {
            writer
                .write_all(format!("{request}\n").as_bytes())
                .await
                .unwrap();
            let line = tokio::time::timeout(Duration::from_secs(2), lines.next_line())
                .await
                .expect("schema download must not block unrelated MCP requests")
                .unwrap()
                .unwrap();
            let response: serde_json::Value = serde_json::from_str(&line).unwrap();
            assert_eq!(response["id"], request["id"]);
            assert!(response.get("error").is_none(), "{response}");
            assert_ne!(response["result"]["isError"], true);
            if request["id"] == 1 {
                writer
                    .write_all(b"{\"jsonrpc\":\"2.0\",\"method\":\"notifications/initialized\"}\n")
                    .await
                    .unwrap();
            }
        }
        download.abort();
        serving.abort();
    }

    #[test]
    fn public_run_keeps_its_legacy_signature() {
        accepts_legacy_run(super::run);
    }

    #[test]
    fn grpc_user_agent_names_the_client() {
        let version = clap::crate_version!();
        assert_eq!(
            super::grpc_user_agent(super::ClientName::SiftMcp),
            format!("sift_mcp/{version}")
        );
        assert_eq!(
            super::grpc_user_agent(super::ClientName::Chat),
            format!("chat/{version}")
        );
    }
}
