# Change Log
All notable changes to this project will be documented in this file.

This project adheres to [Semantic Versioning](http://semver.org/).

## [v0.4.1] - July 17, 2025

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
