use super::{
    RetryPolicy, SiftStream, SiftStreamMode,
    flow::validate_flows,
    mode::ingestion_config::IngestionConfigMode,
    run::{load_run_by_form, load_run_by_id},
};
use crate::{
    backup::{disk::DiskBackupPolicy, sanitize_name},
    metrics::SiftStreamMetrics,
    stream::tasks::{CONTROL_CHANNEL_CAPACITY, DATA_CHANNEL_CAPACITY, RecoveryConfig, TaskConfig},
};
use sift_connect::{Credentials, SiftChannel, SiftChannelBuilder};
use sift_error::prelude::*;
use sift_rs::{
    assets::v1::Asset,
    ingestion_configs::v2::{FlowConfig, IngestionConfig as IngestionConfigPb},
    metadata::v1::MetadataValue,
    ping::v1::{PingRequest, ping_service_client::PingServiceClient},
    wrappers::{
        assets::{AssetServiceWrapper, new_asset_service},
        ingestion_configs::{IngestionConfigServiceWrapper, new_ingestion_config_service},
    },
};
use std::{marker::PhantomData, sync::Arc, time::Duration};
use uuid::Uuid;

/// The default checkpoint interval (1 minute) to use if left unspecified.
pub const DEFAULT_CHECKPOINT_INTERVAL: Duration = Duration::from_secs(60);

/// The default metrics streaming interval (500 milliseconds) to use if left unspecified.
pub const DEFAULT_METRICS_STREAMING_INTERVAL: Duration = Duration::from_millis(500);

/// Configures and builds an instance of [SiftStream]. The quickest way to get started is to simply
/// pass your [Credentials] to [SiftStreamBuilder::new] as well as your [IngestionConfigForm] and
/// call [SiftStreamBuilder::build] like so:
///
/// ```ignore
/// let mut sift_stream = SiftStreamBuilder::new(credentials)
///     .ingestion_config(ingestion_config)
///     .build()
///     .await?;
/// ```
///
/// To add additional behaviors or modify existing ones, see the methods available on the builder.
///
/// ### Panic
///
/// Because [tonic](https://docs.rs/tonic/latest/tonic/) is an underlying dependency, the
/// [tokio](https://docs.rs/tokio/latest/tokio/) asynchronous runtime is required, otherwise
/// attempts to call [SiftStreamBuilder::build] will panic.
pub struct SiftStreamBuilder<C> {
    credentials: Option<Credentials>,
    channel: Option<SiftChannel>,
    recovery_strategy: Option<RecoveryStrategy>,
    checkpoint_interval: Duration,
    ingestion_config: Option<IngestionConfigForm>,
    enable_tls: bool,
    kind: PhantomData<C>,
    asset_tags: Option<Vec<String>>,
    asset_metadata: Option<Vec<MetadataValue>>,
    control_channel_capacity: usize,
    ingestion_data_channel_capacity: usize,
    backup_data_channel_capacity: usize,
    enable_compression_for_ingestion: bool,
    metrics_streaming_interval: Option<Duration>,

    // Either `run` or `run_id`. If both are provided then the `run_id` will be prioritized.
    run: Option<RunForm>,
    run_id: Option<String>,
}

/// Various recovery strategies users can enable for [SiftStream] when constructing it via
/// [SiftStreamBuilder].
#[derive(Debug, Clone)]
pub enum RecoveryStrategy {
    /// - Enables retries only. Users can provide their own custom retry policy or use the default
    ///   recommended settings via [RetryPolicy::default].
    RetryOnly(RetryPolicy),

    /// - Enables retries as well as disk backups. Users can provide their own custom retry
    ///   policy or use the default recommended settings via [RetryPolicy::default]. Similarly,
    ///   users can provide their own custom disk backup policy or use the default recommended
    ///   settings via [DiskBackupPolicy::default].
    RetryWithBackups {
        retry_policy: RetryPolicy,
        disk_backup_policy: DiskBackupPolicy,
    },
}

