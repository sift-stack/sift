use crate::RetryPolicy;
use crate::backup::DiskBackupPolicy;
use crate::backup::disk::decode_backup;
use crate::metrics::SiftStreamMetrics;
use crate::stream::tasks::{ControlMessage, DataMessage};
use async_channel::{Receiver, Sender};
use chrono::Utc;
use prost::Message as PbMessage;
use sift_error::prelude::*;
use sift_pbfs::{MESSAGE_LENGTH_PREFIX_LEN, chunk::PbfsChunk};
use sift_rs::CompressionEncoding;
use sift_rs::{SiftChannel, ingest::v1::ingest_service_client::IngestServiceClient};
use std::time::Duration;
use std::{
    io::{Error as IoError, ErrorKind as IoErrorKind},
    path::PathBuf,
    sync::Arc,
};
use tokio::fs::{self, File};
use tokio::io::AsyncWriteExt;
use tokio::sync::broadcast;
use tokio::sync::broadcast::error::RecvError;
use tokio_stream::StreamExt;

#[derive(Clone)]
struct BackupConfig {
    enabled: bool,
    directory: PathBuf,
    prefix: String,
    max_size: usize,
    max_file_count: Option<usize>,
    retain_backups: bool,
}
#[derive(Debug, Default)]
struct FileMetadata {
    path: PathBuf,
    message_count: usize,
    has_dropped_messages: bool,
    file_size: usize,
}

/// Disk-based backup with async ingestion implementation.
pub struct AsyncBackupsManager {
    backup_config: BackupConfig,
    control_tx: broadcast::Sender<ControlMessage>,
    control_rx: broadcast::Receiver<ControlMessage>,
    data_rx: async_channel::Receiver<DataMessage>,
    backup_files: Vec<PathBuf>,
    checkpoint_needs_reingest: bool,
    signaled_full: bool,

    // State
    current_file: Option<File>,
    current_file_metadata: FileMetadata,

    metrics: Arc<SiftStreamMetrics>,
}

impl AsyncBackupsManager {
    /// Create new AsyncBackupsManager using [IngestWithConfigDataStreamRequest].
    /// Starts backup task for ingesting sent data to files.
    /// Users shouldn't have to call interact with [AsyncBackupsManager::new] directly, as this is
    /// normally performed as part of builder
    ///
    /// # Arguments
    ///
    /// * `enabled` - Whether the backup manager is enabled
    /// * `new_dir_name` - The name of the directory used for storing backup files
    /// * `backup_prefix` - The prefix added to all backup files
    /// * `disk_backup_policy` - The policy for disk backups, including the root directory to store backups in,
    ///   the maximum size of each backup file, and the rolling file policy
    /// * `control_tx` - The sender for control messages
    /// * `control_rx` - The receiver for control messages
    /// * `data_rx` - The receiver for data messages
    /// * `metrics` - The metrics for the backup manager
    #[allow(clippy::too_many_arguments)]
    pub(crate) async fn new(
        enabled: bool,
        new_dir_name: &str,
        backup_prefix: &str,
        disk_backup_policy: DiskBackupPolicy,
        control_tx: broadcast::Sender<ControlMessage>,
        control_rx: broadcast::Receiver<ControlMessage>,
        data_rx: async_channel::Receiver<DataMessage>,
        metrics: Arc<SiftStreamMetrics>,
    ) -> Result<Self> {
        let Some(backups_root) = disk_backup_policy.backups_dir.or_else(dirs::data_dir) else {
            return Err(
                IoError::new(IoErrorKind::NotFound, "user data directory not found").into(),
            );
        };
        let backups_dir = backups_root.join(new_dir_name);

        match fs::create_dir_all(&backups_dir).await {
            Err(err) if err.kind() != IoErrorKind::AlreadyExists => {
                return Err(Error::new(ErrorKind::BackupsError, err))
                    .with_context(|| format!("failed to create directory for backups at {}", backups_dir.display()))
                    .help("if using a custom path for backups directory ensure that it's valid with proper permissions, otherwise contact Sift")
            }
            _ => ()
        }

        let backup_info = BackupConfig {
            enabled,
            directory: backups_dir,
            prefix: backup_prefix.to_string(),
            max_size: disk_backup_policy.max_backup_file_size,
            max_file_count: disk_backup_policy.rolling_file_policy.max_file_count,
            retain_backups: disk_backup_policy.retain_backups,
        };

        Ok(Self {
            backup_config: backup_info,
            control_tx,
            control_rx,
            data_rx,
            backup_files: Vec::new(),
            checkpoint_needs_reingest: false,
            signaled_full: false,
            current_file: None,
            current_file_metadata: FileMetadata {
                path: PathBuf::new(),
                message_count: 0,
                has_dropped_messages: false,
                file_size: 0,
            },
            metrics,
        })
    }

    pub(crate) async fn run(&mut self) -> Result<()> {
        loop {
            tokio::select! {
                backup_msg = self.data_rx.recv() => {
                    match backup_msg {
                        Ok(backup_msg) => self.handle_data_message(backup_msg).await?,
                        Err(async_channel::RecvError) => break,
                    }
                }
                Ok(control_message) = self.control_rx.recv() => {
                    match control_message {
                        ControlMessage::Shutdown => break,
                        ControlMessage::CheckpointNeedsReingestion => {
                            self.checkpoint_needs_reingest = true;
                        }
                        ControlMessage::CheckpointComplete => {
                            self.checkpoint().await?;
                        }
                        _ => continue,
                    }
                }
            }
        }

        #[cfg(feature = "tracing")]
        tracing::info!("backup manager shutting down");

        self.cleanup().await?;
        Ok(())
    }

    /// Cleanup the backup manager by clearing the backup files list and resetting the current file.
    async fn cleanup(&mut self) -> Result<()> {
        #[cfg(feature = "tracing")]
        tracing::info!("backup manager cleanup started");

        // Drain the data channel of any remaining messages.
        while let Ok(data_message) = self.data_rx.recv().await {
            self.handle_data_message(data_message).await?;
        }

        // Trigger the final checkpoint.
        //
        // It is assumed that the primary ingestion task will send this signal as soon as it receives
        // the shutdown signal, allowing this task to push any final messages to the backup files.
        //
        // Since this is the final checkpoint for shutdown, the reingestion flag is false -- trying to
        // reingest could take some time and we don't want to block shutdown.
        loop {
            match self.control_rx.recv().await {
                Ok(ControlMessage::CheckpointComplete) | Err(_) => break,
                _ => continue,
            }
        }

        // Process the final checkpoint.
        self.checkpoint().await?;

        Ok(())
    }

    /// Process a checkpoint signal by either removing the backup files if the checkpoint was successful (if
    /// not retaining backups) or sending a reingestion signal for all backup files if the checkpoint was
    /// not successful.
    async fn checkpoint(&mut self) -> Result<()> {
        self.signaled_full = false;

        if !self.backup_config.enabled {
            return Ok(());
        }

        // Rotate the current file to ensure it is closed and saved to the backup files list.
        if let Err(e) = self.rotate_file().await {
            #[cfg(feature = "tracing")]
            tracing::warn!("unable to rotate backup file: {e:?}");
        }

        // Update the metrics.
        self.metrics.backups.log_restart();

        // If the checkpoint was successful and the backups should not be retained, remove them.
        if !self.checkpoint_needs_reingest && !self.backup_config.retain_backups {
            #[cfg(feature = "tracing")]
            tracing::info!(
                "reingestion not required, removing backup files for completed checkpoint"
            );

            for file_path in self.backup_files.iter() {
                if let Err(e) = fs::remove_file(file_path).await {
                    #[cfg(feature = "tracing")]
                    tracing::warn!(
                        backup_file = file_path.display().to_string(),
                        "unable to delete backup file: {e:?}"
                    );
                }
            }
        } else if self.checkpoint_needs_reingest {
            let mut backup_files = Vec::new();
            core::mem::swap(&mut self.backup_files, &mut backup_files);

            #[cfg(feature = "tracing")]
            tracing::info!(
                "reingestion required, sending {len} checkpoint backup files for reingestion",
                len = backup_files.len()
            );

            // If the checkpoint was not successful, send a reingestion signal for all backup files.
            if let Err(e) = self
                .control_tx
                .send(ControlMessage::ReingestBackups { backup_files })
            {
                #[cfg(feature = "tracing")]
                tracing::warn!("unable to send reingestion signal: {e:?}");
            }
        }

        // Clear the backup files list and reset the checkpoint needs reingestion flag.
        self.backup_files.clear();
        self.checkpoint_needs_reingest = false;

        Ok(())
    }

