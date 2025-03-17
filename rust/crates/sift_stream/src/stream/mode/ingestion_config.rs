use super::super::{
    channel::ChannelValue, time::TimeValue, RetryPolicy, SiftStream, SiftStreamMode,
};
use futures_core::Stream;
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
    sync::mpsc::{sync_channel, Receiver as SyncReceiver, SyncSender},
    task::{Context, Poll},
    time::Duration,
};
use tokio::{
    sync::{
        mpsc::{error::SendError, unbounded_channel, UnboundedReceiver, UnboundedSender},
        oneshot::{self, Receiver, Sender},
    },
    task::JoinHandle,
};

/// Dependencies specifically for ingestion-config based streaming.
pub struct IngestionConfigMode {
    ingestion_config: IngestionConfig,
    flows: Vec<FlowConfig>,
    run: Option<Run>,
    checkpoint_interval: Duration,
    checkpoint_timeout: Duration,
    streaming_task: JoinHandle<()>,

    retry_policy: Option<RetryPolicy>,
    failed_to_send_message: Option<Message>,
    is_retrying: bool,

    /// Channel to transmit data to the streaming task.
    data_tx: UnboundedSender<Message>,
    /// Transmit signal to streaming task to initiate final checkpoint and terminate streaming.
    termination_signal_tx: Sender<()>,
    /// Channel to receive response from Sift from the streaming task once it terminates.
    server_res_rx: SyncReceiver<Result<IngestWithConfigDataStreamResponse>>,

    /// If there's an error from Sift in the middle of the stream, then [DataStream] will send any
    /// remaining data in its receiver buffer to this channel.
    recovery_rx: UnboundedReceiver<Message>,
}

impl SiftStreamMode for IngestionConfigMode {}

#[derive(Debug, Clone)]
pub struct Message(InnerMessage);

#[derive(Debug, Clone)]
enum InnerMessage {
    Flow {
        name: String,
        timestamp: TimeValue,
        values: Vec<ChannelValue>,
    },
    CheckpointSignal,
}

/// Dependencies used in the Tokio task that actually sends the data to Sift.
struct DataStream {
    flows: Vec<FlowConfig>,
    ingestion_config_id: String,
    run_id: Option<String>,
    data_rx: UnboundedReceiver<Message>,
    recovery_tx: UnboundedSender<Message>,
}

impl Message {
    pub fn new<S: AsRef<str>>(flow_name: S, timestamp: TimeValue, values: &[ChannelValue]) -> Self {
        Self(InnerMessage::Flow {
            timestamp,
            name: flow_name.as_ref().to_string(),
            values: values.to_vec(),
        })
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
        checkpoint_timeout: Duration,
        retry_policy: Option<RetryPolicy>,
    ) -> Self {
        let (data_tx, data_rx) = unbounded_channel::<Message>();
        let (termination_signal_tx, termination_signal_rx) = oneshot::channel::<()>();
        let (server_res_tx, server_res_rx) =
            sync_channel::<Result<IngestWithConfigDataStreamResponse>>(1);
        let (recovery_tx, recovery_rx) = unbounded_channel::<Message>();

        let data_stream = DataStream::new(
            &ingestion_config,
            &flows,
            data_rx,
            run.as_ref(),
            recovery_tx,
        );

        let streaming_task = Self::init_streaming_task(
            grpc_channel.clone(),
            data_stream,
            checkpoint_interval,
            data_tx.clone(),
            termination_signal_rx,
            server_res_tx,
        );

        Self {
            grpc_channel,
            mode: IngestionConfigMode {
                ingestion_config,
                flows,
                run,
                streaming_task,
                checkpoint_interval,
                checkpoint_timeout,
                data_tx,
                termination_signal_tx,
                server_res_rx,
                recovery_rx,
                retry_policy,
                failed_to_send_message: None,
                is_retrying: false,
            },
        }
    }

