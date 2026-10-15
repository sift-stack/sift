use rmcp::{handler::server::wrapper::Parameters, model::CallToolResult, tool, tool_router};

use crate::{
    error::{self, from_anyhow},
    server::SiftMcpServer,
    tool::common::{ListParams, list_body, to_values},
};

#[cfg(test)]
mod test;

#[tool_router(router = webhooks_router, vis = "pub(crate)")]
impl SiftMcpServer {
    #[tool(
        name = "list_webhooks",
        description = "
            List the webhooks a Sift rule can call when it fires, optionally filtered by a CEL expression. Use it to
            resolve a webhook the user names to the `webhook_id` that `create_rule` and `update_rule` take.

            Output:
              - `{ \"webhooks\": [Webhook, ...] }`. Each item carries `webhook_id`, `name`, `event_type`,
                `target_url`, `http_headers`, `is_archived`, `archived_date`, and creator, modifier, and
                timestamp fields.
              - Credentials are withheld. `target_url` is cut to its scheme and the last two labels of its host
                (`https://hooks.slack.com/services/...` becomes `https://slack.com`) and is omitted when it can't
                be parsed. Each `http_headers` entry carries only its `name`, and the request body template is
                omitted. Attaching a webhook needs none of them, so do not ask the user for them.
              - Fields at their proto3 default are OMITTED: a missing `is_archived` key means `false`.
              - `count`: how many items THIS response carries, not how many match `filter`.
              - `has_more`: `true` when the service hit `limit` with matches left over. Never report `count` as a
                total while `has_more` is `true`.

            Parameters:
              - `filter`: CEL expression. Pass an empty string to list everything, archived webhooks included.
                Filterable fields: `webhook_id`, `name`, `event_type`, `is_archived`, `archived_date`. `event_type`
                takes the enum name: `event_type == \"WEBHOOK_EVENT_TYPE_RULE_VIOLATION\"`.
                When filtering or searching, use `name.matches(\"(?i)slack\")`, not `==`. Use `==` only for an
                exact value from a prior result. `contains`/`startsWith`/`endsWith` are case-SENSITIVE:
                `contains(\"Slack\")` silently misses `slack-alerts`.
              - `order_by`: optional comma-separated `FIELD_NAME[ desc]` list. Orderable fields: `created_date`.
                Default sort is `created_date desc` (newest first). Example: `\"created_date\"`.
              - `limit`: max items to return. Start at 50 and only raise it if the result is capped
                and you still need more. Values are clamped to `1..=200`; omitting it defaults to 50.
              - `fields`: optional array of field names to keep on each item, e.g.
                `[\"webhook_id\", \"name\"]`. Omit it for the full object. Names match case-insensitively
                and ignore underscores and hyphens. Any name that matched nothing on any returned item
                is listed in `unmatched_fields`.

            Errors:
              - `INVALID_PARAMS` if `filter` is not a valid CEL expression or `order_by` references an unknown field.
              - `INTERNAL_ERROR` for upstream gRPC failures.

            Guidance:
              - Default add `is_archived == false` to the filter. Sift never calls an archived webhook, and the
                rule tools refuse one.
              - This server cannot create webhooks. When none fits, tell the user to create one in the Sift web app.
        ",
        annotations(title = "webhooks/list_webhooks", read_only_hint = true)
    )]
    pub async fn list_webhooks(&self, params: Parameters<ListParams>) -> error::McpResult {
        let Parameters(ListParams {
            filter,
            order_by,
            limit,
            fields,
        }) = params;

        let page = self
            .webhook_service
            .list_webhooks(filter, order_by, limit)
            .await
            .map_err(from_anyhow)?;

        let webhooks = to_values(&page.items)?;

        Ok(CallToolResult::structured(list_body(
            "webhooks",
            webhooks,
            fields,
            page.has_more,
        )))
    }
}
