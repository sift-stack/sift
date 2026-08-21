use rmcp::{handler::server::wrapper::Parameters, model::CallToolResult, tool, tool_router};

use crate::{
    error::{self, from_anyhow},
    server::SiftMcpServer,
    tool::common::ListParams,
};

#[cfg(test)]
mod test;

#[tool_router(router = tags_router, vis = "pub(crate)")]
impl SiftMcpServer {
    #[tool(
        name = "list_tags",
        description = "
            List tags in Sift, optionally filtered by a CEL expression and ordered by one or more fields.
            Wraps `tags/v2 ListTags`.

            Output:
              - `{ \"tags\": [Tag, ...] }`. Each item has `tag_id`, `name`, `created_by_user_id`, and
                `created_date`.

            Parameters:
              - `filter`: CEL expression. Pass an empty string to list everything. Filterable fields:
                `name`, `tag_id`.
                When filtering or searching, use `name.matches(\"(?i)prod\")`, not `==`. Use `==` only for an
                exact value from a prior result. `contains`/`startsWith`/`endsWith` are case-SENSITIVE:
                `contains(\"Prod\")` silently misses `production`.
              - `order_by`: optional comma-separated `FIELD_NAME[ desc]` list. Orderable fields: `created_date`,
                `name`. If left empty, items are ordered by `created_date` ascending (oldest first) — this
                differs from `list_assets`/`list_runs` (newest first). Example: `\"name,created_date desc\"`.
              - `limit`: max items to return. Start at 50 and only raise it if the result is capped
                and you still need more. Values are clamped to `1..=200`; omitting it defaults to 50.

            Errors:
              - `INVALID_PARAMS` if `filter` is not a valid CEL expression or `order_by` references an unknown
                field.
              - `INTERNAL_ERROR` for upstream gRPC failures.

            Guidance:
              - Check here before inventing new taxonomy on an asset, run, or annotation — a near-duplicate name
                (`prod` vs `production`) fragments filtering later.
              - There is no `create_tag` tool: `update_asset` and `update_run` create a tag by name implicitly
                the first time it's used, so tagging is done through those tools directly.
        ",
        annotations(title = "tags/list_tags", read_only_hint = true)
    )]
    pub async fn list_tags(&self, params: Parameters<ListParams>) -> error::McpResult {
        let Parameters(ListParams {
            filter,
            order_by,
            limit,
        }) = params;

        let out = self
            .tag_service
            .list_tags(filter, order_by, limit)
            .await
            .map(|tags| serde_json::json!({ "tags": tags }))
            .map_err(from_anyhow)?;

        Ok(CallToolResult::structured(out))
    }
}
