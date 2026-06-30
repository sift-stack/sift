use std::{fs::File, path::PathBuf};

use anyhow::Context;
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
    service::{
        data::{ChannelInput, DataService, TimeRange},
        ingest::RunForm,
    },
    tool::common::MetadataEntry,
};

#[derive(Debug, Deserialize, JsonSchema)]
pub struct GetDataParams {
    asset_name: String,
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

#[tool_router(router = data_router, vis = "pub(crate)")]
impl SiftMcpServer {
    #[tool(
        name = "get_data",
        description = "
            Retrieve time-series data for one or more channels of a single asset and write the result to a Parquet file.

            Output schema:
              - Column 0 is `timestamp_unix_nanos` (Int64, non-null) holding the merged ascending timestamps across all
                requested channels.
              - One column per matched channel, named `<channel_name> {channel_id=\"...\", run=\"...\", units=\"...\"}`.
                Cells are null where that channel has no sample at the row's timestamp.
              - Enum and BitField channels carry their decode config in field metadata under the `enum_config` and
                `bit_field_elements` keys respectively.

            Parameters:
              - `asset_name`: exact asset name (not a pattern).
              - `run_name`: optional, exact run name within the asset. When provided, the run's start/stop bounds are
                used as the time range; `start_time_unix_nanos` and/or `end_time_unix_nanos` may narrow either side.
                When omitted, BOTH `start_time_unix_nanos` and `end_time_unix_nanos` are required.
              - `sample_ms`: decimation interval in milliseconds. Use `0` for raw samples; larger values reduce volume.
              - `channel_names`: optional array of exact channel names. Mutually exclusive with `channel_regex`;
                exactly one of the two MUST be set. Prefer this form when the set is known — it's more predictable.
              - `channel_regex`: optional RE2 pattern matched against the channel name. Mutually exclusive with
                `channel_names`; exactly one of the two MUST be set.
              - `output`: filesystem path for the Parquet file. The file is opened in truncate mode; existing
                contents are overwritten.

            Errors:
              - `RESOURCE_NOT_FOUND` if the asset or run is missing or there are no matching channels.
              - `INVALID_PARAMS` if `run_name` is absent and the full time range is not supplied, if neither
                `channel_names` nor `channel_regex` is set, if both are set, or if `channel_names` is empty.

            Guidance:
              - If the user's intent is to view/plot/graph/visualize the data in a UI, call `explore_url` first
                instead — it returns a Sift Explore deep-link and skips the download entirely. Use `get_data` only
                when the bytes are needed locally for SQL, custom analysis, or a static artifact the user explicitly
                asked for.
              - Data is buffered in memory until size/row thresholds are hit, so very large time ranges or wide
                channel sets can be slow or memory-heavy. For large pulls, split the time range into successive calls
                with disjoint `[start, end)` windows.
              - Use `sample_ms > 0` for overview/summary work; reserve `sample_ms = 0` for cases that need raw fidelity.
              - After a successful call, if the user hasn't already indicated a next step, offer to run a SQL query
                against the resulting Parquet file using the `sql` tool.
        ",
        annotations(title = "data_router/get_data", read_only_hint = true)
    )]
    pub async fn get_data(&self, params: Parameters<GetDataParams>) -> error::McpResult {
        let Parameters(GetDataParams {
            asset_name,
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

        let asset_filter = format!("name == \"{}\"", cel_escape(&asset_name));
        let asset = self
            .asset_service
            .list_assets(asset_filter, None, Some(1))
            .await
            .map_err(from_anyhow)?
            .into_iter()
            .next()
            .ok_or_else(|| {
                ErrorData::resource_not_found(format!("asset '{asset_name}' not found"), None)
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
                    .into_iter()
                    .next()
                    .ok_or_else(|| {
                        ErrorData::resource_not_found(
                            format!("run '{name}' not found for asset '{asset_name}'"),
                            None,
                        )
                    })?;
                Some(run)
            }
            None => None,
        };

        let channel_search_filter = match (channel_names, channel_regex) {
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
                format!("name in [{items}]")
            }
            (None, Some(pattern)) => {
                format!("name.matches(\"{}\")", cel_escape(&pattern))
            }
        };
        let channel_filter = format!(
            "asset_id == \"{}\" && {channel_search_filter}",
            asset.asset_id
        );

        let channels = self
            .channel_service
            .list_channels(channel_filter, None, None)
            .await
            .map_err(from_anyhow)?;

        if channels.is_empty() {
            return Err(ErrorData::resource_not_found(
                format!("no channels matched the search criteria for asset '{asset_name}'"),
                None,
            ));
        }

        let channel_inputs = channels
            .into_iter()
            .map(|c| ChannelInput::Raw(Box::new(c)))
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

        self.data_service
            .get_data(&channel_inputs, time_range, sample_ms, &mut file)
            .await
            .context("get data call failure - data_router")
            .map_err(from_anyhow)?;

        let output_str = output.to_string_lossy().into_owned();
        let next_step = format!(
            "Wrote channel data to `{output_str}`. Inform the user where the data lives. \
             If the user hasn't already indicated a next step, offer to run a SQL query against this \
             file with the `sql` tool — for example to filter, aggregate, or summarize the data."
        );

        let mut result = CallToolResult::structured(serde_json::json!({
            "output": output_str,
            "next_step": next_step,
        }));
        result.content = vec![Content::text(next_step)];
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
        annotations(title = "data_router/sql", read_only_hint = true)
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
        result.content = vec![Content::text(next_step)];
        Ok(result)
    }

    #[tool(
        name = "upload_dataset",
        description = "
            Upload a Parquet dataset (typically the output of `get_data` or `sql`) to Sift. The file is streamed
            row-by-row to Sift's ingest service under an automatically-derived flow.

            Expected input schema:
              - Column 0 MUST be `timestamp_unix_nanos` (Int64, non-null).
              - One column per channel, each named in the canonical
                `<channel_name> {channel_id=\"...\"[, bit_field_element=\"...\"][, run=\"...\"][, units=\"...\"]}`
                form produced by `get_data`. Enum and BitField channels are recognized via field metadata under
                the `enum_config` and `bit_field_elements` keys respectively.

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
            title = "data_router/upload_dataset",
            read_only_hint = false,
            destructive_hint = false,
            idempotent_hint = false,
        )
    )]
    pub async fn upload_dataset(
        &self,
        params: Parameters<UploadDatasetParams>,
    ) -> error::McpResult {
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
        result.content = vec![Content::text(next_step)];
        Ok(result)
    }
}

fn cel_escape(s: &str) -> String {
    s.replace('\\', "\\\\").replace('"', "\\\"")
}
