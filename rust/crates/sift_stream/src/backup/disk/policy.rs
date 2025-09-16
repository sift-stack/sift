use std::path::PathBuf;

/// Default maximum backup file size - 500 MiB. Divisible by common file block size of 4096 bytes (4 KiB)
pub const DEFAULT_MAX_BACKUP_SIZE: usize = 500 * 2_usize.pow(20);

/// Default rolling file count - None (unlimited files)
pub const DEFAULT_BACKUP_FILE_COUNT: Option<usize> = None;

/// A policy that is used to configure the disk backup behavior of a Sift stream. Most users wanting disk
/// backups should opt to use the default policy provided by [DiskBackupPolicy::default], however, they are able
/// to completely configure their own.
/// - `backups_dir` is the directory where the backups will get created. If `backups_dir` is
///   `None`, then the user's [data
///   directory](https://docs.rs/dirs/latest/dirs/fn.data_dir.html) is used. If `backups_dir` is provided but
///   doesn't exist, then there will be an attempt to create that directory.
///
/// - `max_backup_file_size` is the maximum size that an individual backup file is allowed to be, befor the
///   file is rolled if using rolling backups, or a checkpoint forced if the max file count is exceeded.
///   Defaults to 500 MiB
///
/// - `rolling_file_policy` is the rolling backup file policy to use
///
/// - `retain_backups` will retain backup files indefinitely, instead of deleting them once a checkpoint
///   has been cleared or the data has otherwise been confirmed to be ingested in Sift.
///
/// **Important Note**: The `max_backup_file_size` does not represent that actual amount of
/// space on disk which is affected by operating system-level compression and block allocation;
/// instead the byte-length is the actual measure.
#[derive(Debug, Clone)]
pub struct DiskBackupPolicy {
    pub backups_dir: Option<PathBuf>,
    pub max_backup_file_size: usize,
    pub rolling_file_policy: RollingFilePolicy,
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

/// A policy that is used to configure the rolling file policy of a Sift stream. Most users wanting disk
/// backups should opt to use the default policy provided by [RollingFilePolicy::default], however, they are able
/// to completely configure their own.
/// - `max_file_count` is the maximum number of files allowed to exist for a backup. Once this count is reached
///   a checkpoint is forced, and the files are either cleared or re-ingested. None signifies unlimited files
#[derive(Debug, Clone)]
pub struct RollingFilePolicy {
    pub max_file_count: Option<usize>,
}

impl Default for RollingFilePolicy {
    fn default() -> Self {
        Self {
            max_file_count: DEFAULT_BACKUP_FILE_COUNT,
        }
    }
}
