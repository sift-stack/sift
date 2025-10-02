mod server;
pub(crate) use server::register_metrics;

#[cfg(feature = "metrics-unstable")]
pub use server::MetricsServerBuilder;

use std::{
    sync::atomic::{AtomicU64, Ordering},
    time::{Duration, SystemTime, UNIX_EPOCH},
};

#[cfg(feature = "metrics-unstable")]
use serde::Serialize;

#[cfg(feature = "metrics-unstable")]
/// **Unstable Feature:** API may change at any time
///
/// Snapshot of checkpoint-related metrics.
///
/// Tracks checkpoint operations, success/failure rates, and current checkpoint performance.
#[derive(Clone, Copy, Debug, Serialize)]
#[non_exhaustive]
pub struct CheckpointMetricsSnapshot {
    /// Total number of checkpoints completed
    pub checkpoint_count: u64,
    /// Number of checkpoints that failed, typically from connection issues
    pub failed_checkpoint_count: u64,
    /// Number of checkpoints triggered by timer
    pub checkpoint_timer_reached_cnt: u64,
    /// Number of checkpoints triggered by other Sift Stream events, such as reaching the max
    /// backup size
    pub checkpoint_manually_reached_cnt: u64,

    /// Elapsed time since current checkpoint started (seconds)
    pub cur_elapsed_secs: f64,
    /// Messages sent in current checkpoint
    pub cur_messages_sent: u64,
    /// Message rate for current checkpoint (messages/sec)
    pub cur_message_rate: f64,
    /// Bytes sent in current checkpoint
    pub cur_bytes_sent: u64,
    /// Byte rate for current checkpoint (bytes/sec)
    pub cur_byte_rate: f64,
}

#[cfg(feature = "metrics-unstable")]
/// **Unstable Feature:** API may change at any time
///
/// Snapshot of backup-related metrics.
///
/// Tracks file counts, byte totals, and ingestion status if backups are enabled.
#[derive(Clone, Copy, Debug, Serialize)]
#[non_exhaustive]
pub struct BackupMetricsSnapshot {
    /// Number of files written in current checkpoint
    pub cur_checkpoint_file_count: u64,
    /// Current file size being written (bytes)
    pub cur_checkpoint_cur_file_size: u64,
    /// Total bytes written in current checkpoint
    pub cur_checkpoint_bytes: u64,
    /// Total messages written in current checkpoint
    pub cur_checkpoint_messages: u64,
    /// Total number of backup files created
    pub total_file_count: u64,
    /// Total bytes written to backup files
    pub total_bytes: u64,
    /// Total messages written to backup files
    pub total_messages: u64,

    /// Number of files waiting to be ingested
    pub files_pending_ingestion: u64,
    /// Number of files successfully ingested
    pub files_ingested: u64,
    /// Current number of ingestion retries
    pub cur_ingest_retries: u64,
}

#[cfg(feature = "metrics-unstable")]
/// **Unstable Feature:** API may change at any time
///
/// Snapshot of all sift stream metrics at a point in time.
///
/// Contains performance and operational metrics for a given SiftStream instance
#[derive(Clone, Copy, Debug, Serialize)]
#[non_exhaustive]
pub struct SiftStreamMetricsSnapshot {
    /// Total elapsed time since SiftStream started (seconds)
    pub elapsed_secs: f64,
    /// Number of flows loaded from configuration
    pub loaded_flows: u64,
    /// Number of unique flows that have received data
    pub unique_flows_received: u64,
    /// Total messages received from data sources
    pub messages_received: u64,
    /// Total messages sent to destinations
    pub messages_sent: u64,
    /// Overall message throughput rate (messages/sec)
    pub message_rate: f64,
    /// Total bytes sent to destinations
    pub bytes_sent: u64,
    /// Overall byte throughput rate (bytes/sec)
    pub byte_rate: f64,
    /// Total messages written to backup storage
    pub messages_sent_to_backup: u64,
    /// Current retry attempt count
    pub cur_retry_count: u64,
    /// Checkpoint-specific metrics
    pub checkpoint: CheckpointMetricsSnapshot,
    /// Backup-specific metrics
    pub backups: BackupMetricsSnapshot,
}

pub(crate) struct StreamingStats {
    pub elapsed_secs: f64,
    pub messages_sent: u64,
    pub message_rate: f64,
    pub bytes_sent: u64,
    pub byte_rate: f64,
}

