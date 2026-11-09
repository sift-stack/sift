# Declarative charts

Use `create_declarative_chart` to validate a complete version 0 YAML or JSON spec and get a Sift link that opens it.
One chart is recommended: a spec with exactly one chart displays inline in Sift agent chat.
Specs with several charts or a top-level `layout` are allowed and still return a working Explore link.

A valid result includes `exploreUrl`. Render it once as a clickable markdown link with descriptive text, such as what the chart plots. Do not paste the spec into your reply.

## User supplies a spec

If the user pastes or attaches a complete spec, pass it to `create_declarative_chart` verbatim. When it returns `invalid_spec` with reported paths, fix each path (or ask the user when the fix needs their intent) and resend the full spec. Repeat until valid or 3 attempts are exhausted, then explain the remaining issues to the user.

## Plain-language request

1. Resolve runs with `list_runs` and channels with `list_channels`. Resolve calculated channels with `list_calculated_channels` and bind them with `calculatedChannelId`, not `channel`. Never invent IDs or channel names.
2. Read the cached schema as described in [Schema reference](#schema-reference). Never guess allowed keys or binding shapes.
3. Write a complete version 0 spec with `dataSources` and one `chart`. Prefer YAML. Use one chart unless the user asked for several or a layout.
4. Call `create_declarative_chart` with the full spec.
5. On `invalid_spec`, fix every reported path and resend the full corrected spec.
6. After 3 failed attempts, stop and summarize the remaining validation problems instead of retrying.
7. To edit a chart, resend the full edited spec. Each valid call returns a new `exploreUrl`.

## Minimal spec

Replace the illustrative ID and channel with values from Sift MCP.

```yaml
version: 0
dataSources:
  flight: { run: { id: 7f3a2b10-0000-4000-8000-000000000001 } }
chart:
  type: timeseries
  series: [{ channel: pressure, source: flight, axis: L1 }]
```

## Checklist

- [ ] IDs and channel names came from Sift MCP results.
- [ ] The complete version 0 spec has one chart unless the user asked for several or a `layout`.
- [ ] Each channel series has exactly one of `channel`, `channelId`, `calculatedChannelId`, or `match`.
- [ ] Encoding series have every required coordinate role.
- [ ] Asset-only charts include an explicit `timeInterval`.
- [ ] Every reported path is fixed before resending the full spec.
- [ ] After 3 failed attempts, explain the remaining problems instead of retrying.
- [ ] Optional `sampling`, `maxGap`, and axis `min`/`max` are omitted unless the user asked for them.

## Schema reference

The MCP keeps a machine-local schema cache, separated by deployment. At startup it reuses a valid file less than 24 hours old; a missing, invalid, or older copy is refreshed in the background. Other tools remain available if the refresh fails, but the schema tool reports the failure rather than silently using a stale copy. There is no periodic download while the MCP is running.

1. Call `get_chart_schema` with no parameters once per agent session. It returns the absolute `schema_path`, `root` (top-level constraints), and `definitions` (exact definition names with summaries).
2. When your host shares the MCP server's filesystem, reuse `schema_path` as a read-only reference. Use a JSON parser to read the root structure and only the `$defs` entries needed for the spec; do not load the entire schema into context or download it again for each chart. The file persists after the MCP exits and can be refreshed by another session.
3. If local-file access or JSON extraction is unavailable, call `get_chart_schema` with exact definition names from the index (e.g. `["timeseriesChart"]`). These calls read the same local cache rather than downloading it per call. Follow only the `references` needed for the spec, not the entire closure.

For an explicitly requested separate download, run `sift-cli get chart-schema -o /absolute/path/chart-schema.json` with the selected `--profile <name>` when applicable. That standalone command fetches afresh on every invocation; it is not needed for normal MCP authoring.

Validation errors from `create_declarative_chart` are the final word on allowed keys and constraints.

## Single chart vs multi-chart specs

`chart` and `charts` are mutually exclusive. `layout` requires `charts`.

- **Single chart** (`chart`): the spec contains one chart with no layout. A single chart displays inline in Sift agent chat. No top-level `layout` is allowed.
- **Multi-chart** (`charts`): an object keyed by aliases, each alias mapping to a chart object. Use `layout` to arrange them. `charts` and `layout` together return a working Explore link.

Prefer `chart` unless the user asked for several charts or a layout.

## Run and inline expression example

```yaml
version: 0
dataSources:
  flight: { run: { id: 7f3a2b10-0000-4000-8000-000000000001 } }
transforms:
  power:
    expr: '$1 * $2'
    inputs:
      $1: { channel: voltage }
      $2: { channel: current }
    unit: W
chart:
  type: timeseries
  title: Electrical power
  sources: [flight]
  timeMode: absolute
  yAxis:
    L1: { label: Power, scale: value }
  series:
    - channel: power
      source: flight
      axis: L1
      style: { line: { color: ruby } }
```

## Asset and explicit interval example

```yaml
version: 0
dataSources:
  pad: { asset: { id: 5d10c4a0-0000-4000-8000-000000000002 } }
timeInterval:
  start: '2024-06-01T00:00:00Z'
  end: '2024-06-01T00:10:00Z'
chart:
  type: timeseries
  title: Pad temperature
  timeMode: absolute
  series:
    - match: { name: { matches: '^temp_.*' } }
      source: pad
      axis: R1
  yAxis:
    R1: { label: Temperature, scale: value }
```

## Calculated channel series example

```yaml
version: 0
dataSources:
  flight: { run: { id: 7f3a2b10-0000-4000-8000-000000000001 } }
chart:
  type: timeseries
  series:
    - calculatedChannelId: 1a2b3c4d-0000-4000-8000-000000000002
      source: flight
      axis: L1
      label: Filtered pressure
```

## Multi-chart layout example

```yaml
version: 0
dataSources:
  flight: { run: { id: 7f3a2b10-0000-4000-8000-000000000001 } }
charts:
  pressure:
    type: timeseries
    series: [{ channel: pressure, source: flight, axis: L1 }]
  correlation:
    type: scatter-plot
    series:
      - encoding:
          xCoordinate: { channel: temperature }
          yCoordinate: { channel: pressure }
        source: flight
layout:
  rows:
    - [pressure, correlation]
```
