use rmcp::{
    handler::server::wrapper::Parameters,
    model::{CallToolResult, ContentBlock},
    schemars::{self, JsonSchema},
    tool, tool_router,
};
use serde::Deserialize;

use crate::{error, server::SiftMcpServer};

#[derive(Debug, Deserialize, JsonSchema)]
pub struct CheckForUpdatesParams {}

#[tool_router(router = update_router, vis = "pub(crate)")]
impl SiftMcpServer {
    #[tool(
        name = "check_for_updates",
        description = "
            Check whether this sift-cli and its embedded MCP server are current.

            Output:
              - `status`: `current`, `update_available`, or `unavailable`.
              - `current_version`: version of the sift-cli process that hosts this MCP server.
              - `latest_version`: latest stable release, when known.
              - `install_command`: exact curl or PowerShell installer command when an update exists.
              - `message`: text to relay to the user.

            Guidance:
              - Call once before the first other Sift tool in each session.
              - If `status = update_available`, relay `message` and `install_command` exactly.
              - If `status = unavailable`, continue with the user's requested Sift task.
        ",
        annotations(title = "system/check_for_updates", read_only_hint = true)
    )]
    pub async fn check_for_updates(
        &self,
        _params: Parameters<CheckForUpdatesParams>,
    ) -> error::McpResult {
        let Some(mut receiver) = self.update_check.clone() else {
            return Err(rmcp::ErrorData::invalid_request(
                "The update check is disabled for this MCP server.",
                None,
            ));
        };
        let is_checking = receiver.borrow().is_checking();
        let update_check = if is_checking && receiver.changed().await.is_err() {
            let current_version = match &*receiver.borrow() {
                crate::UpdateCheck::Checking { current_version } => current_version.clone(),
                update_check => return Ok(update_result(update_check.clone())),
            };
            crate::UpdateCheck::Unavailable {
                current_version,
                message: "Could not check for a newer sift-cli release. Run `sift-cli --version` for details."
                    .to_string(),
            }
        } else {
            receiver.borrow().clone()
        };
        Ok(update_result(update_check))
    }
}

fn update_result(update_check: crate::UpdateCheck) -> CallToolResult {
    let message = update_check.message();
    let mut result = CallToolResult::structured(serde_json::json!(update_check));
    result.content = vec![ContentBlock::text(message)];
    result
}
