use chrono::Utc;
use sift_error::prelude::*;
use sift_pbfs::chunk::PbfsChunk;
use sift_rs::ingest::v1::IngestWithConfigDataStreamRequest;
use std::path::PathBuf;
use tokio::fs::File;
use tokio::io::{AsyncWriteExt, BufWriter};

const BACKUP_FILE_BUFFER_WRITE_SIZE: usize = 128 * 1024;

/// Configuration for file writing operations.
#[derive(Clone)]
pub(crate) struct FileWriterConfig {
    /// The directory to store files in.
    pub(crate) directory: PathBuf,

    /// The prefix to add to all files.
    pub(crate) prefix: String,

    /// The maximum size of a file in bytes.
    pub(crate) max_size: usize,
}

/// Context for a file and the messages it contains.
#[derive(Debug, Default, Clone)]
pub(crate) struct FileContext {
    /// The number of messages in the file.
    pub(crate) message_count: usize,

    /// The path to the file (used for test assertions).
    #[allow(dead_code)]
    pub(crate) file_path: PathBuf,

    /// The estimated size of the file in bytes (based on the messages in the file).
    pub(crate) file_size: usize,
}

/// Manages writing IngestWithConfigDataStreamRequest messages to files with rotation support.
pub(crate) struct FileWriter {
    /// Configuration for file writing.
    pub(crate) config: FileWriterConfig,

    /// The current file context being written to.
    pub(crate) current_file_ctx: FileContext,

    /// The current file being written to.
    pub(crate) current_file: Option<BufWriter<File>>,

    /// Reusable buffer for encoding PbfsChunk data to avoid per-message allocations.
    pub(crate) chunk_encode_buffer: Vec<u8>,
}

impl FileWriter {
    /// Create a new FileWriter with the given configuration.
    pub(crate) fn new(config: FileWriterConfig) -> Self {
        Self {
            config,
            current_file_ctx: FileContext::default(),
            current_file: None,
            chunk_encode_buffer: Vec::new(),
        }
    }

    /// Write a request to the current file, creating a new file if needed.
    pub(crate) async fn write_request(
        &mut self,
        request: &IngestWithConfigDataStreamRequest,
    ) -> Result<()> {
        // Create a new file if one doesn't exist.
        if self.current_file.is_none() {
            let (path, file) = Self::create_file(&self.config).await?;
            self.current_file_ctx = FileContext {
                message_count: 0,
                file_path: path.clone(),
                file_size: 0,
            };
            self.current_file = Some(BufWriter::with_capacity(
                BACKUP_FILE_BUFFER_WRITE_SIZE,
                file,
            ));
        }

        let Some(file) = self.current_file.as_mut() else {
            return Err(Error::new_msg(
                ErrorKind::BackupsError,
                "current file is not set",
            ));
        };

        // Encode message into reusable buffer to avoid per-message allocations
        let encoded_chunk = PbfsChunk::encode_into(
            core::slice::from_ref(request),
            &mut self.chunk_encode_buffer,
        )?;

        // Write immediately
        file.write_all(encoded_chunk).await?;

        self.current_file_ctx.message_count += 1;
        self.current_file_ctx.file_size += encoded_chunk.len();

        Ok(())
    }

    /// Checks if the current file has reached the max size and should be rotated.
    pub(crate) fn should_rotate_file(&self) -> bool {
        self.current_file_ctx.file_size >= self.config.max_size
    }

    /// Rotates the current file by closing it and returning the file context.
    /// Returns None if there's no current file.
    pub(crate) async fn rotate_file(&mut self) -> Result<Option<FileContext>> {
        // Close out the current file by dropping it, if there is no file, there is nothing to do.
        match self.current_file.take() {
            Some(mut writer) => {
                // Flush the buffer to the file.
                writer
                    .flush()
                    .await
                    .map_err(|e| Error::new(ErrorKind::BackupsError, e))?;
                let file = writer.into_inner();

                // Ensure data is persisted to disk and not simply buffered in the kernel.
                if let Err(e) = file.sync_all().await {
                    #[cfg(feature = "tracing")]
                    tracing::warn!("unable to sync file, data may be lost: {e:?}");
                }

                // Return the file context
                let ctx = self.current_file_ctx.clone();
                self.current_file_ctx = FileContext::default();
                Ok(Some(ctx))
            }
            None => Ok(None),
        }
    }

    /// Flush and sync the current file if it exists.
    pub(crate) async fn flush(&mut self) -> Result<()> {
        if let Some(writer) = self.current_file.as_mut() {
            writer
                .flush()
                .await
                .map_err(|e| Error::new(ErrorKind::BackupsError, e))?;
        }
        Ok(())
    }

