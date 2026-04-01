use sift_error::prelude::{Error as SiftError, ErrorKind};
use std::fmt;

/// Returned by the async [`Transport::send`](crate::stream::Transport::send) /
/// [`Transport::send_requests`](crate::stream::Transport::send_requests) when the underlying
/// channel is closed and delivery cannot complete.
///
/// The inner value `T` is the undelivered message. Typical recovery strategies:
///
/// - **Log and discard** — if losing the message is acceptable (e.g. high-frequency sensor
///   data where the next sample will arrive momentarily).
/// - **Buffer and retry** — store the message locally and re-attempt once the channel is
///   re-established.
/// - **Propagate as an error** — call [`into_inner`](SendError::into_inner) to recover the
///   message and return it up the call stack for the application to decide.
///
/// A closed channel usually means the [`SiftStream`](crate::SiftStream) is shutting down.
/// Check that [`SiftStream::finish`](crate::SiftStream::finish) has not already been called.
#[derive(Debug)]
pub struct SendError<T>(pub T);

impl<T> SendError<T> {
    /// Consume the error, returning the undelivered value.
    pub fn into_inner(self) -> T {
        self.0
    }
}

impl<T> fmt::Display for SendError<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "channel closed: failed to send message")
    }
}

impl<T: fmt::Debug> std::error::Error for SendError<T> {}

/// Returned by the sync [`Transport::try_send`](crate::stream::Transport::try_send) /
/// [`Transport::try_send_requests`](crate::stream::Transport::try_send_requests) when
/// immediate delivery fails.
///
/// The undelivered value `T` is always returned inside the variant so the caller can
/// recover it without cloning.
///
/// ## Variants and recovery
///
/// - [`Full`](TrySendError::Full) — the channel is currently at capacity. The message has
///   *not* been dropped; the caller can retry later with another `try_send` call, switch to
///   the backpressure-aware [`send`](crate::SiftStream::send), or discard the message if
///   it is stale.
///
/// - [`Closed`](TrySendError::Closed) — all channel receivers have been dropped. The
///   [`SiftStream`](crate::SiftStream) is shutting down. Retrying on the same stream will
///   not succeed.
#[derive(Debug)]
pub enum TrySendError<T> {
    /// The channel has been closed. The undelivered value is returned.
    Closed(T),
    /// The channel is currently full. The undelivered value is returned.
    Full(T),
}

impl<T> TrySendError<T> {
    /// Consume the error, returning the undelivered value.
    pub fn into_inner(self) -> T {
        match self {
            TrySendError::Closed(v) | TrySendError::Full(v) => v,
        }
    }

    pub fn is_closed(&self) -> bool {
        matches!(self, TrySendError::Closed(_))
    }

    pub fn is_full(&self) -> bool {
        matches!(self, TrySendError::Full(_))
    }
}

impl<T> fmt::Display for TrySendError<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TrySendError::Closed(_) => write!(f, "channel closed: failed to send message"),
            TrySendError::Full(_) => write!(f, "channel full: failed to send message"),
        }
    }
}

impl<T: fmt::Debug> std::error::Error for TrySendError<T> {}

/// Returned by [`SiftStream::send`](crate::SiftStream::send) when delivery fails.
///
/// This is the top-level error type for the high-level send API. It distinguishes
/// encode-time failures from channel-level failures so callers can handle them
/// differently.
#[derive(Debug)]
pub enum SiftStreamSendError<T> {
    /// The message could not be encoded before it was sent.
    ///
    /// This typically indicates a schema mismatch or an invalid value in the message.
    /// Retrying the same message without correcting the schema will not succeed.
    EncodeError(SiftError),

    /// The backing channel closed before the message could be delivered.
    ///
    /// The undelivered message is returned inside the variant. See the recovery
    /// guidance on [`SendError`] for options.
    ChannelClosed(T),
}

impl<T: fmt::Debug> fmt::Display for SiftStreamSendError<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SiftStreamSendError::EncodeError(e) => write!(f, "encode error: {e}"),
            SiftStreamSendError::ChannelClosed(_) => {
                write!(f, "channel closed: failed to send message")
            }
        }
    }
}

impl<T: fmt::Debug> std::error::Error for SiftStreamSendError<T> {}

impl<T> SiftStreamSendError<T> {
    /// Convert an encode failure into this error type. Used internally.
    pub(crate) fn encode_error(msg: &str) -> Self {
        SiftStreamSendError::EncodeError(SiftError::new_msg(ErrorKind::EncodeMessageError, msg))
    }
}

/// Returned by [`SiftStream::try_send`](crate::SiftStream::try_send) when immediate
/// delivery fails.
///
/// This is the top-level error type for the non-blocking send API. It distinguishes
/// encode-time failures from channel-level failures so callers can handle them
/// differently.
#[derive(Debug)]
pub enum SiftStreamTrySendError<T> {
    /// The message could not be encoded before it was sent.
    ///
    /// This typically indicates a schema mismatch or an invalid value in the message.
    /// Retrying the same message without correcting the schema will not succeed.
    EncodeError(SiftError),

