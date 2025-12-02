use crate::metrics::SiftStreamMetrics;
use crate::stream::flow::FlowDescriptor;
use crate::stream::run::{RunSelector, load_run_by_form, load_run_by_id};
use async_trait::async_trait;
use sift_connect::SiftChannel;
use sift_error::prelude::*;
use sift_rs::{
    ingest::v1::IngestWithConfigDataStreamRequest,
    ingestion_configs::v2::FlowConfig,
    runs::v2::Run,
    wrappers::ingestion_configs::{IngestionConfigServiceWrapper, new_ingestion_config_service},
};
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use uuid::Uuid;

#[cfg(feature = "metrics-unstable")]
use crate::metrics::SiftStreamMetricsSnapshot;

/// Concerned with building and configuring and instance of [SiftStream].
pub mod builder;

/// Concerned with constructing values for channels/sensors that get telemetered.
pub mod channel;

/// Shared helper functions used across stream implementations.
mod helpers;

/// Implementations for different modes of streaming.
pub mod mode;

/// Concerned with gRPC retries.
pub mod retry;
pub use retry::RetryPolicy;

/// Concerned with accessing or creating runs for [SiftStream]
pub mod run;

/// Concerned with constructing values of time that make up the time-series sent ot Sift.
pub mod time;

/// Concerned with validating flows and detecting if changes are being made to an ingestion config
/// in a manner that isn't backwards compatible.
pub(crate) mod flow;

/// Task-based architecture for non-blocking SiftStream operations
pub mod tasks;

#[cfg(test)]
mod test;

/// [SiftStream] is a smart wrapper over an actual gRPC stream that makes it robust and more
/// ergonomic to work with. Some additional behaviors that [SiftStream] supports are:
/// - Checkpointing
/// - Retries (disabled by default)
/// - Backups (disabled by default)
/// - Tracing and ingestion metrics
///
/// To initialize a [SiftStream] users will use [builder::SiftStreamBuilder]. Refer to the
/// [crate-level documentation](crate) for further details and examples.
pub struct SiftStream<M: SiftStreamMode> {
    grpc_channel: SiftChannel,
    mode: M,
    metrics: Arc<SiftStreamMetrics>,
    flows_by_name: HashMap<String, FlowDescriptor<String>>,
    run: Option<Run>,
    flows_seen: HashSet<String>,
    sift_stream_id: Uuid,
}

impl<M: SiftStreamMode> SiftStream<M> {
    #[cfg(feature = "metrics-unstable")]
    /// Retrieve a snapshot of the current metrics for this stream.
    pub fn get_metrics_snapshot(&self) -> SiftStreamMetricsSnapshot {
        self.metrics.snapshot()
    }

    /// Modify the existing ingestion config by adding new flows that weren't accounted for during
    /// initialization. This will register the flows with Sift.
    pub async fn add_new_flows(&mut self, flow_configs: &[FlowConfig]) -> Result<()> {
        // Filter out flows that already exist.
        let filtered = flow_configs
            .iter()
            .filter(|f| !self.flows_by_name.contains_key(&f.name))
            .collect::<Vec<_>>();

        // If no new flows are provided, return early.
        if filtered.is_empty() {
            return Ok(());
        }

        #[cfg(feature = "tracing")]
        tracing::info!(
            ingestion_config_id = self.mode.ingestion_config_id(),
            new_flows = filtered
                .iter()
                .map(|f| f.name.as_str())
                .collect::<Vec<&str>>()
                .join(","),
            "adding new flows to ingestion config"
        );

        let mut calls = Vec::with_capacity(filtered.len());
        let create_flows = filtered.into_iter().cloned().collect::<Vec<FlowConfig>>();
        let ingestion_config_id = self.mode.ingestion_config_id().to_string();

        for flow_config in create_flows.iter() {
            let channel = self.grpc_channel.clone();
            let config_id = ingestion_config_id.clone();
            let flow_config = flow_config.clone();

            calls.push(tokio::spawn(async move {
                new_ingestion_config_service(channel)
                    .try_create_flows(&config_id, vec![flow_config])
                    .await
                    .context("SiftStream::add_new_flows")
            }));
        }

        // Wait for all the gRPC calls to complete.
        let results = futures::future::join_all(calls).await;

        let mut add_config = |config: &FlowConfig| -> Result<()> {
            let flow_name = config.name.clone();
            let flow_descriptor =
                FlowDescriptor::try_from((self.mode.ingestion_config_id(), config))?;
            self.flows_by_name.insert(flow_name, flow_descriptor);

            #[cfg(feature = "tracing")]
            tracing::info!(flow = config.name, "successfully registered new flow");

            Ok(())
        };

        // Iterate over the results and update the flow cache for the successfully created flows.
        for (config, result) in create_flows.iter().zip(results.into_iter()) {
            match result {
                Ok(Ok(())) => {
                    add_config(config)?;
                }
                Ok(Err(e)) if e.kind() == ErrorKind::AlreadyExistsError => {
                    add_config(config)?;
                }
                Ok(Err(e)) => {
                    #[cfg(feature = "tracing")]
                    tracing::error!("failed to create flow {}: {e}", config.name,);
                }
                Err(e) => {
                    #[cfg(feature = "tracing")]
                    tracing::error!("failed to create flow {}: {e}", config.name,);
                }
            }
        }

        self.metrics
            .loaded_flows
            .add(self.flows_by_name.len() as u64);

        Ok(())
    }

