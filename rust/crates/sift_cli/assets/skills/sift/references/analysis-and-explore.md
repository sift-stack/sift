# Local data analysis and Sift Explore links

## Local data analysis

When the user wants numbers, summaries, or transformed data — anything where
the output is text or a new dataset — pull the source data locally with
`get_data` (writes a Parquet file) and run `sql` over it. Chain
`get_data` → `sql` for filtering, aggregation, or feature derivation. If the
result should land back in Sift as a new dataset, follow with
`upload_dataset`, and confirm the target asset/run with the user first. When
`upload_dataset` returns an `explore_url`, surface it to the user as plain
text, in full, so they can jump straight to the imported data. Do not wrap it
in a markdown link. If `explore_url` is null, do not invent a link.

## Visualizing in Sift Explore

When the user wants to see, view, graph, plot, or open data in Sift, build
a link with `explore_url` and surface the URL to the user as plain text, in
full. The URL is the deliverable — do not wrap it in a markdown link, do not
summarize it away. Pick the
`panel_type` that fits the request: `timeseries` (default), `histogram`,
`table`, `fft`, `metrics`, `scatter-plot`, or `geo-map`. Prefix channels
with `L1:` / `L2:` for multi-axis plots; with `x:` / `y:` / `color:` for
scatter; with `lat:` / `lon:` / `color:` for geo.

`explore_url` requires `app_uri` configured in the user's `sift-cli` profile,
or an `explore_host` passed per call. Without either it fails with
`INVALID_PARAMS`.

If the user wants both a chart and numbers, produce the `explore_url` link
and chain `get_data` + `sql` together.