    /// Sync the current file to disk if it exists.
    pub(crate) async fn sync(&mut self) -> Result<()> {
        if let Some(writer) = self.current_file.take() {
            let file = writer.into_inner();
            file.sync_all()
                .await
                .map_err(|e| Error::new(ErrorKind::BackupsError, e))?;
            self.current_file = Some(BufWriter::with_capacity(
                BACKUP_FILE_BUFFER_WRITE_SIZE,
                file,
            ));
        }
        Ok(())
    }

    /// Create a new file with a timestamp-based name.
    async fn create_file(config: &FileWriterConfig) -> Result<(PathBuf, File)> {
        let file_path = config.directory.join(format!(
            "{}-{}",
            config.prefix,
            Utc::now().timestamp_micros()
        ));
        let file = File::create(&file_path)
            .await
            .map_err(|e| Error::new(ErrorKind::BackupsError, e))
            .context("failed to generate file")
            .help("please contact Sift")?;

        Ok((file_path, file))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sift_rs::ingest::v1::IngestWithConfigDataStreamRequest;
    use tempdir::TempDir;
    use uuid::Uuid;

    fn create_test_request(flow: &str) -> IngestWithConfigDataStreamRequest {
        IngestWithConfigDataStreamRequest {
            ingestion_config_id: Uuid::new_v4().to_string(),
            flow: flow.to_string(),
            timestamp: None,
            channel_values: vec![],
            run_id: Uuid::new_v4().to_string(),
            end_stream_on_validation_error: false,
            organization_id: Uuid::new_v4().to_string(),
        }
    }

    #[tokio::test]
    async fn test_file_writer_new() {
        let temp_dir = TempDir::new("test_file_writer").unwrap();
        let config = FileWriterConfig {
            directory: temp_dir.path().to_path_buf(),
            prefix: "test".to_string(),
            max_size: 1024,
        };

        let writer = FileWriter::new(config);
        assert_eq!(writer.current_file_ctx.message_count, 0);
        assert_eq!(writer.current_file_ctx.file_size, 0);
        assert!(!writer.should_rotate_file());
    }

    #[tokio::test]
    async fn test_file_writer_write_request_creates_file() {
        let temp_dir = TempDir::new("test_file_writer").unwrap();
        let config = FileWriterConfig {
            directory: temp_dir.path().to_path_buf(),
            prefix: "test".to_string(),
            max_size: 1024 * 1024, // 1MB
        };

        let mut writer = FileWriter::new(config);
        let request = create_test_request("test_flow");

        // Initially no file should exist
        assert!(writer.current_file.is_none());

        // Write a request - should create a file
        assert!(writer.write_request(&request).await.is_ok());

        // File should now exist
        assert!(writer.current_file.is_some());
        assert_eq!(writer.current_file_ctx.message_count, 1);
        assert!(writer.current_file_ctx.file_size > 0);
        assert!(writer.current_file_ctx.file_path.exists());
    }

    #[tokio::test]
    async fn test_file_writer_write_multiple_requests() {
        let temp_dir = TempDir::new("test_file_writer").unwrap();
        let config = FileWriterConfig {
            directory: temp_dir.path().to_path_buf(),
            prefix: "test".to_string(),
            max_size: 1024 * 1024, // 1MB
        };

        let mut writer = FileWriter::new(config);

        // Write multiple requests
        for i in 0..5 {
            let request = create_test_request(&format!("flow_{}", i));
            assert!(writer.write_request(&request).await.is_ok());
        }

        assert_eq!(writer.current_file_ctx.message_count, 5);
        assert!(writer.current_file_ctx.file_size > 0);
    }

    #[tokio::test]
    async fn test_file_writer_should_rotate_file() {
        let temp_dir = TempDir::new("test_file_writer").unwrap();
        let config = FileWriterConfig {
            directory: temp_dir.path().to_path_buf(),
            prefix: "test".to_string(),
            max_size: 100, // Very small max size
        };

        let mut writer = FileWriter::new(config);
        let request = create_test_request("test_flow");

        // Write a request
        assert!(writer.write_request(&request).await.is_ok());

        // Should not rotate yet (unless the encoded chunk is >= 100 bytes)
        let should_rotate = writer.should_rotate_file();

        // If the encoded chunk is small, write more until we exceed max_size
        if !should_rotate {
            // Write many more requests to exceed max_size
            for _ in 0..100 {
                assert!(writer.write_request(&request).await.is_ok());
                if writer.should_rotate_file() {
                    break;
                }
            }
        }

        // At some point we should need to rotate
        assert!(writer.should_rotate_file());
    }

    #[tokio::test]
    async fn test_file_writer_rotate_file() {
        let temp_dir = TempDir::new("test_file_writer").unwrap();
        let config = FileWriterConfig {
            directory: temp_dir.path().to_path_buf(),
            prefix: "test".to_string(),
            max_size: 100, // Very small max size
        };

        let max_size = config.max_size;
        let mut writer = FileWriter::new(config);
        let request = create_test_request("test_flow");

        // Write requests until we need to rotate
        for _ in 0..100 {
            assert!(writer.write_request(&request).await.is_ok());
            if writer.should_rotate_file() {
                break;
            }
        }

        // Rotate the file
        let rotated_ctx = writer.rotate_file().await.expect("failed to rotate file");

        // Should return a context if a file was rotated
        if writer.current_file_ctx.file_size >= max_size {
            assert!(rotated_ctx.is_some());
            let ctx = rotated_ctx.unwrap();
            assert!(ctx.message_count > 0);
            assert!(ctx.file_size > 0);
            assert!(ctx.file_path.exists());

            // After rotation, current file context should be reset
            assert_eq!(writer.current_file_ctx.message_count, 0);
            assert_eq!(writer.current_file_ctx.file_size, 0);
        }
    }

    #[tokio::test]
    async fn test_file_writer_rotate_file_no_file() {
        let temp_dir = TempDir::new("test_file_writer").unwrap();
        let config = FileWriterConfig {
            directory: temp_dir.path().to_path_buf(),
            prefix: "test".to_string(),
            max_size: 1024,
        };

        let mut writer = FileWriter::new(config);

        // Try to rotate when no file exists
        let result = writer.rotate_file().await.expect("failed to rotate file");
        assert!(result.is_none());
    }

    #[tokio::test]
    async fn test_file_writer_flush() {
        let temp_dir = TempDir::new("test_file_writer").unwrap();
        let config = FileWriterConfig {
            directory: temp_dir.path().to_path_buf(),
            prefix: "test".to_string(),
            max_size: 1024,
        };

        let mut writer = FileWriter::new(config);
        let request = create_test_request("test_flow");

        // Flush when no file exists should succeed
        assert!(writer.flush().await.is_ok());

        // Write a request
        assert!(writer.write_request(&request).await.is_ok());

        // Flush should succeed
        assert!(writer.flush().await.is_ok());
    }

    #[tokio::test]
    async fn test_file_writer_sync() {
        let temp_dir = TempDir::new("test_file_writer").unwrap();
        let config = FileWriterConfig {
            directory: temp_dir.path().to_path_buf(),
            prefix: "test".to_string(),
            max_size: 1024,
        };

        let mut writer = FileWriter::new(config);

        // Sync when no file exists should succeed
        assert!(writer.sync().await.is_ok());

        let request = create_test_request("test_flow");

        // Write a request
        assert!(writer.write_request(&request).await.is_ok());

        // Sync should succeed
        assert!(writer.sync().await.is_ok());

        // File should still exist after sync
        assert!(writer.current_file.is_some());
    }

    #[tokio::test]
    async fn test_file_writer_current_file_context() {
        let temp_dir = TempDir::new("test_file_writer").unwrap();
        let config = FileWriterConfig {
            directory: temp_dir.path().to_path_buf(),
            prefix: "test".to_string(),
            max_size: 1024,
        };

        let writer = FileWriter::new(config);
        assert_eq!(writer.current_file_ctx.message_count, 0);
        assert_eq!(writer.current_file_ctx.file_size, 0);
    }

    #[tokio::test]
    async fn test_file_writer_file_naming() {
        let temp_dir = TempDir::new("test_file_writer").unwrap();
        let config = FileWriterConfig {
            directory: temp_dir.path().to_path_buf(),
            prefix: "backup".to_string(),
            max_size: 1024,
        };

        let mut writer = FileWriter::new(config);
        let request = create_test_request("test_flow");

        assert!(writer.write_request(&request).await.is_ok());

        let file_path = writer.current_file_ctx.file_path.clone();
        let file_name = file_path.file_name().unwrap().to_string_lossy();

        // File name should start with prefix
        assert!(file_name.starts_with("backup-"));
    }

    #[tokio::test]
    async fn test_file_writer_multiple_rotations() {
        let temp_dir = TempDir::new("test_file_writer").unwrap();
        let config = FileWriterConfig {
            directory: temp_dir.path().to_path_buf(),
            prefix: "test".to_string(),
            max_size: 100, // Very small max size
        };

        let mut writer = FileWriter::new(config);
        let request = create_test_request("test_flow");

        let mut rotated_files = Vec::new();

        // Write and rotate multiple times
        for _ in 0..3 {
            // Write requests until we need to rotate
            for _ in 0..100 {
                assert!(writer.write_request(&request).await.is_ok());
                if writer.should_rotate_file() {
                    break;
                }
            }

            if writer.should_rotate_file() {
                if let Some(ctx) = writer.rotate_file().await.expect("failed to rotate file") {
                    rotated_files.push(ctx.file_path);
                }
            }
        }

        // Should have created multiple files
        if !rotated_files.is_empty() {
            assert!(rotated_files.len() > 0);
            for file_path in &rotated_files {
                assert!(file_path.exists());
            }
        }
    }
}
