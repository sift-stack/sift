use crate::FlowBuilder;
use crate::metrics::{SiftStreamMetrics, SiftStreamMetricsSnapshot};
use crate::stream::send_error::{SendError, TrySendError};
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

impl LiveStreaming {
    fn prepare_message(
        &mut self,
        stream_id: &Uuid,
        message: IngestWithConfigDataStreamRequest,
    ) -> DataMessage {
        #[cfg(feature = "tracing")]
        {
            if !self.flows_seen.contains(&message.flow) {
                self.metrics.unique_flows_received.increment();
                self.flows_seen.insert(message.flow.clone());
                tracing::info!(sift_stream_id = %stream_id, "flow '{}' being ingested for the first time", &message.flow);
            }
        }

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
        data_msg
    }

    /// Used by `async fn send`. If an oldest message is evicted from the ingestion
    /// channel, awaits until backup has space to accept it. Returns the undeliverable
    /// message on backup channel close or ingestion channel close.
    async fn dispatch_to_ingestion(
        &mut self,
        stream_id: &Uuid,
        data_msg: DataMessage,
    ) -> Option<IngestWithConfigDataStreamRequest> {
        match self.stream_system.ingestion_tx.force_send(data_msg) {
            Ok(None) => None,
            Ok(Some(mut oldest)) => {
                oldest.dropped_for_ingestion = true;
                self.metrics.old_messages_dropped_for_ingestion.increment();
                self.metrics.checkpoint.failed_checkpoint_count.increment();
                // Block until backup has space.
                match self.stream_system.backup_tx.send(oldest).await {
                    Ok(()) => {
                        self.metrics.messages_sent_to_backup.increment();
                        None
                    }
                    Err(async_channel::SendError(dm)) => {
                        self.metrics
                            .old_messages_failed_adding_to_backup
                            .increment();
                        #[cfg(feature = "tracing")]
                        tracing::debug!(sift_stream_id = %stream_id, "backup channel closed while dispatching evicted message");
                        Some(Arc::try_unwrap(dm.request).unwrap_or_else(|arc| (*arc).clone()))
                    }
                }
            }
            Err(async_channel::SendError(dm)) => {
                // ingestion channel closed — return the message to the caller
                #[cfg(feature = "tracing")]
                tracing::debug!(sift_stream_id = %stream_id, "ingestion channel closed");
                Some(Arc::try_unwrap(dm.request).unwrap_or_else(|arc| (*arc).clone()))
            }
        }
    }

    /// Used by `fn try_send`. If an oldest message is evicted from the ingestion
    /// channel and backup is full or closed, returns the evicted message to the
    /// caller. Also returns the message when the ingestion channel itself is closed.
    fn try_dispatch_to_ingestion(
        &mut self,
        stream_id: &Uuid,
        data_msg: DataMessage,
    ) -> Option<IngestWithConfigDataStreamRequest> {
        match self.stream_system.ingestion_tx.force_send(data_msg) {
            Ok(None) => None,
            Ok(Some(mut oldest)) => {
                oldest.dropped_for_ingestion = true;
                self.metrics.old_messages_dropped_for_ingestion.increment();
                self.metrics.checkpoint.failed_checkpoint_count.increment();
                match self.stream_system.backup_tx.try_send(oldest) {
                    Ok(()) => {
                        self.metrics.messages_sent_to_backup.increment();
                        None
                    }
                    Err(async_channel::TrySendError::Full(dm)) => {
                        self.metrics
                            .old_messages_failed_adding_to_backup
                            .increment();
                        Some(Arc::try_unwrap(dm.request).unwrap_or_else(|arc| (*arc).clone()))
                    }
                    Err(async_channel::TrySendError::Closed(dm)) => {
                        self.metrics
                            .old_messages_failed_adding_to_backup
                            .increment();
                        #[cfg(feature = "tracing")]
                        tracing::debug!(sift_stream_id = %stream_id, "backup channel closed while dispatching evicted message");
                        Some(Arc::try_unwrap(dm.request).unwrap_or_else(|arc| (*arc).clone()))
                    }
                }
            }
            Err(async_channel::SendError(dm)) => {
                // ingestion channel closed — return the message to the caller
                #[cfg(feature = "tracing")]
                tracing::debug!(sift_stream_id = %stream_id, "ingestion channel closed");
                Some(Arc::try_unwrap(dm.request).unwrap_or_else(|arc| (*arc).clone()))
            }
        }
    }
}

#[async_trait]
impl Transport for LiveStreaming {
    type Encoder = IngestionConfigEncoder;
    type Message = IngestWithConfigDataStreamRequest;

