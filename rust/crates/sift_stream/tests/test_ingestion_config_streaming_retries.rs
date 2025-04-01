use chrono::Local;
use sift_rs::{
    common::r#type::v1::ChannelDataType,
    ingestion_configs::v2::{ChannelConfig, FlowConfig, IngestionConfig},
};
use sift_stream::{
    ChannelValue, Flow, IngestionConfigMode, RetryPolicy, SiftStream, TimeValue,
    stream::mode::ingestion_config::IngestionConfigModeBackupsManager,
};
use std::{
    sync::{
        Arc,
        atomic::{AtomicBool, AtomicU32, Ordering},
    },
    time::Duration,
};

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

    let num_messages = 1_000;
    let mut messages = (0..num_messages).map(|i| {
        Flow::new(
            "flow",
            TimeValue::from(Local::now().to_utc()),
            &[ChannelValue::new("generator", i as f64)],
        )
    });

    let num_messages_received = Arc::new(AtomicU32::default());
    let num_streams_opened = Arc::new(AtomicU32::default());

    let ingest_service = IngestServiceMock {
        num_stream_opened: num_streams_opened.clone(),
        num_messages_received: num_messages_received.clone(),
        return_error: return_error.clone(),
    };

    let (client, server) = common::start_test_ingest_server(ingest_service).await;

    let ingestion_config = IngestionConfig {
        ingestion_config_id: "ingestion-config-id".to_string(),
        client_key: "ingestion-config-client-key".to_string(),
        asset_id: "asset-id".to_string(),
    };
    let flows = vec![FlowConfig {
        name: "flow".to_string(),
        channels: vec![ChannelConfig {
            name: "generator".to_string(),
            data_type: ChannelDataType::Double.into(),
            ..Default::default()
        }],
    }];

    let mut sift_stream = SiftStream::<IngestionConfigMode>::new(
        client,
        ingestion_config,
        flows,
        None,
        Duration::from_secs(60),
        Some(RetryPolicy::default()),
        Some(IngestionConfigModeBackupsManager::default()),
    );

    tokio::spawn(async move {
        // let streams fail a few times
        while num_streams_opened.load(Ordering::Relaxed) < 3 {
            tokio::time::sleep(Duration::from_millis(100)).await;
        }
        return_error.swap(false, Ordering::Relaxed);
    });

    let _ = sift_stream.send(messages.next().unwrap()).await.unwrap();

    loop {
        if let Some(msg) = messages.next() {
            let send_result = sift_stream.send(msg).await;
            assert!(
                send_result.is_ok(),
                "we should have successfully recovered and not gotten an error"
            );
            continue;
        }
        break;
    }

    let _ = sift_stream.finish().await.unwrap();

    assert_eq!(
        num_messages + 1, // We add 1 because 1 redundant request will be sent when trying to
        // re-establish a connection.
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

    let num_messages = 1_000;
    let mut messages = (0..num_messages).map(|i| {
        Flow::new(
            "flow",
            TimeValue::from(Local::now().to_utc()),
            &[ChannelValue::new("generator", i as f64)],
        )
    });

    let num_messages_received = Arc::new(AtomicU32::default());
    let num_streams_opened = Arc::new(AtomicU32::default());

    let ingest_service = IngestServiceMock {
        num_stream_opened: num_streams_opened.clone(),
        num_messages_received: num_messages_received.clone(),
        return_error: return_error.clone(),
    };

    let (client, server) = common::start_test_ingest_server(ingest_service).await;

    let ingestion_config = IngestionConfig {
        ingestion_config_id: "ingestion-config-id".to_string(),
        client_key: "ingestion-config-client-key".to_string(),
        asset_id: "asset-id".to_string(),
    };
    let flows = vec![FlowConfig {
        name: "flow".to_string(),
        channels: vec![ChannelConfig {
            name: "generator".to_string(),
            data_type: ChannelDataType::Double.into(),
            ..Default::default()
        }],
    }];

    let retry_attempts = 3;

    let mut sift_stream = SiftStream::<IngestionConfigMode>::new(
        client,
        ingestion_config,
        flows,
        None,
        Duration::from_secs(60),
        Some(RetryPolicy {
            max_attempts: retry_attempts,
            initial_backoff: Duration::from_millis(1),
            backoff_multiplier: 2,
            max_backoff: Duration::from_millis(100),
        }),
        None,
    );

    let mut error = None;
    loop {
        if let Some(msg) = messages.next() {
            if let Err(err) = sift_stream.send(msg).await {
                error = Some(err);
                break;
            }
            continue;
        }
        break;
    }
    assert!(error.is_some(), "expected to encounter a server error");
    assert_eq!(
        u32::from(retry_attempts + 1),
        num_streams_opened.load(Ordering::Relaxed),
        "expected number of streams opened to equal retry_attempts + 1"
    );

    assert!(sift_stream.finish().await.is_ok());

    assert!(
        server.await.is_ok(),
        "test server shutdown failed unexpectedly"
    );
}
