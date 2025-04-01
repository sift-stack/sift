use super::{BackupsManager, Message};
use bytesize::ByteSize;
use chrono::Utc;
use prost::Message as PbMessage;
use sift_error::prelude::*;
use std::{
    fs::{self, File},
    io::{BufReader, Error as IoError, ErrorKind as IoErrorKind, Write},
    path::PathBuf,
    sync::Arc,
};
use tokio::{
    sync::{
        Notify,
        mpsc::{Receiver, Sender, channel},
    },
    task::JoinHandle,
};

/// Concerned with writing/reading protobuf from disk.
mod pbfs;
use pbfs::{
    BackupsDecoder,
    chunk::{BATCH_SIZE_LEN, CHECKSUM_HEADER_LEN, MESSAGE_LENGTH_PREFIX_LEN, PbfsChunk},
};

/// Default maximum backup file size - 100 MiB.
pub const DEFAULT_MAX_BACKUP_SIZE: usize = 100 * 2_usize.pow(20);

/// The buffer size used for the channel that sends and receives data to the backup task as well as
/// the in-memory message buffer that gets flushed when full.
const CHANNEL_BUFFER_SIZE: usize = 10_000;

/// Disk-based backup strategy implementation.
#[derive(Debug)]
pub struct DiskBackupsManager<T> {
    pub(crate) backups_root: PathBuf,
    pub(crate) new_dir_name: String,
    pub(crate) backup_prefix: String,
    /// Max allowed backup size in bytes.
    pub(crate) max_backup_size: usize,

    pub(crate) backup_file: PathBuf,
    backup_task: Option<JoinHandle<Result<()>>>,
    flush_and_sync_notification: Arc<Notify>,
    data_tx: Sender<Message<T>>,
}

