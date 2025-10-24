use sift_rs::{
    common::r#type::v1::ChannelDataType,
    ingest::v1::{
        IngestWithConfigDataChannelValue,
        ingest_with_config_data_channel_value::Type as RawChannelValue,
    },
    ingestion_configs::v2::{ChannelConfig, FlowConfig},
};
use sift_stream::{ChannelValue, Flow, IngestionConfigForm, SiftStreamBuilder, TimeValue};
use std::sync::{
    Arc,
    atomic::{AtomicU32, Ordering},
};
use tokio_stream::StreamExt;
use tracing_test::traced_test;

mod common;
use common::prelude::*;

struct IngestServiceMock {
    num_message_received: Arc<AtomicU32>,
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
async fn test_sending_raw_ingest_request() {
    let messages_received = Arc::new(AtomicU32::default());

    let ingest_service = IngestServiceMock {
        num_message_received: messages_received.clone(),
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

    let mut sift_stream = SiftStreamBuilder::from_channel(client)
        .ingestion_config(IngestionConfigForm {
            asset_name: "test_asset".to_string(),
            client_key: "test_client_key".to_string(),
            flows,
        })
        .build()
        .await
        .expect("failed to build sift stream");

    let num_messages = 100;

    let requests = (0..num_messages).map(|i| IngestWithConfigDataStreamRequest {
        ingestion_config_id: "ingestion-config-0".into(),
        flow: "flow-0".into(),
        timestamp: Some(pbjson_types::Timestamp::default()),
        channel_values: vec![IngestWithConfigDataChannelValue {
            r#type: Some(RawChannelValue::Double(i as f64)),
        }],
        ..Default::default()
    });

    assert!(
        sift_stream.send_requests(requests).await.is_ok(),
        "failed to send requests"
    );
    assert!(sift_stream.finish().await.is_ok(), "finish call failed");

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
async fn test_sending_flow() {
    let messages_received = Arc::new(AtomicU32::default());

    let ingest_service = IngestServiceMock {
        num_message_received: messages_received.clone(),
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

    let mut sift_stream = SiftStreamBuilder::from_channel(client)
        .ingestion_config(IngestionConfigForm {
            asset_name: "test_asset".to_string(),
            client_key: "test_client_key".to_string(),
            flows,
        })
        .build()
        .await
        .expect("failed to build sift stream");

    let num_messages = 100;

    let messages = (0..num_messages).map(|i| {
        Flow::new(
            "flow-0".to_string(),
            TimeValue::default(),
            &[ChannelValue::new("generator", i as f64)],
        )
    });

    for message in messages {
        assert!(
            sift_stream.send(message).await.is_ok(),
            "failed to send requests"
        );
    }
    assert!(sift_stream.finish().await.is_ok(), "finish call failed");

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
#[traced_test]
async fn test_sending_flow_not_in_flow_cache() {
    let messages_received = Arc::new(AtomicU32::default());

    let ingest_service = IngestServiceMock {
        num_message_received: messages_received.clone(),
    };

    let (client, server) = common::start_test_ingest_server(ingest_service).await;

    let expected_flows = vec![FlowConfig {
        name: "flow-0".to_string(),
        channels: vec![ChannelConfig {
            name: "generator".to_string(),
            data_type: ChannelDataType::Double.into(),
            ..Default::default()
        }],
    }];

    let mut sift_stream = SiftStreamBuilder::from_channel(client)
        .ingestion_config(IngestionConfigForm {
            asset_name: "test_asset".to_string(),
            client_key: "test_client_key".to_string(),
            flows: expected_flows,
        })
        .build()
        .await
        .expect("failed to build sift stream");

    let num_messages = 100;

    // First send some messages that have a channel not in the expected flow.
    let messages = (0..num_messages / 2).map(|i| {
        Flow::new(
            "flow-0".to_string(),
            TimeValue::default(),
            &[ChannelValue::new("unregistered_channel", i as f64)],
        )
    });

    for message in messages {
        assert!(
            sift_stream.send(message).await.is_ok(),
            "failed to send requests"
        );
    }

    assert!(logs_contain(
        "encountered a message that doesn't match any cached flows"
    ));

    // Next, send some messages that have a flow name not in the expected flows.
    let messages = (0..num_messages / 2).map(|i| {
        Flow::new(
            "unregistered_flow",
            TimeValue::default(),
            &[ChannelValue::new("unregistered_channel", i as f64)],
        )
    });

    for message in messages {
        assert!(
            sift_stream.send(message).await.is_ok(),
            "failed to send requests"
        );
    }
    assert!(logs_contain(
        "flow 'unregistered_flow' not found in local flow cache"
    ));

    assert!(sift_stream.finish().await.is_ok(), "finish call failed");

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
