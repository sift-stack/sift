use super::super::{
    channel::ChannelValue, time::TimeValue, RetryPolicy, SiftStream, SiftStreamMode,
};
use crate::backup::BackupsManager;
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
    ops::Drop,
    pin::Pin,
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
    },
    task::JoinHandle,
};

const DATA_BUFFER_LEN: usize = 10_000;

/// Dependencies specifically for ingestion-config based streaming.
pub struct IngestionConfigMode {
    ingestion_config: IngestionConfig,
    flows: Vec<FlowConfig>,
    run: Option<Run>,
    checkpoint_interval: Duration,
    streaming_task: Option<JoinHandle<Result<IngestWithConfigDataStreamResponse>>>,

    retry_policy: Option<RetryPolicy>,

    /// Channel to transmit data to the streaming task.
    data_tx: BoundedSender<IngestWithConfigDataStreamRequest>,

    backups_manager: Option<BackupsManager<IngestWithConfigDataStreamRequest>>,
}

impl SiftStreamMode for IngestionConfigMode {}

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

impl Flow {
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
        backups_manager: Option<BackupsManager<IngestWithConfigDataStreamRequest>>,
    ) -> Self {
        let (data_tx, data_rx) =
            bounded_channel::<IngestWithConfigDataStreamRequest>(DATA_BUFFER_LEN);
        let (checkpoint_signal_tx, checkpoint_signal_rx) = oneshot::channel::<()>();

        let data_stream = DataStream::new(data_rx, checkpoint_signal_rx);

        let streaming_task = Self::init_streaming_task(
            grpc_channel.clone(),
            data_stream,
            checkpoint_interval,
            checkpoint_signal_tx,
        );

        #[cfg(feature = "tracing")]
        tracing::info!("Sift streaming successfully initialized");

        Self {
            grpc_channel,
            mode: IngestionConfigMode {
                ingestion_config,
                flows,
                run,
                streaming_task: Some(streaming_task),
                checkpoint_interval,
                data_tx,
                retry_policy,
                backups_manager,
            },
        }
    }

    pub async fn send(&mut self, message: Flow) -> Result<()> {
        let Some(req) = Self::message_to_ingest_req(
            &message.flow_name,
            &self.mode.ingestion_config.ingestion_config_id,
            self.mode.run.clone().map(|r| r.run_id).unwrap_or_default(),
            message.timestamp.clone(),
            message.values.clone(),
            &self.mode.flows,
        ) else {
            return Err(Error::new_msg(
                ErrorKind::StreamError,
                "failed to turn provided flow into a valid ingestion request",
            ));
        };

        match self.mode.data_tx.send(req.clone()).await {
            Ok(_) => {
                if let Some(backups_manager) = self.mode.backups_manager.as_mut() {
                    backups_manager.send(req)?;
                }
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
                        tracing::warn!("received an error from Sift while streaming");

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

        Err(Error::new(ErrorKind::StreamError, err))
            .context("exhausted all retry attempts")
            .help("please contact Sift")
    }

    async fn restart_stream(&mut self, reingest_from_last_checkpoint: bool) -> Result<()> {
        let (data_tx, data_rx) =
            bounded_channel::<IngestWithConfigDataStreamRequest>(DATA_BUFFER_LEN);

        let (checkpoint_signal_tx, checkpoint_signal_rx) = oneshot::channel::<()>();

        let data_stream = DataStream::new(data_rx, checkpoint_signal_rx);
        self.mode.data_tx = data_tx.clone();

        let streaming_task = Self::init_streaming_task(
            self.grpc_channel.clone(),
            data_stream,
            self.mode.checkpoint_interval,
            checkpoint_signal_tx,
        );
        self.mode.streaming_task = Some(streaming_task);

        if let Some(backup_manager) = self.mode.backups_manager.as_mut() {
            if reingest_from_last_checkpoint {
                #[cfg(feature = "tracing")]
                tracing::info!("checking backups");

                let backup_data = backup_manager
                    .get_backup_data()
                    .await
                    .context("failed to get backup data")?;

                if backup_data.is_empty() {
                    #[cfg(feature = "tracing")]
                    tracing::info!("backups empty - resuming");
                } else {
                    #[cfg(feature = "tracing")]
                    tracing::info!(
                        recovered_data_points = backup_data.len(),
                        "backups detected - reingesting data from last checkpoint"
                    );

                    for data in backup_data {
                        data_tx
                            .send(data)
                            .await
                            .map_err(|_| {
                                Error::new_msg(
                                    ErrorKind::StreamError,
                                    "something went wrong while re-ingesting backups",
                                )
                            })
                            .help("please contact Sift")?;
                    }

                    #[cfg(feature = "tracing")]
                    tracing::info!("successfully reingested data since last checkpoint");
                }
            }

            backup_manager
                .checkpoint()
                .await
                .context("failed to notify backup manager of checkpoint")?;
        }

        Ok(())
    }

    /// This will conclude the stream and return when Sift has sent its final response.
    pub async fn finish(mut self) -> Result<()> {
        if let Some(backup_manager) = self.mode.backups_manager {
            #[cfg(feature = "tracing")]
            tracing::info!("shutting down backups manager");

            let _ = backup_manager.finish().await;
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
                checkpoint_timer.tick().await;

                #[cfg(feature = "tracing")]
                tracing::info!("initiating checkpoint");

                let _ = checkpoint_signal_tx.send(());
            });

            data_stream.started_at = Instant::now();
            let response = client
                .ingest_with_config_data_stream(data_stream)
                .await
                .map(|res| res.into_inner())
                .map_err(|e| Error::new(ErrorKind::StreamError, e))
                .context("failed to receive checkpoint acknowledgement")
                .help("please contact Sift");

            #[cfg(feature = "tracing")]
            tracing::debug!("received response from Sift");

            checkpoint_task.abort_handle().abort();
            let _ = checkpoint_task.await;
            response
        })
    }

    fn message_to_ingest_req(
        flow_name: &str,
        ingestion_config_id: &str,
        run_id: String,
        time_value: TimeValue,
        values: Vec<ChannelValue>,
        flows: &[FlowConfig],
    ) -> Option<IngestWithConfigDataStreamRequest> {
        let mut maybe_channel_values = None;

        for flow in flows.iter().filter(|f| f.name == flow_name) {
            let mut ordered_values = Vec::with_capacity(values.len());

            for conf in &flow.channels {
                let Some(val) = values
                    .iter()
                    .find(|v| v.name == conf.name && v.pb_data_type() == conf.data_type)
                else {
                    continue;
                };
                ordered_values.push(val);
            }

            if ordered_values.len() == flow.channels.len() {
                maybe_channel_values = Some(ordered_values);
                break;
            }
        }

        let Some(channel_values) = maybe_channel_values.map(|vals| {
            vals.into_iter()
                .map(|v| IngestWithConfigDataChannelValue {
                    r#type: Some(v.pb_value()),
                })
                .collect::<Vec<IngestWithConfigDataChannelValue>>()
        }) else {
            #[cfg(feature = "tracing")]
            tracing::error!(
                values = format!("{values:?}"),
                "encountered channel values for which there is no configured flow"
            );
            return None;
        };

        let request = IngestWithConfigDataStreamRequest {
            flow: flow_name.to_string(),
            ingestion_config_id: ingestion_config_id.to_string(),
            timestamp: Some(time_value.0),
            run_id: run_id.to_string(),
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
            tracing::debug!("termination signal received");

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
