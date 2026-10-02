use std::{collections::HashMap, path::Path, sync::LazyLock, time::Duration};

use reqwest::header::USER_AGENT;
use rmcp::model::JsonObject;
use serde::Serialize;
use serde_json::Value;

use crate::ClientName;

const CLIENT_EVENT_PATH: &str = "/api/v1/analytics/client-events";
const REQUEST_TIMEOUT: Duration = Duration::from_secs(2);

static TOOL_EVENTS: LazyLock<HashMap<String, String>> = LazyLock::new(|| {
    serde_json::from_str(include_str!("tool_events.json"))
        .expect("tool_events.json must contain a JSON object of string pairs")
});

pub struct ClientEventConfig {
    rest_uri: String,
    api_key: String,
}

impl ClientEventConfig {
    pub fn new(rest_uri: String, api_key: String) -> Self {
        Self { rest_uri, api_key }
    }
}

/// A reporter without a target never builds a request, which is how a server
/// launched with `--disable-nonessential-traffic` stays silent.
#[derive(Clone, Default)]
pub(crate) struct ClientEventReporter {
    target: Option<ClientEventTarget>,
}

#[derive(Clone)]
struct ClientEventTarget {
    client: reqwest::Client,
    endpoint: String,
    api_key: String,
    user_agent: String,
}

#[derive(Serialize)]
struct ClientEventRequest {
    event: &'static str,
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    properties: HashMap<&'static str, String>,
}

impl ClientEventReporter {
    pub(crate) fn from_config(
        config: Option<ClientEventConfig>,
        client_name: ClientName,
        cli_version: &str,
    ) -> Self {
        config.map_or_else(Self::default, |config| {
            Self::new(config, client_name, cli_version)
        })
    }

    pub(crate) fn new(
        config: ClientEventConfig,
        client_name: ClientName,
        cli_version: &str,
    ) -> Self {
        let endpoint = format!(
            "{}{CLIENT_EVENT_PATH}",
            config.rest_uri.trim_end_matches('/')
        );
        Self {
            target: Some(ClientEventTarget {
                client: reqwest::Client::new(),
                endpoint,
                api_key: config.api_key,
                user_agent: format!("{}/{cli_version}", client_name.as_str()),
            }),
        }
    }

    #[cfg(test)]
    pub(crate) fn is_reporting(&self) -> bool {
        self.target.is_some()
    }

    pub(crate) async fn send(
        &self,
        tool_name: &str,
        arguments: Option<&JsonObject>,
    ) -> reqwest::Result<()> {
        let Some(target) = &self.target else {
            return Ok(());
        };
        let Some(event) = event_for_tool(tool_name) else {
            return Ok(());
        };

        target
            .client
            .post(&target.endpoint)
            .timeout(REQUEST_TIMEOUT)
            .bearer_auth(&target.api_key)
            .header(USER_AGENT, &target.user_agent)
            .json(&ClientEventRequest {
                event,
                properties: properties_for_tool(tool_name, arguments),
            })
            .send()
            .await?
            .error_for_status()?;

        Ok(())
    }
}

pub(crate) fn event_for_tool(tool_name: &str) -> Option<&'static str> {
    TOOL_EVENTS.get(tool_name).map(String::as_str)
}

/// Properties read from a call's arguments. The event fires before the tool
/// runs, so they describe the request, not its result.
fn properties_for_tool(
    tool_name: &str,
    arguments: Option<&JsonObject>,
) -> HashMap<&'static str, String> {
    let mut properties = HashMap::new();
    if tool_name == "create_artifact"
        && let Some(file_type) = arguments.and_then(artifact_file_type)
    {
        properties.insert("file_type", file_type);
    }
    properties
}

