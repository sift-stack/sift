use super::{BackupsManager, Message};
use crate::pbutil::{encode_message_length_prefixed, ProtobufDecoder};
use bytesize::ByteSize;
use chrono::Utc;
use parking_lot::{Mutex, MutexGuard};
use prost::Message as PbMessage;
use sift_error::prelude::*;
use std::{
    env,
    fs::{self, File},
    io::{BufReader, BufWriter, ErrorKind as IoErrorKind, Write},
    path::{Path, PathBuf},
    sync::Arc,
};
use tokio::{
    sync::mpsc::{channel, Receiver, Sender},
    task::JoinHandle,
};

/// Max allowed backup size in bytes
const DEFAULT_MAX_BACKUP_SIZE: usize = 100 * 2_usize.pow(20);
const CHANNEL_BUFFER_SIZE: usize = 10_000;

#[derive(Debug)]
pub struct DiskBackupsManager<T> {
    pub backup_file: PathBuf,
    writer: Arc<Mutex<BufWriter<File>>>,
    backup_task: JoinHandle<Result<()>>,
    data_tx: Sender<Message<T>>,
    finished_backup_reset_rx: Receiver<()>,

    /// Max allowed backup size in bytes.
    #[allow(dead_code)]
    max_backup_size: usize,
}

impl<T> DiskBackupsManager<T>
where
    T: PbMessage + Default + 'static,
{
    /// TODO: Mention that this will create directory or use existing directory.. if None for
    /// backups root them tmp dir used.
    pub fn new(
        backups_root: Option<PathBuf>,
        new_dir_name: &str,
        backup_prefix: &str,
        max_backup_size: Option<usize>,
    ) -> Result<Self> {
        let (data_tx, data_rx) = channel::<Message<T>>(CHANNEL_BUFFER_SIZE);
        let (finished_backup_reset_tx, finished_backup_reset_rx) = channel::<()>(1);

        let backups_dir = backups_root
            .unwrap_or_else(env::temp_dir)
            .join(new_dir_name);

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

        let writer = File::create(&backup_file)
            .map(BufWriter::new)
            .map(Mutex::new)
            .map(Arc::new)
            .map_err(|e| Error::new(ErrorKind::BackupsError, e))
            .context("failed generate backup file")
            .help("please contact Sift")?;

        let max_backup_size = max_backup_size.unwrap_or(DEFAULT_MAX_BACKUP_SIZE);

        #[cfg(feature = "tracing")]
        tracing::info!(
            backup_file = format!("{}", backup_file.display()),
            max_backup_size = format!("{}", ByteSize::b(max_backup_size as u64)),
            "backup file initialized"
        );

        let backup_task = Self::init_backup_task(
            data_rx,
            &backup_file,
            finished_backup_reset_tx,
            writer.clone(),
            max_backup_size,
        )
        .context("failed to start backup task")?;

        Ok(Self {
            backup_task,
            backup_file,
            data_tx,
            finished_backup_reset_rx,
            writer,
            max_backup_size,
        })
    }

    fn init_backup_task(
        mut data_rx: Receiver<Message<T>>,
        backup_file: &Path,
        finished_backup_reset_tx: Sender<()>,
        writer: Arc<Mutex<BufWriter<File>>>,
        max_backup_size: usize,
    ) -> Result<JoinHandle<Result<()>>> {
        let backup_file = backup_file.to_path_buf();

        let join_handle = tokio::task::spawn_blocking(move || -> Result<()> {
            let mut bytes_processed = 0;

            while let Some(message) = data_rx.blocking_recv() {
                let mut writer_guard = writer.lock();

                match message {
                    Message::Data(val) => {
                        let wire_format = encode_message_length_prefixed(&val);
                        bytes_processed += wire_format.len();

                        if bytes_processed >= max_backup_size {
                            Self::truncate_backup(
                                &mut writer_guard,
                                &backup_file,
                                finished_backup_reset_tx.clone(),
                            )?;
                        }

                        if let Err(err) = writer_guard.write_all(&wire_format) {
                            #[cfg(feature = "tracing")]
                            tracing::warn!(error = format!("{err}"), "failed to backup a single message which may result data-loss during retries");
                        }
                    }
                    Message::Complete => {
                        #[cfg(feature = "tracing")]
                        tracing::debug!(
                            "shutting down backups manager and cleaning up backup file."
                        );

                        if let Err(err) = fs::remove_file(&backup_file) {
                            #[cfg(feature = "tracing")]
                            tracing::warn!(
                                error = format!("{err}"),
                                backup_location = format!("{}", backup_file.display()),
                                "failed to cleanup backup file"
                            );
                        }
                        break;
                    }
                    Message::TruncateBackup => {
                        Self::truncate_backup(
                            &mut writer_guard,
                            &backup_file,
                            finished_backup_reset_tx.clone(),
                        )?;
                    }
                }
            }

            Ok(())
        });

        Ok(join_handle)
    }

    fn truncate_backup(
        writer_guard: &mut MutexGuard<BufWriter<File>>,
        backup_file: &Path,
        finished_backup_reset_tx: Sender<()>,
    ) -> Result<()> {
        #[cfg(feature = "tracing")]
        tracing::debug!(
            backup_location = format!("{}", backup_file.display()),
            "truncating current backup file"
        );

        // flush the old writer first otherwise its `Drop` will get called and
        // write to the newly truncated file.
        let _ = writer_guard.flush();

        match File::create(backup_file).map(BufWriter::new) {
            Ok(_) => {
                if finished_backup_reset_tx.try_send(()).is_err() {
                    #[cfg(feature = "tracing")]
                    tracing::warn!("backups manager failed to notify checkpoint complete but can still process backups - please notify Sift")
                }
                Ok(())
            }
            Err(err) => {
                #[cfg(feature = "tracing")]
                tracing::error!(
                    error = format!("{err}"),
                    "failed to truncate backup after checkpoint - backups manager no longer processing backups - please notify Sift"
                );

                Err(Error::new(ErrorKind::BackupsError, err))
                    .context("failed truncate backup file after checkpoint")
                    .help("please contact Sift")
            }
        }
    }
}

impl<T> BackupsManager<T> for DiskBackupsManager<T>
where
    T: PbMessage + Default + 'static,
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

    /// Notifies the back up manager that a checkpoint was reached in the caller. This will cause
    /// the backup task to truncate the backup file so that new incoming data is relevant only for
    /// the next checkpoint.
    async fn truncate_backup(&mut self) -> Result<()> {
        self.data_tx
            .send(Message::TruncateBackup)
            .await
            .map_err(|_| Error::new_msg(ErrorKind::BackupsError, "failed to initiate checkpoint"))
            .help("please contact Sift")?;

        let _backup_reset_finished = self.finished_backup_reset_rx.recv().await;

        Ok(())
    }

    /// Use for graceful termination. This will clean up the backup file.
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
        let mut writer_guard = self.writer.lock();

        writer_guard
            .flush()
            .map_err(|e| Error::new(ErrorKind::IoError, e))
            .context("failed to flush backups buffer")
            .help("please contact Sift")?;

        File::open(&self.backup_file)
            .map(BufReader::new)
            .map(ProtobufDecoder::new)
            .map_err(|e| Error::new(ErrorKind::IoError, e))
            .context("something went wrong while trying to read backup file")
            .help("please contact Sift")
    }
}
