use rmcp::{handler::server::wrapper::Parameters, model::CallToolResult, tool, tool_router};

use crate::{
    error::{self, from_anyhow},
    server::SiftMcpServer,
    tool::common::ListParams,
};

#[cfg(test)]
mod test;

#[tool_router(router = channels_router, vis = "pub(crate)")]
impl SiftMcpServer {
    #[tool(
        name = "list_channels",
        description = "
            List channels in Sift, optionally filtered by a CEL expression and ordered by one or more fields.

            Output:
              - `{ \"channels\": [Channel, ...] }`. Each item is the full Sift `Channel` shape including
                `channel_id`, `name`, `description`, `asset_id`/`asset_name`, data type, units, enum/bit-field
                configuration, and timestamps.

            Parameters:
              - `filter`: CEL expression. Pass an empty string to list everything. Filterable fields:
                `channel_id`, `asset_id`, `name`, `description`, `run_id`, `run_name`, `run_client_key`,
                `created_date`, `modified_date`, `created_by_user_id`, `modified_by_user_id`.
              - `order_by`: optional comma-separated `FIELD_NAME[ desc]` list. Orderable fields: `name`,
                `created_date`, `modified_date`, `active`. Default sort is `created_date` ascending (oldest first) —
                note this differs from `list_assets` and `list_runs`. Example: `\"name,created_date desc\"`.
              - `limit`: optional cap on returned items. Values in `1..=1000` cap the result set. Omitting it OR
                passing a value above 1000 returns ALL matching channels (paginated server-side).

            Errors:
              - `INVALID_PARAMS` if `filter` is not a valid CEL expression or `order_by` references an unknown field.
              - `INTERNAL_ERROR` for upstream gRPC failures.

            Guidance:
              - Always scope with `asset_id == \"...\"` when the asset is known — channel namespaces are per-asset
                and unscoped queries return cross-asset results.
              - To enumerate channels recorded by a specific run, filter on `run_id` rather than joining client-side.
        ",
        annotations(title = "channels_router/list_channels", read_only_hint = true)
    )]
    pub async fn list_channels(&self, params: Parameters<ListParams>) -> error::McpResult {
        let Parameters(ListParams {
            filter,
            order_by,
            limit,
        }) = params;

        let out = self
            .channel_service
            .list_channels(filter, order_by, limit)
            .await
            .map(|channels| serde_json::json!({ "channels": channels }))
            .map_err(from_anyhow)?;

        Ok(CallToolResult::structured(out))
    }
}
