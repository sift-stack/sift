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
| `import ulog`                  | Import a PX4 ULog file.      |
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

In active development and not built into default releases. Opt in only if
you are exploring or contributing to its development by building from source
with `--features mcp`.

## `agent`

Manage the release-matched skill and MCP sidecar for all detected AI coding
clients. This command is available in builds made with `--features mcp`.

| Command                              | Description                                                                 |
| ------------------------------------ | --------------------------------------------------------------------------- |
| `agent install`                      | Install every client in read-only mode using the default profile.           |
| `agent install --profile <name>`     | Install every client using one named profile.                               |
| `agent install --allow-destructive`  | Install every client with destructive tools enabled.                        |
| `agent update`                       | Check CLI freshness and preserve the installed profile and access mode.     |
| `agent update --profile <name>`      | Refresh every client and switch them to one named profile.                  |
| `agent update --default-profile`     | Refresh every client and return them to the default profile.                |
| `agent update --allow-destructive`   | Refresh every client with destructive tools enabled.                        |
| `agent update --read-only`           | Refresh every client with destructive tools disabled.                       |
| `agent doctor`                       | Diagnose CLI, skill, MCP registration, profile, and access without changes. |
| `agent uninstall`                    | Remove only Sift-managed artifacts from every detected client.              |

## `ping`

Verify credentials and connectivity. See
[Verifying Your Setup](../getting-started/verifying.md).

## `install`

Install optional tooling.

| Command                          | Description                                  |
| -------------------------------- | -------------------------------------------- |
| `install completions print`      | Print shell completions to stdout.           |
| `install completions update`     | Update the completions file for your shell.  |
