use crate::{
    RetryPolicy,
    backup::disk::{AsyncBackupsManager, BackupIngestTask},
    metrics::SiftStreamMetrics,
    stream::tasks::{
        ControlMessage, DataMessage, RecoveryConfig,
        ingestion::{IngestionTask, IngestionTaskConfig},
        metrics::MetricsStreamingTask,
    },
};
use sift_connect::SiftChannel;
use sift_error::prelude::*;
use std::{sync::Arc, time::Duration};
use tokio::{sync::broadcast, task::JoinHandle};
use uuid::Uuid;

/// Configuration for tasks in LiveStreamingOnly mode.
pub(crate) struct LiveOnlyTaskConfig {
    pub(crate) session_name: String,
    pub(crate) sift_stream_id: Uuid,
    pub(crate) setup_channel: SiftChannel,
    pub(crate) ingestion_channel: SiftChannel,
    pub(crate) metrics: Arc<SiftStreamMetrics>,
    pub(crate) enable_compression_for_ingestion: bool,
    pub(crate) ingestion_data_channel_capacity: usize,
    pub(crate) control_channel_capacity: usize,
    pub(crate) retry_policy: RetryPolicy,
    pub(crate) metrics_streaming_interval: Option<Duration>,
}

/// Task handles and channel senders returned for LiveStreamingOnly mode.
pub(crate) struct LiveOnlyTasks {
    pub(crate) ingestion_tx: async_channel::Sender<DataMessage>,
    pub(crate) control_tx: broadcast::Sender<ControlMessage>,
    pub(crate) ingestion: JoinHandle<Result<()>>,
    pub(crate) metrics_streaming: Option<JoinHandle<Result<()>>>,
}

/// Configuration for tasks in LiveStreamingWithBackups mode.
pub(crate) struct LiveWithBackupsTaskConfig {
    pub(crate) session_name: String,
    pub(crate) sift_stream_id: Uuid,
    pub(crate) setup_channel: SiftChannel,
    pub(crate) ingestion_channel: SiftChannel,
    pub(crate) reingestion_channel: SiftChannel,
    pub(crate) metrics: Arc<SiftStreamMetrics>,
    pub(crate) enable_compression_for_ingestion: bool,
    pub(crate) ingestion_data_channel_capacity: usize,
    pub(crate) backup_data_channel_capacity: usize,
    pub(crate) control_channel_capacity: usize,
    pub(crate) checkpoint_interval: Duration,
    pub(crate) retry_policy: RetryPolicy,
    pub(crate) recovery_config: RecoveryConfig,
    pub(crate) metrics_streaming_interval: Option<Duration>,
}

/// Task handles and channel senders returned for LiveStreamingWithBackups mode.
pub(crate) struct LiveWithBackupsTasks {
    pub(crate) backup_tx: async_channel::Sender<DataMessage>,
    pub(crate) ingestion_tx: async_channel::Sender<DataMessage>,
    pub(crate) control_tx: broadcast::Sender<ControlMessage>,
    pub(crate) ingestion: JoinHandle<Result<()>>,
    pub(crate) backup_manager: JoinHandle<Result<()>>,
    pub(crate) reingestion: JoinHandle<Result<()>>,
    pub(crate) metrics_streaming: Option<JoinHandle<Result<()>>>,
}

pub(crate) struct TaskBuilder;

impl TaskBuilder {
    /// Start tasks for LiveStreamingOnly mode.
    /// Creates a single bounded ingestion channel; `send` awaits on it for backpressure.
    /// No checkpoint timer is configured — the IngestionTask runs until Shutdown.
    pub(crate) async fn start_live_only(config: LiveOnlyTaskConfig) -> Result<LiveOnlyTasks> {
        let (ingestion_tx, ingestion_rx) =
            async_channel::bounded(config.ingestion_data_channel_capacity);
        let (control_tx, _control_rx) = broadcast::channel(config.control_channel_capacity);

        let task_config = IngestionTaskConfig {
            session_name: config.session_name.clone(),
            sift_stream_id: config.sift_stream_id,
            ingestion_channel: config.ingestion_channel,
            enable_compression_for_ingestion: config.enable_compression_for_ingestion,
            metrics: config.metrics.clone(),
            retry_policy: config.retry_policy,
            checkpoint_interval: None, // no checkpointing in live-only mode
        };
        let control_rx = control_tx.subscribe();
        let ingestion_task =
            IngestionTask::new(control_tx.clone(), control_rx, ingestion_rx, task_config);
        let ingestion = tokio::spawn(async move {
            let mut task = ingestion_task;
            task.run().await
        });

        let metrics_streaming = if let Some(interval) = config.metrics_streaming_interval {
            let task = MetricsStreamingTask::new(
                config.setup_channel,
                control_tx.subscribe(),
                config.session_name.clone(),
                interval,
                config.metrics,
            )
            .await?;
            Some(tokio::spawn(task.run()))
        } else {
            None
        };

        Ok(LiveOnlyTasks {
            ingestion_tx,
            control_tx,
            ingestion,
            metrics_streaming,
        })
    }