    /// Sends a message, awaiting capacity if the stream is busy.
    ///
    /// This mode prioritizes freshness: newer messages always make it into the stream, and
    /// older buffered messages are displaced to make room when necessary. As a result, if
    /// the stream is closed while a displaced message is being handled, **the message
    /// returned inside `Err` may be older than the message provided**. An error is only
    /// returned on stream close; normal backpressure is handled transparently by waiting.
    ///
    /// Because displaced messages are not automatically retried, enabling file backup
    /// recovery mode is strongly recommended. When enabled, displaced messages are written
    /// to a local file and can be replayed to Sift at a later time, ensuring no data goes
    /// unrecorded.
    async fn send(
        &mut self,
        stream_id: &Uuid,
        message: Self::Message,
    ) -> std::result::Result<(), SendError<Self::Message>> {
        let data_msg = self.prepare_message(stream_id, message);

        self.stream_system
            .backup_tx
            .send(data_msg.clone())
            .await
            .map_err(|async_channel::SendError(dm)| {
                SendError(Arc::try_unwrap(dm.request).unwrap_or_else(|arc| (*arc).clone()))
            })?;

        self.metrics.messages_sent_to_backup.increment();
        if let Some(displaced) = self.dispatch_to_ingestion(stream_id, data_msg).await {
            return Err(SendError(displaced));
        }
        Ok(())
    }

    /// Attempts to send a message without blocking.
    ///
    /// Returns immediately with `TrySendError::Full` or `TrySendError::Closed` if the
    /// stream cannot accept data right now.
    ///
    /// This mode prioritizes freshness: newer messages always make it into the stream, and
    /// older buffered messages are displaced to make room when necessary. If such a
    /// displacement occurs and the stream is full or closed at that point, **the message
    /// returned inside `Err` may be older than the message provided**. Enabling file
    /// backup recovery mode is strongly recommended so that any displaced messages are
    /// written to a local file and can be replayed to Sift later.
    fn try_send(
        &mut self,
        stream_id: &Uuid,
        message: Self::Message,
    ) -> std::result::Result<(), TrySendError<Self::Message>> {
        let data_msg = self.prepare_message(stream_id, message);

        match self.stream_system.backup_tx.try_send(data_msg.clone()) {
            Ok(()) => {}
            Err(async_channel::TrySendError::Full(dm)) => {
                return Err(TrySendError::Full(
                    Arc::try_unwrap(dm.request).unwrap_or_else(|arc| (*arc).clone()),
                ));
            }
            Err(async_channel::TrySendError::Closed(dm)) => {
                return Err(TrySendError::Closed(
                    Arc::try_unwrap(dm.request).unwrap_or_else(|arc| (*arc).clone()),
                ));
            }
        }

        self.metrics.messages_sent_to_backup.increment();
        if let Some(displaced) = self.try_dispatch_to_ingestion(stream_id, data_msg) {
            return Err(TrySendError::Full(displaced));
        }
        Ok(())
    }

    /// Sends a batch of messages in order, awaiting capacity for each one.
    ///
    /// On stream close, stops immediately and returns the undelivered messages starting
    /// from the point of failure. Because this mode may displace older buffered messages
    /// to make room for newer ones (see [`send`](Self::send)), the first element of the
    /// returned `Vec` may be an older displaced message rather than the one that was being
    /// sent at the time of failure.
    async fn send_requests<I>(
        &mut self,
        stream_id: &Uuid,
        requests: I,
    ) -> std::result::Result<(), SendError<Vec<Self::Message>>>
    where
        I: IntoIterator<Item = Self::Message> + Send,
        I::IntoIter: Send,
    {
        let mut iter = requests.into_iter();
        while let Some(msg) = iter.next() {
            if let Err(SendError(failed)) = self.send(stream_id, msg).await {
                let mut undelivered = vec![failed];
                undelivered.extend(iter);
                return Err(SendError(undelivered));
            }
        }
        Ok(())
    }

