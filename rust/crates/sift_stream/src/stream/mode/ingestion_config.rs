use crate::FlowBuilder;
use crate::metrics::{SiftStreamMetrics, SiftStreamMetricsSnapshot};
use crate::stream::{Encodeable, Encoder, MetricsSnapshot, Transport};
use crate::stream::{
    SiftStream,
    channel::ChannelValue,
    flow::FlowDescriptor,
    private::Sealed,
    tasks::{ControlMessage, DataMessage, StreamSystem, TaskConfig, start_tasks},
    time::TimeValue,
};

#[cfg(feature = "metrics-unstable")]
use crate::metrics::register_metrics;

use async_trait::async_trait;
use futures_core::Stream;
use prost::Message;
use sift_error::prelude::*;
use sift_rs::SiftChannel;
use sift_rs::ingestion_configs::v2::FlowConfig;
use sift_rs::retry::{RetryConfig, RetryExt};
use sift_rs::wrappers::ingestion_configs::{
    IngestionConfigServiceWrapper, new_ingestion_config_service,
};
use sift_rs::{
    ingest::v1::IngestWithConfigDataStreamRequest, ingestion_configs::v2::IngestionConfig,
    runs::v2::Run,
};
use std::collections::HashSet;
use std::{
    collections::HashMap,
    pin::Pin,
    sync::{
        Arc,
        atomic::{AtomicU64, Ordering},
    },
    task::{Context, Poll},
};
use tokio::sync::broadcast;
use tokio_stream::wrappers::BroadcastStream;
use uuid::Uuid;

/// Dependencies specifically for ingestion-config based streaming. Users shouldn't have to
/// interact with this directly.
pub struct LiveStreaming {
    message_id_counter: u64,

    // Task-based architecture components for non-blocking operation
    stream_system: StreamSystem,

    flows_seen: HashSet<String>,
    metrics: Arc<SiftStreamMetrics>,
}

// Seal the trait - only this crate can implement SiftStreamMode
impl Sealed for LiveStreaming {}

#[async_trait]
impl Transport for LiveStreaming {
    type Encoder = IngestionConfigEncoder;
    type Message = IngestWithConfigDataStreamRequest;

