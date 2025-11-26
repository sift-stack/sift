#[cfg(feature = "metrics-unstable")]
mod server;

#[cfg(feature = "metrics-unstable")]
pub(crate) use server::register_metrics;

#[cfg(feature = "metrics-unstable")]
pub use server::MetricsServerBuilder;

use std::{
    sync::atomic::{AtomicU64, Ordering},
    time::{Duration, Instant},
};

#[cfg(feature = "metrics-unstable")]
use serde::Serialize;

use sift_rs::{common::r#type::v1::ChannelDataType, ingestion_configs::v2::ChannelConfig};

use crate::stream::channel::ChannelValue;

/// **Unstable Feature:** API may change at any time
///
/// Snapshot of checkpoint-related metrics.
///
/// Tracks checkpoint operations, success/failure rates, and current checkpoint performance.
#[derive(Clone, Copy, Debug)]
#[cfg_attr(feature = "metrics-unstable", derive(Serialize))]
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

/// **Unstable Feature:** API may change at any time
///
/// Snapshot of backup-related metrics.
///
/// Tracks file counts, byte totals, and ingestion status if backups are enabled.
#[derive(Clone, Copy, Debug)]
#[cfg_attr(feature = "metrics-unstable", derive(Serialize))]
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

    /// Message ID of the last committed message
    pub committed_message_id: u64,
    /// Number of checkpoints queued for processing
    pub queued_checkpoints: u64,
    /// Number of file contexts queued for processing
    pub queued_file_ctxs: u64,

    /// Number of files waiting to be ingested
    pub files_pending_ingestion: u64,
    /// Number of files successfully ingested
    pub files_ingested: u64,
    /// Current number of ingestion retries
    pub cur_ingest_retries: u64,
}

/// **Unstable Feature:** API may change at any time
///
/// Snapshot of all sift stream metrics at a point in time.
///
/// Contains performance and operational metrics for a given SiftStream instance
#[derive(Clone, Copy, Debug)]
#[cfg_attr(feature = "metrics-unstable", derive(Serialize))]
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
    /// Total messages dropped for ingestion
    pub old_messages_dropped_for_ingestion: u64,
    /// Total messages dropped for ingestion that failed to be added to backup
    pub old_messages_failed_adding_to_backup: u64,
    /// Current retry attempt count
    pub cur_retry_count: u64,
    /// Depth of the ingestion channel
    pub ingestion_channel_depth: u64,
    /// Depth of the backup channel
    pub backup_channel_depth: u64,
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
        start_time: Instant,
        messages_sent: u64,
        bytes_sent: u64,
    ) -> StreamingStats {
        let elapsed_secs = start_time.elapsed().as_secs_f64();

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

    #[cfg_attr(not(feature = "metrics-unstable"), allow(unused))]
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

    #[cfg_attr(not(feature = "metrics-unstable"), allow(unused))]
    pub fn get(&self) -> u64 {
        self.0.load(Ordering::Relaxed)
    }
}

#[derive(Debug)]
pub(crate) struct AtomicInstant {
    init_instant: Instant,
    offset_ns: AtomicU64,
}

impl AtomicInstant {
    pub fn new(init: Instant) -> AtomicInstant {
        AtomicInstant {
            init_instant: init,
            offset_ns: AtomicU64::new(0),
        }
    }

    pub fn get(&self) -> Instant {
        self.init_instant + Duration::from_nanos(self.offset_ns.load(Ordering::Relaxed))
    }

    pub fn set(&self, val: Instant) {
        let duration_since = val.duration_since(self.init_instant);
        let new_offset_ns = duration_since.as_nanos() as u64;
        self.offset_ns.store(new_offset_ns, Ordering::Relaxed);
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

    pub committed_message_id: U64Signal,
    pub queued_checkpoints: U64Signal,
    pub queued_file_ctxs: U64Signal,

    pub files_pending_ingestion: U64Signal,
    pub files_ingested: U64Counter,
    pub cur_ingest_retries: U64Signal,
}

impl BackupMetrics {
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
            committed_message_id: self.committed_message_id.get(),
            queued_checkpoints: self.queued_checkpoints.get(),
            queued_file_ctxs: self.queued_file_ctxs.get(),
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

#[derive(Debug)]
pub(crate) struct CheckpointMetrics {
    pub checkpoint_count: U64Counter,
    pub failed_checkpoint_count: U64Counter,
    pub checkpoint_timer_reached_cnt: U64Counter,
    pub checkpoint_manually_reached_cnt: U64Counter,

