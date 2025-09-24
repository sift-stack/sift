use super::Message;
use crate::metrics::SiftStreamMetrics;
use crate::RetryPolicy;
use crate::backup::DiskBackupPolicy;
use crate::backup::disk::decode_backup;
use crate::backup::disk::pbfs::BackupsDecoder;
use crate::backup::disk::pbfs::{
    BATCH_SIZE_LEN, CHECKSUM_HEADER_LEN, PbfsChunk, chunk::MESSAGE_LENGTH_PREFIX_LEN,
};
use chrono::Utc;
use prost::Message as PbMessage;
use sift_error::prelude::*;
use sift_rs::ingest::v1::IngestWithConfigDataStreamRequest;
use sift_rs::{SiftChannel, ingest::v1::ingest_service_client::IngestServiceClient};
use std::path::Path;
use std::{
    fs::{self, File},
    io::{Error as IoError, ErrorKind as IoErrorKind, Write},
    path::PathBuf,
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
    time::Duration,
};
use tokio::{
    runtime::Handle,
    sync::{
        Mutex, Notify,
        mpsc::{UnboundedReceiver, UnboundedSender, unbounded_channel},
    },
    task::JoinHandle,
};
use tokio_stream::StreamExt;

/// The buffer size used by the in-memory message buffer that gets flushed to file when full.
const CHANNEL_BUFFER_SIZE: usize = 10_000;

#[derive(Clone)]
struct BackupConfig {
    directory: PathBuf,
    prefix: String,
    max_size: usize,
    max_file_count: Option<usize>,
    retain_backups: bool,
}

/// Disk-based backup with async ingestion implementation.
pub struct AsyncBackupsManager<T> {
    backup_config: BackupConfig,
    backup_retry_policy: RetryPolicy,
    backup_task: Option<JoinHandle<Result<()>>>,
    ingest_task: Option<BackupIngestTask<T>>,
    backup_files: Arc<Mutex<Vec<PathBuf>>>,
    backup_tx: UnboundedSender<Message<T>>,
    backup_full: Arc<AtomicBool>,
    flush_and_sync_notifier: Arc<Notify>,
    restart_backup_notifier: Arc<Notify>,
    grpc_channel: SiftChannel,
    metrics: Arc<SiftStreamMetrics>,
}

impl AsyncBackupsManager<IngestWithConfigDataStreamRequest> {
    /// Create new AsyncBackupsManager using [IngestWithConfigDataStreamRequest].
    /// Starts backup task for ingesting sent data to files.
    /// Users shouldn't have to call interact with [AsyncBackupsManager::new] directly, as this is
    /// normally performed as part of builder
    ///
    /// # Arguments
    ///
    /// * `new_dir_name` - The name of the directory used for storing backup files
    /// * `backup_prefix` - The prefix added to all backup files
    /// * `disk_backup_policy` - The policy for disk backups
    /// * `backup_retry_policy` - The retry policy used if backup ingestion is required, but with unlimited retries
    /// * `grpc_channel` - The SiftChannel used for backup ingestion
    pub(crate) fn new(
        new_dir_name: &str,
        backup_prefix: &str,
        disk_backup_policy: DiskBackupPolicy,
        backup_retry_policy: RetryPolicy,
        grpc_channel: SiftChannel,
        metrics: Arc<SiftStreamMetrics>,
    ) -> Result<Self> {
        let (backup_tx, backup_rx) =
            unbounded_channel::<Message<IngestWithConfigDataStreamRequest>>();

        let Some(backups_root) = disk_backup_policy.backups_dir.or_else(dirs::data_dir) else {
            return Err(
                IoError::new(IoErrorKind::NotFound, "user data directory not found").into(),
            );
        };
        let backups_dir = backups_root.join(new_dir_name);

        match fs::create_dir_all(&backups_dir) {
            Err(err) if err.kind() != IoErrorKind::AlreadyExists => {
                return Err(Error::new(ErrorKind::BackupsError, err))
                    .with_context(|| format!("failed to create directory for backups at {}", backups_dir.display()))
                    .help("if using a custom path for backups directory ensure that it's valid with proper permissions, otherwise contact Sift")
            }
            _ => ()
        }

        let backup_info = BackupConfig {
            directory: backups_dir,
            prefix: backup_prefix.to_string(),
            max_size: disk_backup_policy.max_backup_file_size,
            max_file_count: disk_backup_policy.rolling_file_policy.max_file_count,
            retain_backups: disk_backup_policy.retain_backups,
        };

        let backup_files = Arc::new(Mutex::new(Vec::new()));
        let flush_and_sync_notifier = Arc::new(Notify::new());
        let restart_backup_notifier = Arc::new(Notify::new());
        let backup_full = Arc::new(AtomicBool::new(false));

        let backup_task = Self::init_backup_task(
            backup_info.clone(),
            backup_rx,
            backup_files.clone(),
            backup_full.clone(),
            flush_and_sync_notifier.clone(),
            restart_backup_notifier.clone(),
            metrics.clone(),
        )
        .context("failed to start backup task")?;

        Ok(Self {
            backup_config: backup_info,
            backup_retry_policy,
            backup_task: Some(backup_task),
            ingest_task: None,
            backup_files,
            backup_tx,
            backup_full,
            flush_and_sync_notifier,
            restart_backup_notifier,
            grpc_channel,
            metrics
        })
    }

