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

use sift_error::prelude::*;
use sift_rs::{common::r#type::v1::ChannelDataType, ingestion_configs::v2::ChannelConfig};

use crate::stream::flow::{ChannelIndex, FlowBuilder, FlowDescriptor};

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
    /// Count of stream completions per gRPC status code, indexed by code value (0=Ok .. 16=Unauthenticated, 17=UnknownGrpcCode)
    pub grpc_status_counts: [u64; 18],
    /// Depth of the ingestion channel
    pub ingestion_channel_depth: u64,
    /// Depth of the backup channel
    pub backup_channel_depth: u64,
    /// Total log events dropped because the log channel was full
    pub logs_dropped_channel_full: u64,
    /// Current depth of the log event channel
    pub log_channel_depth: u64,
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
    pub(crate) grpc_status_counts: [U64Counter; 18],
    pub(crate) ingestion_channel_depth: U64Signal,
    pub(crate) backup_channel_depth: U64Signal,
    pub(crate) logs_dropped_channel_full: U64Counter,
    pub(crate) log_channel_depth: U64Signal,
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
        let logs_dropped_channel_full = self.logs_dropped_channel_full.get();
        let log_channel_depth = self.log_channel_depth.get();

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
            grpc_status_counts: std::array::from_fn(|i| self.grpc_status_counts[i].get()),
            ingestion_channel_depth,
            backup_channel_depth,
            logs_dropped_channel_full,
            log_channel_depth,
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
                name: format!("{channel_prefix}.grpc_status_counts.ok"),
                description: "gRPC status code Ok (0) count".into(),
                data_type: ChannelDataType::Uint64.into(),
                ..Default::default()
            },
            ChannelConfig {
                name: format!("{channel_prefix}.grpc_status_counts.cancelled"),
                description: "gRPC status code Cancelled (1) count".into(),
                data_type: ChannelDataType::Uint64.into(),
                ..Default::default()
            },
            ChannelConfig {
                name: format!("{channel_prefix}.grpc_status_counts.unknown"),
                description: "gRPC status code Unknown (2) count".into(),
                data_type: ChannelDataType::Uint64.into(),
                ..Default::default()
            },
            ChannelConfig {
                name: format!("{channel_prefix}.grpc_status_counts.invalid_argument"),
                description: "gRPC status code InvalidArgument (3) count".into(),
                data_type: ChannelDataType::Uint64.into(),
                ..Default::default()
            },
            ChannelConfig {
                name: format!("{channel_prefix}.grpc_status_counts.deadline_exceeded"),
                description: "gRPC status code DeadlineExceeded (4) count".into(),
                data_type: ChannelDataType::Uint64.into(),
                ..Default::default()
            },
            ChannelConfig {
                name: format!("{channel_prefix}.grpc_status_counts.not_found"),
                description: "gRPC status code NotFound (5) count".into(),
                data_type: ChannelDataType::Uint64.into(),
                ..Default::default()
            },
            ChannelConfig {
                name: format!("{channel_prefix}.grpc_status_counts.already_exists"),
                description: "gRPC status code AlreadyExists (6) count".into(),
                data_type: ChannelDataType::Uint64.into(),
                ..Default::default()
            },
            ChannelConfig {
                name: format!("{channel_prefix}.grpc_status_counts.permission_denied"),
                description: "gRPC status code PermissionDenied (7) count".into(),
                data_type: ChannelDataType::Uint64.into(),
                ..Default::default()
            },
            ChannelConfig {
                name: format!("{channel_prefix}.grpc_status_counts.resource_exhausted"),
                description: "gRPC status code ResourceExhausted (8) count".into(),
                data_type: ChannelDataType::Uint64.into(),
                ..Default::default()
            },
            ChannelConfig {
                name: format!("{channel_prefix}.grpc_status_counts.failed_precondition"),
                description: "gRPC status code FailedPrecondition (9) count".into(),
                data_type: ChannelDataType::Uint64.into(),
                ..Default::default()
            },
            ChannelConfig {
                name: format!("{channel_prefix}.grpc_status_counts.aborted"),
                description: "gRPC status code Aborted (10) count".into(),
                data_type: ChannelDataType::Uint64.into(),
                ..Default::default()
            },
            ChannelConfig {
                name: format!("{channel_prefix}.grpc_status_counts.out_of_range"),
                description: "gRPC status code OutOfRange (11) count".into(),
                data_type: ChannelDataType::Uint64.into(),
                ..Default::default()
            },
            ChannelConfig {
                name: format!("{channel_prefix}.grpc_status_counts.unimplemented"),
                description: "gRPC status code Unimplemented (12) count".into(),
                data_type: ChannelDataType::Uint64.into(),
                ..Default::default()
            },
            ChannelConfig {
                name: format!("{channel_prefix}.grpc_status_counts.internal"),
                description: "gRPC status code Internal (13) count".into(),
                data_type: ChannelDataType::Uint64.into(),
                ..Default::default()
            },
            ChannelConfig {
                name: format!("{channel_prefix}.grpc_status_counts.unavailable"),
                description: "gRPC status code Unavailable (14) count".into(),
                data_type: ChannelDataType::Uint64.into(),
                ..Default::default()
            },
            ChannelConfig {
                name: format!("{channel_prefix}.grpc_status_counts.data_loss"),
                description: "gRPC status code DataLoss (15) count".into(),
                data_type: ChannelDataType::Uint64.into(),
                ..Default::default()
            },
            ChannelConfig {
                name: format!("{channel_prefix}.grpc_status_counts.unauthenticated"),
                description: "gRPC status code Unauthenticated (16) count".into(),
                data_type: ChannelDataType::Uint64.into(),
                ..Default::default()
            },
            ChannelConfig {
                name: format!("{channel_prefix}.grpc_status_counts.unknown_grpc_code"),
                description: "Unknown gRPC status code (>16) count".into(),
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
                name: format!("{channel_prefix}.logs_dropped_channel_full"),
                description: "Log events dropped because the log channel was full".into(),
                data_type: ChannelDataType::Uint64.into(),
                ..Default::default()
            },
            ChannelConfig {
                name: format!("{channel_prefix}.log_channel_depth"),
                description: "Current depth of the log event channel".into(),
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

    /// Populates a [`FlowBuilder`] with the current snapshot values using pre-resolved
    /// [`MetricsFlowIndices`], avoiding per-call string allocations and hash lookups.
    pub(crate) fn populate_flow(
        &self,
        indices: &MetricsFlowIndices,
        builder: &mut FlowBuilder<'_, String>,
    ) -> Result<()> {
        builder.set(indices.elapsed_secs, self.elapsed_secs)?;
        builder.set(indices.loaded_flows, self.loaded_flows)?;
        builder.set(indices.unique_flows_received, self.unique_flows_received)?;
        builder.set(indices.messages_received, self.messages_received)?;
        builder.set(indices.messages_sent, self.messages_sent)?;
        builder.set(indices.message_rate, self.message_rate)?;
        builder.set(indices.bytes_sent, self.bytes_sent)?;
        builder.set(indices.byte_rate, self.byte_rate)?;
        builder.set(
            indices.messages_sent_to_backup,
            self.messages_sent_to_backup,
        )?;
        builder.set(
            indices.old_messages_dropped_for_ingestion,
            self.old_messages_dropped_for_ingestion,
        )?;
        builder.set(
            indices.old_messages_failed_adding_to_backup,
            self.old_messages_failed_adding_to_backup,
        )?;
        builder.set(indices.cur_retry_count, self.cur_retry_count)?;
        builder.set(indices.grpc_status_counts.ok, self.grpc_status_counts[0])?;
        builder.set(
            indices.grpc_status_counts.cancelled,
            self.grpc_status_counts[1],
        )?;
        builder.set(
            indices.grpc_status_counts.unknown,
            self.grpc_status_counts[2],
        )?;
        builder.set(
            indices.grpc_status_counts.invalid_argument,
            self.grpc_status_counts[3],
        )?;
        builder.set(
            indices.grpc_status_counts.deadline_exceeded,
            self.grpc_status_counts[4],
        )?;
        builder.set(
            indices.grpc_status_counts.not_found,
            self.grpc_status_counts[5],
        )?;
        builder.set(
            indices.grpc_status_counts.already_exists,
            self.grpc_status_counts[6],
        )?;
        builder.set(
            indices.grpc_status_counts.permission_denied,
            self.grpc_status_counts[7],
        )?;
        builder.set(
            indices.grpc_status_counts.resource_exhausted,
            self.grpc_status_counts[8],
        )?;
        builder.set(
            indices.grpc_status_counts.failed_precondition,
            self.grpc_status_counts[9],
        )?;
        builder.set(
            indices.grpc_status_counts.aborted,
            self.grpc_status_counts[10],
        )?;
        builder.set(
            indices.grpc_status_counts.out_of_range,
            self.grpc_status_counts[11],
        )?;
        builder.set(
            indices.grpc_status_counts.unimplemented,
            self.grpc_status_counts[12],
        )?;
        builder.set(
            indices.grpc_status_counts.internal,
            self.grpc_status_counts[13],
        )?;
        builder.set(
            indices.grpc_status_counts.unavailable,
            self.grpc_status_counts[14],
        )?;
        builder.set(
            indices.grpc_status_counts.data_loss,
            self.grpc_status_counts[15],
        )?;
        builder.set(
            indices.grpc_status_counts.unauthenticated,
            self.grpc_status_counts[16],
        )?;
        builder.set(
            indices.grpc_status_counts.unknown_grpc_code,
            self.grpc_status_counts[17],
        )?;
        builder.set(
            indices.ingestion_channel_depth,
            self.ingestion_channel_depth,
        )?;
        builder.set(indices.backup_channel_depth, self.backup_channel_depth)?;
        builder.set(
            indices.logs_dropped_channel_full,
            self.logs_dropped_channel_full,
        )?;
        builder.set(indices.log_channel_depth, self.log_channel_depth)?;
        builder.set(indices.checkpoint.count, self.checkpoint.checkpoint_count)?;
        builder.set(
            indices.checkpoint.failed_count,
            self.checkpoint.failed_checkpoint_count,
        )?;
        builder.set(
            indices.checkpoint.timer_reached_count,
            self.checkpoint.checkpoint_timer_reached_cnt,
        )?;
        builder.set(
            indices.checkpoint.manually_reached_count,
            self.checkpoint.checkpoint_manually_reached_cnt,
        )?;
        builder.set(
            indices.checkpoint.cur_elapsed_secs,
            self.checkpoint.cur_elapsed_secs,
        )?;
        builder.set(
            indices.checkpoint.cur_messages_sent,
            self.checkpoint.cur_messages_sent,
        )?;
        builder.set(
            indices.checkpoint.cur_message_rate,
            self.checkpoint.cur_message_rate,
        )?;
        builder.set(
            indices.checkpoint.cur_bytes_sent,
            self.checkpoint.cur_bytes_sent,
        )?;
        builder.set(
            indices.checkpoint.cur_byte_rate,
            self.checkpoint.cur_byte_rate,
        )?;
        builder.set(
            indices.backups.cur_checkpoint_file_count,
            self.backups.cur_checkpoint_file_count,
        )?;
        builder.set(
            indices.backups.cur_checkpoint_cur_file_size,
            self.backups.cur_checkpoint_cur_file_size,
        )?;
        builder.set(
            indices.backups.cur_checkpoint_bytes,
            self.backups.cur_checkpoint_bytes,
        )?;
        builder.set(
            indices.backups.cur_checkpoint_messages,
            self.backups.cur_checkpoint_messages,
        )?;
        builder.set(
            indices.backups.total_file_count,
            self.backups.total_file_count,
        )?;
        builder.set(indices.backups.total_bytes, self.backups.total_bytes)?;
        builder.set(indices.backups.total_messages, self.backups.total_messages)?;
        builder.set(
            indices.backups.files_pending_ingestion,
            self.backups.files_pending_ingestion,
        )?;
        builder.set(indices.backups.files_ingested, self.backups.files_ingested)?;
        builder.set(
            indices.backups.cur_ingest_retries,
            self.backups.cur_ingest_retries,
        )?;
        Ok(())
    }
}

