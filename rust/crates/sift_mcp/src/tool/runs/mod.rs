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
    tool::common::{
        ListParams, MetadataEntry,
        bulk::{self, BulkItems, Op},
        list_body, url_clause, with_urls,
    },
};

#[cfg(test)]
mod test;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct UpdateRunItem {
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

fn validate_update_run(item: &UpdateRunItem) -> Result<(), ErrorData> {
    if item.run_id.trim().is_empty() {
        return Err(ErrorData::invalid_params(
            "`run_id` must not be empty",
            None,
        ));
    }
    if !(item.name.is_some()
        || item.description.is_some()
        || item.start_time_unix_nanos.is_some()
        || item.stop_time_unix_nanos.is_some()
        || item.is_pinned.is_some()
        || item.client_key.is_some()
        || item.tags.is_some()
        || item.metadata.is_some())
    {
        return Err(ErrorData::invalid_params(
            "at least one updatable field besides `run_id` must be provided",
            None,
        ));
    }
    Ok(())
}

#[tool_router(router = runs_router, vis = "pub(crate)")]
impl SiftMcpServer {
    #[tool(
        name = "list_runs",
        description = "
            List runs in Sift, optionally filtered by a CEL expression and ordered by one or more fields.

            Output:
              - `{ \"runs\": [Run, ...] }`. Each item is the full Sift `Run` shape including `run_id`, `name`,
                `description`, `asset_ids`, `client_key`, `start_time`, `stop_time`, `duration`, `is_pinned`,
                `is_adhoc`, tags, metadata, timestamps, and archive state, plus an added `url` field with the
                run's Sift web link (`<host>/run/<run_id>`). `url` is omitted when the host can't be derived.
                Present each run to the user as a Markdown link with its name as the text and `url` as the target.
              - `asset_ids` is an array of asset IDs. Runs carry no asset names: to show them, call
                `list_assets` with `asset_id in [\"...\", ...]` built from the collected `asset_ids`.
              - `count`: how many items THIS response carries — read it instead of
                counting the array yourself. It is the size of the page you got back, not
                how many items match `filter`.
              - `has_more`: `true` when the service hit `limit` with matches left over, so
                this page is not the whole set. Never report `count` as a total while
                `has_more` is `true` — narrow `filter` or raise `limit` and ask again.

            Parameters:
              - `filter`: CEL expression. Pass an empty string to list everything. Filterable fields:
                `run_id`, `organization_id`, `asset_id`, `asset_name`, `client_key`, `name`, `description`,
                `start_time`, `stop_time`, `duration`, `duration_string`, `tag_id`, `asset_tag_id`,
                `annotation_comments_count`, `annotation_state`, `created_date`, `modified_date`, `archived_date`,
                `is_archived`, `created_by_user_id`, `modified_by_user_id`, `metadata`.
                Reference metadata entries as `metadata[\"<key>\"]`.
                `duration` is elapsed seconds (numeric). `duration_string` accepts `h`/`m`/`s`/`ms` suffixes via the
                `duration(...)` helper, e.g. `duration_string > duration('10h')`.
                When filtering or searching, use `name.matches(\"(?i)rover\")`, not `==`. Use `==` only for an
                exact value from a prior result. `contains`/`startsWith`/`endsWith` are case-SENSITIVE:
                `contains(\"Rover\")` silently misses `rover-01`.
              - `order_by`: optional comma-separated `FIELD_NAME[ desc]` list. Orderable fields: `name`,
                `description`, `created_date`, `modified_date`, `start_time`, `stop_time`. Default sort is
                `created_date desc` (newest first). Example: `\"created_date desc,modified_date\"`.
              - `limit`: max items to return. Start at 50 and only raise it if the result is capped
                and you still need more. Values are clamped to `1..=200`; omitting it defaults to 50.
              - `fields`: optional array of field names to keep on each item, e.g.
                `[\"name\"]`. Omit it for the full object. Names match case-insensitively
                and ignore underscores and hyphens, so `asset_ids`, `assetIds` and
                `asset-ids` all work. Name output fields, not filter fields: `asset_id`,
                `asset_name`, and `annotation_state` filter runs but are not on the item.
                Any name that matched nothing on any returned item
                is listed in `unmatched_fields`; an empty page reports none, since it
                says nothing about whether a name was spelled right.
                Reach for this whenever you need only a few fields: full objects are wide,
                and a large listing can exceed the response size limit without it.

