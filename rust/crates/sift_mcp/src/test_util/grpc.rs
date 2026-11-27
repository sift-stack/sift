use crate::grpc::{GrpcChannel, grpc_channel};
use hyper_util::rt::TokioIo;
use std::io::Error;
use tonic::transport::{Endpoint, Uri};
use tower::service_fn;

/// Returns a channel that talks to a mock server over `client`, one half of a
/// `tokio::io::duplex` pair. Serve the mock on the other half.
pub async fn memory_grpc_channel(client: tokio::io::DuplexStream) -> GrpcChannel {
    let mut client = Some(client);

    let channel = Endpoint::try_from("http://[::]:50051")
        .unwrap()
        .connect_with_connector(service_fn(move |_: Uri| {
            let client = client.take();

            async move {
                if let Some(client) = client {
                    Ok(TokioIo::new(client))
                } else {
                    Err(Error::other("Client already taken"))
                }
            }
        }))
        .await
        .expect("failed to create gRPC memory channel");

    grpc_channel(channel)
}
