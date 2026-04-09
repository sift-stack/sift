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

/// Error types returned by [`Transport`] send methods.
pub mod send_error;
pub use send_error::{SendError, SiftStreamSendError, SiftStreamTrySendError, TrySendError};

#[cfg(test)]
mod test;

/// Provides a point-in-time snapshot of stream metrics.
///
/// Implemented by [`IngestionConfigEncoder`](crate::IngestionConfigEncoder). Snapshots are
/// non-blocking and do not affect stream operation. Obtain one via
/// [`SiftStream::get_metrics_snapshot`].
pub trait MetricsSnapshot: private::Sealed {
    fn snapshot(&self) -> SiftStreamMetricsSnapshot;
}

/// Implemented by types that can be encoded and sent via [`SiftStream::send`].
///
/// The two concrete implementations are [`Flow`](crate::mode::ingestion_config::Flow) and
/// [`FlowBuilder`](crate::flow::FlowBuilder). The associated `Encoder` type links each
/// encodeable to the specific encoder implementation that processes it — external types cannot
/// implement this trait because `Encoder` is sealed.
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

/// Defines how encoded telemetry messages are delivered to their destination.
///
/// Three concrete implementations are provided:
///
/// - [`LiveStreamingOnly`](crate::LiveStreamingOnly) — delivers messages to Sift in real-time
///   over a single bounded ingestion channel. No checkpointing, no disk backups.
/// - [`LiveStreamingWithBackups`](crate::LiveStreamingWithBackups) — delivers messages to Sift
///   in real-time with periodic checkpointing and disk backups. Uses a dual-channel
///   architecture; see below.
/// - [`FileBackup`](crate::FileBackup) — writes messages to rolling disk files without
///   streaming live to Sift.
///
/// ## Send API
///
/// Each implementation exposes four send methods that differ in their backpressure behaviour:
///
/// | Method | Blocks? | Error on failure |
/// |---|---|---|
/// | [`send`](Transport::send) | Yes — awaits until the channel has capacity | [`SendError<T>`] with the undelivered message |
/// | [`send_requests`](Transport::send_requests) | Yes — per-message backpressure | [`SendError<Vec<T>>`] with all undelivered messages |
/// | [`try_send`](Transport::try_send) | No — returns immediately | [`TrySendError<T>`] as `Full(T)` or `Closed(T)` |
/// | [`try_send_requests`](Transport::try_send_requests) | No — fails on first undeliverable message | [`TrySendError<Vec<T>>`] with all undelivered |
///
/// In every failure case the undelivered message(s) are returned inside the error variant so
/// that the caller can decide whether to retry, log, buffer locally, or discard them.
///
/// ## Backpressure sources
///
/// The channel that applies backpressure to [`send`](Transport::send) differs per mode. Knowing
/// which channel to tune is important when adjusting capacity via the mode builders:
///
/// | Mode | [`send`](Transport::send) awaits on | Capacity setting |
/// |---|---|---|
/// | [`LiveStreamingOnly`](crate::LiveStreamingOnly) | ingestion channel | [`ingestion_data_channel_capacity`](crate::LiveOnlyBuilder::ingestion_data_channel_capacity) |
/// | [`LiveStreamingWithBackups`](crate::LiveStreamingWithBackups) | backup channel only — ingestion uses force-send | [`backup_data_channel_capacity`](crate::LiveWithBackupsBuilder::backup_data_channel_capacity) |
/// | [`FileBackup`](crate::FileBackup) | write channel | [`backup_data_channel_capacity`](crate::FileBackupBuilder::backup_data_channel_capacity) |
///
/// ## Channel semantics for `LiveStreamingWithBackups`
///
/// `LiveStreamingWithBackups` maintains two internal bounded channels:
///
/// - **backup channel** — the primary durability path. [`send`](Transport::send) awaits here.
/// - **ingestion channel** — forwards messages to the gRPC task using a *force-send* strategy:
///   when full, the **oldest buffered message is evicted** to make room for the incoming one.
///   Evicted messages are redirected to the backup channel.
///
/// Because of force-send eviction, the message returned inside an error variant from
/// [`send`](Transport::send) or [`send_requests`](Transport::send_requests) may be an **older
/// displaced message**, not necessarily the one you just sent.
///
/// This trait is sealed: only implementations within this crate are permitted.
#[async_trait]
pub trait Transport: private::Sealed {
    type Message: Send + Sync;
    type Encoder: Encoder<Message = Self::Message>;

