use std::future::{self, Future};

use anyhow::{Context, Result};
use rmcp::{
    ErrorData, RoleServer, ServerHandler, ServiceExt,
    model::{ListToolsResult, PaginatedRequestParams, ServerCapabilities, ServerInfo},
    service::{MaybeSendFuture, RequestContext},
    transport::stdio,
};

struct StartupErrorServer {
    message: String,
}

impl StartupErrorServer {
    fn new(message: String) -> Self {
        Self { message }
    }

    fn error(&self) -> ErrorData {
        ErrorData::internal_error(
            self.message.clone(),
            Some(serde_json::json!({
                "status": "stopped",
                "reason": "AppUriRequired",
                "diagnostic_command": "sift-cli agent doctor",
            })),
        )
    }
}

impl ServerHandler for StartupErrorServer {
    fn list_tools(
        &self,
        _request: Option<PaginatedRequestParams>,
        _context: RequestContext<RoleServer>,
    ) -> impl Future<Output = std::result::Result<ListToolsResult, ErrorData>> + MaybeSendFuture + '_
    {
        future::ready(Err(self.error()))
    }

    fn get_info(&self) -> ServerInfo {
        ServerInfo::new(ServerCapabilities::builder().enable_tools().build())
            .with_instructions(self.message.clone())
    }
}

pub async fn report_startup_error(message: String) -> Result<()> {
    let service = StartupErrorServer::new(message)
        .serve(stdio())
        .await
        .context("failed to start the MCP profile error server")?;
    service
        .waiting()
        .await
        .context("the MCP profile error server stopped unexpectedly")?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use rmcp::ServiceExt;
    use serde_json::Value;
    use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};

    use super::StartupErrorServer;

    #[tokio::test]
    async fn returns_the_reason_when_the_client_lists_tools() {
        let (server_transport, client_transport) = tokio::io::duplex(4096);
        let server = tokio::spawn(async move {
            let service = StartupErrorServer::new("profile needs app_uri".to_string())
                .serve(server_transport)
                .await
                .unwrap();
            service.waiting().await.unwrap()
        });
        let (reader, mut writer) = tokio::io::split(client_transport);
        let mut reader = BufReader::new(reader);

        let request = serde_json::json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "initialize",
            "params": {
                "protocolVersion": "2026-07-28",
                "capabilities": {},
                "clientInfo": { "name": "test-client", "version": "0.0.1" }
            }
        });
        writer
            .write_all(format!("{request}\n").as_bytes())
            .await
            .unwrap();

        let mut response = String::new();
        reader.read_line(&mut response).await.unwrap();
        let init_response: Value = serde_json::from_str(&response).unwrap();
        // `2026-07-28` has no `initialize` handshake, so rmcp negotiates a
        // client that names it there down to the newest legacy version.
        assert_eq!(init_response["result"]["protocolVersion"], "2025-11-25");
        assert_eq!(
            init_response["result"]["instructions"],
            "profile needs app_uri"
        );

        writer
            .write_all(b"{\"jsonrpc\":\"2.0\",\"method\":\"notifications/initialized\"}\n")
            .await
            .unwrap();
        writer
            .write_all(b"{\"jsonrpc\":\"2.0\",\"id\":2,\"method\":\"tools/list\",\"params\":{}}\n")
            .await
            .unwrap();

        response.clear();
        reader.read_line(&mut response).await.unwrap();
        let tool_response: Value = serde_json::from_str(&response).unwrap();
        assert_eq!(tool_response["error"]["message"], "profile needs app_uri");
        assert_eq!(tool_response["error"]["data"]["reason"], "AppUriRequired");

        drop(writer);
        drop(reader);
        server.await.unwrap();
    }
}