impl StreamingStats {
    pub(crate) fn calculate(
        start_time_ms: u64,
        messages_sent: u64,
        bytes_sent: u64,
    ) -> StreamingStats {
        let cur_time_ms = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_else(|_| {
                #[cfg(feature = "tracing")]
                tracing::warn!("System time was before unix epoch");
                Duration::default()
            })
            .as_millis() as u64;

        let elapsed_secs = (cur_time_ms.saturating_sub(start_time_ms) as f64) / 1000.0;

        StreamingStats {
            elapsed_secs,
            messages_sent,
            message_rate: (messages_sent as f64) / elapsed_secs,
            bytes_sent,
            byte_rate: (bytes_sent as f64) / elapsed_secs,
        }
    }
}

#[derive(Default, Debug)]
pub(crate) struct U64Counter(pub AtomicU64);

impl U64Counter {
    pub fn increment(&self) -> u64 {
        self.0.fetch_add(1, Ordering::Relaxed)
    }

    pub fn add(&self, val: u64) -> u64 {
        self.0.fetch_add(val, Ordering::Relaxed)
    }

    pub fn reset(&self) {
        self.0.store(0, Ordering::Relaxed);
    }

    pub fn get(&self) -> u64 {
        self.0.load(Ordering::Relaxed)
    }
}

#[derive(Default, Debug)]
pub(crate) struct U64Signal(pub AtomicU64);

impl U64Signal {
    pub fn set(&self, val: u64) {
        self.0.store(val, Ordering::Relaxed);
    }

    pub fn add(&self, val: u64) -> u64 {
        self.0.fetch_add(val, Ordering::Relaxed)
    }

    pub fn get(&self) -> u64 {
        self.0.load(Ordering::Relaxed)
    }
}

#[derive(Default, Debug)]
pub(crate) struct BackupMetrics {
    pub cur_checkpoint_file_count: U64Counter,
    pub cur_checkpoint_cur_file_size: U64Counter,
    pub cur_checkpoint_bytes: U64Counter,
    pub cur_checkpoint_messages: U64Counter,
    pub total_file_count: U64Counter,
    pub total_bytes: U64Counter,
    pub total_messages: U64Counter,

    pub files_pending_ingestion: U64Signal,
    pub files_ingested: U64Counter,
    pub cur_ingest_retries: U64Signal,
}

impl BackupMetrics {
    #[cfg(feature = "metrics-unstable")]
    pub fn snapshot(&self) -> BackupMetricsSnapshot {
        BackupMetricsSnapshot {
            cur_checkpoint_file_count: self.cur_checkpoint_file_count.get(),
            cur_checkpoint_cur_file_size: self.cur_checkpoint_cur_file_size.get(),
            cur_checkpoint_bytes: self.cur_checkpoint_bytes.get(),
            cur_checkpoint_messages: self.cur_checkpoint_messages.get(),
            total_file_count: self.total_file_count.get(),
            total_bytes: self.total_bytes.get(),
            total_messages: self.total_messages.get(),
            files_pending_ingestion: self.files_pending_ingestion.get(),
            files_ingested: self.files_ingested.get(),
            cur_ingest_retries: self.cur_ingest_retries.get(),
        }
    }

    pub fn log_message(&self, msg_size: u64) {
        self.cur_checkpoint_messages.increment();
        self.total_messages.increment();
        self.cur_checkpoint_cur_file_size.add(msg_size);
        self.cur_checkpoint_bytes.add(msg_size);
        self.total_bytes.add(msg_size);
    }

    pub fn log_new_file(&self) {
        self.cur_checkpoint_file_count.increment();
        self.total_file_count.increment();
        self.cur_checkpoint_cur_file_size.reset();
    }

    pub fn log_restart(&self) {
        self.cur_checkpoint_messages.reset();
        self.cur_checkpoint_cur_file_size.reset();
        self.cur_checkpoint_bytes.reset();
        self.cur_checkpoint_file_count.reset();
    }
}

#[derive(Default, Debug)]
pub(crate) struct CheckpointMetrics {
    pub checkpoint_count: U64Counter,
    pub failed_checkpoint_count: U64Counter,
    pub checkpoint_timer_reached_cnt: U64Counter,
    pub checkpoint_manually_reached_cnt: U64Counter,

    checkpoint_start_time_epoch_ms: AtomicU64,
    pub cur_messages_sent: U64Counter,
    pub cur_bytes_sent: U64Counter,
}

