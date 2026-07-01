use rmcp::{
    ErrorData,
    handler::server::wrapper::Parameters,
    model::{CallToolResult, Content},
    schemars::{self, JsonSchema},
    tool, tool_router,
};
use serde::Deserialize;
use sift_rs::{
    metadata::v1::MetadataValue,
    test_reports::v1::{ErrorInfo, NumericBounds, StringBounds, test_measurement},
};

use crate::{
    error::{self, from_anyhow},
    server::SiftMcpServer,
    service::test_reports::spec::{self, ReportSpec},
    tool::common::{ListParams, MetadataEntry, url_clause},
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

#[derive(Debug, Deserialize, JsonSchema)]
pub struct UpdateTestReportParams {
    pub(crate) test_report_id: String,
    pub(crate) status: Option<String>,
    pub(crate) name: Option<String>,
    pub(crate) test_system_name: Option<String>,
    pub(crate) test_case: Option<String>,
    pub(crate) start_time: Option<String>,
    pub(crate) end_time: Option<String>,
    pub(crate) serial_number: Option<String>,
    pub(crate) part_number: Option<String>,
    pub(crate) system_operator: Option<String>,
    pub(crate) run_id: Option<String>,
    pub(crate) is_archived: Option<bool>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct UpdateTestStepParams {
    pub(crate) test_step_id: String,
    pub(crate) name: Option<String>,
    pub(crate) description: Option<String>,
    pub(crate) step_type: Option<String>,
    pub(crate) step_path: Option<String>,
    pub(crate) status: Option<String>,
    pub(crate) start_time: Option<String>,
    pub(crate) end_time: Option<String>,
    pub(crate) error_code: Option<i32>,
    pub(crate) error_message: Option<String>,
    pub(crate) metadata: Option<Vec<MetadataEntry>>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct UpdateTestMeasurementParams {
    pub(crate) measurement_id: String,
    pub(crate) name: Option<String>,
    pub(crate) measurement_type: Option<String>,
    pub(crate) numeric_value: Option<f64>,
    pub(crate) string_value: Option<String>,
    pub(crate) boolean_value: Option<bool>,
    pub(crate) unit: Option<String>,
    pub(crate) numeric_bounds_min: Option<f64>,
    pub(crate) numeric_bounds_max: Option<f64>,
    pub(crate) string_expected: Option<String>,
    pub(crate) passed: Option<bool>,
    pub(crate) timestamp: Option<String>,
    pub(crate) description: Option<String>,
    pub(crate) channel_names: Option<Vec<String>>,
    pub(crate) metadata: Option<Vec<MetadataEntry>>,
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
        annotations(title = "test_reports_router/list_test_reports", read_only_hint = true)
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
        annotations(title = "test_reports_router/list_test_steps", read_only_hint = true)
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
            title = "test_reports_router/list_test_measurements",
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
        annotations(title = "test_reports_router/count_test_steps", read_only_hint = true)
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
            title = "test_reports_router/count_test_measurements",
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
            title = "test_reports_router/create_test_report",
            read_only_hint = false
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
            title = "test_reports_router/append_test_measurements",
            read_only_hint = false
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

    #[tool(
        name = "update_test_report",
        description = "
            Update specific fields of an existing test report, identified by `test_report_id`. This is a
            WRITE. Only the fields you set are changed; every other field on the report is preserved.
            Wraps `test_reports/v1 UpdateTestReport`.

            Output:
              - `{ \"test_report\": TestReport, \"report_url\": string|null, \"next_step\": string }`. The
                returned `TestReport` is the post-update state from the server; `report_url` is its Sift
                web link (`<host>/test-results/<id>`), or null when the host can't be derived.

            Parameters:
              - `test_report_id`: required; the id of the report to update.
              - `status`: optional; one of `PASSED`, `FAILED`, `ABORTED`, `ERROR`, `IN_PROGRESS`,
                `SKIPPED`, `DRAFT` (the `TEST_STATUS_` prefix is optional).
              - `name`, `test_system_name`, `test_case`: optional new string values.
              - `start_time` / `end_time`: optional RFC3339 timestamps, e.g. \"2026-06-24T10:00:00Z\".
              - `serial_number`, `part_number`, `system_operator`: optional new string values.
              - `run_id`: optional; links the report to a different Sift run (empty string to unlink).
              - `is_archived`: optional; archive (`true`) or unarchive (`false`) the report.

              At least one updatable field besides `test_report_id` must be set; otherwise the tool returns
              `INVALID_PARAMS`.

            Errors:
              - `INVALID_PARAMS` if `test_report_id` is empty, no updatable field is provided, `status` is
                an unknown enum value, or a timestamp is not RFC3339.
              - `RESOURCE_NOT_FOUND` if no report matches `test_report_id`.
              - `INTERNAL_ERROR` for upstream gRPC failures.

            Guidance:
              - CONFIRM the change with the user before invoking: read the current report first (via
                `list_test_reports` filtered by `test_report_id == \"...\"`) and show them the current value
                versus the proposed value for each field you intend to change.
        ",
        annotations(
            title = "test_reports_router/update_test_report",
            read_only_hint = false
        )
    )]
    pub async fn update_test_report(
        &self,
        params: Parameters<UpdateTestReportParams>,
    ) -> error::McpResult {
        let Parameters(UpdateTestReportParams {
            test_report_id,
            status,
            name,
            test_system_name,
            test_case,
            start_time,
            end_time,
            serial_number,
            part_number,
            system_operator,
            run_id,
            is_archived,
        }) = params;

        if test_report_id.is_empty() {
            return Err(ErrorData::invalid_params(
                "`test_report_id` must not be empty",
                None,
            ));
        }

        let has_update = status.is_some()
            || name.is_some()
            || test_system_name.is_some()
            || test_case.is_some()
            || start_time.is_some()
            || end_time.is_some()
            || serial_number.is_some()
            || part_number.is_some()
            || system_operator.is_some()
            || run_id.is_some()
            || is_archived.is_some();
        if !has_update {
            return Err(ErrorData::invalid_params(
                "at least one updatable field besides `test_report_id` must be provided",
                None,
            ));
        }

        let status = spec::parse_status(status.as_deref())
            .map_err(|e| ErrorData::invalid_params(format!("{e}"), None))?;
        let start_time = spec::parse_timestamp(start_time.as_deref(), "start_time")
            .map_err(|e| ErrorData::invalid_params(format!("{e}"), None))?;
        let end_time = spec::parse_timestamp(end_time.as_deref(), "end_time")
            .map_err(|e| ErrorData::invalid_params(format!("{e}"), None))?;

        let report = self
            .test_report_service
            .update_test_report(
                test_report_id,
                status,
                name,
                test_system_name,
                test_case,
                start_time,
                end_time,
                serial_number,
                part_number,
                system_operator,
                run_id,
                is_archived,
            )
            .await
            .map_err(from_anyhow)?;

        let report_url = self
            .url_service
            .build_test_report_url(&report.test_report_id)
            .ok();
        let next_step = format!(
            "Updated test report `{}` ({}).{} Surface the new state to the user and confirm the change \
             matches their intent.",
            report.name,
            report.test_report_id,
            url_clause(report_url.as_deref()),
        );

        let mut result = CallToolResult::structured(serde_json::json!({
            "test_report": report,
            "report_url": report_url,
            "next_step": next_step,
        }));
        result.content = vec![Content::text(next_step)];
        Ok(result)
    }

    #[tool(
        name = "update_test_step",
        description = "
            Update specific fields of an existing test step, identified by `test_step_id`. This is a WRITE.
            Only the fields you set are changed; every other field on the step is preserved. Wraps
            `test_reports/v1 UpdateTestStep`.

            Output:
              - `{ \"test_step\": TestStep, \"report_url\": string|null, \"next_step\": string }`. The
                returned `TestStep` is the post-update state; `report_url` links the step's report in the
                Sift web app, or null when the host can't be derived.

            Parameters:
              - `test_step_id`: required; the id of the step to update.
              - `name`, `description`: optional new string values.
              - `step_type`: optional; one of `SEQUENCE`, `GROUP`, `ACTION`, `FLOW_CONTROL` (the
                `TEST_STEP_TYPE_` prefix is optional).
              - `step_path`: optional new hierarchical path (e.g. \"1.2\"). Editing this can desync the step
                tree — change it only when deliberately restructuring.
              - `status`: optional; see `update_test_report` for the accepted values.
              - `start_time` / `end_time`: optional RFC3339 timestamps.
              - `error_code` / `error_message`: optional; MUST be provided together — they set the step's
                `error_info`. `error_info` is diagnostic only and does not by itself mean the step failed.
              - `metadata`: optional; REPLACES the full metadata list. Each entry is
                `{ \"name\": \"<key>\", \"value\": <scalar> }`. Pass `[]` to clear.

              At least one updatable field besides `test_step_id` must be set; otherwise the tool returns
              `INVALID_PARAMS`.

            Errors:
              - `INVALID_PARAMS` if `test_step_id` is empty, no updatable field is provided, only one of
                `error_code`/`error_message` is set, `step_type`/`status` is unknown, or a timestamp is not
                RFC3339.
              - `RESOURCE_NOT_FOUND` if no step matches `test_step_id`.
              - `INTERNAL_ERROR` for upstream gRPC failures.

            Guidance:
              - CONFIRM before invoking: read the current step (via `list_test_steps` filtered by
                `test_step_id == \"...\"`) and show the user current versus proposed values. `metadata` is a
                REPLACE, so read-modify-write when appending.
        ",
        annotations(title = "test_reports_router/update_test_step", read_only_hint = false)
    )]
    pub async fn update_test_step(
        &self,
        params: Parameters<UpdateTestStepParams>,
    ) -> error::McpResult {
        let Parameters(UpdateTestStepParams {
            test_step_id,
            name,
            description,
            step_type,
            step_path,
            status,
            start_time,
            end_time,
            error_code,
            error_message,
            metadata,
        }) = params;

        if test_step_id.is_empty() {
            return Err(ErrorData::invalid_params(
                "`test_step_id` must not be empty",
                None,
            ));
        }

        let error_info = match (error_code, error_message) {
            (Some(error_code), Some(error_message)) => Some(ErrorInfo {
                error_code,
                error_message,
            }),
            (None, None) => None,
            _ => {
                return Err(ErrorData::invalid_params(
                    "`error_code` and `error_message` must be provided together",
                    None,
                ));
            }
        };

        let has_update = name.is_some()
            || description.is_some()
            || step_type.is_some()
            || step_path.is_some()
            || status.is_some()
            || start_time.is_some()
            || end_time.is_some()
            || error_info.is_some()
            || metadata.is_some();
        if !has_update {
            return Err(ErrorData::invalid_params(
                "at least one updatable field besides `test_step_id` must be provided",
                None,
            ));
        }

        let step_type = spec::parse_step_type(step_type.as_deref())
            .map_err(|e| ErrorData::invalid_params(format!("{e}"), None))?;
        let status = spec::parse_status(status.as_deref())
            .map_err(|e| ErrorData::invalid_params(format!("{e}"), None))?;
        let start_time = spec::parse_timestamp(start_time.as_deref(), "start_time")
            .map_err(|e| ErrorData::invalid_params(format!("{e}"), None))?;
        let end_time = spec::parse_timestamp(end_time.as_deref(), "end_time")
            .map_err(|e| ErrorData::invalid_params(format!("{e}"), None))?;
        let metadata = metadata.map(|m| m.into_iter().map(MetadataValue::from).collect::<Vec<_>>());

        let step = self
            .test_report_service
            .update_test_step(
                test_step_id,
                name,
                description,
                step_type,
                step_path,
                status,
                start_time,
                end_time,
                error_info,
                metadata,
            )
            .await
            .map_err(from_anyhow)?;

        let report_url = self
            .url_service
            .build_test_report_url(&step.test_report_id)
            .ok();
        let next_step = format!(
            "Updated test step `{}` ({}) in report `{}`.{} Surface the new state to the user and confirm \
             it matches their intent. Remember: metadata is a REPLACE.",
            step.name,
            step.test_step_id,
            step.test_report_id,
            url_clause(report_url.as_deref()),
        );

        let mut result = CallToolResult::structured(serde_json::json!({
            "test_step": step,
            "report_url": report_url,
            "next_step": next_step,
        }));
        result.content = vec![Content::text(next_step)];
        Ok(result)
    }

    #[tool(
        name = "update_test_measurement",
        description = "
            Update specific fields of an existing test measurement, identified by `measurement_id`. This is
            a WRITE. Only the fields you set are changed; every other field is preserved. Wraps
            `test_reports/v1 UpdateTestMeasurement`.

            Output:
              - `{ \"test_measurement\": TestMeasurement, \"report_url\": string|null, \"next_step\": string }`.
                The returned `TestMeasurement` is the post-update state; `report_url` links its report in
                the Sift web app, or null when the host can't be derived.

            Parameters:
              - `measurement_id`: required; the id of the measurement to update.
              - `name`, `description`: optional new string values.
              - `measurement_type`: optional; one of `DOUBLE`, `STRING`, `BOOLEAN`, `LIMIT` (the
                `TEST_MEASUREMENT_TYPE_` prefix is optional).
              - `numeric_value` / `string_value` / `boolean_value`: optional new value. Set AT MOST ONE;
                setting more than one returns `INVALID_PARAMS`. This does not auto-recompute `passed`.
              - `unit`: optional unit abbreviation (e.g. \"V\").
              - `numeric_bounds_min` / `numeric_bounds_max`: optional numeric bounds. Setting either sets
                `numeric_bounds` (an unset side is unbounded).
              - `string_expected`: optional expected string; sets `string_bounds`. Mutually exclusive with
                the numeric bounds fields.
              - `passed`: optional pass/fail verdict.
              - `timestamp`: optional RFC3339 timestamp.
              - `channel_names`: optional; REPLACES the full channel-name list. Pass `[]` to clear.
              - `metadata`: optional; REPLACES the full metadata list (`{ \"name\": ..., \"value\": <scalar> }`).

              At least one updatable field besides `measurement_id` must be set; otherwise the tool returns
              `INVALID_PARAMS`.

            Errors:
              - `INVALID_PARAMS` if `measurement_id` is empty, no updatable field is provided, more than one
                value field is set, both numeric and string bounds are set, an enum is unknown, or a
                timestamp is not RFC3339.
              - `RESOURCE_NOT_FOUND` if no measurement matches `measurement_id`.
              - `INTERNAL_ERROR` for upstream gRPC failures.

            Guidance:
              - CONFIRM before invoking: read the current measurement (via `list_test_measurements` filtered
                by `measurement_id == \"...\"`) and show the user current versus proposed values. Changing a
                value does not recompute `passed` — set `passed` explicitly if the verdict should change.
        ",
        annotations(
            title = "test_reports_router/update_test_measurement",
            read_only_hint = false
        )
    )]
    pub async fn update_test_measurement(
        &self,
        params: Parameters<UpdateTestMeasurementParams>,
    ) -> error::McpResult {
        let Parameters(UpdateTestMeasurementParams {
            measurement_id,
            name,
            measurement_type,
            numeric_value,
            string_value,
            boolean_value,
            unit,
            numeric_bounds_min,
            numeric_bounds_max,
            string_expected,
            passed,
            timestamp,
            description,
            channel_names,
            metadata,
        }) = params;

        if measurement_id.is_empty() {
            return Err(ErrorData::invalid_params(
                "`measurement_id` must not be empty",
                None,
            ));
        }

        // At most one value variant.
        let value_count = numeric_value.is_some() as u8
            + string_value.is_some() as u8
            + boolean_value.is_some() as u8;
        if value_count > 1 {
            return Err(ErrorData::invalid_params(
                "set at most one of `numeric_value`, `string_value`, or `boolean_value`",
                None,
            ));
        }
        let value = if let Some(v) = numeric_value {
            Some(test_measurement::Value::NumericValue(v))
        } else if let Some(v) = string_value {
            Some(test_measurement::Value::StringValue(v))
        } else {
            boolean_value.map(test_measurement::Value::BooleanValue)
        };

        // At most one bounds kind.
        let has_numeric_bounds = numeric_bounds_min.is_some() || numeric_bounds_max.is_some();
        if has_numeric_bounds && string_expected.is_some() {
            return Err(ErrorData::invalid_params(
                "set either numeric bounds (`numeric_bounds_min`/`numeric_bounds_max`) or \
                 `string_expected`, not both",
                None,
            ));
        }
        let bounds = if has_numeric_bounds {
            Some(test_measurement::Bounds::NumericBounds(NumericBounds {
                min: numeric_bounds_min,
                max: numeric_bounds_max,
            }))
        } else {
            string_expected.map(|expected_value| {
                test_measurement::Bounds::StringBounds(StringBounds { expected_value })
            })
        };

        let has_update = name.is_some()
            || measurement_type.is_some()
            || value.is_some()
            || unit.is_some()
            || bounds.is_some()
            || passed.is_some()
            || timestamp.is_some()
            || description.is_some()
            || channel_names.is_some()
            || metadata.is_some();
        if !has_update {
            return Err(ErrorData::invalid_params(
                "at least one updatable field besides `measurement_id` must be provided",
                None,
            ));
        }

        let measurement_type = spec::parse_measurement_type(measurement_type.as_deref())
            .map_err(|e| ErrorData::invalid_params(format!("{e}"), None))?;
        let timestamp = spec::parse_timestamp(timestamp.as_deref(), "timestamp")
            .map_err(|e| ErrorData::invalid_params(format!("{e}"), None))?;
        let metadata = metadata.map(|m| m.into_iter().map(MetadataValue::from).collect::<Vec<_>>());

        let measurement = self
            .test_report_service
            .update_test_measurement(
                measurement_id,
                name,
                measurement_type,
                value,
                unit,
                bounds,
                passed,
                timestamp,
                description,
                channel_names,
                metadata,
            )
            .await
            .map_err(from_anyhow)?;

        let report_url = self
            .url_service
            .build_test_report_url(&measurement.test_report_id)
            .ok();
        let next_step = format!(
            "Updated test measurement `{}` ({}) in report `{}`.{} Surface the new state to the user and \
             confirm it matches their intent. Remember: channel_names and metadata are REPLACE \
             operations, and changing a value does not recompute `passed`.",
            measurement.name,
            measurement.measurement_id,
            measurement.test_report_id,
            url_clause(report_url.as_deref()),
        );

        let mut result = CallToolResult::structured(serde_json::json!({
            "test_measurement": measurement,
            "report_url": report_url,
            "next_step": next_step,
        }));
        result.content = vec![Content::text(next_step)];
        Ok(result)
    }
}