/// A form to create a new ingestion config or retrieve an existing ingestion config based on the
/// `client_key` provided. The `client_key` is an arbitrary user-sourced identifier that is
/// expected to be unique across the user's organization; it's used to uniquely identify a
/// particular ingestion config which defines the schema of an asset's telemetry. See the
/// [top-level documentation](crate#ingestion-configs) for further details.
#[derive(Debug, Clone)]
pub struct IngestionConfigForm {
    pub asset_name: String,
    pub client_key: String,
    pub flows: Vec<FlowConfig>,
}

/// A form to create a new run or retrieve an existing run based on the `client_key` provided. This
/// is used in [SiftStreamBuilder::attach_run]. Note that if there is an existing run with the
/// given `client_key`, any other fields that are updated in this [RunForm] will be updated in
/// Sift, with the exception of `Option` fields that are `None`.
#[derive(Debug, Clone, Default)]
pub struct RunForm {
    pub name: String,
    pub client_key: String,
    pub description: Option<String>,
    pub tags: Option<Vec<String>>,
    pub metadata: Option<Vec<MetadataValue>>,
}

impl Default for RecoveryStrategy {
    /// Initializes a retry-only recovery strategy using [RetryPolicy::default].
    fn default() -> Self {
        RecoveryStrategy::RetryOnly(RetryPolicy::default())
    }
}

impl RecoveryStrategy {
    /// Initializes a retry with disk backups recovery strategy using the default recommended
    /// configurations.
    pub fn default_retry_policy_with_backups() -> Self {
        Self::RetryWithBackups {
            retry_policy: RetryPolicy::default(),
            disk_backup_policy: DiskBackupPolicy::default(),
        }
    }
}

impl<C> SiftStreamBuilder<C>
where
    C: SiftStreamMode,
{
    /// Sets the interval to stream [`SiftStreamMetrics`] to Sift. If `None`, metrics streaming will be disabled.
    pub fn metrics_streaming_interval(
        mut self,
        interval: Option<Duration>,
    ) -> SiftStreamBuilder<C> {
        self.metrics_streaming_interval = interval;
        self
    }

    /// Sets the control channel capacity. See the [top-level documentation](crate#checkpoints)
    /// for further details.
    pub fn control_channel_capacity(mut self, capacity: usize) -> SiftStreamBuilder<C> {
        self.control_channel_capacity = capacity;
        self
    }

    /// Sets the ingestion data channel capacity. See the [top-level documentation](crate#checkpoints)
    /// for further details.
    pub fn ingestion_data_channel_capacity(mut self, capacity: usize) -> SiftStreamBuilder<C> {
        self.ingestion_data_channel_capacity = capacity;
        self
    }

    /// Sets the backup data channel capacity. See the [top-level documentation](crate#checkpoints)
    /// for further details.
    pub fn backup_data_channel_capacity(mut self, capacity: usize) -> SiftStreamBuilder<C> {
        self.backup_data_channel_capacity = capacity;
        self
    }

    /// Sets the recovery strategy to use. See [RecoveryStrategy].
    pub fn recovery_strategy(mut self, strategy: RecoveryStrategy) -> SiftStreamBuilder<C> {
        self.recovery_strategy = Some(strategy);
        self
    }

    /// Sets the run to use for this period of streaming. Any data sent will be associated with
    /// this run. If the `run` used is an existing run, then any fields that have been updated will
    /// also be updated in Sift. Optional fields that are `None` will be ignored when determining
    /// which fields to update. This method should not be used if [SiftStreamBuilder::attach_run_id]
    /// is used. If for whatever reason both are used, [SiftStreamBuilder::attach_run_id] will take
    /// precedent.
    pub fn attach_run(mut self, run: RunForm) -> SiftStreamBuilder<C> {
        self.run = Some(run);
        self
    }

    // Sets the run based on run ID for this period of streaming. Any data sent will be associated
    // with this run. This method should not be used if [SiftStreamBuilder::attach_run] is used. If
    // for whatever reason both are used, this will take precedent.
    pub fn attach_run_id(mut self, run_id: &str) -> SiftStreamBuilder<C> {
        self.run_id = Some(run_id.into());
        self
    }

    /// Sets whether compression is enabled.
    ///
    /// Currently only gzip is supported.
    ///
    /// WARNING: Compression adds additional overhead both on the client and server, so can reduce
    /// the overall throughput of a stream. It is not recommended to enable compression by default.
    pub fn enable_compression_for_ingestion(mut self, enable: bool) -> SiftStreamBuilder<C> {
        self.enable_compression_for_ingestion = enable;
        self
    }

    /// Disables TLS. Useful for testing. This is ignored if [SiftStreamBuilder::from_channel] is
    /// used to initialize the builder.
    pub fn disable_tls(mut self) -> SiftStreamBuilder<C> {
        self.enable_tls = false;
        self
    }

    /// Creates or updates the asset tags. If Some is provided, asset tags will be replaced
    /// with the tags provided. If None, no update to tags will occur.
    pub fn add_asset_tags(mut self, tags: Option<Vec<String>>) -> SiftStreamBuilder<C> {
        self.asset_tags = tags;
        self
    }

    /// Creates or updates the asset metadata. If Some is provided, asset metadata will be replaced
    /// with the key:value pairs provided. If None, no update to metadata will occur.
    pub fn add_asset_metadata(
        mut self,
        metadata: Option<Vec<MetadataValue>>,
    ) -> SiftStreamBuilder<C> {
        self.asset_metadata = metadata;
        self
    }
}