    /// Start tasks for LiveStreamingWithBackups mode.
    /// Equivalent to the current `start_tasks()` function, rewritten to take
    /// `LiveWithBackupsTaskConfig` and return `LiveWithBackupsTasks`.
    pub(crate) async fn start_live_with_backups(
        config: LiveWithBackupsTaskConfig,
    ) -> Result<LiveWithBackupsTasks> {
        let (control_tx, _control_rx) = broadcast::channel(config.control_channel_capacity);

        let (ingestion_tx, ingestion_rx) =
            async_channel::bounded(config.ingestion_data_channel_capacity);
        let (backup_tx, backup_rx) = async_channel::bounded(config.backup_data_channel_capacity);

        let backup_control_tx = control_tx.clone();
        let backup_control_rx = backup_control_tx.subscribe();
        let backup_data_rx = backup_rx.clone();

        let mut backup_manager_task = AsyncBackupsManager::new(
            &config.recovery_config.backups_directory,
            &config.recovery_config.backups_prefix,
            config.recovery_config.backup_policy.clone(),
            backup_control_tx,
            backup_control_rx,
            backup_data_rx,
            config.metrics.clone(),
        )
        .await?;

        let sift_stream_id = config.sift_stream_id;
        let backup_manager = tokio::spawn(async move {
            #[cfg(feature = "tracing")]
            tracing::info!(
                sift_stream_id = %sift_stream_id,
                "backup manager task started"
            );
            backup_manager_task.run().await
        });

        let ingestion_control_tx = control_tx.clone();
        let ingestion_control_rx = ingestion_control_tx.subscribe();
        let task_config = IngestionTaskConfig {
            session_name: config.session_name.clone(),
            sift_stream_id: config.sift_stream_id,
            ingestion_channel: config.ingestion_channel,
            enable_compression_for_ingestion: config.enable_compression_for_ingestion,
            metrics: config.metrics.clone(),
            retry_policy: config.retry_policy,
            checkpoint_interval: Some(config.checkpoint_interval),
        };
        let mut ingestion_task = IngestionTask::new(
            ingestion_control_tx,
            ingestion_control_rx,
            ingestion_rx.clone(),
            task_config,
        );
        let ingestion = tokio::spawn(async move {
            #[cfg(feature = "tracing")]
            tracing::info!(
                sift_stream_id = %sift_stream_id,
                "ingestion task started"
            );
            ingestion_task.run().await
        });

        let reingestion_control_tx = control_tx.clone();
        let reingest_retry_policy = RetryPolicy {
            max_attempts: 12,
            initial_backoff: Duration::from_millis(100),
            max_backoff: Duration::from_secs(15),
            backoff_multiplier: 5,
        };
        let reingestion_task = BackupIngestTask::new(
            reingestion_control_tx.subscribe(),
            config.reingestion_channel,
            config.enable_compression_for_ingestion,
            reingest_retry_policy,
            config.recovery_config.backup_policy.retain_backups,
            config.metrics.clone(),
        );
        let reingestion = tokio::spawn(async move {
            #[cfg(feature = "tracing")]
            tracing::info!(
                sift_stream_id = %sift_stream_id,
                "backup re-ingestion task started"
            );
            reingestion_task.run().await
        });

        let metrics_streaming = if let Some(interval) = config.metrics_streaming_interval {
            let metrics_task = MetricsStreamingTask::new(
                config.setup_channel,
                control_tx.subscribe(),
                config.session_name.clone(),
                interval,
                config.metrics.clone(),
            )
            .await?;
            Some(tokio::spawn(async move {
                #[cfg(feature = "tracing")]
                tracing::info!(
                    sift_stream_id = %sift_stream_id,
                    "metrics streaming task started"
                );
                metrics_task.run().await
            }))
        } else {
            None
        };

        #[cfg(feature = "tracing")]
        tracing::info!(
            sift_stream_id = %sift_stream_id,
            "Sift streaming successfully initialized"
        );

        Ok(LiveWithBackupsTasks {
            backup_tx,
            ingestion_tx,
            control_tx,
            ingestion,
            backup_manager,
            reingestion,
            metrics_streaming,
        })
    }
}
