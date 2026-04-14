use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use sift_rs::{
    common::r#type::v1::ChannelDataType,
    ingest::v1::{
        IngestArbitraryProtobufDataStreamRequest, IngestArbitraryProtobufDataStreamResponse,
        IngestWithConfigDataStreamRequest, IngestWithConfigDataStreamResponse,
        ingest_service_server::IngestService,
    },
    ingestion_configs::v2::{ChannelConfig, FlowConfig},
};
use sift_stream::{ChannelValue, Flow, IngestionConfigForm, SiftStreamBuilder, TimeValue};
use tokio_stream::StreamExt;
use tracing_test::traced_test;

mod common;

/// Flow name used by the sift_stream log streaming task.
const LOG_FLOW: &str = "sift-stream-logs";

/// Per-flow message counter shared between a test and `FlowCountingIngestService`.
///
/// Call [`FlowMessageCounts::get`] after the stream finishes to assert how many messages
/// arrived for a given flow name (e.g. `"flow-0"`, `"sift-stream-logs"`).
#[derive(Clone, Default)]
pub struct FlowMessageCounts(Arc<Mutex<HashMap<String, u32>>>);

impl FlowMessageCounts {
    pub fn record(&self, flow: &str) {
        *self.0.lock().unwrap().entry(flow.to_owned()).or_insert(0) += 1;
    }

    /// Returns the number of messages received for `flow`, or 0 if none were received.
    pub fn get(&self, flow: &str) -> u32 {
        self.0.lock().unwrap().get(flow).copied().unwrap_or(0)
    }
}

/// An `IngestService` implementation that counts received messages grouped by flow name.
///
/// Construct with [`FlowCountingIngestService::new`], which returns both the service and a
/// [`FlowMessageCounts`] handle the test can hold onto for assertions.
pub struct FlowCountingIngestService {
    counts: FlowMessageCounts,
}

impl FlowCountingIngestService {
    pub fn new() -> (Self, FlowMessageCounts) {
        let counts = FlowMessageCounts::default();
        (
            Self {
                counts: counts.clone(),
            },
            counts,
        )
    }
}

#[tonic::async_trait]
impl IngestService for FlowCountingIngestService {
    async fn ingest_with_config_data_stream(
        &self,
        request: tonic::Request<tonic::Streaming<IngestWithConfigDataStreamRequest>>,
    ) -> Result<tonic::Response<IngestWithConfigDataStreamResponse>, tonic::Status> {
        let mut stream = request.into_inner();
        while let Ok(Some(msg)) = stream.try_next().await {
            self.counts.record(&msg.flow);
        }
        Ok(tonic::Response::new(IngestWithConfigDataStreamResponse {}))
    }

    async fn ingest_arbitrary_protobuf_data_stream(
        &self,
        _request: tonic::Request<tonic::Streaming<IngestArbitraryProtobufDataStreamRequest>>,
    ) -> Result<tonic::Response<IngestArbitraryProtobufDataStreamResponse>, tonic::Status> {
        unimplemented!("not used in flow-counting tests")
    }
}

#[tokio::test]
async fn test_sending_raw_ingest_request() {
    let (service, counts) = FlowCountingIngestService::new();
    let (client, server) = common::start_test_ingest_server(service).await;

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
        .live_with_backups()
        .build()
        .await
        .expect("failed to build sift stream");

    let num_messages = 100u32;

    let requests = (0..num_messages).map(|i| IngestWithConfigDataStreamRequest {
        ingestion_config_id: "ingestion-config-0".into(),
        flow: "flow-0".into(),
        timestamp: Some(pbjson_types::Timestamp::default()),
        channel_values: vec![sift_rs::ingest::v1::IngestWithConfigDataChannelValue {
            r#type: Some(
                sift_rs::ingest::v1::ingest_with_config_data_channel_value::Type::Double(i as f64),
            ),
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
        counts.get("flow-0"),
        "messages sent and received on flow-0 don't match",
    );
    assert!(
        counts.get(LOG_FLOW) > 0,
        "expected log events to be streamed to the log flow"
    );
    assert!(
        server.await.is_ok(),
        "test server shutdown failed unexpectedly"
    );
}

#[tokio::test]
async fn test_sending_flow() {
    let (service, counts) = FlowCountingIngestService::new();
    let (client, server) = common::start_test_ingest_server(service).await;

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
        .live_with_backups()
        .build()
        .await
        .expect("failed to build sift stream");

    let num_messages = 100u32;

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
        counts.get("flow-0"),
        "messages sent and received on flow-0 don't match",
    );
    assert!(
        counts.get(LOG_FLOW) > 0,
        "expected log events to be streamed to the log flow"
    );
    assert!(
        server.await.is_ok(),
        "test server shutdown failed unexpectedly"
    );
}

#[tokio::test]
#[traced_test]
async fn test_sending_flow_not_in_flow_cache() {
    let (service, counts) = FlowCountingIngestService::new();
    let (client, server) = common::start_test_ingest_server(service).await;

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
        .live_with_backups()
        .build()
        .await
        .expect("failed to build sift stream");

    let num_messages = 100u32;

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

    // Messages sent to unregistered channels/flows are not forwarded to Sift; only the
    // messages that actually matched registered flows count toward the total.
    let user_messages_received = counts.get("flow-0") + counts.get("unregistered_flow");
    assert_eq!(
        num_messages, user_messages_received,
        "total user messages received don't match"
    );
    assert!(
        counts.get(LOG_FLOW) > 0,
        "expected log events to be streamed to the log flow"
    );
    assert!(
        server.await.is_ok(),
        "test server shutdown failed unexpectedly"
    );
}
