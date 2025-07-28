use crate::backup::BACKUPS_TRANSMISSION_MAX_RETRIES;

use super::{BackupsManager, BackupsTransmitter, Message};
use chrono::Utc;
use prost::Message as PbMessage;
use sift_error::prelude::*;
use std::{
    fs::{self, File},
    io::{Error as IoError, ErrorKind as IoErrorKind, Write},
    marker::PhantomData,
    path::{Path, PathBuf},
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
    thread::sleep,
    time::Duration,
};
use tokio::{
    fs::remove_file,
    runtime::Handle,
    sync::{
        Notify,
        mpsc::{UnboundedReceiver, UnboundedSender, unbounded_channel},
    },
    task::JoinHandle,
};

/// Concerned with writing/reading protobuf from disk.
pub(crate) mod pbfs;
pub use pbfs::stream::BackupsIter;
use pbfs::{
    chunk::{BATCH_SIZE_LEN, CHECKSUM_HEADER_LEN, MESSAGE_LENGTH_PREFIX_LEN, PbfsChunk},
    stream::BackupsStream,
};

/// For manual user decoding of backups.
pub use pbfs::stream::decode_backup;

/// Default maximum backup file size - 100 MiB.
pub const DEFAULT_MAX_BACKUP_SIZE: usize = 100 * 2_usize.pow(20);

pub const DEFAULT_BACKUP_ROOT: &str = "sift_stream_backups";

/// 1 GiB
pub const BACKUP_FILE_MAX_SIZE: usize = 1073741824;

/// The buffer size used for the channel that sends and receives data to the backup task as well as
/// the in-memory message buffer that gets flushed when full.
const CHANNEL_BUFFER_SIZE: usize = 10_000;

/// Disk-based backup strategy implementation.
#[derive(Debug)]
pub struct DiskBackupsManager<T, U> {
    backups_full: Arc<AtomicBool>,
    clear_done: Arc<Notify>,
    backup_task: Option<JoinHandle<Result<()>>>,
    data_tx: UnboundedSender<Message<T>>,

    transmitter: PhantomData<U>,
}

