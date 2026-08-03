---
name: sift
description: >-
  Use when working with Sift: ingesting or importing time-series data,
  querying assets/runs/channels/users, exporting data, decimating or running SQL
  over data, opening a view in the Sift Explore web app, writing code
  that integrates with Sift, installing, updating, or diagnosing the Sift
  agent integration, or looking up how Sift works in its product and API
  documentation. Covers the Sift MCP server (started by
  `sift-cli mcp`), the `sift-cli` itself, the Sift REST API over cURL, the
  Sift Python library (`sift_client`), and the Sift Rust streaming library
  (`sift_stream`). Triggers include phrases like "import this file into
  Sift", "stream data to Sift", "list assets/runs/channels", "runs I created",
  "runs a teammate created", "export a
  run", "query Sift", "graph", "plot", "visualize", "open in Explore",
  "write code to integrate with Sift", "how does X work in Sift", "what does
  this endpoint do", or "look up the Sift API reference".
---

<!--
  Managed by sift-cli. Do not edit an installed copy; reinstall it with
  `sift-cli agent install` or `sift-cli agent update`.
-->

# Sift toolbox

Sift is a platform for ingesting, storing, querying, and analyzing time-series
and telemetry data. These instructions tell you which tool to reach for and how
to combine them when working with Sift.

## Your tools

1. **Sift MCP server** — started by `sift-cli mcp`. The preferred surface for
   agents. Exposes structured, authenticated tools:
   - `list_assets`, `list_runs`, `list_channels`, `list_reports`, `list_rules`,
     `list_rule_versions`, `list_annotations`: discover what exists. Pass `limit`
     (start at 50, max 200). Omitting it defaults to 50 and values above 200
     clamp to 200, so raise `limit` when a result comes back capped.
   - `list_report_rule_summaries`: per-rule pass/fail/open breakdown for a report.
   - Searching any of those lists: use `name.matches("(?i)rover")`, not `==`. Use
     `==` only for an exact value from a prior result. `contains`, `startsWith`,
     and `endsWith` are case-SENSITIVE: `contains("Rover")` silently misses
     `rover-01`. An empty result is not proof of absence — retry once with a
     shorter fragment before you tell the user that nothing exists. Each tool's
     description lists its own filterable fields. When a request is too vague to
     filter on, sample with a small `limit` and ask the user to narrow it rather
     than guessing.
   - `list_users`: resolve a person to a `user_id` by name, email, or id, then
     filter another list on `created_by_user_id` — that is how you answer "runs
     Jane created". For "runs I created", pass `me: true`, which resolves the
     caller from their API key; never guess which listed user is the caller.
   - `list_test_reports`, `list_test_steps`, `list_test_measurements`: inspect
     test-results data (reports own steps own measurements); `count_test_steps`,
     `count_test_measurements`: totals matching a filter without fetching rows.
   - `create_test_report`, `append_test_measurements`: create a test report with
     its step/measurement tree, or add measurements to an existing step (writes —
     confirm with the user first).
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
   - Destructive tools (`update_*`, `archive_*`, `unarchive_*`) are gated on
     `--allow-destructive`. If one is blocked, explain that this access is
     disabled by default and ask the user for explicit approval to enable it
     across every detected client. Only after approval, run
     `sift-cli agent update --allow-destructive`, ask the user to reload or
     restart the MCP client, and wait for confirmation before retrying. Never
     enable it silently. Restore safe mode with
     `sift-cli agent update --read-only`.
   - `explore_url`: build a Sift Explore deep-link for an asset/run/channel
     selection, with an optional panel/chart pre-defined. Surface the URL to
     the user as plain text, in full, so the user can open the view. Do not
     wrap it in a markdown link. Requires `app_uri` configured in the user's
     `sift-cli` profile (or pass `explore_host` per-call); fails with
     `INVALID_PARAMS` otherwise.
   - `search_docs`: search Sift's product documentation by keyword (`query`),
     then read a matching page by `path`. One tool, two modes.