/// Pre-resolved [`ChannelIndex`] values for gRPC status count channels.
pub(crate) struct MetricsGrpcStatusIndices {
    pub ok: ChannelIndex,
    pub cancelled: ChannelIndex,
    pub unknown: ChannelIndex,
    pub invalid_argument: ChannelIndex,
    pub deadline_exceeded: ChannelIndex,
    pub not_found: ChannelIndex,
    pub already_exists: ChannelIndex,
    pub permission_denied: ChannelIndex,
    pub resource_exhausted: ChannelIndex,
    pub failed_precondition: ChannelIndex,
    pub aborted: ChannelIndex,
    pub out_of_range: ChannelIndex,
    pub unimplemented: ChannelIndex,
    pub internal: ChannelIndex,
    pub unavailable: ChannelIndex,
    pub data_loss: ChannelIndex,
    pub unauthenticated: ChannelIndex,
    pub unknown_grpc_code: ChannelIndex,
}

/// Pre-resolved [`ChannelIndex`] values for checkpoint metric channels.
pub(crate) struct MetricsCheckpointFlowIndices {
    pub count: ChannelIndex,
    pub failed_count: ChannelIndex,
    pub timer_reached_count: ChannelIndex,
    pub manually_reached_count: ChannelIndex,
    pub cur_elapsed_secs: ChannelIndex,
    pub cur_messages_sent: ChannelIndex,
    pub cur_message_rate: ChannelIndex,
    pub cur_bytes_sent: ChannelIndex,
    pub cur_byte_rate: ChannelIndex,
}

