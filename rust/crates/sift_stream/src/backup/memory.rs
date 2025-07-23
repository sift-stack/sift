use super::{BackupsManager, Message};
use prost::Message as PbMessage;
use sift_error::prelude::*;
use std::sync::{Arc, Mutex};
use tokio::{
    sync::{
        Notify,
        mpsc::{Receiver, Sender, channel},
    },
    task::JoinHandle,
};

/// Default in-memory buffer capacity.
pub const DEFAULT_MAX_BUFFER_SIZE: usize = 100_000;

/// The buffer size used for the channel that sends and receives data to the backup task
const CHANNEL_BUFFER_SIZE: usize = 10_000;

/// In-memory backup strategy implementation.
pub struct InMemoryBackupsManager<T> {
    buffer: Arc<Mutex<Option<Vec<T>>>>,
    backup_task: Option<JoinHandle<Result<()>>>,
    data_tx: Sender<Message<T>>,
    flush_notification: Arc<Notify>,

    pub(crate) max_buffer_size: usize,
}

impl<T> InMemoryBackupsManager<T>
where
    T: PbMessage + Default + 'static,
{
    pub fn new(max_buffer_size: Option<usize>) -> Self {
        let (data_tx, data_rx) = channel::<Message<T>>(CHANNEL_BUFFER_SIZE);
        let max_buffer_size = max_buffer_size.unwrap_or(DEFAULT_MAX_BUFFER_SIZE);
        let buffer = Arc::new(Mutex::new(None));
        let flush_notification = Arc::new(Notify::new());
        let backup_task = Self::init_backup_task(
            data_rx,
            buffer.clone(),
            max_buffer_size,
            flush_notification.clone(),
        );

        #[cfg(feature = "tracing")]
        tracing::info!(
            max_buffer_size = max_buffer_size,
            "in-memory backup buffer initialized"
        );

        Self {
            max_buffer_size,
            backup_task: Some(backup_task),
            buffer,
            data_tx,
            flush_notification,
        }
    }

    fn init_backup_task(
        mut data_rx: Receiver<Message<T>>,
        buffer: Arc<Mutex<Option<Vec<T>>>>,
        max_buffer_size: usize,
        flush_notifier: Arc<Notify>,
    ) -> JoinHandle<Result<()>> {
        tokio::task::spawn_blocking(move || {
            let mut message_buffer = Vec::with_capacity(max_buffer_size);

            while let Some(message) = data_rx.blocking_recv() {
                match message {
                    Message::Data(val) => {
                        message_buffer.push(val);

                        if message_buffer.len() >= max_buffer_size {
                            if let Ok(mut lock) = buffer.lock() {
                                *lock = Some(message_buffer);
                            }
                            flush_notifier.notify_one();
                            break;
                        }
                    }
                    Message::Flush => {
                        if let Ok(mut lock) = buffer.lock() {
                            *lock = Some(message_buffer);
                        }
                        flush_notifier.notify_one();
                        break;
                    }
                    Message::Complete => {
                        #[cfg(feature = "tracing")]
                        tracing::debug!("shutting down backups manager.");
                        break;
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
    async fn send(&mut self, msg: T) -> Result<()> {
        match self.data_tx.send(Message::Data(msg)).await {
            Ok(_) => Ok(()),

            // Backup task has shutdown due to max buffer size being reached.
            Err(_) => {
                let Some(backup_task) = self.backup_task.take() else {
                    return Ok(());
                };
                match backup_task.await {
                    Ok(res) => match res {
                        Ok(_) => Err(Error::new_msg(
                            ErrorKind::BackupLimitReached,
                            "backup limit reached",
                        )),
                        Err(err) => Err(Error::new(ErrorKind::BackupsError, err))
                            .context("backup task encountered an error")
                            .help("please notify Sift"),
                    },
                    Err(err) => Err(Error::new(ErrorKind::BackupsError, err))
                        .context("error waiting for backup task to finish")
                        .help("please notify Sift"),
                }
            }
        }
    }

    /// Use to terminate the backup manager
    async fn finish(mut self) -> Result<()> {
        let _ = self.data_tx.send(Message::Complete).await;

        if let Some(backup_task) = self.backup_task.take() {
            backup_task
                .await
                .map_err(|e| Error::new(ErrorKind::BackupsError, e))
                .context("failed to join in-memory backup task")
                .help("please contact Sift")??;
        }
        Ok(())
    }

    async fn transmit_backups(&self) {
        let _ = self.data_tx.send(Message::Flush).await;
        self.flush_notification.notified().await;
    }
}
