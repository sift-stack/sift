use crate::backup::BACKUPS_TRANSMISSION_MAX_RETRIES;

use super::{BackupsManager, BackupsTransmitter, Message};
use prost::Message as PbMessage;
use sift_error::prelude::*;
use std::{
    marker::PhantomData,
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
    thread::sleep,
    time::Duration,
};
use tokio::{
    runtime::Handle,
    sync::{
        Notify,
        mpsc::{UnboundedReceiver, UnboundedSender, unbounded_channel},
    },
    task::JoinHandle,
};

/// Default in-memory buffer capacity.
pub const DEFAULT_MAX_BUFFER_SIZE: usize = 100_000;

/// In-memory backup strategy implementation.
pub struct InMemoryBackupsManager<T, U> {
    backups_full: Arc<AtomicBool>,
    backup_task: Option<JoinHandle<Result<()>>>,
    data_tx: UnboundedSender<Message<T>>,
    clear_done: Arc<Notify>,
    transmitter: PhantomData<U>,
}

impl<T, U> InMemoryBackupsManager<T, U>
where
    T: PbMessage + Clone + Default + 'static,
    U: BackupsTransmitter<T, Vec<T>> + 'static + Send,
{
    pub fn new(max_buffer_size: Option<usize>, transmitter: U) -> Self {
        let (data_tx, data_rx) = unbounded_channel::<Message<T>>();
        let max_buffer_size = max_buffer_size.unwrap_or(DEFAULT_MAX_BUFFER_SIZE);
        let clear_done = Arc::new(Notify::new());
        let backups_full = Arc::new(AtomicBool::default());
        let backup_task = Self::init_backup_task(
            data_rx,
            max_buffer_size,
            clear_done.clone(),
            backups_full.clone(),
            transmitter,
        );

        #[cfg(feature = "tracing")]
        tracing::info!(
            max_buffer_size = max_buffer_size,
            "in-memory backup buffer initialized"
        );

        Self {
            backups_full,
            backup_task: Some(backup_task),
            data_tx,
            clear_done,
            transmitter: PhantomData,
        }
    }

    fn init_backup_task(
        mut data_rx: UnboundedReceiver<Message<T>>,
        max_buffer_size: usize,
        clear_done: Arc<Notify>,
        backups_full: Arc<AtomicBool>,
        transmitter: U,
    ) -> JoinHandle<Result<()>> {
        tokio::task::spawn_blocking(move || {
            let mut message_buffer = Vec::with_capacity(max_buffer_size);

            while let Some(message) = data_rx.blocking_recv() {
                match message {
                    Message::Data(val) => {
                        message_buffer.push(val);

                        if message_buffer.len() >= max_buffer_size {
                            backups_full.store(true, Ordering::Relaxed);
                        }
                    }
                    Message::Clear => {
                        message_buffer.clear();
                        message_buffer.shrink_to(max_buffer_size);
                        clear_done.notify_one();
                    }
                    Message::Transmit => {
                        #[cfg(feature = "tracing")]
                        tracing::debug!(
                            "backups manager received signal to transmit backups to Sift"
                        );

                        let mut backoff = Duration::from_millis(100);

                        for i in 1..=BACKUPS_TRANSMISSION_MAX_RETRIES {
                            let mut tx = transmitter.clone();
                            let messages = message_buffer.clone();

                            if let Err(err) = Handle::current()
                                .block_on(async move { tx.transmit(messages).await })
                            {
                                #[cfg(feature = "tracing")]
                                tracing::warn!(
                                    retry_counter = i,
                                    error = format!("{err:?}"),
                                    "error while transmitting backups - retrying"
                                );
                                sleep(backoff);
                                backoff *= 2;
                                continue;
                            }
                            break;
                        }
                        message_buffer.clear();
                        message_buffer.shrink_to(max_buffer_size);
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

impl<T, U> BackupsManager<T> for InMemoryBackupsManager<T, U>
where
    T: PbMessage + Clone + Default + 'static,
    U: BackupsTransmitter<T, Vec<T>> + 'static + Send,
{
    /// Send data point to be backed up.
    async fn send(&mut self, msg: T) -> Result<()> {
        if self.backups_full.load(Ordering::Relaxed) {
            return Err(Error::new_msg(
                ErrorKind::BackupLimitReached,
                "backup limit reached",
            ));
        }
        self.data_tx
            .send(Message::Data(msg))
            .map_err(|e| Error::new(ErrorKind::BackupsError, e))
    }

    /// Use to terminate the backup manager
    async fn finish(mut self) -> Result<()> {
        let _ = self.data_tx.send(Message::Complete);

        if let Some(backup_task) = self.backup_task.take() {
            backup_task
                .await
                .map_err(|e| Error::new(ErrorKind::BackupsError, e))
                .context("failed to join in-memory backup task")
                .help("please contact Sift")??;
        }
        Ok(())
    }

    async fn clear(&mut self) -> Result<()> {
        self.data_tx
            .send(Message::Clear)
            .map_err(|e| Error::new(ErrorKind::BackupsError, e))?;

        // Wait for current backups to be cleared
        self.clear_done.notified().await;

        Ok(())
    }

    async fn transmit_backups(&self) -> Result<()> {
        self.data_tx
            .send(Message::Transmit)
            .map_err(|e| Error::new(ErrorKind::BackupsError, e))
    }
}