/// Pre-resolved [`ChannelIndex`] values for backup metric channels.
pub(crate) struct MetricsBackupFlowIndices {
    pub cur_checkpoint_file_count: ChannelIndex,
    pub cur_checkpoint_cur_file_size: ChannelIndex,
    pub cur_checkpoint_bytes: ChannelIndex,
    pub cur_checkpoint_messages: ChannelIndex,
    pub total_file_count: ChannelIndex,
    pub total_bytes: ChannelIndex,
    pub total_messages: ChannelIndex,
    pub files_pending_ingestion: ChannelIndex,
    pub files_ingested: ChannelIndex,
    pub cur_ingest_retries: ChannelIndex,
}

/// Pre-resolved [`ChannelIndex`] values for all metrics channels, built once from the
/// [`FlowDescriptor`] after stream initialization. Used with [`FlowBuilder`] to populate
/// a metrics flow without per-call string allocations or hash lookups.
pub(crate) struct MetricsFlowIndices {
    pub elapsed_secs: ChannelIndex,
    pub loaded_flows: ChannelIndex,
    pub unique_flows_received: ChannelIndex,
    pub messages_received: ChannelIndex,
    pub messages_sent: ChannelIndex,
    pub message_rate: ChannelIndex,
    pub bytes_sent: ChannelIndex,
    pub byte_rate: ChannelIndex,
    pub messages_sent_to_backup: ChannelIndex,
    pub old_messages_dropped_for_ingestion: ChannelIndex,
    pub old_messages_failed_adding_to_backup: ChannelIndex,
    pub cur_retry_count: ChannelIndex,
    pub grpc_status_counts: MetricsGrpcStatusIndices,
    pub ingestion_channel_depth: ChannelIndex,
    pub backup_channel_depth: ChannelIndex,
    pub logs_dropped_channel_full: ChannelIndex,
    pub log_channel_depth: ChannelIndex,
    pub checkpoint: MetricsCheckpointFlowIndices,
    pub backups: MetricsBackupFlowIndices,
}