    // Init the backup task. Runs until Flush or Complete signal given.
    // Recieves messages with backup_rx.
    // Adds files to backup_files once finalized.
    // Backup_full set true if backups are full.
    // Waits on restart_backup_notifier if full.
    // Sends notification of flush_and_sync_notifier when flush is complete.
    fn init_backup_task(
        backup_info: BackupConfig,
        mut backup_rx: UnboundedReceiver<Message<IngestWithConfigDataStreamRequest>>,
        backup_files: Arc<Mutex<Vec<PathBuf>>>,
        backup_full: Arc<AtomicBool>,
        flush_and_sync_notifier: Arc<Notify>,
        restart_backup_notifier: Arc<Notify>,
        metrics: Arc<SiftStreamMetrics>,
    ) -> Result<JoinHandle<Result<()>>> {
        let handle = Handle::current();

        let join_handle = tokio::task::spawn_blocking(move || -> Result<()> {
            let mut message_buffer = Vec::with_capacity(CHANNEL_BUFFER_SIZE);

            let mut cur_bytes_processed = 0;
            let (mut cur_backup_file_path, mut cur_backup_file) =
                Self::create_backup_file(&backup_info)?;
            metrics.backups.log_new_file();

            while let Some(message) = backup_rx.blocking_recv() {
                match message {
                    Message::Data(data) => {
                        let message_bytes = data.encoded_len() + MESSAGE_LENGTH_PREFIX_LEN;

                        metrics.backups.log_message(message_bytes as u64);

                        cur_bytes_processed += message_bytes;
                        message_buffer.push(data);

                        if cur_bytes_processed >= backup_info.max_size
                            || message_buffer.len() >= CHANNEL_BUFFER_SIZE
                        {
                            Self::flush_message_buffer(&mut cur_backup_file, &mut message_buffer)?;
                            cur_bytes_processed += CHECKSUM_HEADER_LEN + BATCH_SIZE_LEN;

                            if cur_bytes_processed >= backup_info.max_size {
                                metrics.backups.log_new_file();

                                #[cfg(feature = "tracing")]
                                tracing::debug!(
                                    cur_backup_file = format!("{}", cur_backup_file_path.display()),
                                    cur_bytes_processed,
                                    total_bytes_written = metrics.backups.total_bytes.get(),
                                    total_files_written = metrics.backups.total_file_count.get(),
                                    max_backup_size = backup_info.max_size,
                                    "current backup file has reached max size - closing out file"
                                );

                                // Close out the current file
                                drop(cur_backup_file);

                                let backup_files_len = {
                                    let mut backup_files_guard = backup_files.blocking_lock();
                                    backup_files_guard.push(cur_backup_file_path);
                                    backup_files_guard.len()
                                };
                                if let Some(max_file_count) = backup_info.max_file_count
                                    && backup_files_len >= max_file_count
                                {
                                    // We've reached our max file/size limit. Send signal and wait for restart notification
                                    backup_full.store(true, Ordering::Relaxed);
                                    handle.block_on(restart_backup_notifier.notified());
                                }

                                cur_bytes_processed = 0;
                                (cur_backup_file_path, cur_backup_file) =
                                    Self::create_backup_file(&backup_info)?;
                            }
                        }
                    }
                    Message::Flush => {
                        metrics.backups.log_new_file();

                        #[cfg(feature = "tracing")]
                        tracing::debug!(
                            cur_backup_file = format!("{}", cur_backup_file_path.display()),
                            cur_bytes_processed,
                            total_bytes_written = metrics.backups.total_bytes.get(),
                            total_files_written = metrics.backups.total_file_count.get(),
                            max_backup_size = backup_info.max_size,
                            "backup task received flush and sync signal - closing out file"
                        );

                        if !message_buffer.is_empty() {
                            Self::flush_message_buffer(&mut cur_backup_file, &mut message_buffer)?;
                        }
                        drop(cur_backup_file);
                        {
                            let mut backup_files_guard = backup_files.blocking_lock();

                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                cur_backup_file = format!("{}", cur_backup_file_path.display()),
                                cur_file_count = backup_files_guard.len(),
                                "adding unprocessed file"
                            );

                            backup_files_guard.push(cur_backup_file_path);
                        }
                        flush_and_sync_notifier.notify_one();
                        cur_bytes_processed = 0;
                        (cur_backup_file_path, cur_backup_file) =
                            Self::create_backup_file(&backup_info)?;
                    }
                    Message::Complete => {
                        metrics.backups.log_new_file();

                        #[cfg(feature = "tracing")]
                        tracing::debug!(
                            cur_backup_file = format!("{}", cur_backup_file_path.display()),
                            cur_bytes_processed,
                            total_bytes_written = metrics.backups.total_bytes.get(),
                            total_files_written = metrics.backups.total_file_count.get(),
                            max_backup_size = backup_info.max_size,
                            "backup task complete - closing file and shutting down"
                        );

                        // Close out current file and add to file list
                        drop(cur_backup_file);
                        {
                            let mut backup_files_guard = backup_files.blocking_lock();

                            #[cfg(feature = "tracing")]
                            tracing::debug!(
                                cur_backup_file = format!("{}", cur_backup_file_path.display()),
                                cur_file_count = backup_files_guard.len(),
                                "adding unprocessed file"
                            );

                            backup_files_guard.push(cur_backup_file_path);
                        }

                        return Ok(());
                    }
                }
            }
            Ok(())
        });
        Ok(join_handle)
    }

    // Write to file from message_buffer. Called from backup task
    fn flush_message_buffer(
        backup_file: &mut File,
        message_buffer: &mut Vec<IngestWithConfigDataStreamRequest>,
    ) -> Result<()> {
        let chunk = PbfsChunk::new(message_buffer)?;
        backup_file.write_all(&chunk)?;
        backup_file.sync_all()?;
        message_buffer.clear();
        Ok(())
    }

    // Create a backup file. Called from backup task
    fn create_backup_file(backup_info: &BackupConfig) -> Result<(PathBuf, File)> {
        let backup_file_path = backup_info.directory.join(format!(
            "{}-{}",
            backup_info.prefix,
            Utc::now().timestamp_millis()
        ));
        let backup_file = File::create(&backup_file_path)
            .map_err(|e| Error::new(ErrorKind::BackupsError, e))
            .context("failed to generate backup file")
            .help("please contact Sift")?;

        Ok((backup_file_path, backup_file))
    }

    /// Restart the backup task. Clears the current list of unprocessed backup files, and deleting them
    /// if allowed by the retain policy. Unpauses the backup task if the backups were full.
    pub(crate) async fn restart(&mut self) -> Result<()> {
        // Flush the current file
        // We don't want to get stuck here, and proceeding before the flush is complete won't cause any harm
        // so keep the timeout small
        let _ = self.backup_tx.send(Message::Flush);
        tokio::select! {
            _ = self.flush_and_sync_notifier.notified() => {
                #[cfg(feature = "tracing")]
                tracing::debug!(
                    "saw flush notification for restart"
                );
            }
            _ = tokio::time::sleep(Duration::from_secs(1)) => {
                #[cfg(feature = "tracing")]
                tracing::debug!(
                    "timed out before flush could complete for restart"
                );
            }
        }

        // Always clear backup files on restart since it indicates a successful checkpoint
        let mut backup_files_guard = self.backup_files.lock().await;
        #[cfg(feature = "tracing")]
        tracing::info!(
            cur_file_count = backup_files_guard.len(),
            "restarting async backup - Clearing existing backup files"
        );
        if !self.backup_config.retain_backups {
            for file_path in backup_files_guard.iter() {
                if let Err(err) = fs::remove_file(file_path) {
                    #[cfg(feature = "tracing")]
                    tracing::warn!(
                        backup_file = file_path.display().to_string(),
                        "unable to delete backup file: {err:?}"
                    );
                }
            }
        }
        backup_files_guard.clear();

        self.metrics.backups.log_restart();

        if self.backup_task.is_some() {
            if self.backup_full.swap(false, Ordering::Relaxed) {
                // Was previously full. Need to restart backup
                self.restart_backup_notifier.notify_one();
            }
        } else {
            #[cfg(feature = "tracing")]
            tracing::warn!(
                "no backup task found - restarting - some backup data may have been lost"
            );

            let (backup_tx, backup_rx) =
                unbounded_channel::<Message<IngestWithConfigDataStreamRequest>>();

            let backup_task = Self::init_backup_task(
                self.backup_config.clone(),
                backup_rx,
                self.backup_files.clone(),
                self.backup_full.clone(),
                self.flush_and_sync_notifier.clone(),
                self.restart_backup_notifier.clone(),
                self.metrics.clone(),
            )
            .context("failed to start backup task")?;

            self.backup_tx = backup_tx;
            self.backup_task = Some(backup_task);
        }

        Ok(())
    }

    /// Start backup ingestion
    /// Send flush command and wait
    /// Takes the current list of backup files and adds them to a queue for ingestion
    /// The ingestion task is started if not already running
    pub(crate) async fn start_backup_ingestion(&mut self) -> usize {
        match self.backup_tx.send(Message::Flush) {
            Ok(_) => {
                // Wait for notification that we've flushed the backup file
                self.flush_and_sync_notifier.notified().await;
                #[cfg(feature = "tracing")]
                tracing::debug!("got flush notification from backup task");
            }
            Err(err) => {
                // Skip the flush notification since we don't want to get stuck
                #[cfg(feature = "tracing")]
                tracing::warn!("error sending flush command to backup task. {:?}", err);
            }
        }

        let unprocessed_files: Vec<PathBuf> = {
            let mut backup_files_guard = self.backup_files.lock().await;

            #[cfg(feature = "tracing")]
            tracing::info!(
                cur_file_count = backup_files_guard.len(),
                "adding backup files for ingest"
            );

            backup_files_guard.drain(..).collect()
        };

        let file_count = unprocessed_files.len();

        if file_count == 0 {
            return file_count;
        }

        self.metrics.backups.files_pending_ingestion.add(file_count as u64);

        if let Some(ingest_task) = self.ingest_task.as_mut() {
            if let Err(err) = ingest_task.add(unprocessed_files.clone()) {
                #[cfg(feature = "tracing")]
                tracing::warn!(
                    "error trying to add files to ingest queue - restarting ingest task. {:?}",
                    err
                );
            } else {
                return file_count;
            }
        }

        // No ingest task, or we saw errors when trying to add to the queue, so we should try restarting it

        let mut ingest_task = BackupIngestTask::new(
            self.grpc_channel.clone(),
            self.backup_retry_policy.clone(),
            self.backup_config.retain_backups,
            self.metrics.clone(),
        );
        if let Err(err) = ingest_task.add(unprocessed_files) {
            #[cfg(feature = "tracing")]
            tracing::warn!(
                "saw error when trying to add files to ingest queue: {:?}",
                err
            );
        }
        self.ingest_task = Some(ingest_task);

        file_count
    }

    /// Send files for backup or sends message to flush/complete
    pub(crate) async fn send(&mut self, msg: IngestWithConfigDataStreamRequest) -> Result<()> {
        if self.backup_full.load(Ordering::Relaxed) {
            return Err(Error::new_msg(
                ErrorKind::BackupLimitReached,
                "backup limit reached",
            ));
        }

        match self.backup_tx.send(Message::Data(msg)) {
            Ok(_) => {
                self.metrics.messages_sent_to_backup.increment();
                Ok(())
            },

            // data_rx must be dropped, indicating backup task has shutdown
            Err(_) => {
                let Some(backup_task) = self.backup_task.take() else {
                    #[cfg(feature = "tracing")]
                    tracing::warn!("no backup task found during send");
                    return Ok(());
                };

                tokio::select! {
                    res = backup_task => {
                        match res {
                            Ok(Ok(_)) => Err(Error::new_msg(
                                ErrorKind::BackupLimitReached,
                                "backup limit reached",
                            )),
                            Ok(Err(err)) => Err(Error::new(ErrorKind::BackupsError, err))
                                .context("backup task encountered an error")
                                .help("please notify Sift"),
                            Err(err) => Err(Error::new(ErrorKind::BackupsError, err))
                                .context("error waiting for backup task to finish")
                                .help("please notify Sift"),
                        }
                    },
                    // Should never reach this, but included to avoid potential block on send
                    _ = tokio::time::sleep(Duration::from_secs(5)) => {
                        Err(Error::new_msg(ErrorKind::BackupsError, "timed out waiting for backup_task to return"))
                            .help("please notify Sift")
                    }
                }
            }
        }
    }

    /// Shutdown the manager. Flushes the open backup file and adds to the backup file list
    pub(crate) async fn finish(&mut self) -> Result<()> {
        // Signal to close out last file
        let _ = self.backup_tx.send(Message::Complete);

        // Abort the ingest task, if it exists
        // Any files marked for ingestion but not ingested will remain on disk
        if let Some(ingest_task) = self.ingest_task.take() {
            ingest_task.task_handle.abort();
        }

        // If we aren't retaining backup files, wait for task to complete and clean up
        if !self.backup_config.retain_backups
            && let Some(backup_task) = self.backup_task.take()
        {
            let _ = backup_task.await;
            for file_path in self.backup_files.lock().await.iter() {
                if let Err(err) = fs::remove_file(file_path) {
                    #[cfg(feature = "tracing")]
                    tracing::warn!(
                        backup_file = file_path.display().to_string(),
                        "unable to delete backup file: {:?}",
                        err
                    );
                }
            }
        }

        Ok(())
    }
}

