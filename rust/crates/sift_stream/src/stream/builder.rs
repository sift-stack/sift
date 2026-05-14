use super::{
    RetryPolicy, SiftStream, helpers,
    mode::{
        file_backup::FileBackup, live_only::LiveStreamingOnly,
        live_with_backups::LiveStreamingWithBackups,
    },
    run::{load_run_by_form, load_run_by_id},
};
use std::collections::HashMap;

mod config_loader;
use crate::{
    FlowDescriptor,
    backup::{disk::DiskBackupPolicy, sanitize_name},
    logging::{LogEvent, LogLevel},
    metrics::SiftStreamMetrics,
    stream::{
        mode::ingestion_config::IngestionConfigEncoder,
        tasks::{
            CONTROL_CHANNEL_CAPACITY, DATA_CHANNEL_CAPACITY, LiveOnlyTaskConfig,
            LiveWithBackupsTaskConfig, RecoveryConfig,
        },
    },
};
use config_loader::load_ingestion_config;
use sift_connect::{Credentials, SiftChannel, SiftChannelBuilder};
use sift_error::prelude::*;
use sift_rs::{
    ingestion_configs::v2::{FlowConfig, IngestionConfig},
    metadata::v1::MetadataValue,
    ping::v1::{PingRequest, ping_service_client::PingServiceClient},
    runs::v2::Run,
};
use std::{sync::Arc, time::Duration};
use uuid::Uuid;

/// Capacity of the bounded log event channel created per stream.
const LOG_CHANNEL_CAPACITY: usize = 512;

/// The default checkpoint interval (1 minute) to use if left unspecified.
pub const DEFAULT_CHECKPOINT_INTERVAL: Duration = Duration::from_secs(60);

/// The default metrics streaming interval (500 milliseconds) to use if left unspecified.
pub const DEFAULT_METRICS_STREAMING_INTERVAL: Duration = Duration::from_millis(500);

/// A form to create a new ingestion config or retrieve an existing one by `client_key`.
///
/// The `client_key` is an arbitrary, user-defined identifier that must be unique within your
/// organization. It acts as a stable handle for the schema: passing the same `client_key` again
/// retrieves the existing ingestion config rather than creating a duplicate. Use it for
/// client-side versioning (e.g. `"mars-rover0-ingestion-config-v2"`).
///
/// See the [top-level documentation](crate#ingestion-configs) for compatibility rules and
/// guidance on when to bump the `client_key`.
#[derive(Debug, Clone)]
pub struct IngestionConfigForm {
    /// Name of the asset whose telemetry this ingestion config describes.
    pub asset_name: String,
    /// Unique, user-defined key that identifies this ingestion config within the organization.
    pub client_key: String,
    /// Ordered list of flow configurations that define the telemetry schema.
    pub flows: Vec<FlowConfig>,
}

/// A form to create a new run or retrieve an existing one by `client_key`.
///
/// Used with [StreamConfigBuilder::attach_run]. If a run with the given `client_key` already
/// exists, any non-`None` fields in this form will be applied as updates in Sift. `Option`
/// fields left as `None` are left unchanged on the existing run.
#[derive(Debug, Clone, Default)]
pub struct RunForm {
    /// Display name for the run.
    pub name: String,
    /// Unique, user-defined key that identifies this run. Used to upsert: creates a new run or
    /// retrieves the existing one with this key.
    pub client_key: String,
    /// Optional human-readable description. If `None` on an existing run, the existing
    /// description is preserved.
    pub description: Option<String>,
    /// Optional list of tags to associate with the run.
    pub tags: Option<Vec<String>>,
    /// Optional metadata key-value pairs to associate with the run.
    pub metadata: Option<Vec<MetadataValue>>,
}