    /// Send a single message with backpressure.
    ///
    /// Awaits until the backing channel has capacity, then delivers the message.
    ///
    /// # Errors
    ///
    /// Returns [`SendError<Self::Message>`] containing a potentially undelivered message.
    ///
    /// Depending on the implementation of [`Transport`], the undelivered message is not
    /// necessarily the message that was provided to the current invocation of [`Self::send`].
    ///
    /// See implementation documentation for details.
    async fn send(
        &mut self,
        stream_id: &Uuid,
        message: Self::Message,
    ) -> std::result::Result<(), SendError<Self::Message>>;

    /// Send a batch of messages with backpressure.
    ///
    /// Awaits channel capacity for each message in turn. Stops on the first failure and
    /// returns the failed message together with all remaining (not-yet-attempted) messages.
    ///
    /// # Errors
    ///
    /// Returns [`SendError<Vec<Self::Message>>`] containing potentially undelivered messages.
    ///
    /// Depending on the implementation of [`Transport`], the undelivered messages are not
    /// necessarily the messages that were provided to the current invocation of [`Self::send_requests`].
    ///
    /// See implementation documentation for details.
    async fn send_requests<I>(
        &mut self,
        stream_id: &Uuid,
        requests: I,
    ) -> std::result::Result<(), SendError<Vec<Self::Message>>>
    where
        I: IntoIterator<Item = Self::Message> + Send,
        I::IntoIter: Send;

    /// Attempt to send a single message without blocking.
    ///
    /// Returns immediately regardless of whether the channel has capacity.
    ///
    /// # Errors
    ///
    /// Returns [`TrySendError<Self::Message>`] containing a potentially undelivered message:
    /// - [`TrySendError::Full`] — the channel is at capacity; consider retrying with
    ///   [`send`](Transport::send) to apply backpressure instead.
    /// - [`TrySendError::Closed`] — the channel has been closed.
    ///
    /// Depending on the implementation of [`Transport`], the undelivered messages are not
    /// necessarily the messages that were provided to the current invocation of [`Self::try_send`].
    ///
    /// See implementation documentation for details.
    fn try_send(
        &mut self,
        stream_id: &Uuid,
        message: Self::Message,
    ) -> std::result::Result<(), TrySendError<Self::Message>>;

    /// Attempt to send a batch of messages without blocking.
    ///
    /// Calls [`try_send`](Transport::try_send) for each message in turn. Returns immediately
    /// on the first failure, bundling the failed message with any remaining unprocessed
    /// messages.
    ///
    /// # Errors
    ///
    /// Returns [`TrySendError<Vec<Self::Message>>`] containing potentially undelivered messages.
    /// - [`TrySendError::Full`] — the channel was at capacity for one of the messages.
    /// - [`TrySendError::Closed`] — the channel was closed.
    ///
    /// Depending on the implementation of [`Transport`], the undelivered messages are not
    /// necessarily the messages that were provided to the current invocation of [`Self::try_send_requests`].
    ///
    /// See implementation documentation for details.
    fn try_send_requests<I>(
        &mut self,
        stream_id: &Uuid,
        requests: I,
    ) -> std::result::Result<(), TrySendError<Vec<Self::Message>>>
    where
        I: IntoIterator<Item = Self::Message> + Send,
        I::IntoIter: Send;

    /// Flush any remaining messages and cleanly shut down the transport.
    ///
    /// Must be called when ingestion is complete. Dropping a [`SiftStream`] without
    /// calling `finish` may result in tail-end data not reaching Sift.
    async fn finish(self, stream_id: &Uuid) -> Result<()>;
}

/// Generic wrapper over a telemetry transport that provides a consistent send API regardless
/// of the underlying mode.
///
/// `E` is the encoder (e.g. [`IngestionConfigEncoder`](crate::IngestionConfigEncoder)) and `T`
/// is the transport (e.g. [`LiveStreamingOnly`](crate::LiveStreamingOnly),
/// [`LiveStreamingWithBackups`](crate::LiveStreamingWithBackups), or
/// [`FileBackup`](crate::FileBackup)). The available features — checkpointing, retry, disk
/// backups — depend entirely on the transport mode chosen at build time.
///
/// Construct a `SiftStream` via [`SiftStreamBuilder`](builder::SiftStreamBuilder). Refer to the
/// [crate-level documentation](crate) for mode comparison, examples, and tuning guidance.
pub struct SiftStream<E, T> {
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

