use crate::{
    DiskBackupPolicy, Flow, FlowConfig, IngestionConfigForm, RecoveryStrategy, RetryPolicy,
    SiftStream, SiftStreamBuilder, TimeValue,
    backup::disk::{AsyncBackupsManager, BackupIngestTask},
    metrics::{SiftStreamMetrics, SiftStreamMetricsSnapshot},
    stream::mode::ingestion_config::{DataStream, IngestionConfigEncoder},
};
use async_channel;
use sift_connect::SiftChannel;
use sift_error::prelude::*;
use sift_rs::{
    CompressionEncoding,
    ingest::v1::{IngestWithConfigDataStreamRequest, ingest_service_client::IngestServiceClient},
};
use std::{
    path::PathBuf,
    pin::Pin,
    sync::{
        Arc,
        atomic::{AtomicU64, Ordering},
    },
    time::Duration,
};
use tokio::{select, sync::broadcast, task::JoinHandle, time::Instant};
use uuid::Uuid;

/// Capacity for the data channel.
pub(crate) const DATA_CHANNEL_CAPACITY: usize = 1024 * 100;

/// Capacity for the control channel.
pub(crate) const CONTROL_CHANNEL_CAPACITY: usize = 1024;

/// Timeout for the checkpoint operation to complete.
pub(crate) const CHECKPOINT_TIMEOUT: Duration = Duration::from_secs(10);

/// Control messages sent between tasks via broadcast channel
/// These are low-frequency control messages, not high-volume data messages
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum ControlMessage {
    /// Signal that the backup is full and a new checkpoint should be started.
    BackupFull,

    /// Request to re-ingest backup files
    ReingestBackups { backup_files: Vec<PathBuf> },

    /// Signal the next checkpoint.
    SignalNextCheckpoint,

    /// Signal to complete the checkpoint.
    CheckpointComplete {
        first_message_id: u64,
        last_message_id: u64,
    },

    /// Signal the checkpoint needs re-ingestion.
    CheckpointNeedsReingestion {
        first_message_id: u64,
        last_message_id: u64,
    },

    /// Shutdown signal for all tasks
    Shutdown,
}

#[derive(Clone)]
pub(crate) struct RecoveryConfig {
    pub(crate) retry_policy: RetryPolicy,
    pub(crate) backups_enabled: bool,
    pub(crate) backups_directory: String,
    pub(crate) backups_prefix: String,
    pub(crate) backup_policy: DiskBackupPolicy,
}

/// Configuration for the task-based SiftStream
#[derive(Clone)]
pub(crate) struct TaskConfig {
    pub(crate) session_name: String,
    pub(crate) sift_stream_id: Uuid,
    pub(crate) setup_channel: SiftChannel,
    pub(crate) ingestion_channel: SiftChannel,
    pub(crate) reingestion_channel: SiftChannel,
    pub(crate) metrics: Arc<SiftStreamMetrics>,
    pub(crate) checkpoint_interval: Duration,
    pub(crate) enable_compression_for_ingestion: bool,
    pub(crate) recovery_config: RecoveryConfig,
    pub(crate) control_channel_capacity: usize,
    pub(crate) ingestion_data_channel_capacity: usize,
    pub(crate) backup_data_channel_capacity: usize,
    pub(crate) metrics_streaming_interval: Option<Duration>,
}

/// Data message with stream ID for routing
#[derive(Debug, Clone)]
pub(crate) struct DataMessage {
    pub(crate) message_id: u64,
    pub(crate) request: Arc<IngestWithConfigDataStreamRequest>,
    pub(crate) dropped_for_ingestion: bool,
}

/// Handles for the three main tasks
pub(crate) struct StreamSystem {
    pub(crate) backup_manager: JoinHandle<Result<()>>,
    pub(crate) ingestion: JoinHandle<Result<()>>,
    pub(crate) reingestion: JoinHandle<Result<()>>,
    pub(crate) metrics_streaming: Option<JoinHandle<Result<()>>>,
    pub(crate) control_tx: broadcast::Sender<ControlMessage>,
    pub(crate) ingestion_tx: async_channel::Sender<DataMessage>,
    pub(crate) backup_tx: async_channel::Sender<DataMessage>,
}