/// Entry-point for constructing a [SiftStream]. Call [SiftStreamBuilder::ingestion_config] to
/// supply the schema and advance to [StreamConfigBuilder] where you choose the streaming mode.
///
/// ### Panic
///
/// Because [tonic](https://docs.rs/tonic/latest/tonic/) is an underlying dependency, the
/// [tokio](https://docs.rs/tokio/latest/tokio/) asynchronous runtime is required, otherwise
/// attempts to call `.build()` on any mode builder will panic.
pub struct SiftStreamBuilder {
    credentials: Option<Credentials>,
    channel: Option<SiftChannel>,
    enable_tls: bool,
}

impl SiftStreamBuilder {
    /// Initializes a new builder from [Credentials].
    pub fn new(credentials: Credentials) -> Self {
        SiftStreamBuilder {
            credentials: Some(credentials),
            channel: None,
            enable_tls: true,
        }
    }

    /// Initializes a new builder from an existing [SiftChannel].
    ///
    /// IMPORTANT:
    ///
    /// It is preferred that credentials are provided so that independent gRPC channels can
    /// be created and used. Cloning the channel results in multiplexing the gRPC requests
    /// over a single connection, which is not desirable for backup re-ingestion which may
    /// starve out primary ingestion.
    pub fn from_channel(channel: SiftChannel) -> Self {
        SiftStreamBuilder {
            credentials: None,
            channel: Some(channel),
            enable_tls: true,
        }
    }

    /// Disables TLS. Useful for testing. Ignored if [SiftStreamBuilder::from_channel] is used.
    pub fn disable_tls(mut self) -> Self {
        self.enable_tls = false;
        self
    }

    /// Sets the ingestion config and advances to [StreamConfigBuilder] where shared options
    /// (run, asset tags/metadata) and the streaming mode can be configured.
    pub fn ingestion_config(self, form: IngestionConfigForm) -> StreamConfigBuilder {
        StreamConfigBuilder {
            credentials: self.credentials,
            channel: self.channel,
            enable_tls: self.enable_tls,
            ingestion_config: form,
            asset_tags: None,
            asset_metadata: None,
            run: None,
            run_id: None,
            log_level_filter: LogLevel::default(),
            #[cfg(feature = "tracing")]
            scoped_dispatch_base: None,
            disable_scoped_dispatch: false,
        }
    }
}

/// Holds shared configuration (run, asset tags/metadata) and provides mode selection.
///
/// Created by [SiftStreamBuilder::ingestion_config]. Call one of the mode selectors to advance
/// to a mode-specific builder:
///
/// - [StreamConfigBuilder::live_only] — single channel, direct backpressure, no disk backups
/// - [StreamConfigBuilder::live_with_backups] — dual channel with checkpointing and disk backups
/// - [StreamConfigBuilder::file_backup] — writes directly to disk, no network ingestion
pub struct StreamConfigBuilder {
    pub(crate) credentials: Option<Credentials>,
    pub(crate) channel: Option<SiftChannel>,
    pub(crate) enable_tls: bool,
    pub(crate) ingestion_config: IngestionConfigForm,
    pub(crate) asset_tags: Option<Vec<String>>,
    pub(crate) asset_metadata: Option<Vec<MetadataValue>>,
    pub(crate) run: Option<RunForm>,
    pub(crate) run_id: Option<String>,
    /// Minimum log level forwarded to Sift via the telemetry layer. Defaults to `Info`.
    pub(crate) log_level_filter: LogLevel,
    /// A pre-built dispatch to use as the forwarding target for the scoped dispatch.
    /// If `None`, the current global dispatch is captured at build time.
    #[cfg(feature = "tracing")]
    pub(crate) scoped_dispatch_base: Option<tracing::Dispatch>,
    /// When `true`, the scoped dispatch is not created for this stream.
    /// Set by the internal metrics sub-stream builder to prevent recursive event capture.
    pub(crate) disable_scoped_dispatch: bool,
}

impl StreamConfigBuilder {
    /// Associates a run with this stream. Data sent will be associated with the run.
    /// If both `attach_run` and `attach_run_id` are called, `attach_run_id` takes precedence.
    pub fn attach_run(mut self, run: RunForm) -> Self {
        self.run = Some(run);
        self
    }

