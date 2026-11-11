# Declarative charts

Author version 0 YAML or JSON using Sift's existing declarative schema, then
deliver it with `create_declarative_chart`. The same spec draws in Explore,
Canvas, and Sift Edge Desktop. It does not promise an inline chart in the
calling chat client. For a simple read-only deep-link, use `explore_url`.

## Choose the surface

Pass `surface` to `create_declarative_chart`. Use the surface the user names,
and Explore otherwise.

| User wants the chart | `surface` | Result |
|---|---|---|
| In the Sift web app, or did not say | `explore` (default) | A saved Explore share link. |
| In a Canvas | `canvas` | Steps to paste the spec into a Canvas chart cell. Nothing is saved in Sift. |
| In Sift Edge Desktop | `edge` | Steps to load the spec as a `.yaml` file in Sift Edge Desktop. Nothing is saved in Sift. |

On every surface, `output` also writes the spec to a local file, exactly as
sent, when it is delivered. Use an absolute `.yaml` path. Always pass it for
`edge`, which loads the spec from a file. For the other surfaces, pass it when
the user asks for a file; otherwise `next_step` says how to offer one.

## Workflow

1. Resolve real assets, runs, and channels with discovery tools. Never invent
   IDs, channel names, types, or a time range. Stop on empty discovery results.
2. Call `get_chart_schema` once and keep its `schema_path`. Read the local schema
   as described below; reuse the file for subsequent charts.
3. Build a minimal spec with `version: 0`, `dataSources`, and either `chart`
   or keyed `charts`. Follow the schema for the chosen panel's series, axes,
   encodings, and settings. Do not copy guessed options from another chart type.
4. Keep the session's selected deployment. Saving a chart share link is a
   read-only exception: it changes no telemetry or other Sift resources and
   needs neither create access nor a separate confirmation.
5. Pass the spec's **contents** as the flat `spec` string to
   `create_declarative_chart`, not a file path, with the `surface` you chose.
   It sends the spec to Sift's validate endpoint for that surface before it
   delivers anything. It does not migrate old versions.
6. On `invalid_spec`, fix the indicated fields using the local schema and retry
   at most three times. Do not repair the spec for an authentication, permission,
   endpoint, or network failure; surface that failure instead.
7. On `reason: warnings`, nothing was delivered: the surface will not draw the
   listed parts as written. Follow `next_step` to fix and resend. Before you
   remove something the user asked for, ask whether to keep it for support that
   is coming; `accept_warnings: true` keeps it. The warnings are the source of
   truth for what each surface draws.
8. Follow `next_step` from the delivered result. It says how to present the
   chart on the surface. For numbers too, use `get_data` and `sql` separately;
   plotting does not require a local telemetry download.

## Schema reference

`get_chart_schema` returns a machine-local file for the selected deployment.
MCP warms the cache in the background and refreshes an absent, invalid, or
24-hour-old copy when needed. Schema retrieval, including startup warming,
is essential traffic. It does not download a new copy for every chart.

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

Sift's validate endpoint describes the runtime limits of each surface. They
appear as `warnings` in the `create_declarative_chart` result. Rely on those
warnings, not on a fixed list.
