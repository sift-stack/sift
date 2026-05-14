# Change Log
All notable changes to this project will be documented in this file.

This project adheres to [Semantic Versioning](http://semver.org/).

## [v0.9.0] - April 22, 2026
### What's New

v0.9.0 introduces a redesigned `SiftStream` builder API with explicit streaming mode selection,
a renamed send method with recoverable error types, a new `LiveStreamingOnly` transport mode,
gRPC status-code metrics, a scoped tracing dispatch for capturing internal logs, and updated
protobufs. This release contains several **breaking API changes** — see the sections below for
full details and a ready-to-use upgrade prompt for AI-assisted migration.

#### Breaking Changes

##### 1. Stepped Builder API — Explicit Mode Selection Required (PRs [#525](https://github.com/sift-stack/sift/pull/525), [#526](https://github.com/sift-stack/sift/pull/526))

The `SiftStreamBuilder` now uses a type-state stepped pattern. You must call a mode-selection
method (`.live_only()`, `.live_with_backups()`, or `.file_backup()`) before calling `.build()`.

**Removed from the public API:**
- `RecoveryStrategy` enum — all uses must be replaced with the appropriate mode builder
- `SiftStreamBuilder::build()` — no longer callable directly; a mode must be selected first
- `SiftStreamBuilder::build_file_backup()` — replaced by `.file_backup().build()`
- `LiveStreaming` type — split into `LiveStreamingOnly` and `LiveStreamingWithBackups`

**Before:**
```rust
use sift_stream::{RecoveryStrategy, SiftStreamBuilder};

// Default live+backup mode
let stream = SiftStreamBuilder::new(credentials)
    .ingestion_config(ingestion_config)
    .recovery_strategy(RecoveryStrategy::default())
    .build()
    .await?;

// File-backup-only mode
let stream = SiftStreamBuilder::new(credentials)
    .ingestion_config(ingestion_config)
    .recovery_strategy(RecoveryStrategy::default())
    .build_file_backup()
    .await?;
```

**After:**
```rust
use sift_stream::{SiftStreamBuilder, backup::DiskBackupPolicy};

// Live streaming with disk backups (replaces RecoveryStrategy::default())
let stream = SiftStreamBuilder::new(credentials)
    .ingestion_config(ingestion_config)
    .live_with_backups()
    .build()
    .await?;

// Live streaming only — single channel, no backups, lightest-weight option
let stream = SiftStreamBuilder::new(credentials)
    .ingestion_config(ingestion_config)
    .live_only()
    .build()
    .await?;

// File-backup-only mode (replaces build_file_backup())
let stream = SiftStreamBuilder::new(credentials)
    .ingestion_config(ingestion_config)
    .file_backup()
    .disk_backup_policy(DiskBackupPolicy {
        backups_dir: Some("/data/backups".into()),
        ..Default::default()
    })
    .build()
    .await?;
```

The new mode builders also accept per-mode configuration:
- `LiveOnlyBuilder::retry_policy(RetryPolicy)` — configure retry behavior
- `LiveWithBackupsBuilder::retry_policy(RetryPolicy)` — configure retry behavior
- `LiveWithBackupsBuilder::disk_backup_policy(DiskBackupPolicy)` — configure disk backups
- `LiveWithBackupsBuilder::checkpoint_interval(Duration)` — set checkpoint frequency
- `FileBackupBuilder::disk_backup_policy(DiskBackupPolicy)` — configure backup files

##### 2. `SiftStream<E, T>` Type Parameter No Longer Has a Default (PR [#526](https://github.com/sift-stack/sift/pull/526))

Any explicit type annotation on `SiftStream` must now include the transport type argument.

**Before:**
```rust
let stream: SiftStream<IngestionConfigEncoder> = /* ... */;
// or
let stream: SiftStream<IngestionConfigEncoder, LiveStreaming> = /* ... */;
```

**After (choose one):**
```rust
use sift_stream::{LiveStreamingOnly, LiveStreamingWithBackups, FileBackup, IngestionConfigEncoder};

let stream: SiftStream<IngestionConfigEncoder, LiveStreamingWithBackups> = /* ... */;
let stream: SiftStream<IngestionConfigEncoder, LiveStreamingOnly> = /* ... */;
let stream: SiftStream<IngestionConfigEncoder, FileBackup> = /* ... */;
```

##### 3. Send API Renamed and Clarified — Backpressure vs. Non-Blocking (PR [#519](https://github.com/sift-stack/sift/pull/519))

The send API is now organized around a consistent naming convention: async methods (`send`,
`send_requests`) apply backpressure by awaiting channel capacity, while their synchronous `try_`
counterparts (`try_send`, `try_send_requests`) return immediately without blocking.

The old `send_requests_nonblocking` has been renamed to `try_send_requests` to match this
convention. The new `try_` methods also return a richer error type that carries the undelivered
messages so callers can recover them.

**Full send API summary:**

| Method | Blocking | Accepts | Use when |
|---|---|---|---|
| `send(message).await` | Yes (backpressure) | Encodeable message | Default; let the stream pace you |
| `send_requests(requests).await` | Yes (backpressure) | Pre-encoded requests | High-throughput with `FlowBuilder`; let the stream pace you |
| `try_send(message)` | No | Encodeable message | Real-time loops where blocking is unacceptable |
| `try_send_requests(requests)` | No | Pre-encoded requests | Real-time loops with `FlowBuilder`; no blocking |

**`send_requests_nonblocking` renamed to `try_send_requests`:**

```rust
// BEFORE
sift_stream.send_requests_nonblocking(vec![request])?;

// AFTER — simple propagation
sift_stream.try_send_requests(vec![request])?;

// AFTER — recover unsent messages when the channel is full
match sift_stream.try_send_requests(vec![request]) {
    Ok(()) => {}
    Err(e) => {
        let unsent_messages = e.into_inner();
        // retry or buffer unsent_messages
    }
}
```

**Prefer `send` / `send_requests` when backpressure is acceptable** — they await channel
capacity and guarantee the message is accepted before returning. Use `try_send` /
`try_send_requests` only in contexts where blocking is truly unacceptable (e.g. a hard
real-time loop), and handle `TrySendError::Full` explicitly.

New error types exported from `sift_stream`:
- `TrySendError<T>` — returned by `try_send` / `try_send_requests`; carries the rejected payload
- `SendError<T>` — returned by `send_requests` on failure; carries undelivered messages
- `SiftStreamSendError` — returned by `send`; wraps a channel-closed or encode error
- `SiftStreamTrySendError` — returned by `try_send`; wraps encode errors and `TrySendError`

#### New Features

##### `LiveStreamingOnly` Mode (PR [#526](https://github.com/sift-stack/sift/pull/526))

A new lightweight streaming mode accessible via `.live_only()`. Uses a single bounded ingestion
channel with direct backpressure: `send` awaits until the ingestion task drains capacity.

```rust
let stream = SiftStreamBuilder::new(credentials)
    .ingestion_config(ingestion_config)
    .live_only()
    .build()
    .await?;
```

| Mode | Builder | Backpressure | Checkpointing | Disk Backup | Retries |
|---|---|---|---|---|---|
| `LiveStreamingOnly` | `.live_only()` | ingestion channel | No | No | Yes |
| `LiveStreamingWithBackups` | `.live_with_backups()` | backup channel | Yes | Yes | Yes |
| `FileBackup` | `.file_backup()` | write channel | No | Yes | N/A |

##### gRPC Status Code Metrics (PR [#530](https://github.com/sift-stack/sift/pull/530))

`SiftStreamMetricsSnapshot` now includes a `grpc_status_counts: [u64; 18]` field — one counter
per canonical gRPC status code (codes 0–16 by name: `ok`, `cancelled`, `unknown`, etc.) plus
one catch-all for any unrecognized status codes, tracking ingestion RPC completions by outcome.

##### Scoped Tracing Dispatch (PR [#534](https://github.com/sift-stack/sift/pull/534))

When the `tracing` feature is enabled, `sift-stream`'s internal background tasks now emit log
events through a scoped dispatch layered on top of your global tracing subscriber. This allows
internal stream logs to be captured independently and forwarded to Sift as telemetry without
interfering with your application's own logging.

Two new builder methods on `StreamConfigBuilder`:

```rust
use sift_stream::{SiftStreamBuilder, logging::LogLevel};

let stream = SiftStreamBuilder::new(credentials)
    .ingestion_config(ingestion_config)
    // Only forward Info-and-above events to Sift (default). Use LogLevel::Debug
    // or LogLevel::Trace for more verbose capture.
    .log_level_filter(LogLevel::Info)
    // Optionally supply a custom dispatch as the forwarding target instead of
    // capturing the current global dispatch at build time (tracing feature only).
    // .with_scoped_dispatch_base(my_dispatch)
    .live_with_backups()
    .build()
    .await?;
```

New types exported from `sift_stream::logging`:
- `LogLevel` — filter level enum (`Error`, `Warn`, `Info`, `Debug`, `Trace`); defaults to `Info`

##### `sift-stream-bindings` 0.3.0

The `sift-stream-bindings` crate has been bumped to 0.3.0 to reflect the breaking API changes
in `sift-stream` 0.9.0 (stepped builder, send rename, removed types). Bindings users should
apply the same migration steps described above.

#### AI-Assisted Migration Prompt (v0.8.2 → v0.9.0)

Copy and paste the following prompt to an AI coding agent to automate the upgrade:

```
You are upgrading a Rust project from sift_stream v0.8.2 to v0.9.0. Apply ALL of the following
changes precisely. Do not make any other modifications.

---

## 1. Update `Cargo.toml`

Change the sift_stream (and related sift_* crate) dependency version from `0.8.2` to `0.9.0`.
If using the workspace version in a workspace `Cargo.toml`, update `version = "0.8.2"` to
`version = "0.9.0"` under `[workspace.package]`.

---

## 2. Remove all imports of `RecoveryStrategy` and `LiveStreaming`

Delete any lines that import either of these types:
  - `use sift_stream::RecoveryStrategy;`
  - `use sift_stream::LiveStreaming;`
  - Variants of the above with path prefixes, e.g. `sift_stream::stream::RecoveryStrategy`

---

## 3. Replace all `SiftStreamBuilder` build call chains

**How to find every affected call site:**

Search for `.build()` and `.build_file_backup()` across all `.rs` files and inspect each
hit. Do not rely solely on the presence of `.recovery_strategy(...)` — it was optional in
v0.8.2, so many call sites will not contain it at all. Builder chains are often formatted
across multiple lines (each `.method(...)` on its own line), so treat any contiguous block
of `.`-chained method calls on a `SiftStreamBuilder` as a single chain to evaluate.

The reliable signal is the terminal call:
- `.build()` — if the receiver is (or returns) a `SiftStreamBuilder` or `StreamConfigBuilder`,
  this must be updated.
- `.build_file_backup()` — always must be updated.

Do **not** modify `.build()` calls that belong to other builder types (e.g. `RetryPolicyBuilder`,
`DiskBackupPolicyBuilder`, `RunForm`, etc.).

### Case A — Chain ends with `.build()`, with or without `.recovery_strategy(...)`
This is the most common case. The old default was live streaming with backups, so replace the
entire chain's terminal segment with `.live_with_backups().build()`:
```
// BEFORE (recovery_strategy present)
SiftStreamBuilder::new(credentials)
    .ingestion_config(ingestion_config)
    .recovery_strategy(RecoveryStrategy::default())
    .build()
    .await?

// BEFORE (recovery_strategy absent — equally valid in v0.8.2)
SiftStreamBuilder::new(credentials)
    .ingestion_config(ingestion_config)
    .build()
    .await?

// AFTER (both cases above become)
SiftStreamBuilder::new(credentials)
    .ingestion_config(ingestion_config)
    .live_with_backups()
    .build()
    .await?
```

### Case B — Chain ends with `.build_file_backup()`
Replace with `.file_backup().build()`. Move any `DiskBackupPolicy` that was inside a
`RecoveryStrategy` argument into a `.disk_backup_policy(...)` call on `FileBackupBuilder`:
```
// BEFORE
SiftStreamBuilder::new(credentials)
    .ingestion_config(ingestion_config)
    .recovery_strategy(RecoveryStrategy::default())
    .build_file_backup()
    .await?

// AFTER
use sift_stream::backup::DiskBackupPolicy;
SiftStreamBuilder::new(credentials)
    .ingestion_config(ingestion_config)
    .file_backup()
    .disk_backup_policy(DiskBackupPolicy::default())
    .build()
    .await?
```

### Case C — Any `RetryPolicy` or `DiskBackupPolicy` that was passed into `RecoveryStrategy`
These arguments move to the mode builder:
- Retry configuration → `.retry_policy(...)` called on `LiveOnlyBuilder` or `LiveWithBackupsBuilder`
- Disk backup configuration → `.disk_backup_policy(...)` called on `LiveWithBackupsBuilder` or `FileBackupBuilder`
- Checkpoint interval → `.checkpoint_interval(...)` called on the mode builder

---

## 4. Update `SiftStream` explicit type annotations

Find any explicit type annotations of the form:
  - `SiftStream<IngestionConfigEncoder>` (missing second type arg)
  - `SiftStream<IngestionConfigEncoder, LiveStreaming>`

Replace them based on the builder mode used to create that stream:
  - `.live_with_backups()` → `SiftStream<IngestionConfigEncoder, LiveStreamingWithBackups>`
  - `.live_only()` → `SiftStream<IngestionConfigEncoder, LiveStreamingOnly>`
  - `.file_backup()` → `SiftStream<IngestionConfigEncoder, FileBackup>`

Add the required imports:
```rust
use sift_stream::{LiveStreamingOnly, LiveStreamingWithBackups, FileBackup, IngestionConfigEncoder};
```

---

## 5. Update send call sites

### 5a. Rename `send_requests_nonblocking` → `try_send_requests`

Find every call to `.send_requests_nonblocking(...)` and rename it to `.try_send_requests(...)`.
The argument type is unchanged (an iterator/vec of pre-encoded requests).

The return type changed from `Result<()>` to `Result<(), TrySendError<Vec<Request>>>`. In most
cases a simple `?` propagation continues to work:
```rust
// BEFORE
stream.send_requests_nonblocking(messages)?;

// AFTER — simple propagation
stream.try_send_requests(messages)?;

// AFTER — recover unsent messages when the channel is full
match stream.try_send_requests(messages) {
    Ok(()) => {}
    Err(e) => {
        let unsent = e.into_inner();
        // retry or buffer unsent
    }
}
```

### 5b. Understand the send API naming convention

The send API now follows a consistent pattern — choose the right method based on whether you
want backpressure or non-blocking behavior:

| Method | Blocking | Use when |
|---|---|---|
| `send(message).await` | Yes (backpressure) | Default; let the stream pace your loop |
| `send_requests(requests).await` | Yes (backpressure) | Pre-encoded (`FlowBuilder`) batch; let the stream pace you |
| `try_send(message)` | No | Real-time loop where blocking is unacceptable |
| `try_send_requests(requests)` | No | Pre-encoded batch, no blocking |

Prefer `send` / `send_requests` unless you have a hard real-time constraint — they await
channel capacity and guarantee delivery before returning. If you switch from `send` to
`try_send` / `try_send_requests`, you must handle `TrySendError::Full` explicitly rather
than relying on the stream to apply backpressure.

If the new error types are referenced explicitly, add:
```rust
use sift_stream::{TrySendError, SendError};
```

---

## 6. Runtime mode selection — enum wrapper pattern

Because `SiftStream<E, T>` encodes the transport mode as a type parameter, you cannot store
different modes in the same variable directly. If the mode must be chosen at runtime (e.g. from
a config flag or CLI argument), wrap the concrete stream types in an enum and dispatch via
`match`:

```rust
use sift_stream::{
    FileBackup, IngestionConfigEncoder, LiveStreamingOnly, LiveStreamingWithBackups,
    SiftStream, SiftStreamBuilder, SiftStreamSendError,
    stream::Encodeable,
};
use sift_rs::ingest::v1::IngestWithConfigDataStreamRequest;
use sift_error::prelude::*;

// Concrete message type shared by all three transport modes.
type Msg = IngestWithConfigDataStreamRequest;

enum AnyStream {
    LiveOnly(SiftStream<IngestionConfigEncoder, LiveStreamingOnly>),
    LiveWithBackups(SiftStream<IngestionConfigEncoder, LiveStreamingWithBackups>),
    FileBackup(SiftStream<IngestionConfigEncoder, FileBackup>),
}

impl AnyStream {
    // All three modes use the same Transport::Message type, so the error type
    // unifies directly — no conversion or new wrapper type needed.
    pub async fn send<M>(&mut self, message: M) -> Result<(), SiftStreamSendError<Msg>>
    where
        M: Encodeable<Encoder = IngestionConfigEncoder, Output = Msg> + Send + Sync,
    {
        match self {
            AnyStream::LiveOnly(s) => s.send(message).await,
            AnyStream::LiveWithBackups(s) => s.send(message).await,
            AnyStream::FileBackup(s) => s.send(message).await,
        }
    }

    pub async fn finish(self) -> Result<()> {
        match self {
            AnyStream::LiveOnly(s) => s.finish().await,
            AnyStream::LiveWithBackups(s) => s.finish().await,
            AnyStream::FileBackup(s) => s.finish().await,
        }
    }
}

// Build the right variant based on a runtime decision:
async fn build_stream(use_backups: bool, use_file_only: bool) -> Result<AnyStream> {
    let builder = SiftStreamBuilder::new(credentials).ingestion_config(ingestion_config);

    if use_file_only {
        Ok(AnyStream::FileBackup(builder.file_backup().build().await?))
    } else if use_backups {
        Ok(AnyStream::LiveWithBackups(builder.live_with_backups().build().await?))
    } else {
        Ok(AnyStream::LiveOnly(builder.live_only().build().await?))
    }
}
```

Add additional `match` arms for any other `SiftStream` methods your code calls (e.g.
`attach_run`, `detach_run`, `try_send`, `try_send_requests`). Each arm simply delegates
to the same method on the inner stream. If only a subset of modes is needed, omit the
unused variants.

---

## 7. Verify the build compiles

Run `cargo build` (or `cargo check`). Fix any remaining compilation errors that reference the
removed types `RecoveryStrategy`, `LiveStreaming`, `build_file_backup`, or `send_requests_nonblocking`.

---

That completes the v0.8.2 → v0.9.0 migration.
```

### Full Changelog
- [Add tracing scoped dispatch to capture sift-stream logs](https://github.com/sift-stack/sift/pull/534)
- [Use flow descriptor/builder for sift-stream metrics](https://github.com/sift-stack/sift/pull/531) *(internal)*
- [Add grpc status codes to sift-stream metrics, better logs](https://github.com/sift-stack/sift/pull/530)
- [Improve sift-stream mode and builder documentation](https://github.com/sift-stack/sift/pull/528)
- [Additional test coverage for sift-stream](https://github.com/sift-stack/sift/pull/527) *(internal)*
- [Add LiveStreamingOnly and LiveStreamingWithBackups](https://github.com/sift-stack/sift/pull/526)
- [Introduce stepped builder API for SiftStream](https://github.com/sift-stack/sift/pull/525)
- [Extract tasks module into submodules](https://github.com/sift-stack/sift/pull/522) *(internal)*
- [sift stream send api updates](https://github.com/sift-stack/sift/pull/519)
- [update and regen protobufs](https://github.com/sift-stack/sift/pull/535)
- [update and regen protobufs](https://github.com/sift-stack/sift/pull/516)
- [update and regen protobufs](https://github.com/sift-stack/sift/pull/514)

## [v0.8.2] - March 20, 2026
### What's New
Updates `sift_connect` to use the `tls-ring` feature from `tonic` instead of `tls-aws-lc`.

### Full Changelog
- [Use tls-ring instead of tls-aws-lc feature in tonic](https://github.com/sift-stack/sift/commit/1da4864f0ad31ddfa2f408d6aad80a994e7b9bd8)

## [v0.8.1] - March 20, 2026
### What's New
Fixes a regression in  `sift_connect` prevending a TLS provider to be selected when creating a `SiftChannel`.

### Full Changelog
- [Fix sift-connect tonic build feature regression](https://github.com/sift-stack/sift/commit/86d0b630fe2bf93397b942128f1596769800aaaa)

## [v0.8.0] - March 17, 2026
### What's New
Updates all crate dependencies to their latest versions, as well as proto build tooling.

### Full Changelog
- [Dependency updates, including Rust proto build tools](https://github.com/sift-stack/sift/commit/eaf5c7011813e5a75aba3f9d09beef1e7ffa60ab)

## [v0.7.3] - January 16, 2026
### What's New
Various bug fixes to the backup manager system, and the file backup mode of `SiftStream`.

### Full Changelog
- [Fix bug with metrics task and file backup mode](https://github.com/sift-stack/sift/commit/40e53550db3075872612a8eafbfbf16c376f8b52)
- [Backup manager bug fixes](https://github.com/sift-stack/sift/commit/d8d07fec171cf3c0d790181cb5496e97ce358ecf)

## [v0.7.2] - January 16, 2026
### What's New
Added support for webpki TLS certificates.

## [v0.7.1] - January 8, 2026
### What's New
#### Sift proto file descriptors now included
The v0.7.1 release includes proto descriptors in generated code.

## [v0.7.0] - January 2, 2026
### What's New
#### SiftStream ergonomic, performance, and more!
The v0.7.0 release includes all changes included in the RC releases. Noteable
changes are listed here; for full details see all 0.7.0 RC releases below.

* Removal of Deprecated Recovery Strategies
* Backup Directory Structure Improvements
* Optimizations and performance improvements
* Independent gRPC Connections for Streaming
* SiftStream will stream it's own metrics to Sift for improved visibility
* Improved Checkpoint Message Tracking
* Added FlowDescriptors and FlowBuilders for a new/higher-performance approach to build streaming requests.
* Support for File-Backup Only Mode
* New "Retry" Capabilities for Unary Sift gRPC Calls


## [v0.7.0-rc.12] - December 24, 2025
### What's New
#### New "Retry" Capabilities for Unary Sift gRPC Calls
A new struct `Retrying` has been added that provides convenient retry capabilities
to Sift Unary gRPC calls. This new retry capability has also been added internally
to `SiftStream`'s Unary gRPC calls, improving behavior when transient errors are
returned during setup, adding new flows, or attaching new runs to `SiftStream`.

### Full Changelog
- [Add generic gRPC retry wrapper, use in sift_stream](https://github.com/sift-stack/sift/commit/cf08dac2b9c4f246bf11ec543f5eda33010f8578)

## [v0.7.0-rc.11] - December 23, 2025
### What's New
#### Improved SiftStream File Path Verification
A minor fix improves how and where SiftStream verifies paths exist, allowing errors
or directory creation errors to be returned when building SiftStream.

### Full Changelog
- [Improve file path validation for spawned tasks on init](https://github.com/sift-stack/sift/commit/6da335e9b3a5d1a8f797cdeee648edd8ebbb7a1a)

## [v0.7.0-rc.10] - December 16, 2025
### What's New
#### SiftStream File-Backup Only Mode Create Parent Directories
A minor fix to create all parent directories for file-backup only mode, as well as ignoring
any "already exists" errors.

### Full Changelog
- [Create file-backup mode directories, ignore exists error](https://github.com/sift-stack/sift/commit/b69fb1c6491c65be8bc5409d9c3ce15769a981d9)

## [v0.7.0-rc.9] - December 16, 2025
### What's New
#### SiftStream File-Backup Only Mode Directory Bug Fix
A minor fix to save the backup files in the specified directory.

### Full Changelog
- [Fix SiftStream file-backup mode directory](https://github.com/sift-stack/sift/commit/fe041085243ca27601b327b5fd2235b6f8399d5a)

## [v0.7.0-rc.8] - December 5, 2025
### What's New
#### SiftStream Support for File-Backup Only Mode
`SiftStream` has a new "mode" of operation where data will _only_ be written to backup files. This
can be useful in a few different situations, such as more "offline" environments with limited network
bandwidth, as well as scenarios where uploading data after recording is preferred (ex: CI systems
where data is only needed/desired if a testcase fails). Though this mode of operation does require
connectivity to Sift in order to synchronize ingestion configurations to ensure backed up data can
be re-ingested later.

This new mode of operation can be selected through `SiftStreamBuilder` calls. And re-upload can be
performed with the latest release of `sift-cli` [Releases](https://github.com/sift-stack/sift/releases).

### Full Changelog
- [Adds a new SiftStream mode for only backup file writes](https://github.com/sift-stack/sift/commit/b8f13b273e37939530109c531d9f8a2953ae868b)

## [v0.7.0-rc.7] - November 26, 2025
### What's New
#### SiftStream Internally Uses `FlowBuilder`
Internally, `SiftStream` now uses the `FlowBuilder` to construct streaming gRPC request messages
which helps improve performance by avoiding repeated map allocations.

#### SiftStream `add_flow_config` Improvements
Adding new flows to `SiftStream` should now be more performant, internally performing concurrent
requests.

### Full Changelog
- [Update sift-stream-bindings to support FlowDescriptor](https://github.com/sift-stack/sift/commit/ab91090c2dc16df6f7d1c82bddf1d4fd21b953e8f)
- [Improve add_flow_config, prevent send_impl error](https://github.com/sift-stack/sift/commit/9d7e03e907b9ab22009338f235f52dd8e46f95c9)

## [v0.7.0-rc.6] - November 24, 2025
### What's New
#### SiftStream APIs to Utilize `FlowDescriptor` and `FlowBuilder`
Two new APIs have been added to allow use of the `FlowDescriptor` and `FlowBuilder` structs added
previously.

### Full Changelog
- [Get FlowDescriptor, send_requests_nonblocking](https://github.com/sift-stack/sift/commit/db0fc7c4829fd4e8f4c50573213b6ea9534dc1ff)


## [v0.7.0-rc.5] - November 24, 2025
### What's New
#### SiftStream FlowDescriptors and FlowBuilders
The `FlowDescriptor` and `FlowBuilder` structs have been added as a new way to send data with `SiftStream`.
The `FlowDescriptor` while similar to the `FlowConfig`, represents the minimum required information to
create the protobuf object required by the ingestion APIs in Sift. It allows flexibility in how "channels"
can be uniquely identified within that flow, with the most performant option to utilize the channel's index
to directly set the value, bypassing potential bottlenecks such as string allocations or hash operations.

#### SiftStream Flow-Config Cache Updates
When initializing `SiftStream`, if no initial flow configs are provided, `SiftStream` will populate it's cache
with all known flows configs from Sift. This can help improve workflows that require restarting `SiftStream`, such
as when deploying new containers or processes.

### Full Changelog
- [Add FlowDescriptor and FlowBuilder to improve performance](https://github.com/sift-stack/sift/commit/833f0927d15a1fc0c6aef50e521f0a84c621b3e9)
- [Improve how sift-stream handles the flow config cache](https://github.com/sift-stack/sift/commit/79ed6f9dd6a65ffabd66bf0d9e17d2c957719eb0)


## [v0.7.0-rc.4] - November 19, 2025
### What's New
#### SiftStream Improved Checkpoint Message Tracking
The checkpoint system with `SiftStream` has been updated and improved to remove edge cases resulting from
slow backup file writing. Overall, the implementation is now more explicit in identifying which messages
are contained in which checkpoints.

#### SiftStream Metrics Streaming
The metrics within `SiftStream` will now be streamed to Sift to aid in visibility into `SiftStream` itself as
well as improve debuggability. This functionality can be adjusted, as well as disabled, through the
`SiftStreamBuilder`.

#### SiftStream Performance Improvements
Multiple performance improvements have been made that should reduce both CPU and memory usage of `SiftStream`.

### Full Changelog
- [Add get_flows to SiftStream](https://github.com/sift-stack/sift/commit/efb5db261c4c5a19e863fbef814ddc2573706048)
- [Improve checkpoint message tracking](https://github.com/sift-stack/sift/commit/24c4a8a99bbd6138adce11fd25633a86d52716a1)
- [Add streaming metrics to Sift](https://github.com/sift-stack/sift/commit/704a6f676caff3c291f7257cc51cdf94b990af91)
- [SiftStream related performance improvements](https://github.com/sift-stack/sift/commit/359012d2956bf938539761a5c3d070c5b45f9384)

## [v0.7.0-rc.3] - November 12, 2025
### What's New
#### SiftStream Independent gRPC Connections for Streaming
In order to prevent backup file re-ingestion from potentially starving out message ingestion through the main
ingestion task, the re-ingestion task and primary ingestion task now have their own independent gRPC connections
and underlying TCP connections. Previously, these streams were using multiplexing over the same connection.

#### SiftStream Ergonomic Improvements
Various minor changes have been made to improve the usability (and testability) of sift_stream and related types.

#### SiftStream Dependency Migration
Shared code used for sift_stream backup files has moved to its own crate to support a `sift_cli` subcommand to re-ingest
those files via the command-line.

### Full Changelog
- [Independent sift connections for ingestion and re-ingestion](https://github.com/sift-stack/sift/commit/8fb3606c084172066a7a807dfb10f6778b7df254)
- [Make Flow fields public to improve testability](https://github.com/sift-stack/sift/commit/b63f29e18fa73fae0e9708575903b1c1f5454603)
- [Use ToString to create a new Flow instead of AsRef<str>](https://github.com/sift-stack/sift/commit/cce4f12be0b0609846a4d1d99d2ddb40d458d9ef)
- [Add backup file ingestion to sift_cli, add sift_pbfs crate](https://github.com/sift-stack/sift/commit/2b60b906fce3cb602f555fb842cb076dac40ae1f)

## [v0.7.0-rc.2] - November 3, 2025
### What's New
#### Improvements For Constrained Environments
Compression support has been added for streaming data into Sift. This can be useful in low-bandwidth or network
constrained environments. Since compression does add overhead to streaming, it is not recommended for high
throughput streaming systems.

Additionally, changes have been made to ensure data is streamed to Sift even when writing backup files lags
behind ingestion.

### Full Changelog
- [Add gzip compression support in sift-stream ingestion](https://github.com/sift-stack/sift/commit/68b08c86c506f1657fae5d7175c6a234f0e21983)

## [v0.7.0-rc.1] - October 24, 2025
### What's New
#### SiftStream Optimizations
The algorithm used in `SiftStream::message_to_ingest_req` to convert a `Flow` into the necessary gRPC request format was substantially
optimized, improving the performance of sending data to Sift.

#### SiftStream Backup Directory Structure
The directory structure used for backup files has been made more human-friendly, organizing backup files by asset name, and run when
available. Additionally, the backup files now include the client-key as the prefix to help identify the associated ingestion config.

#### Removal of Deprecated Recovery Strategies
Previously deprecated recovery strategies `RecoveryStrategy::RetryWithInMemoryBackups` and `RecoveryStrategy::RetryWithDiskBackups`
have been removed.

#### SiftStream Metrics
Metrics have been added throughout `SiftStream`, providing visibility into the performance of the client. These metrics are behind
the feature `metrics-unstable`. These metrics are considered "unstable" and subject to be dramatically changed or refactored in
future releases.

#### Sift CLI
A new command-line interface has been developed to streamline common Sift functions such as importing and exporting data. Additional
capabilities will be added in upcoming releases.

#### SiftStream Refactor for Improved Performance
The internals of `SiftStream` have been refactored to improve the async `await` behavior of `SiftStream::send`. In previous versions
this async call could end up awaiting for an extended period of time, thereby preventing the caller from streaming more data. This
behavior has been corrected, and streaming should now be entirely non-blocking, even when encounting network slow-downs.

Additionally, the refactor allowed simplifications to be made throughout `SiftStream` for improved reliability and performance.

### Full Changelog
- [Optimize flow conversion into gRPC request](https://github.com/sift-stack/sift/commit/bf1c2681392b40409bc9033faf0fd0ad37fdc60b)
- [Improve backup directory names and structure](https://github.com/sift-stack/sift/commit/2c88ca78a090ef2dc4c5fc4bd0425ca0e9bcd559)
- [Remove deprecated recovery strategies](https://github.com/sift-stack/sift/commit/39cfddbfc80c6e8ec7d0a8961d5b5dca83a663ba)
- [SiftStream Metrics](https://github.com/sift-stack/sift/commit/e28c2fe8b2f480563ace745b5d0480bac72b457a)
- [Update Protos](https://github.com/sift-stack/sift/commit/8e0a0e0e239fc4357d14f27816770be3f3fc631e)
- [Correct rust CI and fixes for failing tests](https://github.com/sift-stack/sift/commit/ed9b20dca3f4b2184e90d389373dbaca86345bb5)
- [Add missing build targets for sift-stream-bindings](https://github.com/sift-stack/sift/commit/6abe1d7dfb3d51f3ad787e930aaa5905b7a5e93b)
- [Fix for aarch64 sift-stream build](https://github.com/sift-stack/sift/commit/aa256d39ae758babb8636e53e639b0f62f26d042)
- [Sift CLI](https://github.com/sift-stack/sift/commit/7ac5aa1dd48e71803778c0d4c77a316b4f715788)
- [Refactor SiftStream internals for better nonblocking behavior](https://github.com/sift-stack/sift/commit/3ca9852961fd7b6d142e9977e2f53f1c55cf0319)

## [v0.6.0] - September 19, 2025
### What's New
#### SiftStream Async Backup Manager
A new recovery strategy `RecoveryStrategy::RetryWithBackups` has been developed for SiftStream which is designed to be a more robust backup method than the existing `RetryWithDiskBackups` method.
- Uses rolling backup files (default to 500 MiB and an unlimited file count). By default these files are removed once a checkpoint passes, indicating that Sift has successfully recieved all incoming data, but can also be retained indefinitely with the `retain_backups` option
- Backup file ingestion if the checkpoint fails is performed asynchronously and will not block live ingestion
- The existing `RecoveryStrategy::RetryWithDiskBackups` is being marked deprecated due to the new recovery strategy being considered a more robust method
- `RecoveryStrategy::RetryWithInMemoryBackups` is also being deprecated due to the lack of a known use case
#### SiftStream Asset/Run Tags and Metadata
Users now have the ability to add both tags and metadata when specifying a run, or for an asset during initilization of SiftStream.
- `RunForm` now includes the `metadata` field. Metadata can be easily defined using the `sift_rs::metadata` macro. 
- `SiftStreamBuilder` now includes `add_asset_metadata` and `add_asset_tags`
- See the [SiftStream example](https://github.com/sift-stack/sift/blob/main/rust/crates/sift_stream/examples/quick-start/main.rs) for how tags and metadata can be added.

### Full Changelog
- [Sift stream async backup manager](https://github.com/sift-stack/sift/pull/307)
- [Run and Asset Tags and Metadata](https://github.com/sift-stack/sift/pull/319)
- Protobuf updates [#310](https://github.com/sift-stack/sift/pull/310) [#311](https://github.com/sift-stack/sift/pull/311) [#314](https://github.com/sift-stack/sift/pull/314) [#316](https://github.com/sift-stack/sift/pull/316)

## [v0.5.0] - August 14, 2025

- [Add ability to attach and detach runs to SiftStream](https://github.com/sift-stack/sift/pull/293)
- [Add better stream error handling to SiftStream to avoid checkpoint misses](https://github.com/sift-stack/sift/pull/292)

## [v0.4.2] - July 17, 2025

- [Additional network hiccup resiliency for SiftStream](https://github.com/sift-stack/sift/pull/272)


## [v0.4.1] - June 30, 2025

- [SiftStream now forces a checkpoint if it detects that a checkpoint is overdue. If a checkpoint isn't acknowledge by Sift then SiftStream internal state is reinitialized and backups reingested](https://github.com/sift-stack/sift/pull/258).
- [SiftStream::send now emits a warning if it encounters a message that doesn't match any locally cached flows and the data is transmitted regardless. Users will need to manually confirm that this data was transmitted](https://github.com/sift-stack/sift/pull/259).

## [v0.4.0] - June 25, 2025

- [Include a unique ID for each SiftStream instance that gets emitted with each log for improved traceability](https://github.com/sift-stack/sift/pull/250)
  - `SiftStream` also will log a heartbeat at a regular interval when `tracing` feature flag is enabled.

## [v0.3.0] - June 12, 2025

- Users can now initialize `SiftStreamBuilder` from an existing instance of `SiftChannel`
- Users can now call `SiftStream::add_new_flow` to generate a new flow that wasn't initially configured on their ingestion config.
- Fixed a bug where the disk-based-backups manager would return an error if the backup-directory that needed to be created had intermediate directories that didn't yet exist.
- Fixed a bug where changing the asset-name without changing the client key on the ingestion config didn't return an error - assuming the ingestion config with that key already exists.

All of these changes can be found in this [pull-request](https://github.com/sift-stack/sift/pull/229).

## [v0.2.1] - April 28, 2025

- Downgraded `chrono` from `0.4.40` to `0.4.39` due to function naming collisions introduced
  in [arrow](https://github.com/apache/arrow-rs/issues/7196).

## [v0.2.0] - April 22, 2025

- Method to decode backup file is now public, giving users the ability to write a program that can reingest their backup files manually.
- `SiftStreamBuilder` can now specify a run by run ID.
- When attaching a run using RunForm, Optional fields that are None will not cause corresponding fields to zero out in Sift.
- Users can now send raw protobuf ingestion requests through `SiftStream`.
- Allows users to get a reference to the underlying run attached to `SiftStream` if it exists.

## [v0.1.0] - April 1, 2025

Official `v0.1.0` release of the following crates:
- [sift_rs](https://github.com/sift-stack/sift/tree/main/rust/crates/sift_rs)
- [sift_stream](https://github.com/sift-stack/sift/tree/main/rust/crates/sift_stream)
- [sift_connect](https://github.com/sift-stack/sift/tree/main/rust/crates/sift_connect)
- [sift_error](https://github.com/sift-stack/sift/tree/main/rust/crates/sift_error)

Users who were originally using `sift_rs@v0.1.0-rc.2` will need to migrate how they establish gRPC connections
to Sift.

Originally, the way you would establish a gRPC connection to Sift would look something like this:

```rust
use sift_rs::{
    gen::sift::ping::v1::{ping_service_client::PingServiceClient, PingRequest},
    grpc::{use_sift_channel, SiftChannelConfig},
};
use std::{env, error::Error};

#[tokio::main]
async fn main() {
    let uri = env::var("SIFT_URI").unwrap();
    let apikey = env::var("SIFT_API_KEY").unwrap();
    let grpc_channel = use_sift_channel(SiftChannelConfig { uri, apikey })?;
    todo!("use grpc_channel");
```

Now you would do the following:

```rust
use sift_rs::{
    Credentials, SiftChannelBuilder,
    ping::v1::{PingRequest, ping_service_client::PingServiceClient},
};
use std::env;

#[tokio::main]
async fn main() {
    let credentials = Credentials::Config {
        apikey: env::var("SIFT_API_KEY").unwrap(),
        uri: env::var("SIFT_URI").unwrap(),
    };
    let grpc_channel = SiftChannelBuilder::new(credentials).build().unwrap();
    todo!("use grpc_channel");
}
```

See the [sift_connect](https://docs.rs/sift_connect/latest/sift_connect/) documentation for more details.

## [v0.1.0-rc.2] - November 12, 2024

Official release candidate for `v0.1.0` of `sift_rs` which contains compiled protocol buffers
as well as gRPC utilities for ergonomic setup.