    /// Associates a run by ID. Takes precedence over [StreamConfigBuilder::attach_run].
    pub fn attach_run_id(mut self, run_id: &str) -> Self {
        self.run_id = Some(run_id.into());
        self
    }

    /// Creates or replaces the asset tags. Call this to set tags; omit it to leave tags unchanged.
    pub fn add_asset_tags(mut self, tags: Vec<String>) -> Self {
        self.asset_tags = Some(tags);
        self
    }

    /// Creates or replaces the asset metadata. Call this to set metadata; omit it to leave
    /// metadata unchanged.
    pub fn add_asset_metadata(mut self, metadata: Vec<MetadataValue>) -> Self {
        self.asset_metadata = Some(metadata);
        self
    }

    /// Sets the minimum log level forwarded to Sift via the internal telemetry layer.
    ///
    /// Events below this level are still emitted to the normal tracing subscriber (e.g. the
    /// user's console logger) but will not be captured for streaming to Sift.
    /// Defaults to [`LogLevel::Info`](crate::logging::LogLevel::Info).
    pub fn log_level_filter(mut self, level: LogLevel) -> Self {
        self.log_level_filter = level;
        self
    }

    /// Provide a custom dispatch as the base for the scoped dispatch composition.
    ///
    /// The builder will layer `SiftTelemetryLayer` on top of this dispatch when constructing
    /// the scoped subscriber used by background tasks. If not called, the current global
    /// dispatch is captured at build time and used as the forwarding target.
    ///
    /// Use this to include additional layers (e.g. your own telemetry) alongside
    /// sift_stream's internal telemetry layer.
    #[cfg(feature = "tracing")]
    pub fn with_scoped_dispatch_base(mut self, dispatch: tracing::Dispatch) -> Self {
        self.scoped_dispatch_base = Some(dispatch);
        self
    }

    /// Disable scoped dispatch entirely for this stream.
    ///
    /// Used internally when constructing the metrics sub-stream to prevent
    /// `SiftTelemetryLayer` from capturing the metrics infrastructure's own events.
    pub(crate) fn disable_scoped_dispatch(mut self) -> Self {
        self.disable_scoped_dispatch = true;
        self
    }

    /// Selects [`LiveStreamingOnly`](crate::LiveStreamingOnly) mode: a single bounded ingestion
    /// channel with direct backpressure and retry. No checkpointing, no disk backups.
    ///
    /// [`send`](crate::SiftStream::send) awaits when the ingestion channel is full. Capacity is
    /// configured via [`LiveOnlyBuilder::ingestion_data_channel_capacity`].
    pub fn live_only(self) -> LiveOnlyBuilder {
        LiveOnlyBuilder {
            base: self,
            enable_compression_for_ingestion: false,
            metrics_streaming_interval: Some(DEFAULT_METRICS_STREAMING_INTERVAL),
            ingestion_data_channel_capacity: DATA_CHANNEL_CAPACITY,
            control_channel_capacity: CONTROL_CHANNEL_CAPACITY,
            retry_policy: RetryPolicy::default(),
        }
    }

    /// Selects [`LiveStreamingWithBackups`](crate::LiveStreamingWithBackups) mode: dual-channel
    /// architecture with checkpointing, retry, and disk backups.
    ///
    /// [`send`](crate::SiftStream::send) awaits when the **backup channel** is full; the
    /// ingestion channel uses force-send and never blocks. Capacities are configured via
    /// [`LiveWithBackupsBuilder::backup_data_channel_capacity`] and
    /// [`LiveWithBackupsBuilder::ingestion_data_channel_capacity`].
    pub fn live_with_backups(self) -> LiveWithBackupsBuilder {
        LiveWithBackupsBuilder {
            base: self,
            checkpoint_interval: DEFAULT_CHECKPOINT_INTERVAL,
            retry_policy: RetryPolicy::default(),
            disk_backup_policy: DiskBackupPolicy::default(),
            enable_compression_for_ingestion: false,
            metrics_streaming_interval: Some(DEFAULT_METRICS_STREAMING_INTERVAL),
            ingestion_data_channel_capacity: DATA_CHANNEL_CAPACITY,
            backup_data_channel_capacity: DATA_CHANNEL_CAPACITY,
            control_channel_capacity: CONTROL_CHANNEL_CAPACITY,
        }
    }