    checkpoint_start_time: AtomicInstant,
    pub cur_messages_sent: U64Counter,
    pub cur_bytes_sent: U64Counter,
}

impl CheckpointMetrics {
    pub fn snapshot(&self) -> CheckpointMetricsSnapshot {
        let checkpoint_count = self.checkpoint_count.get();
        let failed_checkpoint_count = self.failed_checkpoint_count.get();
        let checkpoint_timer_reached_cnt = self.checkpoint_timer_reached_cnt.get();
        let checkpoint_manually_reached_cnt = self.checkpoint_manually_reached_cnt.get();
        let cur_messages_sent = self.cur_messages_sent.get();
        let cur_bytes_sent = self.cur_bytes_sent.get();

        let stats = StreamingStats::calculate(
            self.checkpoint_start_time.get(),
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
        #[cfg(feature = "tracing")]
        {
            let checkpoint_stats = StreamingStats::calculate(
                self.checkpoint_start_time.get(),
                self.cur_messages_sent.get(),
                self.cur_bytes_sent.get(),
            );
            let bytes_processed_pretty = bytesize::ByteSize::b(checkpoint_stats.bytes_sent)
                .display()
                .iec();
            let byte_rate_pretty = bytesize::ByteSize::b(checkpoint_stats.byte_rate.ceil() as u64)
                .display()
                .iec();

            tracing::info!(
                checkpoint_count = format!("{}", self.checkpoint_count.get()),
                stream_duration = format!("{:.1}s", checkpoint_stats.elapsed_secs),
                messages_processed = checkpoint_stats.messages_sent,
                message_rate = format!("{} messages/s", checkpoint_stats.message_rate),
                bytes_processed = format!("{bytes_processed_pretty}"),
                byte_rate = format!("{byte_rate_pretty}/s"),
            );
        }

        self.checkpoint_count.increment();
        self.cur_bytes_sent.reset();
        self.cur_messages_sent.reset();
        self.checkpoint_start_time.set(Instant::now());
    }
}

impl Default for CheckpointMetrics {
    fn default() -> CheckpointMetrics {
        CheckpointMetrics {
            checkpoint_count: U64Counter::default(),
            failed_checkpoint_count: U64Counter::default(),
            checkpoint_timer_reached_cnt: U64Counter::default(),
            checkpoint_manually_reached_cnt: U64Counter::default(),
            checkpoint_start_time: AtomicInstant::new(Instant::now()),
            cur_messages_sent: U64Counter::default(),
            cur_bytes_sent: U64Counter::default(),
        }
    }
}

/// Primary metrics collection struct for sift stream operations.
///
/// This struct is managed internally and users should never need to create this,
/// instead using the [crate::SiftStreamBuilder]
#[derive(Debug)]
pub struct SiftStreamMetrics {
    #[cfg_attr(not(feature = "metrics-unstable"), allow(unused))]
    creation_time: Instant,
    pub(crate) loaded_flows: U64Counter,
    pub(crate) unique_flows_received: U64Counter,
    pub(crate) messages_received: U64Counter,
    pub(crate) messages_sent: U64Counter,
    pub(crate) bytes_sent: U64Counter,
    pub(crate) messages_sent_to_backup: U64Counter,
    pub(crate) old_messages_dropped_for_ingestion: U64Counter,
    pub(crate) old_messages_failed_adding_to_backup: U64Counter,
    pub(crate) cur_retry_count: U64Signal,
    pub(crate) ingestion_channel_depth: U64Signal,
    pub(crate) backup_channel_depth: U64Signal,
    pub(crate) checkpoint: CheckpointMetrics,
    pub(crate) backups: BackupMetrics,
}

impl SiftStreamMetrics {
    /// Creates a new SiftStreamMetrics instance with current timestamp. Users should
    /// never need to call this and should instead use [crate::SiftStreamBuilder]
    pub fn new() -> SiftStreamMetrics {
        SiftStreamMetrics {
            creation_time: Instant::now(),
            ..Default::default()
        }
    }

