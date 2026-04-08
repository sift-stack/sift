use sift_rs::{
    common::r#type::v1::ChannelDataType,
    ingest::v1::{
        IngestArbitraryProtobufDataStreamRequest, IngestArbitraryProtobufDataStreamResponse,
        IngestWithConfigDataStreamRequest, IngestWithConfigDataStreamResponse,
    },
    ingestion_configs::v2::{ChannelConfig, FlowConfig},
};
use sift_stream::{
    ChannelValue, Flow, IngestionConfigForm, SiftStreamBuilder, TimeValue, stream::run::RunSelector,
};
use std::sync::{
    Arc,
    atomic::{AtomicU32, Ordering},
};
use tokio::sync::Mutex;
use tokio_stream::StreamExt;

mod common;
use common::prelude::*;

fn standard_flows() -> Vec<FlowConfig> {
    vec![FlowConfig {
        name: "flow-0".to_string(),
        channels: vec![ChannelConfig {
            name: "generator".to_string(),
            data_type: ChannelDataType::Double.into(),
            ..Default::default()
        }],
    }]
}

fn standard_ingestion_config() -> IngestionConfigForm {
    IngestionConfigForm {
        asset_name: "test_asset".to_string(),
        client_key: "test_live_only_client_key".to_string(),
        flows: standard_flows(),
    }
}

/// Simple counting mock
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

/// Run-id capturing mock
struct RunIdCapturingIngestService {
    captured_run_ids: Arc<Mutex<Vec<String>>>,
}

#[async_trait]
impl IngestService for RunIdCapturingIngestService {
    async fn ingest_with_config_data_stream(
        &self,
        request: Request<Streaming<IngestWithConfigDataStreamRequest>>,
    ) -> Result<Response<IngestWithConfigDataStreamResponse>, Status> {
        let mut stream = request.into_inner();
        while let Ok(Some(msg)) = stream.try_next().await {
            self.captured_run_ids.lock().await.push(msg.run_id.clone());
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

/// Live-only mode delivers the correct number of messages end-to-end.
#[tokio::test]
async fn test_live_only_basic_send() {
    let num_received = Arc::new(AtomicU32::default());
    let (client, server) = common::start_test_ingest_server(CountingIngestService {
        num_received: num_received.clone(),
    })
    .await;

    let mut stream = SiftStreamBuilder::from_channel(client)
        .ingestion_config(standard_ingestion_config())
        .live_only()
        .metrics_streaming_interval(None)
        .build()
        .await
        .expect("failed to build live-only stream");

    let num_messages = 50_u32;
    for i in 0..num_messages {
        stream
            .send(Flow::new(
                "flow-0",
                TimeValue::default(),
                &[ChannelValue::new("generator", f64::from(i))],
            ))
            .await
            .expect("send failed");
    }

    stream.finish().await.expect("finish failed");

    assert_eq!(
        num_received.load(Ordering::Relaxed),
        num_messages,
        "server must receive exactly the number of messages sent"
    );
    assert!(server.await.is_ok(), "test server should shut down cleanly");
}

/// `send_requests` delivers a pre-encoded batch in live-only mode.
#[tokio::test]
async fn test_live_only_send_batch_requests() {
    let num_received = Arc::new(AtomicU32::default());
    let (client, server) = common::start_test_ingest_server(CountingIngestService {
        num_received: num_received.clone(),
    })
    .await;

    let mut stream = SiftStreamBuilder::from_channel(client)
        .ingestion_config(standard_ingestion_config())
        .live_only()
        .metrics_streaming_interval(None)
        .build()
        .await
        .expect("failed to build live-only stream");

    let num_messages = 50_u32;
    let requests = (0..num_messages).map(|i| IngestWithConfigDataStreamRequest {
        ingestion_config_id: "any".to_string(),
        flow: "flow-0".to_string(),
        timestamp: Some(pbjson_types::Timestamp::default()),
        channel_values: vec![sift_rs::ingest::v1::IngestWithConfigDataChannelValue {
            r#type: Some(
                sift_rs::ingest::v1::ingest_with_config_data_channel_value::Type::Double(
                    f64::from(i),
                ),
            ),
        }],
        ..Default::default()
    });

    stream
        .send_requests(requests)
        .await
        .expect("send_requests failed");
    stream.finish().await.expect("finish failed");

    assert_eq!(num_received.load(Ordering::Relaxed), num_messages);
    assert!(server.await.is_ok());
}

/// Messages sent after `attach_run_id` carry the run's ID.
/// The mock RunService always returns run_id = "123" for any get_run call.
#[tokio::test]
async fn test_live_only_run_id_propagated_from_builder() {
    let captured_run_ids = Arc::new(Mutex::new(Vec::<String>::new()));
    let (client, server) = common::start_test_ingest_server(RunIdCapturingIngestService {
        captured_run_ids: captured_run_ids.clone(),
    })
    .await;

    // The mock RunService ignores the requested ID and always returns run_id="123".
    let mut stream = SiftStreamBuilder::from_channel(client)
        .ingestion_config(standard_ingestion_config())
        .attach_run_id("any-run-id")
        .live_only()
        .metrics_streaming_interval(None)
        .build()
        .await
        .expect("failed to build live-only stream");

    stream
        .send(Flow::new(
            "flow-0",
            TimeValue::default(),
            &[ChannelValue::new("generator", 1.0_f64)],
        ))
        .await
        .expect("send failed");

    stream.finish().await.expect("finish failed");

    let ids = captured_run_ids.lock().await;
    assert!(
        !ids.is_empty(),
        "at least one message should have been received"
    );
    assert!(
        ids.iter().all(|id| id == "123"),
        "all messages must carry the run_id returned by the mock server, got: {ids:?}"
    );

    assert!(server.await.is_ok());
}

/// After `attach_run` then `detach_run`, subsequent messages carry an empty run_id.
#[tokio::test]
async fn test_live_only_detach_run_clears_run_id() {
    let captured_run_ids = Arc::new(Mutex::new(Vec::<String>::new()));
    let (client, server) = common::start_test_ingest_server(RunIdCapturingIngestService {
        captured_run_ids: captured_run_ids.clone(),
    })
    .await;

    let mut stream = SiftStreamBuilder::from_channel(client)
        .ingestion_config(standard_ingestion_config())
        .live_only()
        .metrics_streaming_interval(None)
        .build()
        .await
        .expect("failed to build live-only stream");

    // Attach a run (mock returns run_id="123").
    stream
        .attach_run(RunSelector::ByForm(sift_stream::RunForm {
            name: "test_run".to_string(),
            client_key: "test_run_key".to_string(),
            ..Default::default()
        }))
        .await
        .expect("attach_run failed");

    // Message with run attached.
    stream
        .send(Flow::new(
            "flow-0",
            TimeValue::default(),
            &[ChannelValue::new("generator", 1.0_f64)],
        ))
        .await
        .expect("send failed");

    // Detach run; subsequent messages should have an empty run_id.
    stream.detach_run();

    stream
        .send(Flow::new(
            "flow-0",
            TimeValue::default(),
            &[ChannelValue::new("generator", 2.0_f64)],
        ))
        .await
        .expect("send after detach failed");

    stream.finish().await.expect("finish failed");

    let ids = captured_run_ids.lock().await;
    assert_eq!(ids.len(), 2, "expected exactly 2 messages");
    assert_eq!(ids[0], "123", "first message must carry run_id from mock");
    assert_eq!(
        ids[1], "",
        "second message must have empty run_id after detach"
    );

    assert!(server.await.is_ok());
}
