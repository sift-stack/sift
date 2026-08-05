use rmcp::{
    ErrorData,
    handler::server::wrapper::Parameters,
    model::{CallToolResult, ContentBlock},
    schemars::{self, JsonSchema},
    tool, tool_router,
};
use serde::Deserialize;
use sift_rs::metadata::v1::MetadataValue;

use crate::{
    error::{self, from_anyhow},
    server::SiftMcpServer,
    service::report_templates::{ReportTemplateUpdate, TemplateRuleIdentifier},
    tool::common::MetadataEntry,
};

#[cfg(test)]
mod test;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct ReportTemplateListParams {
    filter: String,
    order_by: Option<String>,
    limit: Option<u32>,
    organization_id: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct CreateReportTemplateParams {
    name: String,
    description: Option<String>,
    client_key: Option<String>,
    tag_names: Option<Vec<String>>,
    organization_id: Option<String>,
    metadata: Option<Vec<MetadataEntry>>,
    rule_ids: Option<Vec<String>>,
    rule_client_keys: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct UpdateReportTemplateParams {
    report_template_id: String,
    name: Option<String>,
    description: Option<String>,
    tag_names: Option<Vec<String>>,
    rule_ids: Option<Vec<String>>,
    rule_client_keys: Option<Vec<String>>,
    metadata: Option<Vec<MetadataEntry>>,
    is_archived: Option<bool>,
}

#[tool_router(router = report_templates_router, vis = "pub(crate)")]
impl SiftMcpServer {
    #[tool(
        name = "list_report_templates",
        description = "
            List report templates in Sift, optionally filtered by a CEL expression and ordered by one or more fields.
            A report template is a named, reusable bundle of rules; reports created from a template inherit its
            rule set. Only standard rules (`is_external: false`) can be attached to a template; ad-hoc rules
            (`is_external: true`) cannot. Wraps `report_templates/v1 ListReportTemplates`.

            Output:
              - `{ \"report_templates\": [ReportTemplate, ...] }`. Each item carries `report_template_id`,
                `organization_id`, `client_key`, `name`, `description`, `created_date`, `modified_date`,
                `created_by_user_id`, `modified_by_user_id`, `is_archived`, `archived_date`, `tags`, `metadata`,
                and the ordered `rules` list. Each rule includes `rule_id`, `rule_version_id`,
                `rule_version_number`, `client_key`, and `display_order`.

            Parameters:
              - `filter`: CEL expression. Pass an empty string to list everything. Filterable fields:
                `report_template_id`, `tag_id`, `tag_name`, `client_key`, `metadata`, `name`, `is_archived`.
                Reference metadata entries as `metadata.{key}` (e.g. `metadata.owner == \"qa-team\"`).
                When filtering or searching, use `name.matches(\"(?i)avionics\")`, not `==`. Use `==` only for an
                exact value from a prior result. `contains`/`startsWith`/`endsWith` are case-SENSITIVE:
                `contains(\"Avionics\")` silently misses `avionics-power-limit`.
              - `order_by`: optional comma-separated `FIELD_NAME[ desc]` list. Orderable fields: `created_date`,
                `modified_date`. Default sort is `created_date desc` (newest first). Example:
                `\"created_date desc,modified_date\"`.
              - `limit`: max items to return. Start at 50 and only raise it if the result is capped and you still
                need more. Values are clamped to `1..=200`; omitting it defaults to 50.
              - `organization_id`: optional. Required only when the caller belongs to multiple organizations;
                scopes the listing to that org. Omit for single-organization users.

            Errors:
              - `INVALID_PARAMS` if `filter` is not a valid CEL expression or `order_by` references an unknown field.
              - `INTERNAL_ERROR` for upstream gRPC failures.

            Guidance:
              - Use this to discover a template before referencing it from `create_report` via `report_template_id`.
              - Always add `is_archived == false` to the filter. Include archived templates only when the user
                explicitly asks for them.
        ",
        annotations(
            title = "report_templates/list_report_templates",
            read_only_hint = true
        )
    )]
    pub async fn list_report_templates(
        &self,
        params: Parameters<ReportTemplateListParams>,
    ) -> error::McpResult {
        let Parameters(ReportTemplateListParams {
            filter,
            order_by,
            limit,
            organization_id,
        }) = params;

        let templates = self
            .report_template_service
            .list_report_templates(filter, order_by, limit, organization_id)
            .await
            .map_err(from_anyhow)?;

        Ok(CallToolResult::structured(
            serde_json::json!({ "report_templates": templates }),
        ))
    }