    /// Send telemetry with backpressure.
    ///
    /// Encodes `message` and then awaits until the backing channel has capacity. See the
    /// [`Transport`] implementation for specific details on backpressure.
    ///
    /// Use this method when you want the caller to slow down naturally when the pipeline
    /// is under load. For a non-blocking alternative see [`try_send`](SiftStream::try_send).
    ///
    /// # Errors
    ///
    /// - [`SiftStreamSendError::EncodeError`] — the message could not be encoded. This
    ///   indicates a schema mismatch or invalid value and is not recoverable by retrying.
    /// - [`SiftStreamSendError::ChannelClosed`] — the backing channel was closed before the
    ///   message could be delivered. The undelivered message is returned inside the variant.
    ///
    /// # Cancellation safety
    ///
    /// If the returned future is dropped while waiting for channel capacity, no message is
    /// lost — either the send completed before the drop, or the channel slot was never taken.
    pub async fn send<M>(
        &mut self,
        message: M,
    ) -> std::result::Result<(), SiftStreamSendError<<T as Transport>::Message>>
    where
        M: Encodeable<Encoder = E, Output = <T as Transport>::Message> + Send + Sync,
    {
        let encoded = message
            .encode(&mut self.encoder, &self.sift_stream_id, self.run.as_ref())
            .ok_or_else(|| SiftStreamSendError::encode_error("Failed to encode message"))?;

        self.transport
            .send(&self.sift_stream_id, encoded)
            .await
            .map_err(|SendError(msg)| SiftStreamSendError::ChannelClosed(msg))
    }

    /// Send a batch of pre-encoded requests with backpressure.
    ///
    /// Awaits channel capacity for each request in turn. Stops on the first failure and
    /// returns all undelivered messages (the failing one plus any not yet attempted).
    ///
    /// Unlike [`send`](SiftStream::send), this method accepts pre-encoded
    /// [`Transport::Message`](crate::stream::Transport::Message) values directly, bypassing
    /// the encode step. Use [`FlowBuilder`](crate::FlowBuilder) to construct them for maximum
    /// performance.
    ///
    /// # Errors
    ///
    /// [`SendError<Vec<T>>`] containing every message that was not delivered.
    pub async fn send_requests<I>(
        &mut self,
        requests: I,
    ) -> std::result::Result<(), SendError<Vec<<T as Transport>::Message>>>
    where
        I: IntoIterator<Item = <T as Transport>::Message> + Send,
        I::IntoIter: Send,
    {
        self.transport
            .send_requests(&self.sift_stream_id, requests)
            .await
    }

    /// Attempt to send telemetry without blocking.
    ///
    /// Encodes `message` and immediately attempts to place it on the backing channel. Returns
    /// at once regardless of whether the channel has capacity.
    ///
    /// Use this method in tight loops or real-time contexts where blocking is unacceptable.
    /// For backpressure-aware sending see [`send`](SiftStream::send).
    ///
    /// # Errors
    ///
    /// - [`SiftStreamTrySendError::EncodeError`] — the message could not be encoded.
    /// - [`SiftStreamTrySendError::Channel`] wrapping one of:
    ///   - [`TrySendError::Full`] — the backing channel is at capacity; the undelivered
    ///     message is returned. Consider switching to [`send`](SiftStream::send) to apply
    ///     backpressure, or retrying after a short delay.
    ///   - [`TrySendError::Closed`] — the backing channel has been closed; the undelivered
    ///     message is returned.
    pub fn try_send<M>(
        &mut self,
        message: M,
    ) -> std::result::Result<(), SiftStreamTrySendError<<T as Transport>::Message>>
    where
        M: Encodeable<Encoder = E, Output = <T as Transport>::Message> + Send + Sync,
    {
        let encoded = message
            .encode(&mut self.encoder, &self.sift_stream_id, self.run.as_ref())
            .ok_or_else(|| SiftStreamTrySendError::encode_error("Failed to encode message"))?;

        self.transport
            .try_send(&self.sift_stream_id, encoded)
            .map_err(SiftStreamTrySendError::Channel)
    }

    /// Attempt to send a batch of pre-encoded requests without blocking.
    ///
    /// Calls `try_send` on the backing channel for each request. Returns immediately on
    /// the first failure with every undelivered message (the failing one plus any not yet
    /// attempted).
    ///
    /// Unlike [`try_send`](SiftStream::try_send), this method accepts pre-encoded
    /// [`Transport::Message`](crate::stream::Transport::Message) values directly. Use
    /// [`FlowBuilder`](crate::FlowBuilder) to construct them for maximum performance.
    ///
    /// # Errors
    ///
    /// [`TrySendError<Vec<T>>`] containing every message that was not delivered:
    /// - [`TrySendError::Full`] — the backing channel was at capacity.
    /// - [`TrySendError::Closed`] — the backing channel was closed.
    pub fn try_send_requests<I>(
        &mut self,
        requests: I,
    ) -> std::result::Result<(), TrySendError<Vec<<T as Transport>::Message>>>
    where
        I: IntoIterator<Item = <T as Transport>::Message> + Send,
        I::IntoIter: Send,
    {
        self.transport
            .try_send_requests(&self.sift_stream_id, requests)
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
