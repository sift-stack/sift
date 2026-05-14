use crate::FlowBuilder;
use crate::metrics::{SiftStreamMetrics, SiftStreamMetricsSnapshot};
use crate::stream::flow::add_new_flows;
use crate::stream::{Encodeable, Encoder, MetricsSnapshot};
use crate::stream::{
    channel::ChannelValue,
    flow::FlowDescriptor,
    private::Sealed,
    tasks::{ControlMessage, DataMessage},
    time::TimeValue,
};

use futures_core::Stream;
use prost::Message;
use sift_error::prelude::*;
use sift_rs::SiftChannel;
use sift_rs::ingestion_configs::v2::FlowConfig;
use sift_rs::{
    ingest::v1::IngestWithConfigDataStreamRequest, ingestion_configs::v2::IngestionConfig,
    runs::v2::Run,
};
use std::{
    collections::HashMap,
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

pub struct IngestionConfigEncoder {
    pub(crate) grpc_channel: SiftChannel,
    pub(crate) flows_by_name: HashMap<String, FlowDescriptor<String>>,
    pub(crate) ingestion_config: IngestionConfig,
    pub(crate) metrics: Arc<SiftStreamMetrics>,
}

impl Encoder for IngestionConfigEncoder {
    type Message = IngestWithConfigDataStreamRequest;
}

impl MetricsSnapshot for IngestionConfigEncoder {
    fn snapshot(&self) -> SiftStreamMetricsSnapshot {
        self.metrics.snapshot()
    }
}

impl Sealed for IngestionConfigEncoder {}

impl IngestionConfigEncoder {
    fn ingestion_config_id(&self) -> &str {
        &self.ingestion_config.ingestion_config_id
    }

    /// Modify the existing ingestion config by adding new flows that weren't accounted for during
    /// initialization. This will register the flows with Sift.
    ///
    /// [`ErrorKind::AlreadyExistsError`] from any flow creation is treated as success: a
    /// concurrent SiftStream instance may have won the race to create that flow.
    pub async fn add_new_flows(&mut self, flow_configs: &[FlowConfig]) -> Result<()> {
        let filtered: Vec<&FlowConfig> = flow_configs
            .iter()
            .filter(|f| !self.flows_by_name.contains_key(&f.name))
            .collect();

        if filtered.is_empty() {
            return Ok(());
        }

        // Clone to produce owned copies for the spawned tasks; refs are kept for result processing.
        let owned: Vec<FlowConfig> = filtered.iter().map(|&f| f.clone()).collect();

        let results =
            add_new_flows(self.grpc_channel.clone(), self.ingestion_config_id(), owned).await;

        let mut add_config = |config: &FlowConfig| -> Result<()> {
            let flow_name = config.name.clone();
            let flow_descriptor = FlowDescriptor::try_from((self.ingestion_config_id(), config))?;
            self.flows_by_name.insert(flow_name, flow_descriptor);

            #[cfg(feature = "tracing")]
            tracing::info!(flow = config.name, "successfully registered new flow");

            Ok(())
        };

        for (config, result) in filtered.into_iter().zip(results) {
            match result {
                Ok(Ok(())) => {
                    add_config(config)?;
                }
                Ok(Err(e)) if e.kind() == ErrorKind::AlreadyExistsError => {
                    add_config(config)?;
                }
                Ok(Err(e)) => {
                    #[cfg(feature = "tracing")]
                    tracing::error!("failed to create flow {}: {e}", config.name);
                }
                Err(e) => {
                    #[cfg(feature = "tracing")]
                    tracing::error!("failed to create flow {}: {e}", config.name);
                }
            }
        }

        self.metrics
            .loaded_flows
            .add(self.flows_by_name.len() as u64);

        Ok(())
    }

    /// Get a copy of the current flow descriptors known to SiftStream as a HashMap keyed to the flow name.
    pub fn get_flows(&self) -> HashMap<String, FlowDescriptor<String>> {
        self.flows_by_name
            .iter()
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect()
    }

    /// Get the flow descriptor for a given flow name.
    pub fn get_flow_descriptor(&self, flow_name: &str) -> Result<FlowDescriptor<String>> {
        self.flows_by_name
            .get(flow_name)
            .cloned()
            .ok_or(Error::new_msg(
                ErrorKind::NotFoundError,
                format!("flow '{}' not found", flow_name),
            ))
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

impl Encodeable for Flow {
    type Output = IngestWithConfigDataStreamRequest;
    type Encoder = IngestionConfigEncoder;

    fn encode(
        self,
        encoder: &mut Self::Encoder,
        stream_id: &Uuid,
        run: Option<&Run>,
    ) -> Option<Self::Output> {
        let req = if let Some(flows) = encoder.flows_by_name.get(&self.flow_name) {
            if let Some(req) = super::super::helpers::message_to_ingest_req(&self, run, flows) {
                req
            } else {
                #[cfg(feature = "tracing")]
                tracing::warn!(
                    sift_stream_id = %stream_id,
                    values = format!("{:?}", self.flow_name),
                    "encountered a message that doesn't match any cached flows - message will still be written to file"
                );
                super::super::helpers::message_to_ingest_req_direct(
                    &self,
                    encoder.ingestion_config_id(),
                    run,
                )
            }
        } else {
            #[cfg(feature = "tracing")]
            tracing::warn!(
                sift_stream_id = %stream_id,
                "flow '{}' not found in local flow cache - message will still be written to file",
                self.flow_name,
            );
            super::super::helpers::message_to_ingest_req_direct(
                &self,
                encoder.ingestion_config_id(),
                run,
            )
        };

        Some(req)
    }
}

impl<K> Encodeable for FlowBuilder<'_, K>
where
    K: Eq + core::hash::Hash,
{
    type Output = IngestWithConfigDataStreamRequest;
    type Encoder = IngestionConfigEncoder;

    fn encode(
        mut self,
        _: &mut Self::Encoder,
        _: &Uuid,
        run: Option<&Run>,
    ) -> Option<Self::Output> {
        if let Some(run) = run {
            self.attach_run_id(run.run_id.clone());
        }

        Some(self.request(TimeValue::now()))
    }
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
                    sift_stream_id = %self.sift_stream_id,
                    "received signal to conclude SiftStream"
                );
                Poll::Ready(None)
            }
            Poll::Pending => Poll::Pending,
        }
    }
}
