#![cfg_attr(docsrs, feature(doc_cfg))]
//! The `sift_stream` crate is primarily focused on streaming telemetry to Sift in a robust manner.
//!
//! Here are some features highlights:
//! - Builtin retries with default or custom retry policies in the case of a Sift outage or a
//!   client-side network outage.
//! - Periodic checkpointing to confirm that all data within a particular period has been received
//!   by Sift.
//! - Optional automated backups to mitigate data-loss in the case of misc. outages.
//! - Optional tracing/logging to monitor the health of your stream and view various ingestion
//!   performance metrics.
//!
//! Users of this crate will only have to initialize a single instance of [SiftStream] which they
//! would then use for the entirety of data ingestion for a given asset.
//!
//! Comprehensive examples can be found in the
//! [examples](https://github.com/sift-stack/sift/tree/main/rust/crates/sift_stream/examples/)
//! directory of this crate.
//!
//! ## Quick-start
//!
//! ```ignore
//! // Define the schema of your telemetry
//! let ingestion_config = IngestionConfigForm {
//!     asset_name: "MarsRover0".into(),
//!     client_key: "mars-rover0-ingestion-config-v1".into(),
//!     flows: vec![FlowConfig {
//!         name: "robotic-arm".into(),
//!         channels: vec![ChannelConfig {
//!             name: "joint-angle-encoder".into(),
//!             description: "measures the angular position of the arm’s joints".into(),
//!             data_type: ChannelDataType::Double.into(),
//!             unit: "degrees".into(),
//!             ..Default::default()
//!         }],
//!     }],
//! };
//!
//! // Initialize your Sift Stream (IngestionConfigMode - streams to Sift)
//! let mut sift_stream = SiftStreamBuilder::new(credentials)
//!     .ingestion_config(ingestion_config.clone())
//!     .build()
//!     .await?;
//!
//! let flow = Flow::new(
//!     "robotic-arm",
//!     TimeValue::now(),
//!     &[ChannelValue::new("joint-angle-encoder", 7.2_f64)],
//! );
//!
//! // Send telemetry to Sift
//! sift_stream.send(flow).await?;
//!
//! // Gracefully terminate your stream
//! sift_stream.finish().await?;
//!
//! // Alternatively, initialize in FileBackupMode (only writes to backup files)
//! use sift_stream::stream::builder::RecoveryStrategy;
//! let mut backup_stream = SiftStreamBuilder::new(credentials)
//!     .ingestion_config(ingestion_config)
//!     .recovery_strategy(RecoveryStrategy::default_retry_policy_with_backups())
//!     .build_file_backup()
//!     .await?;
//!
//! // Send telemetry to backup files (not to Sift)
//! backup_stream.send(flow).await?;
//!
//! // Gracefully terminate and flush backup files
//! backup_stream.finish().await?
//! ```
//!
//! ## Stream Modes
//!
//! SiftStream supports two different modes of operation, each with different use cases:
//!
//! ### IngestionConfigMode
//!
//! [IngestionConfigMode] is the default streaming mode that sends telemetry data directly to Sift
//! via gRPC streams. This mode supports:
//! - Real-time streaming to Sift
//! - Optional disk backups for data recovery
//! - Checkpointing to confirm data receipt
//! - Retry policies for handling network outages
//!
//! This mode is used when you call [SiftStreamBuilder::build] and is suitable for most use cases
//! where you want to stream data to Sift in real-time.
//!
//! ### FileBackupMode
//!
//! [FileBackupMode] is a specialized mode that only writes telemetry data to backup files on disk
//! without streaming to Sift. This mode is useful for:
//! - CI/CD workflows where data is only needed/uploaded to Sift if a test fails
//! - Situations where network connectivity is unreliable
//! - Creating backup files for later ingestion
//!
//! This mode is used when you call [SiftStreamBuilder::build_file_backup]. Data written in this
//! mode can be ingested into Sift later using separate tooling.
//!
//! **Note**: When using `FileBackupMode`, you must configure a [RecoveryStrategy::RetryWithBackups]
//! recovery strategy, as the backup policy configuration is required for file management.
//!
//! ## Ingestion Configs
//!
//! Both stream modes use [ingestion-config-based streaming](https://docs.siftstack.com/docs/ingestion/ingestion-config-based-streaming),
//! which requires users to define the schema of their telemetry before they start telemetering data.
//! The key parts of an ingestion config are:
//! - **Asset name**: The name of the asset associated with the data that will be streamed.
//! - **Client key**: An arbitrary user-sourced identifier that uniquely identifies the ingestion
//!   config; this can be used to achieve client-side versioning e.g. `mars-rover0-sim-v1`.
//! - **Flows configs**: A list of flow configurations. Simply put, a **flow configuration** is a named group
//!   of channels that are often telemetered together; a **flow** is a single message that contains
//!   a list of channel values that share a common timestamp. When sending a flow to Sift, it is
//!   expected that the **flow** has a corresponding **flow configuration**.
//!
//! The following is an example of a valid ingestion config for the `MarsRover0` asset:
//!
//! ```ignore
//! let ingestion_config = IngestionConfigForm {
//!     asset_name: "MarsRover0".into(),
//!     client_key: "mars-rover0-ingestion-config-v1".into(),
//!     flows: vec![
//!         FlowConfig {
//!             name: "robotic-arm".into(),
//!             channels: vec![ChannelConfig {
//!                 name: "joint-angle-encoder".into(),
//!                 description: "measures the angular position of the arm’s joints".into(),
//!                 data_type: ChannelDataType::Double.into(),
//!                 unit: "degrees".into(),
//!                 ..Default::default()
//!             }],
//!         },
//!         FlowConfig {
//!             name: "navigation-system".into(),
//!             channels: vec![
//!                 ChannelConfig {
//!                     name: "gps-receiver".into(),
//!                     description: "measures latitude and longitude".into(),
//!                     data_type: ChannelDataType::Int32.into(),
//!                     unit: "degrees".into(),
//!                     ..Default::default()
//!                 },
//!                 ChannelConfig {
//!                     name: "imu".into(),
//!                     description: "measures acceleration and angular velocity".into(),
//!                     data_type: ChannelDataType::Float.into(),
//!                     unit: "m/s^2, deg/s".into(),
//!                     ..Default::default()
//!                 },
//!             ],
//!         },
//!     ],
//! };
//! ```
//!
//! Here is an example of how streaming data into Sift might look using this ingestion config:
//!
//! ```ignore
//! let mut sift_stream = SiftStreamBuilder::new(credentials)
//!     .ingestion_config(ingestion_config)
//!     .build()
//!     .await?;
//!
//! // Send data for the `robotic-arm` flow.
//! sift_stream.send(Flow::new(
//!     "robotic-arm",
//!     TimeValue::now(),
//!     &[ChannelValue::new("joint-angle-encoder", 7.2_f64)],
//! )).await?;
//!
//! // Send data for the `navigation-system` flow. Notice that the order of the channels
//! // don't need to be in the same order as they are specified in the `navigation-system`
//! // flow configuration.
//! sift_stream.send(Flow::new(
//!     "navigation-system",
//!     TimeValue::now(),
//!     &[
//!         ChannelValue::new("imu", 9.7_f32),
//!         ChannelValue::new("gps-receiver", 10_i32),
//!     ],
//! )).await?;
//!
//! // Send partial data for the `navigation-system` flow. This totally fine.
//! sift_stream.send(Flow::new(
//!     "navigation-system",
//!     TimeValue::now(),
//!     &[ChannelValue::new("imu", 9.7_f32)]
//! )).await?;
//!
//! // Gracefully terminate your stream
//! sift_stream.finish().await?
//! ```
//!
//! ### Modifying Ingestion Configs
//!
//! Ingestion configs should be re-used whenever possible. Simply reusing the same [IngestionConfigForm] form should
//! allow proper re-use of your ingestion config. If you need to update your ingestion config, it
//! has to be done in a backwards compatible manner. The following changes are considered backwards
//! compatible:
//!
//! - Adding a new [FlowConfig]
//! - Changing the name of an existing [FlowConfig] (this will simply create a new one)
//!   - If you change the name of a [FlowConfig] you are also able to edit its [ChannelConfig]s
//!     safely.
//! - Removing an entire [FlowConfig] from the list of flow configs
//!
//! **Important Note**: Changing an existing [FlowConfig] in any way is considered a backwards
//! incompatible change. E.g. say we were to change our `joint-angle-encoder`'s type in the
//! `robotic-arm` flow configuration from a double to an int32 - if this were the case we will end
//! up with the following error when calling [SiftStreamBuilder::build]:
//!
//! ```text
//! Error: failed to initialize Sift stream
//!
//! Caused by:
//!     [IncompatibleIngestionConfigChange]: flow(s) with name 'robotic-arm' exist but their channel configs do not match what the user specified
//!     
//!     [cause]:
//!        - incompatible change to ingestion config
//!     
//!     [help]:
//!        - Did you modify an existing flow? Try updating the 'client_key' of `sift_stream::IngestionConfigForm`
//! ```
//!
//! In this situation, simply updating the client key to be something like `mars-rover0-ingestion-config-v2` will create a new
//! ingestion config which will allow users to proceed normally.
//!
//! ### Summary
//!
//! In summary, re-use an existing ingestion config as much as possible. The following changes can
//! be made without updating the client key:
//!
//! - Adding a new [FlowConfig]
//! - Changing the name of an existing [FlowConfig] (this will simply create a new one)
//!   - If you change the name of a [FlowConfig] you are also able to edit its [ChannelConfig]s
//!     safely.
//! - Removing an entire [FlowConfig] from the list of flow configs
//!
//! Anything that falls outside of that will require changing the client-key.
//!
//! ## Retry Policy
//!
//! At the time of writing this crate, [tonic](https://docs.rs/tonic/latest/tonic/)
//! [doesn't natively support gRPC retries](https://github.com/hyperium/tonic/issues/733).
//! `sift_stream`, however has its own internal mechanism to handle retries to handle the following
//! cases:
//!
//! - Client-side network outages
//! - Transient Sift outages
//! - Transient errors from Sift's gRPC service
//! - Transient errors that may arise from load balancers
//!
//! Retries are not enabled by default, but users would enable is as part of [SiftStream]
//! initialization:
//!
//! ```ignore
//! let mut sift_stream = SiftStreamBuilder::new(credentials)
//!     .ingestion_config(ingestion_config)
//!     .recovery_strategy(RecoveryStrategy::default())
//!     .build()
//!     .await?;
//! ```
//!
//! This will initialize a [SiftStream] with retries configured with recommended defaults,
//! however, users are able to set their own custom retry policies. For more information on
//! that see [SiftStreamBuilder::recovery_strategy], [RecoveryStrategy], [RetryPolicy], and [DiskBackupPolicy].
//!
//! ## Checkpoints
//!
//! Checkpointing enables clients to receive periodic acknowledgements from Sift, confirming that
//! all data up to the moment the checkpoint was requested has been received. Checkpointing happens
//! periodically and is enabled by default to occur are a regular interval, with the default
//! interval being [stream::builder::DEFAULT_CHECKPOINT_INTERVAL]. Users can, however, specify
//! their own custom checkpoint interval:
//!
//! ```ignore
//! let mut sift_stream = SiftStreamBuilder::new(credentials)
//!     .ingestion_config(ingestion_config)
//!     .checkpoint_interval(Duration::from_secs(30))
//!     .build()
//!     .await?;
//! ```
//!
//! For more information see [SiftStreamBuilder::checkpoint_interval].
//!
//! **Important Note**: Bear in mind that checkpointing does introduce a bit of overhead, as
//! [SiftStream::send] will block on receiving an acknowledgement from Sift and reinitialize the inner
//! gRPC stream, so very small checkpoint intervals are not recommended.
//!
//! ### Concluding a stream
//!
//! To conclude a stream and receive the final checkpoint acknowledgement from Sift, it is
//! important that users call [SiftStream::finish] at the end of their stream, otherwise the stream
//! may terminate prematurely resulting in data-loss at the tail-end.
//!
//! ## Backups
//!
//! Streaming data to Sift is generally very robust and stable, however, due to the asynchronous
//! nature of gRPC streaming, if an error occurs while the user is calling [SiftStream::send]
//! between checkpoints there is no guarantee that the data sent at the moment the error was
//! triggered on the other end successfully reached Sift.
//!
//! While [checkpointing](#checkpoints) gives clients assurance that all data has been received up
//! to a certain point, checkpointing alone doesn't protect against data loss between checkpoints.
//!
//! To protect against data-loss `sift_stream` offers a backup mechanism providing async backups
//! and reingestion of data.
//!
//! This backup method is disabled by default as it introduces some overhead, but can be enabled
//! like so:
//!
//! ```ignore
//! // Async Backups
//! let mut sift_stream = SiftStreamBuilder::new(config)
//!     .ingestion_config(ingestion_config)
//!     .recovery_strategy(RecoveryStrategy::default_retry_policy_with_backups())
//!     .build()
//!     .await?;
//! ```
//!
//! This example initializes backup strategies with the recommended defaults. This includes
//! the default retry policy, the backups directory, and a max individual backup file size with unlimited
//! rolling backup files, which are deleted once a successful checkpoint is observed. If users wish to
//! configure their own backup settings, see [RecoveryStrategy].
//!
//! ### Retry with Backups
//!
//! The [RecoveryStrategy::RetryWithBackups] writes messages passed to [SiftStream::send] to rolling backup
//! files in a buffered manner. Once the file size is reached for a given file, that backup is closed out and
//! the next file is created. If the maximum file count is reached, a checkpoint will be forced. Once a
//! checkpoint is triggered, if passed, the backup file(s) are deleted unless the [DiskBackupPolicy] specifies
//! to retain backups. If a checkpoint fails, the backup files(s) are sent to a separate ingestion task which
//! re-ingests each file into Sift.
//!
//! Backup file re-ingestion is performed on each file in the same order as the data was initially sent, with an
//! indefinite number of retries on a file until a successful ingestion response is recieved by Sift. Files are
//! deleted as specified per the [DiskBackupPolicy] retention policy only once re-ingestion has been confirmed.
//!
//! ### Data Integrity
//!
//! **Important Note**: This section only pertains to the disk-based-backup strategy.
//!
//! The backup files are periodically written to and synced. Each chunk of data written to the backup includes a checksum
//! computed from the chunk itself. When chunks are read back into memory, their checksums are recomputed and compared against
//! the stored values. If a mismatch is detected, the affected chunk and all subsequent chunks are considered corrupt and will be ignored.
//! See the [tracing](#tracing) section for details on enabling logs that notify users when this occurs.
//!
//! ### Guarantees
//!
//! The current backup strategy implementations will protect against data-loss but they do
//! potentially come at a performance cost depending on several variables. The default
//! configurations should satisfy most use-cases, however, users are encouraged to provide their
//! own custom configurations based on their baseline message and byte-rates if they notice
//! backups causing performance issues. Please refer to the [tracing](#tracing) section for
//! information on how to get performance metrics.
//!
//! ## Tracing
//!
//! `sift_stream` only comes with the `tracing` feature flag which is enabled by default. With the
//! `tracing` feature flag users can observe the health of their Sift stream as well as
//! performance metrics that are conducive to debugging. The following is an example of the types
//! of traces users would see with `tracing` enabled and with `RUST_LOG` set to the following
//! `RUST_LOG=sift_stream=info`
//!
//! ```text
//! 2025-03-23T01:33:45.193457Z  INFO an existing ingestion config was found with the provided client-key ingestion_config_id="181bd784-827f-4f3f-a045-ef6b4df6505f"
//! 2025-03-23T01:33:45.335279Z  INFO created new run run_id="2acff183-9cb4-44f4-a811-1c76d64ce77f" run_name="millenium-falcon-ep4-1742693624989"
//! 2025-03-23T01:33:45.335801Z  INFO Sift streaming successfully initialized
//! 2025-03-23T01:34:45.339499Z  INFO initiating checkpoint
//! 2025-03-23T01:34:45.345700Z  INFO stream_duration="60s" messages_processed=91823712 message_rate="1530395.2 messages/s" bytes_processed="1.176 GiB" byte_rate="19.6 MiB/s"
//! 2025-03-23T01:34:46.347922Z  INFO checkpoint acknowledgement received from Sift - resuming stream
//! 2025-03-23T01:34:46.348091Z  INFO successfully initialized a new stream to Sift
//! ```
//!
//! A quick-start example:
//!
//! Install `sift_stream`:
//!
//! ```text
//! cargo add sift_stream
//! ```
//!
//! Then install `tracing_subscriber` with the `fmt` and `env-filter` feature flags:
//!
//! ```text
//! cargo add tracing_subscriber --features fmt,env-filter
//! ```
//!
//! Then configure a tracing subscriber before you start streaming like so:
//!
//! ```ignore
//! tracing_subscriber::fmt()
//!     .with_target(false)
//!     .with_env_filter(tracing_subscriber::filter::EnvFilter::from_default_env())
//!     .init();
//! ```
//!
//! Now when you execute your program, you can control `sift_stream` severity levels using the
//! `RUST_LOG` environment variable. Here are a few examples:
//! - `RUST_LOG=sift_stream=debug`
//! - `RUST_LOG=sift_stream=info`
//!
//! See [tracing-subscriber](https://crates.io/crates/tracing-subscriber) and
//! [tracing](https://docs.rs/tracing/latest/tracing/) for further details.
//!
//! If you do not wish to enable the `tracing` feature, then simply install `sift_stream` without
//! the flag like so:
//!
//! ```text
//! cargo add sift_stream --no-default-features
//! ```
//!
//! ## Metrics
//! SiftStream records metrics related to the performance and operational status separately for each SiftStream instance.
//! While some metrics are provided through [tracing](#tracing), users may expose the ability to access these metrics
//! by enabled the optional `metrics-unstable` feature flag.
//!
//! Metrics are currently considered an unstable feature, and future updates may break the existing metrics API.
//!
//! When the `metrics-unstable` feature flag is enabled, users may currently access metrics through one of two methods:
//! - [SiftStream::get_metrics_snapshot] returns a [SiftStreamMetricsSnapshot]
//! - Enable the light weight HTTP metrics server using [metrics::start_metrics_server], which exposes the `/` and `/metrics`
//!   endpoints, providing a JSON formatted struct of each sift-stream-id and its [SiftStreamMetricsSnapshot]
//!
//! Snapshots of the metrics are taken at any time the user calls [SiftStream::get_metrics_snapshot] or sends a GET request to the metrics
//! server endpoints. Metrics are internally updated atomically, and calls to get metric snapshots are non-blocking to SiftStream
//! operaration.
//!
//! ## Tokio
//!
//! Because [tonic](https://docs.rs/tonic/latest/tonic/) is an underlying dependency, the
//! [tokio](https://docs.rs/tokio/latest/tokio/) asynchronous runtime is required, otherwise
//! attempts to use this crate will result in a panic at the level of [SiftStreamBuilder::build].
//!
//! This crate is compatible with both the current and multi-threaded Tokio runtimes. Performance
//! is expected to be better generally using the multi-threaded runtime.
//!
//! ## Feature flags
//!
//! - `default`: Includes the `tracing` feature flag
//! - `tracing`: Enables logging of SiftStream through the Tracing crate. See [tracing](#tracing)
//! - `metrics-unstable`: Enables the ability for the user to access SiftStream metrics from each [SiftStream] instance,
//!   or through a light-weight HTML metrics server, if enabled. See [metrics](#metrics)

