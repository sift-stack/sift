use super::super::{SiftStream, SiftStreamMode, channel::ChannelValue, time::TimeValue};
use crate::{
    metrics::SiftStreamMetrics,
    stream::{
        run::{RunSelector, load_run_by_form, load_run_by_id},
        tasks::{ControlMessage, DataMessage, StreamSystem, TaskConfig, start_tasks},
    },
};

#[cfg(feature = "metrics-unstable")]
use crate::metrics::register_metrics;

use futures_core::Stream;
use prost::Message;
use sift_error::prelude::*;
use sift_rs::{
    ingest::v1::{IngestWithConfigDataChannelValue, IngestWithConfigDataStreamRequest},
    ingestion_configs::v2::{FlowConfig, IngestionConfig},
    runs::v2::Run,
    wrappers::ingestion_configs::{IngestionConfigServiceWrapper, new_ingestion_config_service},
};
use std::{
    collections::{HashMap, HashSet},
    pin::Pin,
    sync::Arc,
    task::{Context, Poll},
};
use tokio::sync::broadcast;
use tokio_stream::wrappers::BroadcastStream;
use uuid::Uuid;

/// Dependencies specifically for ingestion-config based streaming. Users shouldn't have to
/// interact with this directly.
pub struct IngestionConfigMode {
    pub(crate) run: Option<Run>,
    ingestion_config: IngestionConfig,
    flows_by_name: HashMap<String, Vec<FlowConfig>>,
    flows_seen: HashSet<String>,
    sift_stream_id: Uuid,

    // Task-based architecture components for non-blocking operation
    stream_system: StreamSystem,
}