    pub(crate) fn snapshot(&self) -> SiftStreamMetricsSnapshot {
        let loaded_flows = self.loaded_flows.get();
        let unique_flows_received = self.unique_flows_received.get();
        let messages_received = self.messages_received.get();
        let messages_sent = self.messages_sent.get();
        let bytes_sent = self.bytes_sent.get();
        let messages_sent_to_backup = self.messages_sent_to_backup.get();
        let old_messages_dropped_for_ingestion = self.old_messages_dropped_for_ingestion.get();
        let old_messages_failed_adding_to_backup = self.old_messages_failed_adding_to_backup.get();
        let cur_retry_count = self.cur_retry_count.get();
        let ingestion_channel_depth = self.ingestion_channel_depth.get();
        let backup_channel_depth = self.backup_channel_depth.get();

        let stats = StreamingStats::calculate(self.creation_time, messages_sent, bytes_sent);

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
            old_messages_dropped_for_ingestion,
            old_messages_failed_adding_to_backup,
            cur_retry_count,
            ingestion_channel_depth,
            backup_channel_depth,
            checkpoint: self.checkpoint.snapshot(),
            backups: self.backups.snapshot(),
        }
    }
}

impl SiftStreamMetricsSnapshot {
    /// Creates channel configs for all metrics.
    pub(crate) fn channel_configs(channel_prefix: &str) -> Vec<ChannelConfig> {
        vec![
            ChannelConfig {
                name: format!("{channel_prefix}.elapsed_secs"),
                description: "Elapsed seconds since stream creation".into(),
                data_type: ChannelDataType::Double.into(),
                ..Default::default()
            },
            ChannelConfig {
                name: format!("{channel_prefix}.loaded_flows"),
                description: "Number of loaded flows".into(),
                data_type: ChannelDataType::Uint64.into(),
                ..Default::default()
            },
            ChannelConfig {
                name: format!("{channel_prefix}.unique_flows_received"),
                description: "Number of unique flows received".into(),
                data_type: ChannelDataType::Uint64.into(),
                ..Default::default()
            },
            ChannelConfig {
                name: format!("{channel_prefix}.messages_received"),
                description: "Total messages received".into(),
                data_type: ChannelDataType::Uint64.into(),
                ..Default::default()
            },
            ChannelConfig {
                name: format!("{channel_prefix}.messages_sent"),
                description: "Total messages sent".into(),
                data_type: ChannelDataType::Uint64.into(),
                ..Default::default()
            },
            ChannelConfig {
                name: format!("{channel_prefix}.message_rate"),
                description: "Message rate (messages/sec)".into(),
                data_type: ChannelDataType::Double.into(),
                ..Default::default()
            },
            ChannelConfig {
                name: format!("{channel_prefix}.bytes_sent"),
                description: "Total bytes sent".into(),
                data_type: ChannelDataType::Uint64.into(),
                ..Default::default()
            },
            ChannelConfig {
                name: format!("{channel_prefix}.byte_rate"),
                description: "Byte rate (bytes/sec)".into(),
                data_type: ChannelDataType::Double.into(),
                ..Default::default()
            },
            ChannelConfig {
                name: format!("{channel_prefix}.messages_sent_to_backup"),
                description: "Messages sent to backup".into(),
                data_type: ChannelDataType::Uint64.into(),
                ..Default::default()
            },
            ChannelConfig {
                name: format!("{channel_prefix}.old_messages_dropped_for_ingestion"),
                description: "Old messages dropped for ingestion".into(),
                data_type: ChannelDataType::Uint64.into(),
                ..Default::default()
            },
            ChannelConfig {
                name: format!("{channel_prefix}.old_messages_failed_adding_to_backup"),
                description: "Old messages failed to add to backup".into(),
                data_type: ChannelDataType::Uint64.into(),
                ..Default::default()
            },
            ChannelConfig {
                name: format!("{channel_prefix}.cur_retry_count"),
                description: "Current retry count".into(),
                data_type: ChannelDataType::Uint64.into(),
                ..Default::default()
            },
            ChannelConfig {
                name: format!("{channel_prefix}.ingestion_channel_depth"),
                description: "Ingestion channel depth".into(),
                data_type: ChannelDataType::Uint64.into(),
                ..Default::default()
            },
            ChannelConfig {
                name: format!("{channel_prefix}.backup_channel_depth"),
                description: "Backup channel depth".into(),
                data_type: ChannelDataType::Uint64.into(),
                ..Default::default()
            },
            ChannelConfig {
                name: format!("{channel_prefix}.checkpoint.count"),
                description: "Total checkpoint count".into(),
                data_type: ChannelDataType::Uint64.into(),
                ..Default::default()
            },
            ChannelConfig {
                name: format!("{channel_prefix}.checkpoint.failed_count"),
                description: "Failed checkpoint count".into(),
                data_type: ChannelDataType::Uint64.into(),
                ..Default::default()
            },
            ChannelConfig {
                name: format!("{channel_prefix}.checkpoint.timer_reached_count"),
                description: "Checkpoint timer reached count".into(),
                data_type: ChannelDataType::Uint64.into(),
                ..Default::default()
            },
            ChannelConfig {
                name: format!("{channel_prefix}.checkpoint.manually_reached_count"),
                description: "Checkpoint manually reached count".into(),
                data_type: ChannelDataType::Uint64.into(),
                ..Default::default()
            },
            ChannelConfig {
                name: format!("{channel_prefix}.checkpoint.cur_elapsed_secs"),
                description: "Current checkpoint elapsed seconds".into(),
                data_type: ChannelDataType::Double.into(),
                ..Default::default()
            },
            ChannelConfig {
                name: format!("{channel_prefix}.checkpoint.cur_messages_sent"),
                description: "Current checkpoint messages sent".into(),
                data_type: ChannelDataType::Uint64.into(),
                ..Default::default()
            },
            ChannelConfig {
                name: format!("{channel_prefix}.checkpoint.cur_message_rate"),
                description: "Current checkpoint message rate".into(),
                data_type: ChannelDataType::Double.into(),
                ..Default::default()
            },
            ChannelConfig {
                name: format!("{channel_prefix}.checkpoint.cur_bytes_sent"),
                description: "Current checkpoint bytes sent".into(),
                data_type: ChannelDataType::Uint64.into(),
                ..Default::default()
            },
            ChannelConfig {
                name: format!("{channel_prefix}.checkpoint.cur_byte_rate"),
                description: "Current checkpoint byte rate".into(),
                data_type: ChannelDataType::Double.into(),
                ..Default::default()
            },
            ChannelConfig {
                name: format!("{channel_prefix}.backups.cur_checkpoint_file_count"),
                description: "Current checkpoint file count".into(),
                data_type: ChannelDataType::Uint64.into(),
                ..Default::default()
            },
            ChannelConfig {
                name: format!("{channel_prefix}.backups.cur_checkpoint_cur_file_size"),
                description: "Current checkpoint current file size".into(),
                data_type: ChannelDataType::Uint64.into(),
                ..Default::default()
            },
            ChannelConfig {
                name: format!("{channel_prefix}.backups.cur_checkpoint_bytes"),
                description: "Current checkpoint bytes".into(),
                data_type: ChannelDataType::Uint64.into(),
                ..Default::default()
            },
            ChannelConfig {
                name: format!("{channel_prefix}.backups.cur_checkpoint_messages"),
                description: "Current checkpoint messages".into(),
                data_type: ChannelDataType::Uint64.into(),
                ..Default::default()
            },
            ChannelConfig {
                name: format!("{channel_prefix}.backups.total_file_count"),
                description: "Total file count".into(),
                data_type: ChannelDataType::Uint64.into(),
                ..Default::default()
            },
            ChannelConfig {
                name: format!("{channel_prefix}.backups.total_bytes"),
                description: "Total bytes".into(),
                data_type: ChannelDataType::Uint64.into(),
                ..Default::default()
            },
            ChannelConfig {
                name: format!("{channel_prefix}.backups.total_messages"),
                description: "Total messages".into(),
                data_type: ChannelDataType::Uint64.into(),
                ..Default::default()
            },
            ChannelConfig {
                name: format!("{channel_prefix}.backups.files_pending_ingestion"),
                description: "Files pending ingestion".into(),
                data_type: ChannelDataType::Uint64.into(),
                ..Default::default()
            },
            ChannelConfig {
                name: format!("{channel_prefix}.backups.files_ingested"),
                description: "Files ingested".into(),
                data_type: ChannelDataType::Uint64.into(),
                ..Default::default()
            },
            ChannelConfig {
                name: format!("{channel_prefix}.backups.cur_ingest_retries"),
                description: "Current ingest retries".into(),
                data_type: ChannelDataType::Uint64.into(),
                ..Default::default()
            },
        ]
    }

