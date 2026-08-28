use std::{collections::HashSet, fs::File, path::PathBuf};

use anyhow::Context;
use rmcp::{
    ErrorData,
    handler::server::wrapper::Parameters,
    model::{CallToolResult, ContentBlock},
    schemars::{self, JsonSchema},
    tool, tool_router,
};
use serde::Deserialize;
use serde_json::Value;

use sift_rs::metadata::v1::MetadataValue;

use crate::{
    error::{self, from_anyhow},
    server::SiftMcpServer,
    service::{
        calculated_channels::UnresolvedCalculation,
        common::{self, cel_escape},
        data::{ChannelInput, DataService, NoChannelData, TimeRange},
        ingest::RunForm,
    },
    tool::common::MetadataEntry,
};

#[cfg(test)]
mod test;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct GetDataParams {
    asset_name: Option<String>,
    asset_id: Option<String>,
    run_name: Option<String>,
    start_time_unix_nanos: Option<i64>,
    end_time_unix_nanos: Option<i64>,
    sample_ms: u32,
    channel_names: Option<Vec<String>>,
    channel_regex: Option<String>,
    output: PathBuf,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct SqlParams {
    inputs: Vec<PathBuf>,
    table_name: String,
    query: String,
    output: PathBuf,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct UploadDatasetParams {
    asset: String,
    run_name: Option<String>,
    #[serde(default)]
    tags: Vec<String>,
    #[serde(default)]
    metadata: Vec<MetadataEntry>,
    input: PathBuf,
}

fn string_array(values: Vec<String>) -> Value {
    Value::Array(values.into_iter().map(Value::String).collect())
}

/// `ErrorData.data` payload naming the channels a failed `get_data` did not
/// return.
///
/// `empty_channels` is `Some` only for the no-data error, which is the only
/// failure that knows which channels came back without samples; a transport
/// failure never got far enough to tell. `unmatched_channel_names` is known
/// from the request itself, so it rides along on any failure. Returns `None`
/// when there is nothing to report, leaving `data` unset rather than carrying
/// two empty arrays on every unrelated error.
fn gap_report(empty_channels: Option<Vec<String>>, unmatched: &[String]) -> Option<Value> {
    if empty_channels.is_none() && unmatched.is_empty() {
        return None;
    }

    let mut payload = serde_json::Map::new();
    if let Some(empty_channels) = empty_channels {
        payload.insert("empty_channels".to_string(), string_array(empty_channels));
    }
    if !unmatched.is_empty() {
        payload.insert(
            "unmatched_channel_names".to_string(),
            string_array(unmatched.to_vec()),
        );
    }
    Some(Value::Object(payload))
}

#[tool_router(router = data_router, vis = "pub(crate)")]
impl SiftMcpServer {
    #[tool(
        name = "get_data",
        description = "
            Retrieve time-series data for one or more channels of a single asset and write the result to a Parquet file.
            Serves both raw channels and saved calculated channels.

            Output schema:
              - Column 0 is `timestamp_unix_nanos` (Int64, non-null) holding the merged ascending timestamps across all
                requested channels.
              - One column per matched channel, named `<channel_name> {channel_id=\"...\", run=\"...\", units=\"...\"}`.
                Cells are null where that channel has no sample at the row's timestamp. A saved calculated channel
                has no channel id, so its column carries the calculated channel's name in both places.
              - Enum and BitField channels carry their decode config in field metadata under the `enum_config` and
                `bit_field_elements` keys respectively.
              - A requested channel that produced no samples has NO column at all, not an all-null one. The tool
                result reports these so they never have to be inferred from the schema:
                `unmatched_channel_names` lists requested names served by neither a raw channel nor a saved
                calculated channel, and `empty_channels` lists channels that matched but returned no samples in
                the window. Both keys are ALWAYS present; two empty arrays mean every requested channel is in the
                file. When either is non-empty, name those channels to the user before presenting any analysis —
                the file is a partial answer.
              - `unresolved_calculated_channels` (`[{ \"name\", \"reason\" }]`) is present when a requested name
                reached calculated-channel resolution and could not be served. It carries the reason for every
                name in `unmatched_channel_names`.

            Parameters:
              - `asset_name`: optional, exact asset name (not a pattern). Mutually exclusive with `asset_id`;
                exactly one of the two MUST be set.
              - `asset_id`: optional, asset UUID. Mutually exclusive with `asset_name`; exactly one of the two
                MUST be set. Runs reference assets by id (`list_runs` returns `asset_id`, not a name), so pass
                that id here directly instead of guessing a name.
              - `run_name`: optional, exact run name within the asset. When provided, the run's start/stop bounds are
                used as the time range; `start_time_unix_nanos` and/or `end_time_unix_nanos` may narrow either side.
                When omitted, BOTH `start_time_unix_nanos` and `end_time_unix_nanos` are required.
              - `sample_ms`: decimation interval in milliseconds. Use `0` for raw samples; larger values reduce volume.
              - `channel_names`: optional array of exact channel names. Mutually exclusive with `channel_regex`;
                exactly one of the two MUST be set. Prefer this form when the set is known — it's more predictable.
                A name with no raw channel on the asset is resolved as an active saved calculated channel and
                evaluated for the requested asset and run. A raw channel wins a name it shares with a calculated
                channel, so a calculated channel named after an existing raw channel is not served here.
              - `channel_regex`: optional RE2 pattern matched against the channel name. Mutually exclusive with
                `channel_names`; exactly one of the two MUST be set. Matches raw channels only; name saved
                calculated channels explicitly in `channel_names`.
              - `output`: filesystem path for the Parquet file. The file is opened in truncate mode; existing
                contents are overwritten.

            Errors:
              - `RESOURCE_NOT_FOUND` if the asset or run is missing or there are no matching channels.
              - `INTERNAL_ERROR` if every matched channel returned no samples in the window. The message names the
                channels; widen the time range or drop the run scope rather than concluding the asset has no data.
                The error's `data` carries `empty_channels`, and `unmatched_channel_names` when the request also
                held a name that matched nothing — a failed call still reports both, so a retry does not repeat a
                typo the first call already detected.
              - `RESOURCE_NOT_FOUND` naming every unresolved name when nothing requested can be served: no raw
                channel matched and no named calculated channel exists or applies to the asset. A calculated
                channel does not apply when the asset is outside its scope or lacks a channel its expression
                references. Verify the name with `list_calculated_channels` filtered by `asset_id`.
              - `INVALID_PARAMS` if neither `asset_name` nor `asset_id` is set, or if both are set.
              - `INVALID_PARAMS` if `run_name` is absent and the full time range is not supplied, if neither
                `channel_names` nor `channel_regex` is set, if both are set, or if `channel_names` is empty.
              - `INVALID_PARAMS` if the channel selection matches 200 or more channels — the result would be
                silently incomplete. Narrow `channel_regex`, pass explicit `channel_names`, or split the request
                into multiple calls.

            Guidance:
              - If the user's intent is to view/plot/graph/visualize the data in a UI, call `explore_url` first
                instead — it returns a Sift Explore deep-link and skips the download entirely. Use `get_data` only
                when the bytes are needed locally for SQL, custom analysis, or a static artifact the user explicitly
                asked for.
              - Data is buffered in memory until size/row thresholds are hit, so very large time ranges or wide
                channel sets can be slow or memory-heavy. For large pulls, split the time range into successive calls
                with disjoint `[start, end)` windows.
              - Use `sample_ms > 0` for overview/summary work; reserve `sample_ms = 0` for cases that need raw fidelity.
              - A successful call does NOT mean every requested channel is in the file. Check
                `unmatched_channel_names` and `empty_channels` before reporting the result or aggregating over it,
                and check the same two keys on the error's `data` when a call fails.
              - A partial result is possible: when some calculated channels resolve and others do not, the file is
                written from what resolved and `unresolved_calculated_channels` names the rest. Never report on the
                data without telling the user what is missing.
              - After a successful call, if the user hasn't already indicated a next step, offer to run a SQL query
                against the resulting Parquet file using the `sql` tool.
        ",
        annotations(title = "data/get_data", read_only_hint = true)
    )]
    pub async fn get_data(&self, params: Parameters<GetDataParams>) -> error::McpResult {
        let Parameters(GetDataParams {
            asset_name,
            asset_id,
            run_name,
            channel_names,
            channel_regex,
            start_time_unix_nanos,
            end_time_unix_nanos,
            sample_ms,
            output,
        }) = params;

        if run_name.is_none() && (start_time_unix_nanos.is_none() || end_time_unix_nanos.is_none())
        {
            return Err(ErrorData::invalid_params(
                "start_time_unix_nanos and end_time_unix_nanos are required when run_name is not provided",
                None,
            ));
        }

        let (asset_filter, asset_label) = match (&asset_name, &asset_id) {
            (Some(_), Some(_)) => {
                return Err(ErrorData::invalid_params(
                    "exactly one of `asset_name` or `asset_id` must be set, not both",
                    None,
                ));
            }
            (None, None) => {
                return Err(ErrorData::invalid_params(
                    "one of `asset_name` or `asset_id` must be set",
                    None,
                ));
            }
            (Some(name), None) => (
                format!("name == \"{}\"", cel_escape(name)),
                format!("asset '{name}'"),
            ),
            (None, Some(id)) => (
                format!("asset_id == \"{}\"", cel_escape(id)),
                format!("asset with id '{id}'"),
            ),
        };
        let asset = self
            .asset_service
            .list_assets(asset_filter, None, Some(1))
            .await
            .map_err(from_anyhow)?
            .items
            .into_iter()
            .next()
            .ok_or_else(|| {
                ErrorData::resource_not_found(format!("{asset_label} not found"), None)
            })?;

        let run = match run_name.as_deref() {
            Some(name) => {
                let filter = format!(
                    "name == \"{}\" && asset_id == \"{}\"",
                    cel_escape(name),
                    asset.asset_id,
                );
                let run = self
                    .run_service
                    .list_runs(filter, None, Some(1))
                    .await
                    .map_err(from_anyhow)?
                    .items
                    .into_iter()
                    .next()
                    .ok_or_else(|| {
                        ErrorData::resource_not_found(
                            format!("run '{name}' not found for asset '{}'", asset.name),
                            None,
                        )
                    })?;
                Some(run)
            }
            None => None,
        };

        // Hold on to the caller's own strings: `name in [...]` matches what it can
        // and says nothing about the rest, so this is the last point at which an
        // unmatched name can still be identified.
        let (channel_search_filter, requested_names) = match (channel_names, channel_regex) {
            (Some(_), Some(_)) => {
                return Err(ErrorData::invalid_params(
                    "exactly one of `channel_names` or `channel_regex` must be set, not both",
                    None,
                ));
            }
            (None, None) => {
                return Err(ErrorData::invalid_params(
                    "one of `channel_names` or `channel_regex` must be set",
                    None,
                ));
            }
            (Some(names), None) => {
                if names.is_empty() {
                    return Err(ErrorData::invalid_params(
                        "`channel_names` must contain at least one name",
                        None,
                    ));
                }
                let items = names
                    .iter()
                    .map(|n| format!("\"{}\"", cel_escape(n)))
                    .collect::<Vec<_>>()
                    .join(", ");
                (format!("name in [{items}]"), Some(names))
            }
            (None, Some(pattern)) => (format!("name.matches(\"{}\")", cel_escape(&pattern)), None),
        };
        let channel_filter = format!(
            "asset_id == \"{}\" && {channel_search_filter}",
            asset.asset_id
        );

        let page = self
            .channel_service
            .list_channels(channel_filter, None, Some(common::PAGE_SIZE))
            .await
            .map_err(from_anyhow)?;
        let channels = page.items;

        // A raw channel wins a name it shares with a saved calculated channel;
        // only names with no raw channel go on to calculated-channel resolution.
        // A regex selection carries no per-name expectation, so it contributes none.
        // Repeats are dropped: two queries sharing a channel key merge into one
        // column, which duplicates timestamps in the output.
        let unmatched_names = requested_names
            .map(|names| {
                let matched = channels
                    .iter()
                    .map(|c| c.name.as_str())
                    .collect::<HashSet<_>>();

                let mut unmatched = Vec::<String>::new();
                for name in names {
                    if !matched.contains(name.as_str()) && !unmatched.contains(&name) {
                        unmatched.push(name);
                    }
                }
                unmatched
            })
            .unwrap_or_default();

        if channels.is_empty() && unmatched_names.is_empty() {
            return Err(ErrorData::resource_not_found(
                format!(
                    "no channels matched the search criteria for asset '{}'",
                    asset.name
                ),
                None,
            ));
        }

        // A truncated selection would write a Parquet file that is silently
        // missing channels. The service reports the cut directly now, so this no
        // longer has to infer it from the result filling the record cap — which
        // also cried wolf whenever a selection landed on exactly that many.
        if page.has_more {
            return Err(ErrorData::invalid_params(
                format!(
                    "channel selection matched more than {} channels and is incomplete; \
                     narrow `channel_regex`, pass explicit `channel_names`, or split the \
                     request into multiple calls",
                    common::PAGE_SIZE
                ),
                None,
            ));
        }

        let mut channel_inputs = channels
            .into_iter()
            .map(|c| ChannelInput::Raw(Box::new(c)))
            .collect::<Vec<_>>();

        let mut unresolved = Vec::new();
        if !unmatched_names.is_empty() {
            let resolution = self
                .calculated_channel_service
                .resolve_calculated_channels(
                    unmatched_names,
                    asset.asset_id.clone(),
                    run.as_ref().map(|r| r.run_id.clone()),
                )
                .await
                .map_err(from_anyhow)?;

            channel_inputs.extend(resolution.resolved.into_iter().map(|calculation| {
                ChannelInput::SavedCalculation {
                    channel_key: calculation.name,
                    expression_request: Box::new(calculation.expression_request),
                }
            }));
            unresolved = resolution.unresolved;
        }

        let unresolved_report = (!unresolved.is_empty())
            .then(|| unresolved_message(&unresolved, &asset.name, run_name.as_deref()));

        if channel_inputs.is_empty() {
            return Err(ErrorData::resource_not_found(
                unresolved_report.unwrap_or_else(|| {
                    format!(
                        "no channels matched the search criteria for asset '{}'",
                        asset.name
                    )
                }),
                None,
            ));
        }

        // Everything the request asked for and neither path could serve. A name
        // that resolved to a calculated channel is in the file, so it is not
        // unmatched; what is left is exactly the unresolved set.
        let unmatched_channel_names = unresolved
            .iter()
            .map(|entry| entry.name.clone())
            .collect::<Vec<_>>();

        let time_range = match run {
            Some(run) => TimeRange::Run {
                run: Box::new(run),
                start_time_unix_nanos,
                end_time_unix_nanos,
            },
            None => TimeRange::Asset {
                start_time_unix_nanos: start_time_unix_nanos.expect("validated above"),
                end_time_unix_nanos: end_time_unix_nanos.expect("validated above"),
            },
        };

        let mut file = File::options()
            .create(true)
            .write(true)
            .truncate(true)
            .open(&output)
            .context("failed to open output parquet file")
            .map_err(from_anyhow)?;

        let data_output = match self
            .data_service
            .get_data(&channel_inputs, time_range, sample_ms, &mut file)
            .await
        {
            Ok(output) => output,
            Err(err) => {
                // The no-data error names the channels that came back empty, but
                // the names that matched nothing were computed up here and would
                // be lost with the early return. A caller told only "no samples
                // for pressure" widens the window, retries, and is still
                // carrying the typo nothing has mentioned.
                let empty_channels = err
                    .downcast_ref::<NoChannelData>()
                    .map(|no_data| no_data.empty_channels.clone());
                let err = err.context("get data call failure - data_router");
                // A failure here says nothing about channels that were never
                // queried, so carry the report into it. Otherwise an empty window
                // reads as "the asset has no data" when part of the request never
                // resolved.
                let err = match unresolved_report.as_ref() {
                    Some(report) => err.context(report.clone()),
                    None => err,
                };
                let mut error = from_anyhow(err);
                error.data = gap_report(empty_channels, &unmatched_channel_names);
                return Err(error);
            }
        };

        let output_str = output.to_string_lossy().into_owned();

        let mut gaps = Vec::new();
        if let Some(report) = &unresolved_report {
            gaps.push(format!("{report}."));
        }
        if !data_output.empty_channels.is_empty() {
            gaps.push(format!(
                "{} returned no samples in this window: {}.",
                data_output.empty_channels.len(),
                common::name_list(&data_output.empty_channels),
            ));
        }

        let mut next_step =
            format!("Wrote channel data to `{output_str}`. Inform the user where the data lives.");
        // Without this the agent reads a plain success and reports a partial
        // fetch as a complete one, because a channel with no column is
        // indistinguishable from one that was never requested.
        if !gaps.is_empty() {
            next_step.push_str(&format!(
                " The file does NOT have a column for every channel requested. {} Name those \
                 channels to the user before presenting any analysis, and do not describe the \
                 fetch as complete.",
                gaps.join(" "),
            ));
        }
        next_step.push_str(
            " If the user hasn't already indicated a next step, offer to run a SQL query against \
             this file with the `sql` tool — for example to filter, aggregate, or summarize the \
             data.",
        );

        let mut body = serde_json::Map::new();
        body.insert("output".to_string(), Value::from(output_str));
        body.insert("next_step".to_string(), Value::from(next_step.clone()));
        // Always present, even when empty. Omitting them would leave "every
        // channel arrived" and "this tool never checked" looking identical, and
        // the first is the assurance a caller needs before trusting the file.
        // Deliberately unlike `unmatched_fields` on the list tools, where
        // silence costs nothing.
        body.insert(
            "unmatched_channel_names".to_string(),
            string_array(unmatched_channel_names),
        );
        body.insert(
            "empty_channels".to_string(),
            string_array(data_output.empty_channels),
        );

        if !unresolved.is_empty() {
            body.insert(
                "unresolved_calculated_channels".to_string(),
                serde_json::json!(
                    unresolved
                        .iter()
                        .map(|entry| serde_json::json!({
                            "name": entry.name,
                            "reason": entry.reason,
                        }))
                        .collect::<Vec<_>>()
                ),
            );
        }

        let mut result = CallToolResult::structured(Value::Object(body));
        result.content = vec![ContentBlock::text(next_step)];
        Ok(result)
    }

