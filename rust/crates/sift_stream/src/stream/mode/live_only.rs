use crate::metrics::SiftStreamMetrics;
use crate::stream::flow::FlowDescriptor;
use crate::stream::mode::ingestion_config::IngestionConfigEncoder;
use crate::stream::send_error::{SendError, TrySendError};
use crate::stream::tasks::{ControlMessage, DataMessage, LiveOnlyTaskConfig, TaskBuilder};
use crate::stream::{SiftStream, Transport, private::Sealed};
use async_trait::async_trait;
use sift_error::prelude::*;
use sift_rs::{
    ingest::v1::IngestWithConfigDataStreamRequest, ingestion_configs::v2::IngestionConfig,
    runs::v2::Run,
};
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use tokio::sync::broadcast;
use tokio::task::JoinHandle;
use uuid::Uuid;

/// Transport for real-time streaming over a single bounded ingestion channel.
///
/// Messages are delivered directly to the gRPC ingestion task. The caller blocks
/// until the ingestion task drains capacity.
///
/// **Backpressure**: [`send`](crate::SiftStream::send) awaits when the **ingestion channel**
/// is full. The channel capacity is set via
/// [`LiveOnlyBuilder::ingestion_data_channel_capacity`](crate::LiveOnlyBuilder::ingestion_data_channel_capacity)
/// (default: [`DATA_CHANNEL_CAPACITY`](crate::stream::tasks::DATA_CHANNEL_CAPACITY)).
pub struct LiveStreamingOnly {
    message_id_counter: u64,
    ingestion_tx: async_channel::Sender<DataMessage>,
    control_tx: broadcast::Sender<ControlMessage>,
    ingestion_task: JoinHandle<Result<()>>,
    metrics_streaming: Option<JoinHandle<Result<()>>>,
    flows_seen: HashSet<String>,
    metrics: Arc<SiftStreamMetrics>,
}

impl Sealed for LiveStreamingOnly {}

impl LiveStreamingOnly {
    fn prepare_message(
        &mut self,
        stream_id: &Uuid,
        message: IngestWithConfigDataStreamRequest,
    ) -> DataMessage {
        #[cfg(feature = "tracing")]
        {
            if !self.flows_seen.contains(&message.flow) {
                self.metrics.unique_flows_received.increment();
                self.flows_seen.insert(message.flow.clone());
                tracing::info!(sift_stream_id = %stream_id, "flow '{}' being ingested for the first time", &message.flow);
            }
        }

        self.metrics
            .ingestion_channel_depth
            .set(self.ingestion_tx.len() as u64);
        self.metrics.messages_received.increment();

        let data_msg = DataMessage {
            message_id: self.message_id_counter,
            request: Arc::new(message),
            dropped_for_ingestion: false,
        };
        self.message_id_counter += 1;
        data_msg
    }
}

#[async_trait]
impl Transport for LiveStreamingOnly {
    type Encoder = IngestionConfigEncoder;
    type Message = IngestWithConfigDataStreamRequest;

    /// Sends a message, awaiting capacity on the **ingestion channel** if it is full.
    ///
    /// Backpressure comes from the bounded ingestion channel. The caller blocks until
    /// the ingestion task drains capacity. Returns an error only if the channel is
    /// closed (i.e. the stream is shutting down).
    async fn send(
        &mut self,
        stream_id: &Uuid,
        message: Self::Message,
    ) -> std::result::Result<(), SendError<Self::Message>> {
        let data_msg = self.prepare_message(stream_id, message);
        self.ingestion_tx
            .send(data_msg)
            .await
            .map_err(|async_channel::SendError(dm)| {
                SendError(Arc::try_unwrap(dm.request).unwrap_or_else(|arc| (*arc).clone()))
            })
    }