/// Creates and starts all three tasks
pub(crate) async fn start_tasks(config: TaskConfig) -> Result<StreamSystem> {
    // Create broadcast channel for control messages (low frequency)
    let (control_tx, _control_rx) = broadcast::channel(config.control_channel_capacity);

    // Create data channel for high-frequency data messages
    let (ingestion_tx, ingestion_rx) =
        async_channel::bounded(config.ingestion_data_channel_capacity);
    let (backup_tx, backup_rx) = async_channel::bounded(config.backup_data_channel_capacity);

    // Clone the sender for each task
    let backup_control_tx = control_tx.clone();
    let ingestion_control_tx = control_tx.clone();
    let reingestion_control_tx = control_tx.clone();

    // Start backup manager task
    let backup_config = config.clone();
    let backup_control_rx = backup_control_tx.subscribe();
    let backup_data_rx = backup_rx.clone();

    let mut backup_manager = AsyncBackupsManager::new(
        backup_config.recovery_config.backups_enabled,
        &backup_config.recovery_config.backups_directory,
        &backup_config.recovery_config.backups_prefix,
        backup_config.recovery_config.backup_policy,
        backup_control_tx,
        backup_control_rx,
        backup_data_rx,
        backup_config.metrics.clone(),
    )
    .await?;

    let backup_manager = tokio::spawn(async move {
        #[cfg(feature = "tracing")]
        tracing::info!(
            sift_stream_id = %config.sift_stream_id,
            "backup manager task started"
        );

        backup_manager.run().await
    });

    // Start gRPC client task
    let ingestion_config = config.clone();
    let ingestion_data_rx = ingestion_rx.clone();
    let mut ingestion_task =
        IngestionTask::new(ingestion_control_tx, ingestion_data_rx, ingestion_config);
    let ingestion = tokio::spawn(async move {
        #[cfg(feature = "tracing")]
        tracing::info!(
            sift_stream_id = %config.sift_stream_id,
            "ingestion task started"
        );
        ingestion_task.run().await
    });

    // Re-ingestion task has it's own retry policy to give more time to re-ingest backup files
    // when the network is slow or may be out for only a minute or two.
    let reingestion_config = config.clone();
    let reingest_retry_policy = RetryPolicy {
        max_attempts: 12,
        initial_backoff: Duration::from_millis(100),
        max_backoff: Duration::from_secs(15),
        backoff_multiplier: 5,
    };
    let reingestion_task = BackupIngestTask::new(
        reingestion_control_tx.subscribe(),
        reingestion_config.reingestion_channel,
        reingestion_config.enable_compression_for_ingestion,
        reingest_retry_policy,
        reingestion_config
            .recovery_config
            .backup_policy
            .retain_backups,
        reingestion_config.metrics.clone(),
    );
    let reingestion = tokio::spawn(async move {
        #[cfg(feature = "tracing")]
        tracing::info!(
            sift_stream_id = %config.sift_stream_id,
            "backup re-ingestion task started"
        );
        reingestion_task.run().await
    });

    // Start metrics streaming task if an interval is configured.
    let metrics_config = config.clone();
    let metrics_control_rx = control_tx.subscribe();
    let metrics_streaming = if let Some(interval) = config.metrics_streaming_interval {
        let metrics_task =
            MetricsStreamingTask::new(metrics_control_rx, interval, metrics_config).await?;
        Some(tokio::spawn(async move {
            #[cfg(feature = "tracing")]
            tracing::info!(
                sift_stream_id = %config.sift_stream_id,
                "metrics streaming task started"
            );

            metrics_task.run().await
        }))
    } else {
        None
    };

    #[cfg(feature = "tracing")]
    tracing::info!(
        sift_stream_id = %config.sift_stream_id,
        "Sift streaming successfully initialized"
    );

    Ok(StreamSystem {
        backup_manager,
        ingestion,
        reingestion,
        metrics_streaming,
        control_tx,
        ingestion_tx,
        backup_tx,
    })
}

pub(crate) struct IngestionTask {
    control_tx: broadcast::Sender<ControlMessage>,
    control_rx: broadcast::Receiver<ControlMessage>,
    data_rx: async_channel::Receiver<DataMessage>,
    config: TaskConfig,
}

impl IngestionTask {
    pub(crate) fn new(
        control_tx: broadcast::Sender<ControlMessage>,
        data_rx: async_channel::Receiver<DataMessage>,
        config: TaskConfig,
    ) -> Self {
        let control_rx = control_tx.subscribe();
        Self {
            control_tx,
            control_rx,
            data_rx,
            config,
        }
    }