/// `json` for a structured artifact, otherwise the lowercase extension of
/// `file_path`. An artifact created without a file has no file type.
fn artifact_file_type(arguments: &JsonObject) -> Option<String> {
    let storage_class = arguments
        .get("storage_class")
        .and_then(Value::as_str)
        .map(|value| value.trim().to_ascii_lowercase());
    if matches!(
        storage_class.as_deref(),
        Some("structured" | "artifact_storage_class_structured")
    ) {
        return Some("json".to_string());
    }
    let file_path = arguments.get("file_path").and_then(Value::as_str)?;
    Path::new(file_path.trim())
        .extension()
        .and_then(|extension| extension.to_str())
        .filter(|extension| !extension.is_empty())
        .map(str::to_ascii_lowercase)
}

#[cfg(test)]
pub(crate) async fn start_http_server(
    response: Vec<u8>,
) -> (String, tokio::task::JoinHandle<Vec<u8>>) {
    use tokio::{
        io::{AsyncReadExt, AsyncWriteExt},
        net::TcpListener,
    };

    fn request_complete(request: &[u8]) -> bool {
        let header_end = request.windows(4).position(|window| window == b"\r\n\r\n");
        let Some(header_end) = header_end else {
            return false;
        };
        let Ok(headers) = std::str::from_utf8(&request[..header_end]) else {
            return false;
        };
        let content_length = headers.lines().find_map(|line| {
            let (name, value) = line.split_once(':')?;
            name.eq_ignore_ascii_case("content-length")
                .then(|| value.trim().parse::<usize>().ok())
                .flatten()
        });
        content_length.is_none_or(|length| request.len() >= header_end + 4 + length)
    }

    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = listener.local_addr().unwrap();
    let server = tokio::spawn(async move {
        let (mut stream, _) = listener.accept().await.unwrap();
        let mut request = Vec::new();
        loop {
            let mut buffer = [0; 1024];
            let count = stream.read(&mut buffer).await.unwrap();
            assert!(
                count > 0,
                "the client closed before the request was complete"
            );
            request.extend_from_slice(&buffer[..count]);
            if request_complete(&request) {
                break;
            }
        }
        stream.write_all(&response).await.unwrap();
        request
    });

    (format!("http://{address}"), server)
}

#[cfg(test)]
pub(crate) async fn start_event_server() -> (String, tokio::task::JoinHandle<Vec<u8>>) {
    start_http_server(
        b"HTTP/1.1 200 OK\r\ncontent-type: application/json\r\ncontent-length: 2\r\nconnection: close\r\n\r\n{}"
            .to_vec(),
    )
    .await
}

#[cfg(test)]
mod tests {
    use super::{
        ClientEventConfig, ClientEventReporter, event_for_tool, properties_for_tool,
        start_event_server,
    };
    use crate::ClientName;

    fn arguments(value: serde_json::Value) -> rmcp::model::JsonObject {
        value.as_object().unwrap().clone()
    }

    #[test]
    fn create_artifact_reports_the_file_type() {
        let cases = [
            (
                serde_json::json!({ "file_path": "/workspace/out/Report.PDF" }),
                Some("pdf"),
            ),
            (
                serde_json::json!({ "file_path": "dashboard.html" }),
                Some("html"),
            ),
            (
                serde_json::json!({ "storage_class": "Structured", "payload": {} }),
                Some("json"),
            ),
            (serde_json::json!({ "file_path": "notes" }), None),
            (serde_json::json!({ "title": "No file yet" }), None),
        ];
        for (args, expected) in cases {
            let properties = properties_for_tool("create_artifact", Some(&arguments(args.clone())));
            assert_eq!(
                properties.get("file_type").map(String::as_str),
                expected,
                "{args}"
            );
        }
        assert!(properties_for_tool("create_artifact", None).is_empty());
    }

    #[test]
    fn other_tools_report_no_properties() {
        let args = arguments(serde_json::json!({ "file_path": "report.pdf" }));
        assert!(properties_for_tool("update_artifact", Some(&args)).is_empty());
    }

