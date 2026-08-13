use std::{collections::HashMap, sync::LazyLock, time::Duration};

use reqwest::header::USER_AGENT;
use serde::Serialize;

const CLIENT_EVENT_PATH: &str = "/api/v1/analytics/client-events";
const CLIENT_NAME: &str = "sift_mcp";
const REQUEST_TIMEOUT: Duration = Duration::from_secs(2);

static TOOL_EVENTS: LazyLock<HashMap<String, String>> = LazyLock::new(|| {
    serde_json::from_str(include_str!("tool_events.json"))
        .expect("tool_events.json must contain a JSON object of string pairs")
});

pub struct ClientEventConfig {
    rest_uri: String,
    api_key: String,
    cli_version: String,
}

impl ClientEventConfig {
    pub fn new(rest_uri: String, api_key: String, cli_version: String) -> Self {
        Self {
            rest_uri,
            api_key,
            cli_version,
        }
    }

    pub(crate) fn cli_version(&self) -> &str {
        &self.cli_version
    }
}

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
}

impl ClientEventReporter {
    pub(crate) fn new(config: ClientEventConfig) -> Self {
        let endpoint = format!(
            "{}{CLIENT_EVENT_PATH}",
            config.rest_uri.trim_end_matches('/')
        );
        Self {
            target: Some(ClientEventTarget {
                client: reqwest::Client::new(),
                endpoint,
                api_key: config.api_key,
                user_agent: format!("{CLIENT_NAME}/{}", config.cli_version),
            }),
        }
    }

    pub(crate) async fn send(&self, tool_name: &str) -> reqwest::Result<()> {
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
            .json(&ClientEventRequest { event })
            .send()
            .await?
            .error_for_status()?;

        Ok(())
    }
}

pub(crate) fn event_for_tool(tool_name: &str) -> Option<&'static str> {
    TOOL_EVENTS.get(tool_name).map(String::as_str)
}

#[cfg(test)]
pub(crate) async fn start_event_server() -> (String, tokio::task::JoinHandle<Vec<u8>>) {
    use tokio::{
        io::{AsyncReadExt, AsyncWriteExt},
        net::TcpListener,
    };

    fn request_length(request: &[u8]) -> Option<usize> {
        let header_end = request
            .windows(4)
            .position(|window| window == b"\r\n\r\n")?;
        let headers = std::str::from_utf8(&request[..header_end]).ok()?;
        let content_length = headers.lines().find_map(|line| {
            let (name, value) = line.split_once(':')?;
            name.eq_ignore_ascii_case("content-length")
                .then(|| value.trim().parse::<usize>().ok())
                .flatten()
        })?;
        Some(header_end + 4 + content_length)
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
            if request_length(&request).is_some_and(|length| request.len() >= length) {
                break;
            }
        }
        stream
            .write_all(
                b"HTTP/1.1 200 OK\r\ncontent-type: application/json\r\ncontent-length: 2\r\nconnection: close\r\n\r\n{}",
            )
            .await
            .unwrap();
        request
    });

    (format!("http://{address}"), server)
}

#[cfg(test)]
mod tests {
    use super::{ClientEventConfig, ClientEventReporter, start_event_server};

    #[tokio::test]
    async fn sends_only_the_event_with_the_cli_version() {
        let (rest_uri, server) = start_event_server().await;
        let reporter = ClientEventReporter::new(ClientEventConfig::new(
            rest_uri,
            "test-key".to_string(),
            "7.8.9".to_string(),
        ));

        reporter.send("list_assets").await.unwrap();
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
}
