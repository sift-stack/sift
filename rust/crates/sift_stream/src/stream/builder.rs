use super::{
    flow::validate_flows, mode::ingestion_config::IngestionConfigMode,
    mode::ingestion_config::IngestionConfigModeBackupsManager, RetryPolicy, SiftStream,
    SiftStreamMode,
};
use crate::backup::{DiskBackupsManager, InMemoryBackupsManager};
use sift_connect::{Credentials, SiftChannel, SiftChannelBuilder};
use sift_error::prelude::*;
use sift_rs::{
    ingestion_configs::v2::{FlowConfig, IngestionConfig as IngestionConfigPb},
    runs::v2::Run,
    wrappers::{
        ingestion_configs::{new_ingestion_config_service, IngestionConfigServiceWrapper},
        runs::{new_run_service, RunServiceWrapper},
    },
};
use std::{collections::HashSet, marker::PhantomData, path::PathBuf, time::Duration};

pub struct SiftStreamBuilder<C> {
    credentials: Credentials,
    run: Option<RunForm>,
    recovery_strategy: Option<RecoveryStrategy>,
    checkpoint_interval: Duration,
    ingestion_config: Option<IngestionConfigForm>,
    enable_tls: bool,
    kind: PhantomData<C>,
}

/// Mention how backups_dir `None` uses the system tmp dir.
#[derive(Debug)]
pub enum RecoveryStrategy {
    RetryOnly(RetryPolicy),
    RetryWithInMemoryBackups {
        retry_policy: RetryPolicy,
        max_buffer_size: Option<usize>,
    },
    RetryWithDiskBackups {
        retry_policy: RetryPolicy,
        backups_dir: Option<PathBuf>,
        max_backups_file_size: Option<usize>,
    },
}

#[derive(Debug)]
pub struct IngestionConfigForm {
    pub asset_name: String,
    pub client_key: String,
    pub flows: Vec<FlowConfig>,
}

#[derive(Debug)]
pub struct RunForm {
    pub name: String,
    pub client_key: String,
    pub description: Option<String>,
    pub tags: Option<Vec<String>>,
}

