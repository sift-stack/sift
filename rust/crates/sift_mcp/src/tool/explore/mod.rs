use rmcp::{
    handler::server::wrapper::Parameters,
    model::{CallToolResult, ContentBlock},
    schemars::{self, JsonSchema},
    tool, tool_router,
};
use serde::Deserialize;
use serde_json::{Value, json};

use crate::{
    error,
    server::SiftMcpServer,
    service::{
        declarative::{Issue, ShareLinkResult},
        url::ExploreUrlRequest,
    },
};

#[cfg(test)]
mod test;

/// Failed attempts the agent gets to repair a spec before it must report the problem to the user.
const MAX_REPAIR_ATTEMPTS: u32 = 3;

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
            Validate a declarative chart spec (YAML or JSON) against the Sift schema, migrating it to the latest
            version first, then create a shareable Sift Explore link for it. A valid result hands back the
            migrated spec and the link.

            Output:
              - Valid: `{ \"valid\": true, \"spec\": \"<migrated spec>\", \"shortLink\", \"exploreUrl\",
                \"warnings\": [{\"path\", \"message\"}], \"sourceVersion\", \"targetVersion\", \"next_step\" }`.
                `exploreUrl` is the link that opens the chart in Sift.
              - Invalid: `{ \"valid\": false, \"reason\": \"invalid_spec\", \"issues\": [{\"path\", \"message\"}],
                \"next_step\" }`. `path` locates the problem in the spec; `/` means the whole spec.
              - Unavailable: `{ \"valid\": false, \"reason\": \"service_unavailable\", \"issues\": [...],
                \"next_step\" }` when the validation or link service could not be reached.

            Parameters:
              - `spec`: the complete spec as a YAML or JSON string. The Sift backend decides what is valid; the
                tool does not pre-check the spec, so multi-chart specs and a top-level `layout` are accepted.

            Errors:
              - A bad spec is not an MCP error. It returns `valid: false` with `issues` so you can fix and retry.
              - `INVALID_PARAMS` is returned only when `spec` is missing or not a string.

            Guidance:
              - Resolve run IDs and channel names with `list_runs`, `list_channels`, and
                `list_calculated_channels` before writing the spec. Never invent them.
              - Bind a calculated channel with `calculatedChannelId`, not by name.
              - A spec with exactly one chart displays inline in Sift agent chat. Specs with several charts or a
                `layout` still return a working Explore link.
              - Prefer YAML. JSON is accepted.
              - On `invalid_spec`, fix every listed path and resend the complete spec. After 3 failed attempts,
                stop and summarize the problem to the user instead of retrying.
              - Omit `sampling`, `maxGap`, and axis `min`/`max` unless the user asked for them.
              - On a valid result, render `exploreUrl` once as a clickable markdown link with descriptive text, such
                as what the chart plots. Do not paste the spec into your reply.
        ",
        annotations(
            title = "explore/create_declarative_chart",
            read_only_hint = false,
            destructive_hint = false,
            idempotent_hint = false
        )
    )]
    pub async fn create_declarative_chart(
        &self,
        params: Parameters<CreateDeclarativeChartParams>,
    ) -> error::McpResult {
        let Parameters(CreateDeclarativeChartParams { spec }) = params;

        let migration = match self.declarative_service.migrate(&spec).await {
            Ok(migration) => migration,
            Err(error) => return Ok(service_unavailable_result(&error)),
        };
        if !migration.valid || !migration.errors.is_empty() {
            return Ok(invalid_spec_result(issues_or(
                &migration.errors,
                "the spec could not be migrated",
            )));
        }
        let migrated_spec = if migration.migrated_spec.is_empty() {
            spec
        } else {
            migration.migrated_spec
        };

        let validation = match self.declarative_service.validate(&migrated_spec).await {
            Ok(validation) => validation,
            Err(error) => return Ok(service_unavailable_result(&error)),
        };
        if !validation.valid || !validation.errors.is_empty() {
            return Ok(invalid_spec_result(issues_or(
                &validation.errors,
                "the spec failed validation",
            )));
        }

        let short_link = match self
            .declarative_service
            .create_share_link(&migrated_spec)
            .await
        {
            Ok(ShareLinkResult::Ok { short_link }) => short_link,
            Ok(ShareLinkResult::Invalid { message }) => {
                return Ok(invalid_spec_result(vec![issue("/", &message)]));
            }
            Err(error) => return Ok(service_unavailable_result(&error)),
        };
        let explore_url = self.url_service.build_share_url(&short_link)?;

        let mut warnings = issues_json(&migration.warnings);
        warnings.extend(issues_json(&validation.warnings));

        let next_step = format!(
            "The chart spec validated and a Sift link was created: {explore_url}\n\nRender this URL \
             in your response as a clickable markdown link with descriptive text naming what the \
             chart plots, never the word \"link\" or the bare URL. Mention it once. Do not paste \
             the spec into your response."
        );
        let mut result = CallToolResult::structured(json!({
            "valid": true,
            "spec": migrated_spec,
            "shortLink": short_link,
            "exploreUrl": explore_url,
            "warnings": warnings,
            "sourceVersion": migration.source_version,
            "targetVersion": migration.target_version,
            "next_step": next_step,
        }));
        result.content = vec![ContentBlock::text(next_step)];
        Ok(result)
    }
}

