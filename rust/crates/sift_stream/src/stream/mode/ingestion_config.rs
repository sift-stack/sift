use crate::metrics::SiftStreamMetrics;
use crate::stream::{
    SendContext, SiftStream, SiftStreamMode,
    channel::ChannelValue,
    flow::FlowDescriptor,
    helpers,
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
use sift_rs::{
    ingest::v1::IngestWithConfigDataStreamRequest, ingestion_configs::v2::IngestionConfig,
    runs::v2::Run,
};
use std::{
    collections::{HashMap, HashSet},
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
pub struct IngestionConfigMode {
    ingestion_config: IngestionConfig,
    message_id_counter: u64,

    // Task-based architecture components for non-blocking operation
    stream_system: StreamSystem,
}

// Seal the trait - only this crate can implement SiftStreamMode
impl Sealed for IngestionConfigMode {}

#[async_trait]
impl SiftStreamMode for IngestionConfigMode {
    fn ingestion_config_id(&self) -> &str {
        &self.ingestion_config.ingestion_config_id
    }

    async fn send(&mut self, ctx: &mut SendContext<'_>, message: Flow) -> Result<()> {
        ctx.metrics.messages_received.increment();

        let run_id = ctx.run.as_ref().map(|r| r.run_id.clone());

        let Some(flows) = ctx.flows_by_name.get(&message.flow_name) else {
            #[cfg(feature = "tracing")]
            tracing::warn!(
                sift_stream_id = ctx.sift_stream_id.to_string(),
                "flow '{}' not found in local flow cache - message will still be transmitted but will not show in Sift if the flow was not registered",
                message.flow_name,
            );
            let req = helpers::message_to_ingest_req_direct(
                &message,
                &self.ingestion_config.ingestion_config_id,
                run_id,
            );
            return self.send_impl(ctx, req);
        };
        let Some(req) = helpers::message_to_ingest_req(
            &message,
            ctx.run.as_ref().map(|r| r.run_id.clone()),
            flows,
        ) else {
            #[cfg(feature = "tracing")]
            tracing::warn!(
                sift_stream_id = ctx.sift_stream_id.to_string(),
                values = format!("{message:?}"),
                "encountered a message that doesn't match any cached flows - message will still be transmitted but will not show in Sift if the flow was not registered"
            );
            let req = helpers::message_to_ingest_req_direct(
                &message,
                &self.ingestion_config.ingestion_config_id,
                run_id,
            );
            return self.send_impl(ctx, req);
        };
        self.send_impl(ctx, req)
    }

    async fn send_requests<I>(&mut self, ctx: &mut SendContext<'_>, requests: I) -> Result<()>
    where
        I: IntoIterator<Item = IngestWithConfigDataStreamRequest> + Send,
        I::IntoIter: Send,
    {
        for req in requests {
            ctx.metrics.messages_received.increment();
            self.send_impl(ctx, req)?;
        }
        Ok(())
    }

    fn send_requests_nonblocking<I>(&mut self, ctx: &mut SendContext<'_>, requests: I) -> Result<()>
    where
        I: IntoIterator<Item = IngestWithConfigDataStreamRequest> + Send,
        I::IntoIter: Send,
    {
        for req in requests {
            ctx.metrics.messages_received.increment();
            self.send_impl(ctx, req)?;
        }
        Ok(())
    }

    /// This will conclude the stream and return when Sift has sent its final response. It is
    /// important that this method be called in order to obtain the final checkpoint
    /// acknowledgement from Sift, otherwise some tail-end data may fail to send.
    async fn finish(self, ctx: &mut SendContext<'_>) -> Result<()> {
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
            sift_stream_id = ctx.sift_stream_id.to_string(),
            asset_id = self.ingestion_config.asset_id,
            ingestion_config_id = self.ingestion_config.ingestion_config_id,
            run = ctx.run.as_ref().map(|r| r.name.clone()).unwrap_or_default(),
            "successfully shutdown streaming system"
        );

        Ok(())
    }
}

impl IngestionConfigMode {
    /// Concerned with sending the actual ingest request to [DataStream] which will then write it
    /// to the gRPC stream. If backups are enabled, the request will be backed up as well.
    fn send_impl(
        &mut self,
        ctx: &mut SendContext<'_>,
        request: IngestWithConfigDataStreamRequest,
    ) -> Result<()> {
        #[cfg(feature = "tracing")]
        {
            if !ctx.flows_seen.contains(&request.flow) {
                ctx.metrics.unique_flows_received.increment();
                ctx.flows_seen.insert(request.flow.clone());
                tracing::info!(
                    sift_stream_id = ctx.sift_stream_id.to_string(),
                    "flow '{}' being ingested for the first time",
                    &request.flow,
                );
            }
        }

        // Track the channel depths.
        ctx.metrics
            .ingestion_channel_depth
            .set(self.stream_system.ingestion_tx.len() as u64);
        ctx.metrics
            .backup_channel_depth
            .set(self.stream_system.backup_tx.len() as u64);

        let data_msg = DataMessage {
            message_id: self.message_id_counter,
            request: Arc::new(request),
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
                sift_stream_id = ctx.sift_stream_id.to_string(),
                "failed to send data to backup system, data will still be streamed to Sift: {e}"
            );
        }

        ctx.metrics.messages_sent_to_backup.increment();

        // Send the message for ingestion.
        //
        // If the channel is full, the oldest message will be removed in order to create space for the newer message.
        // For ingestion, newer data is preferred over older data.
        match self.stream_system.ingestion_tx.force_send(data_msg) {
            Ok(None) => Ok(()),
            Ok(Some(mut oldest_message)) => {
                #[cfg(feature = "tracing")]
                tracing::debug!(
                    sift_stream_id = ctx.sift_stream_id.to_string(),
                    "data channel full, dropping oldest message"
                );

                oldest_message.dropped_for_ingestion = true;
                ctx.metrics.old_messages_dropped_for_ingestion.increment();
                ctx.metrics.messages_sent_to_backup.increment();
                ctx.metrics.checkpoint.failed_checkpoint_count.increment();

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
                    ctx.metrics.old_messages_failed_adding_to_backup.increment();

                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        sift_stream_id = ctx.sift_stream_id.to_string(),
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

impl SiftStream<IngestionConfigMode> {
    /// Initializes a new [SiftStream]. Users should instead use [`SiftStreamBuilder`].
    ///
    /// [`SiftStreamBuilder`]: crate::stream::builder::SiftStreamBuilder
    pub(crate) fn new(
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

        let stream_system =
            start_tasks(task_config).context("failed to start task-based architecture")?;

        Ok(Self {
            grpc_channel,
            mode: IngestionConfigMode {
                ingestion_config,
                stream_system,
                message_id_counter: 0,
            },
            metrics,
            flows_by_name,
            run,
            flows_seen: HashSet::new(),
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
                    sift_stream_id = self.sift_stream_id.to_string(),
                    "received signal to conclude SiftStream"
                );
                Poll::Ready(None)
            }
            Poll::Pending => Poll::Pending,
        }
    }
}