impl SiftStreamMode for IngestionConfigMode {}

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
        flows: Vec<FlowConfig>,
        run: Option<Run>,
        task_config: TaskConfig,
        metrics: Arc<SiftStreamMetrics>,
    ) -> Result<Self> {
        let mut flows_by_name = HashMap::<String, Vec<FlowConfig>>::new();

        for flow in flows {
            flows_by_name
                .entry(flow.name.clone())
                .and_modify(|group| group.push(flow.clone()))
                .or_insert_with(|| vec![flow]);
        }

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
                flows_by_name,
                flows_seen: HashSet::new(),
                sift_stream_id,
                run,
                stream_system,
            },
            metrics,
        })
    }

    /// The entry-point to send actual telemetry to Sift in the form of [Flow]s. If a `message` is
    /// sent that doesn't match any flows that [SiftStream] catches locally, the message will
    /// still be transmitted and a warning log emitted. If users are certain that the message
    /// corresponds to an unregistered flow then [SiftStream::add_new_flows] should be called first
    /// to register the flow before calling [SiftStream::send]; otherwise users should monitor the
    /// Sift DLQ either in the Sift UI or Sift API to ensure successful transmission.
    ///
    /// When "sending" messages, first the message will sent to the backup system. This system
    /// is used to backup data to disk until the data is confirmed received by Sift. If streaming
    /// encounters errors, the backed up data will be re-ingested ensuring all data is received
    /// by Sift.
    ///
    /// If the backup system has fallen behind and the backup queue/channel is full, it will
    /// proceed to sending the message to Sift.
    ///
    /// This ensures data is sent to Sift even if the backup system is lagging.
    pub async fn send(&mut self, message: Flow) -> Result<()> {
        self.metrics.messages_received.increment();

        let ingestion_config_id = &self.mode.ingestion_config.ingestion_config_id;
        let run_id = self.mode.run.as_ref().map(|r| r.run_id.clone());

        let Some(flows) = self.mode.flows_by_name.get(&message.flow_name) else {
            #[cfg(feature = "tracing")]
            tracing::warn!(
                sift_stream_id = self.mode.sift_stream_id.to_string(),
                "flow '{}' not found in local flow cache - message will still be transmitted but will not show in Sift if the flow was not registered",
                message.flow_name,
            );
            let req = Self::message_to_ingest_req_direct(&message, ingestion_config_id, run_id);
            return self.send_impl(req);
        };
        let Some(req) = Self::message_to_ingest_req(
            &message,
            &self.mode.ingestion_config.ingestion_config_id,
            self.mode.run.as_ref().map(|r| r.run_id.clone()),
            flows,
        ) else {
            #[cfg(feature = "tracing")]
            tracing::warn!(
                sift_stream_id = self.mode.sift_stream_id.to_string(),
                values = format!("{message:?}"),
                "encountered a message that doesn't match any cached flows - message will still be transmitted but will not show in Sift if the flow was not registered"
            );
            let req = Self::message_to_ingest_req_direct(&message, ingestion_config_id, run_id);
            return self.send_impl(req);
        };
        self.send_impl(req)
    }

    /// This method offers a way to send data in a manner that's identical to the raw
    /// [`gRPC service`] for ingestion-config based streaming. Users are expected to handle
    /// channel value ordering as well as empty values correctly.
    ///
    /// ### Important
    ///
    /// Note that most users should prefer to use [SiftStream::send]. This method primarily exists
    /// to make is easier for existing integrations to utilize `sift-stream`.
    ///
    /// [`gRPC service`]: https://github.com/sift-stack/sift/blob/main/protos/sift/ingest/v1/ingest.proto#L11
    pub async fn send_requests<I>(&mut self, requests: I) -> Result<()>
    where
        I: IntoIterator<Item = IngestWithConfigDataStreamRequest>,
    {
        for req in requests {
            self.metrics.messages_received.increment();
            self.send_impl(req)?;
        }
        Ok(())
    }

    /// Concerned with sending the actual ingest request to [DataStream] which will then write it
    /// to the gRPC stream. If backups are enabled, the request will be backed up as well.
    fn send_impl(&mut self, req: IngestWithConfigDataStreamRequest) -> Result<()> {
        #[cfg(feature = "tracing")]
        {
            if !self.mode.flows_seen.contains(&req.flow) {
                self.metrics.unique_flows_received.increment();
                self.mode.flows_seen.insert(req.flow.clone());
                tracing::info!(
                    sift_stream_id = self.mode.sift_stream_id.to_string(),
                    "flow '{}' being ingested for the first time",
                    &req.flow,
                );
            }
        }

        // Track the channel depths.
        self.metrics
            .ingestion_channel_depth
            .set(self.mode.stream_system.ingestion_tx.len() as u64);
        self.metrics
            .backup_channel_depth
            .set(self.mode.stream_system.backup_tx.len() as u64);

        let data_msg = DataMessage {
            request: req.clone(),
            dropped_for_ingestion: false,
        };

        // Send the message for backup first. If this fails, log an error and continue.
        //
        // Failure to backup can lead to data loss though it is preferable to attempt
        // to stream the message to Sift rather than return the error and prevent both.
        if let Err(e) = self.mode.stream_system.backup_tx.try_send(data_msg.clone()) {
            #[cfg(feature = "tracing")]
            tracing::warn!(
                sift_stream_id = self.mode.sift_stream_id.to_string(),
                "failed to send data to backup system, data will still be streamed to Sift: {e}"
            );
        }

        self.metrics.messages_sent_to_backup.increment();

        // Send the message for ingestion.
        //
        // If the channel is full, the oldest message will be removed in order to create space for the newer message.
        // For ingestion, newer data is preferred over older data.
        match self.mode.stream_system.ingestion_tx.force_send(data_msg) {
            Ok(None) => Ok(()),
            Ok(Some(mut oldest_message)) => {
                #[cfg(feature = "tracing")]
                tracing::debug!(
                    sift_stream_id = self.mode.sift_stream_id.to_string(),
                    "data channel full, dropping oldest message"
                );

                oldest_message.dropped_for_ingestion = true;
                self.metrics.old_messages_dropped_for_ingestion.increment();
                self.metrics.messages_sent_to_backup.increment();
                self.metrics.checkpoint.failed_checkpoint_count.increment();

                // Re-send the oldest message to the backup to ensure it is re-ingested later despite being
                // dropped from the ingestion channel.
                self.mode
                    .stream_system
                    .backup_tx
                    .try_send(oldest_message)
                    .map_err(|e| Error::new(ErrorKind::StreamError, e))
                    .context("failed to send data to backup task system")
            }
            Err(e) => Err(Error::new_msg(
                ErrorKind::StreamError,
                format!("queueing data for ingestion failed: {e}"),
            )),
        }
    }

    /// Modify the existing ingestion config by adding new flows that weren't accounted for during
    /// initialization.
    pub async fn add_new_flows(&mut self, flow_configs: &[FlowConfig]) -> Result<()> {
        new_ingestion_config_service(self.grpc_channel.clone())
            .try_create_flows(
                &self.mode.ingestion_config.ingestion_config_id,
                flow_configs,
            )
            .await
            .context("SiftStream::add_new_flows")?;

        self.metrics.loaded_flows.add(flow_configs.len() as u64);

        for flow_config in flow_configs {
            self.mode
                .flows_by_name
                .entry(flow_config.name.clone())
                .and_modify(|flows| flows.push(flow_config.clone()))
                .or_insert_with(|| vec![flow_config.clone()]);

            #[cfg(feature = "tracing")]
            tracing::info!(
                sift_stream_id = self.mode.sift_stream_id.to_string(),
                flow = flow_config.name,
                "successfully registered new flow"
            );
        }
        Ok(())
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

        self.mode.run = Some(run);

        Ok(())
    }

    /// Detach the run, if any, associated with the stream. Any data provided through [SiftStream::send] after
    /// this function is called will not be associated with a run.
    pub fn detach_run(&mut self) {
        self.mode.run = None;
    }

    /// This will conclude the stream and return when Sift has sent its final response. It is
    /// important that this method be called in order to obtain the final checkpoint
    /// acknowledgement from Sift, otherwise some tail-end data may fail to send.
    pub async fn finish(self) -> Result<()> {
        // Close the data channels.
        drop(self.mode.stream_system.ingestion_tx);
        drop(self.mode.stream_system.backup_tx);

        // Send the shutdown signal to the tasks.
        self.mode
            .stream_system
            .control_tx
            .send(ControlMessage::Shutdown)
            .map_err(|e| Error::new(ErrorKind::StreamError, e))
            .context("failed to send shutdown signal to task-based architecture")?;

        // Wait for the tasks to complete.
        let _ = tokio::try_join!(
            self.mode.stream_system.backup_manager,
            self.mode.stream_system.ingestion,
            self.mode.stream_system.reingestion,
        );

        #[cfg(feature = "tracing")]
        tracing::info!(
            sift_stream_id = self.mode.sift_stream_id.to_string(),
            asset_id = self.mode.ingestion_config.asset_id,
            ingestion_config_id = self.mode.ingestion_config.ingestion_config_id,
            run = self.mode.run.map(|r| r.name).unwrap_or_default(),
            "successfully shutdown streaming system"
        );

        Ok(())
    }

    /// Flows passed into this function should have names match `flow_name`. The only case
    /// in which this returns `None` is if there is no [FlowConfig] for the given `message`.
    pub(crate) fn message_to_ingest_req(
        message: &Flow,
        ingestion_config_id: &str,
        run_id: Option<String>,
        flows: &[FlowConfig],
    ) -> Option<IngestWithConfigDataStreamRequest> {
        // Find the flow config for the given flow name.
        let found_flow = flows.iter().find(|f| f.name == message.flow_name)?;

        // Create a vector of empty channel values. If the provided channel values
        // have a matching channel name and data type, the value will be updated.
        let mut channel_values = found_flow
            .channels
            .iter()
            .map(|_| IngestWithConfigDataChannelValue {
                r#type: Some(ChannelValue::empty_pb()),
            })
            .collect::<Vec<IngestWithConfigDataChannelValue>>();

        // Create a map of channel name and data type to the index of the channel in the vector
        // so we can update the channel value if it matches.
        let channel_map: HashMap<(&str, i32), usize> = found_flow
            .channels
            .iter()
            .enumerate()
            .map(|(i, channel)| ((channel.name.as_str(), channel.data_type), i))
            .collect();

        for v in &message.values {
            let i = channel_map.get(&(v.name.as_str(), v.pb_data_type()))?;
            channel_values[*i].r#type = Some(v.pb_value());
        }

        let request = IngestWithConfigDataStreamRequest {
            flow: message.flow_name.to_string(),
            ingestion_config_id: ingestion_config_id.to_string(),
            timestamp: Some(message.timestamp.0),
            run_id: run_id.unwrap_or_default(),
            channel_values,
            ..Default::default()
        };

        Some(request)
    }

    /// Creates an [IngestWithConfigDataStreamRequest] directly without consulting the flow cache.
    pub(crate) fn message_to_ingest_req_direct(
        message: &Flow,
        ingestion_config_id: &str,
        run_id: Option<String>,
    ) -> IngestWithConfigDataStreamRequest {
        let channel_values = message
            .values
            .iter()
            .map(|val| IngestWithConfigDataChannelValue {
                r#type: Some(val.pb_value()),
            })
            .collect::<Vec<_>>();

        IngestWithConfigDataStreamRequest {
            channel_values,
            flow: message.flow_name.to_string(),
            ingestion_config_id: ingestion_config_id.to_string(),
            timestamp: Some(message.timestamp.0),
            run_id: run_id.unwrap_or_default(),
            ..Default::default()
        }
    }
}

impl DataStream {
    pub(crate) fn new(
        data_rx: async_channel::Receiver<DataMessage>,
        control_tx: broadcast::Sender<ControlMessage>,
        sift_stream_id: Uuid,
        metrics: Arc<SiftStreamMetrics>,
    ) -> Self {
        let control_rx = BroadcastStream::new(control_tx.subscribe());
        Self {
            data_rx: Box::pin(data_rx),
            control_rx: Box::pin(control_rx),
            sift_stream_id,
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
            Poll::Ready(Some(DataMessage { request, .. })) => {
                let message_size = request.encoded_len() as u64;
                self.metrics.messages_sent.increment();
                self.metrics.checkpoint.cur_messages_sent.increment();
                self.metrics.bytes_sent.add(message_size);
                self.metrics.checkpoint.cur_bytes_sent.add(message_size);
                Poll::Ready(Some(request))
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
