use super::{mode::ingestion_config::IngestionConfigMode, RetryPolicy, SiftStream, SiftStreamMode};
use sift_connect::{Credentials, SiftChannel};
use sift_error::prelude::*;
use sift_rs::{
    ingestion_configs::v2::{FlowConfig, IngestionConfig},
    runs::v2::Run,
    wrappers::{
        ingestion_configs::{new_ingestion_config_service, IngestionConfigServiceWrapper},
        runs::{new_run_service, RunServiceWrapper},
    },
};
use std::{collections::HashSet, marker::PhantomData, time::Duration};

const DEFAULT_CHECKPOINT_INTERVAL_SEC: u64 = 60;

pub struct SiftStreamBuilder<C: SiftStreamMode> {
    credentials: Credentials,
    run_selector: Option<RunSelector>,
    retry_policy: Option<RetryPolicy>,
    checkpoint_interval: Option<Duration>,
    ingestion_config_selector: Option<IngestionConfigSelector>,
    kind: PhantomData<C>,
}

#[derive(Debug)]
pub enum IngestionConfigSelector {
    Id(String),
    ClientKey(String),
    Form {
        asset_name: String,
        client_key: String,
        flows: Vec<FlowConfig>,
    },
}

#[derive(Debug)]
pub enum RunSelector {
    Id(String),
    ClientKey(String),
    Form {
        name: String,
        client_key: String,
        description: Option<String>,
        tags: Option<Vec<String>>,
    },
}

impl<C: SiftStreamMode> SiftStreamBuilder<C> {
    pub fn new_with_ingestion_config(
        selector: IngestionConfigSelector,
        credentials: Credentials,
    ) -> SiftStreamBuilder<IngestionConfigMode> {
        SiftStreamBuilder {
            credentials,
            ingestion_config_selector: Some(selector),
            run_selector: None,
            kind: PhantomData,
            checkpoint_interval: None,
            retry_policy: None,
        }
    }

    pub fn retry_policy(mut self, policy: RetryPolicy) -> SiftStreamBuilder<C> {
        self.retry_policy = Some(policy);
        self
    }

    pub fn attach_run(mut self, run_selector: RunSelector) -> SiftStreamBuilder<C> {
        self.run_selector = Some(run_selector);
        self
    }

    async fn run_from_selector(grpc_channel: SiftChannel, selector: RunSelector) -> Result<Run> {
        #[cfg(feature = "tracing")]
        let _info_span = tracing::info_span!("run_from_selector");

        let mut run_service = new_run_service(grpc_channel);

        match selector {
            RunSelector::Id(run_id) => run_service.try_get_run_by_id(&run_id).await,
            RunSelector::ClientKey(client_key) => {
                run_service.try_get_run_by_client_key(&client_key).await
            }
            RunSelector::Form {
                name,
                description,
                tags,
                client_key,
            } => match run_service.try_get_run_by_client_key(&client_key).await {
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
            },
        }
    }
}

impl SiftStreamBuilder<IngestionConfigMode> {
    pub async fn build(self) -> Result<SiftStream<IngestionConfigMode>> {
        let SiftStreamBuilder {
            credentials,
            checkpoint_interval,
            ingestion_config_selector,
            run_selector,
            retry_policy,
            ..
        } = self;

        let Some(ingestion_config_selector) = ingestion_config_selector else {
            return Err(Error::new_arg_error(
                "ingestion_config_selector is required",
            ));
        };

        let channel = sift_connect::connect_grpc(credentials)?;

        let (ingestion_config, flows) =
            Self::ingestion_config_from_selector(channel.clone(), ingestion_config_selector)
                .await?;

        let run = if let Some(selector) = run_selector {
            Some(Self::run_from_selector(channel.clone(), selector).await?)
        } else {
            None
        };

        let checkpoint_interval = checkpoint_interval
            .unwrap_or_else(|| Duration::from_secs(DEFAULT_CHECKPOINT_INTERVAL_SEC));

        Ok(SiftStream::<IngestionConfigMode>::new(
            channel,
            ingestion_config,
            flows,
            run,
            checkpoint_interval,
            retry_policy,
        ))
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
        self.checkpoint_interval = Some(duration);
        self
    }

    async fn ingestion_config_from_selector(
        grpc_channel: SiftChannel,
        selector: IngestionConfigSelector,
    ) -> Result<(IngestionConfig, Vec<FlowConfig>)> {
        #[cfg(feature = "tracing")]
        let _info_span = tracing::info_span!("ingestion_config_from_selector");

        let mut ingestion_config_service = new_ingestion_config_service(grpc_channel);

        match selector {
            IngestionConfigSelector::Id(ingestion_config_id) => {
                let ingestion_config = ingestion_config_service
                    .try_get_ingestion_config_by_id(&ingestion_config_id)
                    .await?;
                let flows = ingestion_config_service
                    .try_filter_flows(&ingestion_config.ingestion_config_id, "")
                    .await?;
                Ok((ingestion_config, flows))
            }

            IngestionConfigSelector::ClientKey(client_key) => {
                let ingestion_config = ingestion_config_service
                    .try_get_ingestion_config_by_client_key(&client_key)
                    .await?;
                let flows = ingestion_config_service
                    .try_filter_flows(&ingestion_config.ingestion_config_id, "")
                    .await?;
                Ok((ingestion_config, flows))
            }

            IngestionConfigSelector::Form {
                asset_name,
                client_key,
                flows,
            } => {
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

                            for existing_flow in
                                existing_flows.iter().filter(|ef| ef.name == flow.name)
                            {
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
                                .try_create_flows(
                                    &ingestion_config.ingestion_config_id,
                                    &flows_to_create,
                                )
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
                        }

                        Ok((ingestion_config, flows))
                    }
                }
            }
        }
    }
}
