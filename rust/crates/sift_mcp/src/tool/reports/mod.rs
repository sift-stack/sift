use rmcp::{
    ErrorData,
    handler::server::wrapper::Parameters,
    model::{CallToolResult, Content},
    schemars::{self, JsonSchema},
    tool, tool_router,
};
use serde::Deserialize;
use sift_rs::metadata::v1::MetadataValue;

use crate::{
    error::{self, from_anyhow},
    server::SiftMcpServer,
    service::reports::{ReportSource, RuleIdentifier},
    tool::common::{MetadataEntry, url_clause, with_urls},
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

#[derive(Debug, Deserialize, JsonSchema)]
pub struct ReportRuleSummaryListParams {
    report_id: String,
    filter: String,
    order_by: Option<String>,
    limit: Option<u32>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct CreateReportParams {
    run_id: String,
    name: String,
    organization_id: Option<String>,
    metadata: Option<Vec<MetadataEntry>>,
    report_template_id: Option<String>,
    description: Option<String>,
    tag_names: Option<Vec<String>>,
    rule_ids: Option<Vec<String>>,
    rule_client_keys: Option<Vec<String>>,
    rule_version_ids: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct UpdateReportParams {
    report_id: String,
    metadata: Vec<MetadataEntry>,
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
                tags, metadata, timestamps, and archive state, plus an added `url` field with the report's Sift
                web link (`<host>/reports/<report_id>`). `url` is omitted when the host can't be derived.
                Surface these links to the user when presenting reports.

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

        let reports = self
            .report_service
            .list_reports(filter, order_by, limit, organization_id)
            .await
            .map_err(from_anyhow)?;

        let reports = with_urls(&reports, |r| {
            self.url_service.build_report_url(&r.report_id).ok()
        })?;

        Ok(CallToolResult::structured(
            serde_json::json!({ "reports": reports }),
        ))
    }

    #[tool(
        name = "list_report_rule_summaries",
        description = "
            List the per-rule summaries for a single report. Wraps `reports/v1 ListReportRuleSummaries`.

            Output:
              - `{ \"report_rule_summaries\": [ReportRuleSummary, ...], \"report_url\": string|null,
                \"next_step\": string }`. Each summary carries `rule_id`, `rule_client_key`, `rule_version_id`,
                `asset_id`, pass/fail/open counts (`num_passed`, `num_failed`, `num_open`), `status`,
                `status_details`, and `display_order`. `report_url` is the report's Sift web link, or null when
                the host can't be derived.

            Parameters:
              - `report_id`: required; the report whose rule summaries to list.
              - `filter`: CEL expression. Pass an empty string to list everything. Filterable fields include
                `rule_id`, `rule_version_id`, `asset_id`, and `status`.
              - `order_by`: optional comma-separated `FIELD_NAME[ desc]` list. Orderable fields: `display_order`,
                `created_date`, `modified_date`. Default sort is `display_order` ascending.
              - `limit`: optional cap on returned items. Values in `1..=1000` cap the result set; omitting it OR
                passing a value above 1000 returns ALL summaries (paginated server-side).

            Errors:
              - `INVALID_PARAMS` if `report_id` is empty or `filter`/`order_by` is invalid.
              - `RESOURCE_NOT_FOUND` if no report matches `report_id`.
              - `INTERNAL_ERROR` for upstream gRPC failures.

            Guidance:
              - Use this to drill into why a report passed or failed after locating it with `list_reports`.
              - Filter by `status` (e.g. `status == \"REPORT_RULE_STATUS_FAILED\"`) to surface only failing rules.
        ",
        annotations(
            title = "reports_router/list_report_rule_summaries",
            read_only_hint = true
        )
    )]
    pub async fn list_report_rule_summaries(
        &self,
        params: Parameters<ReportRuleSummaryListParams>,
    ) -> error::McpResult {
        let Parameters(ReportRuleSummaryListParams {
            report_id,
            filter,
            order_by,
            limit,
        }) = params;

        if report_id.is_empty() {
            return Err(ErrorData::invalid_params(
                "`report_id` must not be empty",
                None,
            ));
        }

        let report_url = self.url_service.build_report_url(&report_id).ok();

        let summaries = self
            .report_service
            .list_report_rule_summaries(report_id, filter, order_by, limit)
            .await
            .map_err(from_anyhow)?;

        let next_step = format!(
            "Listed {} rule summaries.{} Surface the per-rule pass/fail/open breakdown to the user.",
            summaries.len(),
            url_clause(report_url.as_deref()),
        );

        let mut result = CallToolResult::structured(serde_json::json!({
            "report_rule_summaries": summaries,
            "report_url": report_url,
            "next_step": next_step,
        }));
        result.content = vec![Content::text(next_step)];
        Ok(result)
    }

    #[tool(
        name = "create_report",
        description = "
            Create a report over a run, either from a report template or from an explicit set of rules. Wraps
            `reports/v1 CreateReport`.

            Output:
              - `{ \"report\": Report, \"report_url\": string|null, \"next_step\": string }`. The returned `Report`
                is the server-assigned state including its new `report_id` and `job_id`. `report_url` is the report's
                Sift web link (`<host>/reports/<report_id>`), or null on self-hosted deployments where the host
                can't be derived.

            Parameters:
              - `run_id`: required; the run the report is generated over.
              - `name`: required; the report name.
              - `organization_id`: optional. Required only when the caller belongs to multiple organizations.
              - `metadata`: optional list of `{ \"name\": \"<key>\", \"value\": <scalar> }` entries.

              The report SOURCE is one of two mutually exclusive shapes — provide exactly one:
              - Template: set `report_template_id`. The template defines which rules run.
              - Rules: leave `report_template_id` unset and provide EXACTLY ONE of `rule_ids`, `rule_client_keys`,
                or `rule_version_ids`. `description` and `tag_names` are optional and apply only to this shape.

            Errors:
              - `INVALID_PARAMS` if `run_id` or `name` is empty, if both a template and rule identifiers are given,
                if neither is given, or if more than one rule-identifier list is given.
              - `INTERNAL_ERROR` for upstream gRPC failures (e.g. unknown run, template, or rule).

            Guidance:
              - This is a write that kicks off report execution. CONFIRM the run, source, and name with the user
                before invoking.
              - Use `list_report_rule_summaries` on the returned `report_id` to track per-rule progress.
        ",
        annotations(title = "reports_router/create_report", read_only_hint = false)
    )]
    pub async fn create_report(&self, params: Parameters<CreateReportParams>) -> error::McpResult {
        let Parameters(CreateReportParams {
            run_id,
            name,
            organization_id,
            metadata,
            report_template_id,
            description,
            tag_names,
            rule_ids,
            rule_client_keys,
            rule_version_ids,
        }) = params;

        if run_id.is_empty() {
            return Err(ErrorData::invalid_params(
                "`run_id` must not be empty",
                None,
            ));
        }
        if name.is_empty() {
            return Err(ErrorData::invalid_params("`name` must not be empty", None));
        }

        let rule_sources = [
            rule_ids.map(RuleIdentifier::RuleIds),
            rule_client_keys.map(RuleIdentifier::RuleClientKeys),
            rule_version_ids.map(RuleIdentifier::RuleVersionIds),
        ];
        let rule_count = rule_sources.iter().filter(|r| r.is_some()).count();

        let source = match (report_template_id, rule_count) {
            (Some(report_template_id), 0) => ReportSource::Template { report_template_id },
            (Some(_), _) => {
                return Err(ErrorData::invalid_params(
                    "provide either `report_template_id` or rule identifiers, not both",
                    None,
                ));
            }
            (None, 1) => {
                let rules = rule_sources
                    .into_iter()
                    .flatten()
                    .next()
                    .expect("one source");
                ReportSource::Rules {
                    description,
                    tag_names: tag_names.unwrap_or_default(),
                    rules,
                }
            }
            (None, 0) => {
                return Err(ErrorData::invalid_params(
                    "provide a report source: `report_template_id`, or exactly one of `rule_ids`, \
                     `rule_client_keys`, `rule_version_ids`",
                    None,
                ));
            }
            (None, _) => {
                return Err(ErrorData::invalid_params(
                    "provide exactly one of `rule_ids`, `rule_client_keys`, `rule_version_ids`",
                    None,
                ));
            }
        };

        let metadata = metadata.map(|m| m.into_iter().map(MetadataValue::from).collect::<Vec<_>>());

        let report = self
            .report_service
            .create_report(organization_id, run_id, name, metadata, source)
            .await
            .map_err(from_anyhow)?;

        let report_url = self.url_service.build_report_url(&report.report_id).ok();
        let next_step = format!(
            "Created report `{}` ({}).{} Surface it to the user. Use `list_report_rule_summaries` \
             with this `report_id` to track per-rule progress.",
            report.name,
            report.report_id,
            url_clause(report_url.as_deref()),
        );

        let mut result = CallToolResult::structured(serde_json::json!({
            "report": report,
            "report_url": report_url,
            "next_step": next_step,
        }));
        result.content = vec![Content::text(next_step)];
        Ok(result)
    }

    #[tool(
        name = "update_report",
        description = "
            Update an existing report's metadata. Wraps `reports/v1 UpdateReport`.

            Output:
              - `{ \"report\": Report, \"report_url\": string|null, \"next_step\": string }`. The returned `Report`
                is re-fetched after the update (the update RPC itself returns no body). `report_url` is the report's
                Sift web link, or null when the host can't be derived.

            Parameters:
              - `report_id`: required; the id of the report to update.
              - `metadata`: required; REPLACES the report's full metadata list. Each entry is
                `{ \"name\": \"<key>\", \"value\": <scalar> }` where `value` is a string, number, or boolean.
                Pass `[]` to clear all metadata. The key must already exist in the organization's metadata schema.

            Note: per the API, only `metadata` is updatable through this tool. Report name/description and the
            archive flow are not exposed here.

            Errors:
              - `INVALID_PARAMS` if `report_id` is empty.
              - `RESOURCE_NOT_FOUND` if no report matches `report_id`.
              - `INTERNAL_ERROR` for upstream gRPC failures (e.g. unknown metadata key).

            Guidance:
              - This is a write with REPLACE semantics. CONFIRM the full metadata list with the user — for appends,
                read the current report via `list_reports` filtered by `report_id == \"<id>\"` and send the union.
        ",
        annotations(title = "reports_router/update_report", read_only_hint = false)
    )]
    pub async fn update_report(&self, params: Parameters<UpdateReportParams>) -> error::McpResult {
        let Parameters(UpdateReportParams {
            report_id,
            metadata,
        }) = params;

        if report_id.is_empty() {
            return Err(ErrorData::invalid_params(
                "`report_id` must not be empty",
                None,
            ));
        }

        let metadata = metadata
            .into_iter()
            .map(MetadataValue::from)
            .collect::<Vec<_>>();

        let report = self
            .report_service
            .update_report(report_id, metadata)
            .await
            .map_err(from_anyhow)?;

        let report_url = self.url_service.build_report_url(&report.report_id).ok();
        let next_step = format!(
            "Updated metadata on report `{}` ({}).{} Surface the new state to the user and confirm \
             nothing was unintentionally dropped — metadata is a REPLACE operation.",
            report.name,
            report.report_id,
            url_clause(report_url.as_deref()),
        );

        let mut result = CallToolResult::structured(serde_json::json!({
            "report": report,
            "report_url": report_url,
            "next_step": next_step,
        }));
        result.content = vec![Content::text(next_step)];
        Ok(result)
    }
}
