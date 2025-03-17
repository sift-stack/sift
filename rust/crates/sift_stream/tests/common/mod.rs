use hyper_util::rt::TokioIo;
use sift_connect::{grpc::interceptor::AuthInterceptor, SiftChannel};
use sift_rs::ingest::v1::ingest_service_server::{IngestService, IngestServiceServer};
use std::io::Error as IoError;
use tokio::task::JoinHandle;
use tonic::transport::{Endpoint, Server, Uri};
use tower::{service_fn, ServiceBuilder};

/// re-exports everything needed to implement an [IngestService].
pub mod prelude;

pub async fn start_test_ingest_server<I: IngestService>(
    ingest_service: I,
) -> (SiftChannel, JoinHandle<()>) {
    let (client, server) = tokio::io::duplex(1024);

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

/// All tests are run on a single-threaded run-time so we'll need to yield
/// if we want to give our mock server time to do some processing after sending it some data.
pub async fn task_yield() {
    tokio::task::yield_now().await;
}