/// Concerned with streaming telemetry into Sift.
pub mod stream;
pub use sift_rs::{
    common::r#type::v1::ChannelDataType,
    ingestion_configs::v2::{ChannelConfig, FlowConfig},
};
pub use stream::{
    RetryPolicy, SiftStream,
    builder::{IngestionConfigForm, RecoveryStrategy, RunForm, SiftStreamBuilder},
    channel::{ChannelValue, Value},
    flow::{ChannelIndex, FlowBuilder, FlowDescriptor, FlowDescriptorBuilder},
    mode::{
        file_backup::FileBackup,
        ingestion_config::{Flow, IngestionConfigEncoder, LiveStreaming},
    },
    time::TimeValue,
};

/// Re-export IngestionConfigEncoder as IngestionConfigMode for backwards compatibility.
pub use IngestionConfigEncoder as IngestionConfigMode;

/// Concerned with backing up data as its streamed to Sift and backups accessible.
pub mod backup;
pub use backup::DiskBackupPolicy;

pub use sift_connect::grpc::{Credentials, SiftChannel};

/// Concerned with metrics for SiftStream
pub mod metrics;

#[cfg(feature = "metrics-unstable")]
pub use metrics::SiftStreamMetricsSnapshot;

#[cfg(test)]
mod test;
