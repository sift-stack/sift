# MCP Server

`sift-cli mcp` starts a [Model Context Protocol](https://modelcontextprotocol.io)
server that exposes Sift to agentic coding tools such as Claude Code. The agent
discovers assets, pulls data, runs queries, and uploads results through
structured tools rather than shelling out to the CLI.

> **The Sift MCP server is in active development.** It is not yet a stable
> part of the CLI, and the tools, prompts, and behavior described on this page
> may change without notice. It is intentionally not built into default
> releases — the `mcp` Cargo feature is off by default. If you are exploring
> or helping develop it, opt in by building from source with `--features mcp`;
> see [Installation](../getting-started/installation.md). It is not yet
> recommended for production workflows.

```sh
sift-cli mcp
```

The server reads credentials from your configured profile, so make sure
[configuration](../getting-started/configuration.md) is done and
`sift-cli ping` succeeds first. Pass `--profile` and `--disable-tls` as needed:

```sh
sift-cli --profile mission mcp
```

The server communicates over stdio and is meant to be launched by an MCP client,
not run interactively.

## Destructive tools

Tools that modify existing state (`update_asset`, `update_run`, `update_report`,
`update_annotation`, `update_rule`, `archive_rule`, `unarchive_rule`) are
disabled by default. For an integration managed by `sift-cli agent`, enable
them across every detected MCP client together:

```sh
sift-cli agent update --allow-destructive
```

Then reload or restart each MCP client. A normal `agent update` preserves the
current access mode. Return all detected clients to safe mode with:

```sh
sift-cli agent update --read-only
```

Additive writes (`create_annotation`, `create_report`, `create_rule`,
`create_test_report`, `append_test_measurements`, `upload_dataset`) remain
available without the flag. The MCP error and installed skill tell agents to
ask for explicit approval before enabling destructive access. For an unmanaged
client, add `--allow-destructive` after `mcp` in its server command manually.

## Available tools

| Tool             | Purpose                                                                       |
| ---------------- | ----------------------------------------------------------------------------- |
| `list_assets`    | List assets, with filtering and ordering.                                     |
| `list_runs`      | List runs, with filtering and ordering.                                       |
| `list_channels`  | List channels for an asset.                                                   |
| `list_reports`   | List reports, with filtering and ordering.                                    |
| `list_report_rule_summaries` | List the per-rule pass/fail/open summaries for a single report.   |
| `create_report`  | Create a report over a run, from a template or a set of rules.                |
| `update_report`  | Update a report's metadata (replace semantics).                               |
| `list_annotations` | List annotations, with filtering and ordering.                              |
| `create_annotation` | Create a data-review or phase annotation over a time range.                |
| `update_annotation` | Update an annotation's fields (replace semantics for collections).         |
| `list_rules`     | List rules, with filtering and ordering.                                      |
| `list_rule_versions` | List the version history of a single rule.                                |
| `list_test_reports` | List test reports (test-results runs), with filtering and ordering.        |
| `list_test_steps` | List test steps within a report, with filtering and ordering.                |
| `list_test_measurements` | List test measurements, with filtering and ordering.                  |
| `count_test_steps` | Count test steps matching a filter, without fetching them.                  |
| `count_test_measurements` | Count test measurements matching a filter, without fetching them.    |
| `create_test_report` | Create a test report with its steps and measurements from a JSON document.  |
| `append_test_measurements` | Append measurements to an existing test step.                          |
| `get_data`       | Download channel data for an asset/run to a Parquet file, with optional decimation. |
| `sql`            | Run SQL over one or more Parquet files; chain after `get_data` for analysis.  |
| `upload_dataset` | Stream a Parquet dataset into Sift.                                           |
| `update_asset`   | Update an asset's tags and/or metadata (replace semantics).                   |
| `update_run`     | Update a run's fields (name, time bounds, pin, tags, metadata).               |
| `create_rule`    | Create a rule from a JSON definition.                                         |
| `update_rule`    | Update specific fields of a rule; unspecified fields are preserved.          |
| `archive_rule`   | Archive a rule so it stops evaluating.                                        |
| `unarchive_rule` | Restore an archived rule.                                                     |
| `search_docs`    | Search and read the Sift documentation and REST/gRPC API reference.           |

A typical agent flow is `list_assets` → `list_channels` → `get_data` → `sql`,
and `upload_dataset` to write results back.

## Built-in prompts

The server also ships [built-in prompts](./prompts.md): ready-made workflows
that chain these tools to explore an asset, analyze a run, or derive and upload
a new dataset.

## Configuring clients

Configure the MCP sidecar and the matching Sift skill for every detected client
in one step:

```sh
sift-cli agent install
```

This installs read-only registrations by default. To opt in to destructive
tools during initial setup, use `sift-cli agent install --allow-destructive`.
To make every detected client use a named Sift profile, include it in the same
installation:

```sh
sift-cli agent install --profile mission
```

An ordinary `agent update` preserves that profile. Use
`sift-cli agent update --profile <name>` to switch all clients to another named
profile, or `sift-cli agent update --default-profile` to return all clients to
the default.

Verify the result without making changes:

```sh
sift-cli agent doctor
```

The lifecycle and supported-client matrix are documented under
[Agent Integration](./skills.md).

For clients not managed by that command, point the MCP client at the CLI
manually:

```json
{
  "mcpServers": {
    "sift": {
      "command": "sift-cli",
      "args": ["mcp"]
    }
  }
}
```

For Claude Code specifically:

```sh
claude mcp add sift -- sift-cli mcp
```

The managed install uses user scope and the exact `sift-cli` executable that ran
the command, which also makes local debug builds deterministic.
