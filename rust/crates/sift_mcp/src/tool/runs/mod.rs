use rmcp::{handler::server::wrapper::Parameters, model::CallToolResult, tool, tool_router};

use crate::{
    error::{self, from_anyhow},
    server::SiftMcpServer,
    tool::common::ListParams,
};

#[cfg(test)]
mod test;

#[tool_router(router = runs_router, vis = "pub(crate)")]
impl SiftMcpServer {
    #[tool(
        name = "list_runs",
        description = "
            List runs in Sift, optionally filtered by a CEL expression and ordered by one or more fields.

            Output:
              - `{ \"runs\": [Run, ...] }`. Each item is the full Sift `Run` shape including `run_id`, `name`,
                `asset_id`/`asset_name`, `client_key`, `start_time`, `stop_time`, duration, annotation state, tags,
                and metadata.

            Parameters:
              - `filter`: CEL expression. Pass an empty string to list everything. Filterable fields:
                `run_id`, `organization_id`, `asset_id`, `asset_name`, `client_key`, `name`, `description`,
                `start_time`, `stop_time`, `duration`, `duration_string`, `tag_id`, `asset_tag_id`,
                `annotation_comments_count`, `annotation_state`, `created_date`, `modified_date`, `archived_date`,
                `is_archived`, `created_by_user_id`, `modified_by_user_id`, `metadata`.
                Reference metadata entries as `metadata.{key}`.
                `duration` is elapsed seconds (numeric). `duration_string` accepts `h`/`m`/`s`/`ms` suffixes via the
                `duration(...)` helper, e.g. `duration_string > duration('10h')`.
              - `order_by`: optional comma-separated `FIELD_NAME[ desc]` list. Orderable fields: `name`,
                `description`, `created_date`, `modified_date`, `start_time`, `stop_time`. Default sort is
                `created_date desc` (newest first). Example: `\"created_date desc,modified_date\"`.
              - `limit`: optional cap on returned items. Values in `1..=1000` cap the result set. Omitting it OR
                passing a value above 1000 returns ALL matching runs (paginated server-side).

            Errors:
              - `INVALID_PARAMS` if `filter` is not a valid CEL expression or `order_by` references an unknown field.
              - `INTERNAL_ERROR` for upstream gRPC failures.

            Guidance:
              - When the caller already knows the asset, narrow with `asset_id == \"...\"` first — it's the most
                selective field.
              - To find runs covering a specific moment, filter on both `start_time` and `stop_time` rather than
                pulling everything and filtering client-side.
              - Order by `start_time desc` when surfacing the most recent runs to a user.
        ",
        annotations(title = "runs_router/list_runs", read_only_hint = true)
    )]
    pub async fn list_runs(&self, params: Parameters<ListParams>) -> error::McpResult {
        let Parameters(ListParams {
            filter,
            order_by,
            limit,
        }) = params;

        let out = self
            .run_service
            .list_runs(filter, order_by, limit)
            .await
            .map(|runs| serde_json::json!({ "runs": runs }))
            .map_err(from_anyhow)?;

        Ok(CallToolResult::structured(out))
    }
}
