use chrono::Local;
use sift_error::ErrorKind;
use sift_rs::{
    common::r#type::v1::ChannelDataType,
    ingestion_configs::v2::{ChannelConfig, FlowConfig, IngestionConfig},
};
use sift_stream::{ChannelValue, IngestionConfigMode, Message, RetryPolicy, SiftStream, TimeValue};
use std::{
    sync::{
        atomic::{AtomicBool, AtomicU32, Ordering},
        Arc,
    },
    time::Duration,
};

mod common;
use common::prelude::*;

struct IngestServiceMock {
    num_messages_received: Arc<AtomicU32>,
    return_error: Arc<AtomicBool>,
}

#[async_trait]
impl IngestService for IngestServiceMock {
    async fn ingest_with_config_data_stream(
        &self,
        request: Request<Streaming<IngestWithConfigDataStreamRequest>>,
    ) -> Result<Response<IngestWithConfigDataStreamResponse>, Status> {
        let mut data_stream = request.into_inner();

        while let Ok(Some(_)) = data_stream.message().await {
            self.num_messages_received.fetch_add(1, Ordering::Relaxed);
            if self.return_error.load(Ordering::Relaxed) {
                return Err(Status::resource_exhausted("resource exhausted"));
            }
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
pub async fn test_retries_succeed() {
    let return_error = Arc::new(AtomicBool::new(true));

    let num_messages = 1_000;
    let mut messages = (0..num_messages).map(|i| {
        Message::new(
            "flow",
            TimeValue::from(Local::now().to_utc()),
            &[ChannelValue::new("generator", i as f64)],
        )
    });

    let num_messages_received = Arc::new(AtomicU32::default());

    let ingest_service = IngestServiceMock {
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
        Duration::from_secs(60),
        Some(RetryPolicy::default()),
    );

    let mut error = None;
    loop {
        if let Some(msg) = messages.next() {
            if let Err(err) = sift_stream.send(msg) {
                error = Some(err);
                break;
            }
            common::task_yield().await;
            continue;
        }
        break;
    }
    assert!(error.is_some(), "expected to encounter a server error");

    let error = error.unwrap();
    assert!(
        format!("{error}").contains("resource exhausted"),
        "expected a resource exhausted error"
    );
    assert_eq!(
        error.kind(),
        ErrorKind::StreamErrorRetriable,
        "expected error to indicate that stream can be retried"
    );

    let send_result = sift_stream.send(Message::new(
        "flow",
        TimeValue::default(),
        &[ChannelValue::new("generator", 1.0_f64)],
    ));
    assert!(
        send_result.is_err(),
        "should not be able to send when stream is broken"
    );
    assert!(format!("{}", send_result.unwrap_err()).contains("possible to retry"));

    assert!(return_error.swap(false, Ordering::Relaxed));
    assert!(
        sift_stream.retry().await.is_ok(),
        "retry should have succeeded"
    );

    for message in messages {
        assert!(
            sift_stream.send(message).is_ok(),
            "we should be able to call send"
        );
    }
    assert!(
        sift_stream.finish().await.is_ok(),
        "expected stream to terminate without error"
    );

    assert_eq!(
        num_messages as u32,
        num_messages_received.load(Ordering::Relaxed),
        "number of messages received by the mock server and number of messaged sent should match",
    );

    assert!(
        server.await.is_ok(),
        "test server shutdown failed unexpectedly"
    );
}

#[tokio::test]
pub async fn test_retries_fail() {
    let return_error = Arc::new(AtomicBool::new(true));

    let num_messages = 1_000;
    let mut messages = (0..num_messages).map(|i| {
        Message::new(
            "flow",
            TimeValue::from(Local::now().to_utc()),
            &[ChannelValue::new("generator", i as f64)],
        )
    });

    let num_messages_received = Arc::new(AtomicU32::default());

    let ingest_service = IngestServiceMock {
        num_messages_received,
        return_error,
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
        Duration::from_secs(60),
        Some(RetryPolicy::default()),
    );

    let mut error = None;
    loop {
        if let Some(msg) = messages.next() {
            if let Err(err) = sift_stream.send(msg) {
                error = Some(err);
                break;
            }
            common::task_yield().await;
            continue;
        }
        break;
    }
    assert!(error.is_some(), "expected to encounter a server error");

    let error = error.unwrap();
    assert!(
        format!("{error}").contains("resource exhausted"),
        "expected a resource exhausted error"
    );
    assert_eq!(
        error.kind(),
        ErrorKind::StreamErrorRetriable,
        "expected error to indicate that stream can be retried"
    );

    let retry_result = sift_stream.retry().await;
    assert!(
        retry_result.is_err(),
        "expected all retries to be exhausted"
    );
    let error = retry_result.unwrap_err();
    assert!(format!("{error}").contains("exhausted all retry attempts"));

    assert!(
        server.await.is_ok(),
        "test server shutdown failed unexpectedly"
    );
}
