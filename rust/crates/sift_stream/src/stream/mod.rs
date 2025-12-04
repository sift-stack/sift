use crate::stream::mode::ingestion_config::LiveStreaming;
use crate::stream::run::{RunSelector, load_run_by_form, load_run_by_id};
use async_trait::async_trait;
use sift_connect::SiftChannel;
use sift_error::prelude::*;
use sift_rs::runs::v2::Run;
use uuid::Uuid;

use crate::metrics::SiftStreamMetricsSnapshot;

/// Concerned with building and configuring and instance of [SiftStream].
pub mod builder;

/// Concerned with constructing values for channels/sensors that get telemetered.
pub mod channel;

/// Shared helper functions used across stream implementations.
mod helpers;

/// Implementations for different modes of streaming.
pub mod mode;

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

/// Task-based architecture for non-blocking SiftStream operations
pub mod tasks;

#[cfg(test)]
mod test;

/// A trait that how metrics are accessed.
pub trait MetricsSnapshot: private::Sealed {
    fn snapshot(&self) -> SiftStreamMetricsSnapshot;
}

pub trait Encodeable {
    type Output: Send + Sync;
    type Encoder: Encoder<Message = Self::Output>;

    fn encode(
        self,
        encoder: &mut Self::Encoder,
        stream_id: &Uuid,
        run: Option<&Run>,
    ) -> Option<Self::Output>;
}

/// A trait that indicates that a type can be encoded by it.
///
/// This trait is used to tie an [`Encoder`] to the [`Encodeable`]s that
/// it can encode.
pub trait Encoder: private::Sealed {
    type Message: Send + Sync;
}

/// A trait that defines how data is transmitted, or streamed.
///
/// For example, a live streaming implementation might use a
/// gRPC stream to transmit data in real-time to Sift, while
/// an alternative implementation might write data to a file
/// for a more "offline" use-case.
#[async_trait]
pub trait Transport: private::Sealed {
    type Message: Send + Sync;
    type Encoder: Encoder<Message = Self::Message>;

    /// Send a [`Self::Message`].
    fn send(&mut self, stream_id: &Uuid, message: Self::Message) -> Result<()>;

    /// Send a batch of messages via an iterator.
    ///
    /// This method is used as a more performant way to send a batch of messages, assuming
    /// the iterator itself is not performing substantial work.
    ///
    /// However, this is less convenient since the caller will need to ensure the
    /// resulting [`Self::Message`]s are properly created.
    fn send_requests<I>(&mut self, stream_id: &Uuid, requests: I) -> Result<()>
    where
        I: IntoIterator<Item = Self::Message> + Send,
        I::IntoIter: Send;

    /// Finish the stream. The mode implementation handles the actual cleanup logic.
    async fn finish(self, stream_id: &Uuid) -> Result<()>;
}

/// [SiftStream] is a smart wrapper over an actual gRPC stream that makes it robust and more
/// ergonomic to work with. Some additional behaviors that [SiftStream] supports are:
/// - Checkpointing
/// - Retries (disabled by default)
/// - Backups (disabled by default)
/// - Tracing and ingestion metrics
///
/// To initialize a [SiftStream] users will use [builder::SiftStreamBuilder]. Refer to the
/// [crate-level documentation](crate) for further details and examples.
pub struct SiftStream<E, T = LiveStreaming> {
    grpc_channel: SiftChannel,
    encoder: E,
    transport: T,
    run: Option<Run>,
    sift_stream_id: Uuid,
}

impl<E, T> SiftStream<E, T>
where
    E: Encoder + MetricsSnapshot,
    T: Transport<Encoder = E>,
{
    #[cfg(feature = "metrics-unstable")]
    /// Retrieve a snapshot of the current metrics for this stream.
    pub fn get_metrics_snapshot(&self) -> SiftStreamMetricsSnapshot {
        self.encoder.snapshot()
    }

    /// Attach a run to the stream. Any data provided through [SiftStream::send] after return
    /// of this function will be associated with the run.
    pub async fn attach_run(&mut self, run_selector: RunSelector) -> Result<()> {
        let run = match run_selector {
            RunSelector::ById(run_id) => load_run_by_id(self.grpc_channel.clone(), &run_id).await?,
            RunSelector::ByForm(run_form) => {
                load_run_by_form(self.grpc_channel.clone(), run_form).await?
            }
        };

        self.run = Some(run);

        Ok(())
    }

    /// Detach the run, if any, associated with the stream. Any data provided through [SiftStream::send] after
    /// this function is called will not be associated with a run.
    pub fn detach_run(&mut self) {
        self.run = None;
    }

    /// Retrieves the attached run if it exists.
    pub fn run(&self) -> Option<&Run> {
        self.run.as_ref()
    }

    /// The entry-point to send actual telemetry to Sift in the form of [Flow]s.
    pub async fn send<M>(&mut self, message: M) -> Result<()>
    where
        M: Encodeable<Encoder = E, Output = <T as Transport>::Message> + Send + Sync,
    {
        let encoded = message
            .encode(&mut self.encoder, &self.sift_stream_id, self.run.as_ref())
            .ok_or(Error::new_msg(
                ErrorKind::EncodeMessageError,
                "Failed to encode message",
            ))?;

        self.transport.send(&self.sift_stream_id, encoded)
    }

    /// This method offers a way to send data in a manner that's identical to the raw
    /// [`gRPC service`] for ingestion-config based streaming.
    ///
    /// [`gRPC service`]: https://github.com/sift-stack/sift/blob/main/protos/sift/ingest/v1/ingest.proto#L11
    pub async fn send_requests<I>(&mut self, requests: I) -> Result<()>
    where
        I: IntoIterator<Item = <T as Transport>::Message> + Send,
        I::IntoIter: Send,
    {
        self.transport.send_requests(&self.sift_stream_id, requests)
    }

    /// This method offers a way to send data in a manner that's identical to the raw
    /// [`gRPC service`] for ingestion-config based streaming.
    ///
    /// [`gRPC service`]: https://github.com/sift-stack/sift/blob/main/protos/sift/ingest/v1/ingest.proto#L11
    pub fn send_requests_nonblocking<I>(&mut self, requests: I) -> Result<()>
    where
        I: IntoIterator<Item = <T as Transport>::Message> + Send,
        I::IntoIter: Send,
    {
        self.transport.send_requests(&self.sift_stream_id, requests)
    }

    /// Gracefully finish the stream, draining any remaining data before returning.
    ///
    /// It is important to always call this method when you are done sending data and
    /// before the object is dropped.
    pub async fn finish(self) -> Result<()> {
        self.transport.finish(&self.sift_stream_id).await
    }
}

impl<E, T> std::ops::Deref for SiftStream<E, T>
where
    E: Encoder + MetricsSnapshot,
    T: Transport<Encoder = E>,
{
    type Target = E;
    fn deref(&self) -> &Self::Target {
        &self.encoder
    }
}

impl<E, T> std::ops::DerefMut for SiftStream<E, T>
where
    E: Encoder + MetricsSnapshot,
    T: Transport<Encoder = E>,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.encoder
    }
}

/// Sealed trait to prevent external implementations of `SiftStreamMode`.
mod private {
    /// This trait is sealed and cannot be implemented outside this crate.
    ///
    /// It is public so it can be used as a supertrait, but the module is private,
    /// preventing external code from implementing it.
    pub trait Sealed {}
}
