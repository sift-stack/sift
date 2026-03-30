use std::fmt;

/// Returned by the async `Transport::send` / `Transport::send_requests` when
/// the underlying channel is closed and delivery cannot complete.
///
/// The inner value `T` is the undelivered message so the caller can decide
/// what to do with it (e.g. log, retry, discard).
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

/// Returned by the sync `Transport::try_send` / `Transport::try_send_requests`
/// when immediate delivery fails.
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
}
