---
name: sift
description: >-
  Use when working with Sift: ingesting or importing time-series data,
  querying assets/runs/channels/users, managing calculated channels, rules,
  user-defined functions, exporting data,
  decimating or running SQL over data, opening a view in the Sift Explore web
  app, writing code that integrates with Sift, installing, updating, or
  diagnosing the Sift agent integration, or looking up how Sift works in its
  product and API documentation. Covers the Sift MCP server (started by
  `sift-cli mcp`), the `sift-cli` itself, the Sift REST API over cURL, the Sift
  Python library (`sift_client`), and the Sift Rust streaming library
  (`sift_stream`).
  Triggers include phrases like "import this file into Sift", "stream data to
  Sift", "list assets/runs/channels", "runs I created", "runs a teammate
  created", "export a run", "query Sift", "graph", "plot", "visualize", "open
  in Explore", "write code to integrate with Sift", "how does X work in Sift",
  "what does this endpoint do", "list calculated channels", "preview a rule",
  or "look up the Sift API reference".
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

- **Setup:** When available, `check_for_updates` reports the installed sift-cli
  version, the latest stable version, and the exact installer command. Servers
  started with `--disable-update-check` omit this tool. `ping` is a
  connectivity check; when it fails, expect every other Sift tool to fail too.
- **Discovery:** `list_assets`, `list_runs`, `list_channels`, `list_reports`,
  `list_report_templates`, `list_rules`, `list_rule_versions`, `list_annotations`.
- **Derived channels:** `list_calculated_channels`,
  `list_calculated_channel_versions`, `list_user_defined_functions`,
  `list_user_defined_function_versions`.
- **People:** `list_users`.
- **Rollups:** `list_report_rule_summaries` for a report's rule progress.
- **Rule dry runs:** `preview_rule` returns the annotations a rule would create
  without persisting anything. It is read-only whatever the server's access
  mode, so it needs no write flag.
- **Test results:** `list_test_reports`, `list_test_steps`,
  `list_test_measurements`, `count_test_steps`, `count_test_measurements`.
  These, along with the `create_test_report` and `append_test_measurements`
  writes below, are gated behind the `test-reports` Cargo feature (default on).
  A server built with `--no-default-features` will not expose them.
- **Data:** `get_data` writes channel data to a Parquet file. `sql` queries
  Parquet files. `upload_dataset` streams a Parquet dataset into Sift.
- **Links:** `explore_url`.
- **Docs:** `search_docs`.
- **Writes:** `create_rule`, `update_rule`, `archive_rule`, `unarchive_rule`,
  `create_annotation`, `update_annotation`, `create_report`, `update_report`,
  `create_report_template`, `update_report_template`,
  `create_calculated_channel`, `update_calculated_channel`,
  `archive_calculated_channel`, `unarchive_calculated_channel`,
  `create_user_defined_function`, `update_user_defined_function`,
  `archive_user_defined_function`, `unarchive_user_defined_function`,
  `create_test_report`, `append_test_measurements`, `update_asset`,
  `update_run`.

## Workflows that span tools

- **Start a Sift session.** If `check_for_updates` is available, call it once at
  the start of each session. Call it before any other Sift tool. If it reports
  `update_available`, relay its
  `message` and exact `install_command`. If it reports `unavailable`, continue
  with the requested Sift task.
- **Search a list.** Filter with a pattern rather than an exact match. Each
  tool's description names its own filterable fields. When the request is too
  vague to filter on, sample with a small `limit` and ask the user to narrow
  it. Do not guess.
- **Attribute something to a person.** Resolve the person with `list_users`,
  then filter another list on `created_by_user_id`. For "runs I created", pass
  `me: true`. Never guess which listed user is the caller.
- **Update annotations.** `update_annotation` takes a required
  `annotation_ids` list of 1 to 1000 ids and applies the same changes to every
  target. Check its per-id `failures`, `not_attempted` ids, and archive outcome
  before reporting success; a partial failure sets `isError`.
- **Produce numbers.** `get_data` writes a Parquet file. `sql` then queries it.
  Add `upload_dataset` when the result belongs back in Sift.
- **Query a derived channel.** `get_data` serves saved calculated channels as
  well as raw ones: name the calculated channel in `channel_names` and it is
  evaluated for the requested asset and run. Confirm the name with
  `list_calculated_channels` filtered on the asset first. A raw channel wins a
  name it shares with a calculated channel, and `channel_regex` matches raw
  channels only, so name calculated channels explicitly. When the result
  carries `unresolved_calculated_channels`, the file is missing those columns:
  tell the user which channels did not resolve rather than reporting the file as
  complete.