/// Builds a [SiftStream] specifically for ingestion-config based streaming.
impl SiftStreamBuilder<IngestionConfigMode> {
    /// Initializes a new builder for ingestion-config-based streaming from [Credentials].
    pub fn new(credentials: Credentials) -> SiftStreamBuilder<IngestionConfigMode> {
        SiftStreamBuilder {
            credentials: Some(credentials),
            channel: None,
            enable_tls: true,
            enable_compression_for_ingestion: false,
            ingestion_config: None,
            run: None,
            run_id: None,
            kind: PhantomData,
            checkpoint_interval: DEFAULT_CHECKPOINT_INTERVAL,
            recovery_strategy: None,
            asset_tags: None,
            asset_metadata: None,
            control_channel_capacity: CONTROL_CHANNEL_CAPACITY,
            ingestion_data_channel_capacity: DATA_CHANNEL_CAPACITY,
            backup_data_channel_capacity: DATA_CHANNEL_CAPACITY,
            metrics_streaming_interval: Some(DEFAULT_METRICS_STREAMING_INTERVAL),
        }
    }

    /// Initializes a new builder for ingestion-config-based streaming from a [SiftChannel].
    ///
    /// IMPORTANT:
    ///
    /// It is preferred that credentials are provided so that independent gRPC channels can
    /// be created and used. Cloning the channel results in multiplexing the gRPC requests
    /// over a single connection, which is not desirable for backup re-ingestion which may
    /// starve out primary ingestion.
    pub fn from_channel(channel: SiftChannel) -> SiftStreamBuilder<IngestionConfigMode> {
        SiftStreamBuilder {
            credentials: None,
            channel: Some(channel),
            enable_tls: true,
            enable_compression_for_ingestion: false,
            ingestion_config: None,
            run: None,
            run_id: None,
            kind: PhantomData,
            checkpoint_interval: DEFAULT_CHECKPOINT_INTERVAL,
            recovery_strategy: None,
            asset_tags: None,
            asset_metadata: None,
            control_channel_capacity: CONTROL_CHANNEL_CAPACITY,
            ingestion_data_channel_capacity: DATA_CHANNEL_CAPACITY,
            backup_data_channel_capacity: DATA_CHANNEL_CAPACITY,
            metrics_streaming_interval: Some(DEFAULT_METRICS_STREAMING_INTERVAL),
        }
    }

