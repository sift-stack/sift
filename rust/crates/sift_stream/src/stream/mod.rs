use crate::metrics::SiftStreamMetrics;
use sift_connect::SiftChannel;
use sift_rs::runs::v2::Run;
use std::sync::Arc;

#[cfg(feature = "metrics-unstable")]
use crate::metrics::SiftStreamMetricsSnapshot;

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

/// Concerned with accessing or creating runs for [SiftStream]
pub mod run;

/// Concerned with constructing values of time that make up the time-series sent ot Sift.
pub mod time;

/// Concerned with validating flows and detecting if changes are being made to an ingestion config
/// in a manner that isn't backwards compatible.
pub(crate) mod flow;

#[cfg(test)]
mod test;

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
    metrics: Arc<SiftStreamMetrics>,
}

impl<M: SiftStreamMode> SiftStream<M> {
    #[cfg(feature = "metrics-unstable")]
    /// Retrieve a snapshot of the current metrics for this stream.
    pub fn metrics(&self) -> SiftStreamMetricsSnapshot {
        self.metrics.snapshot()
    }
}

/// A trait that defines a particular mode of streaming. Only one more is currently supported.
pub trait SiftStreamMode {}

impl SiftStream<IngestionConfigMode> {
    /// Retrieves the attached run if if it exists.
    pub fn run(&self) -> Option<&Run> {
        self.mode.run.as_ref()
    }
}
