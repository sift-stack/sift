#[allow(clippy::all)]
/// Protobuf generated code to interface with Sift's gRPC API.
pub mod gen;

/// Preconfigured gRPC utilities to initialize client stubs from the `gen` package.
///
/// ```
/// use sift_rs::{
///     gen::sift::ping::v1::{ping_service_client::PingServiceClient, PingRequest},
///     grpc::{use_sift_channel, SiftChannelConfig},
/// };
/// use std::{env, error::Error};
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn Error>> {
///     let uri = env::var("SIFT_URI")?;
///     let apikey = env::var("SIFT_API_KEY")?;
///     let grpc_channel = use_sift_channel(SiftChannelConfig { uri, apikey })?;
///     let response_from_sift = PingServiceClient::new(grpc_channel)
///         .ping(PingRequest {})
///         .await?;
///     println!("{}", response_from_sift.get_ref().response);
///     Ok(())
/// }
/// ```
pub mod grpc;

/// Error types specific for this library. Note that when using the `gen` module
/// errors may occur that are not accounted for in this module.
pub mod error;
pub use error::Result;