    /// Consume the builder and return a [SiftStream] configured for ingestion-config-based
    /// streaming.
    pub async fn build(self) -> Result<SiftStream<IngestionConfigMode>> {
        let SiftStreamBuilder {
            checkpoint_interval,
            channel: grpc_channel,
            credentials,
            enable_tls,
            enable_compression_for_ingestion,
            ingestion_config,
            recovery_strategy,
            run,
            run_id,
            ..
        } = self;

        let Some(ingestion_config) = ingestion_config else {
            return Err(Error::new_arg_error("ingestion_config is required"));
        };

        // Need a channel or credentials to build the channels.
        if grpc_channel.is_none() && credentials.is_none() {
            return Err(Error::new_arg_error(
                "either credentials or a gRPC channel must be provided",
            ));
        }

        let build_channel = |credentials: Credentials| -> Result<SiftChannel> {
            let mut sift_channel_builder = SiftChannelBuilder::new(credentials);
            if enable_tls {
                sift_channel_builder = sift_channel_builder.use_tls(true);
            }
            sift_channel_builder.build()
        };

        // Create a setup channel, an ingestion channel, and a reingestion channel if not provided.
        //
        // It is preferred that credentials are provided so that independent gRPC channels can
        // be created and used. Cloning the channel results in multiplexing the gRPC requests
        // over a single connection, which is not desirable for backup re-ingestion which may
        // starve out primary ingestion.
        let (setup_channel, ingestion_channel, reingestion_channel) = match grpc_channel {
            Some(ch) => (ch.clone(), ch.clone(), ch),
            None => {
                let creds = credentials.unwrap();

                let setup_channel = build_channel(creds.clone())?;
                let ingestion_channel = setup_channel.clone();
                let reingestion_channel = build_channel(creds)?;

                (setup_channel, ingestion_channel, reingestion_channel)
            }
        };

        // Since the gRPC connection is lazy, we'll connect right away and ensure the connection is
        // valid.
        for channel in [
            setup_channel.clone(),
            ingestion_channel.clone(),
            reingestion_channel.clone(),
        ] {
            PingServiceClient::new(channel).ping(PingRequest::default())
                .await
                .map_err(|e| Error::new(ErrorKind::GrpcConnectError, e))
                .context("failed to connect to Sift")
                .help("ensure that your API key and Sift gRPC API URL is correct and TLS is configured properly")?;
        }

        let (ingestion_config, flows, asset) =
            Self::load_ingestion_config(setup_channel.clone(), ingestion_config).await?;

        let run = {
            if let Some(run_id) = run_id.as_ref() {
                Some(load_run_by_id(setup_channel.clone(), run_id).await?)
            } else if let Some(selector) = run {
                Some(load_run_by_form(setup_channel.clone(), selector).await?)
            } else {
                None
            }
        };

        let asset_name = asset.name.clone();

        // Try updating tags or metadata. Update only occurs if either asset_tags or asset_metadata is Some
        Self::update_asset_tags_and_metadata(
            asset,
            self.asset_tags,
            self.asset_metadata,
            setup_channel.clone(),
        )
        .await?;

        let metrics = Arc::new(SiftStreamMetrics::new());

        let recovery_config = match recovery_strategy {
            Some(RecoveryStrategy::RetryOnly(retry_policy)) => RecoveryConfig {
                retry_policy: retry_policy.clone(),
                backups_enabled: false,
                backups_directory: String::new(),
                backups_prefix: String::new(),
                backup_policy: DiskBackupPolicy::default(),
            },
            Some(RecoveryStrategy::RetryWithBackups {
                retry_policy,
                disk_backup_policy,
            }) => {
                let mut dir_name = sanitize_name(&asset_name);
                if let Some(run) = run.as_ref() {
                    dir_name.push_str(&format!("/{}", sanitize_name(&run.name)));
                }

                RecoveryConfig {
                    retry_policy: retry_policy.clone(),
                    backups_enabled: true,
                    backups_directory: dir_name,
                    backups_prefix: ingestion_config.client_key.clone(),
                    backup_policy: disk_backup_policy.clone(),
                }
            }
            None => RecoveryConfig {
                retry_policy: RetryPolicy::default(),
                backups_enabled: false,
                backups_directory: String::new(),
                backups_prefix: String::new(),
                backup_policy: DiskBackupPolicy::default(),
            },
        };

        let task_config = TaskConfig {
            session_name: format!("stream.{}.{}", asset_name, ingestion_config.client_key),
            sift_stream_id: Uuid::new_v4(),
            ingestion_channel,
            reingestion_channel,
            setup_channel,
            metrics: metrics.clone(),
            checkpoint_interval,
            enable_compression_for_ingestion,
            recovery_config,
            control_channel_capacity: self.control_channel_capacity,
            ingestion_data_channel_capacity: self.ingestion_data_channel_capacity,
            backup_data_channel_capacity: self.backup_data_channel_capacity,
            metrics_streaming_interval: self.metrics_streaming_interval,
        };

        SiftStream::<IngestionConfigMode>::new(ingestion_config, flows, run, task_config, metrics)
    }

