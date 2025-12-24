//! `sift_rs` largely contains protobuf-generated code used to interact with Sift's gRPC API as well as some
//! utility wrappers. Due to [tonic](https://docs.rs/tonic/latest/tonic/) relying on
//! [tokio](https://docs.rs/tokio/latest/tokio/), it is required that users have `tokio` installed
//! in order to communicate with Sift over gRPC.
//!
//! ## Quickstart
//!
//! Start by installing `sift_rs`
//!
//! ```text
//! cargo add sift_rs
//! ```
//!
//! Then install `tokio`:
//!
//! ```text
//! cargo add tokio --features full
//! ```
//!
//! Then we can establish a connection to Sift and use one of the protobuf service clients like so:
//!
//! ```no_run
//! use sift_rs::{
//!     Credentials, SiftChannelBuilder,
//!     ping::v1::{PingRequest, ping_service_client::PingServiceClient},
//! };
//! use std::env;
//!
//! #[tokio::main]
//! async fn main() {
//!     let credentials = Credentials::Config {
//!         apikey: env::var("SIFT_TOKEN").unwrap(),
//!         uri: env::var("SIFT_GRPC").unwrap(),
//!     };
//!
//!     let conn = SiftChannelBuilder::new(credentials).build().unwrap();
//!     let mut ping_service = PingServiceClient::new(conn);
//!     let ping_response = ping_service.ping(PingRequest::default()).await.unwrap();
//!
//!     println!("{}", ping_response.into_inner().response);
//! }
//! ```

#[allow(clippy::all)]
/// Protobuf generated code to interface with Sift's gRPC API.
pub mod r#gen;
pub use r#gen::sift::*;

/// Utility wrappers for select gRPC services.
pub mod wrappers;

/// Generic retry extension for gRPC wrapper services.
pub mod retry;

pub use retry::{DefaultGrpcRetry, RetryConfig, RetryDecider, RetryExt, Retrying};
pub use sift_connect::{Credentials, SiftChannel, SiftChannelBuilder};
pub use tonic::codec::CompressionEncoding;