    /// Sends the message to Sift for live ingestion, while in parallel also sends a backup of the message to a file.
    fn send(&mut self, stream_id: &Uuid, message: Self::Message) -> Result<()> {
        #[cfg(feature = "tracing")]
        {
            if !self.flows_seen.contains(&message.flow) {
                self.metrics.unique_flows_received.increment();
                self.flows_seen.insert(message.flow.clone());
                tracing::info!(
                    sift_stream_id = %stream_id,
                    "flow '{}' being ingested for the first time",
                    &message.flow,
                );
            }
        }

        // Track the channel depths.
        self.metrics
            .ingestion_channel_depth
            .set(self.stream_system.ingestion_tx.len() as u64);
        self.metrics
            .backup_channel_depth
            .set(self.stream_system.backup_tx.len() as u64);

        self.metrics.messages_received.increment();

        let data_msg = DataMessage {
            message_id: self.message_id_counter,
            request: Arc::new(message),
            dropped_for_ingestion: false,
        };

        self.message_id_counter += 1;

        // Send the message for backup first. If this fails, log an error and continue.
        //
        // Failure to backup can lead to data loss though it is preferable to attempt
        // to stream the message to Sift rather than return the error and prevent both.
        //
        // TODO(tsift): Make this behavior optional via a builder arg.
        if let Err(e) = self.stream_system.backup_tx.try_send(data_msg.clone()) {
            #[cfg(feature = "tracing")]
            tracing::warn!(
                sift_stream_id = %stream_id,
                "failed to send data to backup system, data will still be streamed to Sift: {e}"
            );
        }

        self.metrics.messages_sent_to_backup.increment();

        // Send the message for ingestion.
        //
        // If the channel is full, the oldest message will be removed in order to create space for the newer message.
        // For ingestion, newer data is preferred over older data.
        match self.stream_system.ingestion_tx.force_send(data_msg) {
            Ok(None) => Ok(()),
            Ok(Some(mut oldest_message)) => {
                #[cfg(feature = "tracing")]
                tracing::debug!(
                    sift_stream_id = %stream_id,
                    "data channel full, dropping oldest message"
                );

                oldest_message.dropped_for_ingestion = true;
                self.metrics.old_messages_dropped_for_ingestion.increment();
                self.metrics.messages_sent_to_backup.increment();
                self.metrics.checkpoint.failed_checkpoint_count.increment();

                // Re-send the oldest message to the backup to ensure it is re-ingested later despite being
                // dropped from the ingestion channel.
                //
                // On failure, rely on metrics to track occurences. Logging can quickly become spammy as
                // the system works through bursts of messages so logs are reduced to the debug level.
                if let Err(e) = self
                    .stream_system
                    .backup_tx
                    .try_send(oldest_message)
                    .map_err(|e| Error::new(ErrorKind::StreamError, e))
                    .context("failed to send data to backup task system")
                {
                    self.metrics
                        .old_messages_failed_adding_to_backup
                        .increment();

                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        sift_stream_id = %stream_id,
                        "failed to send oldest data to backup task system: {e}"
                    );
                }

                // Do not interupt ingestion.
                Ok(())
            }
            Err(e) => Err(Error::new_msg(
                ErrorKind::StreamError,
                format!("queueing data for ingestion failed: {e}"),
            )),
        }
    }

    fn send_requests<I>(&mut self, stream_id: &Uuid, requests: I) -> Result<()>
    where
        I: IntoIterator<Item = Self::Message> + Send,
        I::IntoIter: Send,
    {
        for req in requests {
            self.send(stream_id, req)?;
        }
        Ok(())
    }

    /// This will conclude the stream and return when Sift has sent its final response. It is
    /// important that this method be called in order to obtain the final checkpoint
    /// acknowledgement from Sift, otherwise some tail-end data may fail to send.
    async fn finish(self, stream_id: &Uuid) -> Result<()> {
        // Close the data channels.
        drop(self.stream_system.ingestion_tx);
        drop(self.stream_system.backup_tx);

        // Send the shutdown signal to the tasks.
        self.stream_system
            .control_tx
            .send(ControlMessage::Shutdown)
            .map_err(|e| Error::new(ErrorKind::StreamError, e))
            .context("failed to send shutdown signal to task-based architecture")?;

        // Wait for the tasks to complete.
        let _ = tokio::try_join!(
            self.stream_system.backup_manager,
            self.stream_system.ingestion,
            self.stream_system.reingestion,
        );

        // Finally, wait for the metrics streaming task to complete.
        if let Some(metrics_streaming) = self.stream_system.metrics_streaming {
            let _ = metrics_streaming.await;
        }

        #[cfg(feature = "tracing")]
        tracing::info!(
            sift_stream_id = %stream_id,
            "successfully shutdown streaming system"
        );

        Ok(())
    }
}

pub struct IngestionConfigEncoder {
    pub(crate) grpc_channel: SiftChannel,
    pub(crate) flows_by_name: HashMap<String, FlowDescriptor<String>>,
    pub(crate) ingestion_config: IngestionConfig,
    pub(crate) metrics: Arc<SiftStreamMetrics>,
}

impl Encoder for IngestionConfigEncoder {
    type Message = IngestWithConfigDataStreamRequest;
}

impl MetricsSnapshot for IngestionConfigEncoder {
    fn snapshot(&self) -> SiftStreamMetricsSnapshot {
        self.metrics.snapshot()
    }
}

impl Sealed for IngestionConfigEncoder {}

impl IngestionConfigEncoder {
    fn ingestion_config_id(&self) -> &str {
        &self.ingestion_config.ingestion_config_id
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
            ingestion_config_id = self.ingestion_config_id(),
            new_flows = filtered
                .iter()
                .map(|f| f.name.as_str())
                .collect::<Vec<&str>>()
                .join(","),
            "adding new flows to ingestion config"
        );

