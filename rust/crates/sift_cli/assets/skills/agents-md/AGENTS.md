<!--
  LOCKSTEP: This file shares its body with assets/skills/claude-code/SKILL.md.
  Everything from the "# Sift toolbox" heading down must stay identical to the
  body of SKILL.md (which carries the same content under YAML frontmatter).
  When you change one, change the other in the same commit.
  See rust/crates/sift_cli/CLAUDE.md for the rules.
-->

# Sift toolbox

Sift is a platform for ingesting, storing, querying, and analyzing time-series
and telemetry data. These instructions tell you which tool to reach for and how
to combine them when working with Sift.

## Your tools

1. **Sift MCP server** — started by `sift-cli mcp`. The preferred surface for
   agents. Exposes structured, authenticated tools:
   - `list_assets`, `list_runs`, `list_channels`, `list_reports`, `list_rules`,
     `list_rule_versions`: discover what exists.
   - `get_data`: download channel data for an asset/run to a Parquet file.
   - `sql`: run SQL over one or more Parquet files (chain after `get_data`).
   - `upload_dataset`: stream a Parquet dataset into Sift.
   - `create_rule`, `update_rule`, `archive_rule`, `unarchive_rule`: manage rules
     (writes — confirm the change with the user first).
   - `explore_url`: build a Sift Explore deep-link for an asset/run/channel
     selection, with an optional panel/chart pre-defined. Surface the URL
     inline as a clickable link so the user can open the view.
2. **`sift-cli`** — the command-line tool. Key subcommands:
   - `import`: `csv`, `parquet flat-dataset`, `tdms`, `hdf5`, `backups`.
   - `export`: `run`, `asset` (to CSV and other formats).
   - `mcp`: start the MCP server.
   - `ping`: verify credentials and connectivity.
   - `config`: manage profiles and credentials.
   - `install`: install completions and these agent skills.
3. **REST API over cURL** — the full API surface. Docs:
   https://docs.siftstack.com/api/rest
4. **Sift Python library** — module `sift_client`. Reference:
   https://sift-stack.github.io/sift/python/latest/reference/sift_client/
   Use `sift_client`. The older `sift_py` module is deprecated; reach for it
   only as a last resort when `sift_client` lacks a needed capability.
5. **Sift Rust streaming library** — `sift_stream`, for high-throughput
   streaming ingestion. Reference: https://docs.rs/sift_stream/latest/sift_stream/

## Order of preference

When a user asks you to carry out a task against Sift, try tools in this order
and stop at the first that does the job:

1. **MCP server.** Check whether an MCP tool already covers the task. It is
   structured, authenticated, and purpose-built for agents.
2. **`sift-cli`.** Use for operations the MCP server does not cover, such as
   importing additional file types, exporting, and managing config.
3. **REST API over cURL.** Use for anything the MCP server and CLI do not
   expose. This is the complete API surface.
4. **Python library (`sift_client`).** Use when the task needs a script:
   custom streaming, data transformation, or programmatic logic the above
   cannot express. Prefer `sift_client` over the deprecated `sift_py`.

## Local data analysis

When the user wants numbers, summaries, or transformed data — anything where
the output is text or a new dataset — pull the source data locally with
`get_data` (writes a Parquet file) and run `sql` over it. Chain
`get_data` → `sql` for filtering, aggregation, or feature derivation. If the
result should land back in Sift as a new dataset, follow with
`upload_dataset`, and confirm the target asset/run with the user first.

## Visualizing in Sift Explore

When the user wants to see, view, graph, plot, or open data in Sift, build
a link with `explore_url` and render the URL inline as a clickable markdown
link. The URL is the deliverable — do not summarize it away. Pick the
`panel_type` that fits the request: `timeseries` (default), `histogram`,
`table`, `fft`, `metrics`, `scatter-plot`, or `geo-map`. Prefix channels
with `L1:` / `L2:` for multi-axis plots; with `x:` / `y:` / `color:` for
scatter; with `lat:` / `lon:` / `color:` for geo.

If the user wants both a chart and numbers, produce the `explore_url` link
and chain `get_data` + `sql` together.

## Importing data

There are two ways to get data into Sift: importing a file, or streaming.

### Import a file

`sift-cli` and the Python library import several file types directly. The CLI
supports CSV, Parquet (flat dataset), TDMS, HDF5, and `sift_stream` backups.

If the user's file type is not supported by the CLI or MCP server, you have
three options:

1. Transform the data into a CSV or Parquet file and import that with the CLI.
2. Transform the data into a Parquet file and upload it with the MCP
   `upload_dataset` tool.
3. Stream the data into Sift with the Python library.

### Stream data

Stream when the data is live, large, or in a format that does not map cleanly
to a file import.

- **Rust:** `sift_stream`. Reference and examples:
  https://docs.rs/sift_stream/latest/sift_stream/
- **Python:** ingestion examples:
  https://sift-stack.github.io/sift/python/latest/examples/ingestion/

## Writing integration code

When a user wants help integrating their own systems with Sift, consult the
docs above (REST, Python, Rust) and write code against `sift_client` for
Python or `sift_stream` for Rust. Use the examples in those docs as the
starting point rather than inventing API shapes.
