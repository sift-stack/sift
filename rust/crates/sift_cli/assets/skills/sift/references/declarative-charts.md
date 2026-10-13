# Declarative charts

Use `create_declarative_chart` to validate a complete version 0 YAML or JSON spec and get a Sift link that opens it.
One chart is recommended: a spec with exactly one chart displays inline in Sift agent chat.
Specs with several charts or a top-level `layout` are allowed and still return a working Explore link.

A valid result includes `exploreUrl`. Render it once as a clickable markdown link with descriptive text, such as what the chart plots. Do not paste the spec into your reply.

## User supplies a spec

If the user pastes or attaches a complete spec, pass it to `create_declarative_chart` verbatim. When it returns `invalid_spec` with reported paths, fix each path (or ask the user when the fix needs their intent) and resend the full spec. Repeat until valid or 3 attempts are exhausted, then explain the remaining issues to the user.

## Plain-language request

1. Resolve runs with Sift MCP `list_runs` and channels with `list_channels`. Resolve calculated channels with `list_calculated_channels` and bind them with `calculatedChannelId`, not `channel`. Never invent IDs or channel names.
2. Write a complete version 0 spec with `dataSources` and one `chart`. Prefer YAML. Use one chart unless the user asked for several.
3. Call `create_declarative_chart` with the full spec.
4. On `invalid_spec`, fix every reported path and resend the full corrected spec.
5. After 3 failed attempts, stop and summarize the remaining validation problems instead of retrying.
6. To edit a chart, resend the full edited spec. Each valid call returns a new `exploreUrl`.

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

Validation errors are the source of truth for allowed keys and constraints. When keys fail validation, read the reported paths and messages to understand what is allowed. The tool accepts `version: 0` YAML or JSON. A single chart uses `chart` or one entry under `charts` and displays inline in Sift agent chat. Several charts or a top-level `layout` are accepted and return an Explore link.

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
