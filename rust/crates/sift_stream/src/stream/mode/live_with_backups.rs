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

/// Transport for real-time streaming with periodic checkpointing and optional disk backups.
///
/// Maintains two internal bounded channels:
///
/// - **backup channel** — the primary durability path and the sole source of backpressure.
///   [`send`](crate::SiftStream::send) awaits when this channel is full. Capacity is set via
///   [`LiveWithBackupsBuilder::backup_data_channel_capacity`](crate::LiveWithBackupsBuilder::backup_data_channel_capacity)
///   (default: [`DATA_CHANNEL_CAPACITY`](crate::stream::tasks::DATA_CHANNEL_CAPACITY)).
/// - **ingestion channel** — forwards messages to the gRPC task using force-send. When full,
///   the **oldest buffered message is evicted** rather than blocking the caller. Evicted
///   messages are redirected to the backup channel. Capacity is set via
///   [`LiveWithBackupsBuilder::ingestion_data_channel_capacity`](crate::LiveWithBackupsBuilder::ingestion_data_channel_capacity).
///
/// Because of force-send eviction, the message inside a [`SendError`](crate::SendError) or
/// [`TrySendError`](crate::TrySendError) returned by [`send`](crate::SiftStream::send) /
/// [`try_send`](crate::SiftStream::try_send) may be an **older displaced message**, not
/// necessarily the one passed to the current call.
///
/// **Disk backups are optional.** Checkpointing and retry are active regardless of whether
/// disk backups are enabled. Disk backups activate only when `disk_backup_policy.backups_dir`
/// is set via
/// [`LiveWithBackupsBuilder::disk_backup_policy`](crate::LiveWithBackupsBuilder::disk_backup_policy).
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

    /// Sends a message, awaiting capacity on the **backup channel** if it is full.
    ///
    /// Backpressure comes exclusively from the bounded backup channel. Once the backup
    /// channel accepts the message, the message is dispatched to the ingestion channel via
    /// force-send: if the ingestion channel is full, the **oldest buffered message in the
    /// ingestion channel** is evicted and redirected to the backup channel — the caller does
    /// not block for this step.
    ///
    /// An error is returned only when a channel has closed (stream shutdown). Because of
    /// force-send eviction, **the message returned inside `Err` may be an older displaced
    /// message**, not necessarily the one passed to this call.
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
    /// Returns immediately with `TrySendError::Full` if the **backup channel** is at
    /// capacity, or `TrySendError::Closed` if the backup channel has been closed. If the
    /// backup channel accepts the message, force-send dispatch to the ingestion channel
    /// proceeds: if the ingestion channel is full, the oldest buffered message is evicted and
    /// a non-blocking attempt is made to redirect it to the backup channel. If that
    /// redirection also fails (backup full or closed), the evicted message is returned as
    /// `TrySendError::Full`.
    ///
    /// Because of force-send eviction, **the message returned inside `Err` may be an older
    /// displaced message**, not necessarily the one passed to this call.
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

    /// Closes both internal channels, signals shutdown, and awaits all background tasks.
    ///
    /// Dropping both channels causes the ingestion task to drain any already-queued messages
    /// before acting on the shutdown signal, so all messages sent before `finish` is called
    /// will be processed. This method also triggers the final checkpoint, which requests
    /// delivery confirmation from Sift for all data up to this point.
    ///
    /// Always call `finish` when done sending — dropping a [`SiftStream`](crate::SiftStream)
    /// without calling it may result in tail-end data not reaching Sift.
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
    use crate::stream::tasks::ControlMessage;
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

    #[tokio::test]
    async fn test_send_evicts_oldest_when_ingestion_full() {
        // ingestion capacity=1 pre-filled; backup has plenty of room.
        let (mut transport, ingestion_rx, backup_rx) = make_live_streaming_with_backups(1, 10);

        // Pre-fill ingestion with a sentinel message.
        let old_msg = DataMessage {
            message_id: 99,
            request: Arc::new(IngestWithConfigDataStreamRequest {
                ingestion_config_id: uuid::Uuid::new_v4().to_string(),
                flow: "old_flow".to_string(),
                timestamp: None,
                channel_values: vec![],
                run_id: String::new(),
                end_stream_on_validation_error: false,
                organization_id: String::new(),
            }),
            dropped_for_ingestion: false,
        };
        transport.ingestion_tx.try_send(old_msg).unwrap();

        // Sending a new message should:
        //   1. send it to backup (backup_rx[0])
        //   2. force-send it into ingestion, evicting the old message
        //   3. send the evicted old message to backup (backup_rx[1])
        let stream_id = uuid::Uuid::new_v4();
        let new_req = make_request(); // flow = "test_flow"
        transport.send(&stream_id, new_req).await.unwrap();

        // Ingestion should contain the new message.
        let in_msg = ingestion_rx.try_recv().unwrap();
        assert_eq!(in_msg.message_id, 0);
        assert!(!in_msg.dropped_for_ingestion);

        // Backup first receives the new message (sent before dispatch_to_ingestion)…
        let backup_first = backup_rx.try_recv().unwrap();
        assert_eq!(backup_first.message_id, 0);
        assert!(!backup_first.dropped_for_ingestion);

        // …then the evicted old message.
        let backup_evicted = backup_rx.try_recv().unwrap();
        assert_eq!(backup_evicted.message_id, 99);
        assert!(backup_evicted.dropped_for_ingestion);
        assert_eq!(backup_evicted.request.flow, "old_flow");
    }

    #[tokio::test]
    async fn test_try_send_evicts_oldest_to_backup_when_ingestion_full() {
        let (mut transport, ingestion_rx, backup_rx) = make_live_streaming_with_backups(1, 10);

        let old_msg = DataMessage {
            message_id: 99,
            request: Arc::new(IngestWithConfigDataStreamRequest {
                ingestion_config_id: uuid::Uuid::new_v4().to_string(),
                flow: "old_flow".to_string(),
                timestamp: None,
                channel_values: vec![],
                run_id: String::new(),
                end_stream_on_validation_error: false,
                organization_id: String::new(),
            }),
            dropped_for_ingestion: false,
        };
        transport.ingestion_tx.try_send(old_msg).unwrap();

        let stream_id = uuid::Uuid::new_v4();
        transport.try_send(&stream_id, make_request()).unwrap();

        let in_msg = ingestion_rx.try_recv().unwrap();
        assert_eq!(in_msg.message_id, 0);
        assert!(!in_msg.dropped_for_ingestion);

        let backup_first = backup_rx.try_recv().unwrap();
        assert_eq!(backup_first.message_id, 0);

        let backup_evicted = backup_rx.try_recv().unwrap();
        assert_eq!(backup_evicted.message_id, 99);
        assert!(backup_evicted.dropped_for_ingestion);
        assert_eq!(backup_evicted.request.flow, "old_flow");
    }

    #[tokio::test]
    async fn test_send_returns_err_when_ingestion_closed() {
        let (mut transport, ingestion_rx, _backup_rx) = make_live_streaming_with_backups(10, 10);
        // Close the ingestion channel.
        drop(ingestion_rx);

        let stream_id = uuid::Uuid::new_v4();
        let req = make_request();
        let flow = req.flow.clone();
        // send() first succeeds writing to backup, then fails when dispatching to ingestion.
        let err = transport.send(&stream_id, req).await.unwrap_err();
        assert_eq!(err.into_inner().flow, flow);
    }

    #[tokio::test]
    async fn test_try_send_returns_full_when_evicted_and_backup_full() {
        // backup capacity=1, ingestion capacity=1 (pre-filled).
        // When try_send is called:
        //   backup accepts the new msg  (now full)
        //   force_send evicts old_msg from ingestion
        //   backup.try_send(old_msg) → Full (backup already has one item)
        //   try_send returns Full containing the evicted OLD message.
        let (mut transport, _ingestion_rx, backup_rx) = make_live_streaming_with_backups(1, 1);

        let old_msg = DataMessage {
            message_id: 99,
            request: Arc::new(IngestWithConfigDataStreamRequest {
                ingestion_config_id: uuid::Uuid::new_v4().to_string(),
                flow: "old_flow".to_string(),
                timestamp: None,
                channel_values: vec![],
                run_id: String::new(),
                end_stream_on_validation_error: false,
                organization_id: String::new(),
            }),
            dropped_for_ingestion: false,
        };
        transport.ingestion_tx.try_send(old_msg).unwrap();

        let stream_id = uuid::Uuid::new_v4();
        let err = transport.try_send(&stream_id, make_request()).unwrap_err();
        // The returned message is the evicted (older) one.
        assert!(err.is_full(), "expected Full, got {err}");
        assert_eq!(err.into_inner().flow, "old_flow");

        drop(backup_rx);
    }

    #[tokio::test]
    async fn test_message_id_counter_increments_monotonically() {
        let (mut transport, _ingestion_rx, _backup_rx) = make_live_streaming_with_backups(10, 10);
        let stream_id = uuid::Uuid::new_v4();

        for _ in 0..5 {
            transport.send(&stream_id, make_request()).await.unwrap();
        }

        assert_eq!(transport.message_id_counter, 5);
    }

    #[tokio::test]
    async fn test_finish_awaits_all_three_tasks() {
        use std::sync::atomic::{AtomicU32, Ordering};

        let completed = Arc::new(AtomicU32::new(0));
        // Keep a broadcast receiver alive so control_tx.send() doesn't fail.
        let (control_tx, _ctrl_rx) = broadcast::channel::<ControlMessage>(10);
        let (ingestion_tx, _) = async_channel::bounded::<DataMessage>(10);
        let (backup_tx, _) = async_channel::bounded::<DataMessage>(10);

        macro_rules! counting_task {
            ($counter:expr) => {{
                let c = $counter.clone();
                tokio::spawn(async move {
                    tokio::time::sleep(std::time::Duration::from_millis(5)).await;
                    c.fetch_add(1, Ordering::Relaxed);
                    Ok(())
                })
            }};
        }

        let transport = LiveStreamingWithBackups {
            message_id_counter: 0,
            backup_tx,
            ingestion_tx,
            control_tx,
            ingestion_task: counting_task!(completed),
            backup_manager: counting_task!(completed),
            reingestion_task: counting_task!(completed),
            metrics_streaming: None,
            flows_seen: std::collections::HashSet::new(),
            metrics: Arc::new(crate::metrics::SiftStreamMetrics::default()),
        };

        let stream_id = uuid::Uuid::new_v4();
        transport.finish(&stream_id).await.unwrap();

        assert_eq!(
            completed.load(Ordering::Relaxed),
            3,
            "finish() must await all three internal tasks before returning"
        );
    }
}
