use crate::metrics::SiftStreamMetrics;
use crate::stream::flow::FlowDescriptor;
use crate::stream::mode::ingestion_config::IngestionConfigEncoder;
use crate::stream::send_error::{SendError, TrySendError};
use crate::stream::tasks::{ControlMessage, DataMessage, LiveWithBackupsTaskConfig, TaskBuilder};
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

/// Transport for real-time streaming with disk backups and checkpointing.
///
/// Maintains two internal channels:
/// - **backup channel** — bounded; `send` awaits here for backpressure.
/// - **ingestion channel** — force-send; oldest message is evicted if full
///   (prioritizes message freshness over completeness).
///
/// Use this when durability through disk backups and checkpoint-based recovery
/// is required.
pub struct LiveStreamingWithBackups {
    message_id_counter: u64,
    backup_tx: async_channel::Sender<DataMessage>,
    ingestion_tx: async_channel::Sender<DataMessage>,
    control_tx: broadcast::Sender<ControlMessage>,
    ingestion_task: JoinHandle<Result<()>>,
    backup_manager: JoinHandle<Result<()>>,
    reingestion_task: JoinHandle<Result<()>>,
    metrics_streaming: Option<JoinHandle<Result<()>>>,
    flows_seen: HashSet<String>,
    metrics: Arc<SiftStreamMetrics>,
}

impl Sealed for LiveStreamingWithBackups {}

impl LiveStreamingWithBackups {
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
        self.metrics
            .backup_channel_depth
            .set(self.backup_tx.len() as u64);
        self.metrics.messages_received.increment();

        let data_msg = DataMessage {
            message_id: self.message_id_counter,
            request: Arc::new(message),
            dropped_for_ingestion: false,
        };
        self.message_id_counter += 1;
        data_msg
    }

    /// Used by `async fn send`. If an oldest message is evicted from the ingestion
    /// channel, awaits until backup has space to accept it. Returns the undeliverable
    /// message on backup channel close or ingestion channel close.
    async fn dispatch_to_ingestion(
        &mut self,
        stream_id: &Uuid,
        data_msg: DataMessage,
    ) -> Option<IngestWithConfigDataStreamRequest> {
        match self.ingestion_tx.force_send(data_msg) {
            Ok(None) => None,
            Ok(Some(mut oldest)) => {
                oldest.dropped_for_ingestion = true;
                self.metrics.old_messages_dropped_for_ingestion.increment();
                self.metrics.checkpoint.failed_checkpoint_count.increment();
                match self.backup_tx.send(oldest).await {
                    Ok(()) => {
                        self.metrics.messages_sent_to_backup.increment();
                        None
                    }
                    Err(async_channel::SendError(dm)) => {
                        self.metrics
                            .old_messages_failed_adding_to_backup
                            .increment();
                        #[cfg(feature = "tracing")]
                        tracing::debug!(sift_stream_id = %stream_id, "backup channel closed while dispatching evicted message");
                        Some(Arc::try_unwrap(dm.request).unwrap_or_else(|arc| (*arc).clone()))
                    }
                }
            }
            Err(async_channel::SendError(dm)) => {
                #[cfg(feature = "tracing")]
                tracing::debug!(sift_stream_id = %stream_id, "ingestion channel closed");
                Some(Arc::try_unwrap(dm.request).unwrap_or_else(|arc| (*arc).clone()))
            }
        }
    }

    /// Used by `fn try_send`. If an oldest message is evicted from the ingestion
    /// channel and backup is full or closed, returns the evicted message to the
    /// caller. Also returns the message when the ingestion channel itself is closed.
    fn try_dispatch_to_ingestion(
        &mut self,
        stream_id: &Uuid,
        data_msg: DataMessage,
    ) -> Option<IngestWithConfigDataStreamRequest> {
        match self.ingestion_tx.force_send(data_msg) {
            Ok(None) => None,
            Ok(Some(mut oldest)) => {
                oldest.dropped_for_ingestion = true;
                self.metrics.old_messages_dropped_for_ingestion.increment();
                self.metrics.checkpoint.failed_checkpoint_count.increment();
                match self.backup_tx.try_send(oldest) {
                    Ok(()) => {
                        self.metrics.messages_sent_to_backup.increment();
                        None
                    }
                    Err(async_channel::TrySendError::Full(dm)) => {
                        self.metrics
                            .old_messages_failed_adding_to_backup
                            .increment();
                        Some(Arc::try_unwrap(dm.request).unwrap_or_else(|arc| (*arc).clone()))
                    }
                    Err(async_channel::TrySendError::Closed(dm)) => {
                        self.metrics
                            .old_messages_failed_adding_to_backup
                            .increment();
                        #[cfg(feature = "tracing")]
                        tracing::debug!(sift_stream_id = %stream_id, "backup channel closed while dispatching evicted message");
                        Some(Arc::try_unwrap(dm.request).unwrap_or_else(|arc| (*arc).clone()))
                    }
                }
            }
            Err(async_channel::SendError(dm)) => {
                #[cfg(feature = "tracing")]
                tracing::debug!(sift_stream_id = %stream_id, "ingestion channel closed");
                Some(Arc::try_unwrap(dm.request).unwrap_or_else(|arc| (*arc).clone()))
            }
        }
    }
}

