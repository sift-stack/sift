use rmcp::{
    handler::server::wrapper::Parameters,
    model::CallToolResult,
    schemars::{self, JsonSchema},
    tool, tool_router,
};
use serde::Deserialize;

use crate::{
    error::{self, from_anyhow},
    server::SiftMcpServer,
};

#[derive(Debug, Deserialize, JsonSchema)]
pub struct PingParams {}

#[tool_router(router = ping_router, vis = "pub(crate)")]
impl SiftMcpServer {
    #[tool(
        name = "ping",
        description = "
            Send a Ping request to the Sift backend and return the server-supplied response string.

            Output:
              - `{ \"response\": \"<string>\" }`. Server-defined message.

            Parameters:
              - None.

            Errors:
              - Surfaces upstream gRPC failures via the standard retry / soft-signal flow:
                `UNAVAILABLE` retries up to the policy limit before reporting `backend_unreachable`;
                `RESOURCE_EXHAUSTED`, `DEADLINE_EXCEEDED`, `INTERNAL`, and other soft-signal codes
                return immediately with their structured guidance.

            Guidance:
              - Useful as a connectivity smoke check. If `ping` fails, the broader Sift toolset is
                likely also failing — surface that to the user instead of attempting other tools.
        ",
        annotations(title = "ping_router/ping", read_only_hint = true)
    )]
    pub async fn ping(&self, _params: Parameters<PingParams>) -> error::McpResult {
        let out = self
            .ping_service
            .ping()
            .await
            .map(|response| serde_json::json!({ "response": response }))
            .map_err(from_anyhow)?;

        Ok(CallToolResult::structured(out))
    }
}
