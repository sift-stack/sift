use rmcp::{handler::server::wrapper::Parameters, model::CallToolResult, tool, tool_router};

use crate::{
    error::{self, from_anyhow},
    server::SiftMcpServer,
    tool::common::{ListParams, list_body, to_values},
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
              - `count`: how many items THIS response carries — read it instead of
                counting the array yourself. It is the size of the page you got back, not
                how many items match `filter`.
              - `has_more`: `true` when the service hit `limit` with matches left over, so
                this page is not the whole set. Never report `count` as a total while
                `has_more` is `true` — narrow `filter` or raise `limit` and ask again.

            Parameters:
              - `filter`: CEL expression. Pass an empty string to list everything. Filterable fields:
                `channel_id`, `asset_id`, `name`, `description`, `run_id`, `run_name`, `run_client_key`,
                `created_date`, `modified_date`, `created_by_user_id`, `modified_by_user_id`.
                When filtering or searching, use `name.matches(\"(?i)motor\")`, not `==`. Use `==` only for an
                exact value from a prior result. `contains`/`startsWith`/`endsWith` are case-SENSITIVE:
                `contains(\"Motor\")` silently misses `motor_d.current`. Channel names embed `.`, a regex wildcard,
                so match a full literal name with `contains`, not `matches`.
              - `order_by`: optional comma-separated `FIELD_NAME[ desc]` list. Orderable fields: `name`,
                `created_date`, `modified_date`, `active`. Default sort is `created_date` ascending (oldest first) —
                note this differs from `list_assets` and `list_runs`. Example: `\"name,created_date desc\"`.
              - `limit`: max items to return. Start at 50 and only raise it if the result is capped
                and you still need more. Values are clamped to `1..=200`; omitting it defaults to 50.
              - `fields`: optional array of field names to keep on each item, e.g.
                `[\"name\"]`. Omit it for the full object. Names match case-insensitively
                and ignore underscores, so `asset_id` and `assetId` both work. Any name
                that matched nothing is returned in `unmatched_fields` beside the results.
                Reach for this whenever you need only a few fields: full objects are wide,
                and a large listing can exceed the response size limit without it.

            Errors:
              - `INVALID_PARAMS` if `filter` is not a valid CEL expression or `order_by` references an unknown field.
              - `INTERNAL_ERROR` for upstream gRPC failures.

            Guidance:
              - Always scope with `asset_id == \"...\"` when the asset is known — channel namespaces are per-asset
                and unscoped queries return cross-asset results.
              - To enumerate channels recorded by a specific run, filter on `run_id` rather than joining client-side.
        ",
        annotations(title = "channels/list_channels", read_only_hint = true)
    )]
    pub async fn list_channels(&self, params: Parameters<ListParams>) -> error::McpResult {
        let Parameters(ListParams {
            filter,
            order_by,
            limit,
            fields,
        }) = params;

        let page = self
            .channel_service
            .list_channels(filter, order_by, limit)
            .await
            .map_err(from_anyhow)?;

        let channels = to_values(&page.items)?;

        Ok(CallToolResult::structured(list_body(
            "channels",
            channels,
            fields,
            page.has_more,
        )))
    }
}