    /// Process a data message for backup.
    async fn handle_data_message(&mut self, msg: DataMessage) -> Result<()> {
        if !self.backup_config.enabled {
            return Ok(());
        }

        // If the message was dropped for ingestion, set the checkpoint needs reingestion flag.
        if msg.dropped_for_ingestion {
            self.checkpoint_needs_reingest = true;
        }

        // Write message to file.
        self.write_to_file(msg).await?;

        // Rotate the backup file when necessary.
        if self.should_rotate_file() {
            self.rotate_file().await?;
        }

        // Check if we've reached the max file count, sending a control message that should
        // trigger a new checkpoint and thus flush or reingest the backup files.
        //
        // Note: Remaining below the max file count is best effort and not guaranteed. It is
        // preferrable to slightly exceed this limit in order to avoid data loss and potentially
        // blocking data ingestion due to a full backup files.
        if let Some(max_file_count) = self.backup_config.max_file_count
            && self.backup_files.len() >= max_file_count
            && !self.signaled_full
        {
            self.control_tx
                .send(ControlMessage::BackupFull)
                .map_err(|e| Error::new(ErrorKind::BackupsError, e))?;
            self.signaled_full = true;
        }

        Ok(())
    }

    /// Writes a message to the provided file.
    async fn write_to_file(&mut self, msg: DataMessage) -> Result<()> {
        // Create a new file if one doesn't exist.
        if self.current_file.is_none() {
            let (path, file) = Self::create_backup_file(&self.backup_config).await?;
            self.current_file_metadata = FileMetadata {
                path: path.clone(),
                message_count: 0,
                has_dropped_messages: false,
                file_size: 0,
            };
            self.current_file = Some(file);
            self.metrics.backups.log_new_file();
        }

        let Some(file) = self.current_file.as_mut() else {
            return Err(Error::new_msg(
                ErrorKind::BackupsError,
                "current file is not set",
            ));
        };

        let message_bytes = msg.request.encoded_len() + MESSAGE_LENGTH_PREFIX_LEN;
        self.metrics.backups.log_message(message_bytes as u64);

        // Create a single-message chunk for immediate write
        let chunk = PbfsChunk::new(&[msg.request])?;

        // Write immediately and sync
        file.write_all(&chunk).await?;

        self.current_file_metadata.message_count += 1;
        self.current_file_metadata.file_size += chunk.len();
        self.current_file_metadata.has_dropped_messages |= msg.dropped_for_ingestion;

        Ok(())
    }

    /// Checks if the current file has reached the max size and should be rotated.
    fn should_rotate_file(&self) -> bool {
        self.current_file_metadata.file_size >= self.backup_config.max_size
    }

    /// Rotates the current backup file by closing it and saving the file path to the backup files list.
    async fn rotate_file(&mut self) -> Result<()> {
        // Close out the current file by dropping it, if there is no file, there is nothing to do.
        match self.current_file.take() {
            Some(file) => {
                // Ensure data is persisted to disk and not simply buffered.
                if let Err(e) = file.sync_all().await {
                    #[cfg(feature = "tracing")]
                    tracing::warn!("unable to sync backup file, data may be lost: {e:?}");
                }
            }
            None => return Ok(()),
        };

        // Save the current file path to the backup files list.
        let file_path = self.current_file_metadata.path.clone();
        self.backup_files.push(file_path);

        self.current_file_metadata = FileMetadata::default();
        Ok(())
    }

    /// Create a backup file.
    async fn create_backup_file(backup_info: &BackupConfig) -> Result<(PathBuf, File)> {
        let backup_file_path = backup_info.directory.join(format!(
            "{}-{}",
            backup_info.prefix,
            Utc::now().timestamp_millis()
        ));
        let backup_file = File::create(&backup_file_path)
            .await
            .map_err(|e| Error::new(ErrorKind::BackupsError, e))
            .context("failed to generate backup file")
            .help("please contact Sift")?;

        Ok((backup_file_path, backup_file))
    }
}

impl Drop for AsyncBackupsManager {
    fn drop(&mut self) {
        if let Some(file) = self.current_file.take() {
            #[cfg(feature = "tracing")]
            tracing::warn!(
                "graceful shutdown was not used -- attempting to sync backup file during drop to prevent data loss"
            );

            // Conver to standard file for blocking sync_all.
            let std_file = match file.try_into_std() {
                Ok(std_file) => std_file,
                Err(_) => {
                    #[cfg(feature = "tracing")]
                    tracing::error!("failed to convert backup file to std file, data may be lost");
                    return;
                }
            };

            // Attempt to sync the file.
            if let Err(e) = std_file.sync_all() {
                #[cfg(feature = "tracing")]
                tracing::warn!(
                    error = %e,
                    "unable to sync backup file during drop, data may be lost"
                );
            }
        }
    }
}

/// Contains handle to the ingest task and an unbound queue for transmitting data.
/// Task will ingest each file provided in the ingestion queue, retrying indefinitely if needed.
/// Successfully ingested files are cleared using the provided retention policy.
pub(crate) struct BackupIngestTask {
    control_rx: broadcast::Receiver<ControlMessage>,
    to_reingest_rx: Receiver<PathBuf>,
    to_reingest_tx: Sender<PathBuf>,
    enable_compression_for_ingestion: bool,
    grpc_channel: SiftChannel,
    retry_policy: RetryPolicy,
    retain_backups: bool,
    metrics: Arc<SiftStreamMetrics>,
}

impl BackupIngestTask {
    pub(crate) fn new(
        control_rx: broadcast::Receiver<ControlMessage>,
        grpc_channel: SiftChannel,
        enable_compression_for_ingestion: bool,
        retry_policy: RetryPolicy,
        retain_backups: bool,
        metrics: Arc<SiftStreamMetrics>,
    ) -> Self {
        let (to_reingest_tx, to_reingest_rx) = async_channel::bounded(1024);
        Self {
            control_rx,
            to_reingest_rx,
            to_reingest_tx,
            enable_compression_for_ingestion,
            grpc_channel,
            retry_policy,
            retain_backups,
            metrics,
        }
    }

