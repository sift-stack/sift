use prost::Message as PbMessage;
use sift_error::prelude::*;

pub mod disk;
pub use disk::DiskBackupsManager;

pub mod memory;
pub use memory::InMemoryBackupsManager;

#[cfg(test)]
mod test;

pub(crate) trait BackupsManager<T>
where
    T: PbMessage + Default + 'static,
{
    /// Send data point to be backed up.
    async fn send(&self, msg: T) -> Result<()>;

    /// Truncate the backup.
    async fn truncate_backup(&mut self) -> Result<()>;

    /// Use for graceful termination. This will clean up the backup file.
    async fn finish(self) -> Result<()>;

    /// Retrieve the backup data as an [Iterator].
    async fn get_backup_data(&self) -> Result<impl Iterator<Item = T>>;
}

#[derive(Debug, Clone)]
enum Message<T> {
    /// Data to be backed up.
    Data(T),
    /// Notifies the backup manager that a checkpoint has been reached.
    TruncateBackup,
    /// Graceful termination; cleans up the backup file.
    Complete,
}