    pub(crate) fn channel_values(&self, channel_prefix: &str) -> Vec<ChannelValue> {
        vec![
            ChannelValue::new(&format!("{channel_prefix}.elapsed_secs"), self.elapsed_secs),
            ChannelValue::new(&format!("{channel_prefix}.loaded_flows"), self.loaded_flows),
            ChannelValue::new(
                &format!("{channel_prefix}.unique_flows_received"),
                self.unique_flows_received,
            ),
            ChannelValue::new(
                &format!("{channel_prefix}.messages_received"),
                self.messages_received,
            ),
            ChannelValue::new(
                &format!("{channel_prefix}.messages_sent"),
                self.messages_sent,
            ),
            ChannelValue::new(&format!("{channel_prefix}.message_rate"), self.message_rate),
            ChannelValue::new(&format!("{channel_prefix}.bytes_sent"), self.bytes_sent),
            ChannelValue::new(&format!("{channel_prefix}.byte_rate"), self.byte_rate),
            ChannelValue::new(
                &format!("{channel_prefix}.messages_sent_to_backup"),
                self.messages_sent_to_backup,
            ),
            ChannelValue::new(
                &format!("{channel_prefix}.old_messages_dropped_for_ingestion"),
                self.old_messages_dropped_for_ingestion,
            ),
            ChannelValue::new(
                &format!("{channel_prefix}.old_messages_failed_adding_to_backup"),
                self.old_messages_failed_adding_to_backup,
            ),
            ChannelValue::new(
                &format!("{channel_prefix}.cur_retry_count"),
                self.cur_retry_count,
            ),
            ChannelValue::new(
                &format!("{channel_prefix}.ingestion_channel_depth"),
                self.ingestion_channel_depth,
            ),
            ChannelValue::new(
                &format!("{channel_prefix}.backup_channel_depth"),
                self.backup_channel_depth,
            ),
            ChannelValue::new(
                &format!("{channel_prefix}.checkpoint.count"),
                self.checkpoint.checkpoint_count,
            ),
            ChannelValue::new(
                &format!("{channel_prefix}.checkpoint.failed_count"),
                self.checkpoint.failed_checkpoint_count,
            ),
            ChannelValue::new(
                &format!("{channel_prefix}.checkpoint.timer_reached_count"),
                self.checkpoint.checkpoint_timer_reached_cnt,
            ),
            ChannelValue::new(
                &format!("{channel_prefix}.checkpoint.manually_reached_count"),
                self.checkpoint.checkpoint_manually_reached_cnt,
            ),
            ChannelValue::new(
                &format!("{channel_prefix}.checkpoint.cur_elapsed_secs"),
                self.checkpoint.cur_elapsed_secs,
            ),
            ChannelValue::new(
                &format!("{channel_prefix}.checkpoint.cur_messages_sent"),
                self.checkpoint.cur_messages_sent,
            ),
            ChannelValue::new(
                &format!("{channel_prefix}.checkpoint.cur_message_rate"),
                self.checkpoint.cur_message_rate,
            ),
            ChannelValue::new(
                &format!("{channel_prefix}.checkpoint.cur_bytes_sent"),
                self.checkpoint.cur_bytes_sent,
            ),
            ChannelValue::new(
                &format!("{channel_prefix}.checkpoint.cur_byte_rate"),
                self.checkpoint.cur_byte_rate,
            ),
            ChannelValue::new(
                &format!("{channel_prefix}.backups.cur_checkpoint_file_count"),
                self.backups.cur_checkpoint_file_count,
            ),
            ChannelValue::new(
                &format!("{channel_prefix}.backups.cur_checkpoint_cur_file_size"),
                self.backups.cur_checkpoint_cur_file_size,
            ),
            ChannelValue::new(
                &format!("{channel_prefix}.backups.cur_checkpoint_bytes"),
                self.backups.cur_checkpoint_bytes,
            ),
            ChannelValue::new(
                &format!("{channel_prefix}.backups.cur_checkpoint_messages"),
                self.backups.cur_checkpoint_messages,
            ),
            ChannelValue::new(
                &format!("{channel_prefix}.backups.total_file_count"),
                self.backups.total_file_count,
            ),
            ChannelValue::new(
                &format!("{channel_prefix}.backups.total_bytes"),
                self.backups.total_bytes,
            ),
            ChannelValue::new(
                &format!("{channel_prefix}.backups.total_messages"),
                self.backups.total_messages,
            ),
            ChannelValue::new(
                &format!("{channel_prefix}.backups.files_pending_ingestion"),
                self.backups.files_pending_ingestion,
            ),
            ChannelValue::new(
                &format!("{channel_prefix}.backups.files_ingested"),
                self.backups.files_ingested,
            ),
            ChannelValue::new(
                &format!("{channel_prefix}.backups.cur_ingest_retries"),
                self.backups.cur_ingest_retries,
            ),
        ]
    }
}

impl Default for SiftStreamMetrics {
    fn default() -> SiftStreamMetrics {
        SiftStreamMetrics {
            creation_time: Instant::now(),
            loaded_flows: U64Counter::default(),
            unique_flows_received: U64Counter::default(),
            messages_received: U64Counter::default(),
            messages_sent: U64Counter::default(),
            bytes_sent: U64Counter::default(),
            messages_sent_to_backup: U64Counter::default(),
            old_messages_dropped_for_ingestion: U64Counter::default(),
            old_messages_failed_adding_to_backup: U64Counter::default(),
            cur_retry_count: U64Signal::default(),
            ingestion_channel_depth: U64Signal::default(),
            backup_channel_depth: U64Signal::default(),
            checkpoint: CheckpointMetrics::default(),
            backups: BackupMetrics::default(),
        }
    }
}
