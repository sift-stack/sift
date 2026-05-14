use sift_pbfs::BackupsDecoder;
use sift_rs::{
    common::r#type::v1::ChannelDataType,
    ingest::v1::{
        IngestArbitraryProtobufDataStreamRequest, IngestArbitraryProtobufDataStreamResponse,
        IngestWithConfigDataStreamRequest, IngestWithConfigDataStreamResponse,
    },
    ingestion_configs::v2::{ChannelConfig, FlowConfig},
};
use sift_stream::{
    ChannelValue, DiskBackupPolicy, Flow, IngestionConfigForm, SiftStreamBuilder, TimeValue,
};
use std::io::BufReader;
use tempdir::TempDir;
use tokio_stream::StreamExt;

mod common;
use common::prelude::*;

/// No-op ingest service for file-backup tests (data never reaches the server)
struct NoOpIngestService;

#[async_trait]
impl IngestService for NoOpIngestService {
    async fn ingest_with_config_data_stream(
        &self,
        request: Request<Streaming<IngestWithConfigDataStreamRequest>>,
    ) -> Result<Response<IngestWithConfigDataStreamResponse>, Status> {
        // Drain so the client doesn't stall; we don't care about the data.
        let mut stream = request.into_inner();
        while stream.try_next().await.unwrap_or(None).is_some() {}
        Ok(Response::new(IngestWithConfigDataStreamResponse {}))
    }
    async fn ingest_arbitrary_protobuf_data_stream(
        &self,
        _request: Request<Streaming<IngestArbitraryProtobufDataStreamRequest>>,
    ) -> Result<Response<IngestArbitraryProtobufDataStreamResponse>, Status> {
        unimplemented!()
    }
}

fn standard_flows() -> Vec<FlowConfig> {
    vec![FlowConfig {
        name: "flow-0".to_string(),
        channels: vec![ChannelConfig {
            name: "sensor".to_string(),
            data_type: ChannelDataType::Double.into(),
            ..Default::default()
        }],
    }]
}

fn make_flow(value: f64) -> Flow {
    Flow::new(
        "flow-0",
        TimeValue::default(),
        &[ChannelValue::new("sensor", value)],
    )
}

/// Count the total number of successfully decoded messages across all `.pbfs`
/// files in `dir` (non-recursive).
fn count_messages_in_dir(dir: &std::path::Path) -> usize {
    let entries = match std::fs::read_dir(dir) {
        Ok(e) => e,
        Err(_) => return 0,
    };
    let mut total = 0usize;
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_file()
            && let Ok(f) = std::fs::File::open(&path)
        {
            let decoder =
                BackupsDecoder::<IngestWithConfigDataStreamRequest, _>::new(BufReader::new(f));
            total += decoder.filter(|r| r.is_ok()).count();
        }
    }
    total
}

/// All messages sent in file-backup mode land in a backup file on disk.
#[tokio::test]
async fn test_file_backup_basic_write() {
    let (client, server) = common::start_test_ingest_server(NoOpIngestService).await;
    let tmp = TempDir::new("sift_stream_fb_basic").expect("tempdir");

    let mut stream = SiftStreamBuilder::from_channel(client)
        .ingestion_config(IngestionConfigForm {
            asset_name: "test_asset".to_string(),
            client_key: "test_file_backup_key".to_string(),
            flows: standard_flows(),
        })
        .file_backup()
        .disk_backup_policy(DiskBackupPolicy {
            backups_dir: Some(tmp.path().to_path_buf()),
            ..Default::default()
        })
        .metrics_streaming_interval(None)
        .build()
        .await
        .expect("build failed");

    let num_messages = 20_usize;
    for i in 0..num_messages {
        stream.send(make_flow(i as f64)).await.expect("send failed");
    }

    stream.finish().await.expect("finish failed");

    // Files land at backups_dir / asset_name /
    let backup_dir = tmp.path().join("test_asset");
    let total = count_messages_in_dir(&backup_dir);
    assert_eq!(
        total, num_messages,
        "decoded message count must equal the number sent"
    );

    assert!(server.await.is_ok());
}

/// When `max_backup_file_size` is tiny, multiple files must be created.
#[tokio::test]
async fn test_file_backup_file_rotation_creates_multiple_files() {
    let (client, server) = common::start_test_ingest_server(NoOpIngestService).await;
    let tmp = TempDir::new("sift_stream_fb_rotation").expect("tempdir");

    let mut stream = SiftStreamBuilder::from_channel(client)
        .ingestion_config(IngestionConfigForm {
            asset_name: "test_asset".to_string(),
            client_key: "test_rotation_key".to_string(),
            flows: standard_flows(),
        })
        .file_backup()
        .disk_backup_policy(DiskBackupPolicy {
            backups_dir: Some(tmp.path().to_path_buf()),
            // 1-byte threshold guarantees rotation after every message.
            max_backup_file_size: 1,
            ..Default::default()
        })
        .metrics_streaming_interval(None)
        .build()
        .await
        .expect("build failed");

    for i in 0..10_usize {
        stream.send(make_flow(i as f64)).await.expect("send failed");
    }

    stream.finish().await.expect("finish failed");

    let backup_dir = tmp.path().join("test_asset");
    let file_count = std::fs::read_dir(&backup_dir)
        .expect("backup dir must exist")
        .filter(|e| e.as_ref().map(|e| e.path().is_file()).unwrap_or(false))
        .count();

    assert!(
        file_count > 1,
        "rotation must produce more than one backup file, got {file_count}"
    );

    assert!(server.await.is_ok());
}

/// finish() flushes all buffered data — nothing is lost between send and finish.
#[tokio::test]
async fn test_file_backup_finish_flushes_all() {
    let (client, server) = common::start_test_ingest_server(NoOpIngestService).await;
    let tmp = TempDir::new("sift_stream_fb_flush").expect("tempdir");

    let mut stream = SiftStreamBuilder::from_channel(client)
        .ingestion_config(IngestionConfigForm {
            asset_name: "test_asset".to_string(),
            client_key: "test_flush_key".to_string(),
            flows: standard_flows(),
        })
        .file_backup()
        .disk_backup_policy(DiskBackupPolicy {
            backups_dir: Some(tmp.path().to_path_buf()),
            ..Default::default()
        })
        .metrics_streaming_interval(None)
        .build()
        .await
        .expect("build failed");

    // Send without any explicit flush.
    let num_messages = 15_usize;
    for i in 0..num_messages {
        stream.send(make_flow(i as f64)).await.expect("send failed");
    }

    // finish() must flush and sync everything before returning.
    stream.finish().await.expect("finish failed");

    let backup_dir = tmp.path().join("test_asset");
    let total = count_messages_in_dir(&backup_dir);
    assert_eq!(
        total, num_messages,
        "finish must flush all messages — none should be lost"
    );

    assert!(server.await.is_ok());
}

/// Building in file-backup mode without a `backups_dir` must return an error.
#[tokio::test]
async fn test_file_backup_build_fails_without_backups_dir() {
    let (client, server) = common::start_test_ingest_server(NoOpIngestService).await;

    let result = SiftStreamBuilder::from_channel(client)
        .ingestion_config(IngestionConfigForm {
            asset_name: "test_asset".to_string(),
            client_key: "test_no_dir_key".to_string(),
            flows: standard_flows(),
        })
        .file_backup()
        // Deliberately omit disk_backup_policy / backups_dir.
        .build()
        .await;

    assert!(
        result.is_err(),
        "build() must fail when backups_dir is not set"
    );

    server.abort();
}