    /// Attempts to send a batch of messages in order without blocking.
    ///
    /// Stops and returns on the first failure. The returned `Vec` contains the undelivered
    /// messages starting from the point of failure. Because this mode may displace older
    /// buffered messages to make room for newer ones (see [`try_send`](Self::try_send)),
    /// the first element may be an older displaced message rather than the one that was
    /// being sent at the time of failure.
    fn try_send_requests<I>(
        &mut self,
        stream_id: &Uuid,
        requests: I,
    ) -> std::result::Result<(), TrySendError<Vec<Self::Message>>>
    where
        I: IntoIterator<Item = Self::Message> + Send,
        I::IntoIter: Send,
    {
        let mut iter = requests.into_iter();
        while let Some(msg) = iter.next() {
            match self.try_send(stream_id, msg) {
                Ok(()) => {}
                Err(TrySendError::Full(failed)) => {
                    let mut undelivered = vec![failed];
                    undelivered.extend(iter);
                    return Err(TrySendError::Full(undelivered));
                }
                Err(TrySendError::Closed(failed)) => {
                    let mut undelivered = vec![failed];
                    undelivered.extend(iter);
                    return Err(TrySendError::Closed(undelivered));
                }
            }
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::stream::tasks::DataMessage;
    use std::collections::HashSet;

    fn make_request() -> IngestWithConfigDataStreamRequest {
        IngestWithConfigDataStreamRequest {
            ingestion_config_id: uuid::Uuid::new_v4().to_string(),
            flow: "test_flow".to_string(),
            timestamp: None,
            channel_values: vec![],
            run_id: String::new(),
            end_stream_on_validation_error: false,
            organization_id: String::new(),
        }
    }

    fn make_live_streaming(
        ingestion_capacity: usize,
        backup_capacity: usize,
    ) -> (
        LiveStreaming,
        async_channel::Receiver<DataMessage>,
        async_channel::Receiver<DataMessage>,
    ) {
        let (control_tx, _) = broadcast::channel(10);
        let (ingestion_tx, ingestion_rx) = async_channel::bounded(ingestion_capacity);
        let (backup_tx, backup_rx) = async_channel::bounded(backup_capacity);

        let system = StreamSystem {
            backup_manager: tokio::spawn(async { Ok(()) }),
            ingestion: tokio::spawn(async { Ok(()) }),
            reingestion: tokio::spawn(async { Ok(()) }),
            metrics_streaming: None,
            control_tx,
            ingestion_tx,
            backup_tx,
        };

        let live = LiveStreaming {
            message_id_counter: 0,
            stream_system: system,
            flows_seen: HashSet::new(),
            metrics: Arc::new(crate::metrics::SiftStreamMetrics::default()),
        };

        (live, ingestion_rx, backup_rx)
    }

    #[tokio::test]
    async fn test_try_send_backup_closed_returns_closed() {
        let (mut live, _ingestion_rx, backup_rx) = make_live_streaming(10, 10);
        drop(backup_rx);
        let stream_id = uuid::Uuid::new_v4();
        let req = make_request();
        let flow = req.flow.clone();
        let err = live.try_send(&stream_id, req).unwrap_err();
        assert!(err.is_closed(), "expected Closed, got {err}");
        assert_eq!(err.into_inner().flow, flow);
    }

    #[tokio::test]
    async fn test_try_send_backup_full_returns_full() {
        let (mut live, _ingestion_rx, backup_rx) = make_live_streaming(10, 1);
        // Pre-fill the backup channel so the next try_send finds it full.
        let dummy = DataMessage {
            message_id: 0,
            request: Arc::new(make_request()),
            dropped_for_ingestion: false,
        };
        live.stream_system.backup_tx.try_send(dummy).unwrap();

        let stream_id = uuid::Uuid::new_v4();
        let req = make_request();
        let flow = req.flow.clone();
        let err = live.try_send(&stream_id, req).unwrap_err();
        assert!(err.is_full(), "expected Full, got {err}");
        assert_eq!(err.into_inner().flow, flow);
        drop(backup_rx);
    }

    #[tokio::test]
    async fn test_send_backup_closed_returns_send_error() {
        let (mut live, _ingestion_rx, backup_rx) = make_live_streaming(10, 10);
        drop(backup_rx);
        let stream_id = uuid::Uuid::new_v4();
        let req = make_request();
        let flow = req.flow.clone();
        let err = live.send(&stream_id, req).await.unwrap_err();
        assert_eq!(err.into_inner().flow, flow);
    }

    #[tokio::test]
    async fn test_send_blocks_until_backup_space_available() {
        let (mut live, _ingestion_rx, backup_rx) = make_live_streaming(10, 1);
        // Fill the backup channel so send will have to wait.
        let dummy = DataMessage {
            message_id: 0,
            request: Arc::new(make_request()),
            dropped_for_ingestion: false,
        };
        live.stream_system.backup_tx.try_send(dummy).unwrap();

        // After a short delay, consume the dummy so send can proceed.
        tokio::spawn(async move {
            tokio::time::sleep(std::time::Duration::from_millis(20)).await;
            let _ = backup_rx.recv().await;
            // Keep the receiver alive so the channel stays open.
            tokio::time::sleep(std::time::Duration::from_millis(100)).await;
        });

        let stream_id = uuid::Uuid::new_v4();
        live.send(&stream_id, make_request()).await.unwrap();
    }

    #[tokio::test]
    async fn test_try_send_requests_returns_undelivered_on_full() {
        let (mut live, _ingestion_rx, backup_rx) = make_live_streaming(10, 1);
        let dummy = DataMessage {
            message_id: 0,
            request: Arc::new(make_request()),
            dropped_for_ingestion: false,
        };
        live.stream_system.backup_tx.try_send(dummy).unwrap();

        let stream_id = uuid::Uuid::new_v4();
        let reqs = vec![make_request(), make_request(), make_request()];
        let err = live.try_send_requests(&stream_id, reqs).unwrap_err();
        assert!(err.is_full(), "expected Full, got {err}");
        assert_eq!(err.into_inner().len(), 3);
        drop(backup_rx);
    }

    #[tokio::test]
    async fn test_send_requests_returns_undelivered_on_closed() {
        let (mut live, _ingestion_rx, backup_rx) = make_live_streaming(10, 10);
        drop(backup_rx);

        let stream_id = uuid::Uuid::new_v4();
        let reqs = vec![make_request(), make_request(), make_request()];
        let err = live.send_requests(&stream_id, reqs).await.unwrap_err();
        assert_eq!(err.into_inner().len(), 3);
    }
}
