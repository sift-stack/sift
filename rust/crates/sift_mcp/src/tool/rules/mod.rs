use rmcp::{
    ErrorData,
    handler::server::wrapper::Parameters,
    model::{CallToolResult, Content},
    schemars::{self, JsonSchema},
    tool, tool_router,
};
use serde::Deserialize;
use sift_rs::rules::v1::{UpdateConditionRequest, UpdateRuleRequest};

use crate::{
    error::{self, from_anyhow},
    server::SiftMcpServer,
    service::rules::RuleUpdate,
    tool::common::{ListParams, url_clause, with_urls},
};

#[cfg(test)]
mod test;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct RuleDefinitionParams {
    rule_json: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct RuleVersionListParams {
    rule_id: String,
    filter: Option<String>,
    limit: Option<u32>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct UpdateRuleParams {
    rule_id: String,
    name: Option<String>,
    description: Option<String>,
    version_notes: Option<String>,
    is_live_evaluation_enabled: Option<bool>,
    asset_ids: Option<Vec<String>>,
    tag_ids: Option<Vec<String>>,
    conditions_json: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct RuleArchiveParams {
    rule_id: Option<String>,
    client_key: Option<String>,
}

#[tool_router(router = rules_router, vis = "pub(crate)")]
impl SiftMcpServer {
    #[tool(
        name = "list_rules",
        description = "
            List rules in Sift, optionally filtered by a CEL expression and ordered by one or more fields.

            Output:
              - `{ \"rules\": [Rule, ...] }`. Each item is the full Sift `Rule` shape including `rule_id`,
                `client_key`, `name`, `description`, `is_enabled`, `is_external`, `is_archived`,
                `archived_date`, `is_live_evaluation_enabled`, `organization_id`, `conditions`, `rule_version`,
                `current_version_id`, `asset_configuration` (asset_ids + tag_ids), `contextual_channels`,
                `metadata`, and timestamps, plus an added `url` field with the rule's Sift web link
                (`<host>/rules/<rule_id>`). `url` is omitted when the host can't be derived. Surface these links
                to the user when presenting rules.

            Parameters:
              - `filter`: CEL expression. Pass an empty string to list everything. Filterable fields:
                `rule_id`, `client_key`, `name`, `description`, `is_external`, `asset_id`, `tag_id`,
                `created_date`, `created_by_user_id`, `metadata`, `modified_date`, `modified_by_user_id`,
                `deleted_date`, `is_archived`, `archived_date`, `is_live_evaluation_enabled`.
                Reference metadata entries as `metadata.{key}` (e.g. `metadata.severity == \"high\"`).
              - `order_by`: optional comma-separated `FIELD_NAME[ desc]` list. Orderable fields:
                `created_date`, `modified_date`. Default sort is `created_date desc` (newest first).
                Example: `\"created_date desc,modified_date\"`.
              - `limit`: optional cap on returned items. Values in `1..=1000` cap the result set. Omitting it OR
                passing a value above 1000 returns ALL matching rules (paginated server-side).

            Errors:
              - `INVALID_PARAMS` if `filter` is not a valid CEL expression or `order_by` references an unknown field.
              - `INTERNAL_ERROR` for upstream gRPC failures.

            Guidance:
              - Scope with `asset_id == \"...\"` when the rule's target asset is known — it's the most selective
                field for narrowing rule listings.
              - Use `is_archived == false` to exclude archived rules unless they're explicitly needed.
              - Use `is_live_evaluation_enabled == true` to find only rules that run against live data.
        ",
        annotations(title = "rules_router/list_rules", read_only_hint = true)
    )]
    pub async fn list_rules(&self, params: Parameters<ListParams>) -> error::McpResult {
        let Parameters(ListParams {
            filter,
            order_by,
            limit,
        }) = params;

        let rules = self
            .rule_service
            .list_rules(filter, order_by, limit)
            .await
            .map_err(from_anyhow)?;

        let rules = with_urls(&rules, |r| self.url_service.build_rule_url(&r.rule_id).ok())?;

        Ok(CallToolResult::structured(
            serde_json::json!({ "rules": rules }),
        ))
    }

    #[tool(
        name = "list_rule_versions",
        description = "
            List the version history of a single Sift rule, newest entries returned by the server, optionally
            filtered by a CEL expression.

            Output:
              - `{ \"rule_versions\": [RuleVersion, ...], \"rule_url\": string|null, \"next_step\": string }`. Each
                item includes `rule_id`, `rule_version_id`, `version`, `created_date`, `created_by_user_id`,
                `version_notes`, and `generated_change_message`. `rule_url` is the rule's Sift web link, or null
                when the host can't be derived.

            Parameters:
              - `rule_id`: required. The rule whose versions to list.
              - `filter`: optional CEL expression. Filterable fields: `rule_version_id`, `user_notes`, and
                `change_message`. Omit or pass an empty string to list all versions.
              - `limit`: optional cap on returned items. Values in `1..=1000` cap the result set. Omitting it OR
                passing a value above 1000 returns ALL versions (paginated server-side).

            Errors:
              - `INVALID_PARAMS` if `filter` is not a valid CEL expression.
              - `INTERNAL_ERROR` for upstream gRPC failures.

            Guidance:
              - Use this to review how a rule changed over time, or to find a specific `rule_version_id` before
                inspecting or referencing that version. Resolve the `rule_id` with `list_rules` first if you only
                have the rule name.
        ",
        annotations(title = "rules_router/list_rule_versions", read_only_hint = true)
    )]
    pub async fn list_rule_versions(
        &self,
        params: Parameters<RuleVersionListParams>,
    ) -> error::McpResult {
        let Parameters(RuleVersionListParams {
            rule_id,
            filter,
            limit,
        }) = params;

        let rule_url = rule_url_for(self, &rule_id);

        let rule_versions = self
            .rule_service
            .list_rule_versions(rule_id, filter.unwrap_or_default(), limit)
            .await
            .map_err(from_anyhow)?;

        let next_step = format!(
            "Listed {} rule versions.{} Surface the version history to the user.",
            rule_versions.len(),
            url_clause(rule_url.as_deref()),
        );

        let mut result = CallToolResult::structured(serde_json::json!({
            "rule_versions": rule_versions,
            "rule_url": rule_url,
            "next_step": next_step,
        }));
        result.content = vec![Content::text(next_step)];
        Ok(result)
    }

    #[tool(
        name = "create_rule",
        description = "
            Create a Sift rule from a full rule definition supplied as a JSON string. This is a WRITE.

            Output:
              - `{ \"rule_id\": \"<id>\", \"rule_url\": string|null, \"next_step\": \"...\" }`. `rule_id` is the
                server-assigned id of the new rule; `rule_url` is its Sift web link (`<host>/rules/<rule_id>`), or
                null when the host can't be derived.

            Parameters:
              - `rule_json`: a JSON object matching `protos/sift/rules/v1/rules.proto::UpdateRuleRequest`. Use the
                proto field names. Omit `rule_id` (this is a create). Key fields:
                - `name` (required), `description` (required).
                - `conditions` (required): array of `{ \"expression\": {...}, \"actions\": [...] }`. The
                  expression and action shapes are non-trivial; mirror an existing rule retrieved via
                  `list_rules` rather than authoring blind.
                - `asset_configuration`: `{ \"asset_ids\": [...], \"tag_ids\": [...] }` — which assets/tags the
                  rule applies to.
                - `metadata` (required): array of metadata values (may be empty: `[]`).
                - `client_key` (optional): a caller-defined identifier. Immutable after creation.
                - `is_live_evaluation_enabled` (optional): evaluate against live data.

            Errors:
              - `INVALID_PARAMS` if `rule_json` is not valid JSON or does not match the rule schema.
              - `INTERNAL_ERROR` for upstream gRPC failures (e.g. a referenced asset does not exist).

            Guidance:
              - Rule definitions are complex. Retrieve a similar rule with `list_rules` first and adapt its shape;
                do not invent `conditions`/`actions` structures.
              - This creates a live resource. Confirm the rule's name, the target assets/tags, and the conditions
                with the user before calling. Do not silently default them.
        ",
        annotations(
            title = "rules_router/create_rule",
            read_only_hint = false,
            destructive_hint = false,
            idempotent_hint = false,
        )
    )]
    pub async fn create_rule(&self, params: Parameters<RuleDefinitionParams>) -> error::McpResult {
        let Parameters(RuleDefinitionParams { rule_json }) = params;

        let update = parse_rule_definition(&rule_json)?;

        let rule_id = self
            .rule_service
            .create_rule(update)
            .await
            .map_err(from_anyhow)?;

        let rule_url = self.url_service.build_rule_url(&rule_id).ok();
        let next_step = format!(
            "Created rule with id `{rule_id}`.{} Tell the user the new rule id. If they haven't \
             indicated a next step, offer to confirm it with `list_rules` \
             (filter `rule_id == \"{rule_id}\"`).",
            url_clause(rule_url.as_deref()),
        );

        let mut result = CallToolResult::structured(serde_json::json!({
            "rule_id": rule_id,
            "rule_url": rule_url,
            "next_step": next_step,
        }));
        result.content = vec![Content::text(next_step)];
        Ok(result)
    }

    #[tool(
        name = "update_rule",
        description = "
            Update specific fields of an existing Sift rule, identified by `rule_id`. This is a WRITE. Only the
            fields you set are changed; everything else on the rule is preserved. The tool fetches the current
            rule, overlays your changes, and saves the result, so it is safe to call with just the fields you
            want to modify.

            Output:
              - `{ \"rule_id\": \"<id>\", \"rule_url\": string|null, \"next_step\": \"...\" }`. `rule_url` is the
                rule's Sift web link, or null when the host can't be derived.

            Parameters:
              - `rule_id`: required. The rule to update.
              - `name`: optional. New rule name.
              - `description`: optional. New description.
              - `version_notes`: optional. Notes attached to this update in the rule's version history.
              - `is_live_evaluation_enabled`: optional. Enable or disable live evaluation.
              - `asset_ids`: optional. Replaces the rule's set of target asset ids.
              - `tag_ids`: optional. Replaces the rule's set of target tag ids.
              - `conditions_json`: optional. Replaces the rule's conditions. A JSON array matching the
                `conditions` field of `create_rule`'s `rule_json`. Omit it to keep the existing conditions
                unchanged — only provide it when you intend to rewrite the rule's logic.
              - At least one field besides `rule_id` must be set.

            Errors:
              - `INVALID_PARAMS` if no field besides `rule_id` is set, or if `conditions_json` is not a valid
                conditions array.
              - `RESOURCE_NOT_FOUND` if no rule has the given `rule_id`.
              - `INTERNAL_ERROR` for upstream gRPC failures.

            Guidance:
              - Prefer this over recreating a rule. To change only the name, pass `rule_id` and `name`; the
                conditions, metadata, and asset targeting are preserved automatically.
              - Confirm the intended changes with the user before calling.
        ",
        annotations(
            title = "rules_router/update_rule",
            read_only_hint = false,
            destructive_hint = true,
            idempotent_hint = true,
        )
    )]
    pub async fn update_rule(&self, params: Parameters<UpdateRuleParams>) -> error::McpResult {
        let Parameters(UpdateRuleParams {
            rule_id,
            name,
            description,
            version_notes,
            is_live_evaluation_enabled,
            asset_ids,
            tag_ids,
            conditions_json,
        }) = params;

        let conditions = conditions_json
            .as_deref()
            .map(parse_conditions)
            .transpose()?;

        let changes = RuleUpdate {
            name,
            description,
            version_notes,
            is_live_evaluation_enabled,
            asset_ids,
            tag_ids,
            conditions,
        };

        let nothing_to_change = changes.name.is_none()
            && changes.description.is_none()
            && changes.version_notes.is_none()
            && changes.is_live_evaluation_enabled.is_none()
            && changes.asset_ids.is_none()
            && changes.tag_ids.is_none()
            && changes.conditions.is_none();
        if nothing_to_change {
            return Err(ErrorData::invalid_params(
                "at least one field besides `rule_id` must be set",
                None,
            ));
        }

        let rule_id = self
            .rule_service
            .update_rule(rule_id, changes)
            .await
            .map_err(from_anyhow)?;

        let rule_url = self.url_service.build_rule_url(&rule_id).ok();
        let next_step = format!(
            "Updated rule `{rule_id}`.{} Tell the user the update succeeded. If they haven't \
             indicated a next step, offer to confirm it with `list_rules` \
             (filter `rule_id == \"{rule_id}\"`).",
            url_clause(rule_url.as_deref()),
        );

        let mut result = CallToolResult::structured(serde_json::json!({
            "rule_id": rule_id,
            "rule_url": rule_url,
            "next_step": next_step,
        }));
        result.content = vec![Content::text(next_step)];
        Ok(result)
    }

    #[tool(
        name = "archive_rule",
        description = "
            Archive a Sift rule so it no longer evaluates. This is a WRITE. Reversible with `unarchive_rule`.

            Output:
              - `{ \"archived\": true, \"rule_url\": string|null, \"next_step\": \"...\" }`. `rule_url` is the
                rule's Sift web link when targeted by `rule_id`; null when targeted by `client_key` or when the
                host can't be derived.

            Parameters:
              - `rule_id`: optional. The id of the rule to archive.
              - `client_key`: optional. The caller-defined key of the rule to archive.
              - Exactly one of `rule_id` or `client_key` must be set.

            Errors:
              - `INVALID_PARAMS` if neither or both of `rule_id` and `client_key` are set.
              - `INTERNAL_ERROR` for upstream gRPC failures.

            Guidance:
              - Archiving stops live evaluation but does not delete the rule; it can be restored with
                `unarchive_rule`. Confirm the target rule with the user before calling.
        ",
        annotations(
            title = "rules_router/archive_rule",
            read_only_hint = false,
            destructive_hint = true,
            idempotent_hint = true,
        )
    )]
    pub async fn archive_rule(&self, params: Parameters<RuleArchiveParams>) -> error::McpResult {
        let Parameters(RuleArchiveParams {
            rule_id,
            client_key,
        }) = params;

        let (rule_id, client_key) = rule_identifier(rule_id, client_key)?;

        let rule_url = rule_url_for(self, &rule_id);

        self.rule_service
            .archive_rule(rule_id, client_key)
            .await
            .map_err(from_anyhow)?;

        let next_step = format!(
            "Rule archived.{} Tell the user it is archived and will no longer evaluate, \
             and that `unarchive_rule` restores it.",
            url_clause(rule_url.as_deref()),
        );

        let mut result = CallToolResult::structured(serde_json::json!({
            "archived": true,
            "rule_url": rule_url,
            "next_step": next_step,
        }));
        result.content = vec![Content::text(next_step)];
        Ok(result)
    }

    #[tool(
        name = "unarchive_rule",
        description = "
            Restore a previously archived Sift rule so it evaluates again. This is a WRITE.

            Output:
              - `{ \"unarchived\": true, \"rule_url\": string|null, \"next_step\": \"...\" }`. `rule_url` is the
                rule's Sift web link when targeted by `rule_id`; null when targeted by `client_key` or when the
                host can't be derived.

            Parameters:
              - `rule_id`: optional. The id of the rule to unarchive.
              - `client_key`: optional. The caller-defined key of the rule to unarchive.
              - Exactly one of `rule_id` or `client_key` must be set.

            Errors:
              - `INVALID_PARAMS` if neither or both of `rule_id` and `client_key` are set.
              - `INTERNAL_ERROR` for upstream gRPC failures.

            Guidance:
              - Confirm the target rule with the user before calling.
        ",
        annotations(
            title = "rules_router/unarchive_rule",
            read_only_hint = false,
            destructive_hint = true,
            idempotent_hint = true,
        )
    )]
    pub async fn unarchive_rule(&self, params: Parameters<RuleArchiveParams>) -> error::McpResult {
        let Parameters(RuleArchiveParams {
            rule_id,
            client_key,
        }) = params;

        let (rule_id, client_key) = rule_identifier(rule_id, client_key)?;

        let rule_url = rule_url_for(self, &rule_id);

        self.rule_service
            .unarchive_rule(rule_id, client_key)
            .await
            .map_err(from_anyhow)?;

        let next_step = format!(
            "Rule unarchived.{} Tell the user it is restored and will evaluate again.",
            url_clause(rule_url.as_deref()),
        );

        let mut result = CallToolResult::structured(serde_json::json!({
            "unarchived": true,
            "rule_url": rule_url,
            "next_step": next_step,
        }));
        result.content = vec![Content::text(next_step)];
        Ok(result)
    }
}

