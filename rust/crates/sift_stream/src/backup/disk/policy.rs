use std::path::PathBuf;

/// Default maximum backup file size - 500 MiB. Divisible by common file block size of 4096 bytes (4 KiB)
pub const DEFAULT_MAX_BACKUP_SIZE: usize = 500 * 2_usize.pow(20);

/// Default rolling file count - None (unlimited files)
pub const DEFAULT_BACKUP_FILE_COUNT: Option<usize> = None;

/// Configures the disk backup behavior for a [`LiveStreamingWithBackups`](crate::LiveStreamingWithBackups)
/// or [`FileBackup`](crate::FileBackup) stream.
///
/// Most users should start with [`DiskBackupPolicy::default`] and override only the fields
/// that need to change.
#[derive(Debug, Clone)]
pub struct DiskBackupPolicy {
    /// Directory in which backup files are created.
    ///
    /// If `None`, the platform's [user data directory](https://docs.rs/dirs/latest/dirs/fn.data_dir.html)
    /// is used. If a path is provided but does not exist, an attempt is made to create it.
    ///
    /// For [`FileBackup`](crate::FileBackup) this field must be `Some` or [`build`](crate::FileBackupBuilder::build) will return an error.
    pub backups_dir: Option<PathBuf>,
    /// Maximum uncompressed byte length of a single backup file before it is rolled.
    ///
    /// When this threshold is reached the current file is closed and a new one is opened. If
    /// the rolling file count limit is also reached, a checkpoint is forced instead.
    /// Defaults to [`DEFAULT_MAX_BACKUP_SIZE`] (500 MiB).
    ///
    /// **Note**: This is the raw byte length of the encoded data, not the amount of space
    /// consumed on disk (which is affected by OS-level compression and block allocation).
    pub max_backup_file_size: usize,
    /// Policy governing how many rolling backup files are retained at once.
    pub rolling_file_policy: RollingFilePolicy,
    /// When `true`, backup files are retained indefinitely rather than being deleted after
    /// a successful checkpoint or confirmed re-ingestion.
    pub retain_backups: bool,
}

impl Default for DiskBackupPolicy {
    fn default() -> Self {
        Self {
            backups_dir: Default::default(),
            max_backup_file_size: DEFAULT_MAX_BACKUP_SIZE,
            rolling_file_policy: Default::default(),
            retain_backups: false,
        }
    }
}

/// Configures the rolling file behavior within a [`DiskBackupPolicy`].
///
/// Most users should start with [`RollingFilePolicy::default`] (unlimited files).
#[derive(Debug, Clone)]
pub struct RollingFilePolicy {
    /// Maximum number of backup files that may exist simultaneously.
    ///
    /// Once this limit is reached a checkpoint is forced; files are then either deleted or
    /// re-ingested depending on the checkpoint outcome and [`DiskBackupPolicy::retain_backups`].
    /// `None` means unlimited files (the default).
    pub max_file_count: Option<usize>,
}

impl Default for RollingFilePolicy {
    fn default() -> Self {
        Self {
            max_file_count: DEFAULT_BACKUP_FILE_COUNT,
        }
    }
}
