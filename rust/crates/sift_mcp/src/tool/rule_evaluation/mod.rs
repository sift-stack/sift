use rmcp::{
    ErrorData,
    handler::server::wrapper::Parameters,
    model::{CallToolResult, ContentBlock},
    schemars::{self, JsonSchema},
    tool, tool_router,
};
use serde::Deserialize;
use sift_rs::rules::v1::UpdateRuleRequest;

use crate::{
    error::{self, from_anyhow},
    server::SiftMcpServer,
    service::{common::cel_escape, rule_evaluation::PreviewRuleSource},
};

#[cfg(test)]
mod test;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct PreviewRuleParams {
    pub(crate) run_id: String,
    pub(crate) rule_id: Option<String>,
    pub(crate) rule_name: Option<String>,
    pub(crate) draft_rule_config: Option<String>,
    pub(crate) organization_id: Option<String>,
}

/// Which saved-rule selector the caller gave, prior to resolving a name to an id.
enum SavedRuleSelector {
    Id(String),
    Name(String),
}

#[tool_router(router = rule_evaluation_router, vis = "pub(crate)")]
impl SiftMcpServer {
    #[tool(
        name = "preview_rule",
        description = "
            Dry-run a rule against a run and return the annotations that would be generated. This is a READ-ONLY
            action: nothing is persisted (no report, annotation, or rule version is created). To persist an
            evaluation, use `create_report` instead once the preview looks right.

            Output:
              - `{ \"created_annotation_count\": number, \"dry_run_annotations\": [DryRunAnnotation, ...],
                \"next_step\": string }`. Each `DryRunAnnotation` has `condition_id`, `name`, `start_time`,
                `end_time`, and `condition_version_id` for one annotation the rule would create.

            Parameters:
              - `run_id`: required; the run to evaluate the rule against.
              - The rule under test is one of two mutually exclusive shapes — provide exactly one:
                - SAVED rule: set `rule_id` or `rule_name` (not both). `rule_name` is resolved to a rule id via an
                  exact `name ==` lookup; if more than one rule shares the name, an arbitrary match is used, so
                  prefer `rule_id` when it's known.
                - DRAFT rule: set `draft_rule_config`, a JSON string matching
                  `protos/sift/rules/v1/rules.proto::UpdateRuleRequest` — the same shape `create_rule` takes as
                  `rule_json`. Nothing is looked up or saved for this shape; the whole definition travels in the
                  request. Mirror an existing rule retrieved via `list_rules` rather than authoring blind.
              - `organization_id`: optional. Required only when the caller belongs to multiple organizations.

            Errors:
              - `INVALID_PARAMS` if `run_id` is empty; if neither a saved-rule selector nor `draft_rule_config` is
                given, or both are; if both `rule_id` and `rule_name` are given; or if `draft_rule_config` is not
                valid JSON matching the rule schema.
              - `RESOURCE_NOT_FOUND` if `rule_name` matches no rule, or `run_id` does not exist.
              - `INTERNAL_ERROR` for upstream gRPC failures.

            Guidance:
              - This is read-only — no user confirmation is needed before calling it, unlike `create_report`.
              - After a good preview, offer the user the next step: `create_report` to persist the evaluation as a
                report, or `create_rule`/`update_rule` to save a draft as a real rule.
        ",
        annotations(title = "rule_evaluation/preview_rule", read_only_hint = true)
    )]
    pub async fn preview_rule(&self, params: Parameters<PreviewRuleParams>) -> error::McpResult {
        let Parameters(PreviewRuleParams {
            run_id,
            rule_id,
            rule_name,
            draft_rule_config,
            organization_id,
        }) = params;

        if run_id.is_empty() {
            return Err(ErrorData::invalid_params(
                "`run_id` must not be empty",
                None,
            ));
        }

        let selector = saved_rule_selector(rule_id, rule_name)?;

        let source = match (selector, draft_rule_config) {
            (Some(_), Some(_)) => {
                return Err(ErrorData::invalid_params(
                    "provide either a saved rule (`rule_id`/`rule_name`) or `draft_rule_config`, not both",
                    None,
                ));
            }
            (None, None) => {
                return Err(ErrorData::invalid_params(
                    "provide either a saved rule (`rule_id`/`rule_name`) or `draft_rule_config`",
                    None,
                ));
            }
            (Some(selector), None) => {
                let rule_id = self.resolve_saved_rule_id(selector).await?;
                PreviewRuleSource::SavedRuleId(rule_id)
            }
            (None, Some(draft_rule_config)) => {
                let config = parse_draft_rule_config(&draft_rule_config)?;
                PreviewRuleSource::DraftRuleConfig(Box::new(config))
            }
        };

        let result = self
            .rule_evaluation_service
            .preview_rule(run_id, source, organization_id)
            .await
            .map_err(from_anyhow)?;

        let next_step = format!(
            "Preview evaluated {} annotation(s) that would be created. Nothing was persisted (no report, \
             annotation, or rule was saved). Review the dry-run results with the user; if they want to keep \
             them, use `create_report` to persist an evaluation over this run.",
            result.created_annotation_count,
        );

        let mut result = CallToolResult::structured(serde_json::json!({
            "created_annotation_count": result.created_annotation_count,
            "dry_run_annotations": result.dry_run_annotations,
            "next_step": next_step,
        }));
        result.content = vec![ContentBlock::text(next_step)];
        Ok(result)
    }
}

/// Resolve the mutually exclusive `(rule_id, rule_name)` params into an
/// optional selector. `None` means the caller gave neither (the "draft rule"
/// case is expected instead); both set is an error.
fn saved_rule_selector(
    rule_id: Option<String>,
    rule_name: Option<String>,
) -> Result<Option<SavedRuleSelector>, ErrorData> {
    match (rule_id, rule_name) {
        (Some(id), None) => Ok(Some(SavedRuleSelector::Id(id))),
        (None, Some(name)) => Ok(Some(SavedRuleSelector::Name(name))),
        (None, None) => Ok(None),
        (Some(_), Some(_)) => Err(ErrorData::invalid_params(
            "provide at most one of `rule_id` or `rule_name`, not both",
            None,
        )),
    }
}

/// Deserialize a draft rule config JSON string into an `UpdateRuleRequest`,
/// mapping any parse error to `INVALID_PARAMS` so the agent can correct it.
fn parse_draft_rule_config(draft_rule_config: &str) -> Result<UpdateRuleRequest, ErrorData> {
    serde_json::from_str::<UpdateRuleRequest>(draft_rule_config).map_err(|e| {
        ErrorData::invalid_params(
            format!("`draft_rule_config` is not a valid rule definition: {e}"),
            None,
        )
    })
}

impl SiftMcpServer {
    /// Resolve a saved-rule selector to a `rule_id`, going through the existing
    /// rules service for name lookups so this doesn't duplicate rule lookup logic.
    async fn resolve_saved_rule_id(
        &self,
        selector: SavedRuleSelector,
    ) -> Result<String, ErrorData> {
        match selector {
            SavedRuleSelector::Id(id) => Ok(id),
            SavedRuleSelector::Name(name) => {
                let filter = format!("is_archived == false && name == \"{}\"", cel_escape(&name));
                let rule = self
                    .rule_service
                    .list_rules(filter, None, Some(1))
                    .await
                    .map_err(from_anyhow)?
                    .items
                    .into_iter()
                    .next()
                    .ok_or_else(|| {
                        ErrorData::resource_not_found(format!("rule '{name}' not found"), None)
                    })?;
                Ok(rule.rule_id)
            }
        }
    }
}