    pub(crate) async fn run(&mut self) -> Result<()> {
        let now = tokio::time::Instant::now();
        let mut timer = tokio::time::interval_at(
            now + self.config.checkpoint_interval,
            self.config.checkpoint_interval,
        );

        let mut stream_created_at = now;
        let mut current_wait = Duration::ZERO;

        // The stream needs to be kept alive independently from receiving control messages in the
        // loop below, so an [`Option`] is used to store the stream future and updated as needed.
        let mut stream = None;
        let first_message_id = Arc::new(AtomicU64::new(0));
        let last_message_id = Arc::new(AtomicU64::new(0));

        loop {
            // Create a new stream if one doesn't exist yet.
            if stream.is_none() {
                #[cfg(feature = "tracing")]
                tracing::info!(
                    sift_stream_id = %self.config.sift_stream_id,
                    "creating new stream"
                );

                stream_created_at = tokio::time::Instant::now();

                // Create the structs needed for the stream outside of the async task to avoid
                // any race conditions in that task being polled for the first time and other
                // events occurring in the system.
                let mut client = IngestServiceClient::new(self.config.ingestion_channel.clone());

                // If compression is enabled, add the compression codecs to the client.
                if self.config.enable_compression_for_ingestion {
                    client = client
                        .send_compressed(CompressionEncoding::Gzip)
                        .accept_compressed(CompressionEncoding::Gzip);
                }

                let data_stream = DataStream::new(
                    self.data_rx.clone(),
                    self.control_tx.clone(),
                    self.config.sift_stream_id,
                    first_message_id.clone(),
                    last_message_id.clone(),
                    self.config.metrics.clone(),
                );

                stream = Some(Box::pin(async move {
                    // Wait for the retry exponential backoff to complete before performing the next gRPC stream operation.
                    tokio::time::sleep(current_wait).await;

                    // Perform the gRPC stream operation.
                    let res = client.ingest_with_config_data_stream(data_stream).await;

                    // Currently the stream result is not used, so to simplify we return a unit value.
                    res.map(|_| ())
                        .map_err(|e| Error::new(ErrorKind::StreamError, e))
                }));

                #[cfg(feature = "tracing")]
                tracing::info!(
                    sift_stream_id = %self.config.sift_stream_id,
                    "successfully initialized a new stream to Sift"
                );
            }

            // Wait for the stream to complete or for a control message to be received.
            tokio::select! {
                res = stream.as_mut().unwrap() => {
                    match res {
                        Ok(_) => {
                            self.config.metrics.cur_retry_count.set(0);
                            current_wait = Duration::ZERO;
                        }
                        Err(e) => {
                            current_wait = self.handle_failed_stream(&e, stream_created_at, current_wait, first_message_id.load(Ordering::Relaxed), last_message_id.load(Ordering::Relaxed))?;
                        }
                    }

                    stream = None;

                    if self.data_rx.is_closed() {
                        break;
                    }
                }
                _ = timer.tick() => {
                    #[cfg(feature = "tracing")]
                    tracing::info!(
                        sift_stream_id = %self.config.sift_stream_id,
                        "checkpoint expired"
                    );

                    // Signal the next checkpoint to the data stream.
                    self.control_tx.send(ControlMessage::SignalNextCheckpoint).map_err(|e| Error::new(ErrorKind::StreamError, e))?;
                    self.config.metrics.checkpoint.checkpoint_timer_reached_cnt.increment();

                    // Timeout if Sift doesn't respond to the checkpoint signal quickly.
                    match tokio::time::timeout(CHECKPOINT_TIMEOUT, stream.as_mut().unwrap()).await {
                        Ok(Ok(_)) => {
                            #[cfg(feature = "tracing")]
                            tracing::info!(
                                sift_stream_id = %self.config.sift_stream_id,
                                "checkpoint succeeded - data streamed to Sift successfully"
                            );
                            self.config.metrics.cur_retry_count.set(0);
                        }
                        Ok(Err(e)) => {
                            current_wait = self.handle_failed_stream(&e, stream_created_at, current_wait, first_message_id.load(Ordering::Relaxed), last_message_id.load(Ordering::Relaxed))?;
                        }
                        Err(elapsed) => {
                            #[cfg(feature = "tracing")]
                            tracing::error!(
                                sift_stream_id = %self.config.sift_stream_id,
                                error = %elapsed,
                                "timed out waiting for checkpoint completion from Sift"
                            );
                            current_wait = self.handle_failed_stream(&Error::new(ErrorKind::StreamError, elapsed), stream_created_at, current_wait, first_message_id.load(Ordering::Relaxed), last_message_id.load(Ordering::Relaxed))?;
                        }
                    }

                    self.config.metrics.checkpoint.next_checkpoint();
                    self.control_tx.send(ControlMessage::CheckpointComplete { first_message_id: first_message_id.load(Ordering::Relaxed), last_message_id: last_message_id.load(Ordering::Relaxed) }).map_err(|e| Error::new(ErrorKind::StreamError, e))?;
                    stream = None;
                }
                ctrl_msg = self.control_rx.recv() => {
                    match ctrl_msg {
                        Ok(ControlMessage::BackupFull) => {
                            #[cfg(feature = "tracing")]
                            tracing::info!(
                                sift_stream_id = %self.config.sift_stream_id,
                                "backup full"
                            );

                            // Reset the timer to expire immediately to start a new checkpoint since backups are full.
                            self.config.metrics.checkpoint.checkpoint_manually_reached_cnt.increment();
                            timer.reset_immediately();
                        }
                        Ok(ControlMessage::Shutdown) => {
                            break;
                        }
                        _ => continue,
                    }
                }
            }
        }

        self.shutdown(stream, first_message_id, last_message_id)
            .await?;

        Ok(())
    }

    /// Handle a failed stream operation, sending the re-ingest signal and logging the error and incrementing metrics.
    fn handle_failed_stream(
        &mut self,
        e: &Error,
        stream_created_at: Instant,
        current_wait: Duration,
        first_message_id: u64,
        last_message_id: u64,
    ) -> Result<Duration> {
        #[cfg(feature = "tracing")]
        tracing::error!(
            sift_stream_id = %self.config.sift_stream_id,
            retry_counter = self.config.metrics.cur_retry_count.get(),
            error = %e,
            "stream failed - failed to ingest data to Sift - if backups are enabled, backup files will be re-ingested"
        );

        self.config
            .metrics
            .checkpoint
            .failed_checkpoint_count
            .increment();
        self.control_tx
            .send(ControlMessage::CheckpointNeedsReingestion {
                first_message_id,
                last_message_id,
            })
            .map_err(|e| Error::new(ErrorKind::StreamError, e))?;

        // If the stream was healthy for sufficiently long, reset the wait time used for exponential backoff.
        let backoff = if stream_created_at.elapsed()
            > self.config.recovery_config.retry_policy.max_backoff * 2
        {
            self.config.metrics.cur_retry_count.set(0);
            Duration::ZERO
        } else {
            self.config.metrics.cur_retry_count.add(1);
            self.config
                .recovery_config
                .retry_policy
                .backoff(current_wait)
        };

        Ok(backoff)
    }