/// Contains handle to the ingest task and an unbound queue for transmitting data.
/// Task will ingest each file provided in the ingestion queue, retrying indefinitely if needed.
/// Successfully ingested files are cleared using the provided retention policy.
struct BackupIngestTask<T> {
    ingest_tx: UnboundedSender<PathBuf>,
    task_handle: JoinHandle<Result<()>>,
    _phantom_data: std::marker::PhantomData<T>,
}

impl BackupIngestTask<IngestWithConfigDataStreamRequest> {
    fn new(grpc_channel: SiftChannel, retry_policy: RetryPolicy, retain_ingested: bool, metrics: Arc<SiftStreamMetrics>) -> Self {
        let (ingest_tx, mut ingest_rx) = unbounded_channel::<PathBuf>();

        let task_handle = tokio::spawn(async move {
            // Will sleep, waiting for more files until BackupIngestTask is dropped
            while let Some(backup_file_path) = ingest_rx.recv().await {
                let mut retries = 0;
                let mut current_wait = retry_policy.initial_backoff;

                loop {
                    if ingest_rx.is_closed() {
                        break;
                    }

                    if let Err(err) =
                        Self::ingest_file(grpc_channel.clone(), &backup_file_path).await
                    {
                        retries += 1;
                        metrics.backups.cur_ingest_retries.set(retries);

                        #[cfg(feature = "tracing")]
                        tracing::warn!(
                            backup_file = backup_file_path.display().to_string(),
                            err = format!("{err:?}"),
                            current_backup_retry = retries,
                            "retrying backup file ingestion after backoff: {} secs",
                            current_wait.as_secs_f32()
                        );
                        tokio::time::sleep(current_wait).await;
                        current_wait = (current_wait * u32::from(retry_policy.backoff_multiplier))
                            .min(retry_policy.max_backoff);

                        continue;
                    }

                    let files_ingested = metrics.backups.files_ingested.increment();
                    metrics.backups.files_pending_ingestion.set(ingest_rx.len() as u64);
                    metrics.backups.cur_ingest_retries.set(0);

                    #[cfg(feature = "tracing")]
                    tracing::info!(
                        backup_file = backup_file_path.display().to_string(),
                        files_ingested,
                        "backup file ingested",
                    );
                    break;
                }

                if !retain_ingested && let Err(err) = std::fs::remove_file(backup_file_path.clone())
                {
                    #[cfg(feature = "tracing")]
                    tracing::warn!(
                        backup_file = backup_file_path.display().to_string(),
                        "unable to delete ingested backup file: {err:?}"
                    );
                }
            }

            Ok(())
        });

        Self {
            ingest_tx,
            task_handle,
            _phantom_data: std::marker::PhantomData,
        }
    }

