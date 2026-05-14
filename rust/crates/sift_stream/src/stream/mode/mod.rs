#[cfg(feature = "unstable")]
pub mod bench;

#[cfg(test)]
mod test;

/// Concerned with ingestion-config-based streaming.
pub mod ingestion_config;

/// Concerned with file-backup-based streaming.
pub mod file_backup;

/// Transport for live streaming without disk backups.
pub mod live_only;

/// Transport for live streaming with disk backups and checkpointing.
pub mod live_with_backups;
