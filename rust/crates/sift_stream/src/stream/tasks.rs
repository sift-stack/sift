use crate::{
    DiskBackupPolicy, RetryPolicy,
    backup::disk::{AsyncBackupsManager, BackupIngestTask},
    metrics::SiftStreamMetrics,
    stream::mode::ingestion_config::DataStream,
};
use async_channel;
use sift_connect::SiftChannel;
use sift_error::prelude::*;
use sift_rs::ingest::v1::{
    IngestWithConfigDataStreamRequest, ingest_service_client::IngestServiceClient,
};
use std::{path::PathBuf, sync::Arc, time::Duration};
use tokio::{sync::broadcast, task::JoinHandle};
use uuid::Uuid;

/// Capacity for the data channel.
pub(crate) const DATA_CHANNEL_CAPACITY: usize = 1024 * 10;

/// Capacity for the control channel.
pub(crate) const CONTROL_CHANNEL_CAPACITY: usize = 1024;

/// Control messages sent between tasks via broadcast channel
/// These are low-frequency control messages, not high-volume data messages
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ControlMessage {
    /// Signal that the backup is full and a new checkpoint should be started.
    BackupFull,

    /// Request to re-ingest backup files
    ReingestBackups { backup_files: Vec<PathBuf> },

    /// Signal to complete the checkpoint.
    CheckpointComplete,

    /// Signal the checkpoint needs re-ingestion.
    CheckpointNeedsReingestion,

    /// Error signal
    ErrorSignal { error: String },

    /// Shutdown signal for all tasks
    Shutdown,
}

#[derive(Clone)]
pub struct RecoveryConfig {
    pub retry_policy: RetryPolicy,
    pub backups_enabled: bool,
    pub backups_directory: String,
    pub backups_prefix: String,
    pub backup_policy: DiskBackupPolicy,
}

/// Configuration for the task-based SiftStream
#[derive(Clone)]
pub struct TaskConfig {
    pub sift_stream_id: Uuid,
    pub grpc_channel: SiftChannel,
    pub metrics: Arc<SiftStreamMetrics>,
    pub checkpoint_interval: Duration,
    pub recovery_config: RecoveryConfig,
    pub control_channel_capacity: usize,
    pub data_channel_capacity: usize,
}

/// Data message with stream ID for routing
#[derive(Debug, Clone)]
pub struct DataMessage {
    pub request: IngestWithConfigDataStreamRequest,
    pub dropped_for_ingestion: bool,
}

/// Handles for the three main tasks
pub struct StreamSystem {
    pub backup_manager: JoinHandle<Result<()>>,
    pub ingestion: JoinHandle<Result<()>>,
    pub reingestion: JoinHandle<Result<()>>,
    pub control_tx: broadcast::Sender<ControlMessage>,
    pub ingestion_tx: async_channel::Sender<DataMessage>,
    pub backup_tx: async_channel::Sender<DataMessage>,
}