    /// Run the backup re-ingestion task.
    ///
    /// This task will listen for re-ingestion signals and attempt to re-ingest the backup files.
    pub(crate) async fn run(mut self) -> Result<()> {
        // Future to re-ingest backup files.
        let mut reingest_fut = Box::pin(Self::reingest_files(
            self.to_reingest_rx.clone(),
            self.grpc_channel.clone(),
            self.retry_policy,
            self.enable_compression_for_ingestion,
            self.retain_backups,
            self.metrics.clone(),
        ));

        // Receive control messages and re-ingest backup files cooperatively.
        loop {
            tokio::select! {
                control_msg = self.control_rx.recv() => {
                    match control_msg {
                        Ok(ControlMessage::Shutdown) => {
                            break;
                        },
                        Ok(ControlMessage::ReingestBackups { backup_files }) => {
                            for backup_file in backup_files {
                                if let Err(e) = self.to_reingest_tx.force_send(backup_file) {
                                    #[cfg(feature = "tracing")]
                                    tracing::warn!("re-ingestion queue is full, backup file will have to be manually re-ingested : {e:?}");
                                }
                            }
                        },
                        Err(RecvError::Closed) => break,
                        _ => continue,
                    }
                }
                res = (&mut reingest_fut) => {
                    match res {
                        Ok(_) => {
                            #[cfg(feature = "tracing")]
                            tracing::info!("reingestion task completed");
                        }
                        Err(e) => {
                            #[cfg(feature = "tracing")]
                            tracing::warn!("reingestion encountered an error: {e:?}");
                        }
                    }
                    break;
                }
            }
        }

        #[cfg(feature = "tracing")]
        tracing::info!("re-ingestion task shutting down");

        Ok(())
    }

