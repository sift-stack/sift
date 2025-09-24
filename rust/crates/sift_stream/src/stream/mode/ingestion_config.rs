use super::super::{
    RetryPolicy, SiftStream, SiftStreamMode, channel::ChannelValue, time::TimeValue,
};
use crate::{
    backup::disk::AsyncBackupsManager,
    metrics::SiftStreamMetrics,
    stream::run::{RunSelector, load_run_by_form, load_run_by_id},
};
use futures_core::Stream;
use prost::Message;
use sift_connect::SiftChannel;
use sift_error::prelude::*;
use sift_rs::{
    ingest::v1::{
        IngestWithConfigDataChannelValue, IngestWithConfigDataStreamRequest,
        IngestWithConfigDataStreamResponse, ingest_service_client::IngestServiceClient,
    },
    ingestion_configs::v2::{FlowConfig, IngestionConfig},
    runs::v2::Run,
    wrappers::ingestion_configs::{IngestionConfigServiceWrapper, new_ingestion_config_service},
};
use std::{
    collections::{HashMap, HashSet},
    ops::Drop,
    pin::Pin,
    sync::{
        Arc,
        mpsc::{Receiver as StdReceiver, Sender as StdSender},
    },
    task::{Context, Poll},
    time::{Duration, Instant},
};
use tokio::{
    sync::{
        Notify,
        mpsc::{
            Receiver as BoundedReceiver, Sender as BoundedSender, channel as bounded_channel,
            error::SendError,
        },
        oneshot::{self, Receiver, Sender},
    },
    task::JoinHandle,
};
use uuid::Uuid;

/// The size of the channel buffer that connects [SiftStream::send] with the task that actually
/// streams data to Sift.
const DATA_BUFFER_CAPACITY: usize = 10_000;

/// If Sift doesn't send back a checkpoint acknowledgement after this amount of time we will
/// forcefully end the current stream.
const FORCE_CHECKPOINT_THRESHOLD_SEC: u64 = 10;

/// Dependencies specifically for ingestion-config based streaming. Users shouldn't have to
/// interact with this directly.
pub struct IngestionConfigMode {
    pub(crate) run: Option<Run>,
    ingestion_config: IngestionConfig,
    flows_by_name: HashMap<String, Vec<FlowConfig>>,
    flows_seen: HashSet<String>,
    checkpoint_interval: Duration,
    streaming_task: Option<DataStreamTask>,
    retry_policy: Option<RetryPolicy>,
    data_tx: Option<BoundedSender<StreamMessage>>,
    shutdown_tx: Option<Sender<()>>,
    backups_manager: Option<AsyncBackupsManager<IngestWithConfigDataStreamRequest>>,
    sift_stream_id: Uuid,

    /// It's possible that [DataStream] may still have some data in its buffer by the time it gets
    /// dropped due to checkpointing or errors while streaming. This is going to be the receiving
    /// end of the drain that recovers that data and puts it into the new receiver associated with
    /// the new `data_tx` when we reinitialize the gRPC stream. When [SiftStream::finish] is
    /// called, any data still in the drain will be sent as a batch to Sift in a final stream.
    drain_rx: Option<StdReceiver<IngestWithConfigDataStreamRequest>>,
}

/// Used for fine-grain control of [DataStream].
enum StreamMessage {
    Request(IngestWithConfigDataStreamRequest),
    CheckpointSignal,
    ErrorSignal,
}

impl SiftStreamMode for IngestionConfigMode {}

/// A single message that users can send to Sift via [SiftStream::send]. It is expected that this
/// flow has a corresponding flow configuration specified in the ingestion config. See the
/// [top-level documentation](crate#ingestion-configs) for more details.
#[derive(Debug, Clone)]
pub struct Flow {
    flow_name: String,
    timestamp: TimeValue,
    values: Vec<ChannelValue>,
}

/// Dependencies used in the Tokio task that actually sends the data to Sift.
struct DataStream {
    data_rx: BoundedReceiver<StreamMessage>,
    drain_tx: StdSender<IngestWithConfigDataStreamRequest>,
    sift_stream_id: Uuid,
    heartbeat_task: JoinHandle<()>,
    started_at: Instant,
    metrics: Arc<SiftStreamMetrics>,
}

