use sift_connect::SiftChannel;

pub mod builder;

pub mod channel;

/// Implementations for streaming modes.
pub mod mode;

/// Retry policy
pub mod retry;
pub use retry::RetryPolicy;

pub mod time;

/// Concerned with validating flows and detecting if changes are being made to an ingestion config
/// in a manner that isn't backwards compatible.
pub(crate) mod flow;

pub struct SiftStream<M: SiftStreamMode> {
    grpc_channel: SiftChannel,
    mode: M,
}

pub trait SiftStreamMode {}