impl Default for RecoveryStrategy {
    fn default() -> Self {
        RecoveryStrategy::RetryWithDiskBackups {
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
    pub fn recovery_strategy(mut self, strategy: RecoveryStrategy) -> SiftStreamBuilder<C> {
        self.recovery_strategy = Some(strategy);
        self
    }

    pub fn attach_run(mut self, run: RunForm) -> SiftStreamBuilder<C> {
        self.run = Some(run);
        self
    }

    pub fn disable_tls(mut self) -> SiftStreamBuilder<C> {
        self.enable_tls = false;
        self
    }

    async fn load_run(grpc_channel: SiftChannel, run_form: RunForm) -> Result<Run> {
        #[cfg(feature = "tracing")]
        let _info_span = tracing::info_span!("load_run");

        let mut run_service = new_run_service(grpc_channel);

        let RunForm {
            name,
            description,
            tags,
            client_key,
        } = run_form;

        match run_service.try_get_run_by_client_key(&client_key).await {
            Err(e) if e.kind() == ErrorKind::NotFoundError => {
                let run = run_service
                    .try_create_run(
                        &name,
                        &client_key,
                        &description.unwrap_or_default(),
                        tags.unwrap_or_default().as_slice(),
                    )
                    .await?;

                #[cfg(feature = "tracing")]
                tracing::info!(run_id = run.run_id, run_name = run.name, "created new run");

                Ok(run)
            }
            Err(e) => Err(e),

            Ok(mut run) => {
                #[cfg(feature = "tracing")]
                tracing::info!(
                    run_id = run.run_id,
                    "an existing run was found with the provided client-key"
                );

                // An existing run was found; see if we need to update it.
                let mut update_mask = Vec::new();

                if name != run.name {
                    update_mask.push("name".to_string());
                    run.name = name;
                }
                if description.as_ref() != Some(&run.description) {
                    update_mask.push("description".to_string());
                    run.description = description.unwrap_or_default();
                }
                match tags {
                    Some(new_tags) if run.tags.is_empty() => {
                        update_mask.push("tags".to_string());
                        run.tags = new_tags;
                    }
                    Some(new_tags) => {
                        let new_tags_set = HashSet::<&String>::from_iter(new_tags.iter());
                        let current_tags_set = HashSet::from_iter(run.tags.iter());
                        let difference = new_tags_set.difference(&current_tags_set);

                        if difference.count() == 0 {
                            update_mask.push("tags".to_string());
                            run.tags = new_tags;
                        }
                    }
                    None if !run.tags.is_empty() => {
                        update_mask.push("tags".to_string());
                        run.tags = Vec::new();
                    }
                    _ => (),
                }

                if update_mask.is_empty() {
                    return Ok(run);
                }

                #[cfg(feature = "tracing")]
                tracing::info!(
                    "updating run fields as some fields have changed: {}",
                    update_mask.join(", ")
                );

                let updated_run = run_service.try_update_run(run, &update_mask).await?;

                #[cfg(feature = "tracing")]
                tracing::info!("successfully updated run");

                Ok(updated_run)
            }
        }
    }
}

impl SiftStreamBuilder<IngestionConfigMode> {
    pub fn new(credentials: Credentials) -> SiftStreamBuilder<IngestionConfigMode> {
        SiftStreamBuilder {
            credentials,
            enable_tls: true,
            ingestion_config: None,
            run: None,
            kind: PhantomData,
            checkpoint_interval: Duration::from_secs(60),
            recovery_strategy: None,
        }
    }

    pub async fn build(self) -> Result<SiftStream<IngestionConfigMode>> {
        let SiftStreamBuilder {
            credentials,
            checkpoint_interval,
            ingestion_config,
            run,
            recovery_strategy,
            enable_tls,
            ..
        } = self;

        let Some(ingestion_config) = ingestion_config else {
            return Err(Error::new_arg_error("ingestion_config is required"));
        };

        let mut sift_channel_builder = SiftChannelBuilder::new(credentials);

        if enable_tls {
            sift_channel_builder = sift_channel_builder.use_tls(true);
        }
        let channel = sift_channel_builder.build()?;

        let (ingestion_config, flows) =
            Self::load_ingestion_config(channel.clone(), ingestion_config).await?;

        let run = if let Some(selector) = run {
            Some(Self::load_run(channel.clone(), selector).await?)
        } else {
            None
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

    pub fn ingestion_config(mut self, ingestion_config: IngestionConfigForm) -> Self {
        self.ingestion_config = Some(ingestion_config);
        self
    }

    /// Sets the minimum duration a stream will transmit data before requesting an
    /// acknowledgment from Sift that all data sent up to that point has been received.
    ///
    /// Checkpointing terminates the current stream and starts a new one. However, a
    /// checkpoint is not guaranteed to occur precisely at this interval, especially if
    /// the stream remains open but idle. Checkpointing only occurs when data is actively
    /// being sent on the stream.
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
        let _info_span = tracing::info_span!("load_ingestion_config");

        let mut ingestion_config_service = new_ingestion_config_service(grpc_channel);

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

                let flows = ingestion_config_service
                    .try_filter_flows(&ingestion_config.ingestion_config_id, "")
                    .await?;

                #[cfg(feature = "tracing")]
                {
                    let flow_names = flows
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

                Ok((ingestion_config, flows))
            }
            Err(err) => Err(err),

            Ok(ingestion_config) => {
                #[cfg(feature = "tracing")]
                tracing::info!(
                    ingestion_config_id = ingestion_config.ingestion_config_id,
                    "an existing ingestion config was found with the provided client-key"
                );

                let flow_names = flows
                    .iter()
                    .map(|f| format!("'{}'", f.name))
                    .collect::<Vec<String>>()
                    .join(",");

                let filter = format!("flow_name in [{flow_names}]");
                let existing_flows = ingestion_config_service
                    .try_filter_flows(&ingestion_config.ingestion_config_id, &filter)
                    .await?;

                let mut flows_to_create: Vec<FlowConfig> = Vec::new();

                for flow in &flows {
                    let mut flow_exists = false;

                    for existing_flow in existing_flows.iter().filter(|ef| ef.name == flow.name) {
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
