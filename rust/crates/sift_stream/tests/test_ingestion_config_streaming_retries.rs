use chrono::Local;
use sift_rs::{
    common::r#type::v1::ChannelDataType,
    ingestion_configs::v2::{ChannelConfig, FlowConfig},
};
use sift_stream::backup::DiskBackupPolicy;
use sift_stream::{
    ChannelValue, Flow, IngestionConfigForm, RecoveryStrategy, RetryPolicy, SiftStreamBuilder,
    TimeValue,
};
use std::{
    sync::{
        Arc,
        atomic::{AtomicBool, AtomicU32, Ordering},
    },
    time::Duration,
};
use tempdir::TempDir;

mod common;
use common::prelude::*;

struct IngestServiceMock {
    num_stream_opened: Arc<AtomicU32>,
    num_messages_received: Arc<AtomicU32>,
    return_error: Arc<AtomicBool>,
}

#[async_trait]
impl IngestService for IngestServiceMock {
    async fn ingest_with_config_data_stream(
        &self,
        request: Request<Streaming<IngestWithConfigDataStreamRequest>>,
    ) -> Result<Response<IngestWithConfigDataStreamResponse>, Status> {
        self.num_stream_opened.fetch_add(1, Ordering::Relaxed);

        if self.return_error.load(Ordering::Relaxed) {
            return Err(Status::resource_exhausted("resource exhausted"));
        }
        let mut data_stream = request.into_inner();

        while let Ok(Some(_)) = data_stream.message().await {
            self.num_messages_received.fetch_add(1, Ordering::Relaxed);
        }

        Ok(Response::new(IngestWithConfigDataStreamResponse {}))
    }
    async fn ingest_arbitrary_protobuf_data_stream(
        &self,
        _request: Request<Streaming<IngestArbitraryProtobufDataStreamRequest>>,
    ) -> Result<Response<IngestArbitraryProtobufDataStreamResponse>, Status> {
        unimplemented!("not relevant to this test")
    }
}

#[tokio::test]
async fn test_retries_succeed() {
    let return_error = Arc::new(AtomicBool::new(true));

    let num_messages_received = Arc::new(AtomicU32::default());
    let num_streams_opened = Arc::new(AtomicU32::default());

    let ingest_service = IngestServiceMock {
        num_stream_opened: num_streams_opened.clone(),
        num_messages_received: num_messages_received.clone(),
        return_error: return_error.clone(),
    };

    let (client, server) = common::start_test_ingest_server(ingest_service).await;

    let flows = vec![FlowConfig {
        name: "flow-0".to_string(),
        channels: vec![ChannelConfig {
            name: "generator".to_string(),
            data_type: ChannelDataType::Double.into(),
            ..Default::default()
        }],
    }];

    let tmp_dir = TempDir::new("test_retries_succeed").expect("failed to create tempdir");
    let tmp_dir_path = tmp_dir.path();

    let disk_backup_policy = DiskBackupPolicy {
        backups_dir: Some(tmp_dir_path.to_path_buf()),
        ..Default::default()
    };
    let mut sift_stream = SiftStreamBuilder::from_channel(client)
        .ingestion_config(IngestionConfigForm {
            asset_name: "test_asset".to_string(),
            client_key: "test_client_key".to_string(),
            flows,
        })
        .recovery_strategy(RecoveryStrategy::RetryWithBackups {
            retry_policy: RetryPolicy {
                max_attempts: 2,
                initial_backoff: Duration::from_millis(1),
                backoff_multiplier: 2,
                max_backoff: Duration::from_millis(100),
            },
            disk_backup_policy,
        })
        .metrics_streaming_interval(None)
        .build()
        .await
        .expect("failed to build sift stream");

    // Send some messages while the server is returning errors.
    //
    // None of these messages should be captured by the mock server.
    while num_streams_opened.load(Ordering::Relaxed) < 3 as u32 {
        let msg = Flow::new(
            "flow",
            TimeValue::from(Local::now().to_utc()),
            &[ChannelValue::new("generator", 1.0)],
        );
        assert!(
            sift_stream.send(msg).await.is_ok(),
            "sending should always succeed"
        );
        tokio::time::sleep(Duration::from_millis(10)).await;
    }

    return_error.swap(false, Ordering::Relaxed);

    // After recover, send more messages to verify correct recovery.
    let num_messages = 100;
    for _ in 0..num_messages {
        let msg = Flow::new(
            "flow",
            TimeValue::from(Local::now().to_utc()),
            &[ChannelValue::new("generator", 1.0)],
        );
        assert!(
            sift_stream.send(msg).await.is_ok(),
            "we should have successfully recovered and not gotten an error"
        );
        tokio::time::sleep(Duration::from_millis(10)).await;
    }

    assert_eq!(
        num_messages as u32,
        num_messages_received.load(Ordering::Relaxed),
        "expected no messages to be dropped",
    );

    assert!(
        server.await.is_ok(),
        "test server shutdown failed unexpectedly"
    );
}

#[tokio::test]
pub async fn test_retries_exhausted() {
    let return_error = Arc::new(AtomicBool::new(true));

    let num_messages_received = Arc::new(AtomicU32::default());
    let num_streams_opened = Arc::new(AtomicU32::default());

    let ingest_service = IngestServiceMock {
        num_stream_opened: num_streams_opened.clone(),
        num_messages_received: num_messages_received.clone(),
        return_error: return_error.clone(),
    };

    let (client, server) = common::start_test_ingest_server(ingest_service).await;

    let flows = vec![FlowConfig {
        name: "flow-0".to_string(),
        channels: vec![ChannelConfig {
            name: "generator".to_string(),
            data_type: ChannelDataType::Double.into(),
            ..Default::default()
        }],
    }];

    let retry_attempts = 3;

    let mut sift_stream = SiftStreamBuilder::from_channel(client)
        .ingestion_config(IngestionConfigForm {
            asset_name: "test_asset".to_string(),
            client_key: "test_client_key".to_string(),
            flows,
        })
        .recovery_strategy(RecoveryStrategy::RetryOnly(RetryPolicy {
            max_attempts: retry_attempts,
            initial_backoff: Duration::from_millis(1),
            backoff_multiplier: 2,
            max_backoff: Duration::from_millis(100),
        }))
        .build()
        .await
        .expect("failed to build sift stream");

    while num_streams_opened.load(Ordering::Relaxed) < retry_attempts as u32 {
        let msg = Flow::new(
            "flow",
            TimeValue::from(Local::now().to_utc()),
            &[ChannelValue::new("generator", 1.0)],
        );
        assert!(
            sift_stream.send(msg).await.is_ok(),
            "we should have successfully recovered and not gotten an error"
        );
        tokio::time::sleep(Duration::from_millis(100)).await;
    }

    assert!(sift_stream.finish().await.is_ok());

    assert!(
        server.await.is_ok(),
        "test server shutdown failed unexpectedly"
    );
}
