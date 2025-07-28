use super::super::{
    RetryPolicy, SiftStream, SiftStreamMode, channel::ChannelValue, time::TimeValue,
};
use crate::{
    backup::{BackupsManager, DiskBackupsManager, InMemoryBackupsManager},
    stream::backups::ingestion_config::{BackupsTransmitterDisk, BackupsTransmitterInMemory},
};
use futures_core::Stream;
use prost::Message;
use sift_connect::SiftChannel;
use sift_error::prelude::*;
use sift_rs::{
    ingest::v1::{
        IngestWithConfigDataChannelValue, IngestWithConfigDataStreamRequest as IngestRequest,
        IngestWithConfigDataStreamResponse, ingest_service_client::IngestServiceClient,
    },
    ingestion_configs::v2::{FlowConfig, IngestionConfig},
    runs::v2::Run,
    wrappers::ingestion_configs::{IngestionConfigServiceWrapper, new_ingestion_config_service},
};
use std::{
    collections::HashMap,
    ops::Drop,
    pin::Pin,
    sync::Arc,
    task::{Context, Poll},
    time::{Duration, Instant},
};
use tokio::{
    sync::{
        Notify,
        mpsc::{
            Receiver as BoundedReceiver, Sender as BoundedSender, UnboundedReceiver,
            UnboundedSender, channel as bounded_channel, error::SendError, unbounded_channel,
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
    checkpoint_interval: Duration,
    streaming_task: Option<DataStreamTask>,
    retry_policy: Option<RetryPolicy>,
    data_tx: Option<BoundedSender<StreamMessage>>,
    /// Used to manually force a checkpoint.
    initiate_checkpoint_tx: Option<Sender<()>>,
    backup_manager: Option<IngestionConfigModeBackupsManager>,
    sift_stream_id: Uuid,

    /// It's possible that [DataStream] may still have some data in its buffer by the time it gets
    /// dropped due to checkpointing or errors while streaming. This is going to be the receiving
    /// end of the drain that recovers that data and puts it into the new receiver associated with
    /// the new `data_tx` when we reinitialize the gRPC stream. When [SiftStream::finish] is
    /// called, any data still in the drain will be sent as a batch to Sift in a final stream.
    drain_rx: Option<UnboundedReceiver<IngestRequest>>,
}

/// The flavor of backups manager. Users shouldn't have to interact with this directly.
pub enum IngestionConfigModeBackupsManager {
    Disk(DiskBackupsManager<IngestRequest, BackupsTransmitterDisk>),
    InMemory(InMemoryBackupsManager<IngestRequest, BackupsTransmitterInMemory>),
}

/// Used for fine-grain control of [DataStream].
enum StreamMessage {
    Request(IngestRequest),
    CheckpointSignal,
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
    drain_tx: UnboundedSender<IngestRequest>,
    sift_stream_id: Uuid,
    heartbeat_task: JoinHandle<()>,
    messages_processed: usize,
    bytes_processed: usize,
    started_at: Instant,
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
    pub fn new(
        grpc_channel: SiftChannel,
        ingestion_config: IngestionConfig,
        flows: Vec<FlowConfig>,
        run: Option<Run>,
        checkpoint_interval: Duration,
        retry_policy: Option<RetryPolicy>,
        backup_manager: Option<IngestionConfigModeBackupsManager>,
    ) -> Self {
        let mut flows_by_name = HashMap::<String, Vec<FlowConfig>>::new();

        for flow in flows {
            flows_by_name
                .entry(flow.name.clone())
                .and_modify(|group| group.push(flow.clone()))
                .or_insert_with(|| vec![flow]);
        }

        let (data_tx, data_rx) = bounded_channel::<StreamMessage>(DATA_BUFFER_CAPACITY);
        let (drain_tx, drain_rx) = unbounded_channel::<IngestRequest>();
        let (initiate_checkpoint_tx, initiate_checkpoint_rx) = oneshot::channel::<()>();
        let begin_checkpoint_notifier = Arc::new(Notify::new());
        let sift_stream_id = Uuid::new_v4();

        let data_stream = DataStream::new(sift_stream_id, data_rx, drain_tx);

        let streaming_task = Self::init_streaming_task(
            grpc_channel.clone(),
            sift_stream_id,
            data_stream,
            checkpoint_interval,
            data_tx.clone(),
            initiate_checkpoint_rx,
            begin_checkpoint_notifier.clone(),
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
                sift_stream_id,
                run,
                streaming_task: Some(streaming_task),
                checkpoint_interval,
                data_tx: Some(data_tx),
                initiate_checkpoint_tx: Some(initiate_checkpoint_tx),
                drain_rx: Some(drain_rx),
                retry_policy,
                backup_manager,
            },
        }
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
        I: IntoIterator<Item = IngestRequest>,
    {
        for req in requests {
            self.send_impl(req).await?;
        }
        Ok(())
    }

    /// Concerned with sending the actual ingest request to [DataStream] which will then write it
    /// to the gRPC stream. If backups are enabled, the request will be backed up as well.
    async fn send_impl(&mut self, req: IngestRequest) -> Result<()> {
        match self.backup_data(&req).await {
            Err(err) if err.kind() == ErrorKind::BackupLimitReached => {
                #[cfg(feature = "tracing")]
                tracing::info!(
                    sift_stream_id = self.mode.sift_stream_id.to_string(),
                    "backup size limit reached - forcing checkpoint"
                );
                if let Some(initiate_checkpoint_tx) = self.mode.initiate_checkpoint_tx.take() {
                    let _ = initiate_checkpoint_tx.send(());
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
                self.restart_stream(false).await;
                self.clear_backups().await;

                if let Err(err) = self.backup_data(&req).await {
                    self.mode.backup_manager = None;

                    #[cfg(feature = "tracing")]
                    tracing::warn!(
                        sift_stream_id = self.mode.sift_stream_id.to_string(),
                        error = format!("{err:?}"),
                        "an unexpected error occurred trying to backup data after clearing previous backups - backups no longer processed, please contact Sift",
                    );
                }
            }
            Err(err) => {
                self.mode.backup_manager = None;

                #[cfg(feature = "tracing")]
                tracing::warn!(
                    sift_stream_id = self.mode.sift_stream_id.to_string(),
                    error = format!("{err:?}"),
                    "an unexpected error occurred while trying to backup data - backups no longer processed, please contact Sift",
                );
            }
            _ => (),
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

            Err(SendError(_)) => match self.mode.streaming_task.take() {
                None => {
                    self.restart_stream(false).await;
                    Box::pin(self.send_impl(req)).await
                }

                Some(streaming_task) => match streaming_task.await {
                    Ok(Ok(_)) => {
                        self.restart_stream(false).await;
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
                },
            },
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

        for flow_config in flow_configs {
            self.mode
                .flows_by_name
                .entry(flow_config.name.clone())
                .and_modify(|flows| flows.push(flow_config.clone()))
                .or_insert_with(|| vec![flow_config.clone()]);

            tracing::info!(
                sift_stream_id = self.mode.sift_stream_id.to_string(),
                flow = flow_config.name,
                "successfully registered new flow"
            );
        }
        Ok(())
    }

    /// Send a single data point to the backups manager to be backed up.
    async fn backup_data(&mut self, req: &IngestRequest) -> Result<()> {
        let Some(manager) = self.mode.backup_manager.as_mut() else {
            return Ok(());
        };
        match manager {
            IngestionConfigModeBackupsManager::Disk(manager) => manager.send(req.clone()).await,
            IngestionConfigModeBackupsManager::InMemory(manager) => manager.send(req.clone()).await,
        }
    }

    async fn shutdown_backups_worker(manager: IngestionConfigModeBackupsManager) -> Result<()> {
        match manager {
            IngestionConfigModeBackupsManager::Disk(manager) => manager.finish().await,
            IngestionConfigModeBackupsManager::InMemory(manager) => manager.finish().await,
        }
    }

    async fn retry(&mut self, ingest_req: IngestRequest, err: Error) -> Result<()> {
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

                    self.restart_stream(true).await;

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

    async fn restart_stream(&mut self, reingest_from_last_checkpoint: bool) {
        let (data_tx, data_rx) = bounded_channel::<StreamMessage>(DATA_BUFFER_CAPACITY);
        let (drain_tx, drain_rx) = unbounded_channel::<IngestRequest>();
        let (initiate_checkpoint_tx, initiate_checkpoint_rx) = oneshot::channel::<()>();
        let begin_checkpoint_notifier = Arc::new(Notify::new());
        let data_stream = DataStream::new(self.mode.sift_stream_id, data_rx, drain_tx);

        self.mode.data_tx = Some(data_tx.clone());
        self.mode.initiate_checkpoint_tx = Some(initiate_checkpoint_tx);

        let streaming_task = Self::init_streaming_task(
            self.grpc_channel.clone(),
            self.mode.sift_stream_id,
            data_stream,
            self.mode.checkpoint_interval,
            data_tx.clone(),
            initiate_checkpoint_rx,
            begin_checkpoint_notifier.clone(),
        );
        self.mode.streaming_task = Some(streaming_task);

        if let Some(mut drain) = self.mode.drain_rx.replace(drain_rx) {
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
            if let Err(process_backups_err) = self.process_backups().await {
                #[cfg(feature = "tracing")]
                tracing::warn!(
                    sift_stream_id = self.mode.sift_stream_id.to_string(),
                    error = format!("{process_backups_err:?}"),
                    "an unexpected error occurred while processing backups - backups will no longer be processed, please notify Sift"
                );
                self.mode.backup_manager = None;
            }
        }
        begin_checkpoint_notifier.notify_one();

        #[cfg(feature = "tracing")]
        tracing::info!(
            sift_stream_id = self.mode.sift_stream_id.to_string(),
            "successfully initialized a new stream to Sift"
        );
    }

    async fn process_backups(&self) -> Result<()> {
        let Some(manager) = self.mode.backup_manager.as_ref() else {
            return Ok(());
        };

        #[cfg(feature = "tracing")]
        tracing::info!(
            sift_stream_id = self.mode.sift_stream_id.to_string(),
            "processing backups"
        );

        match manager {
            IngestionConfigModeBackupsManager::Disk(manager) => manager.transmit_backups().await,
            IngestionConfigModeBackupsManager::InMemory(manager) => {
                manager.transmit_backups().await
            }
        }
    }

    async fn clear_backups(&mut self) {
        let Some(manager) = self.mode.backup_manager.as_mut() else {
            return;
        };
        let clear_res = match manager {
            IngestionConfigModeBackupsManager::Disk(manager) => manager.clear().await,
            IngestionConfigModeBackupsManager::InMemory(manager) => manager.clear().await,
        };
        if let Err(err) = clear_res {
            self.mode.backup_manager = None;

            #[cfg(feature = "tracing")]
            tracing::warn!(
                sift_stream_id = self.mode.sift_stream_id.to_string(),
                error = format!("{err:?}"),
                "unexpected error occurred while clearing backups - no longer processing backups, please notify Sift"
            );
        }
    }

    /// This will conclude the stream and return when Sift has sent its final response. It is
    /// important that this method be called in order to obtain the final checkpoint
    /// acknowledgement from Sift, otherwise some tail-end data may fail to send.
    pub async fn finish(mut self) -> Result<()> {
        if let Some(manager) = self.mode.backup_manager {
            #[cfg(feature = "tracing")]
            tracing::info!(
                sift_stream_id = self.mode.sift_stream_id.to_string(),
                "shutting down backups manager"
            );
            let _ = Self::shutdown_backups_worker(manager).await;
        }

        if let Some(streaming_task) = self.mode.streaming_task.take() {
            if let Some(initiate_checkpoint_tx) = self.mode.initiate_checkpoint_tx.take() {
                let _ = initiate_checkpoint_tx.send(());
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

        if let Some(mut drain) = self.mode.drain_rx.take() {
            let mut remaining_requests = vec![];

            while let Ok(req) = drain.try_recv() {
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

    fn init_streaming_task(
        grpc_channel: SiftChannel,
        sift_stream_id: Uuid,
        mut data_stream: DataStream,
        checkpoint_interval: Duration,
        data_tx: BoundedSender<StreamMessage>,
        initiate_checkpoint_rx: Receiver<()>,
        begin_checkpoint_notifier: Arc<Notify>,
    ) -> DataStreamTask {
        tokio::spawn(async move {
            let mut client = IngestServiceClient::new(grpc_channel);
            let force_checkpoint = Arc::new(Notify::new());
            let force_checkpoint_c = force_checkpoint.clone();

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
                        #[cfg(feature = "tracing")]
                        tracing::info!(sift_stream_id = sift_stream_id.to_string(), "checkpoint timer elapsed - initiating checkpoint");
                    }
                    _ = initiate_checkpoint_rx => {
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
                        Err(err) => Err(err),
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
    ) -> Option<IngestRequest> {
        let mut maybe_channel_values = None;

        for flow in flows {
            let mut ordered_values = flow
                .channels
                .iter()
                .map(|_| None)
                .collect::<Vec<Option<ChannelValue>>>();
            let mut num_channels_accounted_for = 0;

            'outer: for v in &message.values {
                for (i, conf) in flow.channels.iter().enumerate() {
                    if v.name == conf.name && v.pb_data_type() == conf.data_type {
                        num_channels_accounted_for += 1;
                        ordered_values[i] = Some(v.clone());
                        continue 'outer;
                    }
                }
            }

            // All channel values accounted for in this flow
            if num_channels_accounted_for == message.values.len() {
                maybe_channel_values = Some(ordered_values);
                break;
            }
        }

        let channel_values = maybe_channel_values.map(|vals| {
            vals.into_iter()
                .map(|v| IngestWithConfigDataChannelValue {
                    r#type: Some(v.map_or_else(ChannelValue::empty_pb, |val| val.pb_value())),
                })
                .collect::<Vec<IngestWithConfigDataChannelValue>>()
        })?;

        let request = IngestRequest {
            flow: message.flow_name.to_string(),
            ingestion_config_id: ingestion_config_id.to_string(),
            timestamp: Some(message.timestamp.0),
            run_id: run_id.unwrap_or_default(),
            channel_values,
            ..Default::default()
        };

        Some(request)
    }

    /// Creates an [IngestRequest] directly without consulting the flow cache.
    pub(crate) fn message_to_ingest_req_direct(
        message: &Flow,
        ingestion_config_id: &str,
        run_id: Option<String>,
    ) -> IngestRequest {
        let channel_values = message
            .values
            .iter()
            .map(|val| IngestWithConfigDataChannelValue {
                r#type: Some(val.pb_value()),
            })
            .collect::<Vec<_>>();

        IngestRequest {
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
        drain_tx: UnboundedSender<IngestRequest>,
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
            messages_processed: 0,
            bytes_processed: 0,
            started_at: Instant::now(),
        }
    }
}

impl Stream for DataStream {
    type Item = IngestRequest;

    fn poll_next(mut self: Pin<&mut Self>, ctx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        match self.data_rx.poll_recv(ctx) {
            Poll::Ready(Some(msg)) => match msg {
                StreamMessage::Request(req) => {
                    self.messages_processed += 1;
                    self.bytes_processed += req.encode_length_delimited_to_vec().len();
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
            let elapsed = self.started_at.elapsed();
            let elapsed_secs = elapsed.as_secs();
            let elapsed_secs_f64 = elapsed_secs as f64;
            let message_rate = (self.messages_processed as f64) / elapsed_secs_f64;
            let bytes_processed_pretty = bytesize::ByteSize::b(self.bytes_processed as u64)
                .display()
                .iec();
            let byte_rate = ((self.bytes_processed as f64) / elapsed_secs_f64).ceil() as u64;
            let byte_rate_pretty = bytesize::ByteSize::b(byte_rate).display().iec();

            tracing::info!(
                sift_stream_id = self.sift_stream_id.to_string(),
                stream_duration = format!("{elapsed_secs}s"),
                messages_processed = self.messages_processed,
                message_rate = format!("{message_rate} messages/s"),
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
