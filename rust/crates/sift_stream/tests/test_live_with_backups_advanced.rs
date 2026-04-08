use sift_rs::{
    common::r#type::v1::ChannelDataType,
    ingest::v1::{
        IngestArbitraryProtobufDataStreamRequest, IngestArbitraryProtobufDataStreamResponse,
        IngestWithConfigDataStreamRequest, IngestWithConfigDataStreamResponse,
    },
    ingestion_configs::v2::{ChannelConfig, FlowConfig},
};
use sift_stream::{
    ChannelValue, DiskBackupPolicy, Flow, IngestionConfigForm, RetryPolicy, SiftStreamBuilder,
    TimeValue,
};
use std::{
    sync::{
        Arc,
        atomic::{AtomicU32, Ordering},
    },
    time::Duration,
};
use tempdir::TempDir;
use tokio_stream::StreamExt;

mod common;
use common::prelude::*;

struct CountingIngestService {
    num_received: Arc<AtomicU32>,
}

#[async_trait]
impl IngestService for CountingIngestService {
    async fn ingest_with_config_data_stream(
        &self,
        request: Request<Streaming<IngestWithConfigDataStreamRequest>>,
    ) -> Result<Response<IngestWithConfigDataStreamResponse>, Status> {
        let mut stream = request.into_inner();
        while let Ok(Some(_)) = stream.try_next().await {
            self.num_received.fetch_add(1, Ordering::Relaxed);
        }
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

/// With `retain_backups = true`, backup files must still exist on disk after
/// a successful checkpoint (triggered by `finish()`).
#[tokio::test]
async fn test_retain_backups_keeps_files_after_checkpoint() {
    let num_received = Arc::new(AtomicU32::default());
    let (client, server) = common::start_test_ingest_server(CountingIngestService {
        num_received: num_received.clone(),
    })
    .await;

    let tmp = TempDir::new("sift_stream_retain").expect("tempdir");

    let mut stream = SiftStreamBuilder::from_channel(client)
        .ingestion_config(IngestionConfigForm {
            asset_name: "test_asset".to_string(),
            client_key: "test_retain_key".to_string(),
            flows: standard_flows(),
        })
        .live_with_backups()
        .checkpoint_interval(Duration::from_secs(1))
        .disk_backup_policy(DiskBackupPolicy {
            backups_dir: Some(tmp.path().to_path_buf()),
            retain_backups: true,
            ..Default::default()
        })
        .metrics_streaming_interval(None)
        .retry_policy(RetryPolicy {
            max_attempts: 3,
            initial_backoff: Duration::from_millis(10),
            backoff_multiplier: 2,
            max_backoff: Duration::from_millis(100),
        })
        .build()
        .await
        .expect("build failed");

    let num_messages = 20_u32;
    for i in 0..num_messages {
        stream
            .send(make_flow(f64::from(i)))
            .await
            .expect("send failed");
    }

    // finish() triggers a final checkpoint; with retain_backups=true files must survive.
    stream.finish().await.expect("finish failed");

    // Backup files land at backups_dir/test_asset/.
    let backup_dir = tmp.path().join("test_asset");
    let file_count = std::fs::read_dir(&backup_dir)
        .expect("backup directory must exist")
        .filter(|e| e.as_ref().map(|e| e.path().is_file()).unwrap_or(false))
        .count();

    assert!(
        file_count > 0,
        "backup files must be retained after a successful checkpoint when retain_backups=true"
    );

    assert!(server.await.is_ok());
}
