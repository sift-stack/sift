#![allow(async_fn_in_trait)]

use prost::Message as PbMessage;
use sift_error::prelude::*;

pub mod disk;
pub(crate) use disk::DiskBackupsManager;

pub mod memory;
pub(crate) use memory::InMemoryBackupsManager;

#[cfg(test)]
mod test;

const BACKUPS_TRANSMISSION_MAX_RETRIES: usize = 10;

pub(crate) trait BackupsManager<T>
where
    T: PbMessage + Default + 'static,
{
    /// Send data point to be backed up.
    async fn send(&mut self, msg: T) -> Result<()>;

    /// Use for graceful termination. This will clean up the backup file.
    async fn finish(self) -> Result<()>;

    /// Clear current set of backups either in disk or in memory.
    async fn clear(&mut self) -> Result<()>;

    /// Notify the backups manager to transmit backups to Sift.
    async fn transmit_backups(&self) -> Result<()>;
}

/// Not intended to be used directly by users.
pub trait BackupsTransmitter<T, I>: Clone
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
    /// Notify the backup manager to transmit backups to Sift.
    Transmit,
    /// Clear current set of backup files.
    Clear,
}
