---
name: sift
description: >-
  Use when working with Sift: ingesting or importing time-series data,
  querying assets/runs/channels, exporting data, decimating or running SQL
  over data, opening a view in the Sift Explore web app, or writing code
  that integrates with Sift. Covers the Sift MCP server (started by
  `sift-cli mcp`), the `sift-cli` itself, the Sift REST API over cURL, the
  Sift Python library (`sift_client`), and the Sift Rust streaming library
  (`sift_stream`). Triggers include phrases like "import this file into
  Sift", "stream data to Sift", "list assets/runs/channels", "export a
  run", "query Sift", "graph", "plot", "visualize", "open in Explore", or
  "write code to integrate with Sift".
---

<!--
  LOCKSTEP: This file shares its body with assets/skills/agents-md/AGENTS.md.
  Everything from the "# Sift toolbox" heading down must stay identical to the
  body of AGENTS.md. When you change one, change the other in the same commit.
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
     `list_rule_versions`, `list_annotations`: discover what exists.
   - `list_report_rule_summaries`: per-rule pass/fail/open breakdown for a report.
   - `get_data`: download channel data for an asset/run to a Parquet file.
   - `sql`: run SQL over one or more Parquet files (chain after `get_data`).
   - `upload_dataset`: stream a Parquet dataset into Sift. Returns an
     `explore_url` field when the user's profile has `app_uri` configured —
     surface it to the user as plain text, in full. Do not wrap it in a
     markdown link; not every IDE renders markdown. If `explore_url` is null,
     do not invent a link.
   - `update_asset`: replace an existing asset's tags and/or metadata (write —
     replace semantics, so read-modify-write when appending).
   - `update_run`: update a run's name, time bounds, pin state, tags, or metadata
     (write — tags/metadata use replace semantics).
   - `create_rule`, `update_rule`, `archive_rule`, `unarchive_rule`: manage rules
     (writes — confirm the change with the user first).
   - `create_annotation`, `update_annotation`: manage annotations (writes —
     collections use replace semantics, so confirm the change first).
   - `create_report`, `update_report`: manage reports (writes — confirm first).
   - `explore_url`: build a Sift Explore deep-link for an asset/run/channel
     selection, with an optional panel/chart pre-defined. Surface the URL to
     the user as plain text, in full, so the user can open the view. Do not
     wrap it in a markdown link. Requires `app_uri` configured in the user's
     `sift-cli` profile (or pass `explore_host` per-call); fails with
     `INVALID_PARAMS` otherwise.
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

## Running `sift-cli` from your shell

When you reach for `sift-cli` per the order above, invoke it through your
client's shell execution. The first step runs once per session; the rest
apply per subcommand invocation:

1. **Pick a profile (once per session).** Run `sift-cli config show` to list
   the configured profiles. If only one is configured, use it (no
   `--profile` needed). If multiple are configured, ask the user which one
   to target and pass `--profile <name>` as a global flag on every
   subsequent `sift-cli` call in this session. Do not silently default
   when several profiles exist — the user may have prod and staging side
   by side and writing to the wrong one is a real foot-gun.

   **Never switch profiles to recover from a failure.** Once a profile is
   chosen for the session, stick with it. If a command fails — bad
   credentials, host unreachable, the default profile doesn't resolve,
   gRPC errors, anything — surface the failure and ask the user before
   moving to a different profile. Do not retry the same command against
   another profile to "make it work"; that risks writing the user's data
   into the wrong environment. The same applies in reverse: if the user
   has not named a profile and only one exists but it fails, stop and
   report — don't probe other profiles.
2. **Discover the subcommand.** Before constructing the command for a
   subcommand you have not used recently, run `sift-cli <subcommand>
   --help` (or `sift-cli --help` for the top level). The clap-generated
   help is the source of truth for flags, defaults, and value formats.
   Do not guess flag names from memory.
3. **Probe useful optionals.** After reading `--help`, identify optional
   flags whose answer changes the outcome and ask the user about them
   before running. For imports, the common ones are:
   - `--run`: associate the data with a named run. Ask whether to create
     one, and if so what to name it.
   - `--preview`: parse the source file and print the inferred schema
     without uploading. Offer this when the user is unsure about column
     types or the time column.
   - Per-format layout flags surfaced by `--help` (e.g. CSV's
     `--header-row`, `--time-column`, `--time-format`; HDF5's schema
     selection). Ask only when the source's layout differs from the
     defaults shown in `--help`.

   Do not enumerate every flag — pick the ones likely to matter for
   the user's task. When in doubt, ask one focused question rather than
   running with assumed defaults.
4. **Confirm writes.** For any subcommand that mutates Sift state
   (imports, config changes), surface the final proposed command and the
   target (asset, run, profile) to the user and wait for approval before
   running.
5. **Use absolute paths.** Pass absolute paths for any file argument so
   the command does not depend on the shell's current directory.
6. **For imports, always pass `--wait`.** With `--wait` the CLI blocks
   until the server-side import job finishes and emits a final status
   line. Without it you cannot confirm the data actually landed. Relay
   the final stdout line to the user verbatim.
7. **Surface the Explore link from import output.** `sift-cli import`
   prints a tip line after a successful upload. It is either
   `View in Sift: <URL>` (when the profile sets `app_uri`, or the API
   host is a recognized Sift SaaS environment) or a fallback note of the
   form `Run `sift-cli config update --app-uri <SIFT_WEB_URL>` so future
   imports include a Sift Explore link.` (custom or on-prem deployments
   without `app_uri` set). When a URL line appears, surface that URL to
   the user as plain text, in full. Do not wrap it in a markdown link,
   do not summarize it away — the URL is part of the deliverable, and
   not every IDE renders markdown. When the fallback note appears, relay
   it verbatim and do not invent a URL.
8. **On failure, read stderr and retry.** A non-zero exit usually means a
   bad flag combination or missing required argument; the CLI's stderr
   names the exact issue. Adjust the command and run again rather than
   treating the failure as terminal.

## Local data analysis

When the user wants numbers, summaries, or transformed data — anything where
the output is text or a new dataset — pull the source data locally with
`get_data` (writes a Parquet file) and run `sql` over it. Chain
`get_data` → `sql` for filtering, aggregation, or feature derivation. If the
result should land back in Sift as a new dataset, follow with
`upload_dataset`, and confirm the target asset/run with the user first. When
`upload_dataset` returns an `explore_url`, surface it to the user as plain
text, in full, so they can jump straight to the imported data. Do not wrap it
in a markdown link.

## Visualizing in Sift Explore

When the user wants to see, view, graph, plot, or open data in Sift, build
a link with `explore_url` and surface the URL to the user as plain text, in
full. The URL is the deliverable — do not wrap it in a markdown link, do not
summarize it away. Pick the
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