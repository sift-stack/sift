# Command Reference

A flat overview of every command. Append `--help` to any command for the full,
authoritative set of flags for your installed version.

## Global flags

These apply to any command that talks to Sift:

| Flag            | Description                                  |
| --------------- | -------------------------------------------- |
| `--profile`     | Use a named profile from the config file.    |
| `--disable-tls` | Disable TLS for non-cloud Sift environments. |

## `config`

Manage CLI configuration. See [Configuration](../getting-started/configuration.md).

| Command                 | Description                                       |
| ----------------------- | ------------------------------------------------- |
| `config create`         | Create a new (empty) config file.                 |
| `config show`           | Print the current config file.                    |
| `config where`          | Print the path to the config file.                |
| `config update`         | Set fields (`-g`, `-r`, `-k`) or use `--interactive`. |

## `import`

Import time-series files. See [Importing Data](../data/importing.md).

| Command                        | Description                  |
| ------------------------------ | ---------------------------- |
| `import csv`                   | Import a CSV file.           |
| `import parquet flat-dataset`  | Import a flat Parquet file.  |
| `import tdms`                  | Import a TDMS file.          |
| `import hdf5`                  | Import an HDF5 file.         |
| `import backups`              | Replay `sift_stream` backups.|
| `import backups ls`           | List backup files.           |

## `export`

Export data from Sift. See [Exporting Data](../data/exporting.md).

| Command         | Description                |
| --------------- | -------------------------- |
| `export run`    | Export data for a run.     |
| `export asset`  | Export data for an asset.  |

## `mcp`

Start the Sift MCP server. See [MCP Server](../agents/mcp.md).

Gated behind the `mcp` Cargo feature (on by default). Source builds with
`--no-default-features` omit this subcommand.

## `ping`

Verify credentials and connectivity. See
[Verifying Your Setup](../getting-started/verifying.md).

## `install`

Install optional tooling.

| Command                          | Description                                  |
| -------------------------------- | -------------------------------------------- |
| `install completions print`      | Print shell completions to stdout.           |
| `install completions update`     | Update the completions file for your shell.  |
| `install agent-skills <AGENT>`   | Install a Sift skill for an AI assistant.    |
