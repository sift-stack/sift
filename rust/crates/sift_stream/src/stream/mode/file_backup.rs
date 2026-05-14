use crate::stream::mode::ingestion_config::IngestionConfigEncoder;
use crate::stream::send_error::{SendError, TrySendError};
use crate::stream::{SiftStream, Transport, private::Sealed};
use crate::{
    backup::disk::file_writer::{FileWriter, FileWriterConfig},
    metrics::SiftStreamMetrics,
    stream::flow::FlowDescriptor,
};
use async_channel::{Receiver, Sender};
use async_trait::async_trait;
use prost::Message;
use sift_connect::SiftChannel;
use sift_error::prelude::*;
use sift_rs::{
    ingest::v1::IngestWithConfigDataStreamRequest, ingestion_configs::v2::IngestionConfig,
    runs::v2::Run,
};
use std::collections::HashSet;
use std::io::ErrorKind as IoErrorKind;
use std::{collections::HashMap, path::PathBuf, sync::Arc, time::Duration};
use tokio::fs;
use tokio::{sync::broadcast, task::JoinHandle};
use uuid::Uuid;

/// Handles writing backup requests to disk files.
struct FileBackupWriter {
    file_writer: FileWriter,
    metrics: Arc<SiftStreamMetrics>,
}

impl FileBackupWriter {
    /// Creates a new `FileBackupWriter` with the given configuration.
    fn new(file_writer_config: FileWriterConfig, metrics: Arc<SiftStreamMetrics>) -> Self {
        Self {
            file_writer: FileWriter::new(file_writer_config),
            metrics,
        }
    }

    /// Handles a single request by writing it to the current file.
    async fn handle_request(&mut self, request: &IngestWithConfigDataStreamRequest) -> Result<()> {
        // Check if we need to rotate the file
        if self.file_writer.should_rotate_file() {
            let _ = self.file_writer.rotate_file().await?;
            self.metrics.backups.log_new_file();
        }

        // Write the request to file
        self.file_writer.write_request(request).await?;

        // Calculate message size for metrics.
        self.metrics
            .backups
            .log_message(request.encoded_len() as u64);

        Ok(())
    }

    /// Flushes and syncs the current file.
    async fn finalize(&mut self) -> Result<()> {
        self.file_writer.flush().await?;
        self.file_writer.sync().await?;
        Ok(())
    }

    /// Main loop that drains the channel and processes requests.
    ///
    /// This function will run until the channel is closed, then finalize the file.
    pub(crate) async fn run(
        mut self,
        write_rx: Receiver<Arc<IngestWithConfigDataStreamRequest>>,
    ) -> Result<()> {
        while let Ok(request) = write_rx.recv().await {
            // Log errors if they occur, but only break/return when the channel is closed
            // and empty.
            if let Err(e) = self.handle_request(&request).await {
                #[cfg(feature = "tracing")]
                tracing::error!(
                    error = %e,
                    "error handling request"
                );
            }
        }

        // Flush and sync the current file before finishing
        self.finalize().await?;

        Ok(())
    }
}

/// Transport that writes messages to rolling disk files without live network ingestion.
///
/// Messages are queued on a single bounded write channel and consumed by a background
/// file-writer task that manages rolling files according to the configured
/// [`DiskBackupPolicy`](crate::DiskBackupPolicy).
///
/// **Backpressure**: [`send`](crate::SiftStream::send) awaits when the **write channel**
/// is full. The channel capacity is set via
/// [`FileBackupBuilder::backup_data_channel_capacity`](crate::FileBackupBuilder::backup_data_channel_capacity)
/// (default: [`DATA_CHANNEL_CAPACITY`](crate::stream::tasks::DATA_CHANNEL_CAPACITY)).
///
/// Use this mode for CI/CD workflows where data only needs to reach Sift if a test fails,
/// or in environments where network connectivity is unavailable during the recording session.
/// Data written by this mode can be ingested into Sift later using separate tooling.
pub struct FileBackup {
    write_tx: Sender<Arc<IngestWithConfigDataStreamRequest>>,
    write_task: JoinHandle<Result<()>>,
    control_tx: broadcast::Sender<crate::stream::tasks::ControlMessage>,
    metrics_streaming: Option<JoinHandle<Result<()>>>,
    flows_seen: HashSet<String>,
    metrics: Arc<SiftStreamMetrics>,
}

// Seal the trait - only this crate can implement SiftStreamMode
impl Sealed for FileBackup {}

impl FileBackup {
    fn prepare_message(
        &mut self,
        stream_id: &Uuid,
        message: IngestWithConfigDataStreamRequest,
    ) -> Arc<IngestWithConfigDataStreamRequest> {
        self.metrics.messages_received.increment();

        #[cfg(feature = "tracing")]
        {
            if !self.flows_seen.contains(&message.flow) {
                self.metrics.unique_flows_received.increment();
                self.flows_seen.insert(message.flow.clone());
                tracing::info!(sift_stream_id = %stream_id, "flow '{}' being ingested for the first time", &message.flow);
            }
        }

        self.metrics
            .backup_channel_depth
            .set(self.write_tx.len() as u64);
        Arc::new(message)
    }
}

