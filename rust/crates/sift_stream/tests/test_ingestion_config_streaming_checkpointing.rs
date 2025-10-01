use chrono::Local;
use sift_rs::{
    common::r#type::v1::ChannelDataType,
    ingestion_configs::v2::{ChannelConfig, FlowConfig, IngestionConfig},
};
use sift_stream::{
    ChannelValue, Flow, IngestionConfigMode, SiftStream, TimeValue, metrics::SiftStreamMetrics,
};
use std::{
    sync::{
        Arc,
        atomic::{AtomicU32, Ordering},
    },
    time::Duration,
};
use tokio::sync::oneshot::{self, error::TryRecvError};
use tokio_stream::StreamExt;

mod common;
use common::prelude::*;

struct IngestServiceMock {
    num_message_received: Arc<AtomicU32>,
    num_checkpoints: Arc<AtomicU32>,
}

#[async_trait]
impl IngestService for IngestServiceMock {
    async fn ingest_with_config_data_stream(
        &self,
        request: Request<Streaming<IngestWithConfigDataStreamRequest>>,
    ) -> Result<Response<IngestWithConfigDataStreamResponse>, Status> {
        let mut data_stream = request.into_inner();

        loop {
            match data_stream.try_next().await {
                Ok(Some(_msg)) => {
                    self.num_message_received.fetch_add(1, Ordering::Relaxed);
                }
                // Client has ended the stream and is requesting a checkpoint
                Ok(None) => {
                    self.num_checkpoints.fetch_add(1, Ordering::Relaxed);
                    break;
                }
                Err(err) => return Err(err),
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
async fn test_sending_data() {
    let num_checkpoints = Arc::new(AtomicU32::default());
    let messages_received = Arc::new(AtomicU32::default());

    let ingest_service = IngestServiceMock {
        num_checkpoints: num_checkpoints.clone(),
        num_message_received: messages_received.clone(),
    };

    let (client, server) = common::start_test_ingest_server(ingest_service).await;

    let ingestion_config = IngestionConfig {
        ingestion_config_id: "ingestion-config-id".to_string(),
        client_key: "ingestion-config-client-key".to_string(),
        asset_id: "asset-id".to_string(),
    };
    let flows = vec![FlowConfig {
        name: "wheel".to_string(),
        channels: vec![
            ChannelConfig {
                name: "angular_velocity".to_string(),
                data_type: ChannelDataType::Double.into(),
                ..Default::default()
            },
            ChannelConfig {
                name: "log".to_string(),
                data_type: ChannelDataType::String.into(),
                ..Default::default()
            },
        ],
    }];

    let mut sift_stream = SiftStream::<IngestionConfigMode>::new(
        client,
        ingestion_config,
        flows,
        None,
        Duration::from_secs(30),
        None,
        None,
        Arc::new(SiftStreamMetrics::new()),
    );

    let num_messages = 100;

    for _ in 0..num_messages {
        let send_result = sift_stream
            .send(Flow::new(
                "wheel",
                TimeValue::from(Local::now().to_utc()),
                &[
                    ChannelValue::new("angular_velocity", 1.0_f64),
                    ChannelValue::new("log", "foobar"),
                ],
            ))
            .await;
        assert!(send_result.is_ok(), "streaming failed unexpectedly");
    }

    assert!(sift_stream.finish().await.is_ok());

    assert_eq!(
        1,
        num_checkpoints.load(Ordering::Relaxed),
        "always at least 1 checkpoint due to call to finish"
    );

    assert_eq!(
        num_messages,
        messages_received.load(Ordering::Relaxed),
        "messages sent and received don't match",
    );

    assert!(
        server.await.is_ok(),
        "test server shutdown failed unexpectedly"
    );
}

#[tokio::test]
async fn test_checkpointing() {
    let num_checkpoints = Arc::new(AtomicU32::new(0));
    let messages_received = Arc::new(AtomicU32::new(0));

    let ingest_service = IngestServiceMock {
        num_checkpoints: num_checkpoints.clone(),
        num_message_received: messages_received.clone(),
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

    let checkpoint_interval = Duration::from_secs(1);
    let mut sift_stream = SiftStream::<IngestionConfigMode>::new(
        client,
        ingestion_config,
        flows,
        None,
        checkpoint_interval,
        None,
        None,
        Arc::new(SiftStreamMetrics::new()),
    );

    let (terminate_streaming_tx, mut terminate_streaming_rx) = oneshot::channel::<()>();

    let streaming_task = tokio::task::spawn(async move {
        let mut messages_sent: u32 = 0;

        while let Err(TryRecvError::Empty) = terminate_streaming_rx.try_recv() {
            let timestamp = TimeValue::from(Local::now().to_utc());

            let send_result = sift_stream
                .send(Flow::new(
                    "flow",
                    timestamp,
                    &[ChannelValue::new("generator", 1.0_f64)],
                ))
                .await;
            assert!(send_result.is_ok(), "streaming failed unexpectedly");
            messages_sent += 1;
        }

        assert!(sift_stream.finish().await.is_ok());

        messages_sent
    });

    // Pad an additional second
    tokio::time::sleep(checkpoint_interval * 4).await;

    assert!(
        terminate_streaming_tx.send(()).is_ok(),
        "failed to terminate streaming"
    );

    let messages_sent = streaming_task
        .await
        .expect("something went wrong when terminating streaming task");

    assert_eq!(
        4_u32,
        num_checkpoints.load(Ordering::Relaxed),
        "with a checkpoint interval of 1 second, a sleep of 3.5 seconds, and a call to finish, we should have gotten 4 checkpoints"
    );

    assert_eq!(
        messages_sent,
        messages_received.load(Ordering::Relaxed),
        "messages sent and received don't match",
    );

    assert!(
        server.await.is_ok(),
        "test server shutdown failed unexpectedly"
    );
}