impl CheckpointMetrics {
    #[cfg(feature = "metrics-unstable")]
    pub fn snapshot(&self) -> CheckpointMetricsSnapshot {
        let checkpoint_count = self.checkpoint_count.get();
        let failed_checkpoint_count = self.failed_checkpoint_count.get();
        let checkpoint_timer_reached_cnt = self.checkpoint_timer_reached_cnt.get();
        let checkpoint_manually_reached_cnt = self.checkpoint_manually_reached_cnt.get();
        let cur_messages_sent = self.cur_messages_sent.get();
        let cur_bytes_sent = self.cur_bytes_sent.get();

        let stats = StreamingStats::calculate(
            self.checkpoint_start_time_epoch_ms.load(Ordering::Relaxed),
            cur_messages_sent,
            cur_bytes_sent,
        );

        CheckpointMetricsSnapshot {
            checkpoint_count,
            failed_checkpoint_count,
            checkpoint_timer_reached_cnt,
            checkpoint_manually_reached_cnt,
            cur_elapsed_secs: stats.elapsed_secs,
            cur_messages_sent,
            cur_message_rate: stats.message_rate,
            cur_bytes_sent,
            cur_byte_rate: stats.byte_rate,
        }
    }

    pub fn next_checkpoint(&self) {
        self.checkpoint_count.increment();
        self.cur_bytes_sent.reset();
        self.cur_messages_sent.reset();
        self.checkpoint_start_time_epoch_ms.store(
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_else(|_| {
                    #[cfg(feature = "tracing")]
                    tracing::warn!("System time was before unix epoch");
                    Duration::default()
                })
                .as_millis() as u64,
            Ordering::Relaxed,
        );
    }
}

/// Primary metrics collection struct for sift stream operations.
///
/// This struct is managed internally and users should never need to create this,
/// instead using the [crate::SiftStreamBuilder]
#[derive(Default, Debug)]
pub struct SiftStreamMetrics {
    creation_time_epoch_ms: u64,
    pub(crate) loaded_flows: U64Counter,
    pub(crate) unique_flows_received: U64Counter,
    pub(crate) messages_received: U64Counter,
    pub(crate) messages_sent: U64Counter,
    pub(crate) bytes_sent: U64Counter,
    pub(crate) messages_sent_to_backup: U64Counter,
    pub(crate) cur_retry_count: U64Signal,
    pub(crate) checkpoint: CheckpointMetrics,
    pub(crate) backups: BackupMetrics,
}

impl SiftStreamMetrics {
    /// Creates a new SiftStreamMetrics instance with current timestamp. Users should
    /// never need to call this and should instead use [crate::SiftStreamBuilder]
    pub fn new() -> SiftStreamMetrics {
        SiftStreamMetrics {
            creation_time_epoch_ms: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_else(|_| {
                    #[cfg(feature = "tracing")]
                    tracing::warn!("System time was before unix epoch");
                    Duration::default()
                })
                .as_millis() as u64,
            ..Default::default()
        }
    }

    #[cfg(feature = "metrics-unstable")]
    pub(crate) fn snapshot(&self) -> SiftStreamMetricsSnapshot {
        let loaded_flows = self.loaded_flows.get();
        let unique_flows_received = self.unique_flows_received.get();
        let messages_received = self.messages_received.get();
        let messages_sent = self.messages_sent.get();
        let bytes_sent = self.bytes_sent.get();
        let messages_sent_to_backup = self.messages_sent_to_backup.get();
        let cur_retry_count = self.cur_retry_count.get();

        let stats =
            StreamingStats::calculate(self.creation_time_epoch_ms, messages_sent, bytes_sent);

        SiftStreamMetricsSnapshot {
            elapsed_secs: stats.elapsed_secs,
            loaded_flows,
            unique_flows_received,
            messages_received,
            messages_sent,
            message_rate: stats.message_rate,
            bytes_sent,
            byte_rate: stats.byte_rate,
            messages_sent_to_backup,
            cur_retry_count,
            checkpoint: self.checkpoint.snapshot(),
            backups: self.backups.snapshot(),
        }
    }

    pub(crate) fn get_checkpoint_stats(&self) -> StreamingStats {
        let start_time_ms = self
            .checkpoint
            .checkpoint_start_time_epoch_ms
            .load(Ordering::Relaxed);
        let messages_sent = self.checkpoint.cur_messages_sent.0.load(Ordering::Relaxed);
        let bytes_sent = self.checkpoint.cur_bytes_sent.0.load(Ordering::Relaxed);

        StreamingStats::calculate(start_time_ms, messages_sent, bytes_sent)
    }
}
