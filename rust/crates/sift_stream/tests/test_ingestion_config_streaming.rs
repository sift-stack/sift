use async_trait::async_trait;
use chrono::Local;
use hyper_util::rt::TokioIo;
use sift_connect::{grpc::interceptor::AuthInterceptor, SiftChannel};
use sift_rs::{
    common::r#type::v1::ChannelDataType,
    ingest::v1::{
        ingest_service_server::{IngestService, IngestServiceServer},
        IngestArbitraryProtobufDataStreamRequest, IngestArbitraryProtobufDataStreamResponse,
        IngestWithConfigDataStreamRequest, IngestWithConfigDataStreamResponse,
    },
    ingestion_configs::v2::{ChannelConfig, FlowConfig, IngestionConfig},
};
use sift_stream::{ChannelValue, IngestionConfigMode, Message, SiftStream, TimeValue};
use std::{
    io::Error as IoError,
    sync::{
        atomic::{AtomicU32, Ordering},
        Arc,
    },
    time::Duration,
};
use tokio::{
    sync::oneshot::{self, error::TryRecvError},
    task::JoinHandle,
};
use tonic::{
    transport::{Endpoint, Server, Uri},
    Request, Response, Status, Streaming,
};
use tower::{service_fn, ServiceBuilder};

async fn start_test_server(
    num_message_received: Arc<AtomicU32>,
    num_checkpoints: Arc<AtomicU32>,
) -> (SiftChannel, JoinHandle<()>) {
    let (client, server) = tokio::io::duplex(1024);

    let ingest_service = IngestServiceNull {
        num_checkpoints,
        num_message_received,
    };

    let server = tokio::spawn(async move {
        Server::builder()
            .add_service(IngestServiceServer::new(ingest_service))
            .serve_with_incoming(tokio_stream::once(Ok::<_, IoError>(server)))
            .await
            .unwrap();
    });

    let mut client = Some(client);
    let channel = Endpoint::try_from("http://[::]:50051")
        .unwrap()
        .connect_with_connector(service_fn(move |_: Uri| {
            let client = client.take();

            async move {
                if let Some(client) = client {
                    Ok(TokioIo::new(client))
                } else {
                    Err(std::io::Error::other("Client already taken"))
                }
            }
        }))
        .await
        .unwrap();

    let sift_channel: SiftChannel = ServiceBuilder::new()
        .layer(tonic::service::interceptor(AuthInterceptor {
            apikey: "apikey".to_string(),
        }))
        .service(channel);

    (sift_channel, server)
}

struct IngestServiceNull {
    num_message_received: Arc<AtomicU32>,
    num_checkpoints: Arc<AtomicU32>,
}

#[async_trait]
impl IngestService for IngestServiceNull {
    async fn ingest_with_config_data_stream(
        &self,
        request: Request<Streaming<IngestWithConfigDataStreamRequest>>,
    ) -> Result<Response<IngestWithConfigDataStreamResponse>, tonic::Status> {
        let mut data_stream = request.into_inner();

        loop {
            match data_stream.message().await {
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
        unimplemented!()
    }
}

#[tokio::test]
async fn test_sending_data() {
    let num_checkpoints = Arc::new(AtomicU32::new(0));
    let messages_received = Arc::new(AtomicU32::new(0));

    let (client, server) =
        start_test_server(messages_received.clone(), num_checkpoints.clone()).await;

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
    );

    let num_messages = 100;

    for _ in 0..num_messages {
        let send_result = sift_stream.send(Message::new(
            "wheel",
            TimeValue::from(Local::now().to_utc()),
            &vec![
                ChannelValue::new("angular_velocity", 1.0_f64),
                ChannelValue::new("log", "foobar"),
            ],
        ));
        assert!(send_result.is_ok(), "streaming failed unexpectedly");
    }

    assert!(sift_stream.finish().await.is_ok());

    let _terminate_server = server.await;

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
}

#[tokio::test]
async fn test_checkpointing() {
    let num_checkpoints = Arc::new(AtomicU32::new(0));
    let messages_received = Arc::new(AtomicU32::new(0));

    let (client, _) = start_test_server(messages_received.clone(), num_checkpoints.clone()).await;

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
        Duration::from_secs(1),
    );

    let (terminate_streaming_tx, mut terminate_streaming_rx) = oneshot::channel::<()>();

    let streaming_task = tokio::task::spawn(async move {
        let mut messages_sent: u32 = 0;

        while let Err(TryRecvError::Empty) = terminate_streaming_rx.try_recv() {
            let timestamp = TimeValue::from(Local::now().to_utc());

            let send_result = sift_stream.send(Message::new(
                "flow",
                timestamp,
                &vec![ChannelValue::new("generator", 1.0_f64)],
            ));
            assert!(send_result.is_ok(), "streaming failed unexpectedly");
            messages_sent += 1;

            tokio::time::sleep(Duration::from_millis(100)).await;
        }

        assert!(sift_stream.finish().await.is_ok());

        messages_sent
    });

    tokio::time::sleep(Duration::from_millis(3_500)).await;
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
}