impl MetricsFlowIndices {
    pub(crate) fn new(descriptor: &FlowDescriptor<String>, prefix: &str) -> Result<Self> {
        let m = descriptor.mapping();

        macro_rules! idx {
            ($suffix:literal) => {{
                let key = format!("{prefix}.{}", $suffix);
                *m.get(&key).ok_or_else(|| {
                    Error::new_msg(
                        ErrorKind::NotFoundError,
                        format!("metrics channel '{key}' not found in flow descriptor"),
                    )
                })?
            }};
        }

        Ok(Self {
            elapsed_secs: idx!("elapsed_secs"),
            loaded_flows: idx!("loaded_flows"),
            unique_flows_received: idx!("unique_flows_received"),
            messages_received: idx!("messages_received"),
            messages_sent: idx!("messages_sent"),
            message_rate: idx!("message_rate"),
            bytes_sent: idx!("bytes_sent"),
            byte_rate: idx!("byte_rate"),
            messages_sent_to_backup: idx!("messages_sent_to_backup"),
            old_messages_dropped_for_ingestion: idx!("old_messages_dropped_for_ingestion"),
            old_messages_failed_adding_to_backup: idx!("old_messages_failed_adding_to_backup"),
            cur_retry_count: idx!("cur_retry_count"),
            grpc_status_counts: MetricsGrpcStatusIndices {
                ok: idx!("grpc_status_counts.ok"),
                cancelled: idx!("grpc_status_counts.cancelled"),
                unknown: idx!("grpc_status_counts.unknown"),
                invalid_argument: idx!("grpc_status_counts.invalid_argument"),
                deadline_exceeded: idx!("grpc_status_counts.deadline_exceeded"),
                not_found: idx!("grpc_status_counts.not_found"),
                already_exists: idx!("grpc_status_counts.already_exists"),
                permission_denied: idx!("grpc_status_counts.permission_denied"),
                resource_exhausted: idx!("grpc_status_counts.resource_exhausted"),
                failed_precondition: idx!("grpc_status_counts.failed_precondition"),
                aborted: idx!("grpc_status_counts.aborted"),
                out_of_range: idx!("grpc_status_counts.out_of_range"),
                unimplemented: idx!("grpc_status_counts.unimplemented"),
                internal: idx!("grpc_status_counts.internal"),
                unavailable: idx!("grpc_status_counts.unavailable"),
                data_loss: idx!("grpc_status_counts.data_loss"),
                unauthenticated: idx!("grpc_status_counts.unauthenticated"),
                unknown_grpc_code: idx!("grpc_status_counts.unknown_grpc_code"),
            },
            ingestion_channel_depth: idx!("ingestion_channel_depth"),
            backup_channel_depth: idx!("backup_channel_depth"),
            logs_dropped_channel_full: idx!("logs_dropped_channel_full"),
            log_channel_depth: idx!("log_channel_depth"),
            checkpoint: MetricsCheckpointFlowIndices {
                count: idx!("checkpoint.count"),
                failed_count: idx!("checkpoint.failed_count"),
                timer_reached_count: idx!("checkpoint.timer_reached_count"),
                manually_reached_count: idx!("checkpoint.manually_reached_count"),
                cur_elapsed_secs: idx!("checkpoint.cur_elapsed_secs"),
                cur_messages_sent: idx!("checkpoint.cur_messages_sent"),
                cur_message_rate: idx!("checkpoint.cur_message_rate"),
                cur_bytes_sent: idx!("checkpoint.cur_bytes_sent"),
                cur_byte_rate: idx!("checkpoint.cur_byte_rate"),
            },
            backups: MetricsBackupFlowIndices {
                cur_checkpoint_file_count: idx!("backups.cur_checkpoint_file_count"),
                cur_checkpoint_cur_file_size: idx!("backups.cur_checkpoint_cur_file_size"),
                cur_checkpoint_bytes: idx!("backups.cur_checkpoint_bytes"),
                cur_checkpoint_messages: idx!("backups.cur_checkpoint_messages"),
                total_file_count: idx!("backups.total_file_count"),
                total_bytes: idx!("backups.total_bytes"),
                total_messages: idx!("backups.total_messages"),
                files_pending_ingestion: idx!("backups.files_pending_ingestion"),
                files_ingested: idx!("backups.files_ingested"),
                cur_ingest_retries: idx!("backups.cur_ingest_retries"),
            },
        })
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
            grpc_status_counts: Default::default(),
            ingestion_channel_depth: U64Signal::default(),
            backup_channel_depth: U64Signal::default(),
            logs_dropped_channel_full: U64Counter::default(),
            log_channel_depth: U64Signal::default(),
            checkpoint: CheckpointMetrics::default(),
            backups: BackupMetrics::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{TimeValue, stream::flow::FlowBuilder};
    use sift_rs::{
        ingest::v1::ingest_with_config_data_channel_value::Type, ingestion_configs::v2::FlowConfig,
    };

    fn build_test_descriptor(prefix: &str) -> FlowDescriptor<String> {
        let flow_config = FlowConfig {
            name: "test-flow".to_string(),
            channels: SiftStreamMetricsSnapshot::channel_configs(prefix),
        };
        FlowDescriptor::try_from(("test-ic-id", flow_config)).unwrap()
    }

    /// Verifies that MetricsFlowIndices::new succeeds when the descriptor is built from
    /// channel_configs — i.e. every channel name resolves to a valid index.
    #[test]
    fn test_metrics_flow_indices_new_succeeds() {
        let descriptor = build_test_descriptor("test");
        assert!(MetricsFlowIndices::new(&descriptor, "test").is_ok());
    }

    /// Verifies that populate_flow maps every snapshot field to the correct channel slot.
    ///
    /// Each metric value is set to a distinct number so that a mis-mapping (two fields swapped)
    /// or a missing call (channel left as Empty) will cause the test to fail.
    #[test]
    fn test_populate_flow_maps_all_values_correctly() {
        let prefix = "test";
        let descriptor = build_test_descriptor(prefix);
        let indices = MetricsFlowIndices::new(&descriptor, prefix).unwrap();

        let snapshot = SiftStreamMetricsSnapshot {
            elapsed_secs: 1.0,
            loaded_flows: 2,
            unique_flows_received: 3,
            messages_received: 4,
            messages_sent: 5,
            message_rate: 6.0,
            bytes_sent: 7,
            byte_rate: 8.0,
            messages_sent_to_backup: 9,
            old_messages_dropped_for_ingestion: 10,
            old_messages_failed_adding_to_backup: 11,
            cur_retry_count: 12,
            // grpc_status_counts[0..=17] => 13..=30
            grpc_status_counts: [
                13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30,
            ],
            ingestion_channel_depth: 30,
            backup_channel_depth: 31,
            logs_dropped_channel_full: 32,
            log_channel_depth: 33,
            checkpoint: CheckpointMetricsSnapshot {
                checkpoint_count: 32,
                failed_checkpoint_count: 33,
                checkpoint_timer_reached_cnt: 34,
                checkpoint_manually_reached_cnt: 35,
                cur_elapsed_secs: 36.0,
                cur_messages_sent: 37,
                cur_message_rate: 38.0,
                cur_bytes_sent: 39,
                cur_byte_rate: 40.0,
            },
            backups: BackupMetricsSnapshot {
                cur_checkpoint_file_count: 41,
                cur_checkpoint_cur_file_size: 42,
                cur_checkpoint_bytes: 43,
                cur_checkpoint_messages: 44,
                total_file_count: 45,
                total_bytes: 46,
                total_messages: 47,
                committed_message_id: 0,
                queued_checkpoints: 0,
                queued_file_ctxs: 0,
                files_pending_ingestion: 48,
                files_ingested: 49,
                cur_ingest_retries: 50,
            },
        };

        let mut builder = FlowBuilder::new(&descriptor);
        snapshot.populate_flow(&indices, &mut builder).unwrap();
        let req = builder.request(TimeValue::now());

        // Channel count must match channel_configs exactly.
        let expected_count = SiftStreamMetricsSnapshot::channel_configs(prefix).len();
        assert_eq!(req.channel_values.len(), expected_count);

        // No channel should be left as Empty — catches any missing populate_flow call.
        let m = descriptor.mapping();
        let empty: Vec<&str> = m
            .iter()
            .filter(|(_, idx)| {
                matches!(
                    req.channel_values[idx.as_usize()].r#type,
                    Some(Type::Empty(_))
                )
            })
            .map(|(name, _)| name.as_str())
            .collect();
        assert!(empty.is_empty(), "channels left unpopulated: {empty:?}");

        // Spot-check representative fields from each logical group.
        macro_rules! assert_val {
            ($key:expr, $expected:expr) => {
                assert_eq!(
                    req.channel_values[m[&format!("{prefix}.{}", $key)].as_usize()].r#type,
                    Some($expected),
                    "wrong value for channel '{prefix}.{}'",
                    $key,
                )
            };
        }

        assert_val!("elapsed_secs", Type::Double(1.0));
        assert_val!("loaded_flows", Type::Uint64(2));
        assert_val!("message_rate", Type::Double(6.0));
        assert_val!("byte_rate", Type::Double(8.0));
        assert_val!("cur_retry_count", Type::Uint64(12));
        assert_val!("grpc_status_counts.ok", Type::Uint64(13));
        assert_val!("grpc_status_counts.unauthenticated", Type::Uint64(29));
        assert_val!("ingestion_channel_depth", Type::Uint64(30));
        assert_val!("backup_channel_depth", Type::Uint64(31));
        assert_val!("logs_dropped_channel_full", Type::Uint64(32));
        assert_val!("log_channel_depth", Type::Uint64(33));
        assert_val!("checkpoint.count", Type::Uint64(32));
        assert_val!("checkpoint.cur_elapsed_secs", Type::Double(36.0));
        assert_val!("checkpoint.cur_message_rate", Type::Double(38.0));
        assert_val!("backups.cur_checkpoint_file_count", Type::Uint64(41));
        assert_val!("backups.files_ingested", Type::Uint64(49));
        assert_val!("backups.cur_ingest_retries", Type::Uint64(50));
    }
}
