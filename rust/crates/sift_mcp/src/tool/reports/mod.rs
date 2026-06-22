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

#[cfg(test)]
mod test;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct ReportListParams {
    pub(crate) filter: String,
    pub(crate) order_by: Option<String>,
    pub(crate) limit: Option<u32>,
    pub(crate) organization_id: Option<String>,
}

#[tool_router(router = reports_router, vis = "pub(crate)")]
impl SiftMcpServer {
    #[tool(
        name = "list_reports",
        description = "
            List reports in Sift, optionally filtered by a CEL expression and ordered by one or more fields.

            Output:
              - `{ \"reports\": [Report, ...] }`. Each item is the full Sift `Report` shape including `report_id`,
                `report_template_id`, `run_id`, `organization_id`, `name`, `description`, per-rule `summaries`,
                tags, metadata, timestamps, and archive state.

            Parameters:
              - `filter`: CEL expression. Pass an empty string to list everything. Filterable fields:
                `report_id`, `report_template_id`, `tag_name`, `name`, `run_id`, `is_archived`, `archived_date`,
                `created_date`, `created_by_user_id`, `metadata`, `modified_date`, `modified_by_user_id`.
                Reference metadata entries as `metadata.{key}` (e.g. `metadata.batch == \"nightly\"`).
              - `order_by`: optional comma-separated `FIELD_NAME[ desc]` list. Orderable fields: `name`,
                `created_date`, `modified_date`. Default sort is `created_date desc` (newest first).
                Example: `\"created_date desc,modified_date\"`.
              - `limit`: optional cap on returned items. Values in `1..=1000` cap the result set. Omitting it OR
                passing a value above 1000 returns ALL matching reports (paginated server-side).
              - `organization_id`: optional. Required only when the caller belongs to multiple organizations; it
                scopes the listing to that org. Omit it for single-organization users.

            Errors:
              - `INVALID_PARAMS` if `filter` is not a valid CEL expression or `order_by` references an unknown field.
              - `INTERNAL_ERROR` for upstream gRPC failures.

            Guidance:
              - When the report's run is known, narrow with `run_id == \"...\"` first — it's the most selective field.
              - Use `is_archived == false` to exclude archived reports unless they're explicitly needed.
              - Order by `created_date desc` when surfacing the most recent reports to a user.
        ",
        annotations(title = "reports_router/list_reports", read_only_hint = true)
    )]
    pub async fn list_reports(&self, params: Parameters<ReportListParams>) -> error::McpResult {
        let Parameters(ReportListParams {
            filter,
            order_by,
            limit,
            organization_id,
        }) = params;

        let out = self
            .report_service
            .list_reports(filter, order_by, limit, organization_id)
            .await
            .map(|reports| serde_json::json!({ "reports": reports }))
            .map_err(from_anyhow)?;

        Ok(CallToolResult::structured(out))
    }
}