    /// Get a copy of the current flow descriptors known to SiftStream as a HashMap keyed to the flow name.
    pub fn get_flows(&self) -> HashMap<String, FlowDescriptor<String>> {
        self.flows_by_name
            .iter()
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect()
    }

    /// Get the flow descriptor for a given flow name.
    pub fn get_flow_descriptor(&self, flow_name: &str) -> Result<FlowDescriptor<String>> {
        self.flows_by_name
            .get(flow_name)
            .cloned()
            .ok_or(Error::new_msg(
                ErrorKind::NotFoundError,
                format!("flow '{}' not found", flow_name),
            ))
    }

    /// Attach a run to the stream. Any data provided through [SiftStream::send] after return
    /// of this function will be associated with the run.
    pub async fn attach_run(&mut self, run_selector: RunSelector) -> Result<()> {
        let run = match run_selector {
            RunSelector::ById(run_id) => load_run_by_id(self.grpc_channel.clone(), &run_id).await?,
            RunSelector::ByForm(run_form) => {
                load_run_by_form(self.grpc_channel.clone(), run_form).await?
            }
        };

        self.run = Some(run);

        Ok(())
    }

    /// Detach the run, if any, associated with the stream. Any data provided through [SiftStream::send] after
    /// this function is called will not be associated with a run.
    pub fn detach_run(&mut self) {
        self.run = None;
    }

    /// Retrieves the attached run if it exists.
    pub fn run(&self) -> Option<&Run> {
        self.run.as_ref()
    }

    /// The entry-point to send actual telemetry to Sift in the form of [Flow]s.
    pub async fn send(&mut self, message: mode::ingestion_config::Flow) -> Result<()> {
        let mut ctx = SendContext {
            metrics: &self.metrics,
            run: &self.run,
            flows_by_name: &self.flows_by_name,
            flows_seen: &mut self.flows_seen,
            sift_stream_id: &self.sift_stream_id,
        };
        self.mode.send(&mut ctx, message).await
    }

    /// This method offers a way to send data in a manner that's identical to the raw
    /// [`gRPC service`] for ingestion-config based streaming.
    ///
    /// [`gRPC service`]: https://github.com/sift-stack/sift/blob/main/protos/sift/ingest/v1/ingest.proto#L11
    pub async fn send_requests<I>(&mut self, requests: I) -> Result<()>
    where
        I: IntoIterator<Item = IngestWithConfigDataStreamRequest> + Send,
        I::IntoIter: Send,
    {
        let mut ctx = SendContext {
            metrics: &self.metrics,
            run: &self.run,
            flows_by_name: &self.flows_by_name,
            flows_seen: &mut self.flows_seen,
            sift_stream_id: &self.sift_stream_id,
        };
        self.mode.send_requests(&mut ctx, requests).await
    }

