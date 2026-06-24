use rmcp::{handler::server::wrapper::Parameters, model::CallToolResult, tool, tool_router};

use crate::{
    error::{self, from_anyhow},
    server::SiftMcpServer,
    tool::common::ListParams,
};

#[cfg(test)]
mod test;

#[tool_router(router = assets_router, vis = "pub(crate)")]
impl SiftMcpServer {
    #[tool(
        name = "list_assets",
        description = "
            List assets in Sift, optionally filtered by a CEL expression and ordered by one or more fields.

            Output:
              - `{ \"assets\": [Asset, ...] }`. Each item is the full Sift `Asset` shape including `asset_id`, `name`,
                tags, metadata, timestamps, and archive state.

            Parameters:
              - `filter`: CEL expression. Pass an empty string to list everything. Filterable fields:
                `asset_id`, `name`, `name_lower`, `tag_id`, `tag_name`, `created_date`, `modified_date`,
                `archived_date`, `is_archived`, `created_by_user_id`, `modified_by_user_id`, `metadata`.
                Reference metadata entries as `metadata.{key}` (e.g. `metadata.vehicle_type == \"rover\"`).
              - `order_by`: optional comma-separated `FIELD_NAME[ desc]` list. Orderable fields: `name`,
                `created_date`, `modified_date`, `archived_date`. Default sort is `created_date desc` (newest first).
                Example: `\"created_date desc,modified_date\"`.
              - `limit`: optional cap on returned items. Values in `1..=1000` cap the result set. Omitting it OR
                passing a value above 1000 returns ALL matching assets (paginated server-side).

            Errors:
              - `INVALID_PARAMS` if `filter` is not a valid CEL expression or `order_by` references an unknown field.
              - `INTERNAL_ERROR` for upstream gRPC failures.

            Guidance:
              - Always prefer narrowing the filter over relying on `limit` — very large unfiltered listings can be slow.
              - Use `is_archived == false` to exclude archived assets unless they're explicitly needed.
        ",
        annotations(title = "assets_router/list_assets", read_only_hint = true)
    )]
    pub async fn list_assets(&self, params: Parameters<ListParams>) -> error::McpResult {
        let Parameters(ListParams {
            filter,
            limit,
            order_by,
        }) = params;

        let out = self
            .asset_service
            .list_assets(filter, order_by, limit)
            .await
            .map(|assets| serde_json::json!({ "assets": assets }))
            .map_err(from_anyhow)?;

        Ok(CallToolResult::structured(out))
    }
}