2. **`sift-cli`** — the command-line tool. Key subcommands:
   - `import`: `csv`, `parquet flat-dataset`, `parquet cpr`, `tdms`, `hdf5`, `ulog`, `backups`.
   - `export`: `run`, `asset` (to CSV and other formats).
   - `mcp`: start the MCP server.
   - `ping`: verify credentials and connectivity.
   - `config`: manage profiles and credentials.
   - `agent`: install, update, diagnose, or uninstall Sift's agent integration.
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

## Managing the agent integration

Use the CLI lifecycle instead of editing one client's MCP configuration or
skill:

- Run `sift-cli agent doctor` to diagnose setup, including the installed
  profile and access mode, without changing it.
- For first-time setup, explain that `sift-cli agent install` installs the
  release-matched skill and read-only MCP registration for every detected
  client using the default Sift profile. If the user selected a named profile,
  pass it as `sift-cli agent install --profile <name>`. Run the command when the
  user asks you to install or approves the change.
- Run `sift-cli agent update` to refresh every detected client together. It
  preserves the existing profile and read-only or destructive access mode.
  Switch every client to another named profile with
  `sift-cli agent update --profile <name>`, or return them to the default with
  `sift-cli agent update --default-profile`.
- If the CLI is outdated, relay the exact curl or PowerShell installer printed
  by `agent doctor` or `agent update`. After the user updates `sift-cli`, rerun
  `sift-cli agent update`.
- Never repair or update only one detected client. If doctor reports mixed
  access modes, ask the user to choose `sift-cli agent update --read-only` or
  `sift-cli agent update --allow-destructive`. If it reports mixed profiles,
  ask for the intended profile and use `sift-cli agent update --profile <name>`
  or `sift-cli agent update --default-profile`.

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
     subcommand `one-d`/`two-d`/`compound`; Parquet's `cpr single`
     vs `cpr multi`). Ask only when the source's layout differs from
     the defaults shown in `--help`.

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
   prints a `View in Sift: <URL>` tip line after a successful upload when
   the URL can be resolved — either because the profile sets `app_uri`
   or because the API host is a recognized Sift environment (prod, gov,
   or Sift's dev SaaS). Surface that URL to the user as plain text, in
   full. Do not wrap it in a markdown link, do not summarize it away —
   the URL is part of the deliverable, and not every IDE renders
   markdown. Otherwise the CLI prints a fallback note telling the user
   how to configure `app_uri`; relay that note verbatim and do not
   invent a URL.
8. **On failure, read stderr and retry.** A non-zero exit usually means a
   bad flag combination or missing required argument; the CLI's stderr
   names the exact issue. Adjust the command and run again rather than
   treating the failure as terminal.

## Looking things up in the docs

When you need to know how Sift works — a feature, an endpoint, a parameter, a
CEL expression, calculated channels, UDFs — look it up with `search_docs`
rather than relying on memory. It serves Sift's product documentation (the same
content as docs.siftstack.com, including the full REST/gRPC API reference) and
is authenticated for you. Prefer it over guessing whenever you are unsure of a
detail or about to write code against the API.

`search_docs` has two modes; pass exactly one of `query` or `path`:

- **Search** (`query`): keywords like `asset channels CEL`. Returns ranked
  `hits`, each with `path`, `title`, `score`, `match_line`, `total_lines`, and
  `content` — the first page of the doc inline, so the top hit is usually
  answerable without a second call.
- **Read** (`path`): pass a hit's `path` to page past the `content` already
  returned, using `index` (1-indexed start line) and `lines` (count) with
  `total_lines` to know how far the page goes.

Search the topic, answer from the hit's `content`, and read only to page deeper
into a long doc. Cite the page you used.

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
supports CSV, Parquet (flat-dataset and channel-per-row), TDMS, HDF5, ULog,
and `sift_stream` backups.

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
