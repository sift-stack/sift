use super::super::{SendContext, SiftStream, SiftStreamMode, private::Sealed};
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
use std::{collections::HashMap, path::PathBuf, sync::Arc};
use tokio::task::JoinHandle;
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
            self.handle_request(&request).await?;
        }

        // Flush and sync the current file before finishing
        self.finalize().await?;

        Ok(())
    }
}

/// Dependencies specifically for file-backup based streaming. Users shouldn't have to
/// interact with this directly.
pub struct FileBackupMode {
    ingestion_config: IngestionConfig,
    write_tx: Sender<Arc<IngestWithConfigDataStreamRequest>>,
    write_task: JoinHandle<Result<()>>,
}

// Seal the trait - only this crate can implement SiftStreamMode
impl Sealed for FileBackupMode {}

#[async_trait]
impl SiftStreamMode for FileBackupMode {
    fn ingestion_config_id(&self) -> &str {
        &self.ingestion_config.ingestion_config_id
    }

    async fn send(&mut self, ctx: &mut SendContext<'_>, message: Flow) -> Result<()> {
        ctx.metrics.messages_received.increment();

        let run_id = ctx.run.as_ref().map(|r| r.run_id.clone());

        let req = if let Some(flows) = ctx.flows_by_name.get(&message.flow_name) {
            if let Some(req) = super::super::helpers::message_to_ingest_req(
                &message,
                ctx.run.as_ref().map(|r| r.run_id.clone()),
                flows,
            ) {
                req
            } else {
                #[cfg(feature = "tracing")]
                tracing::warn!(
                    sift_stream_id = ctx.sift_stream_id.to_string(),
                    values = format!("{message:?}"),
                    "encountered a message that doesn't match any cached flows - message will still be written to file"
                );
                super::super::helpers::message_to_ingest_req_direct(
                    &message,
                    &self.ingestion_config.ingestion_config_id,
                    run_id,
                )
            }
        } else {
            #[cfg(feature = "tracing")]
            tracing::warn!(
                sift_stream_id = ctx.sift_stream_id.to_string(),
                "flow '{}' not found in local flow cache - message will still be written to file",
                message.flow_name,
            );
            super::super::helpers::message_to_ingest_req_direct(
                &message,
                &self.ingestion_config.ingestion_config_id,
                run_id,
            )
        };

        self.send_impl(ctx, req)
    }

