use hyper_util::rt::TokioIo;
use sift_connect::grpc::AuthInterceptor;
use std::io::Error;
use tonic::{service::InterceptorLayer, transport::{Endpoint, Uri}};
use tower::{ServiceBuilder, service_fn};


/// A test showing how to use [memory_sift_channel].
#[cfg(test)]
mod test;

/// In-memory channel for testing.
pub async fn memory_sift_channel(
    client: tokio::io::DuplexStream
) -> sift_connect::SiftChannel {
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

    ServiceBuilder::new()
        .layer(InterceptorLayer::new(AuthInterceptor {
            apikey: "sift-api-key".into(),
        }))
        .service(channel)
}
