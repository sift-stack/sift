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

use sift_rs::metadata::v1::{
    MetadataKey, MetadataKeyType, MetadataValue, metadata_value::Value as MetadataValueInner,
};

use crate::{
    error::{self, from_anyhow},
    server::SiftMcpServer,
    service::{
        data::{ChannelInput, DataService, TimeRange},
        ingest::RunForm,
    },
};

#[cfg(test)]
mod test;

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
#[serde(untagged)]
pub enum MetadataScalar {
    String(String),
    Number(f64),
    Boolean(bool),
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct MetadataEntry {
    name: String,
    value: MetadataScalar,
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

#[derive(Debug, Deserialize, JsonSchema)]
pub struct ExploreUrlParams {
    assets: Option<Vec<String>>,
    runs: Option<Vec<String>>,
    channels: Option<Vec<String>>,
    panel_type: Option<String>,
    start_time_unix_nanos: Option<i64>,
    end_time_unix_nanos: Option<i64>,
    explore_host: Option<String>,
}

impl From<MetadataEntry> for MetadataValue {
    fn from(entry: MetadataEntry) -> Self {
        let (key_type, value) = match entry.value {
            MetadataScalar::String(s) => {
                (MetadataKeyType::String, MetadataValueInner::StringValue(s))
            }
            MetadataScalar::Number(n) => {
                (MetadataKeyType::Number, MetadataValueInner::NumberValue(n))
            }
            MetadataScalar::Boolean(b) => (
                MetadataKeyType::Boolean,
                MetadataValueInner::BooleanValue(b),
            ),
        };
        MetadataValue {
            key: Some(MetadataKey {
                name: entry.name,
                r#type: key_type.into(),
                ..Default::default()
            }),
            value: Some(value),
            ..Default::default()
        }
    }
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
        annotations(title = "data_router/upload_dataset", read_only_hint = false)
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

    #[tool(
        name = "explore_url",
        description = "
            Build a Sift Explore deep-link URL for the given asset/run/channel selection. Pure URL construction — no
            API call is made. Hand the returned URL to the user verbatim, rendered inline as a clickable link, so
            they can open the view in the Sift web app.

            Output:
              - `{ \"url\": \"<url>\", \"next_step\": \"...\" }`. `url` is the fully-formed Explore link; `next_step`
                instructs you on how to surface it.

            Parameters:
              - `assets`: optional list of asset names or UUIDs. The Explore service resolves either form.
              - `runs`: optional list of run names or UUIDs. Same resolution rules as `assets`.
              - `channels`: optional list of channel names, UUIDs, or prefixed forms. Axis prefixes (`L1:foo`,
                `L2:bar`) bind a channel to a Y-axis for multi-axis plots. Role prefixes (`x:foo`, `y:foo`,
                `color:foo` for scatter; `lat:foo`, `lon:foo`, `color:foo` for geo-map) bind a channel to a panel
                role. Prefer names returned by `list_assets` / `list_runs` / `list_channels` over guessing.
              - `panel_type`: optional. When omitted Explore defaults to `timeseries`. Unknown values are
                rejected with `INVALID_PARAMS`; the error message enumerates the accepted set.
              - `start_time_unix_nanos`, `end_time_unix_nanos`: optional time window. Provided as Unix nanoseconds
                for parity with `get_data`; the tool converts to ISO 8601 UTC for the URL.
              - `explore_host`: optional override for the Sift web host (e.g. `https://app.staging.siftstack.com`).
                When omitted the host is derived from the server's configured `rest_uri`.

            Errors:
              - `INVALID_PARAMS` if no selection or time parameter is set (the URL would be useless), if
                `panel_type` is not in the known set, if `end_time_unix_nanos < start_time_unix_nanos`, or if the
                Sift web host cannot be derived from `rest_uri` and `explore_host` is unset.

            Guidance:
              - Reach for this tool when the user asks to \"see\", \"view\", \"graph\", \"plot\", \"visualize\", or
                \"open\" data in Sift. Pair it with `get_data` only when the user also wants the data locally for
                SQL or further processing.
              - The tool does not validate that the named asset/run/channel exists — Explore resolves at page
                load. Use names you have already retrieved from `list_*` tools to avoid 404s on click.
        ",
        annotations(title = "data_router/explore_url", read_only_hint = true)
    )]
    pub async fn explore_url(
        &self,
        params: Parameters<ExploreUrlParams>,
    ) -> error::McpResult {
        let Parameters(params) = params;
        let url = build_explore_url(&self.rest_uri, params)?;
        let next_step = format!(
            "Built Sift Explore URL: {url}\n\nRender this URL inline in your response as a \
             clickable markdown link so the user can open the view in Sift. Do not summarize \
             the link away."
        );

        let mut result = CallToolResult::structured(serde_json::json!({
            "url": url,
            "next_step": next_step,
        }));
        result.content = vec![Content::text(next_step)];
        Ok(result)
    }
}

