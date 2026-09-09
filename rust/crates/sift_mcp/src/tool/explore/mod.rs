use rmcp::{
    handler::server::wrapper::Parameters,
    model::{CallToolResult, ContentBlock},
    schemars::{self, JsonSchema},
    tool, tool_router,
};
use serde::Deserialize;

use crate::{error, server::SiftMcpServer, service::url::ExploreUrlRequest};

#[cfg(test)]
mod test;

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
        name = "explore_url",
        description = "
            Build a Sift Explore deep-link URL for the given asset/run/channel selection. Pure URL construction — no
            API call is made. Hand the returned URL to the user verbatim, rendered inline as a clickable link, so
            they can open the view in the Sift web app.

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
              - Reach for this tool when the user asks to \"see\", \"view\", \"graph\", \"plot\", \"visualize\", or
                \"open\" data in Sift. Pair it with `get_data` only when the user also wants the data locally for
                SQL or further processing.
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
             clickable markdown link so the user can open the view in Sift. Do not summarize \
             the link away."
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
}

/// Explore URLs carry ISO 8601 timestamps with millisecond precision, so the URL service drops
/// the sub-millisecond part of each bound. Mirror that here so the tool can echo the bounds the
/// URL actually encodes. Floors toward negative infinity, matching chrono's formatting of the
/// sub-second field.
fn truncate_to_millis(unix_nanos: i64) -> i64 {
    const NANOS_PER_MILLI: i64 = 1_000_000;
    unix_nanos - unix_nanos.rem_euclid(NANOS_PER_MILLI)
}
