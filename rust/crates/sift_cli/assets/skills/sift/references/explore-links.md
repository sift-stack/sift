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

`explore_url` needs `app_uri` in the user's `sift-cli` profile, or an
`explore_host` passed on the call. Without either it fails with
`INVALID_PARAMS`.

If `app_uri` is missing, run `sift-cli agent doctor`. For a production or
government profile, doctor gives a safe `--fix` command. Show that command and
wait for approval before you run it. For any other domain, ask the user to open
their Sift web app. Ask them to copy the URL origin from the browser address
bar. Keep the scheme and host. Do not assume a top-level domain. Propose the
profile-specific `sift-cli config update --app-uri <SIFT_WEB_ORIGIN>` command.
Wait for approval before you run it. Ask the user to restart the MCP client
before you retry `explore_url`.

When the user wants a chart and numbers together, build the link and also run
`get_data` then `sql`.
