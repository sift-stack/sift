use rmcp::{
    ErrorData,
    handler::server::wrapper::Parameters,
    model::{CallToolResult, Content},
    schemars::{self, JsonSchema},
    tool, tool_router,
};
use serde::Deserialize;

use crate::{
    error::{self, from_anyhow},
    server::SiftMcpServer,
    service::test_reports::spec::{self, ReportSpec},
    tool::common::{ListParams, url_clause},
};

#[cfg(test)]
mod test;

/// Parse the `report_json` author spec, mapping any deserialization error to `INVALID_PARAMS`.
fn parse_report_spec(report_json: &str) -> Result<ReportSpec, ErrorData> {
    serde_json::from_str::<ReportSpec>(report_json)
        .map_err(|e| ErrorData::invalid_params(format!("`report_json` is not valid: {e}"), None))
}

/// Filter-only parameters for the `count_*` test-results tools. Counting takes no
/// ordering or page size, so it does not reuse `ListParams`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct CountParams {
    pub(crate) filter: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct CreateTestReportParams {
    pub(crate) report_json: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct AppendMeasurementsParams {
    pub(crate) test_report_id: String,
    pub(crate) test_step_id: String,
    pub(crate) measurements_json: String,
}

#[tool_router(router = test_reports_router, vis = "pub(crate)")]
impl SiftMcpServer {
    #[tool(
        name = "list_test_reports",
        description = "
            List test reports (test-results runs) in Sift, optionally filtered by a CEL expression and ordered
            by one or more fields. A test report is the top of the results tree: it owns test steps, which own
            test measurements. Start an audit here, then drill into `list_test_steps` and `list_test_measurements`.

            Output:
              - `{ \"test_reports\": [TestReport, ...] }`. Each item includes `test_report_id`, `status`, `name`,
                `test_system_name`, `test_case`, `start_time`, `end_time`, `serial_number`, `part_number`,
                `system_operator`, `run_id`, `metadata`, `is_archived`, and `archived_date`. `run_id` links the
                report to the Sift run that holds the ingested channel data; empty when none is associated.

            Parameters:
              - `filter`: CEL expression. Pass an empty string to list everything. Filterable fields:
                `test_report_id`, `status`, `name`, `test_system_name`, `test_case`, `start_time`, `end_time`,
                `serial_number`, `part_number`, `system_operator`, `run_id`, `archived_date`,
                `created_by_user_id`, `modified_by_user_id`, `metadata`. Reference metadata entries as
                `metadata.{key}`. `status` matches the `TestStatus` enum: `TEST_STATUS_DRAFT`,
                `TEST_STATUS_PASSED`, `TEST_STATUS_FAILED`, `TEST_STATUS_ABORTED`, `TEST_STATUS_ERROR`,
                `TEST_STATUS_IN_PROGRESS`, `TEST_STATUS_SKIPPED`. Filtering `test_report_id == \"...\"` fetches one
                report (there is no separate get tool).
              - `order_by`: optional comma-separated `FIELD_NAME[ desc]` list. Orderable fields: `test_report_id`,
                `name`, `test_system_name`, `test_case`, `start_time`, `end_time`, `created_date`, `modified_date`.
                Default sort is `start_time desc` (newest first). Example: `\"start_time desc,name\"`.
              - `limit`: optional cap on returned items. Values in `1..=1000` cap the result set. Omitting it OR
                passing a value above 1000 returns ALL matching reports (paginated server-side).

            Errors:
              - `INVALID_PARAMS` if `filter` is not a valid CEL expression or `order_by` references an unknown field.
              - `INTERNAL_ERROR` for upstream gRPC failures.

            Guidance:
              - To audit a single run, resolve the report first (by `name`, `run_id`, or `test_case`), then pass
                its `test_report_id` to the step and measurement tools.
        ",
        annotations(title = "test_reports/list_test_reports", read_only_hint = true)
    )]
    pub async fn list_test_reports(&self, params: Parameters<ListParams>) -> error::McpResult {
        let Parameters(ListParams {
            filter,
            order_by,
            limit,
        }) = params;

        let out = self
            .test_report_service
            .list_test_reports(filter, order_by, limit)
            .await
            .map(|test_reports| serde_json::json!({ "test_reports": test_reports }))
            .map_err(from_anyhow)?;

        Ok(CallToolResult::structured(out))
    }

    #[tool(
        name = "list_test_steps",
        description = "
            List test steps, optionally filtered by a CEL expression and ordered by one or more fields. Steps form
            a tree within a report via `parent_step_id` and the hierarchical `step_path` (e.g. \"1\", \"1.1\",
            \"1.2.3\"). Scope to one report with `test_report_id == \"...\"`.

            Output:
              - `{ \"test_steps\": [TestStep, ...] }`. Each item includes `test_step_id`, `test_report_id`,
                `parent_step_id`, `name`, `description`, `step_type`, `step_path`, `status`, `start_time`,
                `end_time`, `error_info` (`{ error_code, error_message }`), and `metadata`. `error_info` is
                diagnostic only: a populated `error_info` does not by itself mean the step failed — derive
                pass/fail from `status`.

            Parameters:
              - `filter`: CEL expression. Pass an empty string to list everything (rarely useful; almost always
                scope by `test_report_id`). Filterable fields: `test_step_id`, `test_report_id`, `parent_step_id`,
                `name`, `description`, `step_type`, `step_path`, `status`, `start_time`, `end_time`, `error_code`,
                `error_message`, `created_date`, `modified_date`, `metadata`. `step_type` matches the
                `TestStepType` enum: `TEST_STEP_TYPE_SEQUENCE`, `TEST_STEP_TYPE_GROUP`, `TEST_STEP_TYPE_ACTION`,
                `TEST_STEP_TYPE_FLOW_CONTROL`. `status` matches `TestStatus` (see `list_test_reports`).
              - `order_by`: optional comma-separated `FIELD_NAME[ desc]` list. Orderable fields: `test_step_id`,
                `name`, `step_type`, `step_path`, `status`, `start_time`, `end_time`, `created_date`,
                `modified_date`. Default sort is `step_path` ascending (tree order). Example:
                `\"step_path asc,start_time desc\"`.
              - `limit`: optional cap on returned items. Values in `1..=1000` cap the result set. Omitting it OR
                passing a value above 1000 returns ALL matching steps (paginated server-side). Use
                `count_test_steps` to learn the true total before relying on a capped page.

            Errors:
              - `INVALID_PARAMS` if `filter` is not a valid CEL expression or `order_by` references an unknown field.
              - `INTERNAL_ERROR` for upstream gRPC failures.

            Guidance:
              - To find failures, filter `test_report_id == \"...\" && status == TEST_STATUS_FAILED`.
              - Order by `step_path` to reconstruct the execution tree; use `parent_step_id` to attach children.
        ",
        annotations(title = "test_reports/list_test_steps", read_only_hint = true)
    )]
    pub async fn list_test_steps(&self, params: Parameters<ListParams>) -> error::McpResult {
        let Parameters(ListParams {
            filter,
            order_by,
            limit,
        }) = params;

        let out = self
            .test_report_service
            .list_test_steps(filter, order_by, limit)
            .await
            .map(|test_steps| serde_json::json!({ "test_steps": test_steps }))
            .map_err(from_anyhow)?;

        Ok(CallToolResult::structured(out))
    }

    #[tool(
        name = "list_test_measurements",
        description = "
            List test measurements, optionally filtered by a CEL expression and ordered by one or more fields. A
            measurement is a single recorded value on a step, with optional bounds and a pass/fail verdict. Scope
            to a report with `test_report_id == \"...\"` or to one step with `test_step_id == \"...\"`.

            Output:
              - `{ \"test_measurements\": [TestMeasurement, ...] }`. Each item includes `measurement_id`,
                `measurement_type`, `name`, `test_step_id`, `test_report_id`, the value (`numeric_value`,
                `string_value`, or `boolean_value`), `unit`, bounds (`numeric_bounds` `{ min, max }` or
                `string_bounds` `{ expected_value }`), `passed`, `timestamp`, `description`, `channel_names`, and
                `metadata`. `channel_names` ties the measurement to Sift channels on the report's run for
                cross-plotting in Explore.

            Parameters:
              - `filter`: CEL expression. Pass an empty string to list everything (almost always scope by
                `test_report_id` or `test_step_id`). Filterable fields: `measurement_id`, `measurement_type`,
                `name`, `test_step_id`, `test_report_id`, `numeric_value`, `string_value`, `boolean_value`,
                `passed`, `timestamp`, `created_date`, `modified_date`, `metadata`. `measurement_type` matches the
                `TestMeasurementType` enum: `TEST_MEASUREMENT_TYPE_DOUBLE`, `TEST_MEASUREMENT_TYPE_STRING`,
                `TEST_MEASUREMENT_TYPE_BOOLEAN`, `TEST_MEASUREMENT_TYPE_LIMIT`.
              - `order_by`: optional comma-separated `FIELD_NAME[ desc]` list. Orderable fields: `measurement_id`,
                `name`, `measurement_type`, `test_step_id`, `test_report_id`, `passed`, `timestamp`,
                `created_date`, `modified_date`. Default sort is `timestamp` ascending. Example:
                `\"timestamp asc,name\"`.
              - `limit`: optional cap on returned items. Values in `1..=1000` cap the result set. Omitting it OR
                passing a value above 1000 returns ALL matching measurements (paginated server-side). Use
                `count_test_measurements` to learn the true total before relying on a capped page.

            Errors:
              - `INVALID_PARAMS` if `filter` is not a valid CEL expression or `order_by` references an unknown field.
              - `INTERNAL_ERROR` for upstream gRPC failures.

            Guidance:
              - To audit limit checks, filter `passed == false` within a report and compare each value against its
                `numeric_bounds`/`string_bounds`.
        ",
        annotations(
            title = "test_reports/list_test_measurements",
            read_only_hint = true
        )
    )]
    pub async fn list_test_measurements(&self, params: Parameters<ListParams>) -> error::McpResult {
        let Parameters(ListParams {
            filter,
            order_by,
            limit,
        }) = params;

        let out = self
            .test_report_service
            .list_test_measurements(filter, order_by, limit)
            .await
            .map(|test_measurements| serde_json::json!({ "test_measurements": test_measurements }))
            .map_err(from_anyhow)?;

        Ok(CallToolResult::structured(out))
    }

    #[tool(
        name = "count_test_steps",
        description = "
            Count test steps matching a CEL filter, without fetching them. Use this to learn the true total when a
            report may hold more steps than the 1000-item list cap, or to reconcile expected vs actual counts in an
            audit (e.g. \"how many steps failed in this report\").

            Output:
              - `{ \"count\": <int64> }`. The total number of steps matching the filter.

            Parameters:
              - `filter`: CEL expression. Pass an empty string to count all steps. Filterable fields are identical
                to `list_test_steps`: `test_step_id`, `test_report_id`, `parent_step_id`, `name`, `description`,
                `step_type`, `step_path`, `status`, `start_time`, `end_time`, `error_code`, `error_message`,
                `created_date`, `modified_date`, `metadata`.

            Errors:
              - `INVALID_PARAMS` if `filter` is not a valid CEL expression.
              - `INTERNAL_ERROR` for upstream gRPC failures.
        ",
        annotations(title = "test_reports/count_test_steps", read_only_hint = true)
    )]
    pub async fn count_test_steps(&self, params: Parameters<CountParams>) -> error::McpResult {
        let Parameters(CountParams { filter }) = params;

        let out = self
            .test_report_service
            .count_test_steps(filter)
            .await
            .map(|count| serde_json::json!({ "count": count }))
            .map_err(from_anyhow)?;

        Ok(CallToolResult::structured(out))
    }

    #[tool(
        name = "count_test_measurements",
        description = "
            Count test measurements matching a CEL filter, without fetching them. Use this to learn the true total
            when a report may hold more measurements than the 1000-item list cap, or to reconcile expected vs actual
            counts in an audit (e.g. \"how many measurements failed their limits\").

            Output:
              - `{ \"count\": <int64> }`. The total number of measurements matching the filter.

            Parameters:
              - `filter`: CEL expression. Pass an empty string to count all measurements. Filterable fields are
                identical to `list_test_measurements`: `measurement_id`, `measurement_type`, `name`,
                `test_step_id`, `test_report_id`, `numeric_value`, `string_value`, `boolean_value`, `passed`,
                `timestamp`, `created_date`, `modified_date`, `metadata`.

            Errors:
              - `INVALID_PARAMS` if `filter` is not a valid CEL expression.
              - `INTERNAL_ERROR` for upstream gRPC failures.
        ",
        annotations(
            title = "test_reports/count_test_measurements",
            read_only_hint = true
        )
    )]
    pub async fn count_test_measurements(
        &self,
        params: Parameters<CountParams>,
    ) -> error::McpResult {
        let Parameters(CountParams { filter }) = params;

        let out = self
            .test_report_service
            .count_test_measurements(filter)
            .await
            .map(|count| serde_json::json!({ "count": count }))
            .map_err(from_anyhow)?;

        Ok(CallToolResult::structured(out))
    }

    #[tool(
        name = "create_test_report",
        description = "
            Create a test report together with its tree of steps and measurements in one call,
            from a single JSON document. WRITE: this creates data in Sift. Confirm the destination
            with the user before calling, and report back what was created.

            Parameters:
              - `report_json`: a JSON object describing the report. Shape:
                - Report fields: `name` (required), `test_system_name` (required), `test_case`
                  (required), `status` (optional, default `PASSED`; accepts `PASSED` or
                  `TEST_STATUS_PASSED`), `start_time`/`end_time` (optional RFC3339, e.g.
                  \"2026-06-24T10:00:00Z\"; default now), `serial_number`, `part_number`,
                  `system_operator`, `run_id` (optional; links the report to an existing Sift run),
                  `metadata` (optional flat object of string/number/bool values).
                - `steps`: optional array. Each step has `name` (required), `status` (optional,
                  default `PASSED`), `step_type` (optional, default `ACTION`; one of `SEQUENCE`,
                  `GROUP`, `ACTION`, `FLOW_CONTROL`), `description`, `start_time`/`end_time`
                  (optional, default the report window), `error_info` (`{ error_code, error_message }`),
                  `metadata`, `measurements`, and a nested `steps` array for children. `step_path`
                  and `parent_step_id` are computed from nesting (roots `1`, `2`; child of `1` is
                  `1.1`) — do not supply them.
                - Each measurement has `name` (required), EXACTLY ONE of `numeric_value`,
                  `string_value`, or `boolean_value`, optional `numeric_bounds` (`{ min, max }`, with
                  `numeric_value`) or `string_expected` (with `string_value`), `unit`, `passed`
                  (optional; computed from bounds when omitted — numeric bounds inclusive, string
                  equals `string_expected`, else true), `measurement_type` (optional; derived from
                  the value kind), `timestamp` (optional RFC3339, default the step start), `description`,
                  `channel_names` (array), and `metadata`.

            Output:
              - `{ \"test_report_id\": \"...\", \"steps_created\": N, \"measurements_created\": M, \"report_url\": string|null, \"next_step\": \"...\" }`.
                `report_url` is the report's Sift web link (`<host>/test-results/<id>`), or null when
                the host cannot be derived (self-hosted deployments). Surface it to the user as a
                clickable link.

            Errors:
              - `INVALID_PARAMS` if `report_json` is not valid JSON, omits a required field, names an
                unknown enum value, has a non-RFC3339 timestamp, or a measurement does not set exactly
                one value (or pairs bounds with the wrong value kind).
              - `INTERNAL_ERROR` for upstream gRPC failures.

            Guidance:
              - The create calls are not transactional. If a step or measurement fails after the report
                is created, the report and earlier steps remain; the error names the created
                `test_report_id`. Verify with `list_test_steps` (filter `test_report_id == \"...\"`).
        ",
        annotations(
            title = "test_reports/create_test_report",
            read_only_hint = false,
            destructive_hint = false,
            idempotent_hint = false,
        )
    )]
    pub async fn create_test_report(
        &self,
        params: Parameters<CreateTestReportParams>,
    ) -> error::McpResult {
        let Parameters(CreateTestReportParams { report_json }) = params;

        let report_spec = parse_report_spec(&report_json)?;
        let built = spec::build(report_spec)
            .map_err(|e| ErrorData::invalid_params(format!("{e}"), None))?;

        let created = self
            .test_report_service
            .create_test_report(built)
            .await
            .map_err(from_anyhow)?;

        let report_url = self
            .url_service
            .build_test_report_url(&created.test_report_id)
            .ok();

        let next_step = format!(
            "Created test report `{}` with {} step(s) and {} measurement(s) in Sift.{} Tell the \
             user the new `test_report_id` and surface the link. If they haven't indicated a next \
             step, offer to verify with `list_test_steps` (filter `test_report_id == \"{}\"`).",
            created.test_report_id,
            created.steps_created,
            created.measurements_created,
            url_clause(report_url.as_deref()),
            created.test_report_id,
        );

        let mut result = CallToolResult::structured(serde_json::json!({
            "test_report_id": created.test_report_id,
            "steps_created": created.steps_created,
            "measurements_created": created.measurements_created,
            "report_url": report_url,
            "next_step": next_step,
        }));
        result.content = vec![Content::text(next_step)];
        Ok(result)
    }

    #[tool(
        name = "append_test_measurements",
        description = "
            Append measurements to an existing test step. WRITE: this creates data in Sift.
            Confirm the destination with the user before calling.

            Parameters:
              - `test_report_id`: the report the step belongs to (required).
              - `test_step_id`: the step to attach the measurements to (required).
              - `measurements_json`: a JSON array of measurement objects, each with the same shape
                as a measurement in `create_test_report`: `name` (required), EXACTLY ONE of
                `numeric_value`/`string_value`/`boolean_value`, optional `numeric_bounds`
                (`{ min, max }`) or `string_expected`, `unit`, `passed` (computed from bounds when
                omitted), `measurement_type` (derived from the value kind), `timestamp` (RFC3339,
                default now), `description`, `channel_names`, and `metadata`. Must be non-empty.

            Output:
              - `{ \"test_report_id\": \"...\", \"test_step_id\": \"...\", \"measurements_created\": N, \"report_url\": string|null, \"next_step\": \"...\" }`.
                `report_url` is the report's Sift web link, or null when the host cannot be derived.

            Errors:
              - `INVALID_PARAMS` if `measurements_json` is not a valid array, is empty, or a
                measurement does not set exactly one value (or pairs bounds with the wrong value kind).
              - `RESOURCE_NOT_FOUND` / `INVALID_PARAMS` if `test_report_id` or `test_step_id` does not
                exist (the server rejects unknown ids).
              - `INTERNAL_ERROR` for upstream gRPC failures.
        ",
        annotations(
            title = "test_reports/append_test_measurements",
            read_only_hint = false,
            destructive_hint = false,
            idempotent_hint = false,
        )
    )]
    pub async fn append_test_measurements(
        &self,
        params: Parameters<AppendMeasurementsParams>,
    ) -> error::McpResult {
        let Parameters(AppendMeasurementsParams {
            test_report_id,
            test_step_id,
            measurements_json,
        }) = params;

        let specs: Vec<spec::MeasurementSpec> =
            serde_json::from_str(&measurements_json).map_err(|e| {
                ErrorData::invalid_params(format!("`measurements_json` is not valid: {e}"), None)
            })?;
        if specs.is_empty() {
            return Err(ErrorData::invalid_params(
                "`measurements_json` must contain at least one measurement",
                None,
            ));
        }

        let measurements = spec::build_measurements(specs)
            .map_err(|e| ErrorData::invalid_params(format!("{e}"), None))?;

        let created = self
            .test_report_service
            .append_test_measurements(test_report_id.clone(), test_step_id.clone(), measurements)
            .await
            .map_err(from_anyhow)?;

        let report_url = self.url_service.build_test_report_url(&test_report_id).ok();

        let next_step = format!(
            "Appended {created} measurement(s) to step `{test_step_id}` in report \
             `{test_report_id}`.{} Tell the user. If they haven't indicated a next step, offer to \
             verify with `list_test_measurements` (filter `test_step_id == \"{test_step_id}\"`).",
            url_clause(report_url.as_deref()),
        );

        let mut result = CallToolResult::structured(serde_json::json!({
            "test_report_id": test_report_id,
            "test_step_id": test_step_id,
            "measurements_created": created,
            "report_url": report_url,
            "next_step": next_step,
        }));
        result.content = vec![Content::text(next_step)];
        Ok(result)
    }
}