#[async_trait]
impl Transport for FileBackup {
    type Encoder = IngestionConfigEncoder;
    type Message = IngestWithConfigDataStreamRequest;

    /// Sends a message to be written to disk, awaiting capacity on the **write channel**
    /// if it is full.
    ///
    /// Backpressure comes from the bounded write channel between the caller and the
    /// background file-writer task. Returns an error only if the channel is closed (i.e.
    /// the stream is shutting down), in which case the original message is returned inside
    /// `Err`.
    async fn send(
        &mut self,
        stream_id: &Uuid,
        message: Self::Message,
    ) -> std::result::Result<(), SendError<Self::Message>> {
        let arc = self.prepare_message(stream_id, message);

        self.write_tx
            .send(arc)
            .await
            .map_err(|async_channel::SendError(a)| {
                SendError(Arc::try_unwrap(a).unwrap_or_else(|a| (*a).clone()))
            })?;

        self.metrics.messages_sent.increment();
        Ok(())
    }

    /// Attempts to send a message without blocking.
    ///
    /// Returns immediately with `TrySendError::Full` if the **write channel** is at
    /// capacity, or `TrySendError::Closed` if the channel has been closed. In either case
    /// the original message is returned unchanged inside the error variant.
    fn try_send(
        &mut self,
        stream_id: &Uuid,
        message: Self::Message,
    ) -> std::result::Result<(), TrySendError<Self::Message>> {
        let arc = self.prepare_message(stream_id, message);

        match self.write_tx.try_send(arc) {
            Ok(()) => {
                self.metrics.messages_sent.increment();
                Ok(())
            }
            Err(async_channel::TrySendError::Full(a)) => Err(TrySendError::Full(
                Arc::try_unwrap(a).unwrap_or_else(|a| (*a).clone()),
            )),
            Err(async_channel::TrySendError::Closed(a)) => Err(TrySendError::Closed(
                Arc::try_unwrap(a).unwrap_or_else(|a| (*a).clone()),
            )),
        }
    }

    /// Sends a batch of messages in order to be written to the backup file, awaiting
    /// capacity for each one.
    ///
    /// On stream close, stops immediately and returns the undelivered messages starting
    /// from the point of failure. The first element of the returned `Vec` is always the
    /// message that failed to send.
    async fn send_requests<I>(
        &mut self,
        stream_id: &Uuid,
        requests: I,
    ) -> std::result::Result<(), SendError<Vec<Self::Message>>>
    where
        I: IntoIterator<Item = Self::Message> + Send,
        I::IntoIter: Send,
    {
        let mut iter = requests.into_iter();
        while let Some(msg) = iter.next() {
            if let Err(SendError(failed)) = self.send(stream_id, msg).await {
                let mut undelivered = vec![failed];
                undelivered.extend(iter);
                return Err(SendError(undelivered));
            }
        }
        Ok(())
    }

    /// Attempts to send a batch of messages in order without blocking.
    ///
    /// Stops and returns on the first failure. The returned `Vec` contains the undelivered
    /// messages starting from the point of failure, with the first element always being
    /// the message that failed to send.
    fn try_send_requests<I>(
        &mut self,
        stream_id: &Uuid,
        requests: I,
    ) -> std::result::Result<(), TrySendError<Vec<Self::Message>>>
    where
        I: IntoIterator<Item = Self::Message> + Send,
        I::IntoIter: Send,
    {
        let mut iter = requests.into_iter();
        while let Some(msg) = iter.next() {
            match self.try_send(stream_id, msg) {
                Ok(()) => {}
                Err(TrySendError::Full(failed)) => {
                    let mut undelivered = vec![failed];
                    undelivered.extend(iter);
                    return Err(TrySendError::Full(undelivered));
                }
                Err(TrySendError::Closed(failed)) => {
                    let mut undelivered = vec![failed];
                    undelivered.extend(iter);
                    return Err(TrySendError::Closed(undelivered));
                }
            }
        }
        Ok(())
    }

    /// This will conclude the stream and flush any remaining data to disk.
    async fn finish(self, stream_id: &Uuid) -> Result<()> {
        // Send shutdown signal to metrics streaming task
        let _ = self
            .control_tx
            .send(crate::stream::tasks::ControlMessage::Shutdown);

        // Close the channel to signal the background task to finish
        drop(self.write_tx);

        // Wait for the background task to complete
        self.write_task.await.map_err(|e| {
            Error::new_msg(
                ErrorKind::StreamError,
                format!("file backup write task panicked: {e}"),
            )
        })??;

        // Wait for metrics streaming task to complete if it exists
        if let Some(metrics_streaming) = self.metrics_streaming {
            let _ = metrics_streaming.await;
        }

        #[cfg(feature = "tracing")]
        tracing::info!(
            sift_stream_id = %stream_id,
            "successfully finished file backup stream"
        );

        Ok(())
    }
}

