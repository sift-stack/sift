# MCP Server

`sift-cli mcp` starts a [Model Context Protocol](https://modelcontextprotocol.io)
server that exposes Sift to agentic coding tools such as Claude Code. The agent
discovers assets, pulls data, runs queries, and uploads results through
structured tools rather than shelling out to the CLI.

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
| `get_data`       | Download channel data for an asset/run to a Parquet file, with optional decimation. |
| `sql`            | Run SQL over one or more Parquet files; chain after `get_data` for analysis.  |
| `upload_dataset` | Stream a Parquet dataset into Sift.                                           |

A typical agent flow is `list_assets` → `list_channels` → `get_data` → `sql`,
and `upload_dataset` to write results back.

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