    /// Attempt to ingest a provided file into sift
    async fn ingest_file(grpc_channel: SiftChannel, backup_file_path: &Path) -> Result<()> {
        let mut client = IngestServiceClient::new(grpc_channel);

        let decoder_res: Result<BackupsDecoder<IngestWithConfigDataStreamRequest, _>> =
            decode_backup(backup_file_path);

        match decoder_res {
            Ok(backups_decoder) => {
                let iter_stream =
                    tokio_stream::iter(backups_decoder.into_iter()).filter_map(|res| res.ok());

                let raw_response = client.ingest_with_config_data_stream(iter_stream).await;
                let response = raw_response
                    .map(|res| res.into_inner())
                    .map_err(|e| Error::new(ErrorKind::StreamError, e));

                match response {
                    Ok(_) => {
                        #[cfg(feature = "tracing")]
                        tracing::info!(
                            backup_file = backup_file_path.display().to_string(),
                            "ingested backup file"
                        );
                        Ok(())
                    }
                    Err(err) => {
                        #[cfg(feature = "tracing")]
                        tracing::info!(
                            backup_file = backup_file_path.display().to_string(),
                            "encountered error from sift ingesting backup file: {:?}",
                            err
                        );
                        Err(err)
                    }
                }
            }
            Err(err) => {
                #[cfg(feature = "tracing")]
                tracing::warn!(
                    backup_file = backup_file_path.display().to_string(),
                    "saw an error while trying to open backup file for ingestion - ignoring file: {:?}",
                    err
                );
                // We want to return Ok here, since we aren't going to try re-ingesting this file
                Ok(())
            }
        }
    }

    /// Add files into the ingestion queue
    fn add(&mut self, files: Vec<PathBuf>) -> Result<()> {
        if self.task_handle.is_finished() {
            return Err(Error::new_msg(
                ErrorKind::BackupsError,
                "BackupIngestTask ended prematurely",
            ));
        }

        for file in files {
            self.ingest_tx
                .send(file)
                .map_err(|e| Error::new(ErrorKind::BackupsError, e))?;
        }

        Ok(())
    }
}
