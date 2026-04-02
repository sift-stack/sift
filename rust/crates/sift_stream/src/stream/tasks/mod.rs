pub(crate) mod builder;
pub(crate) mod ingestion;
pub(crate) mod metrics;

pub(crate) use builder::{LiveOnlyTaskConfig, LiveWithBackupsTaskConfig, TaskBuilder};
pub(crate) use metrics::MetricsStreamingTask;

use crate::{DiskBackupPolicy, RetryPolicy};
use std::{path::PathBuf, sync::Arc, time::Duration};

/// Capacity for the data channel.
pub(crate) const DATA_CHANNEL_CAPACITY: usize = 1024 * 100;

/// Capacity for the control channel.
pub(crate) const CONTROL_CHANNEL_CAPACITY: usize = 1024;

/// Timeout for the checkpoint operation to complete.
pub(crate) const CHECKPOINT_TIMEOUT: Duration = Duration::from_secs(10);

/// Control messages sent between tasks via broadcast channel.
/// These are low-frequency control messages, not high-volume data messages.
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

/// Data message with stream ID for routing
#[derive(Debug, Clone)]
pub(crate) struct DataMessage {
    pub(crate) message_id: u64,
    pub(crate) request: Arc<sift_rs::ingest::v1::IngestWithConfigDataStreamRequest>,
    pub(crate) dropped_for_ingestion: bool,
}