fn issue(path: &str, message: &str) -> Value {
    json!({ "path": path, "message": message })
}

fn issues_json(issues: &[Issue]) -> Vec<Value> {
    issues
        .iter()
        .map(|found| {
            let path = if found.path.is_empty() {
                "/"
            } else {
                found.path.as_str()
            };
            issue(path, &found.message)
        })
        .collect()
}

/// The service's issues, or one whole-spec issue carrying `fallback` when it listed none.
fn issues_or(issues: &[Issue], fallback: &str) -> Vec<Value> {
    if issues.is_empty() {
        vec![issue("/", fallback)]
    } else {
        issues_json(issues)
    }
}

/// A non-error result the client can act on: `reason`, the `issues`, and a `next_step` that is
/// also the text content.
fn failure_result(reason: &str, issues: Vec<Value>, next_step: String) -> CallToolResult {
    let listing = issues
        .iter()
        .map(|found| {
            let path = found["path"].as_str().unwrap_or("/");
            let message = found["message"].as_str().unwrap_or_default();
            format!("{path}: {message}")
        })
        .collect::<Vec<_>>()
        .join("\n");
    let mut result = CallToolResult::structured(json!({
        "valid": false,
        "reason": reason,
        "issues": issues,
        "next_step": next_step,
    }));
    result.content = vec![ContentBlock::text(format!("{listing}\n\n{next_step}"))];
    result
}

fn invalid_spec_result(issues: Vec<Value>) -> CallToolResult {
    failure_result(
        "invalid_spec",
        issues,
        format!(
            "The spec is not valid yet. Fix every listed path and resend the complete spec. After \
             {MAX_REPAIR_ATTEMPTS} failed attempts, stop retrying and summarize the problem to the user."
        ),
    )
}

// TODO: Non-retryable 4xx responses (401, 403, 404: bad API key, missing permission, or a backend
// without the declarative endpoint) also land here and tell the agent to retry. Tag them in
// DeclarativeService with a typed error and map them to a no-retry, check-credentials result.
fn service_unavailable_result(error: &anyhow::Error) -> CallToolResult {
    failure_result(
        "service_unavailable",
        vec![issue("/", &format!("{error:#}"))],
        "The spec could not be checked because the validation service failed, which is not a problem \
         with the spec. Retry once; if it fails again, tell the user the chart could not be validated."
            .to_string(),
    )
}

/// Explore URLs carry ISO 8601 timestamps with millisecond precision, so the URL service drops
/// the sub-millisecond part of each bound. Mirror that here so the tool can echo the bounds the
/// URL actually encodes. Floors toward negative infinity, matching chrono's formatting of the
/// sub-second field.
fn truncate_to_millis(unix_nanos: i64) -> i64 {
    const NANOS_PER_MILLI: i64 = 1_000_000;
    unix_nanos - unix_nanos.rem_euclid(NANOS_PER_MILLI)
}
