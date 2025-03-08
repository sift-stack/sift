use super::super::{channel::ChannelDataPoint, time::TimeValue, SiftStream, SiftStreamMode};
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
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
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
    streaming_task: JoinHandle<Result<()>>,
    ingestion_config: IngestionConfig,
    flows: Vec<FlowConfig>,
    run: Option<Run>,
    checkpoint_interval: Duration,
    data_tx: UnboundedSender<Message>,
}

impl SiftStreamMode for IngestionConfigMode {}

pub struct Message {
    flow: String,
    timestamp: TimeValue,
    values: Vec<ChannelDataPoint>,
}

/// Dependencies used in the Tokio task that actually sends the data to Sift.
struct DataStream {
    flows: Vec<FlowConfig>,
    ingestion_config_id: String,
    run_id: Option<String>,
    data_rx: UnboundedReceiver<Message>,
    _termination_listener: JoinHandle<Result<()>>,
    terminated: Arc<AtomicBool>,
}

impl SiftStream<IngestionConfigMode> {
    pub(crate) fn new(
        grpc_channel: SiftChannel,
        ingestion_config: IngestionConfig,
        flows: Vec<FlowConfig>,
        run: Option<Run>,
        checkpoint_interval: Duration,
    ) -> Self {
        let (data_tx, data_rx) = unbounded_channel::<Message>();
        let (terminate_tx, terminate_rx) = oneshot::channel::<()>();

        let data_stream = DataStream::new(
            &ingestion_config,
            &flows,
            data_rx,
            run.as_ref(),
            terminate_rx,
        );

        let streaming_task = Self::init_streaming_task(
            grpc_channel.clone(),
            data_stream,
            checkpoint_interval,
            terminate_tx,
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
            },
        }
    }

    pub fn send(&mut self, message: Message) -> Result<()> {
        match self.mode.data_tx.send(message) {
            Ok(_) => Ok(()),

            // Start a new stream; previous one concluded due to successful checkpointing
            Err(SendError(msg)) => {
                let (data_tx, data_rx) = unbounded_channel::<Message>();
                let (terminate_tx, terminate_rx) = oneshot::channel::<()>();

                let data_stream = DataStream::new(
                    &self.mode.ingestion_config,
                    &self.mode.flows,
                    data_rx,
                    self.mode.run.as_ref(),
                    terminate_rx,
                );
                self.mode.data_tx = data_tx;

                self.mode.streaming_task = Self::init_streaming_task(
                    self.grpc_channel.clone(),
                    data_stream,
                    self.mode.checkpoint_interval,
                    terminate_tx,
                );

                // resend message... woah recursion!
                self.send(msg)
            }
        }
    }

    fn init_streaming_task(
        grpc_channel: SiftChannel,
        data_stream: DataStream,
        checkpoint_interval: Duration,
        terminate_tx: Sender<()>,
    ) -> JoinHandle<Result<()>> {
        tokio::task::spawn(async move {
            let mut client = IngestServiceClient::new(grpc_channel);
            let mut checkpoint_timer = tokio::time::interval(checkpoint_interval);

            tokio::select! {
                _ = checkpoint_timer.tick() => {
                    terminate_tx.send(())
                        .map_err(|_| Error::new_msg(ErrorKind::StreamError, "failed to start checkpoint"))
                        .help("please contact Sift")?;
                    Ok(())
                }
                resp = client.ingest_with_config_data_stream(data_stream) => {
                    resp.map_err(|e| Error::new(ErrorKind::StreamError, e))
                        .context("a stream unexpectedly terminated with an error")
                        .help("please contact Sift")?;
                    Ok(())
                }
            }
        })
    }
}

impl DataStream {
    fn new(
        ingestion_config: &IngestionConfig,
        flows: &[FlowConfig],
        data_rx: UnboundedReceiver<Message>,
        run: Option<&Run>,
        termination_rx: Receiver<()>,
    ) -> Self {
        let terminated = Arc::new(AtomicBool::new(false));

        let termination_switch = terminated.clone();
        let termination_listener = tokio::task::spawn(async move {
            let _ = termination_rx
                .await
                .map_err(|e| Error::new(ErrorKind::StreamError, e))
                .context("stream failed to shutdown unexpectedly")
                .help("please context Sift");
            termination_switch.swap(true, Ordering::Relaxed);
            Ok(())
        });

        Self {
            run_id: run.map(|r| r.run_id.clone()),
            data_rx,
            ingestion_config_id: ingestion_config.ingestion_config_id.clone(),
            flows: flows.to_vec(),
            terminated,
            _termination_listener: termination_listener,
        }
    }
}

impl Stream for DataStream {
    type Item = IngestWithConfigDataStreamRequest;

    fn poll_next(mut self: Pin<&mut Self>, _ctx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        // This will terminate this stream
        if self.terminated.load(Ordering::Relaxed) {
            return Poll::Ready(None);
        }

        let Some(Message {
            flow,
            timestamp,
            values,
        }) = self.data_rx.blocking_recv()
        else {
            #[cfg(feature = "tracing")]
            tracing::error!("termination terminated in an unexpected manner. Please contact Sift.");

            // This shouldn't really happen
            return Poll::Ready(None);
        };

        let mut maybe_channel_values = None;

        for flow in self.flows.iter().filter(|f| f.name == flow) {
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
            flow,
            ingestion_config_id: self.ingestion_config_id.clone(),
            timestamp: Some(timestamp.0),
            run_id: self.run_id.clone().unwrap_or_default(),
            channel_values,
            ..Default::default()
        };

        Poll::Ready(Some(request))
    }
}