    /// Attempts to send a message without blocking.
    ///
    /// Returns immediately with `TrySendError::Full` or `TrySendError::Closed` if the
    /// channel cannot accept data right now.
    fn try_send(
        &mut self,
        stream_id: &Uuid,
        message: Self::Message,
    ) -> std::result::Result<(), TrySendError<Self::Message>> {
        let data_msg = self.prepare_message(stream_id, message);
        self.ingestion_tx.try_send(data_msg).map_err(|e| match e {
            async_channel::TrySendError::Full(dm) => {
                TrySendError::Full(Arc::try_unwrap(dm.request).unwrap_or_else(|arc| (*arc).clone()))
            }
            async_channel::TrySendError::Closed(dm) => TrySendError::Closed(
                Arc::try_unwrap(dm.request).unwrap_or_else(|arc| (*arc).clone()),
            ),
        })
    }

    /// Sends a batch of messages in order, awaiting capacity for each one.
    async fn send_requests<I>(
        &mut self,
        stream_id: &Uuid,
        requests: I,
    ) -> std::result::Result<(), SendError<Vec<Self::Message>>>
    where
        I: IntoIterator<Item = Self::Message> + Send,
        I::IntoIter: Send,
    {
        let mut iter = requests.into_iter();
        while let Some(msg) = iter.next() {
            if let Err(SendError(failed)) = self.send(stream_id, msg).await {
                let mut undelivered = vec![failed];
                undelivered.extend(iter);
                return Err(SendError(undelivered));
            }
        }
        Ok(())
    }

    /// Attempts to send a batch of messages in order without blocking.
    fn try_send_requests<I>(
        &mut self,
        stream_id: &Uuid,
        requests: I,
    ) -> std::result::Result<(), TrySendError<Vec<Self::Message>>>
    where
        I: IntoIterator<Item = Self::Message> + Send,
        I::IntoIter: Send,
    {
        let mut iter = requests.into_iter();
        while let Some(msg) = iter.next() {
            match self.try_send(stream_id, msg) {
                Ok(()) => {}
                Err(TrySendError::Full(failed)) => {
                    let mut undelivered = vec![failed];
                    undelivered.extend(iter);
                    return Err(TrySendError::Full(undelivered));
                }
                Err(TrySendError::Closed(failed)) => {
                    let mut undelivered = vec![failed];
                    undelivered.extend(iter);
                    return Err(TrySendError::Closed(undelivered));
                }
            }
        }
        Ok(())
    }

    /// Closes the ingestion channel, sends the shutdown signal, and awaits task completion.
    ///
    /// The ingestion task drains any messages already queued before acting on the shutdown
    /// signal, so all messages sent before `finish` is called will be delivered.
    async fn finish(self, stream_id: &Uuid) -> Result<()> {
        self.ingestion_tx.close();
        let _ = self.control_tx.send(ControlMessage::Shutdown);
        let _ = self.ingestion_task.await;
        if let Some(t) = self.metrics_streaming {
            let _ = t.await;
        }

        #[cfg(feature = "tracing")]
        tracing::info!(
            sift_stream_id = %stream_id,
            "successfully shutdown live-only streaming system"
        );

        Ok(())
    }
}