        let mut calls = Vec::with_capacity(filtered.len());
        let create_flows = filtered.into_iter().cloned().collect::<Vec<FlowConfig>>();
        let ingestion_config_id = self.ingestion_config_id().to_string();

        for flow_config in create_flows.iter() {
            let channel = self.grpc_channel.clone();
            let config_id = ingestion_config_id.clone();
            let flow_config = flow_config.clone();

            calls.push(tokio::spawn(async move {
                let wrapper = new_ingestion_config_service(channel);
                let retrying = wrapper.retrying(RetryConfig::default());
                retrying
                    .call(|mut w| {
                        let config_id = config_id.clone();
                        let flow_config = flow_config.clone();
                        async move { w.try_create_flows(&config_id, vec![flow_config]).await }
                    })
                    .await
                    .context("SiftStream::add_new_flows")
            }));
        }

        // Wait for all the gRPC calls to complete.
        let results = futures::future::join_all(calls).await;

        let mut add_config = |config: &FlowConfig| -> Result<()> {
            let flow_name = config.name.clone();
            let flow_descriptor = FlowDescriptor::try_from((self.ingestion_config_id(), config))?;
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
}

/// A single message that users can send to Sift via [SiftStream::send]. It is expected that this
/// flow has a corresponding flow configuration specified in the ingestion config. See the
/// [top-level documentation](crate#ingestion-configs) for more details.
#[derive(Debug, Clone)]
pub struct Flow {
    pub flow_name: String,
    pub timestamp: TimeValue,
    pub values: Vec<ChannelValue>,
}

impl Encodeable for Flow {
    type Output = IngestWithConfigDataStreamRequest;
    type Encoder = IngestionConfigEncoder;