/// Build the web URL for a rule when its `rule_id` is known. Returns `None` when
/// `rule_id` is empty (the caller targeted the rule by `client_key`, so there is
/// no id to link) or when the host can't be derived.
fn rule_url_for(server: &SiftMcpServer, rule_id: &str) -> Option<String> {
    if rule_id.is_empty() {
        return None;
    }
    server.url_service.build_rule_url(rule_id).ok()
}

/// Deserialize a rule definition JSON string into an `UpdateRuleRequest`,
/// mapping any parse error to `INVALID_PARAMS` so the agent can correct it.
fn parse_rule_definition(rule_json: &str) -> Result<UpdateRuleRequest, ErrorData> {
    serde_json::from_str::<UpdateRuleRequest>(rule_json).map_err(|e| {
        ErrorData::invalid_params(
            format!("`rule_json` is not a valid rule definition: {e}"),
            None,
        )
    })
}

/// Deserialize a conditions JSON array into `Vec<UpdateConditionRequest>`,
/// mapping any parse error to `INVALID_PARAMS`.
fn parse_conditions(conditions_json: &str) -> Result<Vec<UpdateConditionRequest>, ErrorData> {
    serde_json::from_str::<Vec<UpdateConditionRequest>>(conditions_json).map_err(|e| {
        ErrorData::invalid_params(
            format!("`conditions_json` is not a valid conditions array: {e}"),
            None,
        )
    })
}

/// Resolve the `(rule_id, client_key)` request fields from the mutually
/// exclusive optional params, returning `INVALID_PARAMS` unless exactly one is set.
fn rule_identifier(
    rule_id: Option<String>,
    client_key: Option<String>,
) -> Result<(String, String), ErrorData> {
    match (rule_id, client_key) {
        (Some(r), None) => Ok((r, String::new())),
        (None, Some(c)) => Ok((String::new(), c)),
        (Some(_), Some(_)) => Err(ErrorData::invalid_params(
            "exactly one of `rule_id` or `client_key` must be set, not both",
            None,
        )),
        (None, None) => Err(ErrorData::invalid_params(
            "one of `rule_id` or `client_key` must be set",
            None,
        )),
    }
}