fn cel_escape(s: &str) -> String {
    s.replace('\\', "\\\\").replace('"', "\\\"")
}

const KNOWN_PANEL_TYPES: &[&str] = &[
    "timeseries",
    "histogram",
    "table",
    "fft",
    "metrics",
    "scatter-plot",
    "geo-map",
];

const VALUE_ENCODE_SET: &percent_encoding::AsciiSet = &percent_encoding::NON_ALPHANUMERIC
    .remove(b'-')
    .remove(b'_')
    .remove(b'.')
    .remove(b'~')
    .remove(b':');

fn encode_value(s: &str) -> String {
    percent_encoding::utf8_percent_encode(s, VALUE_ENCODE_SET).to_string()
}

fn join_encoded(values: &[String]) -> String {
    values
        .iter()
        .map(|v| encode_value(v))
        .collect::<Vec<_>>()
        .join(",")
}

fn derive_explore_host(rest_uri: &str) -> Result<String, ErrorData> {
    let swapped = rest_uri.replacen("://api.", "://app.", 1);
    if swapped == rest_uri {
        return Err(ErrorData::invalid_params(
            "could not derive Sift web host from `rest_uri` (no `api.` subdomain); pass \
             `explore_host` explicitly",
            None,
        ));
    }
    Ok(swapped.split('/').take(3).collect::<Vec<_>>().join("/"))
}

fn build_explore_url(rest_uri: &str, params: ExploreUrlParams) -> Result<String, ErrorData> {
    let ExploreUrlParams {
        assets,
        runs,
        channels,
        panel_type,
        start_time_unix_nanos,
        end_time_unix_nanos,
        explore_host,
    } = params;

    let no_selection = assets.as_ref().is_none_or(|v| v.is_empty())
        && runs.as_ref().is_none_or(|v| v.is_empty())
        && channels.as_ref().is_none_or(|v| v.is_empty())
        && panel_type.is_none()
        && start_time_unix_nanos.is_none()
        && end_time_unix_nanos.is_none();
    if no_selection {
        return Err(ErrorData::invalid_params(
            "at least one of `assets`, `runs`, `channels`, `panel_type`, \
             `start_time_unix_nanos`, or `end_time_unix_nanos` must be set",
            None,
        ));
    }

    if let (Some(start), Some(end)) = (start_time_unix_nanos, end_time_unix_nanos)
        && end < start
    {
        return Err(ErrorData::invalid_params(
            "`end_time_unix_nanos` must be greater than or equal to `start_time_unix_nanos`",
            None,
        ));
    }

    if let Some(ref p) = panel_type
        && !KNOWN_PANEL_TYPES.contains(&p.as_str())
    {
        return Err(ErrorData::invalid_params(
            format!(
                "unknown `panel_type` `{p}`; expected one of: {}",
                KNOWN_PANEL_TYPES.join(", ")
            ),
            None,
        ));
    }

    let host = match explore_host {
        Some(h) => h,
        None => derive_explore_host(rest_uri)?,
    };

    let mut query = String::from("method=single");
    if let Some(v) = assets.as_ref().filter(|v| !v.is_empty()) {
        query.push_str("&assets=");
        query.push_str(&join_encoded(v));
    }
    if let Some(v) = runs.as_ref().filter(|v| !v.is_empty()) {
        query.push_str("&runs=");
        query.push_str(&join_encoded(v));
    }
    if let Some(v) = channels.as_ref().filter(|v| !v.is_empty()) {
        query.push_str("&channels=");
        query.push_str(&join_encoded(v));
    }
    if let Some(p) = panel_type {
        query.push_str("&panelType=");
        query.push_str(&encode_value(&p));
    }
    if let Some(start) = start_time_unix_nanos {
        query.push_str("&startTime=");
        query.push_str(&encode_value(
            &chrono::DateTime::from_timestamp_nanos(start)
                .to_rfc3339_opts(chrono::SecondsFormat::Millis, true),
        ));
    }
    if let Some(end) = end_time_unix_nanos {
        query.push_str("&endTime=");
        query.push_str(&encode_value(
            &chrono::DateTime::from_timestamp_nanos(end)
                .to_rfc3339_opts(chrono::SecondsFormat::Millis, true),
        ));
    }

    Ok(format!("{host}/explore?{query}"))
}
