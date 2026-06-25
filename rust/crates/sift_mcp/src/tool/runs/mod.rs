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
    tool::common::{ListParams, MetadataEntry, url_clause, with_urls},
};

#[cfg(test)]
mod test;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct UpdateRunParams {
    run_id: String,
    name: Option<String>,
    description: Option<String>,
    start_time_unix_nanos: Option<i64>,
    stop_time_unix_nanos: Option<i64>,
    is_pinned: Option<bool>,
    client_key: Option<String>,
    tags: Option<Vec<String>>,
    metadata: Option<Vec<MetadataEntry>>,
}

#[tool_router(router = runs_router, vis = "pub(crate)")]
impl SiftMcpServer {
    #[tool(
        name = "list_runs",
        description = "
            List runs in Sift, optionally filtered by a CEL expression and ordered by one or more fields.

            Output:
              - `{ \"runs\": [Run, ...] }`. Each item is the full Sift `Run` shape including `run_id`, `name`,
                `asset_id`/`asset_name`, `client_key`, `start_time`, `stop_time`, duration, annotation state, tags,
                and metadata, plus an added `url` field with the run's Sift web link (`<host>/run/<run_id>`). `url`
                is omitted when the host can't be derived. Surface these links to the user when presenting runs.

            Parameters:
              - `filter`: CEL expression. Pass an empty string to list everything. Filterable fields:
                `run_id`, `organization_id`, `asset_id`, `asset_name`, `client_key`, `name`, `description`,
                `start_time`, `stop_time`, `duration`, `duration_string`, `tag_id`, `asset_tag_id`,
                `annotation_comments_count`, `annotation_state`, `created_date`, `modified_date`, `archived_date`,
                `is_archived`, `created_by_user_id`, `modified_by_user_id`, `metadata`.
                Reference metadata entries as `metadata.{key}`.
                `duration` is elapsed seconds (numeric). `duration_string` accepts `h`/`m`/`s`/`ms` suffixes via the
                `duration(...)` helper, e.g. `duration_string > duration('10h')`.
              - `order_by`: optional comma-separated `FIELD_NAME[ desc]` list. Orderable fields: `name`,
                `description`, `created_date`, `modified_date`, `start_time`, `stop_time`. Default sort is
                `created_date desc` (newest first). Example: `\"created_date desc,modified_date\"`.
              - `limit`: optional cap on returned items. Values in `1..=1000` cap the result set. Omitting it OR
                passing a value above 1000 returns ALL matching runs (paginated server-side).

            Errors:
              - `INVALID_PARAMS` if `filter` is not a valid CEL expression or `order_by` references an unknown field.
              - `INTERNAL_ERROR` for upstream gRPC failures.

            Guidance:
              - When the caller already knows the asset, narrow with `asset_id == \"...\"` first — it's the most
                selective field.
              - To find runs covering a specific moment, filter on both `start_time` and `stop_time` rather than
                pulling everything and filtering client-side.
              - Order by `start_time desc` when surfacing the most recent runs to a user.
        ",
        annotations(title = "runs_router/list_runs", read_only_hint = true)
    )]
    pub async fn list_runs(&self, params: Parameters<ListParams>) -> error::McpResult {
        let Parameters(ListParams {
            filter,
            order_by,
            limit,
        }) = params;

        let runs = self
            .run_service
            .list_runs(filter, order_by, limit)
            .await
            .map_err(from_anyhow)?;

        let runs = with_urls(&runs, |r| self.url_service.build_run_url(&r.run_id).ok())?;

        Ok(CallToolResult::structured(
            serde_json::json!({ "runs": runs }),
        ))
    }

    #[tool(
        name = "update_run",
        description = "
            Update specific fields of an existing run, identified by `run_id`. This is a WRITE. Only the fields you
            set are changed; everything else on the run is preserved. Wraps `runs/v2 UpdateRun`.

            Output:
              - `{ \"run\": Run, \"run_url\": string|null, \"next_step\": string }`. The returned `Run` is the
                post-update state from the server; `run_url` is its Sift web link (`<host>/run/<run_id>`), or null
                when the host can't be derived.

            Parameters:
              - `run_id`: required; the id of the run to update.
              - `name`: optional new name.
              - `description`: optional new description.
              - `start_time_unix_nanos` / `stop_time_unix_nanos`: optional new bounds in Unix nanoseconds.
              - `is_pinned`: optional; pin or unpin the run.
              - `client_key`: optional caller-defined key. Can be set only ONCE — a second attempt errors.
              - `tags`: optional; REPLACES the full tag list. Pass `[]` to clear all tags.
              - `metadata`: optional; REPLACES the full metadata list. Each entry is
                `{ \"name\": \"<key>\", \"value\": <scalar> }`. Pass `[]` to clear.

              At least one updatable field besides `run_id` must be set; otherwise the tool returns `INVALID_PARAMS`.

            Errors:
              - `INVALID_PARAMS` if `run_id` is empty or no updatable field is provided.
              - `RESOURCE_NOT_FOUND` if no run matches `run_id`.
              - `INTERNAL_ERROR` for upstream gRPC failures (e.g. re-setting an already-set `client_key`).

            Guidance:
              - This is a write with REPLACE semantics for `tags`/`metadata`. CONFIRM the full proposed values with
                the user before invoking; for appends, read the run via `list_runs` and send the union.
              - Note: `start_time` may be overwritten automatically if data is later ingested for this run.
        ",
        annotations(title = "runs_router/update_run", read_only_hint = false)
    )]
    pub async fn update_run(&self, params: Parameters<UpdateRunParams>) -> error::McpResult {
        let Parameters(UpdateRunParams {
            run_id,
            name,
            description,
            start_time_unix_nanos,
            stop_time_unix_nanos,
            is_pinned,
            client_key,
            tags,
            metadata,
        }) = params;

        if run_id.is_empty() {
            return Err(ErrorData::invalid_params(
                "`run_id` must not be empty",
                None,
            ));
        }

        let has_update = name.is_some()
            || description.is_some()
            || start_time_unix_nanos.is_some()
            || stop_time_unix_nanos.is_some()
            || is_pinned.is_some()
            || client_key.is_some()
            || tags.is_some()
            || metadata.is_some();
        if !has_update {
            return Err(ErrorData::invalid_params(
                "at least one updatable field besides `run_id` must be provided",
                None,
            ));
        }

        let metadata = metadata.map(|m| m.into_iter().map(MetadataValue::from).collect::<Vec<_>>());

        let run = self
            .run_service
            .update_run(
                run_id,
                name,
                description,
                start_time_unix_nanos,
                stop_time_unix_nanos,
                is_pinned,
                client_key,
                tags,
                metadata,
            )
            .await
            .map_err(from_anyhow)?;

        let run_url = self.url_service.build_run_url(&run.run_id).ok();
        let next_step = format!(
            "Updated run `{}` ({}).{} Surface the new state to the user and confirm the change matches their \
             intent. Remember: tags and metadata are REPLACE operations.",
            run.name,
            run.run_id,
            url_clause(run_url.as_deref()),
        );

        let mut result = CallToolResult::structured(serde_json::json!({
            "run": run,
            "run_url": run_url,
            "next_step": next_step,
        }));
        result.content = vec![Content::text(next_step)];
        Ok(result)
    }
}