- **Author or change a calculated channel.** A calculated channel is a SEL
  expression plus an asset scope. `create_calculated_channel` and
  `update_calculated_channel` take the expression with `$1`, `$2`, …
  placeholders plus `expression_channel_references_json`, a JSON string array
  mapping each placeholder to a channel. Resolve those channel names with
  `list_channels` first. Update and archive both create a new version instead of
  replacing or deleting anything, and `list_calculated_channel_versions` shows
  the history.
- **Use or change a user-defined function.** Call
  `list_user_defined_functions` before writing an expression that calls a UDF:
  it gives the exact name, input order, and output type the expression must
  match. An update creates a new version and leaves earlier ones intact, so
  `list_user_defined_function_versions` gives history and pinned version ids.
  Send a rename on its own. The API applies a `name` change by itself and
  ignores every other field, so `update_user_defined_function` rejects `name`
  combined with anything else.
- **Produce a chart.** Build a link with `explore_url`. When the user wants a
  chart and numbers, do both and give the user both.
- **Answer a question about how Sift works.** Call `search_docs`. Do not answer
  from memory, and cite the page you used.
- **Create an asset.** There is no `create_asset` MCP tool because Sift creates
  assets implicitly on ingest. The MCP path is `upload_dataset` with the target
  asset name; the asset is registered as a side effect. `upload_dataset` is a
  create tool, so it is gated by `--allow-create`. If the server is read-only,
  DO NOT reach for the REST `CreateAsset` RPC, `sift-cli import`, `sift_stream`,
  or a gRPC client to make the asset another way. Surface the block: name the
  gated tool and the exact `sift-cli agent update --allow-create` command, and
  wait for the user to widen access.
- **Evaluate rules against a run.** Find rules with `list_rules` and author rules
  with `create_rule`. Dry-run first: `preview_rule` returns the annotations a
  rule would generate for one run and persists nothing. It takes either a saved
  rule (`rule_id` or `rule_name`) or a fully ad-hoc `draft_rule_config` JSON
  string, so a rule need not be saved to be tested. Use `create_report` when the
  evaluation should persist. To reuse the same rule set across many runs, bundle
  standard rules (`is_external: false`) into a template with `create_report_template`,
  then call `create_report` with `report_template_id`. For a one-off — or for
  ad-hoc rules (`is_external: true`, which the API also calls "external" but
  cannot be attached to a template) — skip the template and pass `rule_ids`,
  `rule_client_keys`, or `rule_version_ids` directly to `create_report`. Track
  progress via `list_report_rule_summaries`.

## Rules that always apply

- **Stop on empty list results.** If any `list_*` tool returns no items, tell
  the user that nothing matched and ask how to proceed. Do not make subsequent
  tool calls using an empty or placeholder name or id from the missing result.
- **Surface URLs as plain text, in full.** The link from `explore_url` and the
  `View in Sift:` line from `sift-cli import` are deliverables. Never invent a
  URL that a tool did not return.
- **Confirm every write before you run it.** Show the user the proposed change
  and its target, then wait for approval.
- **Write tools are off by default.** Read-only is the default access mode.
  `create_*`, `upload_dataset`, and `append_test_measurements` need
  `--allow-create`. `update_*`, `archive_*`, and `unarchive_*` need
  `--allow-destructive` (which implies create). If a call is blocked, tell the
  user that this access is disabled by default and ask for explicit approval.
  Never widen access silently.
- **A blocked MCP tool is a user policy signal, not a transport error.** If
  the MCP gate blocks a write, do NOT route around it — do not shell to
  `sift-cli import`, `curl` against the REST API, `sift_client` Python, or
  another MCP server that happens to be in destructive mode. Surface the
  block and the exact remediation command; wait for the user to widen access.
  The procedure is in [references/agent-setup.md](references/agent-setup.md).
- **"No dedicated MCP tool" is not the same as "can't do this in MCP".**
  Several creates happen as side effects of other MCP tools — an asset is
  created when `upload_dataset` names one that doesn't exist, a run is
  created when a `create_report`/`create_test_report`/`upload_dataset` names
  one, and a tag is created when `update_asset` or `update_run` names one that
  doesn't exist (there is no `create_tag` tool). When the user asks for a create
  with no matching `create_*` tool, look for the tool that creates it as a side
  effect before falling out of MCP. If that side-effect tool is gated and
  blocked, that IS the block —
  surface it, do not treat "no `create_asset` tool" as license to shell out
  to REST/gRPC/`sift-cli import`.
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