    /// Attempt to re-ingest a provided file into Sift.
    async fn reingest_files(
        to_reingest_rx: Receiver<PathBuf>,
        grpc_channel: SiftChannel,
        retry_policy: RetryPolicy,
        enable_compression_for_ingestion: bool,
        retain_backups: bool,
        metrics: Arc<SiftStreamMetrics>,
    ) -> Result<()> {
        let mut client = IngestServiceClient::new(grpc_channel);

        // If compression is enabled, add the compression codecs to the client.
        if enable_compression_for_ingestion {
            client = client
                .send_compressed(CompressionEncoding::Gzip)
                .accept_compressed(CompressionEncoding::Gzip);
        }

        while let Ok(backup_file_path) = to_reingest_rx.recv().await {
            metrics
                .backups
                .files_pending_ingestion
                .set(to_reingest_rx.len() as u64);

            let mut current_wait = Duration::ZERO;
            metrics.backups.cur_ingest_retries.set(0);

            // Attempt to ingest the file up to the maximum number of retries.
            for _ in 0..retry_policy.max_attempts {
                // Decode the backup file.
                //
                // TODO: Convert this to use async file operations.
                let backups_decoder = decode_backup(&backup_file_path)?;

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
                        metrics.backups.files_ingested.increment();

                        if !retain_backups && let Err(e) = fs::remove_file(&backup_file_path).await
                        {
                            #[cfg(feature = "tracing")]
                            tracing::warn!(
                                backup_file = backup_file_path.display().to_string(),
                                "unable to delete backup file: {e:?}"
                            );
                        }

                        break;
                    }
                    Err(err) => {
                        #[cfg(feature = "tracing")]
                        tracing::info!(
                            backup_file = backup_file_path.display().to_string(),
                            "encountered error from sift ingesting backup file: {:?}",
                            err
                        );
                        metrics.backups.cur_ingest_retries.add(1);
                        current_wait = retry_policy.backoff(current_wait);
                        tokio::time::sleep(current_wait).await;
                    }
                }
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::{TimeValue, backup::disk::RollingFilePolicy};
    use tracing_test::traced_test;

    use super::*;
    use sift_rs::ingest::v1::{
        IngestWithConfigDataChannelValue, IngestWithConfigDataStreamRequest,
        ingest_with_config_data_channel_value::Type,
    };
    use tempdir::TempDir;

    #[tokio::test]
    async fn test_async_manager_setup_and_configuration() {
        let tmp_dir = TempDir::new("test_async_manager").unwrap();
        let tmp_dir_path = tmp_dir.path();
        let backup_policy = DiskBackupPolicy {
            backups_dir: Some(tmp_dir_path.to_path_buf()),
            max_backup_file_size: 1024,
            rolling_file_policy: RollingFilePolicy {
                max_file_count: Some(1),
            },
            retain_backups: true,
        };
        let (control_tx, control_rx) = broadcast::channel(1024);
        let (_, data_rx) = async_channel::bounded(1024);
        let metrics = Arc::new(SiftStreamMetrics::default());
        let backup_manager = AsyncBackupsManager::new(
            true,
            "test",
            "test",
            backup_policy,
            control_tx,
            control_rx,
            data_rx,
            metrics,
        )
        .await
        .unwrap();

        // Verify the backup manager configuration.
        assert!(
            backup_manager.backup_config.enabled,
            "backup manager should be enabled"
        );
        assert_eq!(
            backup_manager.backup_config.directory,
            tmp_dir_path.join("test")
        );
        assert_eq!(backup_manager.backup_config.prefix, "test");
        assert_eq!(backup_manager.backup_config.max_size, 1024);
        assert_eq!(backup_manager.backup_config.max_file_count, Some(1));
        assert!(
            backup_manager.backup_config.retain_backups,
            "backup manager should retain backups"
        );
        assert!(
            backup_manager.current_file.is_none(),
            "current file should be none"
        );
    }

    #[tokio::test]
    async fn test_async_manager_disabled() {
        let tmp_dir = TempDir::new("test_async_manager").unwrap();
        let tmp_dir_path = tmp_dir.path();
        let backup_policy = DiskBackupPolicy {
            backups_dir: Some(tmp_dir_path.to_path_buf()),
            max_backup_file_size: 64,
            rolling_file_policy: RollingFilePolicy {
                max_file_count: Some(10),
            },
            retain_backups: false,
        };
        let (control_tx, control_rx) = broadcast::channel(1024);
        let (_data_tx, data_rx) = async_channel::bounded(1024);
        let metrics = Arc::new(SiftStreamMetrics::default());
        let mut backup_manager = AsyncBackupsManager::new(
            false,
            "test",
            "test",
            backup_policy.clone(),
            control_tx,
            control_rx,
            data_rx,
            metrics,
        )
        .await
        .unwrap();

        // Send a data message to the backup manager.
        let data_message = DataMessage {
            request: IngestWithConfigDataStreamRequest {
                ingestion_config_id: "test-0".to_string(),
                flow: "some_flow".to_string(),
                timestamp: Some(*TimeValue::now()),
                channel_values: vec![IngestWithConfigDataChannelValue {
                    r#type: Some(Type::Int32(1)),
                }],
                ..Default::default()
            },
            dropped_for_ingestion: false,
        };

        // Handle a few messages to fill up some backup files.
        for _ in 0..9 {
            assert!(
                backup_manager
                    .handle_data_message(data_message.clone())
                    .await
                    .is_ok(),
                "data message should be handled"
            );
            assert!(
                backup_manager.current_file.is_none(),
                "current file should be none"
            );
            assert_eq!(
                backup_manager.current_file_metadata.file_size, 0,
                "current file metadata should have a size of 0"
            );
            assert_eq!(
                backup_manager.current_file_metadata.message_count, 0,
                "current file metadata should have no messages"
            );
        }
    }

    #[tokio::test]
    async fn test_async_manager_create_backup_file() {
        let tmp_dir = TempDir::new("test_async_manager").unwrap();
        let tmp_dir_path = tmp_dir.path();
        let backup_policy = DiskBackupPolicy {
            backups_dir: Some(tmp_dir_path.to_path_buf()),
            max_backup_file_size: 64,
            rolling_file_policy: RollingFilePolicy {
                max_file_count: Some(1),
            },
            retain_backups: true,
        };
        let (control_tx, control_rx) = broadcast::channel(1024);
        let (_data_tx, data_rx) = async_channel::bounded(1024);
        let metrics = Arc::new(SiftStreamMetrics::default());
        let mut backup_manager = AsyncBackupsManager::new(
            true,
            "test",
            "test",
            backup_policy,
            control_tx,
            control_rx,
            data_rx,
            metrics,
        )
        .await
        .unwrap();

        // Send a data message to the backup manager.
        let data_message = DataMessage {
            request: IngestWithConfigDataStreamRequest {
                ingestion_config_id: "test-0".to_string(),
                flow: "some_flow".to_string(),
                timestamp: Some(*TimeValue::now()),
                channel_values: vec![IngestWithConfigDataChannelValue {
                    r#type: Some(Type::Int32(1)),
                }],
                ..Default::default()
            },
            dropped_for_ingestion: false,
        };

        assert!(
            backup_manager
                .handle_data_message(data_message)
                .await
                .is_ok(),
            "data message should be handled"
        );

        // There should be one backup file containing the single streamed message.
        assert!(
            backup_manager.current_file_metadata.path.exists(),
            "backup file should exist"
        );
        assert!(
            backup_manager.current_file_metadata.file_size > 0,
            "backup file should have a size"
        );
        assert_eq!(
            backup_manager.current_file_metadata.message_count, 1,
            "backup file should have one message"
        );
        assert!(
            !backup_manager.current_file_metadata.has_dropped_messages,
            "backup file should not have dropped messages"
        );
    }

    #[tokio::test]
    async fn test_async_manager_rollover_backup_file() {
        let tmp_dir = TempDir::new("test_async_manager").unwrap();
        let tmp_dir_path = tmp_dir.path();
        let backup_policy = DiskBackupPolicy {
            backups_dir: Some(tmp_dir_path.to_path_buf()),
            max_backup_file_size: 128,
            rolling_file_policy: RollingFilePolicy {
                max_file_count: Some(2),
            },
            retain_backups: true,
        };
        let (control_tx, control_rx) = broadcast::channel(1024);
        let (_data_tx, data_rx) = async_channel::bounded(1024);
        let metrics = Arc::new(SiftStreamMetrics::default());
        let mut backup_manager = AsyncBackupsManager::new(
            true,
            "test",
            "test",
            backup_policy,
            control_tx,
            control_rx,
            data_rx,
            metrics,
        )
        .await
        .unwrap();

        // Send a data message to the backup manager.
        let data_message = DataMessage {
            request: IngestWithConfigDataStreamRequest {
                ingestion_config_id: "test-0".to_string(),
                flow: "some_flow".to_string(),
                timestamp: Some(*TimeValue::now()),
                channel_values: vec![IngestWithConfigDataChannelValue {
                    r#type: Some(Type::Int32(1)),
                }],
                ..Default::default()
            },
            dropped_for_ingestion: false,
        };

        // Handle a few messages to fill up the current backup file.
        for _ in 0..4 {
            assert!(
                backup_manager
                    .handle_data_message(data_message.clone())
                    .await
                    .is_ok(),
                "data message should be handled"
            );
        }

        // The backup file should have been rotated.
        assert!(
            backup_manager.backup_files.len() > 0,
            "backup files should be present"
        );
        assert!(
            backup_manager.backup_files[0].exists(),
            "backup file should still exist"
        );

        assert!(
            backup_manager.current_file.is_some(),
            "current file should be set"
        );
        assert!(
            !backup_manager
                .current_file_metadata
                .path
                .display()
                .to_string()
                .is_empty(),
            "current file metadata path should not be empty"
        );
        assert!(
            backup_manager.current_file_metadata.path != backup_manager.backup_files[0],
            "current file metadata path should have a different path"
        );
    }

    #[tokio::test]
    async fn test_async_manager_backup_full() {
        let tmp_dir = TempDir::new("test_async_manager").unwrap();
        let tmp_dir_path = tmp_dir.path();
        let backup_policy = DiskBackupPolicy {
            backups_dir: Some(tmp_dir_path.to_path_buf()),
            max_backup_file_size: 128,
            rolling_file_policy: RollingFilePolicy {
                max_file_count: Some(2),
            },
            retain_backups: true,
        };
        let (control_tx, mut control_rx) = broadcast::channel(1024);
        let (_data_tx, data_rx) = async_channel::bounded(1024);
        let metrics = Arc::new(SiftStreamMetrics::default());
        let mut backup_manager = AsyncBackupsManager::new(
            true,
            "test",
            "test",
            backup_policy.clone(),
            control_tx.clone(),
            control_tx.subscribe(),
            data_rx,
            metrics,
        )
        .await
        .unwrap();

        // Send a data message to the backup manager.
        let data_message = DataMessage {
            request: IngestWithConfigDataStreamRequest {
                ingestion_config_id: "test-0".to_string(),
                flow: "some_flow".to_string(),
                timestamp: Some(*TimeValue::now()),
                channel_values: vec![IngestWithConfigDataChannelValue {
                    r#type: Some(Type::Int32(1)),
                }],
                ..Default::default()
            },
            dropped_for_ingestion: false,
        };

        // Handle a few messages to fill up the current backup file and exhaust the number of backup files.
        for _ in 0..9 {
            assert!(
                backup_manager
                    .handle_data_message(data_message.clone())
                    .await
                    .is_ok(),
                "data message should be handled"
            );
        }

        // There should be the maximum number of backup files.
        assert!(
            backup_manager.backup_files.len()
                >= backup_policy.rolling_file_policy.max_file_count.unwrap(),
            "backup files should be present"
        );

        // The backup manager should send a backup full control message when
        // it reaches the maximum number of backup files.
        assert_eq!(control_rx.try_recv(), Ok(ControlMessage::BackupFull));
        assert!(backup_manager.signaled_full);

        // Since the backup full message was already sent, additional data should
        // not trigger another message.
        assert!(
            backup_manager
                .handle_data_message(data_message.clone())
                .await
                .is_ok(),
            "data message should be handled"
        );
        assert!(
            control_rx.try_recv().is_err(),
            "control message should not have been sent"
        );
    }

    #[tokio::test]
    async fn test_async_manager_checkpoint_complete_retain_backups() {
        let tmp_dir = TempDir::new("test_async_manager").unwrap();
        let tmp_dir_path = tmp_dir.path();
        let backup_policy = DiskBackupPolicy {
            backups_dir: Some(tmp_dir_path.to_path_buf()),
            max_backup_file_size: 64,
            rolling_file_policy: RollingFilePolicy {
                max_file_count: Some(10),
            },
            retain_backups: true,
        };
        let (control_tx, mut control_rx) = broadcast::channel(1024);
        let (_data_tx, data_rx) = async_channel::bounded(1024);
        let metrics = Arc::new(SiftStreamMetrics::default());
        let mut backup_manager = AsyncBackupsManager::new(
            true,
            "test",
            "test",
            backup_policy.clone(),
            control_tx.clone(),
            control_tx.subscribe(),
            data_rx,
            metrics,
        )
        .await
        .unwrap();

        // Send a data message to the backup manager.
        let data_message = DataMessage {
            request: IngestWithConfigDataStreamRequest {
                ingestion_config_id: "test-0".to_string(),
                flow: "some_flow".to_string(),
                timestamp: Some(*TimeValue::now()),
                channel_values: vec![IngestWithConfigDataChannelValue {
                    r#type: Some(Type::Int32(1)),
                }],
                ..Default::default()
            },
            dropped_for_ingestion: false,
        };

        // Handle a few messages to fill up some backup files.
        for _ in 0..9 {
            assert!(
                backup_manager
                    .handle_data_message(data_message.clone())
                    .await
                    .is_ok(),
                "data message should be handled"
            );
        }

        // Save the backup file paths.
        let backup_file_paths = backup_manager.backup_files.clone();
        assert!(
            !backup_file_paths.is_empty(),
            "backup files should be present, pending checkpoint completion"
        );

        // Process the checkpoint.
        assert!(
            backup_manager.checkpoint().await.is_ok(),
            "checkpoint should be processed"
        );

        // The backup files pending checkpoint completion should be empty.
        assert!(
            backup_manager.backup_files.is_empty(),
            "backup file paths should be empty"
        );

        for backup_file_path in backup_file_paths {
            assert!(
                backup_file_path.exists(),
                "backup file should have been retained"
            );
        }

        // The control message should not have been sent.
        assert!(
            control_rx.try_recv().is_err(),
            "control message should not have been sent"
        );
    }

    #[tokio::test]
    async fn test_async_manager_checkpoint_complete_backups_deleted() {
        let tmp_dir = TempDir::new("test_async_manager").unwrap();
        let tmp_dir_path = tmp_dir.path();
        let backup_policy = DiskBackupPolicy {
            backups_dir: Some(tmp_dir_path.to_path_buf()),
            max_backup_file_size: 64,
            rolling_file_policy: RollingFilePolicy {
                max_file_count: Some(10),
            },
            retain_backups: false,
        };
        let (control_tx, mut control_rx) = broadcast::channel(1024);
        let (_data_tx, data_rx) = async_channel::bounded(1024);
        let metrics = Arc::new(SiftStreamMetrics::default());
        let mut backup_manager = AsyncBackupsManager::new(
            true,
            "test",
            "test",
            backup_policy.clone(),
            control_tx.clone(),
            control_tx.subscribe(),
            data_rx,
            metrics,
        )
        .await
        .unwrap();

        // Send a data message to the backup manager.
        let data_message = DataMessage {
            request: IngestWithConfigDataStreamRequest {
                ingestion_config_id: "test-0".to_string(),
                flow: "some_flow".to_string(),
                timestamp: Some(*TimeValue::now()),
                channel_values: vec![IngestWithConfigDataChannelValue {
                    r#type: Some(Type::Int32(1)),
                }],
                ..Default::default()
            },
            dropped_for_ingestion: false,
        };

        // Handle a few messages to fill up some backup files.
        for _ in 0..9 {
            assert!(
                backup_manager
                    .handle_data_message(data_message.clone())
                    .await
                    .is_ok(),
                "data message should be handled"
            );
        }

        // Save the backup file paths.
        let backup_file_paths = backup_manager.backup_files.clone();
        assert!(
            !backup_file_paths.is_empty(),
            "backup files should be present, pending checkpoint completion"
        );

        // Process the checkpoint.
        assert!(
            backup_manager.checkpoint().await.is_ok(),
            "checkpoint should be processed"
        );

        // The backup files pending checkpoint completion should be empty.
        assert!(
            backup_manager.backup_files.is_empty(),
            "backup file paths should be empty"
        );

        for backup_file_path in backup_file_paths {
            assert!(
                !backup_file_path.exists(),
                "backup file should have been deleted"
            );
        }

        // The control message should not have been sent.
        assert!(
            control_rx.try_recv().is_err(),
            "control message should not have been sent"
        );
    }

    #[tokio::test]
    #[traced_test]
    async fn test_async_manager_checkpoint_before_file_is_created() {
        let tmp_dir = TempDir::new("test_async_manager").unwrap();
        let tmp_dir_path = tmp_dir.path();
        let backup_policy = DiskBackupPolicy {
            backups_dir: Some(tmp_dir_path.to_path_buf()),
            max_backup_file_size: 64,
            rolling_file_policy: RollingFilePolicy {
                max_file_count: Some(10),
            },
            retain_backups: false,
        };
        let (control_tx, mut control_rx) = broadcast::channel(1024);
        let (_data_tx, data_rx) = async_channel::bounded(1024);
        let metrics = Arc::new(SiftStreamMetrics::default());
        let mut backup_manager = AsyncBackupsManager::new(
            true,
            "test",
            "test",
            backup_policy.clone(),
            control_tx.clone(),
            control_tx.subscribe(),
            data_rx,
            metrics,
        )
        .await
        .unwrap();

        // Processing the checkpoint when disabled should be a no-op.
        assert!(
            backup_manager.checkpoint().await.is_ok(),
            "checkpoint should be processed"
        );

        // The control message should not have been sent.
        assert!(
            control_rx.try_recv().is_err(),
            "control message should not have been sent"
        );

        // No backup file deletion should be logged.
        assert!(
            !logs_contain("unable to delete backup file"),
            "no backup file deletion should be logged"
        );
    }

    #[tokio::test]
    #[traced_test]
    async fn test_async_manager_checkpoint_backups_disabled() {
        let tmp_dir = TempDir::new("test_async_manager").unwrap();
        let tmp_dir_path = tmp_dir.path();
        let backup_policy = DiskBackupPolicy {
            backups_dir: Some(tmp_dir_path.to_path_buf()),
            max_backup_file_size: 64,
            rolling_file_policy: RollingFilePolicy {
                max_file_count: Some(10),
            },
            retain_backups: false,
        };
        let (control_tx, mut control_rx) = broadcast::channel(1024);
        let (_data_tx, data_rx) = async_channel::bounded(1024);
        let metrics = Arc::new(SiftStreamMetrics::default());
        let mut backup_manager = AsyncBackupsManager::new(
            false,
            "test",
            "test",
            backup_policy.clone(),
            control_tx.clone(),
            control_tx.subscribe(),
            data_rx,
            metrics,
        )
        .await
        .unwrap();

        // Processing the checkpoint when disabled should be a no-op.
        assert!(
            backup_manager.checkpoint().await.is_ok(),
            "checkpoint should be processed"
        );

        // The control message should not have been sent.
        assert!(
            control_rx.try_recv().is_err(),
            "control message should not have been sent"
        );

        // No backup file deletion should be logged.
        assert!(
            !logs_contain("unable to delete backup file"),
            "no backup file deletion should be logged"
        );
    }

    #[tokio::test]
    async fn test_async_manager_checkpoint_complete_reingestion_required() {
        let tmp_dir = TempDir::new("test_async_manager").unwrap();
        let tmp_dir_path = tmp_dir.path();
        let backup_policy = DiskBackupPolicy {
            backups_dir: Some(tmp_dir_path.to_path_buf()),
            max_backup_file_size: 64,
            rolling_file_policy: RollingFilePolicy {
                max_file_count: Some(10),
            },
            retain_backups: false,
        };
        let (control_tx, mut control_rx) = broadcast::channel(1024);
        let (_data_tx, data_rx) = async_channel::bounded(1024);
        let metrics = Arc::new(SiftStreamMetrics::default());
        let mut backup_manager = AsyncBackupsManager::new(
            true,
            "test",
            "test",
            backup_policy.clone(),
            control_tx.clone(),
            control_tx.subscribe(),
            data_rx,
            metrics,
        )
        .await
        .unwrap();

        // Send a data message to the backup manager.
        let data_message = DataMessage {
            request: IngestWithConfigDataStreamRequest {
                ingestion_config_id: "test-0".to_string(),
                flow: "some_flow".to_string(),
                timestamp: Some(*TimeValue::now()),
                channel_values: vec![IngestWithConfigDataChannelValue {
                    r#type: Some(Type::Int32(1)),
                }],
                ..Default::default()
            },
            dropped_for_ingestion: false,
        };

        // Handle a few messages to fill up some backup files.
        for _ in 0..9 {
            assert!(
                backup_manager
                    .handle_data_message(data_message.clone())
                    .await
                    .is_ok(),
                "data message should be handled"
            );
        }

        // Save the backup file paths.
        let mut backup_file_paths = backup_manager.backup_files.clone();
        assert!(
            !backup_file_paths.is_empty(),
            "backup files should be present, pending checkpoint completion"
        );

        // During a checkpoint, the current file is rotated to the backup files list so we expect to see it
        // added to the existing backup files list after the call to `checkpoint()`.
        backup_file_paths.push(backup_manager.current_file_metadata.path.clone());

        // Set the checkpoint needs reingestion flag.
        backup_manager.checkpoint_needs_reingest = true;

        // Process the checkpoint.
        assert!(
            backup_manager.checkpoint().await.is_ok(),
            "checkpoint should be processed"
        );

        // The backup files pending checkpoint completion should be empty.
        assert!(
            backup_manager.backup_files.is_empty(),
            "backup file paths should be empty"
        );

        // The backup files should still exist (so they can be re-ingested).
        for backup_file_path in backup_file_paths.iter() {
            assert!(backup_file_path.exists(), "backup file should still exist");
        }

        // The control message should have been sent for reingestion.
        assert_eq!(
            control_rx.try_recv(),
            Ok(ControlMessage::ReingestBackups {
                backup_files: backup_file_paths
            })
        );
    }

    #[tokio::test]
    async fn test_async_manager_dropped_for_ingestion_causes_reingestion() {
        let tmp_dir = TempDir::new("test_async_manager").unwrap();
        let tmp_dir_path = tmp_dir.path();
        let backup_policy = DiskBackupPolicy {
            backups_dir: Some(tmp_dir_path.to_path_buf()),
            max_backup_file_size: 64,
            rolling_file_policy: RollingFilePolicy {
                max_file_count: Some(10),
            },
            retain_backups: false,
        };
        let (control_tx, control_rx) = broadcast::channel(1024);
        let (_data_tx, data_rx) = async_channel::bounded(1024);
        let metrics = Arc::new(SiftStreamMetrics::default());
        let mut backup_manager = AsyncBackupsManager::new(
            true,
            "test",
            "test",
            backup_policy.clone(),
            control_tx,
            control_rx,
            data_rx,
            metrics,
        )
        .await
        .unwrap();

        // Send a data message to the backup manager.
        let data_message = DataMessage {
            request: IngestWithConfigDataStreamRequest {
                ingestion_config_id: "test-0".to_string(),
                flow: "some_flow".to_string(),
                timestamp: Some(*TimeValue::now()),
                channel_values: vec![IngestWithConfigDataChannelValue {
                    r#type: Some(Type::Int32(1)),
                }],
                ..Default::default()
            },
            dropped_for_ingestion: true,
        };

        assert!(
            backup_manager
                .handle_data_message(data_message.clone())
                .await
                .is_ok(),
            "data message should be handled"
        );

        // The checkpoint needs reingestion flag should be set.
        assert!(
            backup_manager.checkpoint_needs_reingest,
            "checkpoint needs reingestion flag should be set"
        );
    }

    #[tokio::test]
    #[traced_test]
    async fn test_async_manager_shutdown_ungracefully() {
        let tmp_dir = TempDir::new("test_async_manager").unwrap();
        let tmp_dir_path = tmp_dir.path();
        let backup_policy = DiskBackupPolicy {
            backups_dir: Some(tmp_dir_path.to_path_buf()),
            max_backup_file_size: 1024,
            rolling_file_policy: RollingFilePolicy {
                max_file_count: Some(10),
            },
            retain_backups: false,
        };
        let (control_tx, control_rx) = broadcast::channel(1024);
        let (data_tx, data_rx) = async_channel::bounded(1024);
        let metrics = Arc::new(SiftStreamMetrics::default());
        let mut backup_manager = AsyncBackupsManager::new(
            true,
            "test",
            "test",
            backup_policy.clone(),
            control_tx.clone(),
            control_rx,
            data_rx.clone(),
            metrics,
        )
        .await
        .unwrap();

        let backup_task = tokio::spawn(async move { backup_manager.run().await });
        data_tx.close();

        // Wait for the backup manager to start the cleanup process.
        while !logs_contain("backup manager cleanup started") {
            tokio::time::sleep(Duration::from_millis(100)).await;
        }

        // Complete the final checkpoint.
        assert!(
            control_tx.send(ControlMessage::CheckpointComplete).is_ok(),
            "control message should be sent to the backup manager"
        );

        assert!(
            backup_task.await.is_ok(),
            "backup task should complete successfully"
        );
    }

    #[tokio::test]
    async fn test_async_manager_end_to_end() {
        let tmp_dir = TempDir::new("test_async_manager").unwrap();
        let tmp_dir_path = tmp_dir.path();
        let backup_policy = DiskBackupPolicy {
            backups_dir: Some(tmp_dir_path.to_path_buf()),
            max_backup_file_size: 1024,
            rolling_file_policy: RollingFilePolicy {
                max_file_count: Some(10),
            },
            retain_backups: false,
        };
        let (control_tx, control_rx) = broadcast::channel(1024);
        let (data_tx, data_rx) = async_channel::bounded(1024);
        let metrics = Arc::new(SiftStreamMetrics::default());
        let mut backup_manager = AsyncBackupsManager::new(
            true,
            "test",
            "test",
            backup_policy.clone(),
            control_tx.clone(),
            control_rx,
            data_rx.clone(),
            metrics,
        )
        .await
        .unwrap();

        // A task to simulate the data and messages the data manager is expected to see
        // and handle during runtime.
        let control_tx_clone = control_tx.clone();
        let simulator_task = tokio::spawn(async move {
            let mut data_message = DataMessage {
                request: IngestWithConfigDataStreamRequest {
                    ingestion_config_id: "test-0".to_string(),
                    flow: "some_flow".to_string(),
                    timestamp: Some(*TimeValue::now()),
                    channel_values: vec![IngestWithConfigDataChannelValue {
                        r#type: Some(Type::Int32(1)),
                    }],
                    ..Default::default()
                },
                dropped_for_ingestion: false,
            };

            // Send some messages over the channel.
            for _ in 0..100 {
                assert!(
                    data_tx.try_send(data_message.clone()).is_ok(),
                    "data message should be sent to the backup manager"
                );
            }

            // Send a message that was dropped for ingestion too.
            data_message.dropped_for_ingestion = true;
            assert!(
                data_tx.try_send(data_message.clone()).is_ok(),
                "data message should be sent to the backup manager"
            );

            // Wait for the backup manager to process some of the messages.
            while data_tx.len() > 50 {
                tokio::time::sleep(Duration::from_millis(100)).await;
            }

            // Indicate that the checkpoint needs reingestion.
            assert!(
                control_tx_clone
                    .send(ControlMessage::CheckpointNeedsReingestion)
                    .is_ok(),
                "control message should be sent to the backup manager"
            );

            // Send some some more data messages.
            for _ in 0..100 {
                assert!(
                    data_tx.try_send(data_message.clone()).is_ok(),
                    "data message should be sent to the backup manager"
                );
            }

            // Complete a checkpoint.
            assert!(
                control_tx_clone
                    .send(ControlMessage::CheckpointComplete)
                    .is_ok(),
                "control message should be sent to the backup manager"
            );

            // Finally trigger a shutdown.
            assert!(
                control_tx_clone.send(ControlMessage::Shutdown).is_ok(),
                "control message should be sent to the backup manager"
            );
            assert!(
                control_tx_clone
                    .send(ControlMessage::CheckpointComplete)
                    .is_ok(),
                "control message should be sent to the backup manager"
            );

            data_tx.close();
        });

        let backup_task = tokio::spawn(async move {
            assert!(
                backup_manager.run().await.is_ok(),
                "backup task should complete successfully"
            );
            assert!(
                backup_manager.backup_files.is_empty(),
                "backup files should be empty on shutdown"
            );
            assert!(
                backup_manager.data_rx.is_empty(),
                "data receiver should be empty {}",
                backup_manager.data_rx.len()
            );
        });

        // Wait for the tasks to complete, with a timeout.
        let tasks_join = tokio::time::timeout(Duration::from_secs(10), async {
            tokio::join!(backup_task, simulator_task)
        })
        .await;

        let (backup_task_result, simulator_task_result) =
            tasks_join.expect("timeout waiting for backup and simulator tasks");
        assert!(
            backup_task_result.is_ok(),
            "backup task should complete successfully"
        );
        assert!(
            simulator_task_result.is_ok(),
            "simulator task should complete successfully"
        );

        // Final check that the data receiver was drained by the backup manager before it shutdown.
        assert!(
            data_rx.is_empty(),
            "data receiver should be empty {}",
            data_rx.len()
        );
    }

    ///  Helper function to create a backup file for a test.
    async fn create_test_backup_file(
        tmp_dir: &TempDir,
        file_name: &str,
        num_messages: usize,
    ) -> PathBuf {
        let tmp_file_path = tmp_dir.path().join(file_name);
        let mut tmp_file = tokio::fs::File::create(&tmp_file_path)
            .await
            .expect("failed to create file");

        // Write some messages to a backup file that will be re-ingested during the test.
        for i in 0..num_messages {
            let data = IngestWithConfigDataStreamRequest {
                ingestion_config_id: "test-0".to_string(),
                flow: "some_flow".to_string(),
                timestamp: Some(*TimeValue::now()),
                channel_values: vec![IngestWithConfigDataChannelValue {
                    r#type: Some(Type::Int32(i as i32)),
                }],
                ..Default::default()
            };

            let chunk = PbfsChunk::new(&[data]).expect("failed to create pbfs chunk");
            assert!(
                tmp_file.write_all(&chunk).await.is_ok(),
                "failed to write to file"
            );
        }

        // Sync and close the file so it can be re-ingested and removed below.
        assert!(tmp_file.sync_all().await.is_ok(), "failed to sync file");
        drop(tmp_file);

        tmp_file_path
    }

    #[tokio::test]
    async fn test_backup_ingest_reingest_files() {
        let tmp_dir = TempDir::new("testbackup_ingest_reingest_files").unwrap();
        let tmp_file_path = create_test_backup_file(&tmp_dir, "backup_file", 100).await;
        assert!(tmp_file_path.exists(), "file should exist");

        let (to_reingest_tx, to_reingest_rx) = async_channel::bounded(1024);
        let retry_policy = RetryPolicy::default();
        let retain_backups = false;
        let enable_compression_for_ingestion = false;
        let (grpc_channel, mock_service) =
            crate::test::create_mock_grpc_channel_with_service().await;
        let metrics = Arc::new(SiftStreamMetrics::default());

        // Send the file that needs to be re-ingested, then close the channel so the re-ingestion future can complete.
        assert!(
            to_reingest_tx.try_send(tmp_file_path.clone()).is_ok(),
            "failed to send file path to reingest task"
        );
        to_reingest_tx.close();

        // Re-ingest the file.
        assert!(
            BackupIngestTask::reingest_files(
                to_reingest_rx,
                grpc_channel,
                retry_policy,
                enable_compression_for_ingestion,
                retain_backups,
                metrics
            )
            .await
            .is_ok(),
            "failed to reingest files"
        );

        // The file should have been removed.
        assert!(!tmp_file_path.exists(), "file should have been removed");

        // Verify the captured messages were all received (no data lost).
        let captured = mock_service.get_captured_data();
        assert_eq!(captured.len(), 100, "should have captured 100 messages");
        for (index, message) in captured.iter().enumerate() {
            assert_eq!(
                message.ingestion_config_id, "test-0",
                "ingestion config id should be test-0"
            );
            assert_eq!(message.flow, "some_flow", "flow should be some_flow");
            assert_eq!(
                message.channel_values.len(),
                1,
                "should have one channel value"
            );
            assert_eq!(
                message.channel_values[0].r#type,
                Some(Type::Int32(index as i32)),
                "channel value should be int32(1)"
            );
        }
    }

    #[tokio::test]
    async fn test_backup_ingest_reingest_files_retain_backups() {
        let tmp_dir = TempDir::new("testbackup_ingest_reingest_files_retain_backups").unwrap();
        let tmp_file_path = create_test_backup_file(&tmp_dir, "backup_file", 100).await;
        assert!(tmp_file_path.exists(), "file should exist");

        let (to_reingest_tx, to_reingest_rx) = async_channel::bounded(1024);
        let retry_policy = RetryPolicy::default();
        let retain_backups = true;
        let enable_compression_for_ingestion = false;
        let (grpc_channel, mock_service) =
            crate::test::create_mock_grpc_channel_with_service().await;
        let metrics = Arc::new(SiftStreamMetrics::default());

        // Send the file that needs to be re-ingested, then close the channel so the re-ingestion future can complete.
        assert!(
            to_reingest_tx.try_send(tmp_file_path.clone()).is_ok(),
            "failed to send file path to reingest task"
        );
        to_reingest_tx.close();

        // Re-ingest the file.
        assert!(
            BackupIngestTask::reingest_files(
                to_reingest_rx,
                grpc_channel,
                retry_policy,
                enable_compression_for_ingestion,
                retain_backups,
                metrics
            )
            .await
            .is_ok(),
            "failed to reingest files"
        );

        // The file should still exist since the backups are retained.
        assert!(tmp_file_path.exists(), "file should still exist");

        // Verify the captured messages were all received (no data lost).
        let captured = mock_service.get_captured_data();
        assert_eq!(captured.len(), 100, "should have captured 100 messages");
        for (index, message) in captured.iter().enumerate() {
            assert_eq!(
                message.ingestion_config_id, "test-0",
                "ingestion config id should be test-0"
            );
            assert_eq!(message.flow, "some_flow", "flow should be some_flow");
            assert_eq!(
                message.channel_values.len(),
                1,
                "should have one channel value"
            );
            assert_eq!(
                message.channel_values[0].r#type,
                Some(Type::Int32(index as i32)),
                "channel value should be int32(1)"
            );
        }
    }

    #[tokio::test]
    async fn test_backup_ingest_reingest_files_retries() {
        let tmp_dir = TempDir::new("testbackup_ingest_reingest_files").unwrap();
        let tmp_file_path = create_test_backup_file(&tmp_dir, "backup_file", 100).await;
        assert!(tmp_file_path.exists(), "file should exist");

        let (to_reingest_tx, to_reingest_rx) = async_channel::bounded(1024);
        let retry_policy = RetryPolicy {
            max_attempts: 3,
            initial_backoff: Duration::from_millis(100),
            max_backoff: Duration::from_secs(1),
            backoff_multiplier: 5,
        };
        let retain_backups = false;
        let enable_compression_for_ingestion = false;
        let (grpc_channel, mock_service) =
            crate::test::create_mock_grpc_channel_with_service().await;
        let metrics = Arc::new(SiftStreamMetrics::default());

        // Set the mock service to return an error for all but the last ingestion attempt.
        let expected_retries = retry_policy.max_attempts as usize - 1;
        mock_service.set_num_errors_to_return(expected_retries);

        // Send the file that needs to be re-ingested, then close the channel so the re-ingestion future can complete.
        assert!(
            to_reingest_tx.try_send(tmp_file_path.clone()).is_ok(),
            "failed to send file path to reingest task"
        );
        to_reingest_tx.close();

        // Re-ingest the file.
        assert!(
            BackupIngestTask::reingest_files(
                to_reingest_rx,
                grpc_channel,
                retry_policy,
                enable_compression_for_ingestion,
                retain_backups,
                metrics.clone()
            )
            .await
            .is_ok(),
            "failed to reingest files"
        );

        // The file should have been removed.
        assert!(!tmp_file_path.exists(), "file should have been removed");

        // Verify the captured messages were all received (no data lost).
        let captured = mock_service.get_captured_data();
        assert_eq!(captured.len(), 100, "should have captured 100 messages");
        for (index, message) in captured.iter().enumerate() {
            assert_eq!(
                message.ingestion_config_id, "test-0",
                "ingestion config id should be test-0"
            );
            assert_eq!(message.flow, "some_flow", "flow should be some_flow");
            assert_eq!(
                message.channel_values.len(),
                1,
                "should have one channel value"
            );
            assert_eq!(
                message.channel_values[0].r#type,
                Some(Type::Int32(index as i32)),
                "channel value should be int32(1)"
            );
        }

        // Verify retries were performed and tracked with metrics.
        assert_eq!(
            metrics.backups.cur_ingest_retries.get(),
            expected_retries as u64,
            "should have retried the maximum number of times"
        );
    }

    #[tokio::test]
    async fn test_backup_ingest_reingest_files_retries_exhausted() {
        let tmp_dir = TempDir::new("testbackup_ingest_reingest_files_retries_exhausted").unwrap();
        let tmp_file_path = create_test_backup_file(&tmp_dir, "backup_file", 100).await;
        assert!(tmp_file_path.exists(), "file should exist");

        let (to_reingest_tx, to_reingest_rx) = async_channel::bounded(1024);
        let retry_policy = RetryPolicy {
            max_attempts: 1,
            initial_backoff: Duration::from_millis(100),
            max_backoff: Duration::from_secs(1),
            backoff_multiplier: 5,
        };
        let retain_backups = false;
        let enable_compression_for_ingestion = false;
        let (grpc_channel, mock_service) =
            crate::test::create_mock_grpc_channel_with_service().await;
        let metrics = Arc::new(SiftStreamMetrics::default());

        // Set the mock service to return an error for all but the last ingestion attempt.
        let expected_retries = retry_policy.max_attempts as usize;
        mock_service.set_num_errors_to_return(expected_retries);

        // Send the file that needs to be re-ingested, then close the channel so the re-ingestion future can complete.
        assert!(
            to_reingest_tx.try_send(tmp_file_path.clone()).is_ok(),
            "failed to send file path to reingest task"
        );
        to_reingest_tx.close();

        // Re-ingest the file.
        assert!(
            BackupIngestTask::reingest_files(
                to_reingest_rx,
                grpc_channel,
                retry_policy,
                enable_compression_for_ingestion,
                retain_backups,
                metrics.clone()
            )
            .await
            .is_ok(),
            "failed to reingest files"
        );

        // The file should still exist since the retries were exhausted and the file was not successfully re-ingested.
        assert!(tmp_file_path.exists(), "file should still exist");

        // Verify retries were performed and tracked with metrics.
        assert_eq!(
            metrics.backups.cur_ingest_retries.get(),
            expected_retries as u64,
            "should have retried the maximum number of times"
        );
    }

    #[tokio::test]
    async fn test_backup_ingest_end_to_end() {
        let tmp_dir = TempDir::new("testbackup_ingest_end_to_end").unwrap();
        let backup0_file_path = create_test_backup_file(&tmp_dir, "backup_file_0", 100).await;
        let backup1_file_path = create_test_backup_file(&tmp_dir, "backup_file_1", 100).await;
        let backup2_file_path = create_test_backup_file(&tmp_dir, "backup_file_2", 100).await;

        // Verify the backup files exist.
        assert!(backup0_file_path.exists(), "backup0 file should exist");
        assert!(backup1_file_path.exists(), "backup1 file should exist");
        assert!(backup2_file_path.exists(), "backup2 file should exist");

        let (control_tx, control_rx) = broadcast::channel(1024);
        // Send the control message to re-ingest the backup files.
        assert!(
            control_tx
                .send(ControlMessage::ReingestBackups {
                    backup_files: vec![
                        backup0_file_path.clone(),
                        backup1_file_path.clone(),
                        backup2_file_path.clone()
                    ]
                })
                .is_ok(),
            "failed to send control message to reingest task"
        );

        // Create the re-ingestion task.
        let retry_policy = RetryPolicy::default();
        let retain_backups = false;
        let enable_compression_for_ingestion = false;
        let (grpc_channel, mock_service) =
            crate::test::create_mock_grpc_channel_with_service().await;
        let metrics = Arc::new(SiftStreamMetrics::default());

        let reingest_task = BackupIngestTask::new(
            control_rx,
            grpc_channel,
            enable_compression_for_ingestion,
            retry_policy,
            retain_backups,
            metrics,
        );
        let reingest_task = tokio::spawn(async move { reingest_task.run().await });

        // Wait for the re-ingestion task to complete the uploads.
        while mock_service.get_captured_data().len() < 300 {
            tokio::time::sleep(Duration::from_millis(100)).await;
        }

        // Verify the captured messages.
        let captured = mock_service.get_captured_data();
        assert_eq!(captured.len(), 300, "should have captured 300 messages");
        for (index, message) in captured.iter().enumerate() {
            let i = index % 100;
            assert_eq!(
                message.ingestion_config_id, "test-0",
                "ingestion config id should be test-0"
            );
            assert_eq!(message.flow, "some_flow", "flow should be some_flow");
            assert_eq!(
                message.channel_values.len(),
                1,
                "should have one channel value"
            );
            assert_eq!(
                message.channel_values[0].r#type,
                Some(Type::Int32(i as i32)),
                "channel value should be int32({i})"
            );
        }

        // The backup files should have been removed.
        assert!(
            !backup0_file_path.exists(),
            "backup0 file should have been removed"
        );
        assert!(
            !backup1_file_path.exists(),
            "backup1 file should have been removed"
        );
        assert!(
            !backup2_file_path.exists(),
            "backup2 file should have been removed"
        );

        // Send the shutdown message to verify graceful shutdown.
        assert!(
            control_tx.send(ControlMessage::Shutdown).is_ok(),
            "failed to send shutdown message to reingest task"
        );

        // Wait for the re-ingestion task to complete.
        assert!(
            reingest_task.await.is_ok(),
            "reingest task should complete successfully"
        );
    }

    #[tokio::test]
    async fn test_backup_ingest_ungraceful_shutdown() {
        let tmp_dir = TempDir::new("testbackup_ingest_ungraceful_shutdown").unwrap();
        let backup0_file_path = create_test_backup_file(&tmp_dir, "backup_file_0", 100).await;

        // Verify the backup files exist.
        assert!(backup0_file_path.exists(), "backup0 file should exist");

        let (control_tx, control_rx) = broadcast::channel(1024);

        // Create the re-ingestion task.
        let retry_policy = RetryPolicy::default();
        let retain_backups = false;
        let enable_compression_for_ingestion = false;
        let (grpc_channel, _mock_service) =
            crate::test::create_mock_grpc_channel_with_service().await;
        let metrics = Arc::new(SiftStreamMetrics::default());

        let reingest_task = BackupIngestTask::new(
            control_rx,
            grpc_channel,
            enable_compression_for_ingestion,
            retry_policy,
            retain_backups,
            metrics,
        );

        drop(control_tx);
        assert!(
            reingest_task.run().await.is_ok(),
            "failed to run reingest task"
        );

        // The backup file should not have been removed.
        assert!(
            backup0_file_path.exists(),
            "backup0 file should not have been removed"
        );
    }
}