    /// Selects [`FileBackup`](crate::FileBackup) mode: writes all data to rolling disk files
    /// without network ingestion.
    ///
    /// [`send`](crate::SiftStream::send) awaits when the write channel is full. Capacity is
    /// configured via [`FileBackupBuilder::backup_data_channel_capacity`].
    /// `disk_backup_policy.backups_dir` must be set before calling
    /// [`FileBackupBuilder::build`].
    pub fn file_backup(self) -> FileBackupBuilder {
        FileBackupBuilder {
            base: self,
            disk_backup_policy: DiskBackupPolicy::default(),
            backup_data_channel_capacity: DATA_CHANNEL_CAPACITY,
            control_channel_capacity: CONTROL_CHANNEL_CAPACITY,
            metrics_streaming_interval: Some(DEFAULT_METRICS_STREAMING_INTERVAL),
        }
    }
}

/// Builder for [LiveStreamingOnly] mode.
///
/// Created by [StreamConfigBuilder::live_only]. Call [LiveOnlyBuilder::build] to finalize.
pub struct LiveOnlyBuilder {
    base: StreamConfigBuilder,
    enable_compression_for_ingestion: bool,
    metrics_streaming_interval: Option<Duration>,
    ingestion_data_channel_capacity: usize,
    control_channel_capacity: usize,
    retry_policy: RetryPolicy,
}

impl LiveOnlyBuilder {
    /// Sets whether gzip compression is enabled for ingestion.
    pub fn enable_compression_for_ingestion(mut self, enable: bool) -> Self {
        self.enable_compression_for_ingestion = enable;
        self
    }

    /// Sets the interval at which [SiftStreamMetrics](crate::metrics::SiftStreamMetrics) are
    /// streamed to Sift. Defaults to [DEFAULT_METRICS_STREAMING_INTERVAL]. Pass `None` to disable.
    pub fn metrics_streaming_interval(mut self, interval: Option<Duration>) -> Self {
        self.metrics_streaming_interval = interval;
        self
    }

    /// Sets the capacity of the ingestion channel — the bounded queue between the caller and the
    /// gRPC ingestion task.
    ///
    /// [`send`](crate::SiftStream::send) awaits when this channel is full, applying backpressure
    /// to the caller. Defaults to [`DATA_CHANNEL_CAPACITY`](crate::stream::tasks::DATA_CHANNEL_CAPACITY).
    /// Increase for high-throughput producers to absorb bursts; decrease to reduce memory usage
    /// at the cost of earlier backpressure.
    pub fn ingestion_data_channel_capacity(mut self, capacity: usize) -> Self {
        self.ingestion_data_channel_capacity = capacity;
        self
    }

    /// Sets the capacity of the control channel — the broadcast channel used to send internal
    /// signals such as shutdown.
    ///
    /// Defaults to [`CONTROL_CHANNEL_CAPACITY`](crate::stream::tasks::CONTROL_CHANNEL_CAPACITY).
    /// Most users do not need to change this.
    pub fn control_channel_capacity(mut self, capacity: usize) -> Self {
        self.control_channel_capacity = capacity;
        self
    }

    /// Sets the retry policy. Defaults to [RetryPolicy::default].
    pub fn retry_policy(mut self, policy: RetryPolicy) -> Self {
        self.retry_policy = policy;
        self
    }