    #[tool(
        name = "sql",
        description = "
            Run a SQL query against one or more Parquet files and write the result to a new Parquet file. Intended
            to be chained after `get_data` for downstream analysis.

            Output:
              - Parquet file at `output`. Schema is whatever the query produces; column types are inferred from the
                SELECT clause and the input columns.
              - Tool result is `{ \"output\": \"<path>\" }`.

            Parameters:
              - `inputs`: one or more filesystem paths to Parquet files. All files MUST share the same schema —
                they are unioned into a single relation. Order is preserved within each file but not across files.
              - `table_name`: identifier used in `query` to reference the unioned inputs. Must be a valid SQL
                identifier (e.g. `t`, `samples`).
              - `query`: Polars SQL query. The relation `table_name` is the only registered table. Supports
                SELECT/WHERE/GROUP BY/ORDER BY/aggregates and the rest of standard SQL.
              - `output`: filesystem path for the result Parquet file. The file is opened in truncate mode; existing
                contents are overwritten.

            Errors:
              - `INVALID_PARAMS` if `inputs` is empty.
              - `INTERNAL_ERROR` if a path is invalid, an input cannot be scanned, the query fails to parse or
                execute, or the output file cannot be written.

            Guidance:
              - Common pipeline: `get_data` → `sql`. Pass the path returned by `get_data` as one of the `inputs`.
              - The full result is materialized in memory before being written; project (`SELECT col1, col2`) and
                filter (`WHERE ...`) aggressively when inputs are large.
              - All inputs must share an identical schema. To combine data with different channel sets, run separate
                `sql` calls and join the outputs in a follow-up call.
              - Downstream `upload_dataset` requires column 0 of its input to be `timestamp_unix_nanos` (Int64,
                non-null). If the result of this query may be uploaded back to Sift, project `timestamp_unix_nanos`
                first in the `SELECT` and do not rename or drop it. Aggregations that collapse rows MUST still emit
                `timestamp_unix_nanos` — bucket on it (e.g. group by a time expression derived from it) or pick a
                representative via `MIN(timestamp_unix_nanos)`.
        ",
        annotations(title = "data/sql", read_only_hint = true)
    )]
    pub async fn sql(&self, params: Parameters<SqlParams>) -> error::McpResult {
        let Parameters(SqlParams {
            inputs,
            table_name,
            query,
            output,
        }) = params;

        if inputs.is_empty() {
            return Err(ErrorData::invalid_params(
                "inputs must contain at least one parquet file path",
                None,
            ));
        }

        let output_for_task = output.clone();
        tokio::task::spawn_blocking(move || -> anyhow::Result<()> {
            let mut file = File::options()
                .create(true)
                .write(true)
                .truncate(true)
                .open(&output_for_task)
                .context("failed to open output parquet file")?;
            DataService::sql(inputs, &mut file, &table_name, &query)
        })
        .await
        .map_err(|e| ErrorData::internal_error(format!("sql task panicked: {e}"), None))?
        .map_err(from_anyhow)?;

        let output_str = output.to_string_lossy().into_owned();
        let next_step = format!(
            "Wrote SQL result to `{output_str}`. If the user hasn't already indicated a next step, \
             offer to upload this dataset back to Sift via the `upload_dataset` tool. Before calling \
             `upload_dataset`, CONFIRM with the user: \
             (1) the target `asset` name (suggest a sensible default but let the user override), \
             (2) whether to create a run (optional `run_name`; required if tags/metadata are wanted), \
             (3) any tags to attach to the run, and \
             (4) any metadata entries (name + scalar value) to attach to the run."
        );

        let mut result = CallToolResult::structured(serde_json::json!({
            "output": output_str,
            "next_step": next_step,
        }));
        result.content = vec![ContentBlock::text(next_step)];
        Ok(result)
    }

    #[tool(
        name = "upload_dataset",
        description = "
            Upload a Parquet dataset (typically the output of `get_data` or `sql`) to Sift. The file is streamed
            row-by-row to Sift's ingest service under an automatically-derived flow.

            Expected input schema (every rule is enforced; a violation rejects the whole file before
            anything is created in Sift):
              - The file MUST have at least two columns: `timestamp_unix_nanos` plus one channel column.
              - Column 0 MUST be `timestamp_unix_nanos` (Int64) and MUST be declared non-nullable in the
                Parquet schema.
              - Every other column MUST carry the brace-delimited attribute block in its column name:
                `<channel_name> {channel_id=\"...\"[, bit_field_element=\"...\"][, run=\"...\"][, units=\"...\"]}`.
                The single space before `{` is required; a bare column name without the ` {...}` block is
                rejected.
              - `channel_id` is REQUIRED inside the block; the bracketed attributes are optional. This is the
                canonical form produced by `get_data`. Enum and BitField channels are recognized via field
                metadata under the `enum_config` and `bit_field_elements` keys respectively.

            Output:
              - `{ \"input\": \"<path>\", \"next_step\": \"...\" }`.

            Parameters:
              - `asset`: name of the Sift asset to ingest into. The Sift server creates the asset if it does not
                yet exist.
              - `run_name`: optional run name. When provided a new run is created on the server and all uploaded
                samples are scoped to it. When omitted, samples are ingested without a run, and `tags`/`metadata`
                are ignored.
              - `tags`: optional list of tag strings applied to the new run. Ignored unless `run_name` is set.
              - `metadata`: optional list of `{ \"name\": \"<key>\", \"value\": <scalar> }` entries applied to the
                new run. `value` may be a string, number, or boolean — the type is inferred from the JSON literal.
                Ignored unless `run_name` is set.
              - `input`: filesystem path to the source Parquet file.

            Errors:
              - `INVALID_PARAMS` if `tags` or `metadata` is supplied without a `run_name`.
              - `INTERNAL_ERROR` for parquet open/parse failures, invalid column names, ingestion-config or run
                creation failures, and gRPC stream failures.

            Guidance:
              - Common pipeline: `get_data` → (optionally `sql`) → `upload_dataset`. Pass the `output` path
                returned by the prior step as `input` here.
              - Before invoking this tool, CONFIRM the destination with the user: target `asset`, whether to
                create a `run_name` (required for `tags`/`metadata`), and the specific tags/metadata to attach.
                Do not silently default these — surface them for the user to override.
              - The tool does not return until the entire stream has been consumed by the server, so large
                datasets translate to long-running calls.
        ",
        annotations(
            title = "data/upload_dataset",
            read_only_hint = false,
            destructive_hint = false,
            idempotent_hint = false,
        )
    )]
    pub async fn upload_dataset(
        &self,
        params: Parameters<UploadDatasetParams>,
    ) -> error::McpResult {
        self.require_create()?;

        let Parameters(UploadDatasetParams {
            asset,
            run_name,
            tags,
            metadata,
            input,
        }) = params;

        if run_name.is_none() && (!tags.is_empty() || !metadata.is_empty()) {
            return Err(ErrorData::invalid_params(
                "`tags` and `metadata` require a `run_name` — they attach to the created run",
                None,
            ));
        }

        let run = run_name.map(|name| RunForm {
            name,
            tags,
            metadata: metadata.into_iter().map(MetadataValue::from).collect(),
        });

        let file = File::open(&input)
            .context("failed to open input parquet file")
            .map_err(from_anyhow)?;

        let uploaded = self
            .ingest_service
            .upload_dataset(asset, run, file)
            .await
            .context("upload dataset failure - data_router")
            .map_err(from_anyhow)?;

        let input_str = input.to_string_lossy().into_owned();
        let run_summary = match (&uploaded.run_name, &uploaded.run_id) {
            (Some(name), Some(id)) => format!(" (run `{name}`, id `{id}`)"),
            _ => String::new(),
        };
        let next_step = format!(
            "Uploaded `{input_str}` to Sift asset `{}` (id `{}`){run_summary}. \
             Inform the user where the data landed. If the user hasn't already indicated a next \
             step, offer to verify the ingest via `list_runs` (if a run was created) or \
             `list_channels`.",
            uploaded.asset_name, uploaded.asset_id,
        );

        let mut result = CallToolResult::structured(serde_json::json!({
            "input": input_str,
            "asset_name": uploaded.asset_name,
            "asset_id": uploaded.asset_id,
            "run_name": uploaded.run_name,
            "run_id": uploaded.run_id,
            "next_step": next_step,
        }));
        result.content = vec![ContentBlock::text(next_step)];
        Ok(result)
    }
}

/// One phrasing for calculated channels that yielded no data, shared by the
/// partial-result report and the all-unresolved error so the caller reads the
/// same wording either way.
fn unresolved_message(
    unresolved: &[UnresolvedCalculation],
    asset_name: &str,
    run_name: Option<&str>,
) -> String {
    let scope = match run_name {
        Some(run) => format!("asset '{asset_name}' (run '{run}')"),
        None => format!("asset '{asset_name}'"),
    };
    let items = unresolved
        .iter()
        .map(|entry| format!("'{}' ({})", entry.name, entry.reason))
        .collect::<Vec<_>>()
        .join("; ");

    format!("calculated channel data was not resolved for {scope}: {items}")
}
