use prost::Message as PbMessage;
use sift_error::prelude::*;

pub mod disk;
pub use disk::{AsyncBackupsManager, DiskBackupPolicy, DiskBackupsManager};

pub mod memory;
pub use memory::InMemoryBackupsManager;

#[cfg(test)]
mod test;

pub(crate) trait BackupsManager<T>
where
    T: PbMessage + Default + 'static,
{
    /// Send data point to be backed up.
    async fn send(&mut self, msg: T) -> Result<()>;

    /// Use for graceful termination. This will clean up the backup file.
    async fn finish(self) -> Result<()>;

    /// Retrieve the backup data as an [Iterator].
    async fn get_backup_data(&mut self) -> Result<impl Iterator<Item = Result<T>>>;
}

#[derive(Debug, Clone)]
enum Message<T> {
    /// Data to be backed up.
    Data(T),
    /// Graceful termination; cleans up the backup file.
    Complete,
    /// Force the backup task to flush its contents to the target data container.
    Flush,
}