            Errors:
              - `INVALID_PARAMS` if `filter` is not a valid CEL expression or `order_by` references an unknown field.
              - `INTERNAL_ERROR` for upstream gRPC failures.

            Guidance:
              - When the caller already knows the asset, narrow with `asset_id == \"...\"` first — it's the most
                selective field.
              - To find runs covering a specific moment, filter on both `start_time` and `stop_time` rather than
                pulling everything and filtering client-side.
              - Order by `start_time desc` when surfacing the most recent runs to a user.
              - Default add `is_archived == false` to the filter. Include archived runs only when the user
                explicitly asks for them.
        ",
        annotations(title = "runs/list_runs", read_only_hint = true)
    )]
    pub async fn list_runs(&self, params: Parameters<ListParams>) -> error::McpResult {
        let Parameters(ListParams {
            filter,
            order_by,
            limit,
            fields,
        }) = params;

        let page = self
            .run_service
            .list_runs(filter, order_by, limit)
            .await
            .map_err(from_anyhow)?;

        let runs = with_urls(&page.items, |r| {
            self.url_service.build_run_url(&r.run_id).ok()
        })?;

        Ok(list_body("runs", runs, fields, page.has_more).into_result(None))
    }

    #[tool(
        name = "update_run",
        description = "
            Update specific fields of one or more runs, each identified by `run_id`. This is a WRITE. Only the fields you
            set are changed; everything else on the run is preserved. Wraps `runs/v2 UpdateRun`.

            Output:
              - `{ requested_count, selected_count, not_selected, updated_count, successes, failures,
                not_attempted, next_step }`. Each success holds `{ index, result }`, each failure holds
                `{ index, error, outcome_unknown }`, and any failure sets `isError`. Each result is:
              - `{ \"run\": Run, \"run_url\": string|null, \"next_step\": string }`. The returned `Run` is the
                post-update state from the server; `run_url` is its Sift web link (`<host>/run/<run_id>`), or null
                when the host can't be derived.

            Parameters:
              - `items`: required array of 1–100 objects, one per run. Each item takes the fields below.
              - `selected_indices`: zero-based item positions supplied by the approval host; omit when
                proposing. Omitted selects all outside chat; chat requires a mask. `[]` updates nothing.
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
              - `INVALID_PARAMS` before any write for an empty or oversized `items`, duplicate targets,
                unknown item fields, or invalid items; the error names the item index.
              - Item-local errors fail only that item. Backend-wide errors stop the remaining items.

            Guidance:
              - This is a write with REPLACE semantics for `tags`/`metadata`. CONFIRM the full proposed values with
                the user before invoking; for appends, read the run via `list_runs` and send the union.
              - Note: `start_time` may be overwritten automatically if data is later ingested for this run.
              - Items run sequentially; the call is not atomic. Never replay the whole call after a
                partial failure. Check `outcome_unknown` items before retrying, and never retry `not_selected` items.
        ",
        annotations(
            title = "runs/update_run",
            read_only_hint = false,
            destructive_hint = true,
            idempotent_hint = true,
        )
    )]
    pub async fn update_run(
        &self,
        params: Parameters<BulkItems<UpdateRunItem>>,
    ) -> error::McpResult {
        self.require_destructive()?;
        let Parameters(BulkItems {
            items,
            selected_indices,
            ..
        }) = params;
        let items = bulk::prepare(
            items,
            "run",
            |item: UpdateRunItem| {
                validate_update_run(&item)?;
                Ok(item)
            },
            |item| Some(item.run_id.clone()),
        )?;
        bulk::run(
            Op::Update,
            items,
            self.selection(selected_indices),
            false,
            |item| self.update_run_item(Parameters(item)),
        )
        .await
    }

    async fn update_run_item(&self, params: Parameters<UpdateRunItem>) -> error::McpResult {
        self.require_destructive()?;
        validate_update_run(&params.0)?;
        let Parameters(UpdateRunItem {
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
            "Updated run `{}` ({}).{}",
            run.name,
            run.run_id,
            url_clause("run", Some(&run.name), run_url.as_deref()),
        );

        let mut result = CallToolResult::structured(serde_json::json!({
            "run": run,
            "run_url": run_url,
            "next_step": next_step,
        }));
        result.content = vec![ContentBlock::text(next_step)];
        Ok(result)
    }
}
