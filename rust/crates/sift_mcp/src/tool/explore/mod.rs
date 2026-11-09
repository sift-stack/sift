use rmcp::{
    ErrorData,
    handler::server::wrapper::Parameters,
    model::{CallToolResult, ContentBlock},
    schemars::{self, JsonSchema},
    tool, tool_router,
};
use serde::Deserialize;
use serde_json::json;

use crate::{
    error,
    server::SiftMcpServer,
    service::{declarative::HttpError, url::ExploreUrlRequest},
};

#[cfg(test)]
mod test;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct CreateDeclarativeChartParams {
    spec: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct ExploreUrlParams {
    asset_ids: Option<Vec<String>>,
    run_ids: Option<Vec<String>>,
    channels: Option<Vec<String>>,
    panel_type: Option<String>,
    start_time_unix_nanos: Option<i64>,
    end_time_unix_nanos: Option<i64>,
    include_assets_and_runs: Option<bool>,
}

#[tool_router(router = explore_router, vis = "pub(crate)")]
impl SiftMcpServer {
    #[tool(
        name = "get_chart_schema",
        description = "
            Return the path of the declarative chart JSON Schema, cached as a local file for this
            deployment, so you can write a spec for `create_declarative_chart`. Takes no parameters.

            Output:
              - `{ \"schema_path\": \"<absolute path>\", \"schema_version\": 0, \"next_step\": \"...\" }`.
                The schema itself is never returned. It is about 95 KB, so read only what you need.

            Errors:
              - An error if the deployment does not serve the schema, cannot be reached, or the server
                has no REST endpoint.

