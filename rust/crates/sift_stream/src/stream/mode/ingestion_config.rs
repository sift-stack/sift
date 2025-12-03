use super::super::{SiftStream, SiftStreamMode, channel::ChannelValue, time::TimeValue};
use crate::{
    metrics::SiftStreamMetrics,
    stream::{
        flow::{FlowBuilder, FlowDescriptor},
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
    pub(crate) run: Option<Run>,
    ingestion_config: IngestionConfig,
    flows_by_name: HashMap<String, FlowDescriptor<String>>,
    flows_seen: HashSet<String>,
    sift_stream_id: Uuid,
    message_id_counter: u64,

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
        flows: Vec<FlowConfig>,
        run: Option<Run>,
        task_config: TaskConfig,
        metrics: Arc<SiftStreamMetrics>,
    ) -> Result<Self> {
        let ingestion_config_id = ingestion_config.ingestion_config_id.clone();
        let mut flows_by_name =
            HashMap::<String, FlowDescriptor<String>>::with_capacity(flows.len());

        for flow in flows {
            let flow_name = flow.name.clone();
            let flow_descriptor = FlowDescriptor::try_from((&ingestion_config_id, flow))?;
            flows_by_name.insert(flow_name, flow_descriptor);
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
                message_id_counter: 0,
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

        let run_id = self.mode.run.as_ref().map(|r| r.run_id.clone());

        let Some(flows) = self.mode.flows_by_name.get(&message.flow_name) else {
            #[cfg(feature = "tracing")]
            tracing::warn!(
                sift_stream_id = self.mode.sift_stream_id.to_string(),
                "flow '{}' not found in local flow cache - message will still be transmitted but will not show in Sift if the flow was not registered",
                message.flow_name,
            );
            let req = Self::message_to_ingest_req_direct(
                &message,
                &self.mode.ingestion_config.ingestion_config_id,
                run_id,
            );
            return self.send_impl(req);
        };
        let Some(req) = Self::message_to_ingest_req(
            &message,
            self.mode.run.as_ref().map(|r| r.run_id.clone()),
            flows,
        ) else {
            #[cfg(feature = "tracing")]
            tracing::warn!(
                sift_stream_id = self.mode.sift_stream_id.to_string(),
                values = format!("{message:?}"),
                "encountered a message that doesn't match any cached flows - message will still be transmitted but will not show in Sift if the flow was not registered"
            );
            let req = Self::message_to_ingest_req_direct(
                &message,
                &self.mode.ingestion_config.ingestion_config_id,
                run_id,
            );
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

    /// This method offers a way to send data in a manner that's identical to the raw
    /// [`gRPC service`] for ingestion-config based streaming. Users are expected to handle
    /// channel value ordering as well as empty values correctly.
    ///
    /// ### Important
    ///
    /// Note if using this interface, you should use [FlowBuilder::request] to ensure proper
    /// building of the request.
    ///
    /// [`gRPC service`]: https://github.com/sift-stack/sift/blob/main/protos/sift/ingest/v1/ingest.proto#L11
    pub fn send_requests_nonblocking<I>(&mut self, requests: I) -> Result<()>
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
    fn send_impl(&mut self, request: IngestWithConfigDataStreamRequest) -> Result<()> {
        #[cfg(feature = "tracing")]
        {
            if !self.mode.flows_seen.contains(&request.flow) {
                self.metrics.unique_flows_received.increment();
                self.mode.flows_seen.insert(request.flow.clone());
                tracing::info!(
                    sift_stream_id = self.mode.sift_stream_id.to_string(),
                    "flow '{}' being ingested for the first time",
                    &request.flow,
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
            message_id: self.mode.message_id_counter,
            request: Arc::new(request),
            dropped_for_ingestion: false,
        };

        self.mode.message_id_counter += 1;

        // Send the message for backup first. If this fails, log an error and continue.
        //
        // Failure to backup can lead to data loss though it is preferable to attempt
        // to stream the message to Sift rather than return the error and prevent both.
        //
        // TODO(tsift): Make this behavior optional via a builder arg.
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
                //
                // On failure, rely on metrics to track occurences. Logging can quickly become spammy as
                // the system works through bursts of messages so logs are reduced to the debug level.
                if let Err(e) = self
                    .mode
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
                        sift_stream_id = self.mode.sift_stream_id.to_string(),
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

    /// Modify the existing ingestion config by adding new flows that weren't accounted for during
    /// initialization.
    pub async fn add_new_flows(&mut self, flow_configs: &[FlowConfig]) -> Result<()> {
        // Filter out flows that already exist.
        let filtered = flow_configs
            .iter()
            .filter(|f| !self.mode.flows_by_name.contains_key(&f.name))
            .collect::<Vec<_>>();

        // If no new flows are provided, return early.
        if filtered.is_empty() {
            return Ok(());
        }

        #[cfg(feature = "tracing")]
        tracing::info!(
            ingestion_config_id = self.mode.ingestion_config.ingestion_config_id,
            new_flows = filtered
                .iter()
                .map(|f| f.name.as_str())
                .collect::<Vec<&str>>()
                .join(","),
            "adding new flows to ingestion config"
        );

        let mut calls = Vec::with_capacity(filtered.len());
        let create_flows = filtered.into_iter().cloned().collect::<Vec<FlowConfig>>();
        for flow_config in create_flows.iter() {
            let channel = self.grpc_channel.clone();
            let ingestion_config_id = self.mode.ingestion_config.ingestion_config_id.clone();
            let flow_config = flow_config.clone();

            calls.push(tokio::spawn(async move {
                new_ingestion_config_service(channel)
                    .try_create_flows(&ingestion_config_id, vec![flow_config])
                    .await
                    .context("SiftStream::add_new_flows")
            }));
        }

        // Wait for all the gRPC calls to complete.
        let results = futures::future::join_all(calls).await;

        let mut add_config = |config: &FlowConfig| -> Result<()> {
            let flow_name = config.name.clone();
            let flow_descriptor = FlowDescriptor::try_from((
                self.mode.ingestion_config.ingestion_config_id.clone(),
                config,
            ))?;
            self.mode.flows_by_name.insert(flow_name, flow_descriptor);

            #[cfg(feature = "tracing")]
            tracing::info!(
                sift_stream_id = self.mode.sift_stream_id.to_string(),
                flow = config.name,
                "successfully registered new flow"
            );

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
                    tracing::error!(
                        sift_stream_id = self.mode.sift_stream_id.to_string(),
                        "failed to create flow {}: {e}",
                        config.name,
                    );
                }
                Err(e) => {
                    #[cfg(feature = "tracing")]
                    tracing::error!(
                        sift_stream_id = self.mode.sift_stream_id.to_string(),
                        "failed to create flow {}: {e}",
                        config.name,
                    );
                }
            }
        }

        self.metrics
            .loaded_flows
            .add(self.mode.flows_by_name.len() as u64);

        Ok(())
    }

    /// Get a copy of the current flow descriptors known to SiftStream as a HashMap keyed to the flow name.
    /// This includes flows provided at initialization, and any existing configs
    /// previously registered in Sift
    pub fn get_flows(&self) -> HashMap<String, FlowDescriptor<String>> {
        // Currently we get the first FlowConfig provided in the Vec to match how send() validates flows
        self.mode
            .flows_by_name
            .iter()
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect()
    }

    /// Get the flow descriptor for a given flow name.
    pub fn get_flow_descriptor(&self, flow_name: &str) -> Result<FlowDescriptor<String>> {
        self.mode
            .flows_by_name
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

        // Finally, wait for the metrics streaming task to complete.
        if let Some(metrics_streaming) = self.mode.stream_system.metrics_streaming {
            let _ = metrics_streaming.await;
        }

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
        run_id: Option<String>,
        descriptor: &FlowDescriptor<String>,
    ) -> Option<IngestWithConfigDataStreamRequest> {
        // Create a vector of empty channel values. If the provided channel values
        // have a matching channel name and data type, the value will be updated.
        let mut builder = FlowBuilder::new(descriptor);

        // Update all provided channel values in the flow.
        for value in message.values.iter() {
            builder
                .set_with_key(&value.name, value.value.clone())
                .ok()?;
        }

        // Attach the run ID to the flow if it is provided.
        if let Some(run_id) = run_id.as_ref() {
            builder.attach_run_id(run_id);
        }

        Some(builder.request(message.timestamp.clone()))
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
