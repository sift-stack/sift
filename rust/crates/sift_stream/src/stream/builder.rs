use super::{
    RetryPolicy, SiftStream, SiftStreamMode,
    flow::validate_flows,
    mode::ingestion_config::IngestionConfigMode,
    mode::ingestion_config::IngestionConfigModeBackupsManager,
    run::{load_run_by_form, load_run_by_id},
};
use crate::backup::{DiskBackupsManager, InMemoryBackupsManager};
use sift_connect::{Credentials, SiftChannel, SiftChannelBuilder};
use sift_error::prelude::*;
use sift_rs::{
    ingestion_configs::v2::{FlowConfig, IngestionConfig as IngestionConfigPb},
    ping::v1::{PingRequest, ping_service_client::PingServiceClient},
    wrappers::{
        assets::{AssetServiceWrapper, new_asset_service},
        ingestion_configs::{IngestionConfigServiceWrapper, new_ingestion_config_service},
    },
};
use std::{marker::PhantomData, path::PathBuf, time::Duration};

/// The default checkpoint interval (1 minute) to use if left unspecified.
pub const DEFAULT_CHECKPOINT_INTERVAL: Duration = Duration::from_secs(60);

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

    // Either `run` or `run_id`. If both are provided then the `run_id` will be prioritized.
    run: Option<RunForm>,
    run_id: Option<String>,
}

/// Various recovery strategies users can enable for [SiftStream] when constructing it via
/// [SiftStreamBuilder].
#[derive(Debug)]
pub enum RecoveryStrategy {
    /// - Enables retries only. Users can provide their own custom retry policy or use the default
    ///   recommended settings via [RetryPolicy::default].
    RetryOnly(RetryPolicy),

    /// - Enables retries as well as in-memory backups. Users can provide their own custom retry
    ///   policy or use the default recommended settings via [RetryPolicy::default].
    ///
    /// - `max_buffer_size` specifies the capacity of the underlying buffer. If `None`, then the
    ///   default [crate::backup::memory::DEFAULT_MAX_BUFFER_SIZE] is used.
    RetryWithInMemoryBackups {
        retry_policy: RetryPolicy,
        max_buffer_size: Option<usize>,
    },

    /// - Enables retries as well as disk backups. Users can provide their own custom retry
    ///   policy or use the default recommended settings via [RetryPolicy::default].
    ///
    /// - `backups_dir` is the directory where the backups will get created. If `backups_dir` is
    ///   `None`, then the user's [data
    ///   directory](https://docs.rs/dirs/latest/dirs/fn.data_dir.html) is used. If `backups_dir` is provided but
    ///   doesn't exist, then there will be an attempt to create that directory.
    ///
    /// - `max_backups_file_size` is the maximum size that a backup file is allowed to be before a
    ///   checkpoint is forced. Once a checkpoint is forced, a new backup file will be created.
    ///
    /// **Important Note**: The `max_backups_file_size` does not represent that actual amount of
    /// space on disk which is affected by operating system-level compression and block allocation;
    /// instead the byte-length is the actual measure.
    RetryWithDiskBackups {
        retry_policy: RetryPolicy,
        backups_dir: Option<PathBuf>,
        max_backups_file_size: Option<usize>,
    },
}

/// A form to create a new ingestion config or retrieve an existing ingestion config based on the
/// `client_key` provided. The `client_key` is an arbitrary user-sourced identifier that is
/// expected to be unique across the user's organization; it's used to uniquely identify a
/// particular ingestion config which defines the schema of an asset's telemetry. See the
/// [top-level documentation](crate#ingestion-configs) for further details.
#[derive(Debug)]
pub struct IngestionConfigForm {
    pub asset_name: String,
    pub client_key: String,
    pub flows: Vec<FlowConfig>,
}

/// A form to create a new run or retrieve an existing run based on the `client_key` provided. This
/// is used in [SiftStreamBuilder::attach_run]. Note that if there is an existing run with the
/// given `client_key`, any other fields that are updated in this [RunForm] will be updated in
/// Sift, with the exception of `Option` fields that are `None`.
#[derive(Debug)]
pub struct RunForm {
    pub name: String,
    pub client_key: String,
    pub description: Option<String>,
    pub tags: Option<Vec<String>>,
}

impl Default for RecoveryStrategy {
    /// Initializes a retry-only recovery strategy using [RetryPolicy::default].
    fn default() -> Self {
        RecoveryStrategy::RetryOnly(RetryPolicy::default())
    }
}