    /// Sets the ingestion config used for streaming. See the [top-level
    /// documentation](crate#ingestion-configs) for further details on ingestion configs.
    pub fn ingestion_config(mut self, ingestion_config: IngestionConfigForm) -> Self {
        self.ingestion_config = Some(ingestion_config);
        self
    }

    /// Sets the interval between checkpoints. See the [top-level documentation](crate#checkpoints)
    /// for further details.
    pub fn checkpoint_interval(
        mut self,
        duration: Duration,
    ) -> SiftStreamBuilder<IngestionConfigMode> {
        self.checkpoint_interval = duration;
        self
    }

    async fn load_ingestion_config(
        grpc_channel: SiftChannel,
        ingestion_config: IngestionConfigForm,
    ) -> Result<(IngestionConfigPb, Vec<FlowConfig>, Asset)> {
        #[cfg(feature = "tracing")]
        tracing::info_span!("load_ingestion_config");

        let mut ingestion_config_service = new_ingestion_config_service(grpc_channel.clone());
        let mut asset_service = new_asset_service(grpc_channel);

        let IngestionConfigForm {
            asset_name,
            client_key,
            mut flows,
        } = ingestion_config;

        match ingestion_config_service
            .try_get_ingestion_config_by_client_key(&client_key)
            .await
        {
            Err(err) if err.kind() == ErrorKind::NotFoundError => {
                let ingestion_config = ingestion_config_service
                    .try_create_ingestion_config(&asset_name, &client_key, &flows)
                    .await?;

                let new_flows = {
                    if flows.is_empty() {
                        Vec::new()
                    } else {
                        ingestion_config_service
                            .try_filter_flows(&ingestion_config.ingestion_config_id, "")
                            .await?
                    }
                };

                let asset = asset_service
                    .try_get_asset_by_id(&ingestion_config.asset_id)
                    .await
                    .context("failed to retrieve asset specified by ingestion config")?;

                #[cfg(feature = "tracing")]
                {
                    if !new_flows.is_empty() {
                        let flow_names = new_flows
                            .iter()
                            .map(|f| f.name.as_str())
                            .collect::<Vec<&str>>()
                            .join(",");
                        tracing::info!(
                            ingestion_config_id = ingestion_config.ingestion_config_id,
                            flow_names = flow_names,
                            "created new ingestion config"
                        );
                    }
                }
                Ok((ingestion_config, flows, asset))
            }
            Err(err) => Err(err),

            Ok(ingestion_config) => {
                #[cfg(feature = "tracing")]
                tracing::info!(
                    ingestion_config_id = ingestion_config.ingestion_config_id,
                    "an existing ingestion config was found with the provided client-key"
                );

                let asset = asset_service
                    .try_get_asset_by_id(&ingestion_config.asset_id)
                    .await
                    .context("failed to retrieve asset specified by ingestion config")?;

                if asset.name != asset_name {
                    return Err(Error::new_msg(
                        ErrorKind::IncompatibleIngestionConfigChange,
                        format!(
                            "local ingestion config references asset '{asset_name}' but this existing config in Sift refers to asset '{}'",
                            asset.name
                        ),
                    ));
                }

                let flow_names = flows
                    .iter()
                    .map(|f| format!("'{}'", f.name))
                    .collect::<Vec<String>>()
                    .join(",");

                let filter = flow_names
                    .is_empty()
                    .then(String::new)
                    .unwrap_or_else(|| format!("flow_name in [{flow_names}]"));

                let existing_flows = ingestion_config_service
                    .try_filter_flows(&ingestion_config.ingestion_config_id, &filter)
                    .await?;

                let mut flows_to_create: Vec<FlowConfig> = Vec::new();

                for flow in &flows {
                    let mut flow_exists = false;

                    for existing_flow in existing_flows.iter().filter(|ef| ef == &flow) {
                        flow_exists = flow
                            .channels
                            .iter()
                            .zip(existing_flow.channels.iter())
                            .all(|(lhs, rhs)| lhs == rhs);

                        if flow_exists {
                            break;
                        }
                    }

                    if !flow_exists {
                        flows_to_create.push(flow.clone());
                    }
                }

                if !flows_to_create.is_empty() {
                    let _ = ingestion_config_service
                        .try_create_flows(&ingestion_config.ingestion_config_id, &flows_to_create)
                        .await;

                    #[cfg(feature = "tracing")]
                    {
                        let new_flow_names = flows_to_create
                            .iter()
                            .map(|f| f.name.as_str())
                            .collect::<Vec<&str>>()
                            .join(",");
                        tracing::info!(
                            ingestion_config_id = ingestion_config.ingestion_config_id,
                            new_flows = new_flow_names,
                            "created new flows for ingestion config"
                        );
                    }

                    // All the flows Sift sees with the specified names
                    let sift_flows = ingestion_config_service
                        .try_filter_flows(&ingestion_config.ingestion_config_id, &filter)
                        .await?;

                    validate_flows(&flows, &sift_flows)?;

                    // Validation succeeded... used the flows we got for confidence in correctness.
                    flows = sift_flows;
                }

                Ok((ingestion_config, flows, asset))
            }
        }
    }

    async fn update_asset_tags_and_metadata(
        mut asset: Asset,
        asset_tags: Option<Vec<String>>,
        asset_metadata: Option<Vec<MetadataValue>>,
        channel: SiftChannel,
    ) -> Result<()> {
        let mut update_mask = Vec::new();

        if let Some(asset_tags) = asset_tags {
            asset.tags = asset_tags;
            update_mask.push("tags".to_string());
        }

        if let Some(asset_metadata) = asset_metadata {
            asset.metadata = asset_metadata;
            update_mask.push("metadata".to_string());
        }

        if update_mask.is_empty() {
            return Ok(());
        }

        let mut asset_service = new_asset_service(channel);
        let _ = asset_service.try_update_asset(asset, update_mask).await?;

        Ok(())
    }
}

impl From<Credentials> for SiftStreamBuilder<IngestionConfigMode> {
    fn from(value: Credentials) -> Self {
        Self::new(value)
    }
}

impl From<SiftChannel> for SiftStreamBuilder<IngestionConfigMode> {
    fn from(value: SiftChannel) -> Self {
        Self::from_channel(value)
    }
}