impl<T, U> DiskBackupsManager<T, U>
where
    T: PbMessage + Default + 'static,
    U: BackupsTransmitter<T, BackupsStream<T>> + 'static + Send,
{
    /// Users shouldn't have to call interact with [DiskBackupsManager::new] directly.
    ///
    /// `backups_root` is the directory that stores data backups. If it doesn't exist then there
    /// will be an attempt to create it. If `None`, then the user's [data
    /// directory](https://docs.rs/dirs/6.0.0/dirs/fn.data_dir.html) will be used as a default.
    pub fn new(
        backups_root: Option<PathBuf>,
        new_dir_name: &str,
        max_backup_size: Option<usize>,
        transmitter: U,
    ) -> Result<Self> {
        let (data_tx, data_rx) = unbounded_channel::<Message<T>>();

        let Some(backups_root) = backups_root
            .or_else(Self::default_backup_dir)
            .map(|r| r.join(new_dir_name))
        else {
            return Err(
                IoError::new(IoErrorKind::NotFound, "user data directory not found").into(),
            );
        };
        let backups_dir = backups_root
            .join(new_dir_name)
            .join(format!("{}", Utc::now().timestamp_millis()));

        match fs::create_dir_all(&backups_dir) {
            Err(err) if err.kind() != IoErrorKind::AlreadyExists => {
                return Err(Error::new(ErrorKind::BackupsError, err))
                    .with_context(|| format!("failed to create directory for backups at {}", backups_dir.display()))
                    .help("if using a custom path for backups directory ensure that it's valid with proper permissions, otherwise contact Sift")
            }
            _ => ()
        }

        let backups_full = Arc::new(AtomicBool::default());
        let max_backup_size = max_backup_size.unwrap_or(DEFAULT_MAX_BACKUP_SIZE);
        let clear_done = Arc::new(Notify::new());

        let backup_task = Self::init_backup_task(
            data_rx,
            backups_full.clone(),
            max_backup_size,
            backups_dir,
            clear_done.clone(),
            transmitter,
        )
        .context("failed to start backup task")?;

        Ok(Self {
            data_tx,
            clear_done,
            backups_full,
            backup_task: Some(backup_task),
            transmitter: PhantomData,
        })
    }

    fn init_backup_task(
        mut data_rx: UnboundedReceiver<Message<T>>,
        backups_full: Arc<AtomicBool>,
        max_backup_size: usize,
        backups_dir: PathBuf,
        clear_done: Arc<Notify>,
        transmitter: U,
    ) -> Result<JoinHandle<Result<()>>> {
        let mut current_index = 0;
        let backup_path = backups_dir.join(format!("{current_index}"));
        let mut current_backup = Self::new_backup_file(&backup_path)?;

        let join_handle = tokio::task::spawn_blocking(move || -> Result<()> {
            let mut message_buffer = Vec::with_capacity(CHANNEL_BUFFER_SIZE);
            let mut backups = vec![backup_path];
            let mut current_file_bytes = 0;

            // Represents the total amount of bytes between transmissions.
            let mut bytes_processed = 0;

            macro_rules! clear_current_backups {
                () => {
                    for backup in &backups {
                        let _ = remove_file(backup);
                    }
                    backups.clear();
                    current_file_bytes = 0;
                    bytes_processed = 0;
                    current_index += 1;

                    let new_backup_path = backups_dir.join(format!("{current_index}"));
                    current_backup = Self::new_backup_file(&new_backup_path)?;
                    backups.push(new_backup_path);
                };
            }

            while let Some(message) = data_rx.blocking_recv() {
                match message {
                    Message::Data(val) => {
                        let message_len = val.encoded_len() + MESSAGE_LENGTH_PREFIX_LEN;
                        bytes_processed += message_len;
                        current_file_bytes += message_len;
                        message_buffer.push(val);

                        if current_file_bytes >= BACKUP_FILE_MAX_SIZE {
                            current_file_bytes = 0;
                            current_index += 1;

                            let new_backup_path = backups_dir.join(format!("{current_index}"));
                            current_backup = Self::new_backup_file(&new_backup_path)?;
                            backups.push(new_backup_path);
                        }

                        if bytes_processed >= max_backup_size
                            || message_buffer.len() >= CHANNEL_BUFFER_SIZE
                        {
                            let chunk = PbfsChunk::new(&message_buffer)?;
                            current_backup.write_all(&chunk)?;
                            current_backup.sync_all()?;
                            bytes_processed += CHECKSUM_HEADER_LEN + BATCH_SIZE_LEN;
                            message_buffer.clear();

                            if bytes_processed >= max_backup_size {
                                backups_full.store(true, Ordering::Relaxed);
                            }
                        }
                    }
                    Message::Transmit => {
                        #[cfg(feature = "tracing")]
                        tracing::debug!(
                            "backups manager received signal to transmit backups to Sift"
                        );

                        if !message_buffer.is_empty() {
                            let chunk = PbfsChunk::new(&message_buffer)?;
                            current_backup.write_all(&chunk)?;
                            current_backup.sync_all()?;
                        }

                        let mut backoff = Duration::from_millis(100);

                        for i in 1..=BACKUPS_TRANSMISSION_MAX_RETRIES {
                            let backups_stream = BackupsStream::new(&backups);
                            let mut tx = transmitter.clone();

                            if let Err(err) = Handle::current()
                                .block_on(async move { tx.transmit(backups_stream).await })
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
                        clear_current_backups!();
                    }
                    Message::Clear => {
                        clear_current_backups!();
                        clear_done.notify_one();
                        backups_full.store(false, Ordering::Relaxed);
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

    fn default_backup_dir() -> Option<PathBuf> {
        dirs::data_dir().map(|d| d.join(DEFAULT_BACKUP_ROOT))
    }

    fn new_backup_file(p: &Path) -> Result<File> {
        File::create(p)
            .map_err(|e| Error::new(ErrorKind::BackupsError, e))
            .context("failed generate backup file")
            .help("please contact Sift")
    }
}

impl<T, U> BackupsManager<T> for DiskBackupsManager<T, U>
where
    T: PbMessage + Default + 'static,
    U: BackupsTransmitter<T, BackupsStream<T>>,
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

    /// Use for graceful termination. This will clean up the backup file.
    async fn finish(mut self) -> Result<()> {
        let _ = self.data_tx.send(Message::Complete);

        if let Some(backup_task) = self.backup_task.take() {
            backup_task
                .await
                .map_err(|e| Error::new(ErrorKind::BackupsError, e))
                .context("failed to join disk backup task")
                .help("please contact Sift")??;
        }
        Ok(())
    }

    async fn transmit_backups(&self) -> Result<()> {
        self.data_tx
            .send(Message::Transmit)
            .map_err(|e| Error::new(ErrorKind::BackupsError, e))
    }

    async fn clear(&mut self) -> Result<()> {
        self.data_tx
            .send(Message::Clear)
            .map_err(|e| Error::new(ErrorKind::BackupsError, e))?;

        // Wait for current backups to be cleared
        self.clear_done.notified().await;

        Ok(())
    }
}
