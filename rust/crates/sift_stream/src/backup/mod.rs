use prost::Message as PbMessage;
use sift_error::prelude::*;

pub mod disk;
pub(crate) use disk::DiskBackupsManager;

pub mod memory;
pub(crate) use memory::InMemoryBackupsManager;

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

    async fn transmit_backups(&self);
}

pub(crate) trait BackupsTransmitter<T, I>
where
    T: PbMessage + Default + 'static,
    I: IntoIterator<Item = T>,
    <I as IntoIterator>::IntoIter: Send,
{
    async fn transmit(&mut self, stream: I) -> Result<()>;
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