    /// Shuts down the ingestion task by awaiting the stream one last time and sending the final checkpoint complete signal to the backup manager.
    async fn shutdown<T: Future<Output = Result<()>> + Send + 'static>(
        &mut self,
        mut stream: Option<Pin<Box<T>>>,
        first_message_id: Arc<AtomicU64>,
        last_message_id: Arc<AtomicU64>,
    ) -> Result<()> {
        #[cfg(feature = "tracing")]
        tracing::info!(
            sift_stream_id = %self.config.sift_stream_id,
            "ingestion task shutting down"
        );

        // During shutdown the data channel is closed, so to let the stream finish sending all data we need to await the stream
        // one last time before exiting.
        if let Some(stream) = stream.as_mut() {
            match stream.await {
                Ok(_) => {
                    #[cfg(feature = "tracing")]
                    tracing::info!(
                        sift_stream_id = %self.config.sift_stream_id,
                        "final stream completed successfully"
                    );
                }
                Err(e) => {
                    #[cfg(feature = "tracing")]
                    tracing::error!(
                        sift_stream_id = %self.config.sift_stream_id,
                        error = %e,
                        "final stream failed"
                    );
                    self.control_tx
                        .send(ControlMessage::CheckpointNeedsReingestion {
                            first_message_id: first_message_id.load(Ordering::Relaxed),
                            last_message_id: last_message_id.load(Ordering::Relaxed),
                        })
                        .map_err(|e| Error::new(ErrorKind::StreamError, e))?;
                }
            }
        }

        // Send the final checkpoint complete signal to the backup manager.
        self.control_tx
            .send(ControlMessage::CheckpointComplete {
                first_message_id: first_message_id.load(Ordering::Relaxed),
                last_message_id: last_message_id.load(Ordering::Relaxed),
            })
            .map_err(|e| Error::new(ErrorKind::StreamError, e))?;

        Ok(())
    }
}

/// The asset to stream metrics for.
const METRICS_STREAMING_INGESTION_CONFIG_ASSET_NAME: &str = "sift_app";

/// The client key used for sift_stream metrics ingestion config.
const METRICS_STREAMING_INGESTION_CONFIG_CLIENT_KEY: &str = "sift-stream-metrics";

/// The flow name used for sift_stream metrics flow config.
const METRICS_STREAMING_FLOW_NAME: &str = "sift-stream-metrics-flow";

pub(crate) struct MetricsStreamingTask {
    stream: SiftStream<IngestionConfigEncoder>,
    control_rx: broadcast::Receiver<ControlMessage>,
    session_name: String,
    interval: Duration,
    metrics: Arc<SiftStreamMetrics>,
}

impl MetricsStreamingTask {
    pub(crate) async fn new(
        control_rx: broadcast::Receiver<ControlMessage>,
        interval: Duration,
        config: TaskConfig,
    ) -> Result<Self> {
        use std::hash::{Hash, Hasher};
        let session_name = config.session_name;

        let channels = SiftStreamMetricsSnapshot::channel_configs(&session_name);

        // Hash the channel names to create a unique client key for the ingestion config.
        //
        // Given the same "session_name", which influences the channel names, and the same metrics configuration,
        // the ingestion config client key should be the same and re-used.
        let mut hasher = std::hash::DefaultHasher::new();
        channels.iter().for_each(|channel| {
            channel.name.hash(&mut hasher);
        });
        let hash_key = hasher.finish();

        let client_key = format!(
            "{}-{}",
            METRICS_STREAMING_INGESTION_CONFIG_CLIENT_KEY, hash_key
        );

        let ingestion_config = IngestionConfigForm {
            asset_name: METRICS_STREAMING_INGESTION_CONFIG_ASSET_NAME.to_string(),
            client_key,
            flows: vec![FlowConfig {
                name: METRICS_STREAMING_FLOW_NAME.to_string(),
                channels: SiftStreamMetricsSnapshot::channel_configs(&session_name),
            }],
        };

        // Build a new [`SiftStream`] that is responsible for streaming metrics to Sift.
        //
        // Most builder parameters are carried over from the main stream being monitored, however,
        // the differences are noted below:
        //
        // - Channel capacities are substantially lower since this stream deals with less throughput.
        // - Metrics streaming interval is set to `None` to disable streaming.
        // - The `setup-channel` is used for all gRPC channels in this stream since they are less
        //   critical and thus can be multiplexed over a single connection.
        //
        // NOTE: The build future is boxed/pinned due to async recursion -- generally a sift-stream
        // instance will be spawning a second sift-stream instance for streaming it's own metrics though
        // the limit of recursion here is 2 since the metrics-streaming sift-stream doesn't itself
        // spawn another sift-stream instance. Since this is only done during initialization, it is fine.
        let stream_fut = Box::pin(
            SiftStreamBuilder::from_channel(config.setup_channel.clone())
                .metrics_streaming_interval(None)
                .ingestion_config(ingestion_config)
                .control_channel_capacity(100)
                .ingestion_data_channel_capacity(1000)
                .backup_data_channel_capacity(1000)
                .recovery_strategy(RecoveryStrategy::RetryWithBackups {
                    retry_policy: config.recovery_config.retry_policy,
                    disk_backup_policy: config.recovery_config.backup_policy,
                })
                .build(),
        );

        let stream = stream_fut.await?;

        Ok(Self {
            stream,
            control_rx,
            session_name,
            interval,
            metrics: config.metrics.clone(),
        })
    }

