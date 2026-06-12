# Change Log
All notable changes to `sift-cli` will be documented in this file.

This project adheres to [Semantic Versioning](http://semver.org/).

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