impl FileBackup {
    /// Creates a new [`FileBackup`] and spawns the background file-writing task.
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
        file_writer_config: FileWriterConfig,
        channel_capacity: usize,
        metrics: Arc<SiftStreamMetrics>,
        control_channel_capacity: usize,
        metrics_streaming_interval: Option<Duration>,
        setup_channel: SiftChannel,
        session_name: String,
        sift_stream_id: Uuid,
    ) -> Result<Self> {
        // Create channel for sending requests to the background write task
        let (write_tx, write_rx): (
            Sender<Arc<IngestWithConfigDataStreamRequest>>,
            Receiver<Arc<IngestWithConfigDataStreamRequest>>,
        ) = async_channel::bounded(channel_capacity);

        // Create the file backup writer
        let writer = FileBackupWriter::new(file_writer_config, metrics.clone());

        // Spawn background task to handle file writing
        let write_task = tokio::spawn(async move { writer.run(write_rx).await });

        // Create control channel for metrics streaming task
        let (control_tx, _control_rx) = broadcast::channel(control_channel_capacity);

        // Start metrics streaming task if interval is configured
        let metrics_streaming = if let Some(interval) = metrics_streaming_interval {
            let control_rx = control_tx.subscribe();
            let metrics_clone = metrics.clone();
            Some(tokio::spawn(async move {
                let metrics_task = crate::stream::tasks::MetricsStreamingTask::new(
                    setup_channel,
                    control_rx,
                    session_name.clone(),
                    interval,
                    metrics_clone,
                    None,
                )
                .await?;

                #[cfg(feature = "tracing")]
                tracing::info!(
                    sift_stream_id = %sift_stream_id,
                    "metrics streaming task started for file backup mode"
                );
                metrics_task.run().await
            }))
        } else {
            None
        };

        Ok(Self {
            write_tx,
            write_task,
            control_tx,
            metrics_streaming,
            flows_seen: HashSet::new(),
            metrics,
        })
    }
}

impl SiftStream<IngestionConfigEncoder, FileBackup> {
    /// Initializes a new [SiftStream] for file backup mode. Users should instead use [`SiftStreamBuilder`].
    ///
    /// [`SiftStreamBuilder`]: crate::stream::builder::SiftStreamBuilder
    #[allow(clippy::too_many_arguments)]
    pub(crate) async fn new_file_backup(
        grpc_channel: SiftChannel,
        ingestion_config: IngestionConfig,
        flows_by_name: HashMap<String, FlowDescriptor<String>>,
        run: Option<Run>,
        backups_directory: PathBuf,
        output_directory: PathBuf,
        max_file_size: usize,
        channel_capacity: usize,
        control_channel_capacity: usize,
        metrics_streaming_interval: Option<Duration>,
        session_name: String,
        sift_stream_id: Uuid,
        metrics: Arc<SiftStreamMetrics>,
    ) -> Result<Self> {
        let full_backup_path = backups_directory.join(output_directory);

        // Ensure the output directory exists
        if let Err(err) = fs::create_dir_all(&full_backup_path).await
            && err.kind() != IoErrorKind::AlreadyExists
        {
            return Err(Error::new(ErrorKind::BackupsError, err))
                .with_context(|| format!("failed to create directory for backups at {}", full_backup_path.display()))
                .help("if using a custom path for backups directory ensure that it's valid with proper permissions, otherwise contact Sift");
        }

        let file_writer_config = FileWriterConfig {
            directory: full_backup_path,
            prefix: ingestion_config.client_key.clone(),
            max_size: max_file_size,
        };

        let file_backup = FileBackup::new(
            file_writer_config,
            channel_capacity,
            metrics.clone(),
            control_channel_capacity,
            metrics_streaming_interval,
            grpc_channel.clone(),
            session_name,
            sift_stream_id,
        )?;

        Ok(Self {
            grpc_channel: grpc_channel.clone(),
            encoder: IngestionConfigEncoder {
                grpc_channel,
                flows_by_name,
                ingestion_config,
                metrics,
            },
            transport: file_backup,
            run,
            sift_stream_id,
        })
    }
}

