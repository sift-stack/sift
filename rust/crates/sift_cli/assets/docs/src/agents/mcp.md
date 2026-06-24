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
| `get_data`       | Download channel data for an asset/run to a Parquet file, with optional decimation. |
| `sql`            | Run SQL over one or more Parquet files; chain after `get_data` for analysis.  |
| `upload_dataset` | Stream a Parquet dataset into Sift.                                           |
| `update_asset`   | Update an asset's tags and/or metadata (replace semantics).                   |
| `update_run`     | Update a run's fields (name, time bounds, pin, tags, metadata).               |
| `create_rule`    | Create a rule from a JSON definition.                                         |
| `update_rule`    | Update specific fields of a rule; unspecified fields are preserved.          |
| `archive_rule`   | Archive a rule so it stops evaluating.                                        |
| `unarchive_rule` | Restore an archived rule.                                                     |

A typical agent flow is `list_assets` → `list_channels` → `get_data` → `sql`,
and `upload_dataset` to write results back.

## Built-in prompts

The server also ships [built-in prompts](./prompts.md): ready-made workflows
that chain these tools to explore an asset, analyze a run, or derive and upload
a new dataset.

## Configuring a client

Most MCP clients take a command and arguments. Point yours at the CLI:

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

For Claude Code specifically, you can also register it from the terminal:

```sh
claude mcp add sift -- sift-cli mcp
```

To give your assistant guidance on *when* to use these tools alongside the CLI,
REST API, and client libraries, install the [agent skills](./skills.md).