    /// Performs setup and returns a [SiftStream] configured for `LiveStreamingOnly`.
    pub async fn build(self) -> Result<SiftStream<IngestionConfigEncoder, LiveStreamingOnly>> {
        let setup = setup_common(self.base).await?;

        let task_config = LiveOnlyTaskConfig {
            session_name: setup.session_name,
            sift_stream_id: setup.sift_stream_id,
            setup_channel: setup.setup_channel,
            ingestion_channel: setup.ingestion_channel,
            metrics: setup.metrics.clone(),
            enable_compression_for_ingestion: self.enable_compression_for_ingestion,
            ingestion_data_channel_capacity: self.ingestion_data_channel_capacity,
            control_channel_capacity: self.control_channel_capacity,
            metrics_streaming_interval: self.metrics_streaming_interval,
            retry_policy: self.retry_policy,
            #[cfg(feature = "tracing")]
            scoped_dispatch: setup.scoped_dispatch,
            log_rx: setup.log_rx,
        };

        SiftStream::new_live_only(
            setup.ingestion_config,
            setup.flows_by_name,
            setup.run,
            task_config,
            setup.metrics,
        )
        .await
    }
}

/// Builder for [LiveStreamingWithBackups] mode.
///
/// Created by [StreamConfigBuilder::live_with_backups]. Call [LiveWithBackupsBuilder::build] to finalize.
pub struct LiveWithBackupsBuilder {
    base: StreamConfigBuilder,
    checkpoint_interval: Duration,
    retry_policy: RetryPolicy,
    disk_backup_policy: DiskBackupPolicy,
    enable_compression_for_ingestion: bool,
    metrics_streaming_interval: Option<Duration>,
    ingestion_data_channel_capacity: usize,
    backup_data_channel_capacity: usize,
    control_channel_capacity: usize,
}

impl LiveWithBackupsBuilder {
    /// Sets the interval between checkpoints. Defaults to [DEFAULT_CHECKPOINT_INTERVAL].
    pub fn checkpoint_interval(mut self, duration: Duration) -> Self {
        self.checkpoint_interval = duration;
        self
    }

    /// Sets the retry policy. Defaults to [RetryPolicy::default].
    pub fn retry_policy(mut self, policy: RetryPolicy) -> Self {
        self.retry_policy = policy;
        self
    }

    /// Sets the disk backup policy. Defaults to [DiskBackupPolicy::default].
    ///
    /// If `disk_backup_policy.backups_dir` is `Some`, disk backups are enabled.
    /// If `None` (the default), disk backups are disabled and only retries are used.
    pub fn disk_backup_policy(mut self, policy: DiskBackupPolicy) -> Self {
        self.disk_backup_policy = policy;
        self
    }

    /// Sets whether gzip compression is enabled for ingestion.
    pub fn enable_compression_for_ingestion(mut self, enable: bool) -> Self {
        self.enable_compression_for_ingestion = enable;
        self
    }

    /// Sets the interval at which [SiftStreamMetrics](crate::metrics::SiftStreamMetrics) are
    /// streamed to Sift. Defaults to [DEFAULT_METRICS_STREAMING_INTERVAL]. Pass `None` to disable.
    pub fn metrics_streaming_interval(mut self, interval: Option<Duration>) -> Self {
        self.metrics_streaming_interval = interval;
        self
    }

    /// Sets the capacity of the ingestion channel — the bounded queue between the backup manager
    /// and the gRPC ingestion task.
    ///
    /// This channel uses force-send: when full, the **oldest buffered message is evicted** (not
    /// the caller) to preserve message freshness. Defaults to
    /// [`DATA_CHANNEL_CAPACITY`](crate::stream::tasks::DATA_CHANNEL_CAPACITY). Smaller values
    /// increase the likelihood of message eviction under load.
    pub fn ingestion_data_channel_capacity(mut self, capacity: usize) -> Self {
        self.ingestion_data_channel_capacity = capacity;
        self
    }

    /// Sets the capacity of the backup channel — the primary bounded queue between the caller
    /// and the backup manager task.
    ///
    /// [`send`](crate::SiftStream::send) awaits when this channel is full, applying backpressure
    /// to the caller. Defaults to [`DATA_CHANNEL_CAPACITY`](crate::stream::tasks::DATA_CHANNEL_CAPACITY).
    /// Increase for high-throughput producers to absorb bursts; decrease to reduce memory usage
    /// at the cost of earlier backpressure.
    pub fn backup_data_channel_capacity(mut self, capacity: usize) -> Self {
        self.backup_data_channel_capacity = capacity;
        self
    }