    fn encode(
        self,
        encoder: &mut Self::Encoder,
        stream_id: &Uuid,
        run: Option<&Run>,
    ) -> Option<Self::Output> {
        let req = if let Some(flows) = encoder.flows_by_name.get(&self.flow_name) {
            if let Some(req) = super::super::helpers::message_to_ingest_req(&self, run, flows) {
                req
            } else {
                #[cfg(feature = "tracing")]
                tracing::warn!(
                    sift_stream_id = %stream_id,
                    values = format!("{:?}", self.flow_name),
                    "encountered a message that doesn't match any cached flows - message will still be written to file"
                );
                super::super::helpers::message_to_ingest_req_direct(
                    &self,
                    encoder.ingestion_config_id(),
                    run,
                )
            }
        } else {
            #[cfg(feature = "tracing")]
            tracing::warn!(
                sift_stream_id = %stream_id,
                "flow '{}' not found in local flow cache - message will still be written to file",
                self.flow_name,
            );
            super::super::helpers::message_to_ingest_req_direct(
                &self,
                encoder.ingestion_config_id(),
                run,
            )
        };

        Some(req)
    }
}

impl<K> Encodeable for FlowBuilder<'_, K>
where
    K: Eq + core::hash::Hash,
{
    type Output = IngestWithConfigDataStreamRequest;
    type Encoder = IngestionConfigEncoder;

    fn encode(
        mut self,
        _: &mut Self::Encoder,
        _: &Uuid,
        run: Option<&Run>,
    ) -> Option<Self::Output> {
        if let Some(run) = run {
            self.attach_run_id(run.run_id.clone());
        }

        Some(self.request(TimeValue::now()))
    }
}
/// Dependencies used in the Tokio task that actually sends the data to Sift.
pub(crate) struct DataStream {
    data_rx: Pin<Box<async_channel::Receiver<DataMessage>>>,
    control_rx: Pin<Box<BroadcastStream<ControlMessage>>>,
    sift_stream_id: Uuid,
    saw_first_message: bool,
    first_message_id: Arc<AtomicU64>,
    last_message_id: Arc<AtomicU64>,
    metrics: Arc<SiftStreamMetrics>,
}

impl Flow {
    /// Initializes a new flow that can be immediately sent to Sift by passing this to
    /// [SiftStream::send].
    pub fn new<S>(flow_name: S, timestamp: TimeValue, values: &[ChannelValue]) -> Self
    where
        S: ToString,
    {
        Self {
            timestamp,
            flow_name: flow_name.to_string(),
            values: values.to_vec(),
        }
    }
}

impl SiftStream<IngestionConfigEncoder, LiveStreaming> {
    /// Initializes a new [SiftStream]. Users should instead use [`SiftStreamBuilder`].
    ///
    /// [`SiftStreamBuilder`]: crate::stream::builder::SiftStreamBuilder
    pub(crate) async fn new(
        ingestion_config: IngestionConfig,
        flows_by_name: HashMap<String, FlowDescriptor<String>>,
        run: Option<Run>,
        task_config: TaskConfig,
        metrics: Arc<SiftStreamMetrics>,
    ) -> Result<Self> {
        // Spawn a task to register metrics without blocking
        #[cfg(feature = "metrics-unstable")]
        {
            let uuid = task_config.sift_stream_id.to_string();
            let metrics = metrics.clone();
            tokio::spawn(async move {
                register_metrics(uuid, metrics).await;
            });
        }

        metrics.loaded_flows.add(flows_by_name.len() as u64);
        let sift_stream_id = task_config.sift_stream_id;

        // Use the setup channel for API calls that are not related to ingestion to avoid multiplexing
        // on the ingestion channel and potentially starving out ingestion.
        let grpc_channel = task_config.setup_channel.clone();

        let stream_system = start_tasks(task_config)
            .await
            .context("failed to start task-based architecture")?;

        Ok(Self {
            grpc_channel: grpc_channel.clone(),
            encoder: IngestionConfigEncoder {
                grpc_channel,
                flows_by_name,
                ingestion_config,
                metrics: metrics.clone(),
            },
            transport: LiveStreaming {
                message_id_counter: 0,
                stream_system,
                flows_seen: HashSet::new(),
                metrics,
            },
            run,
            sift_stream_id,
        })
    }
}

impl DataStream {
    pub(crate) fn new(
        data_rx: async_channel::Receiver<DataMessage>,
        control_tx: broadcast::Sender<ControlMessage>,
        sift_stream_id: Uuid,
        first_message_id: Arc<AtomicU64>,
        last_message_id: Arc<AtomicU64>,
        metrics: Arc<SiftStreamMetrics>,
    ) -> Self {
        let control_rx = BroadcastStream::new(control_tx.subscribe());
        Self {
            data_rx: Box::pin(data_rx),
            control_rx: Box::pin(control_rx),
            sift_stream_id,
            saw_first_message: false,
            first_message_id,
            last_message_id,
            metrics,
        }
    }
}

impl Stream for DataStream {
    type Item = IngestWithConfigDataStreamRequest;

    fn poll_next(mut self: Pin<&mut Self>, ctx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        // Close the stream if a checkpoint complete signal is received.
        if matches!(
            self.control_rx.as_mut().poll_next(ctx),
            Poll::Ready(Some(Ok(ControlMessage::SignalNextCheckpoint)))
        ) {
            return Poll::Ready(None);
        }

        // Continue with data streaming.
        match self.data_rx.as_mut().poll_next(ctx) {
            Poll::Ready(Some(DataMessage {
                message_id,
                request,
                ..
            })) => {
                if !self.saw_first_message {
                    self.saw_first_message = true;
                    self.first_message_id.store(message_id, Ordering::Relaxed);
                }
                self.last_message_id.store(message_id, Ordering::Relaxed);

                let message_size = request.encoded_len() as u64;
                self.metrics.messages_sent.increment();
                self.metrics.checkpoint.cur_messages_sent.increment();
                self.metrics.bytes_sent.add(message_size);
                self.metrics.checkpoint.cur_bytes_sent.add(message_size);

                // NOTE: This will copy the request which can be expensive.
                Poll::Ready(Some((*request).clone()))
            }
            Poll::Ready(None) => {
                // All senders dropped.. conclude stream
                #[cfg(feature = "tracing")]
                tracing::debug!(
                    sift_stream_id = %self.sift_stream_id,
                    "received signal to conclude SiftStream"
                );
                Poll::Ready(None)
            }
            Poll::Pending => Poll::Pending,
        }
    }
}
