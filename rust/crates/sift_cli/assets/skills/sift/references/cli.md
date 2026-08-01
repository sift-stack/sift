# Running `sift-cli` from your shell

Covers every `sift-cli` invocation: `import`, `export`, `config`, `ping`, and
`agent`. Read this before you construct a command.

Key subcommands:

- `import`: `csv`, `parquet flat-dataset`, `parquet cpr`, `tdms`, `hdf5`,
  `ulog`, `backups`.
- `export`: `run`, `asset` (to CSV and other formats).
- `mcp`: start the MCP server.
- `ping`: verify credentials and connectivity.
- `config`: manage profiles and credentials.
- `agent`: install, update, diagnose, or uninstall Sift's agent integration.

## Protocol

Invoke the CLI through your client's shell execution. The first step runs once
per session. The rest apply to each subcommand invocation.

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