    /// Sets the capacity of the control channel — the broadcast channel used to send internal
    /// signals such as shutdown and checkpoint triggers.
    ///
    /// Defaults to [`CONTROL_CHANNEL_CAPACITY`](crate::stream::tasks::CONTROL_CHANNEL_CAPACITY).
    /// Most users do not need to change this.
    pub fn control_channel_capacity(mut self, capacity: usize) -> Self {
        self.control_channel_capacity = capacity;
        self
    }

    /// Performs setup and returns a [SiftStream] configured for `LiveStreamingWithBackups`.
    pub async fn build(
        self,
    ) -> Result<SiftStream<IngestionConfigEncoder, LiveStreamingWithBackups>> {
        // Build the reingestion channel before consuming self.base.
        // With credentials we create a dedicated channel; with a shared channel we clone it.
        let reingestion_channel_pre = if let Some(ref creds) = self.base.credentials {
            let mut builder = SiftChannelBuilder::new(creds.clone());
            if self.base.enable_tls {
                builder = builder.use_tls(true);
            }
            Some(builder.build()?)
        } else {
            None
        };

        let setup = setup_common(self.base).await?;

        let reingestion_channel =
            reingestion_channel_pre.unwrap_or_else(|| setup.setup_channel.clone());

        let (backups_directory, backups_prefix) = {
            let mut dir_name = sanitize_name(&setup.asset_name);
            if let Some(run) = setup.run.as_ref() {
                dir_name.push_str(&format!("/{}", sanitize_name(&run.name)));
            }
            (dir_name, setup.ingestion_config.client_key.clone())
        };

        let recovery_config = RecoveryConfig {
            backups_directory,
            backups_prefix,
            backup_policy: self.disk_backup_policy,
        };

        let task_config = LiveWithBackupsTaskConfig {
            session_name: setup.session_name,
            sift_stream_id: setup.sift_stream_id,
            setup_channel: setup.setup_channel,
            ingestion_channel: setup.ingestion_channel,
            reingestion_channel,
            metrics: setup.metrics.clone(),
            checkpoint_interval: self.checkpoint_interval,
            enable_compression_for_ingestion: self.enable_compression_for_ingestion,
            retry_policy: self.retry_policy,
            recovery_config,
            control_channel_capacity: self.control_channel_capacity,
            ingestion_data_channel_capacity: self.ingestion_data_channel_capacity,
            backup_data_channel_capacity: self.backup_data_channel_capacity,
            metrics_streaming_interval: self.metrics_streaming_interval,
            #[cfg(feature = "tracing")]
            scoped_dispatch: setup.scoped_dispatch,
            log_rx: setup.log_rx,
        };

        SiftStream::new_live_with_backups(
            setup.ingestion_config,
            setup.flows_by_name,
            setup.run,
            task_config,
            setup.metrics,
        )
        .await
    }
}

/// Builder for [FileBackup] mode.
///
/// Created by [StreamConfigBuilder::file_backup]. Call [FileBackupBuilder::build] to finalize.
///
/// Requires [FileBackupBuilder::disk_backup_policy] with a `backups_dir` set.
pub struct FileBackupBuilder {
    base: StreamConfigBuilder,
    disk_backup_policy: DiskBackupPolicy,
    backup_data_channel_capacity: usize,
    control_channel_capacity: usize,
    metrics_streaming_interval: Option<Duration>,
}

impl FileBackupBuilder {
    /// Sets the disk backup policy. `disk_backup_policy.backups_dir` must be `Some` or
    /// [FileBackupBuilder::build] will return an error.
    pub fn disk_backup_policy(mut self, policy: DiskBackupPolicy) -> Self {
        self.disk_backup_policy = policy;
        self
    }

