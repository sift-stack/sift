use rmcp::{
    ErrorData,
    handler::server::wrapper::Parameters,
    model::{CallToolResult, Content},
    schemars::{self, JsonSchema},
    tool, tool_router,
};
use serde::Deserialize;

use crate::{error, server::SiftMcpServer};

#[cfg(test)]
mod test;

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

#[tool_router(router = explore_router, vis = "pub(crate)")]
impl SiftMcpServer {
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
        annotations(title = "explore_router/explore_url", read_only_hint = true)
    )]
    pub async fn explore_url(&self, params: Parameters<ExploreUrlParams>) -> error::McpResult {
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