    pub(crate) async fn run(mut self) -> Result<()> {
        let mut interval = tokio::time::interval(self.interval);

        loop {
            select! {
                _ = interval.tick() => {
                    let metrics = self.metrics.snapshot();
                    let values = metrics.channel_values(&self.session_name);
                    let flow = Flow::new(METRICS_STREAMING_FLOW_NAME, TimeValue::now(), &values);
                    self.stream.send(flow).await?;
                }
                ctrl_msg = self.control_rx.recv() => {
                    match ctrl_msg {
                        Ok(ControlMessage::Shutdown) => {
                            break;
                        }
                        _ => continue,
                    }
                }
            }
        }

        #[cfg(feature = "tracing")]
        tracing::info!("metrics streaming task shutting down");

        self.stream
            .finish()
            .await
            .map_err(|e| Error::new(ErrorKind::StreamError, e))
    }
}

#[cfg(test)]
mod tests {
    use sift_rs::ingest::v1::{
        IngestWithConfigDataChannelValue, ingest_with_config_data_channel_value::Type,
    };

    use crate::TimeValue;

    use super::*;

    async fn send_messages_for_ingestion(
        data_tx: &async_channel::Sender<DataMessage>,
        count: usize,
    ) {
        for i in 0..count {
            let request = IngestWithConfigDataStreamRequest {
                ingestion_config_id: "test-0".to_string(),
                flow: "some_flow".to_string(),
                timestamp: Some(*TimeValue::now()),
                channel_values: vec![IngestWithConfigDataChannelValue {
                    r#type: Some(Type::Int32(i as i32)),
                }],
                run_id: "test-run-id".to_string(),
                end_stream_on_validation_error: false,
                organization_id: "test-organization-id".to_string(),
            };
            assert!(
                data_tx
                    .try_send(DataMessage {
                        message_id: i as u64,
                        request: Arc::new(request),
                        dropped_for_ingestion: false
                    })
                    .is_ok(),
                "failed to send data message to ingestion task"
            );
        }

        for _ in 0..5 {
            if data_tx.is_empty() {
                break;
            }
            tokio::time::sleep(Duration::from_millis(100)).await;
        }

        println!("data tx len: {}", data_tx.len());
    }

    #[tokio::test]
    async fn test_ingestion_task_shutdown() {
        let (ingestion_channel, _mock_service) =
            crate::test::create_mock_grpc_channel_with_service().await;
        let reingestion_channel = ingestion_channel.clone();
        let setup_channel = ingestion_channel.clone();
        let (control_tx, mut control_rx) = broadcast::channel(1024);
        let (data_tx, data_rx) = async_channel::bounded(1024);
        let metrics = Arc::new(SiftStreamMetrics::default());
        let checkpoint_interval = Duration::from_secs(60);
        let config = TaskConfig {
            session_name: "test-session".to_string(),
            sift_stream_id: Uuid::new_v4(),
            setup_channel,
            ingestion_channel,
            reingestion_channel,
            metrics: metrics.clone(),
            checkpoint_interval,
            enable_compression_for_ingestion: false,
            control_channel_capacity: 128,
            ingestion_data_channel_capacity: 128,
            backup_data_channel_capacity: 128,
            metrics_streaming_interval: None,
            recovery_config: RecoveryConfig {
                retry_policy: RetryPolicy::default(),
                backups_enabled: true,
                backups_directory: "backup_directory".to_string(),
                backups_prefix: "prefix".to_string(),
                backup_policy: DiskBackupPolicy::default(),
            },
        };

        let mut ingestion_task = IngestionTask::new(control_tx.clone(), data_rx, config);

        // Wait for the ingestion task to drain the data channel.
        let handle = tokio::spawn(async move { ingestion_task.run().await });

        // Send some messages for ingestion.
        send_messages_for_ingestion(&data_tx, 100).await;

        // Close the data channel and send the shutdown message.
        data_tx.close();
        assert!(
            control_tx.send(ControlMessage::Shutdown).is_ok(),
            "failed to send shutdown message to ingestion task"
        );

        // Wait for the ingestion task to complete.
        assert!(
            handle.await.is_ok(),
            "ingestion task should complete successfully"
        );

        // Verify graceful shutdown drained the data channel and sent the final checkpoint complete message.
        assert!(data_tx.is_empty(), "data channel should be empty");

        // Each checkpoint expiration should generate a checkpoint complete control message.
        let mut complete_count = 0;
        while let Ok(msg) = control_rx.try_recv() {
            if matches!(
                msg,
                ControlMessage::CheckpointComplete {
                    first_message_id: _,
                    last_message_id: _
                }
            ) {
                complete_count += 1;
            }
        }
        assert_eq!(complete_count, 1, "should have completed 1 checkpoint");
    }

