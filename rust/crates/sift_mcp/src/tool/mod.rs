use rmcp::model::CallToolResult;

pub mod resource;

pub use tonic::transport::Body as TransportBody;

/// gRPC in-memory duplex channel for allow client-server communication during testing.
#[cfg(test)]
async fn memory_grpc_channel(
    client: tokio::io::DuplexStream
) -> tonic::transport::Channel {
    use hyper_util::rt::TokioIo;
    use std::io::Error;
    use tonic::transport::{Endpoint, Uri};
    use tower::service_fn;

    // This is awkward but it's done to satisfy the `service_fn` FnMut requirement.
    let mut client = Some(client);

    Endpoint::try_from("http://[::]:50051")
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
        .expect("failed to create gRPC memory channel")
}