    /// The backing channel was full or closed.
    ///
    /// The inner [`TrySendError`] carries the undelivered message. Inspect the variant
    /// to distinguish between a transient backpressure condition (`Full`) and a permanent
    /// shutdown (`Closed`). See [`TrySendError`] for recovery guidance.
    Channel(TrySendError<T>),
}

impl<T: fmt::Debug> fmt::Display for SiftStreamTrySendError<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SiftStreamTrySendError::EncodeError(e) => write!(f, "encode error: {e}"),
            SiftStreamTrySendError::Channel(e) => write!(f, "{e}"),
        }
    }
}

impl<T: fmt::Debug> std::error::Error for SiftStreamTrySendError<T> {}

impl<T> SiftStreamTrySendError<T> {
    /// Convert an encode failure into this error type. Used internally.
    pub(crate) fn encode_error(msg: &str) -> Self {
        SiftStreamTrySendError::EncodeError(SiftError::new_msg(ErrorKind::EncodeMessageError, msg))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn send_error_into_inner() {
        let err = SendError(42u32);
        assert_eq!(err.into_inner(), 42u32);
    }

    #[test]
    fn send_error_display() {
        let err = SendError("msg");
        assert!(err.to_string().contains("channel closed"));
    }

    #[test]
    fn send_error_is_error() {
        fn assert_error<E: std::error::Error>(_: &E) {}
        let err = SendError(0u8);
        assert_error(&err);
    }

    #[test]
    fn try_send_error_into_inner_closed() {
        let err: TrySendError<u32> = TrySendError::Closed(7);
        assert_eq!(err.into_inner(), 7);
    }

    #[test]
    fn try_send_error_into_inner_full() {
        let err: TrySendError<u32> = TrySendError::Full(9);
        assert_eq!(err.into_inner(), 9);
    }

    #[test]
    fn try_send_error_is_closed() {
        assert!(TrySendError::<u8>::Closed(0).is_closed());
        assert!(!TrySendError::<u8>::Full(0).is_closed());
    }

    #[test]
    fn try_send_error_is_full() {
        assert!(TrySendError::<u8>::Full(0).is_full());
        assert!(!TrySendError::<u8>::Closed(0).is_full());
    }

    #[test]
    fn try_send_error_display() {
        assert!(
            TrySendError::<u8>::Closed(0)
                .to_string()
                .contains("channel closed")
        );
        assert!(
            TrySendError::<u8>::Full(0)
                .to_string()
                .contains("channel full")
        );
    }

    #[test]
    fn try_send_error_is_error() {
        fn assert_error<E: std::error::Error>(_: &E) {}
        let err = TrySendError::Closed(0u8);
        assert_error(&err);
    }

    #[test]
    fn try_send_error_debug() {
        let closed = TrySendError::Closed(42u32);
        let full = TrySendError::Full(42u32);
        assert!(format!("{:?}", closed).contains("Closed"));
        assert!(format!("{:?}", full).contains("Full"));
    }

    // SiftStreamSendError tests

    #[test]
    fn sift_stream_send_error_encode_error_display() {
        let err = SiftStreamSendError::<u32>::encode_error("bad encoding");
        assert!(err.to_string().contains("encode error"));
    }

    #[test]
    fn sift_stream_send_error_channel_closed_display() {
        let err = SiftStreamSendError::ChannelClosed(42u32);
        assert!(err.to_string().contains("channel closed"));
    }

    #[test]
    fn sift_stream_send_error_is_error() {
        fn assert_error<E: std::error::Error>(_: &E) {}
        let err = SiftStreamSendError::ChannelClosed(0u8);
        assert_error(&err);
    }

    #[test]
    fn sift_stream_send_error_debug() {
        let err = SiftStreamSendError::ChannelClosed(42u32);
        assert!(format!("{:?}", err).contains("ChannelClosed"));
        let err2 = SiftStreamSendError::<u32>::encode_error("oops");
        assert!(format!("{:?}", err2).contains("EncodeError"));
    }

    // SiftStreamTrySendError tests

    #[test]
    fn sift_stream_try_send_error_encode_error_display() {
        let err = SiftStreamTrySendError::<u32>::encode_error("bad");
        assert!(err.to_string().contains("encode error"));
    }

    #[test]
    fn sift_stream_try_send_error_channel_full_display() {
        let err = SiftStreamTrySendError::Channel(TrySendError::Full(42u32));
        assert!(err.to_string().contains("channel full"));
    }

    #[test]
    fn sift_stream_try_send_error_channel_closed_display() {
        let err = SiftStreamTrySendError::Channel(TrySendError::Closed(42u32));
        assert!(err.to_string().contains("channel closed"));
    }

    #[test]
    fn sift_stream_try_send_error_is_error() {
        fn assert_error<E: std::error::Error>(_: &E) {}
        let err = SiftStreamTrySendError::Channel(TrySendError::Closed(0u8));
        assert_error(&err);
    }

    #[test]
    fn sift_stream_try_send_error_debug() {
        let err = SiftStreamTrySendError::Channel(TrySendError::Full(42u32));
        assert!(format!("{:?}", err).contains("Full"));
        let err2 = SiftStreamTrySendError::<u32>::encode_error("oops");
        assert!(format!("{:?}", err2).contains("EncodeError"));
    }
}