            Guidance:
              - Call once per session and reuse `schema_path`. The file persists across restarts and is
                downloaded again by the server when it is missing, invalid, or 24 hours old.
              - Read the file with your file tools or a JSON parser: the root `properties` and
                `required` first, then only the `$defs` entries your chart uses.
        ",
        annotations(title = "explore/get_chart_schema", read_only_hint = true)
    )]
    pub async fn get_chart_schema(&self) -> error::McpResult {
        let path = self
            .declarative_service
            .chart_schema_path()
            .await
            .map_err(error::from_anyhow)?;
        let schema_path = path.display().to_string();
        let next_step = format!(
            "The chart schema is at {schema_path}. Read only the parts you need: the root \
             `properties` and `required`, then the `$defs` entries your chart uses. Do not load the \
             whole file. Then call `create_declarative_chart` with the spec."
        );
        let mut result = CallToolResult::structured(json!({
            "schema_path": schema_path,
            "schema_version": 0,
            "next_step": next_step,
        }));
        result.content = vec![ContentBlock::text(next_step)];
        Ok(result)
    }

    #[tool(
        name = "explore_url",
        description = "
            Build a quick Sift Explore deep-link URL that opens specific assets, runs, or channels. Pure URL
            construction — no API call is made. Hand the returned URL to the user verbatim, rendered inline as a
            clickable link, so they can open the view in the Sift web app. To plot, chart, graph, or visualize
            data, call `create_declarative_chart` instead.

            Output:
              - `{ \"url\": \"<url>\", \"next_step\": \"...\", \"start_time_unix_nanos\": <i64>?,
                \"end_time_unix_nanos\": <i64>? }`. `url` is the fully-formed Explore link; `next_step` instructs
                you on how to surface it. The two time fields are present only when the matching parameter was
                given and hold that bound truncated to the whole millisecond, exactly as the URL encodes it.

            Parameters:
              - `asset_ids`: optional list of asset IDs returned by `list_assets`.
              - `run_ids`: optional list of run IDs returned by `list_runs`.
              - `include_assets_and_runs`: optional, defaults to false. A link carries one source type by
                default: pass `run_ids` when the request names a run, `asset_ids` otherwise. Setting both
                `asset_ids` and `run_ids` without this flag is rejected. Set it to true only when the user
                explicitly asked to see runs and assets together in one view. Ignored unless both are set.
              - `channels`: optional list of channel names, UUIDs, or prefixed forms. Axis prefixes (`L1:foo`,
                `L2:bar`) bind a channel to a Y-axis for multi-axis plots. Role prefixes (`x:foo`, `y:foo`,
                `color:foo` for scatter; `lat:foo`, `lon:foo`, `color:foo` for geo-map) bind a channel to a panel
                role. Prefer names returned by `list_assets` / `list_runs` / `list_channels` over guessing.
              - `panel_type`: optional. When omitted Explore defaults to `timeseries`. Unknown values are
                rejected with `INVALID_PARAMS`; the error message enumerates the accepted set.
              - `start_time_unix_nanos`, `end_time_unix_nanos`: optional time window in Unix nanoseconds, for
                parity with `get_data`. Explore URLs serialize time as ISO 8601 UTC with millisecond precision, so
                the tool truncates each bound to the whole millisecond (`1788462000333333333` becomes
                `2026-09-03T19:00:00.333Z`). Truncate to whole milliseconds yourself before calling: a
                millisecond-aligned nanosecond count survives JSON number conversion exactly, and the value you
                pass then matches the URL. The result echoes the truncated bounds as `start_time_unix_nanos` /
                `end_time_unix_nanos`; when you report the link's time window to the user, report those echoed
                values, not more precise intermediate arithmetic. You may show the exact arithmetic separately.
            Errors:
              - `INVALID_PARAMS` if no selection or time parameter is set (the URL would be useless), if both
                `asset_ids` and `run_ids` are set without `include_assets_and_runs`, if `panel_type` is not in the
                known set, or if `end_time_unix_nanos < start_time_unix_nanos`.

            Guidance:
              - Reach for this tool when the user asks to \"open\" or \"link to\" specific assets, runs, or channels
                in Sift. Requests to \"plot\", \"chart\", \"graph\", or \"visualize\" data belong to
                `create_declarative_chart`. Pair this tool with `get_data` only when the user also wants the data
                locally for SQL or further processing.
              - Do not add the asset alongside a run to be thorough. A run is already scoped to its asset, so
                the asset adds a second, wider source to the view. Send both only on an explicit request for
                both.
              - The tool does not validate that the provided asset/run/channel exists — Explore resolves at page
                load. Use IDs and channel names you have already retrieved from `list_*` tools to avoid 404s.
        ",
        annotations(title = "explore/explore_url", read_only_hint = true)
    )]
    pub async fn explore_url(&self, params: Parameters<ExploreUrlParams>) -> error::McpResult {
        let Parameters(ExploreUrlParams {
            asset_ids,
            run_ids,
            channels,
            panel_type,
            start_time_unix_nanos,
            end_time_unix_nanos,
            include_assets_and_runs,
        }) = params;

        let url = self.url_service.build_explore_url(ExploreUrlRequest {
            asset_ids,
            run_ids,
            channels,
            panel_type,
            start_time_unix_nanos,
            end_time_unix_nanos,
            include_assets_and_runs: include_assets_and_runs.unwrap_or(false),
        })?;

        let start_time_unix_nanos = start_time_unix_nanos.map(truncate_to_millis);
        let end_time_unix_nanos = end_time_unix_nanos.map(truncate_to_millis);

        let mut next_step = format!(
            "Built Sift Explore URL: {url}\n\nRender this URL inline in your response as a \
             clickable markdown link so the user can open the view in Sift. Use descriptive \
             link text that names what the view shows, such as the asset or run and the \
             channels, never the word \"link\" or the bare URL. Do not summarize the link away."
        );
        if start_time_unix_nanos.is_some() || end_time_unix_nanos.is_some() {
            next_step.push_str(
                "\n\nExplore URLs keep whole milliseconds only. The link's time bounds, truncated to the \
                 millisecond, are:",
            );
            if let Some(start) = start_time_unix_nanos {
                next_step.push_str(&format!(" start_time_unix_nanos={start}"));
            }
            if let Some(end) = end_time_unix_nanos {
                next_step.push_str(&format!(" end_time_unix_nanos={end}"));
            }
            next_step.push_str(
                ". When you report the link's time window to the user, report these values as the \
                 bounds; show any more precise arithmetic separately.",
            );
        }

        let mut structured = serde_json::json!({
            "url": url,
            "next_step": next_step,
        });
        if let Some(start) = start_time_unix_nanos {
            structured["start_time_unix_nanos"] = serde_json::json!(start);
        }
        if let Some(end) = end_time_unix_nanos {
            structured["end_time_unix_nanos"] = serde_json::json!(end);
        }

        let mut result = CallToolResult::structured(structured);
        result.content = vec![ContentBlock::text(next_step)];
        Ok(result)
    }

    #[tool(
        name = "create_declarative_chart",
        description = "
            Create a shareable Sift Explore link from a declarative chart spec (YAML or JSON). Sift
            validates the spec when it creates the link. This is a read-only policy exception:
            it saves only a content-addressed view, not telemetry or other Sift resources. It is
            available by default, with no create-access flag or confirmation required.

            Output:
              - Valid: `{ \"valid\": true, \"shortLink\", \"exploreUrl\", \"next_step\" }`.
                `exploreUrl` opens the chart in Sift.
              - Invalid: `{ \"valid\": false, \"reason\": \"invalid_spec\", \"issues\": [{\"message\"}],
                \"next_step\" }`. Sift rejected the spec; `issues` says why.

            Parameters:
              - `spec`: the complete spec as a YAML or JSON string, sent to Sift as is. Read the schema
                with `get_chart_schema` before writing one.

            Errors:
              - A rejected spec is not an MCP error. It returns `valid: false` so you can fix and resend.
              - An error that says to stop when Sift rejects the credentials or permissions, or does not
                serve the endpoint. Do not retry; tell the user to check their configuration.
              - Any other error is a server or network failure, not a problem with the spec.

            Guidance:
              - Resolve run IDs and channel names with `list_runs`, `list_channels`, and
                `list_calculated_channels` before writing the spec. Never invent them.
              - Bind a calculated channel with `calculatedChannelId`, not by name.
              - Prefer YAML. Omit `sampling`, `maxGap`, and axis `min`/`max` unless the user asked for them.
              - On `invalid_spec`, fix every problem listed and resend the complete spec. After 3 failed
                attempts, stop and summarize the problem to the user.
              - On a valid result, render `exploreUrl` once as a clickable markdown link with descriptive
                text, such as what the chart plots. Do not paste the spec into your reply.
        ",
        annotations(
            title = "explore/create_declarative_chart",
            read_only_hint = true,
            destructive_hint = false,
            idempotent_hint = true
        )
    )]
    pub async fn create_declarative_chart(
        &self,
        params: Parameters<CreateDeclarativeChartParams>,
    ) -> error::McpResult {
        // Check presentation config before persisting; the backend receives only the spec.
        self.url_service.build_share_url("")?;
        let Parameters(CreateDeclarativeChartParams { spec }) = params;

        let short_link = match self.declarative_service.create_share_link(&spec).await {
            Ok(short_link) => short_link,
            Err(error) => return share_link_failure(error),
        };
        let explore_url = self.url_service.build_share_url(&short_link)?;

        let next_step = format!(
            "The chart spec was accepted and a Sift link was created: {explore_url}\n\nRender this URL \
             in your response as a clickable markdown link with descriptive text naming what the \
             chart plots, never the word \"link\" or the bare URL. Mention it once. Do not paste \
             the spec into your response."
        );
        let mut result = CallToolResult::structured(json!({
            "valid": true,
            "shortLink": short_link,
            "exploreUrl": explore_url,
            "next_step": next_step,
        }));
        result.content = vec![ContentBlock::text(next_step)];
        Ok(result)
    }
}

