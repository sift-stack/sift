use std::{
    sync::atomic::{AtomicU64, Ordering},
    time::{Duration, SystemTime, UNIX_EPOCH},
};

pub struct StreamingStats {
    pub elapsed_secs: f64,
    pub messages_sent: u64,
    pub message_rate: f64,
    pub bytes_sent: u64,
    pub byte_rate: f64,
}

impl StreamingStats {
    pub(crate) fn calculate(start_time_ms: u64, messages_sent: u64, bytes_sent: u64) -> StreamingStats {
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

#[derive(Default)]
pub(crate) struct U64Counter(AtomicU64);

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

#[derive(Default)]
pub(crate) struct U64Signal(AtomicU64);

impl U64Signal {
    pub fn set(&self, val: u64) {
        self.0.store(val, Ordering::Relaxed);
    }

    pub fn add(&self, val: u64) -> u64 {
        self.0.fetch_add(val, Ordering::Relaxed)
    }

    // pub fn get(&self) -> u64 {
    //     self.0.load(Ordering::Relaxed)
    // }
}

#[derive(Default)]
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

#[derive(Default)]
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
    pub fn next_checkpoint(&self) {
        self.checkpoint_count.increment();
        self.cur_bytes_sent.reset();
        self.cur_messages_sent.reset();
        self.checkpoint_start_time_epoch_ms.store(SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_else(|_| {
                #[cfg(feature = "tracing")]
                tracing::warn!("System time was before unix epoch");
                Duration::default()
            })
            .as_millis() as u64,
            Ordering::Relaxed
        );
    }
}

#[derive(Default)]
pub struct SiftStreamMetrics {
    creation_time_epoch_ms: u64,
    pub(crate) loaded_flows: U64Counter,
    pub(crate) unique_flows_received: U64Counter,
    pub(crate) messages_received: U64Counter,
    pub(crate) messages_sent: U64Counter,
    pub(crate) bytes_sent: U64Counter,
    pub(crate) checkpoint: CheckpointMetrics,
    pub(crate) messages_sent_to_backup: U64Counter,
    pub(crate) cur_retry_count: U64Signal,
    pub(crate) backups: BackupMetrics,
}


impl SiftStreamMetrics {
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

    pub fn get_stream_stats(&self) -> StreamingStats {
        // TODO: Look into other ordering to try and avoid atomics writes until all three of these are complete
        let start_time_ms = self.creation_time_epoch_ms;
        let messages_sent = self.messages_sent.0.load(Ordering::Relaxed);
        let bytes_sent = self.bytes_sent.0.load(Ordering::Relaxed);

        StreamingStats::calculate(start_time_ms, messages_sent, bytes_sent)
    }

    pub fn get_checkpoint_stats(&self) -> StreamingStats {
        // TODO: Look into other ordering to try and avoid atomics writes until all three of these are complete
        let start_time_ms = self.checkpoint.checkpoint_start_time_epoch_ms.load(Ordering::Relaxed);
        let messages_sent = self.checkpoint.cur_messages_sent.0.load(Ordering::Relaxed);
        let bytes_sent = self.checkpoint.cur_bytes_sent.0.load(Ordering::Relaxed);

        StreamingStats::calculate(start_time_ms, messages_sent, bytes_sent)
    }
}