    /// Sets the capacity of the write channel — the bounded queue between the caller and the
    /// background file-writer task.
    ///
    /// [`send`](crate::SiftStream::send) awaits when this channel is full, applying backpressure
    /// to the caller. Defaults to [`DATA_CHANNEL_CAPACITY`](crate::stream::tasks::DATA_CHANNEL_CAPACITY).
    /// Increase for high-throughput producers to absorb bursts; decrease to reduce memory usage
    /// at the cost of earlier backpressure.
    pub fn backup_data_channel_capacity(mut self, capacity: usize) -> Self {
        self.backup_data_channel_capacity = capacity;
        self
    }

    /// Sets the capacity of the control channel — the broadcast channel used to send internal
    /// signals such as shutdown.
    ///
    /// Defaults to [`CONTROL_CHANNEL_CAPACITY`](crate::stream::tasks::CONTROL_CHANNEL_CAPACITY).
    /// Most users do not need to change this.
    pub fn control_channel_capacity(mut self, capacity: usize) -> Self {
        self.control_channel_capacity = capacity;
        self
    }

    /// Sets the interval at which [SiftStreamMetrics](crate::metrics::SiftStreamMetrics) are
    /// streamed to Sift. Defaults to [DEFAULT_METRICS_STREAMING_INTERVAL]. Pass `None` to disable.
    pub fn metrics_streaming_interval(mut self, interval: Option<Duration>) -> Self {
        self.metrics_streaming_interval = interval;
        self
    }

    /// Performs setup and returns a [SiftStream] configured for `FileBackup`.
    pub async fn build(self) -> Result<SiftStream<IngestionConfigEncoder, FileBackup>> {
        let Some(backups_dir) = self.disk_backup_policy.backups_dir.clone() else {
            return Err(Error::new_arg_error(
                "disk_backup_policy.backups_dir must be set to use file backup mode",
            ));
        };

        let setup = setup_common(self.base).await?;

        let mut output_directory = sanitize_name(&setup.asset_name);
        if let Some(run) = setup.run.as_ref() {
            output_directory.push_str(&format!("/{}", sanitize_name(&run.name)));
        }

        SiftStream::new_file_backup(
            setup.setup_channel,
            setup.ingestion_config,
            setup.flows_by_name,
            setup.run,
            backups_dir.to_path_buf(),
            output_directory.into(),
            self.disk_backup_policy.max_backup_file_size,
            self.backup_data_channel_capacity,
            self.control_channel_capacity,
            self.metrics_streaming_interval,
            setup.session_name,
            setup.sift_stream_id,
            setup.metrics,
        )
        .await
    }
}

/// Result of the common setup steps shared by all mode builders.
#[derive(Clone)]
struct CommonSetup {
    setup_channel: SiftChannel,
    ingestion_channel: SiftChannel,
    ingestion_config: IngestionConfig,
    flows_by_name: HashMap<String, FlowDescriptor<String>>,
    asset_name: String,
    run: Option<Run>,
    metrics: Arc<SiftStreamMetrics>,
    session_name: String,
    sift_stream_id: Uuid,
    /// Scoped dispatch to install on spawned background task futures.
    /// `None` when the `tracing` feature is disabled or `disable_scoped_dispatch` was set.
    #[cfg(feature = "tracing")]
    scoped_dispatch: Option<tracing::Dispatch>,
    /// Receiver for log events captured by the scoped dispatch.
    /// `None` when the `tracing` feature is disabled or `disable_scoped_dispatch` was set.
    log_rx: Option<async_channel::Receiver<LogEvent>>,
}