    async fn send_requests<I>(&mut self, ctx: &mut SendContext<'_>, requests: I) -> Result<()>
    where
        I: IntoIterator<Item = IngestWithConfigDataStreamRequest> + Send,
        I::IntoIter: Send,
    {
        for req in requests {
            ctx.metrics.messages_received.increment();
            self.send_impl(ctx, req)?;
        }
        Ok(())
    }

    fn send_requests_nonblocking<I>(&mut self, ctx: &mut SendContext<'_>, requests: I) -> Result<()>
    where
        I: IntoIterator<Item = IngestWithConfigDataStreamRequest> + Send,
        I::IntoIter: Send,
    {
        for req in requests {
            ctx.metrics.messages_received.increment();
            self.send_impl(ctx, req)?;
        }
        Ok(())
    }

    /// This will conclude the stream and flush any remaining data to disk.
    async fn finish(self, ctx: &mut SendContext<'_>) -> Result<()> {
        // Close the channel to signal the background task to finish
        drop(self.write_tx);

        // Wait for the background task to complete
        self.write_task.await.map_err(|e| {
            Error::new_msg(
                ErrorKind::StreamError,
                format!("file backup write task panicked: {e}"),
            )
        })??;

        #[cfg(feature = "tracing")]
        tracing::info!(
            sift_stream_id = ctx.sift_stream_id.to_string(),
            asset_id = self.ingestion_config.asset_id,
            ingestion_config_id = self.ingestion_config.ingestion_config_id,
            run = ctx.run.as_ref().map(|r| r.name.clone()).unwrap_or_default(),
            "successfully finished file backup stream"
        );

        Ok(())
    }
}

impl FileBackupMode {
    /// Creates a new `FileBackupMode` and spawns the background file-writing task.
    pub(crate) fn new(
        file_writer_config: FileWriterConfig,
        channel_capacity: usize,
        ingestion_config: IngestionConfig,
        metrics: Arc<SiftStreamMetrics>,
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

        Ok(Self {
            ingestion_config,
            write_tx,
            write_task,
        })
    }

    /// Sends the request to the background write task.
    pub(crate) fn send_impl(
        &mut self,
        ctx: &mut SendContext<'_>,
        request: IngestWithConfigDataStreamRequest,
    ) -> Result<()> {
        #[cfg(feature = "tracing")]
        {
            if !ctx.flows_seen.contains(&request.flow) {
                ctx.metrics.unique_flows_received.increment();
                ctx.flows_seen.insert(request.flow.clone());
                tracing::info!(
                    sift_stream_id = ctx.sift_stream_id.to_string(),
                    "flow '{}' being written for the first time",
                    &request.flow,
                );
            }
        }

        // Track the backup channel depth.
        ctx.metrics
            .backup_channel_depth
            .set(self.write_tx.len() as u64);

        // Send the request to the background write task (non-blocking)
        let request_arc = Arc::new(request);
        self.write_tx.try_send(request_arc).map_err(|e| {
            if e.is_full() {
                Error::new_msg(ErrorKind::StreamError, "file backup write channel is full")
            } else {
                Error::new_msg(
                    ErrorKind::StreamError,
                    format!("file backup write channel is closed: {e}"),
                )
            }
        })?;

        ctx.metrics.messages_sent.increment();
        Ok(())
    }
}

impl SiftStream<FileBackupMode> {
    /// Initializes a new [SiftStream] for file backup mode. Users should instead use [`SiftStreamBuilder`].
    ///
    /// [`SiftStreamBuilder`]: crate::stream::builder::SiftStreamBuilder
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new_file_backup(
        grpc_channel: SiftChannel,
        ingestion_config: IngestionConfig,
        flows_by_name: HashMap<String, FlowDescriptor<String>>,
        run: Option<Run>,
        output_directory: PathBuf,
        max_file_size: usize,
        channel_capacity: usize,
        metrics: Arc<SiftStreamMetrics>,
    ) -> Result<Self> {
        let sift_stream_id = Uuid::new_v4();

        let file_writer_config = FileWriterConfig {
            directory: output_directory.clone(),
            prefix: ingestion_config.client_key.clone(),
            max_size: max_file_size,
        };

        let mode = FileBackupMode::new(
            file_writer_config,
            channel_capacity,
            ingestion_config,
            metrics.clone(),
        )?;

        Ok(Self {
            grpc_channel,
            mode,
            metrics,
            flows_by_name,
            run,
            flows_seen: std::collections::HashSet::new(),
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
    use crate::{ChannelValue, TimeValue};
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

    fn create_test_flow(flow_name: &str) -> Flow {
        Flow::new(
            flow_name,
            TimeValue::now(),
            &[
                ChannelValue::new("channel1", 1.0_f64),
                ChannelValue::new("channel2", 42_i32),
            ],
        )
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
    async fn test_file_backup_mode_ingestion_config_id() {
        let ingestion_config = create_test_ingestion_config();
        let temp_dir = TempDir::new("test_file_backup").unwrap();
        let file_writer_config = FileWriterConfig {
            directory: temp_dir.path().to_path_buf(),
            prefix: ingestion_config.client_key.clone(),
            max_size: 1024,
        };

        let metrics = Arc::new(SiftStreamMetrics::default());
        let mode = FileBackupMode::new(
            file_writer_config,
            1024 * 100,
            ingestion_config.clone(),
            metrics,
        )
        .unwrap();

        assert_eq!(
            mode.ingestion_config_id(),
            &ingestion_config.ingestion_config_id
        );
    }

    #[tokio::test]
    async fn test_file_backup_mode_send_impl() {
        let ingestion_config = create_test_ingestion_config();
        let temp_dir = TempDir::new("test_file_backup").unwrap();
        let metrics = Arc::new(SiftStreamMetrics::default());
        let mut flows_seen = std::collections::HashSet::new();
        let sift_stream_id = Uuid::new_v4();

        let file_writer_config = FileWriterConfig {
            directory: temp_dir.path().to_path_buf(),
            prefix: ingestion_config.client_key.clone(),
            max_size: 1024 * 1024, // 1MB
        };

        let mut mode = FileBackupMode::new(
            file_writer_config,
            1024 * 100,
            ingestion_config.clone(),
            metrics.clone(),
        )
        .unwrap();

        let request = create_test_request("test_flow", &ingestion_config.ingestion_config_id);

        let mut ctx = SendContext {
            metrics: &metrics,
            run: &None,
            flows_by_name: &HashMap::new(),
            flows_seen: &mut flows_seen,
            sift_stream_id: &sift_stream_id,
        };

        // Send the request
        mode.send_impl(&mut ctx, request).unwrap();

        // Wait for the background task to process the message
        wait_for_backup_metrics(&metrics, 1, 1000).await;

        // Verify metrics were updated
        assert!(metrics.messages_sent.get() > 0);
        assert_eq!(metrics.backups.total_messages.get(), 1);

        // Finish to ensure all data is written
        let mut ctx = SendContext {
            metrics: &metrics,
            run: &None,
            flows_by_name: &HashMap::new(),
            flows_seen: &mut flows_seen,
            sift_stream_id: &sift_stream_id,
        };
        mode.finish(&mut ctx).await.unwrap();
    }

    #[tokio::test]
    async fn test_file_backup_mode_send_impl_tracks_unique_flows() {
        let ingestion_config = create_test_ingestion_config();
        let temp_dir = TempDir::new("test_file_backup").unwrap();
        let metrics = Arc::new(SiftStreamMetrics::default());
        let mut flows_seen = std::collections::HashSet::new();
        let sift_stream_id = Uuid::new_v4();

        let file_writer_config = FileWriterConfig {
            directory: temp_dir.path().to_path_buf(),
            prefix: ingestion_config.client_key.clone(),
            max_size: 1024 * 1024,
        };

        let mut mode = FileBackupMode::new(
            file_writer_config,
            1024 * 100,
            ingestion_config.clone(),
            metrics.clone(),
        )
        .unwrap();

        // Send requests with different flows
        let request1 = create_test_request("flow1", &ingestion_config.ingestion_config_id);
        let request2 = create_test_request("flow2", &ingestion_config.ingestion_config_id);
        let request3 = create_test_request("flow1", &ingestion_config.ingestion_config_id); // Duplicate

        let mut ctx = SendContext {
            metrics: &metrics,
            run: &None,
            flows_by_name: &HashMap::new(),
            flows_seen: &mut flows_seen,
            sift_stream_id: &sift_stream_id,
        };

        mode.send_impl(&mut ctx, request1).unwrap();
        mode.send_impl(&mut ctx, request2).unwrap();
        mode.send_impl(&mut ctx, request3).unwrap();

        // Wait for the background task to process all messages
        wait_for_backup_metrics(&metrics, 3, 1000).await;

        // Should have tracked 2 unique flows
        assert_eq!(metrics.unique_flows_received.get(), 2);
        assert_eq!(metrics.messages_sent.get(), 3);
        assert_eq!(metrics.backups.total_messages.get(), 3);

        // Finish to ensure all data is written
        let mut ctx = SendContext {
            metrics: &metrics,
            run: &None,
            flows_by_name: &HashMap::new(),
            flows_seen: &mut flows_seen,
            sift_stream_id: &sift_stream_id,
        };
        mode.finish(&mut ctx).await.unwrap();
    }

    #[tokio::test]
    async fn test_file_backup_mode_send_requests() {
        let ingestion_config = create_test_ingestion_config();
        let temp_dir = TempDir::new("test_file_backup").unwrap();
        let metrics = Arc::new(SiftStreamMetrics::default());
        let mut flows_seen = std::collections::HashSet::new();
        let sift_stream_id = Uuid::new_v4();

        let file_writer_config = FileWriterConfig {
            directory: temp_dir.path().to_path_buf(),
            prefix: ingestion_config.client_key.clone(),
            max_size: 1024 * 1024,
        };

        let mut mode = FileBackupMode::new(
            file_writer_config,
            1024 * 100,
            ingestion_config.clone(),
            metrics.clone(),
        )
        .unwrap();

        let requests = vec![
            create_test_request("flow1", &ingestion_config.ingestion_config_id),
            create_test_request("flow2", &ingestion_config.ingestion_config_id),
            create_test_request("flow3", &ingestion_config.ingestion_config_id),
        ];

        let mut ctx = SendContext {
            metrics: &metrics,
            run: &None,
            flows_by_name: &HashMap::new(),
            flows_seen: &mut flows_seen,
            sift_stream_id: &sift_stream_id,
        };

        mode.send_requests(&mut ctx, requests).await.unwrap();

        // Wait for the background task to process all messages
        wait_for_backup_metrics(&metrics, 3, 1000).await;

        assert_eq!(metrics.messages_received.get(), 3);
        assert_eq!(metrics.messages_sent.get(), 3);
        assert_eq!(metrics.backups.total_messages.get(), 3);

        // Finish to ensure all data is written
        let mut ctx = SendContext {
            metrics: &metrics,
            run: &None,
            flows_by_name: &HashMap::new(),
            flows_seen: &mut flows_seen,
            sift_stream_id: &sift_stream_id,
        };
        mode.finish(&mut ctx).await.unwrap();
    }

    #[tokio::test]
    async fn test_file_backup_mode_send_requests_nonblocking() {
        let ingestion_config = create_test_ingestion_config();
        let temp_dir = TempDir::new("test_file_backup").unwrap();
        let metrics = Arc::new(SiftStreamMetrics::default());
        let mut flows_seen = std::collections::HashSet::new();
        let sift_stream_id = Uuid::new_v4();

        let file_writer_config = FileWriterConfig {
            directory: temp_dir.path().to_path_buf(),
            prefix: ingestion_config.client_key.clone(),
            max_size: 1024 * 1024,
        };

        let mut mode = FileBackupMode::new(
            file_writer_config,
            1024 * 100,
            ingestion_config.clone(),
            metrics.clone(),
        )
        .unwrap();

        let requests = vec![
            create_test_request("flow1", &ingestion_config.ingestion_config_id),
            create_test_request("flow2", &ingestion_config.ingestion_config_id),
        ];

        let mut ctx = SendContext {
            metrics: &metrics,
            run: &None,
            flows_by_name: &HashMap::new(),
            flows_seen: &mut flows_seen,
            sift_stream_id: &sift_stream_id,
        };

        mode.send_requests_nonblocking(&mut ctx, requests).unwrap();

        // Wait for the background task to process all messages
        wait_for_backup_metrics(&metrics, 2, 1000).await;

        assert_eq!(metrics.messages_received.get(), 2);
        assert_eq!(metrics.messages_sent.get(), 2);
        assert_eq!(metrics.backups.total_messages.get(), 2);

        // Finish to ensure all data is written
        let mut ctx = SendContext {
            metrics: &metrics,
            run: &None,
            flows_by_name: &HashMap::new(),
            flows_seen: &mut flows_seen,
            sift_stream_id: &sift_stream_id,
        };
        mode.finish(&mut ctx).await.unwrap();
    }

    #[tokio::test]
    async fn test_file_backup_mode_send_with_flow_descriptor() {
        let ingestion_config = create_test_ingestion_config();
        let temp_dir = TempDir::new("test_file_backup").unwrap();
        let metrics = Arc::new(SiftStreamMetrics::default());
        let mut flows_seen = std::collections::HashSet::new();
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

        let mut mode = FileBackupMode::new(
            file_writer_config,
            1024 * 100,
            ingestion_config,
            metrics.clone(),
        )
        .unwrap();

        let flow = create_test_flow(flow_name);

        let mut ctx = SendContext {
            metrics: &metrics,
            run: &None,
            flows_by_name: &flows_by_name,
            flows_seen: &mut flows_seen,
            sift_stream_id: &sift_stream_id,
        };

        mode.send(&mut ctx, flow).await.unwrap();

        // Wait for the background task to process the message
        wait_for_backup_metrics(&metrics, 1, 1000).await;

        assert_eq!(metrics.messages_received.get(), 1);
        assert_eq!(metrics.messages_sent.get(), 1);
        assert_eq!(metrics.backups.total_messages.get(), 1);

        // Finish to ensure all data is written
        let mut ctx = SendContext {
            metrics: &metrics,
            run: &None,
            flows_by_name: &flows_by_name,
            flows_seen: &mut flows_seen,
            sift_stream_id: &sift_stream_id,
        };
        mode.finish(&mut ctx).await.unwrap();
    }

    #[tokio::test]
    async fn test_file_backup_mode_send_without_flow_descriptor() {
        let ingestion_config = create_test_ingestion_config();
        let temp_dir = TempDir::new("test_file_backup").unwrap();
        let metrics = Arc::new(SiftStreamMetrics::default());
        let mut flows_seen = std::collections::HashSet::new();
        let sift_stream_id = Uuid::new_v4();

        let file_writer_config = FileWriterConfig {
            directory: temp_dir.path().to_path_buf(),
            prefix: ingestion_config.client_key.clone(),
            max_size: 1024 * 1024,
        };

        let mut mode = FileBackupMode::new(
            file_writer_config,
            1024 * 100,
            ingestion_config,
            metrics.clone(),
        )
        .unwrap();

        let flow = create_test_flow("unknown_flow");

        let mut ctx = SendContext {
            metrics: &metrics,
            run: &None,
            flows_by_name: &HashMap::new(), // Empty flows map
            flows_seen: &mut flows_seen,
            sift_stream_id: &sift_stream_id,
        };

        // Should still succeed even without flow descriptor
        mode.send(&mut ctx, flow).await.unwrap();

        // Wait for the background task to process the message
        wait_for_backup_metrics(&metrics, 1, 1000).await;

        assert_eq!(metrics.messages_received.get(), 1);
        assert_eq!(metrics.messages_sent.get(), 1);
        assert_eq!(metrics.backups.total_messages.get(), 1);

        // Finish to ensure all data is written
        let mut ctx = SendContext {
            metrics: &metrics,
            run: &None,
            flows_by_name: &HashMap::new(),
            flows_seen: &mut flows_seen,
            sift_stream_id: &sift_stream_id,
        };
        mode.finish(&mut ctx).await.unwrap();
    }

    #[tokio::test]
    async fn test_sift_stream_new_file_backup() {
        let (grpc_channel, _) = create_mock_grpc_channel_with_service().await;
        let ingestion_config = create_test_ingestion_config();
        let temp_dir = TempDir::new("test_file_backup").unwrap();
        let metrics = Arc::new(SiftStreamMetrics::default());

        let stream = SiftStream::new_file_backup(
            grpc_channel,
            ingestion_config.clone(),
            HashMap::new(),
            None,
            temp_dir.path().to_path_buf(),
            1024 * 1024,
            1024 * 100, // channel_capacity
            metrics,
        )
        .unwrap();

        assert_eq!(
            stream.mode.ingestion_config_id(),
            &ingestion_config.ingestion_config_id
        );
    }

    #[tokio::test]
    async fn test_sift_stream_finish() {
        let (grpc_channel, _) = create_mock_grpc_channel_with_service().await;
        let ingestion_config = create_test_ingestion_config();
        let temp_dir = TempDir::new("test_file_backup").unwrap();
        let metrics = Arc::new(SiftStreamMetrics::default());

        let stream = SiftStream::new_file_backup(
            grpc_channel,
            ingestion_config,
            HashMap::new(),
            None,
            temp_dir.path().to_path_buf(),
            1024 * 1024,
            1024 * 100, // channel_capacity
            metrics,
        )
        .unwrap();

        // Finish should succeed
        stream.finish().await.unwrap();
    }

    #[tokio::test]
    async fn test_sift_stream_finish_with_written_data() {
        let (grpc_channel, _) = create_mock_grpc_channel_with_service().await;
        let ingestion_config = create_test_ingestion_config();
        let temp_dir = TempDir::new("test_file_backup").unwrap();
        let metrics = Arc::new(SiftStreamMetrics::default());

        let mut stream = SiftStream::new_file_backup(
            grpc_channel,
            ingestion_config.clone(),
            HashMap::new(),
            None,
            temp_dir.path().to_path_buf(),
            1024 * 1024,
            1024 * 100, // channel_capacity
            metrics,
        )
        .unwrap();

        // Write some data first
        let request = create_test_request("test_flow", &ingestion_config.ingestion_config_id);
        stream.send_requests(vec![request]).await.unwrap();

        // Finish should succeed and flush data
        stream.finish().await.unwrap();
    }
}