    /// TODO: Transform this to better public-facing docs.
    ///
    /// This is synchronous to reduce context switching overhead, but since it is hooked into the
    /// asynchronous data sending task, getting back an "Ok" does not necessarily imply that the
    /// server accepted the data. We'd have to call `send` again to see if an error was sent back
    /// from the server on the previous call.
    pub fn send(&mut self, message: Message) -> Result<()> {
        match self.mode.data_tx.send(message) {
            Ok(_) => Ok(()),

            Err(SendError(_)) if self.can_retry() => {
                Err(Error::new_msg(ErrorKind::StreamError, "stream is closed due to an error but possible to retry"))
                    .help("trying calling `SiftStream<IngestionConfigMode>::retry`")
            }

            Err(SendError(_)) if self.is_retrying() => {
                Err(Error::new_msg(ErrorKind::StreamError, "cannot call send when in the process of retrying"))
                    .help("trying waiting until `SiftStream<IngestionConfigMode>::retry` has been polled to completion")
            }

            // We will only hit this branch if DataStream has been dropped.
            Err(SendError(msg)) => {
                match self.mode.server_res_rx.recv_timeout(self.mode.checkpoint_timeout) {
                    Ok(Ok(_)) => {
                        #[cfg(feature = "tracing")]
                        tracing::info!("checkpoint acknowledgement received from Sift");

                        self.recover_data_and_reinit_stream();
                        self.send(msg)
                    }
                    Ok(Err(err)) => {
                        self.mode.failed_to_send_message = Some(msg);

                        if self.mode.retry_policy.is_some() {
                            Err(Error::new(ErrorKind::StreamErrorRetriable, err))
                                .context("received an error from Sift while streaming but possible to retry")
                                .help("trying calling `SiftStream<IngestionConfigMode>::retry`")
                        } else {
                            Err(Error::new(ErrorKind::StreamError, err))
                                .context("received an error from Sift while streaming; no retry policy detected")
                                .help("please contact Sift and consider adding a retry policy")
                        }
                    }
                    Err(err) => {
                        self.mode.failed_to_send_message = Some(msg);

                        if self.mode.retry_policy.is_some() {
                            Err(Error::new(ErrorKind::StreamErrorRetriable, err))
                                .context("exceeded checkpoint timeout waiting for a checkpoint acknowledgement but possible to retry")
                                .help("trying calling `SiftStream<IngestionConfigMode>::retry`")
                        } else {
                            Err(Error::new(ErrorKind::StreamError, err))
                                .context("exceeded checkpoint timeout waiting for a checkpoint acknowledgement")
                                .help("please contact Sift and consider adding a retry policy")
                        }
                    }
                }
            }
        }
    }

    pub async fn retry(&mut self) -> Result<()> {
        let Some(retry_policy) = self.mode.retry_policy.as_ref() else {
            return Err(Error::new_msg(ErrorKind::StreamError, "no retry policy detected"))
                .help("`SiftStream<IngestionConfigMode>::retry` can only be called when a retry policy is registered");
        };

        let Some(Message(InnerMessage::Flow {
            name,
            timestamp,
            values,
        })) = self.mode.failed_to_send_message.take()
        else {
            return Err(Error::new_msg(ErrorKind::StreamError, "no need to retry"))
                .help("try calling `SiftStream<IngestionConfigMode>::send` as normal");
        };

        let IngestionConfigMode {
            ingestion_config:
                IngestionConfig {
                    ingestion_config_id,
                    ..
                },
            run,
            flows,
            ..
        } = &self.mode;

        let Some(ingest_req) = message_to_ingest_req(
            &name,
            ingestion_config_id,
            run.as_ref().map(|r| r.run_id.clone()).unwrap_or_default(),
            timestamp,
            values,
            flows,
        ) else {
            return Err(Error::new_msg(ErrorKind::StreamError, "retry failed"))
                .context("tried to resend a malformed message")
                .help("please conteact Sift");
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

            match client
                .ingest_with_config_data_stream(tokio_stream::once(ingest_req.clone()))
                .await
            {
                Ok(_) => {
                    #[cfg(feature = "tracing")]
                    tracing::info!(retry_counter = i, "successfully re-established connection");

                    self.recover_data_and_reinit_stream();
                    return Ok(());
                }
                Err(err) => {
                    #[cfg(feature = "tracing")]
                    if i < retry_policy.max_attempts {
                        tracing::warn!(
                            retry_counter = i,
                            "retry attempt failed - backing off for {} ms",
                            current_wait.as_millis()
                        );
                    } else {
                        tracing::warn!(
                            retry_counter = i,
                            "all retry attempts exhausted due to: {err}"
                        );
                    }

                    tokio::time::sleep(current_wait).await;
                    current_wait = (current_wait * u32::from(retry_policy.backoff_multiplier))
                        .min(retry_policy.max_backoff);
                    continue;
                }
            }
        }

        Err(Error::new_msg(
            ErrorKind::StreamError,
            "exhausted all retry attempts",
        ))
        .help("please contact Sift")
    }

    fn can_retry(&self) -> bool {
        self.mode.retry_policy.is_some() && self.mode.failed_to_send_message.is_some()
    }

    fn is_retrying(&self) -> bool {
        self.mode.is_retrying
    }

    fn recover_data_and_reinit_stream(&mut self) {
        let (data_tx, data_rx) = unbounded_channel::<Message>();

        // Recover messages from previously dropped [DataStream] and buffer into new
        // receiver for new [DataStream].
        while let Ok(message) = self.mode.recovery_rx.try_recv() {
            let _ = data_tx.send(message);
        }

        let (termination_signal_tx, termination_signal_rx) = oneshot::channel::<()>();
        let (server_res_tx, server_res_rx) =
            sync_channel::<Result<IngestWithConfigDataStreamResponse>>(1);
        let (recovery_tx, recovery_rx) = unbounded_channel::<Message>();

        self.mode.recovery_rx = recovery_rx;
        self.mode.termination_signal_tx = termination_signal_tx;
        self.mode.server_res_rx = server_res_rx;

        let data_stream = DataStream::new(
            &self.mode.ingestion_config,
            &self.mode.flows,
            data_rx,
            self.mode.run.as_ref(),
            recovery_tx,
        );
        self.mode.data_tx = data_tx.clone();

        self.mode.streaming_task = Self::init_streaming_task(
            self.grpc_channel.clone(),
            data_stream,
            self.mode.checkpoint_interval,
            data_tx,
            termination_signal_rx,
            server_res_tx,
        );
    }

