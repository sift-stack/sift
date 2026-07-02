# Change Log
All notable changes to `sift-cli` will be documented in this file.

This project adheres to [Semantic Versioning](http://semver.org/).

## [Unreleased]

### What's New

#### ULog imports
PX4 ULog (`.ulg`) files can now be imported directly with `sift-cli import ulog`. Every logged topic is imported one channel per field, named `<message>_<multi_id>.<field>`, and logged status text becomes `log_messages` channels. `--preview` lists the detected channels client-side before any upload. Flags cover timestamp anchoring for logs without a GPS time fix (`--relative-start-time`), importing info and parameter values as run metadata (`--info-key`, `--param-key`), and recoverable-error handling (`--parse-error-policy`).

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