impl RecoveryStrategy {
    /// Initializes a retry with in-memory backups recovery strategy using the default recommended
    /// configurations.
    pub fn default_retry_policy_in_memory_backups() -> Self {
        Self::RetryWithInMemoryBackups {
            retry_policy: RetryPolicy::default(),
            max_buffer_size: None,
        }
    }

    /// Initializes a retry with disk backups recovery strategy using the default recommended
    /// configurations.
    pub fn default_retry_policy_disk_backups() -> Self {
        Self::RetryWithDiskBackups {
            retry_policy: RetryPolicy::default(),
            backups_dir: None,
            max_backups_file_size: None,
        }
    }
}

impl<C> SiftStreamBuilder<C>
where
    C: SiftStreamMode,
{
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

    /// Disables TLS. Useful for testing. This is ignored if [SiftStreamBuilder::from_channel] is
    /// used to initialize the builder.
    pub fn disable_tls(mut self) -> SiftStreamBuilder<C> {
        self.enable_tls = false;
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
            ingestion_config: None,
            run: None,
            run_id: None,
            kind: PhantomData,
            checkpoint_interval: DEFAULT_CHECKPOINT_INTERVAL,
            recovery_strategy: None,
        }
    }

    /// Initializes a new builder for ingestion-config-based streaming from a [SiftChannel].
    pub fn from_channel(channel: SiftChannel) -> SiftStreamBuilder<IngestionConfigMode> {
        SiftStreamBuilder {
            credentials: None,
            channel: Some(channel),
            enable_tls: true,
            ingestion_config: None,
            run: None,
            run_id: None,
            kind: PhantomData,
            checkpoint_interval: DEFAULT_CHECKPOINT_INTERVAL,
            recovery_strategy: None,
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
            ingestion_config,
            recovery_strategy,
            run,
            run_id,
            ..
        } = self;

        let Some(ingestion_config) = ingestion_config else {
            return Err(Error::new_arg_error("ingestion_config is required"));
        };

        let channel = match grpc_channel {
            Some(ch) => ch,
            None if credentials.is_some() => {
                let mut sift_channel_builder = SiftChannelBuilder::new(credentials.unwrap());

                if enable_tls {
                    sift_channel_builder = sift_channel_builder.use_tls(true);
                }
                sift_channel_builder.build()?
            }
            None => {
                return Err(Error::new_arg_error(
                    "either credentials or a gRPC channel must be provided",
                ));
            }
        };

        // Since the gRPC connection is lazy, we'll connect right away and ensure the connection is
        // valid.
        PingServiceClient::new(channel.clone()).ping(PingRequest::default())
            .await
            .map_err(|e| Error::new(ErrorKind::GrpcConnectError, e))
            .context("failed to connect to Sift")
            .help("ensure that your API key and Sift gRPC API URL is correct and TLS is configured properly")?;

        let (ingestion_config, flows) =
            Self::load_ingestion_config(channel.clone(), ingestion_config).await?;

        let run = {
            if let Some(run_id) = run_id.as_ref() {
                Some(load_run_by_id(channel.clone(), run_id).await?)
            } else if let Some(selector) = run {
                Some(load_run_by_form(channel.clone(), selector).await?)
            } else {
                None
            }
        };

        let mut backups_manager = None;
        let mut policy = None;

        if let Some(strategy) = recovery_strategy {
            match strategy {
                RecoveryStrategy::RetryOnly(retry_policy) => {
                    policy = Some(retry_policy);
                }
                RecoveryStrategy::RetryWithInMemoryBackups {
                    retry_policy,
                    max_buffer_size,
                } => {
                    policy = Some(retry_policy);
                    let manager = IngestionConfigModeBackupsManager::InMemory(
                        InMemoryBackupsManager::new(max_buffer_size),
                    );

                    backups_manager = Some(manager);
                }
                RecoveryStrategy::RetryWithDiskBackups {
                    retry_policy,
                    backups_dir,
                    max_backups_file_size,
                } => {
                    policy = Some(retry_policy);
                    let manager = DiskBackupsManager::new(
                        backups_dir,
                        &ingestion_config.asset_id,
                        &ingestion_config.ingestion_config_id,
                        max_backups_file_size,
                    )
                    .map(IngestionConfigModeBackupsManager::Disk)
                    .context("failed to build backups manager")?;

                    backups_manager = Some(manager);
                }
            }
        }

        Ok(SiftStream::<IngestionConfigMode>::new(
            channel,
            ingestion_config,
            flows,
            run,
            checkpoint_interval,
            policy,
            backups_manager,
        ))
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
    ) -> Result<(IngestionConfigPb, Vec<FlowConfig>)> {
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
                Ok((ingestion_config, flows))
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

                Ok((ingestion_config, flows))
            }
        }
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