impl<T> DiskBackupsManager<T>
where
    T: PbMessage + Default + 'static,
{
    /// Users shouldn't have to call interact with [DiskBackupsManager::new] directly.
    ///
    /// `backups_root` is the directory that stores data backups. If it doesn't exist then there
    /// will be an attempt to create it. If `None`, then the user's [data
    /// directory](https://docs.rs/dirs/6.0.0/dirs/fn.data_dir.html) will be used as a default.
    pub fn new(
        backups_root: Option<PathBuf>,
        new_dir_name: &str,
        backup_prefix: &str,
        max_backup_size: Option<usize>,
    ) -> Result<Self> {
        let (data_tx, data_rx) = channel::<Message<T>>(CHANNEL_BUFFER_SIZE);

        let Some(backups_root) = backups_root.or_else(dirs::data_dir) else {
            return Err(
                IoError::new(IoErrorKind::NotFound, "user data directory not found").into(),
            );
        };
        let backups_dir = backups_root.join(new_dir_name);

        match fs::create_dir(&backups_dir) {
            Err(err) if err.kind() != IoErrorKind::AlreadyExists => {
                return Err(Error::new(ErrorKind::BackupsError, err))
                    .with_context(|| format!("failed to create directory for backups at {}", backups_dir.display()))
                    .help("if using a custom path for backups directory ensure that it's valid with proper permissions, otherwise contact Sift")
            }
            _ => ()
        }

        let backup_file =
            backups_dir.join(format!("{backup_prefix}-{}", Utc::now().timestamp_millis()));

        let max_backup_size = max_backup_size.unwrap_or(DEFAULT_MAX_BACKUP_SIZE);

        #[cfg(feature = "tracing")]
        tracing::info!(
            backup_file = format!("{}", backup_file.display()),
            max_backup_size = format!("{}", ByteSize::b(max_backup_size as u64)),
            "backup file initialized"
        );

        let backup = File::create(&backup_file)
            .map_err(|e| Error::new(ErrorKind::BackupsError, e))
            .context("failed generate backup file")
            .help("please contact Sift")?;

        let flush_and_sync_notification = Arc::new(Notify::new());

        let backup_task = Self::init_backup_task(
            data_rx,
            max_backup_size,
            backup,
            flush_and_sync_notification.clone(),
        )
        .context("failed to start backup task")?;

        Ok(Self {
            backups_root,
            new_dir_name: new_dir_name.into(),
            backup_prefix: backup_prefix.into(),
            backup_task: Some(backup_task),
            backup_file,
            data_tx,
            max_backup_size,
            flush_and_sync_notification,
        })
    }

    fn init_backup_task(
        mut data_rx: Receiver<Message<T>>,
        max_backup_size: usize,
        mut backup: File,
        flush_and_sync_notifier: Arc<Notify>,
    ) -> Result<JoinHandle<Result<()>>> {
        let join_handle = tokio::task::spawn_blocking(move || -> Result<()> {
            let mut message_buffer = Vec::with_capacity(CHANNEL_BUFFER_SIZE);
            let mut bytes_processed = 0;

            while let Some(message) = data_rx.blocking_recv() {
                match message {
                    Message::Data(val) => {
                        bytes_processed += val.encoded_len() + MESSAGE_LENGTH_PREFIX_LEN;
                        message_buffer.push(val);

                        if bytes_processed >= max_backup_size
                            || message_buffer.len() >= CHANNEL_BUFFER_SIZE
                        {
                            let chunk = PbfsChunk::new(&message_buffer)?;
                            backup.write_all(&chunk)?;
                            backup.sync_all()?;
                            bytes_processed += CHECKSUM_HEADER_LEN + BATCH_SIZE_LEN;
                            message_buffer.clear();

                            if bytes_processed >= max_backup_size {
                                #[cfg(feature = "tracing")]
                                tracing::debug!("backup size exceeded max configured");

                                flush_and_sync_notifier.notify_one();
                                break;
                            }
                        }
                    }
                    Message::Flush => {
                        #[cfg(feature = "tracing")]
                        tracing::debug!("backup task received flush and sync signal");

                        if !message_buffer.is_empty() {
                            let chunk = PbfsChunk::new(&message_buffer)?;
                            backup.write_all(&chunk)?;
                            backup.sync_all()?;
                        }
                        flush_and_sync_notifier.notify_one();
                        break;
                    }
                    Message::Complete => {
                        #[cfg(feature = "tracing")]
                        tracing::debug!("backup task complete - shutting down");

                        break;
                    }
                }
            }
            Ok(())
        });
        Ok(join_handle)
    }
}

impl<T> BackupsManager<T> for DiskBackupsManager<T>
where
    T: PbMessage + Default + 'static,
{
    /// Send data point to be backed up.
    async fn send(&mut self, msg: T) -> Result<()> {
        match self.data_tx.send(Message::Data(msg)).await {
            Ok(_) => Ok(()),

            // Backup task has shutdown due to max bytes being reached.
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

    /// Use for graceful termination. This will clean up the backup file.
    async fn finish(mut self) -> Result<()> {
        let _ = self.data_tx.send(Message::Complete).await;

        if let Some(backup_task) = self.backup_task.take() {
            backup_task
                .await
                .map_err(|e| Error::new(ErrorKind::BackupsError, e))
                .context("failed to join disk backup task")
                .help("please contact Sift")??;
        }
        Ok(())
    }

    /// Sends the a message to the backup task to flush and sync if there's any buffered data
    /// before creating an iterator over the backup's contents.
    async fn get_backup_data(&mut self) -> Result<impl Iterator<Item = Result<T>>> {
        let _ = self.data_tx.send(Message::Flush).await;
        self.flush_and_sync_notification.notified().await;

        let backups_decoder = File::open(&self.backup_file)
            .map(BufReader::new)
            .map(BackupsDecoder::new)
            .map_err(|e| Error::new(ErrorKind::IoError, e))
            .context("failed to open backup")
            .help("contact Sift")?;

        Ok(backups_decoder.into_iter())
    }
}

impl<T> Drop for DiskBackupsManager<T> {
    fn drop(&mut self) {
        let _ = fs::remove_file(&self.backup_file);
    }
}
