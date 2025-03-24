use super::super::{
    channel::ChannelValue, time::TimeValue, RetryPolicy, SiftStream, SiftStreamMode,
};
use crate::backup::{BackupsManager, DiskBackupsManager, InMemoryBackupsManager};
use futures_core::Stream;
use prost::Message;
use sift_connect::SiftChannel;
use sift_error::prelude::*;
use sift_rs::{
    ingest::v1::{
        ingest_service_client::IngestServiceClient, IngestWithConfigDataChannelValue,
        IngestWithConfigDataStreamRequest, IngestWithConfigDataStreamResponse,
    },
    ingestion_configs::v2::{FlowConfig, IngestionConfig},
    runs::v2::Run,
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
        mpsc::{
            channel as bounded_channel, error::SendError, Receiver as BoundedReceiver,
            Sender as BoundedSender,
        },
        oneshot::{self, Receiver, Sender},
        Notify,
    },
    task::JoinHandle,
};

/// The size of the channel buffer that connects [SiftStream::send] with the task that actually
/// streams data to Sift.
const DATA_BUFFER_LEN: usize = 10_000;

/// Dependencies specifically for ingestion-config based streaming. Users shouldn't have to
/// interact with this directly.
pub struct IngestionConfigMode {
    ingestion_config: IngestionConfig,
    flows_by_name: HashMap<String, Vec<FlowConfig>>,
    run: Option<Run>,
    checkpoint_interval: Duration,
    streaming_task: Option<JoinHandle<Result<IngestWithConfigDataStreamResponse>>>,
    retry_policy: Option<RetryPolicy>,
    data_tx: BoundedSender<IngestWithConfigDataStreamRequest>,
    backups_manager: Option<IngestionConfigModeBackupsManager>,
}

/// The flavor of backups manager. Users shouldn't have to interact with this directly.
pub enum IngestionConfigModeBackupsManager {
    Disk(DiskBackupsManager<IngestWithConfigDataStreamRequest>),
    InMemory(InMemoryBackupsManager<IngestWithConfigDataStreamRequest>),
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
    data_rx: BoundedReceiver<IngestWithConfigDataStreamRequest>,
    checkpoint_signal_rx: Receiver<()>,
    messages_processed: usize,
    bytes_processed: usize,
    started_at: Instant,
}