/// Creates and starts all three tasks
pub(crate) fn start_tasks(config: TaskConfig) -> Result<StreamSystem> {
    // Create broadcast channel for control messages (low frequency)
    let (control_tx, _control_rx) = broadcast::channel(config.control_channel_capacity);

    // Create data channel for high-frequency data messages
    let (ingestion_tx, ingestion_rx) = async_channel::bounded(config.data_channel_capacity);
    let (backup_tx, backup_rx) = async_channel::bounded(config.data_channel_capacity);

    // Clone the sender for each task
    let backup_control_tx = control_tx.clone();
    let ingestion_control_tx = control_tx.clone();
    let reingestion_control_tx = control_tx.clone();

    // Start backup manager task
    let backup_config = config.clone();
    let backup_control_rx = backup_control_tx.subscribe();
    let backup_data_rx = backup_rx.clone();
    let backup_manager = tokio::spawn(async move {
        #[cfg(feature = "tracing")]
        tracing::info!(
            sift_stream_id = config.sift_stream_id.to_string(),
            "backup manager task started"
        );

        AsyncBackupsManager::new(
            backup_config.recovery_config.backups_enabled,
            &backup_config.recovery_config.backups_directory,
            &backup_config.recovery_config.backups_prefix,
            backup_config.recovery_config.backup_policy,
            backup_control_tx,
            backup_control_rx,
            backup_data_rx,
            backup_config.metrics.clone(),
        )
        .await?
        .run()
        .await
    });

    // Start gRPC client task
    let ingestion_config = config.clone();
    let ingestion_data_rx = ingestion_rx.clone();
    let mut ingestion_task =
        IngestionTask::new(ingestion_control_tx, ingestion_data_rx, ingestion_config);
    let ingestion = tokio::spawn(async move {
        #[cfg(feature = "tracing")]
        tracing::info!(
            sift_stream_id = config.sift_stream_id.to_string(),
            "ingestion task started"
        );
        ingestion_task.run().await
    });

    // Start backup re-ingestion task
    let reingestion_config = config.clone();
    let reingestion_task = BackupIngestTask::new(
        reingestion_control_tx.subscribe(),
        reingestion_config.grpc_channel,
        reingestion_config.recovery_config.retry_policy,
        reingestion_config
            .recovery_config
            .backup_policy
            .retain_backups,
        reingestion_config.metrics.clone(),
    );
    let reingestion = tokio::spawn(async move {
        #[cfg(feature = "tracing")]
        tracing::info!(
            sift_stream_id = config.sift_stream_id.to_string(),
            "backup re-ingestion task started"
        );
        reingestion_task.run().await
    });

    #[cfg(feature = "tracing")]
    tracing::info!(
        sift_stream_id = config.sift_stream_id.to_string(),
        "Sift streaming successfully initialized"
    );

    Ok(StreamSystem {
        backup_manager,
        ingestion,
        reingestion,
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
        let mut timer = tokio::time::interval_at(
            tokio::time::Instant::now() + self.config.checkpoint_interval,
            self.config.checkpoint_interval,
        );

        let mut current_wait = Duration::ZERO;

        // The stream needs to be kept alive independently from receiving control messages in the
        // loop below, so an [`Option`] is used to store the stream future and updated as needed.
        let mut stream = None;

        loop {
            // Create a new stream if one doesn't exist yet.
            if stream.is_none() {
                #[cfg(feature = "tracing")]
                tracing::info!(
                    sift_stream_id = self.config.sift_stream_id.to_string(),
                    "creating new stream"
                );

                stream = Some(Box::pin(async {
                    let mut client = IngestServiceClient::new(self.config.grpc_channel.clone());

                    let data_stream = DataStream::new(
                        self.data_rx.clone(),
                        self.control_tx.clone(),
                        self.config.sift_stream_id,
                        self.config.metrics.clone(),
                    );
                    client.ingest_with_config_data_stream(data_stream).await
                }));

                #[cfg(feature = "tracing")]
                tracing::info!(
                    sift_stream_id = self.config.sift_stream_id.to_string(),
                    "successfully initialized a new stream to Sift"
                );
            }

            // Wait for the stream to complete or for a control message to be received.
            tokio::select! {
                res = stream.as_mut().unwrap() => {
                    match res {
                        Ok(_) => {
                            #[cfg(feature = "tracing")]
                            tracing::info!(
                                sift_stream_id = self.config.sift_stream_id.to_string(),
                                "checkpoint succeeded - data streamed to Sift successfully"
                            );

                            stream = None;
                            current_wait = Duration::ZERO;
                            self.config.metrics.cur_retry_count.set(0);
                        }
                        Err(e) => {
                            #[cfg(feature = "tracing")]
                            tracing::error!(
                                sift_stream_id = self.config.sift_stream_id.to_string(),
                                retry_counter = self.config.metrics.cur_retry_count.get(),
                                error = %e,
                                "checkpoint failed - failed to ingest data to Sift - backup files will be re-ingested"
                            );

                            stream = None;
                            current_wait = self.config.recovery_config.retry_policy.backoff(current_wait);
                            self.config.metrics.cur_retry_count.add(1);
                            self.config.metrics.checkpoint.failed_checkpoint_count.increment();
                            self.control_tx.send(ControlMessage::CheckpointNeedsReingestion).map_err(|e| Error::new(ErrorKind::StreamError, e))?;
                        }
                    }
                }
                _ = timer.tick() => {
                    #[cfg(feature = "tracing")]
                    tracing::info!(
                        sift_stream_id = self.config.sift_stream_id.to_string(),
                        "checkpoint expired"
                    );

                    stream = None;
                    self.config.metrics.checkpoint.checkpoint_timer_reached_cnt.increment();
                    self.control_tx.send(ControlMessage::CheckpointComplete).map_err(|e| Error::new(ErrorKind::StreamError, e))?;
                }
                ctrl_msg = self.control_rx.recv() => {
                    match ctrl_msg {
                        Ok(ControlMessage::BackupFull) => {
                            #[cfg(feature = "tracing")]
                            tracing::info!(
                                sift_stream_id = self.config.sift_stream_id.to_string(),
                                "backup full"
                            );

                            // Since the backup files are full, we need to flush them now.
                            // Reset the timer to start a new checkpoint interval starting `now`.
                            timer.reset();

                            stream = None;
                            self.config.metrics.checkpoint.checkpoint_manually_reached_cnt.increment();
                            self.control_tx.send(ControlMessage::CheckpointComplete).map_err(|e| Error::new(ErrorKind::StreamError, e))?;
                        }
                        Ok(ControlMessage::Shutdown) => {
                            #[cfg(feature = "tracing")]
                            tracing::info!(
                                sift_stream_id = self.config.sift_stream_id.to_string(),
                                "ingestion task shutting down"
                            );

                            // During shutdown the data channel is closed, so to let the stream finish sending all data we need to await the stream
                            // one last time before exiting.
                            if let Some(stream) = stream.as_mut() {
                                match stream.await {
                                    Ok(_) => {
                                        #[cfg(feature = "tracing")]
                                        tracing::info!(
                                            sift_stream_id = self.config.sift_stream_id.to_string(),
                                            "final stream completed successfully"
                                        );
                                    }
                                    Err(e) => {
                                        #[cfg(feature = "tracing")]
                                        tracing::error!(
                                            sift_stream_id = self.config.sift_stream_id.to_string(),
                                            error = %e,
                                            "final stream failed"
                                        );
                                    }
                                }
                            }

                            // Send the final checkpoint complete signal to the backup manager.
                            self.control_tx.send(ControlMessage::CheckpointComplete).map_err(|e| Error::new(ErrorKind::StreamError, e))?;

                            break;
                        }
                        _ => continue,
                    }
                }
            }
        }

        Ok(())
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
                        request,
                        dropped_for_ingestion: false
                    })
                    .is_ok(),
                "failed to send data message to ingestion task"
            );
        }

        while !data_tx.is_empty() {
            tokio::time::sleep(Duration::from_millis(100)).await;
        }
    }

    #[tokio::test]
    async fn test_ingestion_task_shutdown() {
        let (grpc_channel, _mock_service) =
            crate::test::create_mock_grpc_channel_with_service().await;
        let (control_tx, mut control_rx) = broadcast::channel(1024);
        let (data_tx, data_rx) = async_channel::bounded(1024);
        let metrics = Arc::new(SiftStreamMetrics::default());
        let checkpoint_interval = Duration::from_secs(60);
        let config = TaskConfig {
            sift_stream_id: Uuid::new_v4(),
            grpc_channel,
            metrics: metrics.clone(),
            checkpoint_interval,
            control_channel_capacity: 128,
            data_channel_capacity: 128,
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
            if msg == ControlMessage::CheckpointComplete {
                complete_count += 1;
            }
        }
        assert_eq!(complete_count, 1, "should have completed 1 checkpoint");
    }

    #[tokio::test]
    async fn test_ingestion_task_shutdown_errors() {
        let (grpc_channel, mock_service) =
            crate::test::create_mock_grpc_channel_with_service().await;
        let (control_tx, mut control_rx) = broadcast::channel(1024);
        let (data_tx, data_rx) = async_channel::bounded(1024);
        let metrics = Arc::new(SiftStreamMetrics::default());
        let checkpoint_interval = Duration::from_secs(60);
        let config = TaskConfig {
            sift_stream_id: Uuid::new_v4(),
            grpc_channel,
            metrics: metrics.clone(),
            checkpoint_interval,
            control_channel_capacity: 128,
            data_channel_capacity: 128,
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
            if msg == ControlMessage::CheckpointComplete {
                complete_count += 1;
            }
        }
        assert_eq!(complete_count, 1, "should have completed 1 checkpoint");
    }

    #[tokio::test]
    async fn test_ingestion_task_stream() {
        let (grpc_channel, mock_service) =
            crate::test::create_mock_grpc_channel_with_service().await;
        let (control_tx, _control_rx) = broadcast::channel(1024);
        let (data_tx, data_rx) = async_channel::bounded(1024);
        let metrics = Arc::new(SiftStreamMetrics::default());
        let config = TaskConfig {
            sift_stream_id: Uuid::new_v4(),
            grpc_channel,
            metrics: metrics.clone(),
            checkpoint_interval: Duration::from_secs(60),
            control_channel_capacity: 128,
            data_channel_capacity: 128,
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
        let (grpc_channel, mock_service) =
            crate::test::create_mock_grpc_channel_with_service().await;
        let (control_tx, mut control_rx) = broadcast::channel(1024);
        let (data_tx, data_rx) = async_channel::bounded(1024);
        let metrics = Arc::new(SiftStreamMetrics::default());
        let config = TaskConfig {
            sift_stream_id: Uuid::new_v4(),
            grpc_channel,
            metrics: metrics.clone(),
            checkpoint_interval: Duration::from_secs(60),
            control_channel_capacity: 128,
            data_channel_capacity: 128,
            recovery_config: RecoveryConfig {
                retry_policy: RetryPolicy {
                    max_attempts: 3,
                    initial_backoff: Duration::from_millis(10),
                    max_backoff: Duration::from_millis(500),
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

        // The messages are sent in a batch, so no messages are expected to be captured.
        let captured = mock_service.get_captured_data();
        assert!(captured.is_empty(), "should have captured no messages");

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
        assert_eq!(
            metrics.cur_retry_count.get(),
            4,
            "should have retried the checkpoint 4 times"
        );

        // Each gRPC call failure should trigger a checkpoint reingestion control message.
        let mut needs_reingestion_count = 0;
        while let Ok(msg) = control_rx.try_recv() {
            if msg == ControlMessage::CheckpointNeedsReingestion {
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
        let (grpc_channel, _mock_service) =
            crate::test::create_mock_grpc_channel_with_service().await;
        let (control_tx, mut control_rx) = broadcast::channel(1024);
        let (data_tx, data_rx) = async_channel::bounded(1024);
        let metrics = Arc::new(SiftStreamMetrics::default());
        let checkpoint_interval = Duration::from_millis(100);
        let config = TaskConfig {
            sift_stream_id: Uuid::new_v4(),
            grpc_channel,
            metrics: metrics.clone(),
            checkpoint_interval,
            control_channel_capacity: 128,
            data_channel_capacity: 128,
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
            if msg == ControlMessage::CheckpointComplete {
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
        let (grpc_channel, _mock_service) =
            crate::test::create_mock_grpc_channel_with_service().await;
        let (control_tx, mut control_rx) = broadcast::channel(1024);
        let (data_tx, data_rx) = async_channel::bounded(1024);
        let metrics = Arc::new(SiftStreamMetrics::default());
        let checkpoint_interval = Duration::from_secs(60);
        let config = TaskConfig {
            sift_stream_id: Uuid::new_v4(),
            grpc_channel,
            metrics: metrics.clone(),
            checkpoint_interval,
            control_channel_capacity: 128,
            data_channel_capacity: 128,
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
            metrics.checkpoint.checkpoint_timer_reached_cnt.get(),
            0,
            "should have reached the checkpoint timer 0 times"
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
            if msg == ControlMessage::CheckpointComplete {
                complete_count += 1;
            }
        }
        assert!(
            complete_count >= 2,
            "should have completed at least 2 checkpoints (1 for the final checkpoint)"
        );
    }
}
