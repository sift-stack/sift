# Built-in Prompts

The Sift MCP server ships with a set of built-in prompts: ready-made workflows
that walk an agent through a complete task using the [MCP tools](./mcp.md). A
prompt expands into step-by-step instructions tailored to the arguments you
give it, so you start a focused session without writing the workflow yourself.

Prompts are served by `sift-cli mcp`, so the server must be
[configured as a client](./mcp.md#configuring-a-client) first. The MCP server
is in active development and is not built into default releases — see
[MCP Server](./mcp.md) for current status.

## Using a prompt

How you invoke a prompt depends on your MCP client, and on the name you
registered the server under. Assuming you registered it as `sift` in your
`.mcp.json`:

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

then in Claude Code each prompt is available as a slash command of the form
`/mcp__sift__<prompt>`:

- `/mcp__sift__explore_asset`
- `/mcp__sift__analyze_run`
- `/mcp__sift__derive_and_upload`

The `sift` in the command is your registered server name, so a different name
in `.mcp.json` changes the prefix accordingly. Arguments are passed positionally
after the command, separated by spaces; wrap any argument containing spaces in
quotes. Other MCP clients discover the same prompts but may present them
differently, so check your client's documentation for its invocation syntax.

## `explore_asset`

A read-only starting point for a session. It resolves an asset, lists its
recent runs, and inventories its channels, without pulling any sample data.

| Argument | Required | Description                            |
| -------- | -------- | -------------------------------------- |
| `asset`  | yes      | Asset name to explore (exact or partial match). |

Example:

```
/mcp__sift__explore_asset "Falcon 9 Booster"
```

The agent resolves the asset, then reports its most recent runs and the channel
inventory grouped by data type, surfacing the exact run and channel names so you
can reuse them with the other prompts.

## `analyze_run`

Pulls a run's channel data and produces a per-channel statistical summary. You
can target specific channels and pose a question for the agent to answer.

| Argument   | Required | Description                                                  |
| ---------- | -------- | ------------------------------------------------------------ |
| `asset`    | yes      | Asset the run belongs to.                                    |
| `run`      | yes      | Run to analyze.                                              |
| `channels` | no       | Channels to pull. Omit to let the agent choose a subset.     |
| `question` | no       | A question for the agent to answer from the data.            |

Analyze every channel on a run:

```
/mcp__sift__analyze_run "Falcon 9 Booster" "Static Fire 2024-05-01"
```

Target specific channels and ask a question:

```
/mcp__sift__analyze_run "Falcon 9 Booster" "Static Fire 2024-05-01" "chamber_pressure,fuel_temp" "Did chamber pressure stay within nominal range?"
```

The agent pulls the data with `get_data`, summarizes it with `sql` (row counts,
min/max/mean, null rate), and reports the Parquet paths so you can continue the
work.

## `derive_and_upload`

Derives a new dataset from an existing run via SQL and uploads it back to Sift.
Because the upload is a write, the agent confirms the destination with you before
running it.

| Argument       | Required | Description                                                       |
| -------------- | -------- | ----------------------------------------------------------------- |
| `source_asset` | yes      | Asset to read from.                                               |
| `source_run`   | yes      | Run to read from.                                                 |
| `transform`    | yes      | Plain-language description of the transform to apply.             |
| `target_asset` | no       | Asset to write to. The agent proposes a default if omitted.       |
| `target_run`   | no       | Run to create. The agent asks whether to create one if omitted.   |

Derive a new dataset, letting the agent propose the destination:

```
/mcp__sift__derive_and_upload "Falcon 9 Booster" "Static Fire 2024-05-01" "1-second average of chamber_pressure and fuel_temp"
```

Specify the destination explicitly:

```
/mcp__sift__derive_and_upload "Falcon 9 Booster" "Static Fire 2024-05-01" "1-second average of chamber_pressure" "Falcon 9 Booster-derived" "Static Fire 2024-05-01 1s-avg"
```

The agent extracts the source data, applies the transform with `sql` (keeping
`timestamp_unix_nanos` as the first column, as Sift requires), confirms the
target asset, run, and any tags with you, then uploads the result with
`upload_dataset`.