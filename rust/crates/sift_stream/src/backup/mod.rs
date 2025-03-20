use crate::pbutil::{decode_messages_length_prefixed, encode_message_length_prefixed};
use chrono::Utc;
use prost::Message as PbMessage;
use sift_error::prelude::*;
use std::{
    env, fs,
    io::{BufReader, BufWriter, ErrorKind as IoErrorKind, Write},
    path::{Path, PathBuf},
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Condvar, Mutex,
    },
};
use tokio::{
    sync::mpsc::{
        channel, unbounded_channel, Receiver, Sender, UnboundedReceiver, UnboundedSender,
    },
    task::JoinHandle,
};

#[cfg(test)]
mod test;

#[derive(Debug)]
pub struct BackupsManager<T> {
    pub(crate) backup_task: JoinHandle<Result<()>>,
    pub(crate) backup_file: PathBuf,
    pub(crate) is_reading: Arc<AtomicBool>,
    pub(crate) reading_cvar: Arc<(Mutex<bool>, Condvar)>,
    data_tx: UnboundedSender<Message<T>>,
    finished_backup_reset_rx: Receiver<()>,
}

enum Message<T> {
    /// Data to be backed up.
    Data(T),
    /// Notifies the backup manager that a checkpoint has been reached.
    CheckpointReached,
    /// Graceful termination; cleans up the backup file.
    Complete,
}

