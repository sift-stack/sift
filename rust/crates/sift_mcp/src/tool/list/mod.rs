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
pub struct ListParams {
    filter: String,
    order_by: Option<String>,
    limit: Option<u32>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct ReportListParams {
    filter: String,
    order_by: Option<String>,
    limit: Option<u32>,
    organization_id: Option<String>,
}

#[tool_router(router = list_router, vis = "pub(crate)")]
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
        annotations(title = "list_router/list_assets", read_only_hint = true)
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
        annotations(title = "list_router/list_runs", read_only_hint = true)
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
        annotations(title = "list_router/list_channels", read_only_hint = true)
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
        annotations(title = "list_router/list_reports", read_only_hint = true)
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
