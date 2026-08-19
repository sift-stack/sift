# Change Log
All notable changes to `sift-cli` will be documented in this file.

This project adheres to [Semantic Versioning](http://semver.org/).

## [Unreleased]

### What's New

- Added a `--disable-nonessential-traffic` flag to `sift-cli mcp` that stops
  the MCP server from making non-essential network requests. Release checks
  stay under `--disable-update-check`.
- `sift-cli agent install` now works in headless environments (container
  builds, CI). A client's config directory counts as detection even when its
  binary is not on PATH, the skill always installs (it is plain file IO), and
  a client-CLI-backed MCP registration that cannot run is skipped with a
  warning instead of failing the install. `agent doctor` reports such a
  registration as a warning rather than an error.

## [v0.4.1] - August 10, 2026

### What's New

- Added an `--allow-create` MCP access tier alongside `--allow-destructive`.
  The server now recognizes three tiers: read-only (default), create (enables
  `create_*`, `upload_dataset`, `append_test_measurements`), and destructive
  (implies create; enables `update_*`, `archive_*`, `unarchive_*`). The flag is
  available on `sift-cli mcp`, `sift-cli agent install`, and
  `sift-cli agent update`; blocked calls now name the exact remediation command
  for the tier they need.
- Add in `check_for_updates` tool into MCP that proactively checks to see if
  the current `sift-cli` version is out of date. Token optimized. If outdated,
  provides a clear installation path for the latest version.

## [v0.4.0] - August 5, 2026

### What's New

- Added `sift-cli agent install`, `update`, `doctor`, and `uninstall` for
  stateless, lockstep management of the release-matched Sift skill and MCP
  sidecar across detected Claude Code, Codex, Cursor, and OpenCode clients.
  Cursor and OpenCode support is new in this release.
- Added safe-by-default, lockstep MCP access controls: install defaults to
  read-only, update preserves the existing mode, explicit flags enable or
  disable destructive tools for all detected clients, and doctor reports mixed
  modes.
- The `agent` command and `mcp` sidecar are no longer behind a Cargo feature
  and ship in every build. `cargo build -p sift_cli` now includes them by
  default; the prior `--features mcp` flag is gone.
- Split the installed skill into a router (`SKILL.md`) plus dedicated
  reference files under `references/` for better token efficiency and clearer
  agent guidance.
- Added a `list_users` MCP tool. Combined with the other list tools, this
  supports patterns like "runs I created" or "assets created by <teammate>"
  without guessing IDs.
- Added report-template MCP tools: `list_report_templates`,
  `create_report_template`, and `update_report_template`. Templates bundle a
  reusable set of rules that any `create_report` call can inherit via
  `report_template_id`.
- Improved MCP filter guidance: agents default `is_archived == false` on
  every list call, use regex/case-insensitive matching
  (`name.matches("(?i)...")`) instead of `==` for text search, and always
  pass `limit` to keep results within the context window.
- MCP tool calls no longer proceed on an empty list result. Empty lists
  surface to the user for guidance instead of triggering a follow-up call
  with an empty ID.
- Added an opt-in `test-reports` Cargo feature on `sift_mcp` (forwarded from
  `sift_cli`) that gates the test-report tools. Off by default; enable with
  `--features test-reports` when a build needs them.
- Added `app_uri` as a required profile field. Profile setup now asks for the
  web app origin. `sift-cli agent doctor` treats a missing value as an error and
  prints the config command. `sift-cli mcp` does not expose tools for an
  incomplete profile. MCP links now use only the selected profile value. The
  MCP tool-list error includes the config command.
- Removed the project-scoped `install agent-skills` workflow. Existing Sift
  blocks in project `AGENTS.md` files must be removed manually because the new
  lifecycle is user-scoped.
- Reduced MCP list defaults from 200 records to 50 and the maximum from 1000 to
  200 to protect agent context windows.
- Removed the MCP server, built-in prompt, and agent skill pages from the
  bundled `sift-cli` documentation. The agent-facing tool surface is documented
  in the installed skill instead.
- Replaced the ASCII loading indicator with a braille spinner across the
  `agent` commands.

### Bug Fixes

- `create_report` now actually starts the rule evaluation for the new report
  instead of leaving it in the initial state.
- Various fixes surfaced during the internal alpha across the MCP data,
  reports, and rules tools.

## [v0.3.0] - July 13, 2026

### What's New

#### ULog imports
PX4 ULog (`.ulg`) files can now be imported directly with `sift-cli import ulog`. Every logged topic is imported one channel per field, named `<message>_<multi_id>.<field>`, and logged status text becomes `log_messages` channels. `--preview` lists the detected channels client-side before any upload. Flags cover timestamp anchoring for logs without a GPS time fix (`--relative-start-time`), importing info and parameter values as run metadata (`--info-key`, `--param-key`), and recoverable-error handling (`--parse-error-policy`).

#### Version command with update checks
`sift-cli --version` (`-V`) prints the installed version, checks GitHub for the latest release, and prints an install command when a newer version is available.

#### Help pages overhaul
Every `sift-cli` subcommand's help page is now grouped by section (required, optional, and per-topic headings like `Time` and `Advanced`) and ends with a concrete example. Running a subcommand with no arguments prints the help page instead of a bare error.

Two import commands changed shape so each help page only lists the flags that apply:
- `import hdf5 --schema <SCHEMA>` splits into `import hdf5 one-d`, `two-d`, and `compound`.
- `import parquet cpr --mode <MODE>` splits into `import parquet cpr single` and `multi`.

### Bug Fixes
- `sift-cli doc` now prints a clickable `http://localhost:<port>` URL when the server binds to `0.0.0.0`, instead of the non-clickable `tcp://0.0.0.0:<port>`.
- `sift-cli install completions` now carries a help description alongside `install agent-skills`.

### Full Changelog
- [ULog imports](https://github.com/sift-stack/sift/commit/535774fa9)
- [`sift-cli --version` with update checks](https://github.com/sift-stack/sift/commit/883386f5c)
- [Help pages overhaul](https://github.com/sift-stack/sift/pull/675)
- [Clickable localhost URL for doc server](https://github.com/sift-stack/sift/commit/32293298b)
- [Help description for `install completions`](https://github.com/sift-stack/sift/commit/0c8549d1e)

## [v0.3.0-alpha.1] - June 26, 2026

### What's New

#### MCP server is always bundled
The `mcp` Cargo feature is gone. Every `sift-cli` build ships with the MCP server, so users register it with one line (`claude mcp add sift -- sift-cli mcp`) without rebuilding from source.

#### Major MCP tool expansion
The MCP surface grew substantially. New domain-oriented routers cover:
- **Annotations:** `list_annotations`, `create_annotation`, `update_annotation`.
- **Reports:** `list_reports`, `list_report_rule_summaries`, `create_report`, `update_report`.
- **Rules:** `list_rules`, `list_rule_versions`, `create_rule`, `update_rule`, `archive_rule`, `unarchive_rule`.
- **Test results:** `list_test_reports`, `create_test_report`, `list_test_steps`, `count_test_steps`, `list_test_measurements`, `count_test_measurements`, `append_test_measurements`.
- **Docs:** `search_docs` searches and reads Sift's product documentation (`docs.siftstack.com`), including the REST/gRPC API reference, in a single tool with search and read modes.
- **Assets:** `update_asset` joins the existing `list_assets`.
- **Runs:** `update_run` joins the existing `list_runs`.

#### MCP rate-limiting and retries
Rate-limited responses from the Sift API are surfaced to the agent with structured guidance to back off, and a shared retry policy handles `Unavailable` errors with exponential backoff plus jitter across every service.

#### Imports return an Explore URL
Every `sift-cli import` subcommand now prints an Explore URL when the upload finishes, so users jump straight to the imported data. Configure the destination web app with the new `app_uri` field via `sift-cli config update --app-uri ...`.

#### Tool recipe guide
A new `rust/crates/sift_mcp/CLAUDE.md` documents the recipe for adding MCP tools end to end: anti-bloat gate, service/tool split, flat parameter rules, description structure, field-mask handling for updates, and the pre-merge checklist. The skill files (`SKILL.md` / `AGENTS.md`) were updated in lockstep.

### Bug Fixes
- Corrected wording on the `update_asset` tool description.

### Full Changelog
- [MCP rate-limiting](https://github.com/sift-stack/sift/commit/9eb9250a2)
- [Tool recipe guide and domain router refactor](https://github.com/sift-stack/sift/commit/679ca6fa2)
- [`update_asset` tool](https://github.com/sift-stack/sift/commit/2df0f1461)
- [CLI guidance in agent skills](https://github.com/sift-stack/sift/commit/46599f164)
- [Data review tools (annotations, reports, rules, runs)](https://github.com/sift-stack/sift/commit/5959d16ff)
- [Fix wording on `update_asset`](https://github.com/sift-stack/sift/commit/55a15dc0d)
- [Test results MCP tools](https://github.com/sift-stack/sift/commit/6945f8474)
- [Return Explore URL after import](https://github.com/sift-stack/sift/commit/25259625f)
- [Drop the `mcp` feature flag](https://github.com/sift-stack/sift/commit/09478bcd5)
- [Docs search MCP tool](https://github.com/sift-stack/sift/commit/af2d878bf)

## [v0.2.0] - June 12, 2026

### What's New

#### New file format support
HDF5 and TDMS files can now be imported directly. HDF5 supports one-dimensional, two-dimensional, and compound dataset layouts, with recursive traversal of nested groups, automatic detection of time-stamp channels, and a preview mode that surfaces any datasets the parser couldn't map.

#### Parquet import and export improvements
Parquet imports now run schema detection client-side, auto-detect the time column, and recognize `scpr` / `mcpr` shapes. Parquet is also added as an export format with its own CLI flags.

#### `ping` subcommand
A new `sift-cli ping` verifies your credentials and connectivity to Sift before running any data operations.

#### Bundled documentation
`sift-cli doc` serves a bundled documentation site locally, generated from mdBook sources committed alongside the CLI.

#### Quality of life
- `sift-cli config update` allows partial updates instead of requiring every field.
- Windows builds with the vendored HDF5 dependency now link against the static MSVC CRT.

### Bug Fixes
- Recursive and nested HDF5 imports now work correctly.

### Full Changelog
- [HDF5 imports](https://github.com/sift-stack/sift/commit/4b269e842)
- [TDMS imports](https://github.com/sift-stack/sift/commit/d1cbf7e26)
- [Parquet client-side schema detection](https://github.com/sift-stack/sift/commit/05fc9c383)
- [Parquet `scpr` / `mcpr` imports](https://github.com/sift-stack/sift/commit/2d07ea629)
- [Parquet export CLI args](https://github.com/sift-stack/sift/commit/ab47756b5)
- [Auto-detect Parquet time columns](https://github.com/sift-stack/sift/commit/633be813d)
- [HDF5 / TDMS preview shows time-stamp channels](https://github.com/sift-stack/sift/commit/34fb87146)
- [Surface skipped HDF5 channels in preview](https://github.com/sift-stack/sift/commit/a486bbde9)
- [`ping` subcommand](https://github.com/sift-stack/sift/commit/f4bf87852)
- [Bundled CLI documentation](https://github.com/sift-stack/sift/commit/ce4b30389)
- [Fix recursive HDF5 imports](https://github.com/sift-stack/sift/commit/565294bdf)
- [Allow partial config updates](https://github.com/sift-stack/sift/commit/f45a79708)
- [Static MSVC CRT for vendored HDF5 on Windows](https://github.com/sift-stack/sift/commit/a7c215a13)

## [v0.1.0] - March 23, 2026

### What's New

#### Calculated channel support
The CLI now supports calculated channels when exporting data.

#### Binary renamed to `sift-cli`
The final binary has been renamed from `sift_cli` to `sift-cli` to follow new standardized naming conventions across Sift tools.

### Full Changelog
- [Support calculated channels in CLI](https://github.com/sift-stack/sift/commit/666ad02d)