    #[tool(
        name = "create_report_template",
        description = "
            Create a report template from a set of standard rules. A report template is a named, reusable
            bundle of rules that any future `create_report` call can reference via `report_template_id`. Only
            standard rules (`is_external: false`) can be attached; ad-hoc rules (`is_external: true`) are
            rejected by the server. Wraps `report_templates/v1 CreateReportTemplate`.

            Output:
              - `{ \"report_template\": ReportTemplate, \"next_step\": string }`. The returned template is the
                server-assigned state including its new `report_template_id`, resolved `rules` (with
                `display_order` set), and timestamps.

            Parameters:
              - `name`: required; the template name.
              - `description`: optional; free-form description of what the template evaluates.
              - `client_key`: optional; a stable caller-defined key for referring to this template. Must be unique
                within the organization.
              - `tag_names`: optional list of tag names to attach to the template.
              - `organization_id`: optional. Required only when the caller belongs to multiple organizations.
              - `metadata`: optional list of `{ \"name\": \"<key>\", \"value\": <scalar> }` entries.

              Provide EXACTLY ONE of the following to identify the rules on the new template:
              - `rule_ids`: ordered list of rule IDs. Position in the list becomes each rule's `display_order`
                on the template (first = 0, second = 1, ...).
              - `rule_client_keys`: list of rule client keys. Server resolves keys to rules; the resulting rule
                order on the template is server-defined (not the position in this list).

            Errors:
              - `INVALID_PARAMS` if `name` is empty, or if zero or more than one of `rule_ids`/`rule_client_keys`
                is provided.
              - `INTERNAL_ERROR` for upstream gRPC failures (e.g. unknown rule).

