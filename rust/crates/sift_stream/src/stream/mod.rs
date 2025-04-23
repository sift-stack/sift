use sift_connect::SiftChannel;
use sift_rs::runs::v2::Run;

/// Concerned with building and configuring and instance of [SiftStream].
pub mod builder;

/// Concerned with constructing values for channels/sensors that get telemetered.
pub mod channel;

/// Implementations for different modes of streaming.
pub mod mode;
use mode::ingestion_config::IngestionConfigMode;

/// Concerned with gRPC retries.
pub mod retry;
pub use retry::RetryPolicy;

/// Concerned with constructing values of time that make up the time-series sent ot Sift.
pub mod time;

/// Concerned with validating flows and detecting if changes are being made to an ingestion config
/// in a manner that isn't backwards compatible.
pub(crate) mod flow;

/// [SiftStream] is a smart wrapper over an actual gRPC stream that makes it robust and more
/// ergonomic to work with. Some additional behaviors that [SiftStream] supports are:
/// - Checkpointing
/// - Retries (disabled by default)
/// - Backups (disabled by default)
/// - Tracing and ingestion metrics
///
/// To initialize a [SiftStream] users will use [builder::SiftStreamBuilder]. Refer to the
/// [crate-level documentation](crate) for further details and examples.
pub struct SiftStream<M: SiftStreamMode> {
    grpc_channel: SiftChannel,
    mode: M,
}

/// A trait that defines a particular mode of streaming. Only one more is currently supported.
pub trait SiftStreamMode {}

impl SiftStream<IngestionConfigMode> {
    /// Retrieves the attached run if if it exists.
    pub fn run(&self) -> Option<&Run> {
        self.mode.run.as_ref()
    }
}
