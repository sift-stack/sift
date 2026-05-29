# Sift CLI

`sift-cli` is the command-line interface for [Sift](https://siftstack.com), a
platform for ingesting, storing, querying, and analyzing time-series and
telemetry data.

Use it to:

- **Import** data into Sift from CSV, Parquet, TDMS, and HDF5 files, as well as
  from `sift_stream` backups.
- **Export** asset and run data back out to CSV, Parquet, or SUN.
- **Run the Sift MCP server**, which exposes Sift to agentic coding tools.
- **Install agent skills** so assistants like Claude Code know how to drive
  Sift on your behalf.
- **Verify connectivity and credentials** against your Sift environment.

## How this guide is organized

- **[Getting Started](./getting-started/installation.md)** walks you from
  installing the binary to a working, authenticated setup.
- **[Working with Data](./data/importing.md)** covers every import and export
  path, with a chapter and examples per file type.
- **[Agentic Tooling](./agents/mcp.md)** covers the MCP server and agent
  skills.
- **[Reference](./reference/commands.md)** is a flat list of every command and
  its flags.

If you are setting up for the first time, start with
[Installation](./getting-started/installation.md) and
[Configuration](./getting-started/configuration.md). Everything else assumes you
have a configured profile.

> Throughout this guide the binary is invoked as `sift-cli`. You can append
> `--help` to any command or subcommand to see its full set of flags.
