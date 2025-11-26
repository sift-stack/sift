use pyo3::prelude::*;
use pyo3_stub_gen::derive::*;

use sift_stream::SiftStreamMetricsSnapshot;

#[gen_stub_pyclass]
#[pyclass]
#[derive(Clone)]
pub struct CheckpointMetricsSnapshotPy {
    #[pyo3(get)]
    pub checkpoint_count: u64,
    #[pyo3(get)]
    pub failed_checkpoint_count: u64,
    #[pyo3(get)]
    pub checkpoint_timer_reached_cnt: u64,
    #[pyo3(get)]
    pub checkpoint_manually_reached_cnt: u64,
    #[pyo3(get)]
    pub cur_elapsed_secs: f64,
    #[pyo3(get)]
    pub cur_messages_sent: u64,
    #[pyo3(get)]
    pub cur_message_rate: f64,
    #[pyo3(get)]
    pub cur_bytes_sent: u64,
    #[pyo3(get)]
    pub cur_byte_rate: f64,
}

#[gen_stub_pyclass]
#[pyclass]
#[derive(Clone)]
pub struct BackupMetricsSnapshotPy {
    #[pyo3(get)]
    pub cur_checkpoint_file_count: u64,
    #[pyo3(get)]
    pub cur_checkpoint_cur_file_size: u64,
    #[pyo3(get)]
    pub cur_checkpoint_bytes: u64,
    #[pyo3(get)]
    pub cur_checkpoint_messages: u64,
    #[pyo3(get)]
    pub total_file_count: u64,
    #[pyo3(get)]
    pub total_bytes: u64,
    #[pyo3(get)]
    pub total_messages: u64,
    #[pyo3(get)]
    pub committed_message_id: u64,
    #[pyo3(get)]
    pub queued_checkpoints: u64,
    #[pyo3(get)]
    pub queued_file_ctxs: u64,
    #[pyo3(get)]
    pub files_pending_ingestion: u64,
    #[pyo3(get)]
    pub files_ingested: u64,
    #[pyo3(get)]
    pub cur_ingest_retries: u64,
}

#[gen_stub_pyclass]
#[pyclass]
#[derive(Clone)]
pub struct SiftStreamMetricsSnapshotPy {
    #[pyo3(get)]
    pub elapsed_secs: f64,
    #[pyo3(get)]
    pub loaded_flows: u64,
    #[pyo3(get)]
    pub unique_flows_received: u64,
    #[pyo3(get)]
    pub messages_received: u64,
    #[pyo3(get)]
    pub messages_sent: u64,
    #[pyo3(get)]
    pub message_rate: f64,
    #[pyo3(get)]
    pub bytes_sent: u64,
    #[pyo3(get)]
    pub byte_rate: f64,
    #[pyo3(get)]
    pub messages_sent_to_backup: u64,
    #[pyo3(get)]
    pub old_messages_dropped_for_ingestion: u64,
    #[pyo3(get)]
    pub cur_retry_count: u64,
    #[pyo3(get)]
    pub ingestion_channel_depth: u64,
    #[pyo3(get)]
    pub backup_channel_depth: u64,
    #[pyo3(get)]
    pub checkpoint: CheckpointMetricsSnapshotPy,
    #[pyo3(get)]
    pub backups: BackupMetricsSnapshotPy,
}

impl From<SiftStreamMetricsSnapshot> for SiftStreamMetricsSnapshotPy {
    fn from(snapshot: SiftStreamMetricsSnapshot) -> Self {
        Self {
            elapsed_secs: snapshot.elapsed_secs,
            loaded_flows: snapshot.loaded_flows,
            unique_flows_received: snapshot.unique_flows_received,
            messages_received: snapshot.messages_received,
            messages_sent: snapshot.messages_sent,
            message_rate: snapshot.message_rate,
            bytes_sent: snapshot.bytes_sent,
            byte_rate: snapshot.byte_rate,
            messages_sent_to_backup: snapshot.messages_sent_to_backup,
            old_messages_dropped_for_ingestion: snapshot.old_messages_dropped_for_ingestion,
            cur_retry_count: snapshot.cur_retry_count,
            ingestion_channel_depth: snapshot.ingestion_channel_depth,
            backup_channel_depth: snapshot.backup_channel_depth,
            checkpoint: CheckpointMetricsSnapshotPy {
                checkpoint_count: snapshot.checkpoint.checkpoint_count,
                failed_checkpoint_count: snapshot.checkpoint.failed_checkpoint_count,
                checkpoint_timer_reached_cnt: snapshot.checkpoint.checkpoint_timer_reached_cnt,
                checkpoint_manually_reached_cnt: snapshot
                    .checkpoint
                    .checkpoint_manually_reached_cnt,
                cur_elapsed_secs: snapshot.checkpoint.cur_elapsed_secs,
                cur_messages_sent: snapshot.checkpoint.cur_messages_sent,
                cur_message_rate: snapshot.checkpoint.cur_message_rate,
                cur_bytes_sent: snapshot.checkpoint.cur_bytes_sent,
                cur_byte_rate: snapshot.checkpoint.cur_byte_rate,
            },
            backups: BackupMetricsSnapshotPy {
                cur_checkpoint_file_count: snapshot.backups.cur_checkpoint_file_count,
                cur_checkpoint_cur_file_size: snapshot.backups.cur_checkpoint_cur_file_size,
                cur_checkpoint_bytes: snapshot.backups.cur_checkpoint_bytes,
                cur_checkpoint_messages: snapshot.backups.cur_checkpoint_messages,
                total_file_count: snapshot.backups.total_file_count,
                total_bytes: snapshot.backups.total_bytes,
                total_messages: snapshot.backups.total_messages,
                committed_message_id: snapshot.backups.committed_message_id,
                queued_checkpoints: snapshot.backups.queued_checkpoints,
                queued_file_ctxs: snapshot.backups.queued_file_ctxs,
                files_pending_ingestion: snapshot.backups.files_pending_ingestion,
                files_ingested: snapshot.backups.files_ingested,
                cur_ingest_retries: snapshot.backups.cur_ingest_retries,
            },
        }
    }
}