impl<T> BackupsManager<T>
where
    T: PbMessage + Default + 'static,
{
    /// TODO: Mention that this will create directory or use existing directory.. if None for
    /// backups root them tmp dir used.
    pub fn new(
        backups_root: Option<PathBuf>,
        new_dir_name: &str,
        backup_prefix: &str,
    ) -> Result<Self> {
        let (data_tx, data_rx) = unbounded_channel::<Message<T>>();
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

        let is_reading = Arc::new(AtomicBool::default());
        let reading_cvar = Arc::new((Mutex::new(false), Condvar::new()));
        let backup_file =
            backups_dir.join(format!("{backup_prefix}-{}", Utc::now().timestamp_millis()));

        let backup_task = Self::init_backup_task(
            data_rx,
            &backup_file,
            is_reading.clone(),
            reading_cvar.clone(),
            finished_backup_reset_tx,
        )
        .context("failed to start backup task")?;

        Ok(Self {
            backup_task,
            backup_file,
            data_tx,
            is_reading,
            reading_cvar,
            finished_backup_reset_rx,
        })
    }

    /// Send data point to be backed up.
    pub(crate) fn send(&self, msg: T) -> Result<()> {
        self.data_tx
            .send(Message::Data(msg))
            .map_err(|_| {
                Error::new_msg(ErrorKind::BackupsError, "failed to process data to backup")
            })
            .context("back up task may have died")
            .help("please contact Sift")
    }

    /// Notifies the back up manager that a checkpoint was reached in the caller. This will cause
    /// the backup task to truncate the backup file so that new incoming data is relevant only for
    /// the next checkpoint.
    pub(crate) async fn checkpoint(&mut self) -> Result<()> {
        self.data_tx
            .send(Message::CheckpointReached)
            .map_err(|_| Error::new_msg(ErrorKind::BackupsError, "failed to initiate checkpoint"))
            .help("please contact Sift")?;

        let _backup_reset_finished = self.finished_backup_reset_rx.recv().await;

        Ok(())
    }

    /// Use for graceful termination. This will clean up the backup file.
    pub(crate) async fn finish(self) -> Result<()> {
        self.data_tx
            .send(Message::Complete)
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

    pub(crate) async fn get_backup_data(&self) -> Result<Vec<T>> {
        self.is_reading.store(true, Ordering::SeqCst);

        let (mu, cvar) = &*self.reading_cvar;
        let mut done_reading = mu
            .lock()
            .map_err(|_| Error::new_msg(ErrorKind::BackupsError, "encountered a poisoned mutex"))
            .context("failed to acquire lock while yielding to allow reading for backups")
            .help("no longer processing backups - please contact Sift")?;

        let load_result = self.load_backup_data();

        // These need occur regardless of `load_result` to resume the backup task.
        self.is_reading.store(false, Ordering::SeqCst);
        *done_reading = true;
        cvar.notify_all();

        load_result
    }

    fn load_backup_data(&self) -> Result<Vec<T>> {
        let backup = fs::File::open(&self.backup_file)
            .map(BufReader::new)
            .map_err(|e| Error::new(ErrorKind::IoError, e))
            .context("something went wrong while trying to read backup file")
            .help("please contact Sift")?;

        decode_messages_length_prefixed::<_, T>(backup)
            .context("failed to decode backups as part of get_backup_data")
    }

    fn init_backup_task(
        mut data_rx: UnboundedReceiver<Message<T>>,
        backup_file: &Path,
        is_reading: Arc<AtomicBool>,
        reading_cvar: Arc<(Mutex<bool>, Condvar)>,
        finished_backup_reset_tx: Sender<()>,
    ) -> Result<JoinHandle<Result<()>>> {
        let backup_file = backup_file.to_path_buf();

        let mut writer = fs::File::create(&backup_file)
            .map_err(|e| Error::new(ErrorKind::BackupsError, e))
            .context("failed generate backup file")
            .help("please contact Sift")?;

        #[cfg(feature = "tracing")]
        tracing::info!(
            backup_file = format!("{}", backup_file.display()),
            "backup file initialized"
        );

        let join_handle = tokio::task::spawn_blocking(move || -> Result<()> {
            while let Some(message) = data_rx.blocking_recv() {
                if is_reading.load(Ordering::SeqCst) {
                    #[cfg(feature = "tracing")]
                    tracing::info!("backup data requested");

                    let (mu, cvar) = &*reading_cvar;

                    match mu.lock() {
                        Ok(mut done_reading) => {
                            while !*done_reading {
                                #[cfg(feature = "tracing")]
                                tracing::debug!("sleeping backup task while backups is being read");

                                done_reading = match cvar.wait(done_reading) {
                                    Ok(done) => {
                                        #[cfg(feature = "tracing")]
                                        tracing::debug!(
                                            "backups reading done - resuming backup task"
                                        );

                                        done
                                    }
                                    Err(_) => {
                                        #[cfg(feature = "tracing")]
                                        tracing::error!("backups manager no longer processing backups due to poisoned mutex - please notify Sift");

                                        return Err(Error::new_msg(ErrorKind::BackupsError, "encountered a poisoned mutex"))
                                            .context("failed to wait while yielding to allow reading for backups")
                                            .help("no longer processing backups - please contact Sift");
                                    }
                                }
                            }
                            *done_reading = false;
                        }
                        Err(_) => {
                            #[cfg(feature = "tracing")]
                            tracing::error!("backups manager no longer processing backups due to poisoned mutex - please notify Sift");

                            return Err(Error::new_msg(ErrorKind::BackupsError, "encountered a poisoned mutex"))
                                .context("failed to acquire lock while yielding to allow reading for backups")
                                .help("no longer processing backups - please contact Sift");
                        }
                    }
                }

                let data = match message {
                    Message::Data(val) => val,
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
                    Message::CheckpointReached => {
                        #[cfg(feature = "tracing")]
                        tracing::debug!(
                            backup_location = format!("{}", backup_file.display()),
                            "checkpoint reached - truncating current backup file"
                        );

                        // flush the old writer first otherwise its `Drop` will get called and
                        // write to the newly truncated file.
                        //let _ = writer.flush();

                        match fs::File::create(&backup_file).map(BufWriter::new) {
                            Ok(_) => {
                                if finished_backup_reset_tx.try_send(()).is_err() {
                                    #[cfg(feature = "tracing")]
                                    tracing::warn!("backups manager failed to notify checkpoint complete but can still process backups - please notify Sift")
                                }
                                continue;
                            }
                            Err(err) => {
                                #[cfg(feature = "tracing")]
                                tracing::error!(
                                    error = format!("{err}"),
                                    "failed to truncate backup after checkpoint - backups manager no longer processing backups - please notify Sift"
                                );

                                return Err(Error::new(ErrorKind::BackupsError, err))
                                    .context("failed truncate backup file after checkpoint")
                                    .help("please contact Sift");
                            }
                        }
                    }
                };

                let wire_format = encode_message_length_prefixed(&data);

                if let Err(err) = writer.write_all(&wire_format) {
                    #[cfg(feature = "tracing")]
                    tracing::warn!(error = format!("{err}"), "failed to backup a single message which may result data-loss during retries");
                }
            }

            Ok(())
        });

        Ok(join_handle)
    }
}
