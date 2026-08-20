use rmcp::{
    ErrorData,
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
pub struct ListUsersParams {
    pub(crate) filter: String,
    pub(crate) order_by: Option<String>,
    pub(crate) limit: Option<u32>,
    pub(crate) include_inactive: Option<bool>,
    pub(crate) me: Option<bool>,
}

#[tool_router(router = users_router, vis = "pub(crate)")]
impl SiftMcpServer {
    #[tool(
        name = "list_users",
        description = "
            List users in the caller's organization, optionally filtered by a CEL expression and ordered by one or
            more fields. Use it to resolve a person — by name, email, or id — to the `user_id` that the other
            `list_*` tools filter on.

            Output:
              - `{ \"users\": [User, ...] }`. Each item is the full Sift `User` shape: `user_id`, `user_name`, and
                the `organizations` the user belongs to. `user_name` is the sign-in identifier, typically an email
                address; Sift does not currently store a separate display, first, or last name.

            Parameters:
              - `filter`: CEL expression over `user_id` and `name` (`name` is the user's `user_name`). Pass an empty
                string to list everyone. When searching for a person, use `name.matches(\"(?i)jane\")`, not `==`.
                Use `==` only for an exact address from a prior result. `contains`/`startsWith`/`endsWith` are
                case-SENSITIVE: `contains(\"Jane\")` silently misses `jane@siftstack.com`.
                `name.matches(\"(?i)^(jane|john)@\")` resolves several people at once;
                `user_id in [\"<uuid>\"]` maps ids back to names. Addresses
                contain `.` and `+`, both regex metacharacters, so use `contains` to match one literally.
              - `order_by`: optional comma-separated `FIELD_NAME[ desc]` list. Orderable fields: `name`,
                `created_date`, `modified_date`; with `include_inactive` set, only `created_date` and
                `modified_date`. Default sort is `name` ascending (A-Z). Example: `\"created_date desc,name\"`.
              - `limit`: max items to return. Start at 50 and only raise it if the result is capped and you still
                need more. Values are clamped to `1..=200`; omitting it defaults to 50.
              - `include_inactive`: optional; defaults to false, listing only users active in the organization. Set
                true to also return deactivated accounts — needed when attributing older records to someone who has
                since left.
              - `me`: optional; defaults to false. Set true to return the user whose credentials the server is
                running under, resolved from the API key itself. Needs no configuration. Mutually exclusive with a
                non-empty `filter`; `order_by`, `limit`, and `include_inactive` are ignored when it is set.

            Errors:
              - `INVALID_PARAMS` if `filter` is not a valid CEL expression, `order_by` references an unknown field,
                or `me` is set alongside a non-empty `filter`.
              - `INTERNAL_ERROR` for upstream gRPC failures.

            Guidance:
              - Resolve a person here first, then filter on `created_by_user_id == \"<user_id>\"` in whichever
                `list_*` tool the question is about. Runs, assets, channels, rules, reports, annotations, and test
                reports all expose that field; all but annotations also expose `modified_by_user_id`.
              - When the question is about the caller rather than a named person (\"my runs\", \"reports I filed\"),
                use `me: true` and take the `user_id` from the result. Never guess which listed user is the caller.
        ",
        annotations(title = "users/list_users", read_only_hint = true)
    )]
    pub async fn list_users(&self, params: Parameters<ListUsersParams>) -> error::McpResult {
        let Parameters(ListUsersParams {
            filter,
            order_by,
            limit,
            include_inactive,
            me,
        }) = params;

        let include_inactive = include_inactive.unwrap_or(false);

        if me.unwrap_or(false) {
            if !filter.is_empty() {
                return Err(ErrorData::invalid_params(
                    "`me` and `filter` are mutually exclusive; set one or the other",
                    None,
                ));
            }

            let user = self.user_service.get_me().await.map_err(from_anyhow)?;

            return Ok(CallToolResult::structured(
                serde_json::json!({ "users": [user] }),
            ));
        }

        let users = self
            .user_service
            .list_users(filter, order_by, limit, include_inactive)
            .await
            .map_err(from_anyhow)?;

        Ok(CallToolResult::structured(
            serde_json::json!({ "users": users }),
        ))
    }
}