    /// This will conclude the stream and return when Sift has sent its final response.
    pub async fn finish(self) -> Result<()> {
        self.mode
            .termination_signal_tx
            .send(())
            .map_err(|_| {
                Error::new_msg(
                    ErrorKind::StreamError,
                    "failed to initiate final checkpoint",
                )
            })
            .help("please contact Sift")?;

        self.mode
            .streaming_task
            .await
            .map_err(|e| Error::new(ErrorKind::StreamError, e))
            .context("final checkpoint failure")
            .help("the final checkpoint may or may not have succeeded. Please contact Sift")?;

        Ok(())
    }

    fn init_streaming_task(
        grpc_channel: SiftChannel,
        data_stream: DataStream,
        checkpoint_interval: Duration,
        data_tx: UnboundedSender<Message>,
        termination_signal_rx: Receiver<()>,
        server_res_tx: SyncSender<Result<IngestWithConfigDataStreamResponse>>,
    ) -> JoinHandle<()> {
        tokio::spawn(async move {
            let mut client = IngestServiceClient::new(grpc_channel);

            let checkpoint_task = tokio::spawn(async move {
                let mut checkpoint_timer = {
                    let mut timer = tokio::time::interval(checkpoint_interval);
                    // Time goes off immediately
                    timer.tick().await;
                    timer
                };
                // TODO: Log warning if message fails to send
                tokio::select! {
                    _ = checkpoint_timer.tick() => {
                        let _ = data_tx.send(Message(InnerMessage::CheckpointSignal));
                    }
                    _ = termination_signal_rx => {
                        let _ = data_tx.send(Message(InnerMessage::CheckpointSignal));
                    }
                }
            });

            let response = client
                .ingest_with_config_data_stream(data_stream)
                .await
                .map(|res| res.into_inner())
                .map_err(|e| Error::new(ErrorKind::StreamError, e))
                .context("failed to receive checkpoint acknowledgement")
                .help("please contact Sift");

            checkpoint_task.abort_handle().abort();
            // TODO: Assert/log failure to cancel?
            let _ = checkpoint_task.await;
            let _ = server_res_tx.send(response);
        })
    }
}

impl DataStream {
    fn new(
        ingestion_config: &IngestionConfig,
        flows: &[FlowConfig],
        data_rx: UnboundedReceiver<Message>,
        run: Option<&Run>,
        recovery_tx: UnboundedSender<Message>,
    ) -> Self {
        Self {
            run_id: run.map(|r| r.run_id.clone()),
            data_rx,
            ingestion_config_id: ingestion_config.ingestion_config_id.clone(),
            flows: flows.to_vec(),
            recovery_tx,
        }
    }
}

impl Stream for DataStream {
    type Item = IngestWithConfigDataStreamRequest;

    fn poll_next(mut self: Pin<&mut Self>, ctx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let Message(message) = match self.data_rx.poll_recv(ctx) {
            Poll::Ready(Some(msg)) => msg,
            Poll::Ready(None) => {
                #[cfg(feature = "tracing")]
                tracing::error!("stream terminating unexpectedly. Please notify Sift");

                // If this happens then someone has introduced a critical bug where either the `data_rx` channel is
                // manually getting closed or all senders are getting dropped prematurely. We have
                // safeguards to prevent senders getting dropped i.e. by having `IngestionConfigMode`
                // own both a sender as well as the task that owns DataStream; so really, the only way
                // this can happen is if someone manually closes the `data_rx` channel, however there is
                // absolutely no good reason to do that so we'll scream loudly during development if we
                // see it.
                #[cfg(debug_assertions)]
                {
                    eprintln!("[DEBUG_ASSERTIONS]: polling failed unexpectedly. This is critical and needs to be addressed before release.");
                    std::process::exit(1);
                }
                #[allow(unreachable_code)]
                return Poll::Ready(None);
            }
            Poll::Pending => return Poll::Pending,
        };

        match message {
            InnerMessage::CheckpointSignal => Poll::Ready(None),
            InnerMessage::Flow {
                name,
                timestamp,
                values,
            } => {
                let Some(req) = message_to_ingest_req(
                    &name,
                    &self.ingestion_config_id,
                    self.run_id.clone().unwrap_or_default(),
                    timestamp,
                    values,
                    &self.flows,
                ) else {
                    return Poll::Ready(None);
                };
                Poll::Ready(Some(req))
            }
        }
    }
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

/// Any data that is still buffered should be sent back to [SiftStream] by the time this is
/// dropped.
///
/// TODO: Add some tracing here
impl Drop for DataStream {
    fn drop(&mut self) {
        while let Ok(message) = self.data_rx.try_recv() {
            let _ = self.recovery_tx.send(message);
        }
    }
}
