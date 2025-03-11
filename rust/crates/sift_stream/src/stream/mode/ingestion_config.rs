use super::super::{channel::ChannelValue, time::TimeValue, SiftStream, SiftStreamMode};
use futures_core::Stream;
use sift_connect::SiftChannel;
use sift_error::prelude::*;
use sift_rs::{
    ingest::v1::{
        ingest_service_client::IngestServiceClient, IngestWithConfigDataChannelValue,
        IngestWithConfigDataStreamRequest,
    },
    ingestion_configs::v2::{FlowConfig, IngestionConfig},
    runs::v2::Run,
};
use std::{
    pin::Pin,
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
    data_tx: UnboundedSender<Message>,
    streaming_task: JoinHandle<Result<()>>,
    termination_signal_tx: Sender<()>,
}

impl SiftStreamMode for IngestionConfigMode {}

pub struct Message(InnerMessage);

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
    ) -> Self {
        let (data_tx, data_rx) = unbounded_channel::<Message>();
        let (termination_signal_tx, termination_signal_rx) = oneshot::channel::<()>();

        let data_stream = DataStream::new(&ingestion_config, &flows, data_rx, run.as_ref());

        let streaming_task = Self::init_streaming_task(
            grpc_channel.clone(),
            data_stream,
            checkpoint_interval,
            data_tx.clone(),
            termination_signal_rx,
        );

        Self {
            grpc_channel,
            mode: IngestionConfigMode {
                ingestion_config,
                flows,
                run,
                streaming_task,
                checkpoint_interval,
                data_tx,
                termination_signal_tx,
            },
        }
    }

    pub fn send(&mut self, message: Message) -> Result<()> {
        match self.mode.data_tx.send(message) {
            Ok(_) => Ok(()),

            // Start a new stream; previous one concluded due to successful checkpointing
            Err(SendError(msg)) => {
                let (data_tx, data_rx) = unbounded_channel::<Message>();
                let (termination_signal_tx, termination_signal_rx) = oneshot::channel::<()>();

                self.mode.termination_signal_tx = termination_signal_tx;

                let data_stream = DataStream::new(
                    &self.mode.ingestion_config,
                    &self.mode.flows,
                    data_rx,
                    self.mode.run.as_ref(),
                );
                self.mode.data_tx = data_tx.clone();

                self.mode.streaming_task = Self::init_streaming_task(
                    self.grpc_channel.clone(),
                    data_stream,
                    self.mode.checkpoint_interval,
                    data_tx,
                    termination_signal_rx,
                );

                // resend message... woah recursion!
                self.send(msg)
            }
        }
    }

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
            .help("the final checkpoint may or may not have succeeded. Please contact Sift")??;

        Ok(())
    }

    fn init_streaming_task(
        grpc_channel: SiftChannel,
        data_stream: DataStream,
        checkpoint_interval: Duration,
        data_tx: UnboundedSender<Message>,
        termination_signal_rx: Receiver<()>,
    ) -> JoinHandle<Result<()>> {
        tokio::spawn(async move {
            macro_rules! checkpoint {
                () => {
                    let _ = data_tx
                        .send(Message(InnerMessage::CheckpointSignal))
                        .map_err(|_| {
                            Error::new_msg(
                                ErrorKind::StreamError,
                                "attempt to begin checkpoint failed unexpectedly",
                            )
                        })
                        .help("please contact Sift")?;
                };
            }

            let mut client = IngestServiceClient::new(grpc_channel);
            let mut checkpoint_timer = tokio::time::interval(checkpoint_interval);

            let checkpoint_task = tokio::spawn(async move {
                // First tick happens immediately...
                checkpoint_timer.tick().await;

                tokio::select! {
                    _ = checkpoint_timer.tick() => {
                        checkpoint!();
                        Ok::<(), Error>(())
                    }
                    _ = termination_signal_rx => {
                        checkpoint!();
                        Ok::<(), Error>(())
                    }
                }
            });

            let ingest_task = tokio::spawn(async move {
                client
                    .ingest_with_config_data_stream(data_stream)
                    .await
                    .map_err(|e| Error::new(ErrorKind::StreamError, e))
                    .context("a stream unexpectedly terminated with an error")
                    .help("please contact Sift")
            });

            checkpoint_task
                .await
                .map_err(|e| Error::new(ErrorKind::StreamError, e))
                .context("failed to complete checkpoint")
                .help("please contact Sift")??;

            ingest_task
                .await
                .map_err(|e| Error::new(ErrorKind::StreamError, e))
                .context("failed to complete receive a checkpoint acknowledgement from Sift")
                .help("please contact Sift")??;

            Ok(())
        })
    }
}

impl DataStream {
    fn new(
        ingestion_config: &IngestionConfig,
        flows: &[FlowConfig],
        data_rx: UnboundedReceiver<Message>,
        run: Option<&Run>,
    ) -> Self {
        Self {
            run_id: run.map(|r| r.run_id.clone()),
            data_rx,
            ingestion_config_id: ingestion_config.ingestion_config_id.clone(),
            flows: flows.to_vec(),
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
                let mut maybe_channel_values = None;

                for flow in self.flows.iter().filter(|f| f.name == name) {
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
                    return Poll::Ready(None);
                };

                let request = IngestWithConfigDataStreamRequest {
                    flow: name,
                    ingestion_config_id: self.ingestion_config_id.clone(),
                    timestamp: Some(timestamp.0),
                    run_id: self.run_id.clone().unwrap_or_default(),
                    channel_values,
                    ..Default::default()
                };

                Poll::Ready(Some(request))
            }
        }
    }
}
