---
name: sift
description: >-
  Use when working with Sift: ingesting or importing time-series data,
  querying assets/runs/channels/users, exporting data, decimating or running
  SQL over data, opening a view in the Sift Explore web app, writing code that
  integrates with Sift, installing, updating, or diagnosing the Sift agent
  integration, or looking up how Sift works in its product and API
  documentation. Covers the Sift MCP server (started by `sift-cli mcp`), the
  `sift-cli` itself, the Sift REST API over cURL, the Sift Python library
  (`sift_client`), and the Sift Rust streaming library (`sift_stream`).
  Triggers include phrases like "import this file into Sift", "stream data to
  Sift", "list assets/runs/channels", "runs I created", "runs a teammate
  created", "export a run", "query Sift", "graph", "plot", "visualize", "open
  in Explore", "write code to integrate with Sift", "how does X work in Sift",
  "what does this endpoint do", or "look up the Sift API reference".
---

<!--
  Managed by sift-cli. Do not edit an installed copy; reinstall it with
  `sift-cli agent install` or `sift-cli agent update`.
-->

# Sift toolbox

Sift is a platform for ingesting, storing, querying, and analyzing time-series
and telemetry data.

## Order of preference

Try these in order. Stop at the first that does the job.

1. **Sift MCP server** — started by `sift-cli mcp`. Structured, authenticated,
   and built for agents. Check here first.
2. **`sift-cli`** — file imports the MCP server does not cover, exports, and
   config.
3. **REST API over cURL** — the complete API surface, for anything the first
   two miss.
4. **Python library (`sift_client`)** — when the task needs a script. The older
   `sift_py` is deprecated. Reach for it only when `sift_client` lacks the
   capability.

## What the MCP server exposes

Each tool's own description carries its parameters, filters, defaults, and
errors. Read the tool schema instead of guessing. This map only tells you what
exists.

- **Discovery:** `list_assets`, `list_runs`, `list_channels`, `list_reports`,
  `list_rules`, `list_rule_versions`, `list_annotations`.
- **People:** `list_users`.
- **Report detail:** `list_report_rule_summaries`.
- **Test results:** `list_test_reports`, `list_test_steps`,
  `list_test_measurements`, `count_test_steps`, `count_test_measurements`.
- **Data:** `get_data` writes channel data to a Parquet file. `sql` queries
  Parquet files. `upload_dataset` streams a Parquet dataset into Sift.
- **Links:** `explore_url`.
- **Docs:** `search_docs`.
- **Writes:** `create_rule`, `update_rule`, `archive_rule`, `unarchive_rule`,
  `create_annotation`, `update_annotation`, `create_report`, `update_report`,
  `create_test_report`, `append_test_measurements`, `update_asset`,
  `update_run`.

## Workflows that span tools

- **Search a list.** Filter with a pattern rather than an exact match. Each
  tool's description names its own filterable fields. When the request is too
  vague to filter on, sample with a small `limit` and ask the user to narrow
  it. Do not guess.
- **Attribute something to a person.** Resolve the person with `list_users`,
  then filter another list on `created_by_user_id`. For "runs I created", pass
  `me: true`. Never guess which listed user is the caller.
- **Produce numbers.** `get_data` writes a Parquet file. `sql` then queries it.
  Add `upload_dataset` when the result belongs back in Sift.
- **Produce a chart.** Build a link with `explore_url`. When the user wants a
  chart and numbers, do both and give the user both.
- **Answer a question about how Sift works.** Call `search_docs`. Do not answer
  from memory, and cite the page you used.

## Rules that always apply

- **Stop on empty list results.** If any `list_*` tool returns no items, tell
  the user that nothing matched and ask how to proceed. Do not make subsequent
  tool calls using an empty or placeholder name or id from the missing result.
- **Surface URLs as plain text, in full.** The link from `explore_url` and the
  `View in Sift:` line from `sift-cli import` are deliverables. Never invent a
  URL that a tool did not return.
- **Confirm every write before you run it.** Show the user the proposed change
  and its target, then wait for approval.
- **Destructive tools are off by default.** `update_*`, `archive_*`, and
  `unarchive_*` need `--allow-destructive`. If one is blocked, tell the user
  that this access is disabled by default and ask for explicit approval. Never
  enable it silently. The procedure is in
  [references/agent-setup.md](references/agent-setup.md).
- **Choose one profile for the session and keep it.** Never switch profiles to
  recover from a failure. Surface the failure and ask the user.

## Reference files

Read the file that matches the task in front of you. Do not read them all.

| When the task is | Read |
|---|---|
| any `sift-cli` invocation: import, export, config | [references/cli.md](references/cli.md) |
| a chart, a plot, or an Explore link | [references/explore-links.md](references/explore-links.md) |
| install, update, or diagnose the Sift integration | [references/agent-setup.md](references/agent-setup.md) |
| code written against Sift's libraries or REST API | [references/integration-code.md](references/integration-code.md) |