            Guidance:
              - This is a write. CONFIRM the template name and the rule set with the user before invoking.
              - Prefer `rule_ids` when you want deterministic on-template ordering.
              - After creation, invoke `create_report` with the new `report_template_id` to run the template
                over a run.
        ",
        annotations(
            title = "report_templates/create_report_template",
            read_only_hint = false,
            destructive_hint = false,
            idempotent_hint = false,
        )
    )]
    pub async fn create_report_template(
        &self,
        params: Parameters<CreateReportTemplateParams>,
    ) -> error::McpResult {
        let Parameters(CreateReportTemplateParams {
            name,
            description,
            client_key,
            tag_names,
            organization_id,
            metadata,
            rule_ids,
            rule_client_keys,
        }) = params;

        if name.is_empty() {
            return Err(ErrorData::invalid_params("`name` must not be empty", None));
        }

        let rules = match (rule_ids, rule_client_keys) {
            (Some(ids), None) => TemplateRuleIdentifier::RuleIds(ids),
            (None, Some(keys)) => TemplateRuleIdentifier::RuleClientKeys(keys),
            (None, None) => {
                return Err(ErrorData::invalid_params(
                    "provide exactly one of `rule_ids` or `rule_client_keys` to identify the template's rules",
                    None,
                ));
            }
            (Some(_), Some(_)) => {
                return Err(ErrorData::invalid_params(
                    "provide only one of `rule_ids` or `rule_client_keys`, not both",
                    None,
                ));
            }
        };

        let metadata = metadata
            .map(|m| m.into_iter().map(MetadataValue::from).collect::<Vec<_>>())
            .unwrap_or_default();

        let template = self
            .report_template_service
            .create_report_template(
                organization_id,
                name,
                client_key,
                description,
                tag_names.unwrap_or_default(),
                rules,
                metadata,
            )
            .await
            .map_err(from_anyhow)?;

        let next_step = format!(
            "Created report template `{}` ({}) with {} rule(s). Surface it to the user. To run the template \
             over a run, call `create_report` with `report_template_id = \"{}\"`.",
            template.name,
            template.report_template_id,
            template.rules.len(),
            template.report_template_id,
        );

        let mut result = CallToolResult::structured(serde_json::json!({
            "report_template": template,
            "next_step": next_step,
        }));
        result.content = vec![ContentBlock::text(next_step)];
        Ok(result)
    }

    #[tool(
        name = "update_report_template",
        description = "
            Update selected fields on an existing report template. Wraps `report_templates/v1 UpdateReportTemplate`.

            Output:
              - `{ \"report_template\": ReportTemplate, \"next_step\": string }`. The returned template reflects
                the post-update server state.

            Parameters:
              - `report_template_id`: required; the id of the template to update.
              - `name`: optional new name.
              - `description`: optional new description.
              - `tag_names`: optional REPLACEMENT list of tag names. Passing this overwrites the template's full
                tag list; pass `[]` to clear all tags.
              - `metadata`: optional REPLACEMENT metadata list of `{ \"name\": ..., \"value\": ... }` entries.
                Passing this overwrites the template's full metadata; pass `[]` to clear.
              - `is_archived`: optional; set to `true` to archive the template or `false` to unarchive it.

              To replace the template's rule set, provide EXACTLY ONE of:
              - `rule_ids`: ordered list of rule IDs. Position becomes each rule's `display_order`.
              - `rule_client_keys`: list of rule client keys. Server resolves keys; on-template order is
                server-defined.
              Providing both is rejected. Omit both to leave the rule set untouched.

            At least one updatable field (`name`, `description`, `tag_names`, `rule_ids`, `rule_client_keys`,
            `metadata`, or `is_archived`) must be set.

            Errors:
              - `INVALID_PARAMS` if `report_template_id` is empty, no updatable field is provided, or both
                `rule_ids` and `rule_client_keys` are provided.
              - `RESOURCE_NOT_FOUND` if no template matches `report_template_id`.
              - `INTERNAL_ERROR` for upstream gRPC failures.

            Guidance:
              - This is a write with REPLACE semantics on `tags`, `rules`, and `metadata`. CONFIRM the intended
                shape with the user — for appends, read the current template via `list_report_templates` filtered
                by `report_template_id == \"<id>\"` and send the union.
              - Reports already created from this template are NOT retroactively updated; only future reports
                created from the template pick up the change.
        ",
        annotations(
            title = "report_templates/update_report_template",
            read_only_hint = false,
            destructive_hint = true,
            idempotent_hint = true,
        )
    )]
    pub async fn update_report_template(
        &self,
        params: Parameters<UpdateReportTemplateParams>,
    ) -> error::McpResult {
        self.require_destructive()?;

        let Parameters(UpdateReportTemplateParams {
            report_template_id,
            name,
            description,
            tag_names,
            rule_ids,
            rule_client_keys,
            metadata,
            is_archived,
        }) = params;

        if report_template_id.is_empty() {
            return Err(ErrorData::invalid_params(
                "`report_template_id` must not be empty",
                None,
            ));
        }

        let rules = match (rule_ids, rule_client_keys) {
            (None, None) => None,
            (Some(ids), None) => Some(TemplateRuleIdentifier::RuleIds(ids)),
            (None, Some(keys)) => Some(TemplateRuleIdentifier::RuleClientKeys(keys)),
            (Some(_), Some(_)) => {
                return Err(ErrorData::invalid_params(
                    "provide only one of `rule_ids` or `rule_client_keys`, not both",
                    None,
                ));
            }
        };

        let metadata = metadata.map(|m| m.into_iter().map(MetadataValue::from).collect::<Vec<_>>());

        let changes = ReportTemplateUpdate {
            name,
            description,
            tag_names,
            rules,
            metadata,
            is_archived,
        };

        if changes.name.is_none()
            && changes.description.is_none()
            && changes.tag_names.is_none()
            && changes.rules.is_none()
            && changes.metadata.is_none()
            && changes.is_archived.is_none()
        {
            return Err(ErrorData::invalid_params(
                "provide at least one field to update: `name`, `description`, `tag_names`, `rule_ids`, \
                 `rule_client_keys`, `metadata`, or `is_archived`",
                None,
            ));
        }

        let template = self
            .report_template_service
            .update_report_template(report_template_id, changes)
            .await
            .map_err(from_anyhow)?;

        let archive_clause = if template.is_archived {
            " Template is now archived."
        } else {
            ""
        };
        let next_step = format!(
            "Updated report template `{}` ({}).{} Surface the new state to the user and confirm nothing was \
             unintentionally dropped — `tags`, `rules`, and `metadata` use REPLACE semantics.",
            template.name, template.report_template_id, archive_clause,
        );

        let mut result = CallToolResult::structured(serde_json::json!({
            "report_template": template,
            "next_step": next_step,
        }));
        result.content = vec![ContentBlock::text(next_step)];
        Ok(result)
    }
}