type DataStreamTask = JoinHandle<Result<IngestWithConfigDataStreamResponse>>;

impl Flow {
    /// Initializes a new flow that can be immediately sent to Sift by passing this to
    /// [SiftStream::send].
    pub fn new<S: AsRef<str>>(flow_name: S, timestamp: TimeValue, values: &[ChannelValue]) -> Self {
        Self {
            timestamp,
            flow_name: flow_name.as_ref().to_string(),
            values: values.to_vec(),
        }
    }
}

impl SiftStream<IngestionConfigMode> {
    /// Initializes a new [SiftStream]. Users should never have to call this method directly;
    /// prefer to use [`SiftStreamBuilder`].
    ///
    /// [`SiftStreamBuilder`]: crate::stream::builder::SiftStreamBuilder
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        grpc_channel: SiftChannel,
        ingestion_config: IngestionConfig,
        flows: Vec<FlowConfig>,
        run: Option<Run>,
        checkpoint_interval: Duration,
        retry_policy: Option<RetryPolicy>,
        backups_manager: Option<AsyncBackupsManager<IngestWithConfigDataStreamRequest>>,
        metrics: Arc<SiftStreamMetrics>,
    ) -> Self {
        let mut flows_by_name = HashMap::<String, Vec<FlowConfig>>::new();

        for flow in flows {
            flows_by_name
                .entry(flow.name.clone())
                .and_modify(|group| group.push(flow.clone()))
                .or_insert_with(|| vec![flow]);
        }

        let (data_tx, data_rx) = bounded_channel::<StreamMessage>(DATA_BUFFER_CAPACITY);
        let (drain_tx, drain_rx) = std::sync::mpsc::channel::<IngestWithConfigDataStreamRequest>();
        let (shutdown_tx, shutdown_rx) = oneshot::channel::<()>();
        let begin_checkpoint_notifier = Arc::new(Notify::new());
        let sift_stream_id = Uuid::new_v4();

        metrics.loaded_flows.add(flows_by_name.len() as u64);

        let data_stream = DataStream::new(sift_stream_id, data_rx, drain_tx, metrics.clone());

        let streaming_task = Self::init_streaming_task(
            grpc_channel.clone(),
            sift_stream_id,
            data_stream,
            checkpoint_interval,
            data_tx.clone(),
            shutdown_rx,
            begin_checkpoint_notifier.clone(),
            metrics.clone()
        );

        // Begin checkpoint immediately upon starting
        begin_checkpoint_notifier.notify_one();

        #[cfg(feature = "tracing")]
        tracing::info!(
            sift_stream_id = sift_stream_id.to_string(),
            "Sift streaming successfully initialized"
        );

        Self {
            grpc_channel,
            mode: IngestionConfigMode {
                ingestion_config,
                flows_by_name,
                flows_seen: HashSet::new(),
                sift_stream_id,
                run,
                streaming_task: Some(streaming_task),
                checkpoint_interval,
                data_tx: Some(data_tx),
                shutdown_tx: Some(shutdown_tx),
                drain_rx: Some(drain_rx),
                retry_policy,
                backups_manager,
            },
            metrics,
        }
    }

    pub fn get_metrics(&self) -> Arc<SiftStreamMetrics> {
        self.metrics.clone()
    }

    /// The entry-point to send actual telemetry to Sift in the form of [Flow]s. If a `message` is
    /// sent that doesn't match any flows that [SiftStream] catches locally, the message will
    /// still be transmitted and a warning log emitted. If users are certain that the message
    /// corresponds to an unregistered flow then [SiftStream::add_new_flows] should be called first
    /// to register the flow before calling [SiftStream::send]; otherwise users should monitor the
    /// Sift DLQ either in the Sift UI or Sift API to ensure successful transmission.
    ///
    /// In the case where the underlying error was closed due to an error, this method will invoke
    /// the configured [RetryPolicy] to attempt to reconnect and resend data to Sift. If the amount
    /// of retry attempts exceeds the maximum configured, then an [ErrorKind::RetriesExhausted] is
    /// returned. If backups are enabled, then all messages since the last successful checkpoint
    /// will be reingested.
    ///
    /// Lastly, if the underlying stream was gracefully closed due to a checkpoint, this method
    /// will automatically establish a new connection.
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
            return self.send_impl(req).await;
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
            return self.send_impl(req).await;
        };
        self.send_impl(req).await
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
            self.send_impl(req).await?;
        }
        Ok(())
    }

    /// Concerned with sending the actual ingest request to [DataStream] which will then write it
    /// to the gRPC stream. If backups are enabled, the request will be backed up as well.
    async fn send_impl(&mut self, req: IngestWithConfigDataStreamRequest) -> Result<()> {
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

        if self
            .backup_data(&req)
            .await
            .is_err_and(|e| e.kind() == ErrorKind::BackupLimitReached)
        {
            #[cfg(feature = "tracing")]
            tracing::info!(
                sift_stream_id = self.mode.sift_stream_id.to_string(),
                "backup size limit reached - forcing checkpoint"
            );
            if let Some(shutdown_tx) = self.mode.shutdown_tx.take() {
                let _ = shutdown_tx.send(());
            }
            if let Some(data_tx) = self.mode.data_tx.take() {
                drop(data_tx);
            }
            if let Some(streaming_task) = self.mode.streaming_task.take() {
                let checkpoint_acknowledgement = streaming_task
                    .await
                    .map_err(|e| Error::new(ErrorKind::StreamError, e))
                    .context("failed to force a checkpoint due to backup limit")
                    .help("please contact Sift");

                if let Err(err) = checkpoint_acknowledgement {
                    return self.retry(req, err).await;
                }
            }
            self.restart_stream_and_backups_manager(false).await?;
            self.backup_data(&req).await?;
        }

        let Some(data_tx) = self.mode.data_tx.as_mut() else {
            #[cfg(feature = "tracing")]
            tracing::warn!(
                sift_stream_id = self.mode.sift_stream_id.to_string(),
                "didn't expect data_tx to be missing - attempting retries but please notify Sift",
            );
            return self
                .retry(
                    req,
                    Error::new_msg(ErrorKind::StreamError, "unable to recover SiftStream"),
                )
                .await;
        };

        match data_tx.send(StreamMessage::Request(req.clone())).await {
            Ok(_) => Ok(()),

            Err(SendError(_)) => {
                #[cfg(feature = "tracing")]
                tracing::debug!(
                    sift_stream_id = self.mode.sift_stream_id.to_string(),
                    "returned Err(SendError) during data_tx.send()"
                );
                match self.mode.streaming_task.take() {
                    None => {
                        #[cfg(feature = "tracing")]
                        tracing::debug!(
                            sift_stream_id = self.mode.sift_stream_id.to_string(),
                            "No streaming task was taken. Awaiting restart_stream_and_backups_manager()"
                        );
                        self.restart_stream_and_backups_manager(false).await?;
                        Box::pin(self.send_impl(req)).await
                    }

                    Some(streaming_task) => {
                        #[cfg(feature = "tracing")]
                        tracing::debug!(
                            sift_stream_id = self.mode.sift_stream_id.to_string(),
                            "Awaiting streaming_task"
                        );
                        match streaming_task.await {
                            Ok(Ok(_)) => {
                                #[cfg(feature = "tracing")]
                                tracing::debug!(
                                    sift_stream_id = self.mode.sift_stream_id.to_string(),
                                    "Streaming_task returned Ok(). Awaiting restart_stream_and_backups_manager()"
                                );
                                self.restart_stream_and_backups_manager(false).await?;
                                Box::pin(self.send_impl(req)).await
                            }
                            Ok(Err(err)) => {
                                #[cfg(feature = "tracing")]
                                tracing::warn!(
                                    sift_stream_id = self.mode.sift_stream_id.to_string(),
                                    error = format!("{err:?}"),
                                    "encountered an error while streaming to Sift"
                                );

                                self.retry(req, err).await
                            }
                            Err(err) => {
                                #[cfg(feature = "tracing")]
                                tracing::warn!(
                                    sift_stream_id = self.mode.sift_stream_id.to_string(),
                                    error = format!("{err:?}"),
                                    "something went wrong while waiting for response from Sift"
                                );

                                self.retry(req, Error::new(ErrorKind::StreamError, err))
                                    .await
                            }
                        }
                    }
                }
            }
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

    /// Send a single data point to the backups manager to be backed up.
    async fn backup_data(&mut self, req: &IngestWithConfigDataStreamRequest) -> Result<()> {
        if let Some(backups_manager) = self.mode.backups_manager.as_mut() {
            return backups_manager.send(req.clone()).await;
        }
        Ok(())
    }

    async fn shutdown_backups_manager(
        mut backups_manager: AsyncBackupsManager<IngestWithConfigDataStreamRequest>,
    ) -> Result<()> {
        backups_manager.finish().await
    }

    async fn retry(
        &mut self,
        ingest_req: IngestWithConfigDataStreamRequest,
        err: Error,
    ) -> Result<()> {
        let Some(retry_policy) = self.mode.retry_policy.as_ref() else {
            return Err(Error::new(ErrorKind::StreamError, err))
                .context("no retry policy detected");
        };

        let mut client = IngestServiceClient::new(self.grpc_channel.clone());

        #[cfg(feature = "tracing")]
        tracing::info!(
            sift_stream_id = self.mode.sift_stream_id.to_string(),
            "stream failed - attempting retry with retry policy: {:?}",
            retry_policy
        );

        let mut current_wait = retry_policy.initial_backoff;

        for i in 1..=retry_policy.max_attempts {
            self.metrics.cur_retry_count.set(i as u64);
            #[cfg(feature = "tracing")]
            tracing::info!(
                sift_stream_id = self.mode.sift_stream_id.to_string(),
                retry_counter = i,
                "attempting retry"
            );

            tokio::time::sleep(current_wait).await;
            current_wait = (current_wait * u32::from(retry_policy.backoff_multiplier))
                .min(retry_policy.max_backoff);

            match client
                .ingest_with_config_data_stream(tokio_stream::once(ingest_req.clone()))
                .await
            {
                Ok(_) => {
                    #[cfg(feature = "tracing")]
                    tracing::info!(
                        sift_stream_id = self.mode.sift_stream_id.to_string(),
                        retry_counter = i,
                        "successful retry - re-establishing connection to Sift"
                    );

                    self.restart_stream_and_backups_manager(true).await?;

                    self.metrics.cur_retry_count.set(0);

                    return Ok(());
                }
                Err(e) =>
                {
                    #[cfg(feature = "tracing")]
                    if i < retry_policy.max_attempts {
                        tracing::warn!(
                            sift_stream_id = self.mode.sift_stream_id.to_string(),
                            retry_counter = i,
                            error = format!("{e:?}"),
                            "retry attempt failed - backing off for {}ms",
                            current_wait.as_millis()
                        );
                    } else {
                        tracing::warn!(
                            sift_stream_id = self.mode.sift_stream_id.to_string(),
                            retry_counter = i,
                            original_error = format!("{e:?}"),
                            recent_error = format!("{e:?}"),
                            "all retry attempts exhausted"
                        );
                        break;
                    }
                }
            }
        }
        Err(Error::new(ErrorKind::RetriesExhausted, err))
    }

    async fn restart_stream_and_backups_manager(
        &mut self,
        reingest_from_last_checkpoint: bool,
    ) -> Result<()> {
        let (data_tx, data_rx) = bounded_channel::<StreamMessage>(DATA_BUFFER_CAPACITY);
        let (drain_tx, drain_rx) = std::sync::mpsc::channel::<IngestWithConfigDataStreamRequest>();
        let (shutdown_tx, shutdown_rx) = oneshot::channel::<()>();
        let begin_checkpoint_notifier = Arc::new(Notify::new());
        let data_stream = DataStream::new(self.mode.sift_stream_id, data_rx, drain_tx, self.metrics.clone());

        self.mode.data_tx = Some(data_tx.clone());
        self.mode.shutdown_tx = Some(shutdown_tx);

        let streaming_task = Self::init_streaming_task(
            self.grpc_channel.clone(),
            self.mode.sift_stream_id,
            data_stream,
            self.mode.checkpoint_interval,
            data_tx.clone(),
            shutdown_rx,
            begin_checkpoint_notifier.clone(),
            self.metrics.clone()
        );
        self.mode.streaming_task = Some(streaming_task);

        if let Some(drain) = self.mode.drain_rx.replace(drain_rx) {
            while let Ok(req) = drain.try_recv() {
                if data_tx.send(StreamMessage::Request(req)).await.is_err() {
                    #[cfg(feature = "tracing")]
                    tracing::warn!(
                        sift_stream_id = self.mode.sift_stream_id.to_string(),
                        "unexpected error while draining to new buffer which may result in dropped data - please notify Sift"
                    );

                    break;
                }
            }
        }

        if reingest_from_last_checkpoint {
            // If we entered this branch this some really unexpected things happened. We're going to
            // give up on trying to rescue the backups here and just resume streaming until the
            // next checkpoint.
            if let Err(err) = self.process_backups().await {
                #[cfg(feature = "tracing")]
                {
                    tracing::debug!(
                        sift_stream_id = self.mode.sift_stream_id.to_string(),
                        error = format!("{err:?}"),
                        "not all backups were successfully processed"
                    );
                    tracing::warn!(
                        sift_stream_id = self.mode.sift_stream_id.to_string(),
                        "not all backups were successfully processed due to unexpected stream termination - retrying"
                    );
                }

                return Box::pin(self.restart_stream_and_backups_manager(false)).await;
            }
            begin_checkpoint_notifier.notify_one();
        } else {
            begin_checkpoint_notifier.notify_one();
        }

        self.restart_backup_manager().await?;

        #[cfg(feature = "tracing")]
        tracing::info!(
            sift_stream_id = self.mode.sift_stream_id.to_string(),
            "successfully initialized a new stream to Sift"
        );

        Ok(())
    }

    async fn restart_backup_manager(&mut self) -> Result<()> {
        let Some(mut backups_manager) = self.mode.backups_manager.take() else {
            return Ok(());
        };

        #[cfg(feature = "tracing")]
        tracing::info!(
            sift_stream_id = self.mode.sift_stream_id.to_string(),
            "restarting backups manager"
        );

        // Restart backup manager. Don't replace, since we don't want to drop a running stream backups task
        backups_manager.restart().await?;
        self.mode.backups_manager = Some(backups_manager);

        #[cfg(feature = "tracing")]
        tracing::info!(
            sift_stream_id = self.mode.sift_stream_id.to_string(),
            "successfully restarted backups manager"
        );

        Ok(())
    }

    async fn process_backups(&mut self) -> Result<()> {
        let Some(backup_manager) = self.mode.backups_manager.as_mut() else {
            return Ok(());
        };

        #[cfg(feature = "tracing")]
        tracing::info!(
            sift_stream_id = self.mode.sift_stream_id.to_string(),
            "processing backups"
        );

        let file_count = backup_manager.start_backup_ingestion().await;

        if file_count == 0 {
            #[cfg(feature = "tracing")]
            tracing::info!(
                sift_stream_id = self.mode.sift_stream_id.to_string(),
                "no backups to reingest"
            );
        } else {
            #[cfg(feature = "tracing")]
            tracing::info!(
                sift_stream_id = self.mode.sift_stream_id.to_string(),
                backup_files_to_ingest = file_count,
                "queued backup files since last checkpoint for ingest"
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
    pub async fn finish(mut self) -> Result<()> {
        if let Some(backup_manager) = self.mode.backups_manager {
            #[cfg(feature = "tracing")]
            tracing::info!(
                sift_stream_id = self.mode.sift_stream_id.to_string(),
                "shutting down backups manager"
            );

            let _ = Self::shutdown_backups_manager(backup_manager).await;
        }

        if let Some(streaming_task) = self.mode.streaming_task.take() {
            if let Some(shutdown_tx) = self.mode.shutdown_tx.take() {
                let _ = shutdown_tx.send(());
            }
            drop(self.mode.data_tx);

            streaming_task
                .await
                .map_err(|e| Error::new(ErrorKind::StreamError, e))
                .context("something went wrong while waiting for the final checkpoint")
                .help("please context Sift")?
                .context("final checkpoint failure")
                .help("the final checkpoint may or may not have succeeded. Please contact Sift")?;
        }

        if let Some(drain) = self.mode.drain_rx.take() {
            let mut remaining_requests = vec![];

            while let Ok(req) = drain.recv() {
                remaining_requests.push(req);
            }

            if !remaining_requests.is_empty() {
                let mut client = IngestServiceClient::new(self.grpc_channel);
                let count = remaining_requests.len();
                let request_stream = tokio_stream::iter(remaining_requests);

                client
                    .ingest_with_config_data_stream(request_stream)
                    .await
                    .map_err(|e| Error::new(ErrorKind::StreamError, e))
                    .with_context(|| {
                        format!(
                            "something went wrong trying to ingest the remaining {count} messages"
                        )
                    })
                    .help("please notify sift")?;
            }
        }

        #[cfg(feature = "tracing")]
        tracing::info!(
            sift_stream_id = self.mode.sift_stream_id.to_string(),
            asset_id = self.mode.ingestion_config.asset_id,
            ingestion_config_id = self.mode.ingestion_config.ingestion_config_id,
            run = self.mode.run.map(|r| r.name).unwrap_or_default(),
            "successfully received final checkpoint acknowledgement - concluding stream"
        );

        Ok(())
    }

    #[allow(clippy::too_many_arguments)]
    fn init_streaming_task(
        grpc_channel: SiftChannel,
        sift_stream_id: Uuid,
        mut data_stream: DataStream,
        checkpoint_interval: Duration,
        data_tx: BoundedSender<StreamMessage>,
        shutdown_rx: Receiver<()>,
        begin_checkpoint_notifier: Arc<Notify>,
        metrics: Arc<SiftStreamMetrics>,
    ) -> DataStreamTask {
        tokio::spawn(async move {
            metrics.checkpoint.next_checkpoint();

            let mut client = IngestServiceClient::new(grpc_channel);
            let force_checkpoint = Arc::new(Notify::new());
            let force_checkpoint_c = force_checkpoint.clone();

            let data_tx_c = data_tx.clone();
            let metrics_c = metrics.clone();
            let checkpoint_task = tokio::spawn(async move {
                let mut checkpoint_timer = {
                    let mut timer = tokio::time::interval(checkpoint_interval);
                    timer.tick().await;
                    timer
                };
                begin_checkpoint_notifier.notified().await;

                #[cfg(feature = "tracing")]
                tracing::debug!(
                    sift_stream_id = sift_stream_id.to_string(),
                    "received notification to start checkpoint timer"
                );

                tokio::select! {
                    _ = checkpoint_timer.tick() => {
                        metrics_c.checkpoint.checkpoint_timer_reached_cnt.increment();
                        #[cfg(feature = "tracing")]
                        tracing::info!(sift_stream_id = sift_stream_id.to_string(), "checkpoint timer elapsed - initiating checkpoint");
                    }
                    _ = shutdown_rx => {
                        metrics_c.checkpoint.checkpoint_manually_reached_cnt.increment();
                        #[cfg(feature = "tracing")]
                        tracing::info!(sift_stream_id = sift_stream_id.to_string(), "manually initiating checkpoint");
                    }
                }
                let _ = data_tx.send(StreamMessage::CheckpointSignal).await;

                tokio::spawn(async move {
                    let mut force_checkpoint_timer =
                        tokio::time::interval(Duration::from_secs(FORCE_CHECKPOINT_THRESHOLD_SEC));
                    force_checkpoint_timer.tick().await;
                    force_checkpoint_timer.tick().await;
                    force_checkpoint_c.notify_one();
                });
            });

            data_stream.started_at = Instant::now();

            tokio::select! {
                raw_response = client.ingest_with_config_data_stream(data_stream) => {
                    let response = raw_response
                        .map(|res| res.into_inner())
                        .map_err(|e| Error::new(ErrorKind::StreamError, e));

                    checkpoint_task.abort_handle().abort();
                    let _ = checkpoint_task.await;

                    match response {
                        Ok(res) => {
                            #[cfg(feature = "tracing")]
                            tracing::info!(
                                sift_stream_id = sift_stream_id.to_string(),
                                "checkpoint acknowledgement received from Sift"
                            );
                            Ok(res)
                        }
                        Err(err) => {
                            metrics.checkpoint.failed_checkpoint_count.increment();
                            #[cfg(feature = "tracing")]
                            tracing::info!(
                                sift_stream_id = sift_stream_id.to_string(),
                                "received error from Sift: {:?}",
                                err
                            );
                            let _ = data_tx_c.send(StreamMessage::ErrorSignal).await;
                            Err(err)
                        }
                    }
                }
                _ = force_checkpoint.notified() => {
                    Err(
                        Error::new_msg(ErrorKind::StreamError, "Sift took too long to give a checkpoint acknowledgement"),
                    )
                }
            }
        })
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
    fn new(
        sift_stream_id: Uuid,
        data_rx: BoundedReceiver<StreamMessage>,
        drain_tx: StdSender<IngestWithConfigDataStreamRequest>,
        metrics: Arc<SiftStreamMetrics>
    ) -> Self {
        let heartbeat_task = tokio::spawn(async move {
            loop {
                tracing::debug!(
                    sift_stream_id = sift_stream_id.to_string(),
                    "SiftStream heartbeat - healthy"
                );
                tokio::time::sleep(Duration::from_secs(30)).await;
            }
        });

        Self {
            drain_tx,
            data_rx,
            sift_stream_id,
            heartbeat_task,
            started_at: Instant::now(),
            metrics,
        }
    }
}

impl Stream for DataStream {
    type Item = IngestWithConfigDataStreamRequest;

    fn poll_next(mut self: Pin<&mut Self>, ctx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        match self.data_rx.poll_recv(ctx) {
            Poll::Ready(Some(msg)) => match msg {
                StreamMessage::Request(req) => {
                    let message_size = req.encode_length_delimited_to_vec().len() as u64;
                    self.metrics.messages_sent.increment();
                    self.metrics.checkpoint.cur_messages_sent.increment();
                    self.metrics.bytes_sent.add(message_size);
                    self.metrics.checkpoint.cur_bytes_sent.increment();
                    Poll::Ready(Some(req))
                }
                StreamMessage::CheckpointSignal => {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        sift_stream_id = self.sift_stream_id.to_string(),
                        "stream checkpoint signal received"
                    );

                    // Checkpoint was requested.. conclude stream
                    Poll::Ready(None)
                }
                StreamMessage::ErrorSignal => {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        sift_stream_id = self.sift_stream_id.to_string(),
                        "error signal received",
                    );

                    // Had error response.. conclude stream
                    Poll::Ready(None)
                }
            },
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

impl Drop for DataStream {
    fn drop(&mut self) {
        #[cfg(feature = "tracing")]
        {
            let checkpoint_stats = self.metrics.get_checkpoint_stats();
            let bytes_processed_pretty = bytesize::ByteSize::b(checkpoint_stats.bytes_sent)
                .display()
                .iec();
            let byte_rate_pretty = bytesize::ByteSize::b(checkpoint_stats.byte_rate.ceil() as u64).display().iec();

            tracing::info!(
                sift_stream_id = self.sift_stream_id.to_string(),
                stream_duration = format!("{:.1}s", checkpoint_stats.elapsed_secs),
                messages_processed = checkpoint_stats.messages_sent,
                message_rate = format!("{} messages/s", checkpoint_stats.message_rate),
                bytes_processed = format!("{bytes_processed_pretty}"),
                byte_rate = format!("{byte_rate_pretty}/s"),
            );
        }

        let mut remaining_data_in_buffer_count = 0;
        while let Ok(req) = self.data_rx.try_recv() {
            if let StreamMessage::Request(request) = req {
                if self.drain_tx.send(request).is_err() {
                    break;
                }
                remaining_data_in_buffer_count += 1;
            };
        }

        #[cfg(feature = "tracing")]
        if remaining_data_in_buffer_count > 0 {
            tracing::debug!(
                sift_stream_id = self.sift_stream_id.to_string(),
                num_messages = remaining_data_in_buffer_count,
                "current stream concluded - transferred remaining data to new buffer to use in next stream",
            )
        }
        self.heartbeat_task.abort()
    }
}