    #[tokio::test]
    async fn test_ingestion_task_shutdown_ungracefully() {
        let (ingestion_channel, _mock_service) =
            crate::test::create_mock_grpc_channel_with_service().await;
        let reingestion_channel = ingestion_channel.clone();
        let setup_channel = ingestion_channel.clone();
        let (control_tx, mut control_rx) = broadcast::channel(1024);
        let (data_tx, data_rx) = async_channel::bounded(1024);
        let metrics = Arc::new(SiftStreamMetrics::default());
        let checkpoint_interval = Duration::from_secs(60);
        let config = TaskConfig {
            session_name: "test-session".to_string(),
            sift_stream_id: Uuid::new_v4(),
            setup_channel,
            ingestion_channel,
            reingestion_channel,
            metrics: metrics.clone(),
            checkpoint_interval,
            enable_compression_for_ingestion: false,
            control_channel_capacity: 128,
            ingestion_data_channel_capacity: 128,
            backup_data_channel_capacity: 128,
            metrics_streaming_interval: None,
            recovery_config: RecoveryConfig {
                retry_policy: RetryPolicy::default(),
                backups_enabled: true,
                backups_directory: "backup_directory".to_string(),
                backups_prefix: "prefix".to_string(),
                backup_policy: DiskBackupPolicy::default(),
            },
        };

        let mut ingestion_task = IngestionTask::new(control_tx.clone(), data_rx, config);

        // Wait for the ingestion task to drain the data channel.
        let handle = tokio::spawn(async move { ingestion_task.run().await });

        // Send some messages for ingestion.
        send_messages_for_ingestion(&data_tx, 100).await;

        // Close the data channel to trigger the shutdown process.
        data_tx.close();

        // Wait for the ingestion task to complete.
        let res = tokio::time::timeout(Duration::from_secs(10), handle).await;
        assert!(res.is_ok(), "ingestion task should complete successfully");

        // Verify graceful shutdown drained the data channel and sent the final checkpoint complete message.
        assert!(data_tx.is_empty(), "data channel should be empty");

        // Each checkpoint expiration should generate a checkpoint complete control message.
        let mut complete_count = 0;
        while let Ok(msg) = control_rx.try_recv() {
            if matches!(
                msg,
                ControlMessage::CheckpointComplete {
                    first_message_id: _,
                    last_message_id: _
                }
            ) {
                complete_count += 1;
            }
        }
        assert_eq!(complete_count, 1, "should have completed 1 checkpoint");
    }

    #[tokio::test]
    async fn test_ingestion_task_shutdown_errors() {
        let (ingestion_channel, mock_service) =
            crate::test::create_mock_grpc_channel_with_service().await;
        let reingestion_channel = ingestion_channel.clone();
        let setup_channel = ingestion_channel.clone();
        let (control_tx, mut control_rx) = broadcast::channel(1024);
        let (data_tx, data_rx) = async_channel::bounded(1024);
        let metrics = Arc::new(SiftStreamMetrics::default());
        let checkpoint_interval = Duration::from_secs(60);
        let config = TaskConfig {
            session_name: "test-session".to_string(),
            sift_stream_id: Uuid::new_v4(),
            setup_channel,
            ingestion_channel,
            reingestion_channel,
            metrics: metrics.clone(),
            checkpoint_interval,
            enable_compression_for_ingestion: false,
            control_channel_capacity: 128,
            ingestion_data_channel_capacity: 128,
            backup_data_channel_capacity: 128,
            metrics_streaming_interval: None,
            recovery_config: RecoveryConfig {
                retry_policy: RetryPolicy::default(),
                backups_enabled: true,
                backups_directory: "backup_directory".to_string(),
                backups_prefix: "prefix".to_string(),
                backup_policy: DiskBackupPolicy::default(),
            },
        };

        let mut ingestion_task = IngestionTask::new(control_tx.clone(), data_rx, config);

        // Set the mock service to return errors.
        mock_service.set_num_errors_to_return(2);

        // Wait for the ingestion task to drain the data channel.
        let handle = tokio::spawn(async move { ingestion_task.run().await });

        // Send some messages for ingestion.
        let send = async {
            send_messages_for_ingestion(&data_tx, 100).await;

            // Close the data channel and send the shutdown message.
            data_tx.close();
            assert!(
                control_tx.send(ControlMessage::Shutdown).is_ok(),
                "failed to send shutdown message to ingestion task"
            );
        };

        let (_, handle_result) = tokio::join!(send, handle);
        assert!(
            handle_result.is_ok(),
            "ingestion task should complete successfully"
        );

        // Verify graceful shutdown drained the data channel and sent the final checkpoint complete message.
        assert!(data_tx.is_empty(), "data channel should be empty");

        // Each checkpoint expiration should generate a checkpoint complete control message.
        let mut complete_count = 0;
        while let Ok(msg) = control_rx.try_recv() {
            if matches!(
                msg,
                ControlMessage::CheckpointComplete {
                    first_message_id: _,
                    last_message_id: _
                }
            ) {
                complete_count += 1;
            }
        }
        assert_eq!(complete_count, 1, "should have completed 1 checkpoint");
    }

