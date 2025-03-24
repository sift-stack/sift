use super::{BackupsManager, Message};
use parking_lot::Mutex;
use prost::Message as PbMessage;
use sift_error::prelude::*;
use std::{collections::VecDeque, sync::Arc};
use tokio::{
    sync::mpsc::{channel, Receiver, Sender},
    task::JoinHandle,
};

/// Default in-memory buffer capacity.
pub const DEFAULT_MAX_BUFFER_SIZE: usize = 10_000;

pub struct InMemoryBackupsManager<T> {
    buffer: Arc<Mutex<VecDeque<T>>>,
    backup_task: JoinHandle<Result<()>>,
    data_tx: Sender<Message<T>>,

    #[allow(dead_code)]
    max_buffer_size: usize,
}

impl<T> InMemoryBackupsManager<T>
where
    T: PbMessage + Default + 'static,
{
    pub fn new(max_buffer_size: Option<usize>) -> Self {
        let (data_tx, data_rx) = channel::<Message<T>>(DEFAULT_MAX_BUFFER_SIZE);
        let max_buffer_size = max_buffer_size.unwrap_or(DEFAULT_MAX_BUFFER_SIZE);
        let buffer = Arc::new(Mutex::new(VecDeque::with_capacity(max_buffer_size)));
        let backup_task = Self::init_backup_task(data_rx, buffer.clone(), max_buffer_size);

        Self {
            max_buffer_size,
            backup_task,
            buffer,
            data_tx,
        }
    }

    fn init_backup_task(
        mut data_rx: Receiver<Message<T>>,
        buffer: Arc<Mutex<VecDeque<T>>>,
        max_buffer_size: usize,
    ) -> JoinHandle<Result<()>> {
        tokio::task::spawn_blocking(move || {
            while let Some(message) = data_rx.blocking_recv() {
                let mut buffer_guard = buffer.lock();

                match message {
                    Message::Data(val) => {
                        if buffer_guard.len() == max_buffer_size {
                            buffer_guard.pop_front();
                        }
                        buffer_guard.push_back(val);
                    }
                    Message::Complete => {
                        #[cfg(feature = "tracing")]
                        tracing::debug!("shutting down backups manager.");
                        break;
                    }
                    Message::TruncateBackup => {
                        #[cfg(feature = "tracing")]
                        tracing::debug!("truncating backup buffer");

                        buffer_guard.clear();
                    }
                }
            }
            Ok(())
        })
    }
}

impl<T> BackupsManager<T> for InMemoryBackupsManager<T>
where
    T: PbMessage + Clone + Default + 'static,
{
    /// Send data point to be backed up.
    async fn send(&self, msg: T) -> Result<()> {
        self.data_tx
            .send(Message::Data(msg))
            .await
            .map_err(|_| {
                Error::new_msg(ErrorKind::BackupsError, "failed to process data to backup")
            })
            .context("back up task may have died")
            .help("please contact Sift")
    }

    /// Clear the backup buffer. Use when a checkpoint has been reached and the current buffer
    /// snapshot is no longer needed.
    async fn truncate_backup(&mut self) -> Result<()> {
        self.data_tx
            .send(Message::TruncateBackup)
            .await
            .map_err(|_| Error::new_msg(ErrorKind::BackupsError, "failed to initiate checkpoint"))
            .help("please contact Sift")?;

        Ok(())
    }

    /// Use to terminate the backup manager
    async fn finish(self) -> Result<()> {
        self.data_tx
            .send(Message::Complete)
            .await
            .map_err(|_| {
                Error::new_msg(
                    ErrorKind::BackupsError,
                    "failed to initiate backups manager shutdown",
                )
            })
            .help("please contact Sift")?;

        self.backup_task
            .await
            .map_err(|e| Error::new(ErrorKind::BackupsError, e))
            .context("failed to join backup task")
            .help("please contact Sift")?
    }

    async fn get_backup_data(&self) -> Result<impl Iterator<Item = T>> {
        let buffer_guard = self.buffer.lock();
        Ok(buffer_guard.clone().into_iter())
    }
}
