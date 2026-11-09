# Declarative charts

Author version 0 YAML or JSON using Sift's existing declarative schema, then
create an Explore share link with `create_declarative_chart`. This saves a
spec-backed view, not an image or artifact. It does not promise an inline chart
in the calling chat client. For a simple read-only deep-link, use `explore_url`.

## Workflow

1. Resolve real assets, runs, and channels with discovery tools. Never invent
   IDs, channel names, types, or a time range. Stop on empty discovery results.
2. Call `get_chart_schema` once and keep its `schema_path`. Read the local schema
   as described below; reuse the file for subsequent charts.
3. Build a minimal spec with `version: 0`, `dataSources`, and either `chart`
   or keyed `charts`. Follow the schema for the chosen panel's series, axes,
   encodings, and settings. Do not copy guessed options from another chart type.
4. Show the user the proposed view and target deployment, and obtain approval
   before saving the share link. Creation needs create access; do not widen
   access or bypass a blocked MCP tool through REST, a script, or an artifact.
5. Pass the spec's **contents** as the flat `spec` string to
   `create_declarative_chart`, not a file path. The backend parses and strictly
   validates the spec before saving it; this tool does not migrate old versions.
6. On `invalid_spec`, fix the indicated fields using the local schema and retry
   at most three times. Do not repair the spec for an authentication, permission,
   endpoint, or network failure; surface that failure instead.
7. On success, present the returned `exploreUrl` once as a descriptive Markdown
   link. Do not paste the spec unless asked. For numbers too, use `get_data`
   and `sql` separately; plotting does not require a local telemetry download.

## Schema reference

`get_chart_schema` returns a machine-local file for the selected deployment.
MCP warms the cache in the background and refreshes an absent, invalid, or
24-hour-old copy when needed. It does not download a new copy for every chart.

Read the document root first, then follow its `$ref` entries into `$defs` for
the chosen panel and its nested settings. Read only those portions, not the
entire schema into chat context. For example, if Python is available:

```sh
# Use schema_path from the tool. Add definition names after it on later reads.
python - "$schema_path" <<'PY'
import json, sys
with open(sys.argv[1]) as f:
    schema = json.load(f)
names = sys.argv[2:]
selected = ({n: schema["$defs"][n] for n in names} if names else
            {k: v for k, v in schema.items() if k != "$defs"})
print(json.dumps(selected, indent=2))
PY
```

For a separate explicit fresh export, use `sift-cli get chart-schema --output
./chart-schema.json`; for a named profile, put `--profile <same-profile>` before
`get`. Keep the session's deployment; never fall back to another profile.

## Minimal shape

These placeholders illustrate structure only; replace them with verified data.
An asset source uses `asset: {id: ...}` instead of `run: {id: ...}` and needs an
explicit `timeInterval`. Put `title` inside the chart, not at document level.

```yaml
version: 0
dataSources:
  flight:
    run: {id: <verified-run-uuid>}
chart:
  type: timeseries
  title: Engine temperature
  sources: [flight]
  series:
    - channel: <verified-channel-name>
      source: flight
```

For multiple panels, replace `chart` with keyed `charts` and add `layout`, using
its schema for chart references and grid settings. Do not invent an array of
panels or combine `chart` and `charts`.

## Runtime limits

Schema-valid does not always mean an Explore setting is implemented. Prefer
explicit discovered sources and absolute timestamps. Do not use inline data
sources or inline query objects on a series: Explore does not resolve them.
For relative timeseries, run/timestamp anchors and default annotation-name
anchors (`first`, `last`, or an integer occurrence) work. Avoid `every`,
annotation overrides, family anchors, and lists of anchors per source; these
fall back to run starts rather than producing the requested alignment.