impl Default for IngestionConfigModeBackupsManager {
    /// The default backups manager flavor if backups are enabled.
    fn default() -> Self {
        Self::InMemory(InMemoryBackupsManager::new(None))
    }
}

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
        backups_manager: Option<IngestionConfigModeBackupsManager>,
    ) -> Self {
        let mut flows_by_name = HashMap::<String, Vec<FlowConfig>>::new();

        for flow in flows {
            flows_by_name
                .entry(flow.name.clone())
                .and_modify(|group| group.push(flow.clone()))
                .or_insert_with(|| vec![flow]);
        }

        let (data_tx, data_rx) =
            bounded_channel::<IngestWithConfigDataStreamRequest>(DATA_BUFFER_LEN);
        let (checkpoint_signal_tx, checkpoint_signal_rx) = oneshot::channel::<()>();
        let begin_checkpoint_notifier = Arc::new(Notify::new());

        let data_stream = DataStream::new(data_rx, checkpoint_signal_rx);

        let streaming_task = Self::init_streaming_task(
            grpc_channel.clone(),
            data_stream,
            checkpoint_interval,
            checkpoint_signal_tx,
            begin_checkpoint_notifier.clone(),
        );

        // Begin checkpoint immediately upon starting
        begin_checkpoint_notifier.notify_one();

        #[cfg(feature = "tracing")]
        tracing::info!("Sift streaming successfully initialized");

        Self {
            grpc_channel,
            mode: IngestionConfigMode {
                ingestion_config,
                flows_by_name,
                run,
                streaming_task: Some(streaming_task),
                checkpoint_interval,
                data_tx,
                retry_policy,
                backups_manager,
            },
        }
    }

    /// The entry-point to send actual telemetry to Sift in the form of [Flow]s. This method will
    /// return an error of kind [ErrorKind::UnknownFlow] if the `message` provided has a flow-name
    /// that doesn't match any of the flow-configs specified in the ingestion config.
    ///
    /// Additionally, if retries are enabled, then this method will use the configured retry policy
    /// to attempt to resend data and reconnect until the max-attempts are exhausted, afterwhich a
    /// general [ErrorKind::StreamError] is returned.
    ///
    /// Lastly, if backups are enabled, then any `message` passed to this method will be backed up
    /// and reingested in the case of an error once a retry is successful.
    pub async fn send(&mut self, message: Flow) -> Result<()> {
        let Some(flows) = self.mode.flows_by_name.get(&message.flow_name) else {
            return Err(Error::new_msg(ErrorKind::UnknownFlow, "unknown flow name"))
                .with_context(|| format!("unknown flow provided: {message:?}"))
                .help("try adding this flow to your ingestion config");
        };

        let Some(req) = Self::message_to_ingest_req(
            &message,
            &self.mode.ingestion_config.ingestion_config_id,
            self.mode.run.as_ref().map(|r| r.run_id.clone()),
            flows,
        ) else {
            return Err(Error::new_msg(
                ErrorKind::StreamError,
                "failed to turn provided flow into a valid ingestion request",
            ));
        };

        match self.mode.data_tx.send(req.clone()).await {
            Ok(_) => {
                self.backup_data(req).await?;
                Ok(())
            }

            Err(SendError(_)) => match self.mode.streaming_task.take() {
                None => {
                    self.restart_stream(false).await?;
                    Box::pin(self.send(message)).await
                }

                Some(streaming_task) => match streaming_task.await {
                    Ok(Ok(_)) => {
                        #[cfg(feature = "tracing")]
                        tracing::info!(
                            "checkpoint acknowledgement received from Sift - resuming stream"
                        );

                        self.restart_stream(false).await?;
                        Box::pin(self.send(message)).await
                    }
                    Ok(Err(err)) => {
                        #[cfg(feature = "tracing")]
                        tracing::warn!("encountered an error while streaming to Sift");

                        self.retry(req, err).await
                    }
                    Err(err) => {
                        #[cfg(feature = "tracing")]
                        tracing::warn!("something went wrong while waiting for response from Sift");

                        self.retry(req, Error::new(ErrorKind::StreamError, err))
                            .await
                    }
                },
            },
        }
    }

    async fn backup_data(&mut self, req: IngestWithConfigDataStreamRequest) -> Result<()> {
        if let Some(backups_manager) = self.mode.backups_manager.as_mut() {
            match backups_manager {
                IngestionConfigModeBackupsManager::Disk(manager) => {
                    return manager.send(req).await;
                }
                IngestionConfigModeBackupsManager::InMemory(manager) => {
                    return manager.send(req).await;
                }
            }
        }
        Ok(())
    }

    async fn truncate_backup(&mut self) -> Result<()> {
        if let Some(backups_manager) = self.mode.backups_manager.as_mut() {
            match backups_manager {
                IngestionConfigModeBackupsManager::Disk(manager) => {
                    return manager.truncate_backup().await;
                }
                IngestionConfigModeBackupsManager::InMemory(manager) => {
                    return manager.truncate_backup().await;
                }
            }
        }
        Ok(())
    }

    async fn shutdown_backups_manager(
        backups_manager: IngestionConfigModeBackupsManager,
    ) -> Result<()> {
        match backups_manager {
            IngestionConfigModeBackupsManager::Disk(manager) => manager.finish().await,
            IngestionConfigModeBackupsManager::InMemory(manager) => manager.finish().await,
        }
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
            "stream failed - attempting retry with retry policy: {:?}",
            retry_policy
        );

        let mut current_wait = retry_policy.initial_backoff;

        for i in 1..=retry_policy.max_attempts {
            #[cfg(feature = "tracing")]
            tracing::info!(retry_counter = i, "attempting retry");

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
                        retry_counter = i,
                        "successful retry - re-establishing connection to Sift"
                    );

                    self.restart_stream(true).await?;

                    return Ok(());
                }
                Err(err) => {
                    #[cfg(feature = "tracing")]
                    if i < retry_policy.max_attempts {
                        tracing::warn!(
                            retry_counter = i,
                            "retry attempt failed - backing off for {}ms",
                            current_wait.as_millis()
                        );
                    } else {
                        tracing::warn!(
                            retry_counter = i,
                            "all retry attempts exhausted due to: {err}"
                        );
                    }
                    continue;
                }
            }
        }
        Err(err)
    }

    async fn restart_stream(&mut self, reingest_from_last_checkpoint: bool) -> Result<()> {
        let (data_tx, data_rx) =
            bounded_channel::<IngestWithConfigDataStreamRequest>(DATA_BUFFER_LEN);

        let (checkpoint_signal_tx, checkpoint_signal_rx) = oneshot::channel::<()>();
        let begin_checkpoint_notifier = Arc::new(Notify::new());

        let data_stream = DataStream::new(data_rx, checkpoint_signal_rx);

        self.mode.data_tx = data_tx.clone();

        let streaming_task = Self::init_streaming_task(
            self.grpc_channel.clone(),
            data_stream,
            self.mode.checkpoint_interval,
            checkpoint_signal_tx,
            begin_checkpoint_notifier.clone(),
        );
        self.mode.streaming_task = Some(streaming_task);

        if reingest_from_last_checkpoint {
            // If we entered this branch this some really unxpected things happened. We're going to
            // give up on trying to rescue the backups here and just resume streaming until the
            // next checkpoint.
            if let Err(err) = self.process_backups(data_tx).await {
                #[cfg(feature = "tracing")]
                {
                    tracing::debug!(
                        error = format!("{err:?}"),
                        "not all backups were successfully processed"
                    );
                    tracing::warn!("not all backups were successfully processed due to unexpected stream termination - retrying");
                }

                return Box::pin(self.restart_stream(false)).await;
            }
            begin_checkpoint_notifier.notify_one();
        } else {
            self.truncate_backup().await?;
            begin_checkpoint_notifier.notify_one();
        }

        #[cfg(feature = "tracing")]
        tracing::info!("successfully initialized a new stream to Sift");

        Ok(())
    }

    async fn process_backups(
        &mut self,
        data_tx: BoundedSender<IngestWithConfigDataStreamRequest>,
    ) -> Result<()> {
        let Some(backup_manager) = self.mode.backups_manager.as_mut() else {
            return Ok(());
        };

        #[cfg(feature = "tracing")]
        tracing::info!("processing backups");

        let mut data_points = 0;
        let mut start = Instant::now();

        macro_rules! process_backups {
            ($m:ident) => {
                for data in $m.get_backup_data().await? {
                    data_tx
                        .send(data)
                        .await
                        .map_err(|_| {
                            Error::new_msg(ErrorKind::StreamError, "receiver prematurely closed")
                        })
                        .context("something went wrong while reingesting backups")
                        .help("please contact Sift")?;

                    data_points += 1;

                    if start.elapsed() >= Duration::from_secs(10) {
                        #[cfg(feature = "tracing")]
                        tracing::info!(points_processed = data_points, "processing backups");

                        start = Instant::now();
                    }
                }
                $m.truncate_backup().await?;
            };
        }

        match backup_manager {
            IngestionConfigModeBackupsManager::Disk(manager) => {
                process_backups!(manager);
            }
            IngestionConfigModeBackupsManager::InMemory(manager) => {
                process_backups!(manager);
            }
        }

        if data_points == 0 {
            #[cfg(feature = "tracing")]
            tracing::info!("no backups to reingest");
        } else {
            #[cfg(feature = "tracing")]
            tracing::info!(
                data_points_recovered = data_points,
                "successfully reingested data since last checkpoint"
            );
        }
        Ok(())
    }

    /// This will conclude the stream and return when Sift has sent its final response. It is
    /// important that this method be called in order to obtain the final checkpoint
    /// acknowledgement from Sift, otherwise some tail-end data may fail to send.
    pub async fn finish(mut self) -> Result<()> {
        if let Some(backup_manager) = self.mode.backups_manager {
            #[cfg(feature = "tracing")]
            tracing::info!("shutting down backups manager");

            let _ = Self::shutdown_backups_manager(backup_manager).await;
        }

        #[cfg(feature = "tracing")]
        tracing::info!("initiating final checkpoint");

        if let Some(streaming_task) = self.mode.streaming_task.take() {
            // Terminate client stream
            drop(self.mode.data_tx);

            streaming_task
                .await
                .map_err(|e| Error::new(ErrorKind::StreamError, e))
                .context("something went wrong while waiting for the final checkpoint")
                .help("please context Sift")?
                .context("final checkpoint failure")
                .help("the final checkpoint may or may not have succeeded. Please contact Sift")?;
        }

        #[cfg(feature = "tracing")]
        tracing::info!(
            asset_id = self.mode.ingestion_config.asset_id,
            ingestion_config_id = self.mode.ingestion_config.ingestion_config_id,
            run = self.mode.run.map(|r| r.name).unwrap_or_default(),
            "successfully received final checkpoint acknowledgement - concluding stream"
        );

        Ok(())
    }

    fn init_streaming_task(
        grpc_channel: SiftChannel,
        mut data_stream: DataStream,
        checkpoint_interval: Duration,
        checkpoint_signal_tx: Sender<()>,
        begin_checkpoint_notifier: Arc<Notify>,
    ) -> JoinHandle<Result<IngestWithConfigDataStreamResponse>> {
        tokio::spawn(async move {
            let mut client = IngestServiceClient::new(grpc_channel);

            let checkpoint_task = tokio::spawn(async move {
                let mut checkpoint_timer = {
                    let mut timer = tokio::time::interval(checkpoint_interval);
                    // Time goes off immediately
                    timer.tick().await;
                    timer
                };
                begin_checkpoint_notifier.notified().await;

                #[cfg(feature = "tracing")]
                tracing::debug!("received notification to start checkpoint timer");

                checkpoint_timer.tick().await;

                #[cfg(feature = "tracing")]
                tracing::info!("initiating checkpoint");

                let _ = checkpoint_signal_tx.send(());
            });

            data_stream.started_at = Instant::now();
            let raw_response = client
                .ingest_with_config_data_stream(data_stream)
                .await
                .map(|res| res.into_inner());

            #[cfg(feature = "tracing")]
            tracing::debug!(
                res = format!("{raw_response:?}"),
                "received response from Sift"
            );

            let response = raw_response
                .map_err(|e| Error::new(ErrorKind::StreamError, e))
                .context("failed to receive checkpoint acknowledgement")
                .help("please contact Sift");

            checkpoint_task.abort_handle().abort();
            let _ = checkpoint_task.await;
            response
        })
    }

    /// Flows passed into this function should have names match `flow_name`.
    pub(crate) fn message_to_ingest_req(
        message: &Flow,
        ingestion_config_id: &str,
        run_id: Option<String>,
        flows: &[FlowConfig],
    ) -> Option<IngestWithConfigDataStreamRequest> {
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

        let Some(channel_values) = maybe_channel_values.map(|vals| {
            vals.into_iter()
                .map(|v| IngestWithConfigDataChannelValue {
                    r#type: Some(v.map_or_else(ChannelValue::empty_pb, |val| val.pb_value())),
                })
                .collect::<Vec<IngestWithConfigDataChannelValue>>()
        }) else {
            #[cfg(feature = "tracing")]
            tracing::warn!(
                values = format!("{message:?}"),
                "encountered a message whose channel values do not match any configured flows"
            );
            return None;
        };

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
}