    #[tokio::test]
    async fn sends_create_artifact_file_type_as_a_property() {
        let (rest_uri, server) = start_event_server().await;
        let reporter = ClientEventReporter::new(
            ClientEventConfig::new(rest_uri, "test-key".to_string()),
            ClientName::Chat,
            "7.8.9",
        );

        let args = arguments(serde_json::json!({ "file_path": "report.pdf" }));
        reporter.send("create_artifact", Some(&args)).await.unwrap();
        let request = String::from_utf8(server.await.unwrap()).unwrap();
        let (_, body) = request.split_once("\r\n\r\n").unwrap();

        assert_eq!(
            serde_json::from_str::<serde_json::Value>(body).unwrap(),
            serde_json::json!({
                "event": "CLIENT_EVENT_USER_CALLED_MCP_TOOL_CREATE_ARTIFACT",
                "properties": { "file_type": "pdf" }
            })
        );
    }

    #[test]
    fn artifact_archive_tools_have_client_events() {
        assert_eq!(
            event_for_tool("archive_artifact"),
            Some("CLIENT_EVENT_USER_CALLED_MCP_TOOL_ARCHIVE_ARTIFACT")
        );
        assert_eq!(
            event_for_tool("unarchive_artifact"),
            Some("CLIENT_EVENT_USER_CALLED_MCP_TOOL_UNARCHIVE_ARTIFACT")
        );
        assert_eq!(
            event_for_tool("list_artifact_versions"),
            Some("CLIENT_EVENT_USER_CALLED_MCP_TOOL_LIST_ARTIFACT_VERSIONS")
        );
    }

    #[tokio::test]
    async fn sends_only_the_event_with_the_cli_version() {
        let (rest_uri, server) = start_event_server().await;
        let reporter = ClientEventReporter::new(
            ClientEventConfig::new(rest_uri, "test-key".to_string()),
            ClientName::SiftMcp,
            "7.8.9",
        );

        reporter.send("list_assets", None).await.unwrap();
        let request = String::from_utf8(server.await.unwrap()).unwrap();
        let (headers, body) = request.split_once("\r\n\r\n").unwrap();

        assert!(headers.starts_with("POST /api/v1/analytics/client-events HTTP/1.1"));
        assert!(
            headers
                .lines()
                .any(|line| line.eq_ignore_ascii_case("authorization: Bearer test-key"))
        );
        assert!(
            headers
                .lines()
                .any(|line| line.eq_ignore_ascii_case("user-agent: sift_mcp/7.8.9"))
        );
        assert_eq!(
            serde_json::from_str::<serde_json::Value>(body).unwrap(),
            serde_json::json!({
                "event": "CLIENT_EVENT_USER_CALLED_MCP_TOOL_LIST_ASSETS"
            })
        );
    }

    #[tokio::test]
    async fn chat_config_identifies_as_chat() {
        let (rest_uri, server) = start_event_server().await;
        let reporter = ClientEventReporter::new(
            ClientEventConfig::new(rest_uri, "test-key".to_string()),
            ClientName::Chat,
            "7.8.9",
        );

        reporter.send("list_assets", None).await.unwrap();
        let request = String::from_utf8(server.await.unwrap()).unwrap();
        let (headers, _) = request.split_once("\r\n\r\n").unwrap();

        assert!(
            headers
                .lines()
                .any(|line| line.eq_ignore_ascii_case("user-agent: chat/7.8.9"))
        );
    }

    #[test]
    fn a_missing_config_reports_nothing() {
        assert!(
            !ClientEventReporter::from_config(None, ClientName::SiftMcp, "7.8.9").is_reporting()
        );
        assert!(
            ClientEventReporter::from_config(
                Some(ClientEventConfig::new(
                    "https://rest.test.local".to_string(),
                    "test-key".to_string(),
                )),
                ClientName::SiftMcp,
                "7.8.9",
            )
            .is_reporting()
        );
    }
}