    /// This method offers a way to send data in a manner that's identical to the raw
    /// [`gRPC service`] for ingestion-config based streaming.
    ///
    /// [`gRPC service`]: https://github.com/sift-stack/sift/blob/main/protos/sift/ingest/v1/ingest.proto#L11
    pub fn send_requests_nonblocking<I>(&mut self, requests: I) -> Result<()>
    where
        I: IntoIterator<Item = IngestWithConfigDataStreamRequest> + Send,
        I::IntoIter: Send,
    {
        let mut ctx = SendContext {
            metrics: &self.metrics,
            run: &self.run,
            flows_by_name: &self.flows_by_name,
            flows_seen: &mut self.flows_seen,
            sift_stream_id: &self.sift_stream_id,
        };
        self.mode.send_requests_nonblocking(&mut ctx, requests)
    }

    /// Gracefully finish the stream, draining any remaining data before returning.
    ///
    /// It is important to always call this method when you are done sending data and
    /// before the object is dropped.
    pub async fn finish(mut self) -> Result<()> {
        let mut ctx = SendContext {
            metrics: &self.metrics,
            run: &self.run,
            flows_by_name: &self.flows_by_name,
            flows_seen: &mut self.flows_seen,
            sift_stream_id: &self.sift_stream_id,
        };

        self.mode.finish(&mut ctx).await
    }
}

/// Context passed to mode implementations for send operations.
///
/// This is an internal implementation detail and should not be used directly.
/// It is only visible in the trait signature because it's used by [`SiftStreamMode`],
/// which is sealed and cannot be implemented by external code.
#[doc(hidden)]
pub struct SendContext<'a> {
    pub(crate) metrics: &'a Arc<SiftStreamMetrics>,
    pub(crate) run: &'a Option<Run>,
    pub(crate) flows_by_name: &'a HashMap<String, FlowDescriptor<String>>,
    pub(crate) flows_seen: &'a mut HashSet<String>,
    pub(crate) sift_stream_id: &'a Uuid,
}

/// Sealed trait to prevent external implementations of `SiftStreamMode`.
mod private {
    /// This trait is sealed and cannot be implemented outside this crate.
    ///
    /// It is public so it can be used as a supertrait, but the module is private,
    /// preventing external code from implementing it.
    pub trait Sealed {}
}

/// A trait that defines a particular mode of streaming. Modes must implement the send APIs
/// to handle data transmission in their specific way.
///
/// This trait is sealed and cannot be implemented by external code. Only the crate's
/// internal mode implementations (`IngestionConfigMode` and `FileBackupMode`) can implement it.
///
/// The `private_in_public` warnings are suppressed at the module level because
/// `SendContext` is intentionally private - external code cannot implement this trait
/// (it's sealed), so they will never need to use it.
#[async_trait]
pub trait SiftStreamMode: private::Sealed {
    /// Returns the ingestion config ID for this mode.
    fn ingestion_config_id(&self) -> &str;

    /// Send a flow message. The mode implementation handles the actual transmission logic.
    async fn send(
        &mut self,
        ctx: &mut SendContext<'_>,
        message: mode::ingestion_config::Flow,
    ) -> Result<()>;

    /// Send multiple requests. The mode implementation handles the actual transmission logic.
    async fn send_requests<I>(&mut self, ctx: &mut SendContext<'_>, requests: I) -> Result<()>
    where
        I: IntoIterator<Item = IngestWithConfigDataStreamRequest> + Send,
        I::IntoIter: Send;

    /// Send multiple requests in a non-blocking manner. The mode implementation handles the actual transmission logic.
    fn send_requests_nonblocking<I>(
        &mut self,
        ctx: &mut SendContext<'_>,
        requests: I,
    ) -> Result<()>
    where
        I: IntoIterator<Item = IngestWithConfigDataStreamRequest> + Send,
        I::IntoIter: Send;

    /// Finish the stream. The mode implementation handles the actual cleanup logic.
    async fn finish(self, ctx: &mut SendContext<'_>) -> Result<()>;
}
