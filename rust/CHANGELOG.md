# Change Log
All notable changes to this project will be documented in this file.

This project adheres to [Semantic Versioning](http://semver.org/).

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
