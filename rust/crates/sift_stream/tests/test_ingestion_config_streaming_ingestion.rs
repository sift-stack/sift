use sift_rs::{
    common::r#type::v1::ChannelDataType,
    ingest::v1::{
        IngestWithConfigDataChannelValue,
        ingest_with_config_data_channel_value::Type as ChannelValue,
    },
    ingestion_configs::v2::{ChannelConfig, FlowConfig, IngestionConfig},
};
use sift_stream::{IngestionConfigMode, SiftStream};
use std::{
    sync::{
        Arc,
        atomic::{AtomicU32, Ordering},
    },
    time::Duration,
};
use tokio_stream::StreamExt;

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
                r#type: Some(ChannelValue::Double(i as f64)),
            },
            IngestWithConfigDataChannelValue {
                r#type: Some(ChannelValue::Empty(pbjson_types::Empty {})),
            },
            IngestWithConfigDataChannelValue {
                r#type: Some(ChannelValue::String("value".into())),
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