    #[tokio::test]
    async fn test_ingestion_task_stream() {
        let (ingestion_channel, mock_service) =
            crate::test::create_mock_grpc_channel_with_service().await;
        let reingestion_channel = ingestion_channel.clone();
        let setup_channel = ingestion_channel.clone();
        let (control_tx, _control_rx) = broadcast::channel(1024);
        let (data_tx, data_rx) = async_channel::bounded(1024);
        let metrics = Arc::new(SiftStreamMetrics::default());
        let config = TaskConfig {
            session_name: "test-session".to_string(),
            sift_stream_id: Uuid::new_v4(),
            setup_channel,
            ingestion_channel,
            reingestion_channel,
            metrics: metrics.clone(),
            checkpoint_interval: Duration::from_secs(60),
            enable_compression_for_ingestion: false,
            control_channel_capacity: 128,
            ingestion_data_channel_capacity: 128,
            backup_data_channel_capacity: 128,
            metrics_streaming_interval: None,
            recovery_config: RecoveryConfig {
                retry_policy: RetryPolicy::default(),
                backups_enabled: true,
                backups_directory: "backup_directory".to_string(),
                backups_prefix: "prefix".to_string(),
                backup_policy: DiskBackupPolicy::default(),
            },
        };

        let mut ingestion_task = IngestionTask::new(control_tx.clone(), data_rx, config);
        let handle = tokio::spawn(async move { ingestion_task.run().await });

        // Send some messages for ingestion.
        send_messages_for_ingestion(&data_tx, 10).await;

        // Close the data channel and send the shutdown message.
        data_tx.close();
        assert!(
            control_tx.send(ControlMessage::Shutdown).is_ok(),
            "failed to send shutdown message to ingestion task"
        );

        // Wait for the ingestion task to complete.
        assert!(
            handle.await.is_ok(),
            "ingestion task should complete successfully"
        );

        // Verify the messages were captured.
        let captured = mock_service.get_captured_data();
        assert_eq!(captured.len(), 10, "should have captured 10 messages");
        for (i, message) in captured.iter().enumerate() {
            assert_eq!(
                message.ingestion_config_id, "test-0",
                "ingestion config id should be test-0"
            );
            assert_eq!(message.flow, "some_flow", "flow should be some_flow");
            assert_eq!(
                message.channel_values.len(),
                1,
                "should have one channel value"
            );
            assert_eq!(
                message.channel_values[0].r#type,
                Some(Type::Int32(i as i32)),
                "channel value should be int32({i})"
            );
        }

        // Verify the metrics.
        assert_eq!(
            metrics.messages_sent.get(),
            10,
            "should have sent 10 messages"
        );
        assert!(
            metrics.bytes_sent.get() >= 10 * 70,
            "should have sent at least 10 * 70 bytes"
        );
    }

    #[tokio::test]
    async fn test_ingestion_task_stream_retries() {
        let (ingestion_channel, mock_service) =
            crate::test::create_mock_grpc_channel_with_service().await;
        let reingestion_channel = ingestion_channel.clone();
        let setup_channel = ingestion_channel.clone();
        let (control_tx, mut control_rx) = broadcast::channel(1024);
        let (data_tx, data_rx) = async_channel::bounded(1024);
        let metrics = Arc::new(SiftStreamMetrics::default());
        let checkpoint_interval = Duration::from_millis(100);
        let config = TaskConfig {
            session_name: "test-session".to_string(),
            sift_stream_id: Uuid::new_v4(),
            setup_channel,
            ingestion_channel,
            reingestion_channel,
            metrics: metrics.clone(),
            checkpoint_interval,
            enable_compression_for_ingestion: false,
            control_channel_capacity: 128,
            ingestion_data_channel_capacity: 128,
            backup_data_channel_capacity: 128,
            metrics_streaming_interval: None,
            recovery_config: RecoveryConfig {
                retry_policy: RetryPolicy {
                    max_attempts: 3,
                    initial_backoff: Duration::from_millis(1),
                    max_backoff: Duration::from_millis(100),
                    backoff_multiplier: 5,
                },
                backups_enabled: true,
                backups_directory: "backup_directory".to_string(),
                backups_prefix: "prefix".to_string(),
                backup_policy: DiskBackupPolicy::default(),
            },
        };

        // Ingestion is continuously retried, limited by the max retry duration only.
        mock_service.set_num_errors_to_return(
            config.recovery_config.retry_policy.max_attempts as usize + 1,
        );

        let mut ingestion_task = IngestionTask::new(control_tx.clone(), data_rx, config);

        // Wait for the ingestion task to drain the data channel.
        let handle = tokio::spawn(async move { ingestion_task.run().await });

        // Send some messages for ingestion.
        send_messages_for_ingestion(&data_tx, 10).await;
        tokio::time::sleep(checkpoint_interval).await;

        // Close the data channel and send the shutdown message.
        data_tx.close();
        assert!(
            control_tx.send(ControlMessage::Shutdown).is_ok(),
            "failed to send shutdown message to ingestion task"
        );

        // Wait for the ingestion task to complete.
        let res = tokio::time::timeout(Duration::from_secs(10), handle).await;
        assert!(res.is_ok(), "ingestion task should complete successfully");

        // Verify the metrics.
        assert_eq!(
            metrics.messages_sent.get(),
            10,
            "should have sent 10 messages"
        );
        assert!(
            metrics.bytes_sent.get() >= 10 * 70,
            "should have sent at least 10 * 70 bytes"
        );
        assert_eq!(
            metrics.checkpoint.failed_checkpoint_count.get(),
            4,
            "should have failed the checkpoint 4 times"
        );

        // Each gRPC call failure should trigger a checkpoint reingestion control message.
        let mut needs_reingestion_count = 0;
        while let Ok(msg) = control_rx.try_recv() {
            if matches!(
                msg,
                ControlMessage::CheckpointNeedsReingestion {
                    first_message_id: _,
                    last_message_id: _
                }
            ) {
                needs_reingestion_count += 1;
            }
        }
        assert_eq!(
            needs_reingestion_count, 4,
            "should have received 4 checkpoint needs reingestion messages"
        );
    }