impl SiftStream<IngestionConfigEncoder, LiveStreamingOnly> {
    pub(crate) async fn new_live_only(
        ingestion_config: IngestionConfig,
        flows_by_name: HashMap<String, FlowDescriptor<String>>,
        run: Option<Run>,
        task_config: LiveOnlyTaskConfig,
        metrics: Arc<SiftStreamMetrics>,
    ) -> Result<Self> {
        #[cfg(feature = "metrics-unstable")]
        {
            let uuid = task_config.sift_stream_id.to_string();
            let m = metrics.clone();
            tokio::spawn(async move {
                crate::metrics::register_metrics(uuid, m).await;
            });
        }

        metrics.loaded_flows.add(flows_by_name.len() as u64);
        let sift_stream_id = task_config.sift_stream_id;
        let grpc_channel = task_config.setup_channel.clone();

        let tasks = TaskBuilder::start_live_only(task_config)
            .await
            .context("failed to start live-only streaming tasks")?;

        Ok(Self {
            grpc_channel: grpc_channel.clone(),
            encoder: IngestionConfigEncoder {
                grpc_channel,
                flows_by_name,
                ingestion_config,
                metrics: metrics.clone(),
            },
            transport: LiveStreamingOnly {
                message_id_counter: 0,
                ingestion_tx: tasks.ingestion_tx,
                control_tx: tasks.control_tx,
                ingestion_task: tasks.ingestion,
                metrics_streaming: tasks.metrics_streaming,
                flows_seen: HashSet::new(),
                metrics,
            },
            run,
            sift_stream_id,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::stream::tasks::DataMessage;
    use tokio::sync::broadcast;

    fn make_request() -> IngestWithConfigDataStreamRequest {
        IngestWithConfigDataStreamRequest {
            ingestion_config_id: uuid::Uuid::new_v4().to_string(),
            flow: "test_flow".to_string(),
            timestamp: None,
            channel_values: vec![],
            run_id: String::new(),
            end_stream_on_validation_error: false,
            organization_id: String::new(),
        }
    }

    fn make_live_streaming_only(
        ingestion_capacity: usize,
    ) -> (LiveStreamingOnly, async_channel::Receiver<DataMessage>) {
        let (control_tx, _) = broadcast::channel(10);
        let (ingestion_tx, ingestion_rx) = async_channel::bounded(ingestion_capacity);

        let transport = LiveStreamingOnly {
            message_id_counter: 0,
            ingestion_tx,
            control_tx,
            ingestion_task: tokio::spawn(async { Ok(()) }),
            metrics_streaming: None,
            flows_seen: HashSet::new(),
            metrics: Arc::new(crate::metrics::SiftStreamMetrics::default()),
        };

        (transport, ingestion_rx)
    }

    #[tokio::test]
    async fn test_try_send_returns_full_when_channel_at_capacity() {
        let (mut transport, _ingestion_rx) = make_live_streaming_only(1);
        let dummy = DataMessage {
            message_id: 0,
            request: Arc::new(make_request()),
            dropped_for_ingestion: false,
        };
        transport.ingestion_tx.try_send(dummy).unwrap();

        let stream_id = uuid::Uuid::new_v4();
        let req = make_request();
        let flow = req.flow.clone();
        let err = transport.try_send(&stream_id, req).unwrap_err();
        assert!(err.is_full(), "expected Full, got {err}");
        assert_eq!(err.into_inner().flow, flow);
    }

    #[tokio::test]
    async fn test_try_send_closed_returns_closed() {
        let (mut transport, ingestion_rx) = make_live_streaming_only(10);
        drop(ingestion_rx);
        let stream_id = uuid::Uuid::new_v4();
        let req = make_request();
        let flow = req.flow.clone();
        let err = transport.try_send(&stream_id, req).unwrap_err();
        assert!(err.is_closed(), "expected Closed, got {err}");
        assert_eq!(err.into_inner().flow, flow);
    }

    #[tokio::test]
    async fn test_send_blocks_until_ingestion_space_available() {
        let (mut transport, ingestion_rx) = make_live_streaming_only(1);
        let dummy = DataMessage {
            message_id: 0,
            request: Arc::new(make_request()),
            dropped_for_ingestion: false,
        };
        transport.ingestion_tx.try_send(dummy).unwrap();

        tokio::spawn(async move {
            tokio::time::sleep(std::time::Duration::from_millis(20)).await;
            let _ = ingestion_rx.recv().await;
            tokio::time::sleep(std::time::Duration::from_millis(100)).await;
        });

        let stream_id = uuid::Uuid::new_v4();
        transport.send(&stream_id, make_request()).await.unwrap();
    }

    #[tokio::test]
    async fn test_finish_drains_queued_messages_before_exit() {
        let (control_tx, _) = broadcast::channel(10);
        let (ingestion_tx, ingestion_rx) = async_channel::bounded::<DataMessage>(5);

        for _ in 0..3 {
            let msg = DataMessage {
                message_id: 0,
                request: Arc::new(make_request()),
                dropped_for_ingestion: false,
            };
            ingestion_tx.try_send(msg).unwrap();
        }

        let consumer = tokio::spawn(async move {
            let mut count = 0;
            while ingestion_rx.recv().await.is_ok() {
                count += 1;
            }
            count
        });

        let transport = LiveStreamingOnly {
            message_id_counter: 3,
            ingestion_tx,
            control_tx,
            ingestion_task: tokio::spawn(async { Ok(()) }),
            metrics_streaming: None,
            flows_seen: HashSet::new(),
            metrics: Arc::new(crate::metrics::SiftStreamMetrics::default()),
        };

        let stream_id = uuid::Uuid::new_v4();
        transport.finish(&stream_id).await.unwrap();

        let count = consumer.await.unwrap();
        assert_eq!(count, 3);
    }

    #[tokio::test]
    async fn test_finish_shuts_down_ingestion_task() {
        let (control_tx, mut control_rx) = broadcast::channel(10);
        let (ingestion_tx, _ingestion_rx) = async_channel::bounded::<DataMessage>(10);

        let shutdown_task = tokio::spawn(async move {
            loop {
                if let Ok(ControlMessage::Shutdown) = control_rx.recv().await {
                    break;
                }
            }
            Ok(())
        });

        let transport = LiveStreamingOnly {
            message_id_counter: 0,
            ingestion_tx,
            control_tx,
            ingestion_task: shutdown_task,
            metrics_streaming: None,
            flows_seen: HashSet::new(),
            metrics: Arc::new(crate::metrics::SiftStreamMetrics::default()),
        };

        let stream_id = uuid::Uuid::new_v4();
        transport.finish(&stream_id).await.unwrap();
    }

    #[tokio::test]
    async fn test_message_id_increments_on_each_send() {
        let (mut transport, ingestion_rx) = make_live_streaming_only(10);
        let stream_id = uuid::Uuid::new_v4();

        for _ in 0..5 {
            transport.send(&stream_id, make_request()).await.unwrap();
        }

        let mut ids: Vec<u64> = Vec::new();
        while let Ok(msg) = ingestion_rx.try_recv() {
            ids.push(msg.message_id);
        }
        assert_eq!(ids, vec![0, 1, 2, 3, 4]);
    }

    #[tokio::test]
    async fn test_try_send_requests_stops_at_first_full() {
        let (mut transport, _ingestion_rx) = make_live_streaming_only(1);

        // Pre-fill the channel to capacity.
        let dummy = DataMessage {
            message_id: 99,
            request: Arc::new(make_request()),
            dropped_for_ingestion: false,
        };
        transport.ingestion_tx.try_send(dummy).unwrap();

        let stream_id = uuid::Uuid::new_v4();
        let reqs = vec![make_request(), make_request(), make_request()];
        let err = transport.try_send_requests(&stream_id, reqs).unwrap_err();
        assert!(err.is_full(), "expected Full, got {err}");
        assert_eq!(err.into_inner().len(), 3);
    }

    #[tokio::test]
    async fn test_try_send_requests_stops_at_first_closed() {
        let (mut transport, ingestion_rx) = make_live_streaming_only(10);
        drop(ingestion_rx);

        let stream_id = uuid::Uuid::new_v4();
        let reqs = vec![make_request(), make_request(), make_request()];
        let err = transport.try_send_requests(&stream_id, reqs).unwrap_err();
        assert!(err.is_closed(), "expected Closed, got {err}");
        assert_eq!(err.into_inner().len(), 3);
    }

    #[tokio::test]
    async fn test_send_requests_stops_at_first_closed() {
        let (mut transport, ingestion_rx) = make_live_streaming_only(10);
        drop(ingestion_rx);

        let stream_id = uuid::Uuid::new_v4();
        let reqs = vec![make_request(), make_request(), make_request()];
        let err = transport.send_requests(&stream_id, reqs).await.unwrap_err();
        // All three returned: the one that failed plus the remaining two.
        assert_eq!(err.into_inner().len(), 3);
    }
}
