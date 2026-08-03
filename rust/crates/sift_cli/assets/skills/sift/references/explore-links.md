# Sift Explore links

Build the link with `explore_url`, then surface the URL to the user as plain
text, in full.

Pick the `panel_type` that fits the request: `timeseries` (the default),
`histogram`, `table`, `fft`, `metrics`, `scatter-plot`, or `geo-map`. The tool
rejects an unknown value with `INVALID_PARAMS` and names the accepted set in
the error.

Prefix the channels for the panel you chose:

- `L1:` and `L2:` for multi-axis plots.
- `x:`, `y:`, and `color:` for a scatter plot.
- `lat:`, `lon:`, and `color:` for a geo map.

`explore_url` uses the required `app_uri` from the user's `sift-cli` profile.
The `sift-cli mcp` command rejects a profile without this value.

If `app_uri` is missing, run `sift-cli agent doctor`. Relay the exact config
command for PubCloud or GovCloud. For any other domain, ask the user to copy the
URL origin from their Sift web app. Keep the scheme and host. Do not assume a
top-level domain. For the default profile, propose
`sift-cli config update --app-uri <SIFT_WEB_ORIGIN>`. Add `--profile <name>`
before `config` for a named profile. Wait for approval before you run it. Ask
the user to restart the MCP client before you retry `explore_url`.

When the user wants a chart and numbers together, build the link and also run
`get_data` then `sql`.
