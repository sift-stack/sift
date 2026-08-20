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
    assets: Option<Vec<String>>,
    runs: Option<Vec<String>>,
    channels: Option<Vec<String>>,
    panel_type: Option<String>,
    start_time_unix_nanos: Option<i64>,
    end_time_unix_nanos: Option<i64>,
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
                Send it only for an asset-scoped request.
              - `runs`: optional list of run names or UUIDs. Same resolution rules as `assets`. A run already
                scopes the view to its own asset, so send `runs` alone for a run-scoped request.
              - `channels`: optional list of channel names, UUIDs, or prefixed forms. Axis prefixes (`L1:foo`,
                `L2:bar`) bind a channel to a Y-axis for multi-axis plots. Role prefixes (`x:foo`, `y:foo`,
                `color:foo` for scatter; `lat:foo`, `lon:foo`, `color:foo` for geo-map) bind a channel to a panel
                role. Prefer names returned by `list_assets` / `list_runs` / `list_channels` over guessing.
              - `panel_type`: optional. When omitted Explore defaults to `timeseries`. Unknown values are
                rejected with `INVALID_PARAMS`; the error message enumerates the accepted set.
              - `start_time_unix_nanos`, `end_time_unix_nanos`: optional time window. Provided as Unix nanoseconds
                for parity with `get_data`; the tool converts to ISO 8601 UTC for the URL.
            Errors:
              - `INVALID_PARAMS` if no selection or time parameter is set (the URL would be useless), if
                `panel_type` is not in the known set, or if `end_time_unix_nanos < start_time_unix_nanos`.

            Guidance:
              - Reach for this tool when the user asks to \"see\", \"view\", \"graph\", \"plot\", \"visualize\", or
                \"open\" data in Sift. Pair it with `get_data` only when the user also wants the data locally for
                SQL or further processing.
              - Send only the scope the user asked for: `runs` alone for a run, `assets` alone for an asset.
                Set both only when the user explicitly asked to see asset data and run data in the same view.
                Passing both otherwise opens a wider selection than the user requested.
              - The tool does not validate that the named asset/run/channel exists — Explore resolves at page
                load. Use names you have already retrieved from `list_*` tools to avoid 404s on click.
        ",
        annotations(title = "explore/explore_url", read_only_hint = true)
    )]
    pub async fn explore_url(&self, params: Parameters<ExploreUrlParams>) -> error::McpResult {
        let Parameters(ExploreUrlParams {
            assets,
            runs,
            channels,
            panel_type,
            start_time_unix_nanos,
            end_time_unix_nanos,
        }) = params;

        let url = self.url_service.build_explore_url(ExploreUrlRequest {
            assets,
            runs,
            channels,
            panel_type,
            start_time_unix_nanos,
            end_time_unix_nanos,
        })?;

        let next_step = format!(
            "Built Sift Explore URL: {url}\n\nRender this URL inline in your response as a \
             clickable markdown link so the user can open the view in Sift. Do not summarize \
             the link away."
        );

        let mut result = CallToolResult::structured(serde_json::json!({
            "url": url,
            "next_step": next_step,
        }));
        result.content = vec![ContentBlock::text(next_step)];
        Ok(result)
    }
}
