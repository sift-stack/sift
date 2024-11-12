#[allow(clippy::all)]
/// Protobuf generated code to interface with Sift's gRPC API.
pub mod gen;

/// Preconfigured gRPC utilities to initialize client stubs from the `gen` package.
pub mod grpc;

/// Error types specific for this library. Note that when using the `gen` module
/// errors may occur that are not accounted for in this module.
pub mod error;
pub use error::Result;