impl DataStream {
    fn new(
        data_rx: BoundedReceiver<IngestWithConfigDataStreamRequest>,
        checkpoint_signal_rx: Receiver<()>,
    ) -> Self {
        Self {
            data_rx,
            checkpoint_signal_rx,
            messages_processed: 0,
            bytes_processed: 0,
            started_at: Instant::now(),
        }
    }
}

impl Stream for DataStream {
    type Item = IngestWithConfigDataStreamRequest;

    fn poll_next(mut self: Pin<&mut Self>, ctx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        if self.checkpoint_signal_rx.try_recv().is_ok() {
            #[cfg(feature = "tracing")]
            tracing::debug!("stream checkpoint signal received");

            return Poll::Ready(None);
        }

        match self.data_rx.poll_recv(ctx) {
            Poll::Ready(Some(req)) => {
                self.messages_processed += 1;
                self.bytes_processed += req.encode_length_delimited_to_vec().len();
                Poll::Ready(Some(req))
            }
            Poll::Ready(None) => {
                // All senders dropped.. conclude stream
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
                stream_duration = format!("{elapsed_secs}s"),
                messages_processed = self.messages_processed,
                message_rate = format!("{message_rate} messages/s"),
                bytes_processed = format!("{bytes_processed_pretty}"),
                byte_rate = format!("{byte_rate_pretty}/s"),
            );
        }
    }
}