/// Routes a failed share-link call. HTTP 400 means Sift rejected the spec, so the agent may repair
/// it. 401, 403, and 404 will not change on retry. Everything else carries no repair advice.
fn share_link_failure(error: anyhow::Error) -> error::McpResult {
    let Some(http) = error.downcast_ref::<HttpError>() else {
        return Err(error::from_anyhow(error));
    };
    match http.status.as_u16() {
        400 => {
            let message = if http.message.is_empty() {
                "Sift rejected the spec"
            } else {
                http.message.as_str()
            };
            let next_step = "The spec is not valid. Fix the problem in `issues` and resend the complete \
                spec. After 3 failed attempts, stop retrying and summarize the problem to the user.";
            let mut result = CallToolResult::structured(json!({
                "valid": false,
                "reason": "invalid_spec",
                "issues": [{ "message": message }],
                "next_step": next_step,
            }));
            result.content = vec![ContentBlock::text(format!("{message}\n\n{next_step}"))];
            Ok(result)
        }
        401 | 403 | 404 => {
            let guidance = format!(
                "Sift answered HTTP {}: {}. Do not retry and do not change the spec. Stop and tell the \
                 user to check their API key, their access to this deployment, and that it \
                 supports declarative charts.",
                http.status, http.message
            );
            Err(ErrorData::internal_error(
                guidance.clone(),
                Some(
                    json!({ "status": "stopped", "reason": http.status.as_u16(), "guidance": guidance }),
                ),
            ))
        }
        _ => Err(error::from_anyhow(error)),
    }
}

/// Explore URLs carry ISO 8601 timestamps with millisecond precision, so the URL service drops
/// the sub-millisecond part of each bound. Mirror that here so the tool can echo the bounds the
/// URL actually encodes. Floors toward negative infinity, matching chrono's formatting of the
/// sub-second field.
fn truncate_to_millis(unix_nanos: i64) -> i64 {
    const NANOS_PER_MILLI: i64 = 1_000_000;
    unix_nanos - unix_nanos.rem_euclid(NANOS_PER_MILLI)
}