    #[tokio::test]
    async fn test_ingestion_task_checkpoints() {
        let (ingestion_channel, _mock_service) =
            crate::test::create_mock_grpc_channel_with_service().await;
        let reingestion_channel = ingestion_channel.clone();
        let setup_channel = ingestion_channel.clone();
        let (control_tx, mut control_rx) = broadcast::channel(1024);
        let (data_tx, data_rx) = async_channel::bounded(1024);
        let metrics = Arc::new(SiftStreamMetrics::default());
        let checkpoint_interval = Duration::from_millis(100);
        let config = TaskConfig {
            session_name: "test-session".to_string(),
            sift_stream_id: Uuid::new_v4(),
            setup_channel,
            ingestion_channel,
            reingestion_channel,
            metrics: metrics.clone(),
            checkpoint_interval,
            enable_compression_for_ingestion: false,
            control_channel_capacity: 128,
            ingestion_data_channel_capacity: 128,
            backup_data_channel_capacity: 128,
            metrics_streaming_interval: None,
            recovery_config: RecoveryConfig {
                retry_policy: RetryPolicy::default(),
                backups_enabled: true,
                backups_directory: "backup_directory".to_string(),
                backups_prefix: "prefix".to_string(),
                backup_policy: DiskBackupPolicy::default(),
            },
        };

        let mut ingestion_task = IngestionTask::new(control_tx.clone(), data_rx, config);

        // Wait for the ingestion task to drain the data channel.
        let handle = tokio::spawn(async move { ingestion_task.run().await });

        // Send some messages for ingestion.
        send_messages_for_ingestion(&data_tx, 100).await;

        // Ensure we have waited a few checkpoint durations.
        tokio::time::sleep(checkpoint_interval * 3).await;

        // Close the data channel and send the shutdown message.
        data_tx.close();
        assert!(
            control_tx.send(ControlMessage::Shutdown).is_ok(),
            "failed to send shutdown message to ingestion task"
        );

        // Wait for the ingestion task to complete.
        assert!(
            handle.await.is_ok(),
            "ingestion task should complete successfully"
        );

        assert!(
            metrics.checkpoint.checkpoint_timer_reached_cnt.get() >= 3,
            "should have reached the checkpoint timer at least 3 times"
        );
        assert!(
            metrics.checkpoint.checkpoint_count.get() >= 3,
            "should have completed at least 3 checkpoints"
        );

        // Each checkpoint expiration should generate a checkpoint complete control message.
        let mut complete_count = 0;
        while let Ok(msg) = control_rx.try_recv() {
            if matches!(
                msg,
                ControlMessage::CheckpointComplete {
                    first_message_id: _,
                    last_message_id: _
                }
            ) {
                complete_count += 1;
            }
        }
        assert!(
            complete_count >= 3,
            "should have completed at least 3 checkpoints"
        );
    }

    #[tokio::test]
    async fn test_ingestion_task_backup_full() {
        let (ingestion_channel, _mock_service) =
            crate::test::create_mock_grpc_channel_with_service().await;
        let reingestion_channel = ingestion_channel.clone();
        let setup_channel = ingestion_channel.clone();
        let (control_tx, mut control_rx) = broadcast::channel(1024);
        let (data_tx, data_rx) = async_channel::bounded(1024);
        let metrics = Arc::new(SiftStreamMetrics::default());
        let checkpoint_interval = Duration::from_secs(60);
        let config = TaskConfig {
            session_name: "test-session".to_string(),
            sift_stream_id: Uuid::new_v4(),
            setup_channel,
            ingestion_channel,
            reingestion_channel,
            metrics: metrics.clone(),
            checkpoint_interval,
            enable_compression_for_ingestion: false,
            control_channel_capacity: 128,
            ingestion_data_channel_capacity: 128,
            backup_data_channel_capacity: 128,
            metrics_streaming_interval: None,
            recovery_config: RecoveryConfig {
                retry_policy: RetryPolicy::default(),
                backups_enabled: true,
                backups_directory: "backup_directory".to_string(),
                backups_prefix: "prefix".to_string(),
                backup_policy: DiskBackupPolicy::default(),
            },
        };

        let mut ingestion_task = IngestionTask::new(control_tx.clone(), data_rx, config);

        // Wait for the ingestion task to drain the data channel.
        let handle = tokio::spawn(async move { ingestion_task.run().await });

        // Send some messages for ingestion.
        send_messages_for_ingestion(&data_tx, 100).await;

        // Send the backup full message.
        assert!(
            control_tx.send(ControlMessage::BackupFull).is_ok(),
            "failed to send backup full message to ingestion task"
        );

        // Send some messages for ingestion.
        send_messages_for_ingestion(&data_tx, 100).await;

        // Close the data channel and send the shutdown message.
        data_tx.close();
        assert!(
            control_tx.send(ControlMessage::Shutdown).is_ok(),
            "failed to send shutdown message to ingestion task"
        );

        // Wait for the ingestion task to complete.
        assert!(
            handle.await.is_ok(),
            "ingestion task should complete successfully"
        );

        assert_eq!(
            metrics.checkpoint.checkpoint_manually_reached_cnt.get(),
            1,
            "should have reached the checkpoint manually 1 time"
        );
        assert!(
            metrics.checkpoint.checkpoint_count.get() >= 1,
            "should have completed at least 1 checkpoint"
        );

        // Each checkpoint expiration should generate a checkpoint complete control message.
        let mut complete_count = 0;
        while let Ok(msg) = control_rx.try_recv() {
            if matches!(
                msg,
                ControlMessage::CheckpointComplete {
                    first_message_id: _,
                    last_message_id: _
                }
            ) {
                complete_count += 1;
            }
        }
        assert!(
            complete_count >= 2,
            "should have completed at least 2 checkpoints (1 for the final checkpoint)"
        );
    }
}
