use sift_rs::{
    common::r#type::v1::ChannelDataType,
    ingest::v1::{
        IngestWithConfigDataChannelValue,
        ingest_with_config_data_channel_value::Type as RawChannelValue,
    },
    ingestion_configs::v2::{ChannelConfig, FlowConfig, IngestionConfig},
};
use sift_stream::{ChannelValue, Flow, IngestionConfigMode, SiftStream, TimeValue};
use std::{
    sync::{
        Arc,
        atomic::{AtomicU32, Ordering},
    },
    time::Duration,
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

    let ingestion_config_id = "ingestion-config-id";

    let ingestion_config = IngestionConfig {
        ingestion_config_id: ingestion_config_id.into(),
        client_key: "ingestion-config-client-key".into(),
        asset_id: "asset-id".into(),
    };

    let flow_name = "wheel";
    let angular_velocity = "angular_velocity";
    let torque = "torque";
    let log = "log";

    let flows = vec![FlowConfig {
        name: "wheel".into(),
        channels: vec![
            ChannelConfig {
                name: angular_velocity.into(),
                data_type: ChannelDataType::Double.into(),
                ..Default::default()
            },
            ChannelConfig {
                name: torque.into(),
                data_type: ChannelDataType::Double.into(),
                ..Default::default()
            },
            ChannelConfig {
                name: log.into(),
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
    );

    let num_messages = 100;

    let requests = (0..num_messages).map(|i| IngestWithConfigDataStreamRequest {
        ingestion_config_id: ingestion_config_id.into(),
        flow: flow_name.into(),
        timestamp: Some(pbjson_types::Timestamp::default()),
        channel_values: vec![
            IngestWithConfigDataChannelValue {
                r#type: Some(RawChannelValue::Double(i as f64)),
            },
            IngestWithConfigDataChannelValue {
                r#type: Some(RawChannelValue::Empty(pbjson_types::Empty {})),
            },
            IngestWithConfigDataChannelValue {
                r#type: Some(RawChannelValue::String("value".into())),
            },
        ],
        ..Default::default()
    });

    sift_stream
        .send_requests(requests)
        .await
        .expect("failed to send requests");
    sift_stream.finish().await.expect("finish call failed");

    assert_eq!(
        num_messages,
        messages_received.load(Ordering::Relaxed),
        "messages sent and received don't match",
    );
    server
        .await
        .expect("test server shutdown failed unexpectedly");
}

#[tokio::test]
async fn test_sending_flow() {
    let messages_received = Arc::new(AtomicU32::default());

    let ingest_service = IngestServiceMock {
        num_message_received: messages_received.clone(),
    };

    let (client, server) = common::start_test_ingest_server(ingest_service).await;

    let ingestion_config_id = "ingestion-config-id";

    let ingestion_config = IngestionConfig {
        ingestion_config_id: ingestion_config_id.into(),
        client_key: "ingestion-config-client-key".into(),
        asset_id: "asset-id".into(),
    };

    let flow_name = "wheel";
    let angular_velocity = "angular_velocity";
    let torque = "torque";
    let log = "log";

    let flows = vec![FlowConfig {
        name: "wheel".into(),
        channels: vec![
            ChannelConfig {
                name: angular_velocity.into(),
                data_type: ChannelDataType::Double.into(),
                ..Default::default()
            },
            ChannelConfig {
                name: torque.into(),
                data_type: ChannelDataType::Double.into(),
                ..Default::default()
            },
            ChannelConfig {
                name: log.into(),
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
    );

    let num_messages = 100;

    let messages = (0..num_messages).map(|i| {
        Flow::new(
            flow_name,
            TimeValue::default(),
            &[
                ChannelValue::new(angular_velocity, i as f64),
                ChannelValue::new(torque, i as f64),
                ChannelValue::new(log, "some_log"),
            ],
        )
    });

    for message in messages {
        sift_stream
            .send(message)
            .await
            .expect("failed to send requests");
    }
    sift_stream.finish().await.expect("finish call failed");

    assert_eq!(
        num_messages,
        messages_received.load(Ordering::Relaxed),
        "messages sent and received don't match",
    );
    server
        .await
        .expect("test server shutdown failed unexpectedly");
}

#[tokio::test]
#[traced_test]
async fn test_sending_flow_not_in_flow_cache() {
    let messages_received = Arc::new(AtomicU32::default());

    let ingest_service = IngestServiceMock {
        num_message_received: messages_received.clone(),
    };

    let (client, server) = common::start_test_ingest_server(ingest_service).await;

    let ingestion_config_id = "ingestion-config-id";

    let ingestion_config = IngestionConfig {
        ingestion_config_id: ingestion_config_id.into(),
        client_key: "ingestion-config-client-key".into(),
        asset_id: "asset-id".into(),
    };

    let flow_name = "wheel";
    let angular_velocity = "angular_velocity";
    let torque = "torque";
    let log = "log";

    let flows = vec![FlowConfig {
        name: "wheel".into(),
        channels: vec![
            ChannelConfig {
                name: angular_velocity.into(),
                data_type: ChannelDataType::Double.into(),
                ..Default::default()
            },
            ChannelConfig {
                name: torque.into(),
                data_type: ChannelDataType::Double.into(),
                ..Default::default()
            },
            ChannelConfig {
                name: log.into(),
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
    );

    let num_messages = 100;

    let messages = (0..num_messages / 2).map(|i| {
        Flow::new(
            flow_name,
            TimeValue::default(),
            &[ChannelValue::new("unregistered_channel", i as f64)],
        )
    });
    for message in messages {
        sift_stream
            .send(message)
            .await
            .expect("failed to send requests");
    }
    assert!(logs_contain(
        "encountered a message that doesn't match any cached flows"
    ));

    let messages = (0..num_messages / 2).map(|i| {
        Flow::new(
            "unregistered_flow",
            TimeValue::default(),
            &[ChannelValue::new("unregistered_channel", i as f64)],
        )
    });

    for message in messages {
        sift_stream
            .send(message)
            .await
            .expect("failed to send requests");
    }
    assert!(logs_contain(
        &"flow 'unregistered_flow' not found in local flow cache"
    ));

    sift_stream.finish().await.expect("finish call failed");

    assert_eq!(
        num_messages,
        messages_received.load(Ordering::Relaxed),
        "messages sent and received don't match",
    );
    server
        .await
        .expect("test server shutdown failed unexpectedly");
}