// Re-export Flow type for convenience
pub use super::ingestion_config::Flow;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::create_mock_grpc_channel_with_service;
    use crate::{FlowBuilder, TimeValue};
    use sift_rs::common::r#type::v1::ChannelDataType;
    use std::collections::HashMap;
    use tempdir::TempDir;

    /// Waits for backup metrics to reach expected values with a timeout.
    /// This is used to wait for the background task to process messages.
    async fn wait_for_backup_metrics(
        metrics: &SiftStreamMetrics,
        expected_total_messages: u64,
        timeout_ms: u64,
    ) {
        let start = std::time::Instant::now();
        let timeout = tokio::time::Duration::from_millis(timeout_ms);

        loop {
            let total_messages = metrics.backups.total_messages.get();
            if total_messages >= expected_total_messages {
                return;
            }

            if start.elapsed() > timeout {
                panic!(
                    "Timeout waiting for backup metrics: expected {} messages, got {}",
                    expected_total_messages, total_messages
                );
            }

            tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
        }
    }

    /// Helper function to create a FileBackupMode for tests
    async fn create_test_file_backup_mode(
        file_writer_config: FileWriterConfig,
        channel_capacity: usize,
        metrics: Arc<SiftStreamMetrics>,
    ) -> FileBackup {
        let (grpc_channel, _) = create_mock_grpc_channel_with_service().await;
        FileBackup::new(
            file_writer_config,
            channel_capacity,
            metrics,
            100,  // control_channel_capacity
            None, // metrics_streaming_interval - disabled for tests
            grpc_channel,
            "test_session".to_string(),
            Uuid::new_v4(),
        )
        .unwrap()
    }

    mod file_backup_writer {
        use super::*;

        fn create_test_request(
            flow: &str,
            ingestion_config_id: &str,
        ) -> IngestWithConfigDataStreamRequest {
            IngestWithConfigDataStreamRequest {
                ingestion_config_id: ingestion_config_id.to_string(),
                flow: flow.to_string(),
                timestamp: None,
                channel_values: vec![],
                run_id: Uuid::new_v4().to_string(),
                end_stream_on_validation_error: false,
                organization_id: Uuid::new_v4().to_string(),
            }
        }

        #[tokio::test]
        async fn test_file_backup_writer_handle_request() {
            let temp_dir = TempDir::new("test_file_backup_writer").unwrap();
            let config = FileWriterConfig {
                directory: temp_dir.path().to_path_buf(),
                prefix: "test".to_string(),
                max_size: 1024 * 1024, // 1MB
            };

            let metrics = Arc::new(SiftStreamMetrics::default());
            let mut writer = FileBackupWriter::new(config, metrics);
            let ingestion_config_id = Uuid::new_v4().to_string();
            let request = create_test_request("test_flow", &ingestion_config_id);

            // Handle the request
            writer.handle_request(&request).await.unwrap();

            // Verify file was created
            assert!(writer.file_writer.current_file.is_some());
            assert_eq!(writer.file_writer.current_file_ctx.message_count, 1);
            assert!(writer.file_writer.current_file_ctx.file_size > 0);

            // Verify file exists on disk
            assert!(writer.file_writer.current_file_ctx.file_path.exists());
        }

        #[tokio::test]
        async fn test_file_backup_writer_handle_request_rotates_file() {
            let temp_dir = TempDir::new("test_file_backup_writer").unwrap();
            let config = FileWriterConfig {
                directory: temp_dir.path().to_path_buf(),
                prefix: "test".to_string(),
                max_size: 100, // Very small max size
            };

            let metrics = Arc::new(SiftStreamMetrics::default());
            let mut writer = FileBackupWriter::new(config, metrics);
            let ingestion_config_id = Uuid::new_v4().to_string();
            let request = create_test_request("test_flow", &ingestion_config_id);

            // Write one request to create a file and record it's file path.
            assert!(writer.handle_request(&request).await.is_ok());
            let file_path_before_rotation = writer.file_writer.current_file_ctx.file_path.clone();

            // Write requests until we need to rotate
            for _ in 0..100 {
                writer.handle_request(&request).await.unwrap();
                if writer.file_writer.current_file_ctx.file_path != file_path_before_rotation {
                    break;
                }
            }

            // Verify current file exists
            assert!(file_path_before_rotation.exists());
        }

        #[tokio::test]
        async fn test_file_backup_writer_finalize() {
            let temp_dir = TempDir::new("test_file_backup_writer").unwrap();
            let config = FileWriterConfig {
                directory: temp_dir.path().to_path_buf(),
                prefix: "test".to_string(),
                max_size: 1024 * 1024,
            };

            let metrics = Arc::new(SiftStreamMetrics::default());
            let mut writer = FileBackupWriter::new(config, metrics);
            let ingestion_config_id = Uuid::new_v4().to_string();
            let request = create_test_request("test_flow", &ingestion_config_id);

            // Write a request
            writer.handle_request(&request).await.unwrap();

            // Check file context before finalize
            let file_path = writer.file_writer.current_file_ctx.file_path.clone();
            let message_count_before = writer.file_writer.current_file_ctx.message_count;
            let file_size_before = writer.file_writer.current_file_ctx.file_size;

            // Verify file exists before finalize
            assert!(file_path.exists());
            assert_eq!(message_count_before, 1);
            assert!(file_size_before > 0);

            // Finalize should succeed
            writer.finalize().await.unwrap();

            // Verify file still exists after finalize
            assert!(file_path.exists());
        }

        #[tokio::test]
        async fn test_file_backup_writer_run_drains_channel() {
            let temp_dir = TempDir::new("test_file_backup_writer").unwrap();
            let config = FileWriterConfig {
                directory: temp_dir.path().to_path_buf(),
                prefix: "test".to_string(),
                max_size: 1024 * 1024,
            };

            let writer = FileBackupWriter::new(config, Arc::new(SiftStreamMetrics::default()));
            let (tx, rx) = async_channel::bounded(10);
            let ingestion_config_id = Uuid::new_v4().to_string();

            // Send some requests
            for i in 0..5 {
                let request = create_test_request(&format!("flow_{}", i), &ingestion_config_id);
                tx.send(Arc::new(request)).await.unwrap();
            }

            // Close the channel
            drop(tx);

            // Run should process all requests and complete
            // Note: We can't check file context after run since it consumes self
            // But we can verify the file exists by checking the directory
            writer.run(rx).await.unwrap();

            // Verify at least one file exists with the expected prefix
            let files: Vec<_> = std::fs::read_dir(temp_dir.path())
                .unwrap()
                .filter_map(|entry| entry.ok())
                .filter(|entry| {
                    entry
                        .path()
                        .file_name()
                        .and_then(|n| n.to_str())
                        .map(|n| n.starts_with("test-"))
                        .unwrap_or(false)
                })
                .collect();
            assert!(
                !files.is_empty(),
                "Expected at least one file with prefix 'test-' to be created"
            );

            // Verify the file has content (size > 0)
            for file in &files {
                let metadata = std::fs::metadata(file.path()).unwrap();
                assert!(metadata.len() > 0, "File should have content");
            }
        }

        #[tokio::test]
        async fn test_file_backup_writer_run_continues_after_handle_request_error() {
            let temp_dir = TempDir::new("test_file_backup_writer").unwrap();
            let backup_dir = temp_dir.path().join("backup_subdir");
            assert!(!backup_dir.exists(), "subdir must not exist yet");

            let config = FileWriterConfig {
                directory: backup_dir.clone(),
                prefix: "test".to_string(),
                max_size: 1024 * 1024,
            };

            let writer = FileBackupWriter::new(config, Arc::new(SiftStreamMetrics::default()));
            let (write_tx, write_rx) = async_channel::bounded(10);
            let ingestion_config_id = Uuid::new_v4().to_string();

            let run_handle = tokio::spawn(async move { writer.run(write_rx).await });

            // Send a request to create a file before the directory is created (this should fail internally but the task should continue running).
            let request = create_test_request("flow_0", &ingestion_config_id);
            write_tx.send(Arc::new(request)).await.unwrap();

            // Wait for the request to be consumed.
            while !write_tx.is_empty() {
                tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
            }

            // The task should not have finished yet.
            assert!(!run_handle.is_finished());

            // Create the necessary directory and send a second request. The file should be created successfully this time.
            tokio::fs::create_dir_all(&backup_dir).await.unwrap();

            let second_request = create_test_request("flow_1", &ingestion_config_id);
            write_tx.send(Arc::new(second_request)).await.unwrap();

            // Wait for the request to be consumed.
            while !write_tx.is_empty() {
                tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
            }

            // Drop the write channel to signal the task to finish.
            drop(write_tx);
            assert!(
                run_handle.await.unwrap().is_ok(),
                "run task should complete successfully."
            );

            let files: Vec<_> = std::fs::read_dir(&backup_dir)
                .unwrap()
                .filter_map(|entry| entry.ok())
                .filter(|entry| {
                    entry
                        .path()
                        .file_name()
                        .and_then(|n| n.to_str())
                        .map(|n| n.starts_with("test-"))
                        .unwrap_or(false)
                })
                .collect();
            assert!(
                !files.is_empty(),
                "Expected at least one file with prefix 'test-' after creating directory"
            );
            for file in &files {
                let metadata = std::fs::metadata(file.path()).unwrap();
                assert!(metadata.len() > 0, "File should have content");
            }
        }
    }

    fn create_test_ingestion_config() -> IngestionConfig {
        IngestionConfig {
            ingestion_config_id: Uuid::new_v4().to_string(),
            asset_id: Uuid::new_v4().to_string(),
            client_key: "test_client_key".to_string(),
        }
    }

    fn create_test_flow_descriptor(
        ingestion_config_id: &str,
        flow_name: &str,
    ) -> FlowDescriptor<String> {
        let mut builder = crate::stream::flow::FlowDescriptorBuilder::new(
            ingestion_config_id.to_string(),
            flow_name.to_string(),
        );
        builder.add("channel1".to_string(), ChannelDataType::Double);
        builder.add("channel2".to_string(), ChannelDataType::Int32);
        builder.build()
    }

    fn create_test_request(
        flow: &str,
        ingestion_config_id: &str,
    ) -> IngestWithConfigDataStreamRequest {
        IngestWithConfigDataStreamRequest {
            ingestion_config_id: ingestion_config_id.to_string(),
            flow: flow.to_string(),
            timestamp: None,
            channel_values: vec![],
            run_id: Uuid::new_v4().to_string(),
            end_stream_on_validation_error: false,
            organization_id: Uuid::new_v4().to_string(),
        }
    }

    #[tokio::test]
    async fn test_file_backup_metrics_streaming_task_completes_when_control_channel_closed() {
        let (setup_channel, _) = create_mock_grpc_channel_with_service().await;
        let temp_dir = TempDir::new("test_file_backup").unwrap();
        let file_writer_config = FileWriterConfig {
            directory: temp_dir.path().to_path_buf(),
            prefix: "test".to_string(),
            max_size: 1024 * 1024,
        };
        let metrics = Arc::new(SiftStreamMetrics::default());

        let file_backup = FileBackup::new(
            file_writer_config,
            10,
            metrics,
            100,
            Some(std::time::Duration::from_secs(60)),
            setup_channel,
            "test_session".to_string(),
            Uuid::new_v4(),
        )
        .unwrap();

        // Decompose the file backup to get the control channel and metrics streaming task.
        let FileBackup {
            metrics_streaming,
            control_tx,
            ..
        } = file_backup;

        // Drop the control channel to signal the metrics streaming task to finish (which would happen if the struct was dropped).
        drop(control_tx);

        // Wait for the metrics streaming task to complete.
        let metrics_streaming_result = metrics_streaming
            .expect("metrics streaming task")
            .await
            .expect("metrics streaming task should complete successfully.");
        assert!(
            metrics_streaming_result.is_ok(),
            "metrics streaming task should have returned success."
        );
    }

    #[tokio::test]
    async fn test_file_backup_mode_send_impl() {
        let ingestion_config = create_test_ingestion_config();
        let temp_dir = TempDir::new("test_file_backup").unwrap();
        let metrics = Arc::new(SiftStreamMetrics::default());
        let sift_stream_id = Uuid::new_v4();

        let file_writer_config = FileWriterConfig {
            directory: temp_dir.path().to_path_buf(),
            prefix: ingestion_config.client_key.clone(),
            max_size: 1024 * 1024, // 1MB
        };

        let mut mode =
            create_test_file_backup_mode(file_writer_config, 1024 * 100, metrics.clone()).await;

        let request = create_test_request("test_flow", &ingestion_config.ingestion_config_id);

        // Send the request
        mode.try_send(&sift_stream_id, request).unwrap();

        // Wait for the background task to process the message
        wait_for_backup_metrics(&metrics, 1, 1000).await;

        // Verify metrics were updated
        assert!(metrics.messages_sent.get() > 0);
        assert_eq!(metrics.backups.total_messages.get(), 1);

        // Finish to ensure all data is written
        mode.finish(&sift_stream_id).await.unwrap();
    }

    #[tokio::test]
    async fn test_file_backup_mode_send_impl_tracks_unique_flows() {
        let ingestion_config = create_test_ingestion_config();
        let temp_dir = TempDir::new("test_file_backup").unwrap();
        let metrics = Arc::new(SiftStreamMetrics::default());
        let sift_stream_id = Uuid::new_v4();

        let file_writer_config = FileWriterConfig {
            directory: temp_dir.path().to_path_buf(),
            prefix: ingestion_config.client_key.clone(),
            max_size: 1024 * 1024,
        };

        let mut mode =
            create_test_file_backup_mode(file_writer_config, 1024 * 100, metrics.clone()).await;

        // Send requests with different flows
        let request1 = create_test_request("flow1", &ingestion_config.ingestion_config_id);
        let request2 = create_test_request("flow2", &ingestion_config.ingestion_config_id);
        let request3 = create_test_request("flow1", &ingestion_config.ingestion_config_id); // Duplicate

        mode.try_send(&sift_stream_id, request1).unwrap();
        mode.try_send(&sift_stream_id, request2).unwrap();
        mode.try_send(&sift_stream_id, request3).unwrap();

        // Wait for the background task to process all messages
        wait_for_backup_metrics(&metrics, 3, 1000).await;

        // Should have tracked 2 unique flows
        assert_eq!(metrics.unique_flows_received.get(), 2);
        assert_eq!(metrics.messages_sent.get(), 3);
        assert_eq!(metrics.backups.total_messages.get(), 3);

        // Finish to ensure all data is written
        mode.finish(&sift_stream_id).await.unwrap();
    }

    #[tokio::test]
    async fn test_file_backup_mode_send_requests() {
        let ingestion_config = create_test_ingestion_config();
        let temp_dir = TempDir::new("test_file_backup").unwrap();
        let metrics = Arc::new(SiftStreamMetrics::default());
        let sift_stream_id = Uuid::new_v4();

        let file_writer_config = FileWriterConfig {
            directory: temp_dir.path().to_path_buf(),
            prefix: ingestion_config.client_key.clone(),
            max_size: 1024 * 1024,
        };

        let mut mode =
            create_test_file_backup_mode(file_writer_config, 1024 * 100, metrics.clone()).await;

        let requests = vec![
            create_test_request("flow1", &ingestion_config.ingestion_config_id),
            create_test_request("flow2", &ingestion_config.ingestion_config_id),
            create_test_request("flow3", &ingestion_config.ingestion_config_id),
        ];

        mode.try_send_requests(&sift_stream_id, requests).unwrap();

        // Wait for the background task to process all messages
        wait_for_backup_metrics(&metrics, 3, 1000).await;

        assert_eq!(metrics.messages_received.get(), 3);
        assert_eq!(metrics.messages_sent.get(), 3);
        assert_eq!(metrics.backups.total_messages.get(), 3);

        // Finish to ensure all data is written
        mode.finish(&sift_stream_id).await.unwrap();
    }

    #[tokio::test]
    async fn test_file_backup_mode_send_with_flow_descriptor() {
        let ingestion_config = create_test_ingestion_config();
        let temp_dir = TempDir::new("test_file_backup").unwrap();
        let metrics = Arc::new(SiftStreamMetrics::default());
        let sift_stream_id = Uuid::new_v4();

        let flow_name = "test_flow";
        let flow_descriptor =
            create_test_flow_descriptor(&ingestion_config.ingestion_config_id, flow_name);
        let mut flows_by_name = HashMap::new();
        flows_by_name.insert(flow_name.to_string(), flow_descriptor);

        let file_writer_config = FileWriterConfig {
            directory: temp_dir.path().to_path_buf(),
            prefix: ingestion_config.client_key.clone(),
            max_size: 1024 * 1024,
        };

        let mut mode =
            create_test_file_backup_mode(file_writer_config, 1024 * 100, metrics.clone()).await;

        let descriptor =
            create_test_flow_descriptor(flow_name, &ingestion_config.ingestion_config_id);
        let mut builder = FlowBuilder::new(&descriptor);
        assert!(builder.set_with_key("channel1", 1.0_f64).is_ok());
        assert!(builder.set_with_key("channel2", 42_i32).is_ok());

        let request = builder.request(TimeValue::now());

        mode.try_send(&sift_stream_id, request).unwrap();

        // Wait for the background task to process the message
        wait_for_backup_metrics(&metrics, 1, 1000).await;

        assert_eq!(metrics.messages_received.get(), 1);
        assert_eq!(metrics.messages_sent.get(), 1);
        assert_eq!(metrics.backups.total_messages.get(), 1);

        // Finish to ensure all data is written
        mode.finish(&sift_stream_id).await.unwrap();
    }

    #[tokio::test]
    async fn test_file_backup_mode_send_without_flow_descriptor() {
        let ingestion_config = create_test_ingestion_config();
        let temp_dir = TempDir::new("test_file_backup").unwrap();
        let metrics = Arc::new(SiftStreamMetrics::default());
        let sift_stream_id = Uuid::new_v4();

        let file_writer_config = FileWriterConfig {
            directory: temp_dir.path().to_path_buf(),
            prefix: ingestion_config.client_key.clone(),
            max_size: 1024 * 1024,
        };

        let mut mode =
            create_test_file_backup_mode(file_writer_config, 1024 * 100, metrics.clone()).await;

        let descriptor =
            create_test_flow_descriptor("unknown_flow", &ingestion_config.ingestion_config_id);
        let mut builder = FlowBuilder::new(&descriptor);
        assert!(builder.set_with_key("channel1", 1.0_f64).is_ok());
        assert!(builder.set_with_key("channel2", 42_i32).is_ok());

        let request = builder.request(TimeValue::now());

        // Should still succeed even without flow descriptor
        mode.try_send(&sift_stream_id, request).unwrap();

        // Wait for the background task to process the message
        wait_for_backup_metrics(&metrics, 1, 1000).await;

        assert_eq!(metrics.messages_received.get(), 1);
        assert_eq!(metrics.messages_sent.get(), 1);
        assert_eq!(metrics.backups.total_messages.get(), 1);

        // Finish to ensure all data is written
        mode.finish(&sift_stream_id).await.unwrap();
    }

    #[tokio::test]
    async fn test_sift_stream_finish() {
        let (grpc_channel, _) = create_mock_grpc_channel_with_service().await;
        let ingestion_config = create_test_ingestion_config();
        let temp_dir = TempDir::new("test_file_backup").unwrap();
        let metrics = Arc::new(SiftStreamMetrics::default());

        let session_name = format!("test_stream.{}", ingestion_config.client_key);
        let sift_stream_id = Uuid::new_v4();
        let stream = SiftStream::new_file_backup(
            grpc_channel,
            ingestion_config,
            HashMap::new(),
            None,
            temp_dir.path().to_path_buf(),
            temp_dir.path().to_path_buf(),
            1024 * 1024,
            1024 * 100, // channel_capacity
            100,        // control_channel_capacity
            None,       // metrics_streaming_interval
            session_name,
            sift_stream_id,
            metrics,
        )
        .await
        .expect("failed to create file backup stream");

        // Finish should succeed
        stream.finish().await.unwrap();
    }

    #[tokio::test]
    async fn test_sift_stream_finish_with_written_data() {
        let (grpc_channel, _) = create_mock_grpc_channel_with_service().await;
        let ingestion_config = create_test_ingestion_config();
        let temp_dir = TempDir::new("test_file_backup").unwrap();
        let metrics = Arc::new(SiftStreamMetrics::default());

        let session_name = format!("test_stream.{}", ingestion_config.client_key);
        let sift_stream_id = Uuid::new_v4();
        let mut stream = SiftStream::new_file_backup(
            grpc_channel,
            ingestion_config.clone(),
            HashMap::new(),
            None,
            temp_dir.path().to_path_buf(),
            temp_dir.path().to_path_buf(),
            1024 * 1024,
            1024 * 100, // channel_capacity
            100,        // control_channel_capacity
            None,       // metrics_streaming_interval
            session_name,
            sift_stream_id,
            metrics,
        )
        .await
        .expect("failed to create file backup stream");

        // Write some data first
        let request = create_test_request("test_flow", &ingestion_config.ingestion_config_id);
        stream.send_requests(vec![request]).await.unwrap();

        // Finish should succeed and flush data
        stream.finish().await.unwrap();
    }

    /// Build a minimal FileBackup backed by a controlled channel.
    /// The spawned write task simply drains messages into a black hole so it
    /// never blocks, but the *caller* also gets the Receiver so they can
    /// withhold reads when they need the channel to appear full or closed.
    fn make_file_backup_with_capacity(
        cap: usize,
    ) -> (
        FileBackup,
        async_channel::Receiver<Arc<IngestWithConfigDataStreamRequest>>,
    ) {
        let (write_tx, write_rx) = async_channel::bounded(cap);
        let (control_tx, _) = tokio::sync::broadcast::channel(10);

        let fb = FileBackup {
            write_tx,
            write_task: tokio::spawn(async { Ok(()) }),
            control_tx,
            metrics_streaming: None,
            flows_seen: HashSet::new(),
            metrics: Arc::new(SiftStreamMetrics::default()),
        };

        (fb, write_rx)
    }

    #[tokio::test]
    async fn test_file_backup_try_send_full_returns_full() {
        let (mut fb, write_rx) = make_file_backup_with_capacity(1);
        let stream_id = Uuid::new_v4();
        let ingestion_config_id = Uuid::new_v4().to_string();

        // First send fills the channel.
        fb.try_send(
            &stream_id,
            create_test_request("flow1", &ingestion_config_id),
        )
        .unwrap();

        // Second send should fail with Full because the channel is now at capacity.
        let req = create_test_request("flow2", &ingestion_config_id);
        let flow = req.flow.clone();
        let err = fb.try_send(&stream_id, req).unwrap_err();
        assert!(err.is_full(), "expected Full, got {err}");
        assert_eq!(err.into_inner().flow, flow);

        drop(write_rx);
    }

    #[tokio::test]
    async fn test_file_backup_try_send_closed_returns_closed() {
        let (mut fb, write_rx) = make_file_backup_with_capacity(10);
        let stream_id = Uuid::new_v4();
        let ingestion_config_id = Uuid::new_v4().to_string();

        // Closing the receiver makes every subsequent try_send return Closed.
        drop(write_rx);

        let req = create_test_request("flow1", &ingestion_config_id);
        let flow = req.flow.clone();
        let err = fb.try_send(&stream_id, req).unwrap_err();
        assert!(err.is_closed(), "expected Closed, got {err}");
        assert_eq!(err.into_inner().flow, flow);
    }

    #[tokio::test]
    async fn test_file_backup_send_closed_returns_send_error() {
        let (mut fb, write_rx) = make_file_backup_with_capacity(10);
        let stream_id = Uuid::new_v4();
        let ingestion_config_id = Uuid::new_v4().to_string();

        drop(write_rx);

        let req = create_test_request("flow1", &ingestion_config_id);
        let flow = req.flow.clone();
        let err = fb.send(&stream_id, req).await.unwrap_err();
        assert_eq!(err.into_inner().flow, flow);
    }

    #[tokio::test]
    async fn test_file_backup_send_blocks_until_space_available() {
        let (mut fb, write_rx) = make_file_backup_with_capacity(1);
        let stream_id = Uuid::new_v4();
        let ingestion_config_id = Uuid::new_v4().to_string();

        // Fill the channel so the next async send has to wait.
        fb.try_send(
            &stream_id,
            create_test_request("flow1", &ingestion_config_id),
        )
        .unwrap();

        // Consumer reads after a short delay, freeing space for the blocked send.
        tokio::spawn(async move {
            tokio::time::sleep(std::time::Duration::from_millis(20)).await;
            let _ = write_rx.recv().await;
            // Keep the receiver alive so the channel doesn't close.
            tokio::time::sleep(std::time::Duration::from_millis(100)).await;
        });

        fb.send(
            &stream_id,
            create_test_request("flow2", &ingestion_config_id),
        )
        .await
        .unwrap();
    }

    #[tokio::test]
    async fn test_file_backup_try_send_requests_returns_undelivered_on_full() {
        let (mut fb, write_rx) = make_file_backup_with_capacity(1);
        let stream_id = Uuid::new_v4();
        let ingestion_config_id = Uuid::new_v4().to_string();

        // Fill the channel.
        fb.try_send(
            &stream_id,
            create_test_request("flow0", &ingestion_config_id),
        )
        .unwrap();

        let reqs = vec![
            create_test_request("flow1", &ingestion_config_id),
            create_test_request("flow2", &ingestion_config_id),
            create_test_request("flow3", &ingestion_config_id),
        ];
        let err = fb.try_send_requests(&stream_id, reqs).unwrap_err();
        assert!(err.is_full(), "expected Full, got {err}");
        assert_eq!(err.into_inner().len(), 3);

        drop(write_rx);
    }

    #[tokio::test]
    async fn test_file_backup_send_requests_returns_undelivered_on_closed() {
        let (mut fb, write_rx) = make_file_backup_with_capacity(10);
        let stream_id = Uuid::new_v4();
        let ingestion_config_id = Uuid::new_v4().to_string();

        drop(write_rx);

        let reqs = vec![
            create_test_request("flow1", &ingestion_config_id),
            create_test_request("flow2", &ingestion_config_id),
            create_test_request("flow3", &ingestion_config_id),
        ];
        let err = fb.send_requests(&stream_id, reqs).await.unwrap_err();
        assert_eq!(err.into_inner().len(), 3);
    }
}