/// Performs setup steps common to all mode builders:
///
/// - Creates gRPC channels (setup + ingestion)
/// - Connects to Sift
/// - Loads the ingestion config
/// - Loads the run (if any)
/// - Updates asset tags and metadata
/// - Creates the metrics object
async fn setup_common(base: StreamConfigBuilder) -> Result<CommonSetup> {
    // Need a channel or credentials to build the channels.
    if base.channel.is_none() && base.credentials.is_none() {
        return Err(Error::new_arg_error(
            "either credentials or a gRPC channel must be provided",
        ));
    }

    let build_channel = |credentials: Credentials| -> Result<SiftChannel> {
        let mut sift_channel_builder = SiftChannelBuilder::new(credentials);
        if base.enable_tls {
            sift_channel_builder = sift_channel_builder.use_tls(true);
        }
        sift_channel_builder.build()
    };

    let (setup_channel, ingestion_channel) = match base.channel {
        Some(ch) => (ch.clone(), ch),
        None => {
            let creds = base.credentials.unwrap();
            let setup_channel = build_channel(creds)?;
            let ingestion_channel = setup_channel.clone();
            (setup_channel, ingestion_channel)
        }
    };

    // Since the gRPC connection is lazy, connect right away to verify the connection.
    for channel in [setup_channel.clone(), ingestion_channel.clone()] {
        PingServiceClient::new(channel)
            .ping(PingRequest::default())
            .await
            .map_err(|e| Error::new(ErrorKind::GrpcConnectError, e))
            .context("failed to connect to Sift")
            .help("ensure that your API key and Sift gRPC API URL is correct and TLS is configured properly")?;
    }

    let (ingestion_config, flows, asset) =
        load_ingestion_config(setup_channel.clone(), base.ingestion_config).await?;

    let run = {
        if let Some(run_id) = base.run_id.as_ref() {
            Some(load_run_by_id(setup_channel.clone(), run_id).await?)
        } else if let Some(selector) = base.run {
            Some(load_run_by_form(setup_channel.clone(), selector).await?)
        } else {
            None
        }
    };

    let asset_name = asset.name.clone();

    helpers::update_asset_tags_and_metadata(
        asset.clone(),
        base.asset_tags,
        base.asset_metadata,
        setup_channel.clone(),
    )
    .await?;

    let metrics = Arc::new(SiftStreamMetrics::new());

    let ingestion_config_id = ingestion_config.ingestion_config_id.clone();
    let mut flows_by_name = HashMap::with_capacity(flows.len());

    for flow in flows {
        let flow_name = flow.name.clone();
        let flow_descriptor = FlowDescriptor::try_from((&ingestion_config_id, flow))?;
        flows_by_name.insert(flow_name, flow_descriptor);
    }

    metrics.loaded_flows.add(flows_by_name.len() as u64);

    let session_name = format!("stream.{}.{}", asset_name, ingestion_config.client_key);
    let sift_stream_id = Uuid::new_v4();

    // Build the scoped dispatch and log channel (tracing feature only).
    #[cfg(feature = "tracing")]
    let (scoped_dispatch, log_rx) = {
        use tracing_subscriber::layer::SubscriberExt;
        if base.disable_scoped_dispatch {
            (None, None)
        } else {
            let (log_tx, log_rx) = async_channel::bounded::<LogEvent>(LOG_CHANNEL_CAPACITY);
            let base_dispatch = base
                .scoped_dispatch_base
                .unwrap_or_else(|| tracing::dispatcher::get_default(|d| d.clone()));
            let subscriber = tracing_subscriber::registry()
                .with(crate::telemetry::SiftTelemetryLayer::new(
                    log_tx,
                    base.log_level_filter,
                    metrics.clone(),
                ))
                .with(crate::telemetry::DispatchForwardingLayer(base_dispatch));
            let dispatch = tracing::Dispatch::new(subscriber);
            (Some(dispatch), Some(log_rx))
        }
    };

    #[cfg(not(feature = "tracing"))]
    let log_rx: Option<async_channel::Receiver<LogEvent>> = None;

    Ok(CommonSetup {
        setup_channel,
        ingestion_channel,
        ingestion_config,
        flows_by_name,
        asset_name,
        run,
        metrics,
        session_name,
        sift_stream_id,
        #[cfg(feature = "tracing")]
        scoped_dispatch,
        log_rx,
    })
}

impl From<Credentials> for SiftStreamBuilder {
    fn from(value: Credentials) -> Self {
        Self::new(value)
    }
}

impl From<SiftChannel> for SiftStreamBuilder {
    fn from(value: SiftChannel) -> Self {
        Self::from_channel(value)
    }
}