#[async_trait]
impl Transport for LiveStreamingWithBackups {
    type Encoder = IngestionConfigEncoder;
    type Message = IngestWithConfigDataStreamRequest;

    /// Sends a message, awaiting capacity if the stream is busy.
    ///
    /// This mode prioritizes freshness: newer messages always make it into the stream, and
    /// older buffered messages are displaced to make room when necessary. As a result, if
    /// the stream is closed while a displaced message is being handled, **the message
    /// returned inside `Err` may be older than the message provided**. An error is only
    /// returned on stream close; normal backpressure is handled transparently by waiting.
    async fn send(
        &mut self,
        stream_id: &Uuid,
        message: Self::Message,
    ) -> std::result::Result<(), SendError<Self::Message>> {
        let data_msg = self.prepare_message(stream_id, message);

        self.backup_tx
            .send(data_msg.clone())
            .await
            .map_err(|async_channel::SendError(dm)| {
                SendError(Arc::try_unwrap(dm.request).unwrap_or_else(|arc| (*arc).clone()))
            })?;

        self.metrics.messages_sent_to_backup.increment();
        if let Some(displaced) = self.dispatch_to_ingestion(stream_id, data_msg).await {
            return Err(SendError(displaced));
        }
        Ok(())
    }

    /// Attempts to send a message without blocking.
    ///
    /// Returns immediately with `TrySendError::Full` or `TrySendError::Closed` if the
    /// stream cannot accept data right now.
    ///
    /// This mode prioritizes freshness: newer messages always make it into the stream, and
    /// older buffered messages are displaced to make room when necessary. If such a
    /// displacement occurs and the stream is full or closed at that point, **the message
    /// returned inside `Err` may be older than the message provided**.
    fn try_send(
        &mut self,
        stream_id: &Uuid,
        message: Self::Message,
    ) -> std::result::Result<(), TrySendError<Self::Message>> {
        let data_msg = self.prepare_message(stream_id, message);

        match self.backup_tx.try_send(data_msg.clone()) {
            Ok(()) => {}
            Err(async_channel::TrySendError::Full(dm)) => {
                return Err(TrySendError::Full(
                    Arc::try_unwrap(dm.request).unwrap_or_else(|arc| (*arc).clone()),
                ));
            }
            Err(async_channel::TrySendError::Closed(dm)) => {
                return Err(TrySendError::Closed(
                    Arc::try_unwrap(dm.request).unwrap_or_else(|arc| (*arc).clone()),
                ));
            }
        }

        self.metrics.messages_sent_to_backup.increment();
        if let Some(displaced) = self.try_dispatch_to_ingestion(stream_id, data_msg) {
            return Err(TrySendError::Full(displaced));
        }
        Ok(())
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

    /// This will conclude the stream and return when Sift has sent its final response. It is
    /// important that this method be called in order to obtain the final checkpoint
    /// acknowledgement from Sift, otherwise some tail-end data may fail to send.
    async fn finish(self, stream_id: &Uuid) -> Result<()> {
        drop(self.ingestion_tx);
        drop(self.backup_tx);

        self.control_tx
            .send(ControlMessage::Shutdown)
            .map_err(|e| Error::new(ErrorKind::StreamError, e))
            .context("failed to send shutdown signal to task-based architecture")?;

        let _ = tokio::try_join!(
            self.ingestion_task,
            self.backup_manager,
            self.reingestion_task,
        );

        if let Some(metrics_streaming) = self.metrics_streaming {
            let _ = metrics_streaming.await;
        }

        #[cfg(feature = "tracing")]
        tracing::info!(
            sift_stream_id = %stream_id,
            "successfully shutdown streaming system"
        );

        Ok(())
    }
}

impl SiftStream<IngestionConfigEncoder, LiveStreamingWithBackups> {
    pub(crate) async fn new_live_with_backups(
        ingestion_config: IngestionConfig,
        flows_by_name: HashMap<String, FlowDescriptor<String>>,
        run: Option<Run>,
        task_config: LiveWithBackupsTaskConfig,
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

        let tasks = TaskBuilder::start_live_with_backups(task_config)
            .await
            .context("failed to start live-with-backups streaming tasks")?;

        Ok(Self {
            grpc_channel: grpc_channel.clone(),
            encoder: IngestionConfigEncoder {
                grpc_channel,
                flows_by_name,
                ingestion_config,
                metrics: metrics.clone(),
            },
            transport: LiveStreamingWithBackups {
                message_id_counter: 0,
                backup_tx: tasks.backup_tx,
                ingestion_tx: tasks.ingestion_tx,
                control_tx: tasks.control_tx,
                ingestion_task: tasks.ingestion,
                backup_manager: tasks.backup_manager,
                reingestion_task: tasks.reingestion,
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

    fn make_live_streaming_with_backups(
        ingestion_capacity: usize,
        backup_capacity: usize,
    ) -> (
        LiveStreamingWithBackups,
        async_channel::Receiver<DataMessage>,
        async_channel::Receiver<DataMessage>,
    ) {
        let (control_tx, _) = broadcast::channel(10);
        let (ingestion_tx, ingestion_rx) = async_channel::bounded(ingestion_capacity);
        let (backup_tx, backup_rx) = async_channel::bounded(backup_capacity);

        let transport = LiveStreamingWithBackups {
            message_id_counter: 0,
            backup_tx,
            ingestion_tx,
            control_tx,
            ingestion_task: tokio::spawn(async { Ok(()) }),
            backup_manager: tokio::spawn(async { Ok(()) }),
            reingestion_task: tokio::spawn(async { Ok(()) }),
            metrics_streaming: None,
            flows_seen: HashSet::new(),
            metrics: Arc::new(crate::metrics::SiftStreamMetrics::default()),
        };

        (transport, ingestion_rx, backup_rx)
    }

    #[tokio::test]
    async fn test_try_send_backup_closed_returns_closed() {
        let (mut live, _ingestion_rx, backup_rx) = make_live_streaming_with_backups(10, 10);
        drop(backup_rx);
        let stream_id = uuid::Uuid::new_v4();
        let req = make_request();
        let flow = req.flow.clone();
        let err = live.try_send(&stream_id, req).unwrap_err();
        assert!(err.is_closed(), "expected Closed, got {err}");
        assert_eq!(err.into_inner().flow, flow);
    }

    #[tokio::test]
    async fn test_try_send_backup_full_returns_full() {
        let (mut live, _ingestion_rx, backup_rx) = make_live_streaming_with_backups(10, 1);
        let dummy = DataMessage {
            message_id: 0,
            request: Arc::new(make_request()),
            dropped_for_ingestion: false,
        };
        live.backup_tx.try_send(dummy).unwrap();

        let stream_id = uuid::Uuid::new_v4();
        let req = make_request();
        let flow = req.flow.clone();
        let err = live.try_send(&stream_id, req).unwrap_err();
        assert!(err.is_full(), "expected Full, got {err}");
        assert_eq!(err.into_inner().flow, flow);
        drop(backup_rx);
    }

    #[tokio::test]
    async fn test_send_backup_closed_returns_send_error() {
        let (mut live, _ingestion_rx, backup_rx) = make_live_streaming_with_backups(10, 10);
        drop(backup_rx);
        let stream_id = uuid::Uuid::new_v4();
        let req = make_request();
        let flow = req.flow.clone();
        let err = live.send(&stream_id, req).await.unwrap_err();
        assert_eq!(err.into_inner().flow, flow);
    }

    #[tokio::test]
    async fn test_send_blocks_until_backup_space_available() {
        let (mut live, _ingestion_rx, backup_rx) = make_live_streaming_with_backups(10, 1);
        let dummy = DataMessage {
            message_id: 0,
            request: Arc::new(make_request()),
            dropped_for_ingestion: false,
        };
        live.backup_tx.try_send(dummy).unwrap();

        tokio::spawn(async move {
            tokio::time::sleep(std::time::Duration::from_millis(20)).await;
            let _ = backup_rx.recv().await;
            tokio::time::sleep(std::time::Duration::from_millis(100)).await;
        });

        let stream_id = uuid::Uuid::new_v4();
        live.send(&stream_id, make_request()).await.unwrap();
    }

    #[tokio::test]
    async fn test_try_send_requests_returns_undelivered_on_full() {
        let (mut live, _ingestion_rx, backup_rx) = make_live_streaming_with_backups(10, 1);
        let dummy = DataMessage {
            message_id: 0,
            request: Arc::new(make_request()),
            dropped_for_ingestion: false,
        };
        live.backup_tx.try_send(dummy).unwrap();

        let stream_id = uuid::Uuid::new_v4();
        let reqs = vec![make_request(), make_request(), make_request()];
        let err = live.try_send_requests(&stream_id, reqs).unwrap_err();
        assert!(err.is_full(), "expected Full, got {err}");
        assert_eq!(err.into_inner().len(), 3);
        drop(backup_rx);
    }

    #[tokio::test]
    async fn test_send_requests_returns_undelivered_on_closed() {
        let (mut live, _ingestion_rx, backup_rx) = make_live_streaming_with_backups(10, 10);
        drop(backup_rx);

        let stream_id = uuid::Uuid::new_v4();
        let reqs = vec![make_request(), make_request(), make_request()];
        let err = live.send_requests(&stream_id, reqs).await.unwrap_err();
        assert_eq!(err.into_inner().len(), 3);
    }
}
