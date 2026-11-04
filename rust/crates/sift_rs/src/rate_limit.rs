//! Detection of Sift rate-limit responses and extraction of `google.rpc.RetryInfo`.
//!
//! Sift rate limits a request by returning a gRPC `ResourceExhausted` status that carries a
//! `google.rpc.RetryInfo` detail. `ResourceExhausted` without `RetryInfo` has other causes, such
//! as a request that exceeds the server's message size limit, and is deliberately not treated as
//! a rate limit since retrying cannot help.
//!
//! ```
//! use sift_rs::rate_limit;
//! use sift_rs::GrpcStatus;
//! use sift_error::ErrorKind;
//!
//! fn handle(status: GrpcStatus) {
//!     let err = rate_limit::error_from_status(ErrorKind::CreateRunError, status);
//!     if let Some(delay) = err.retry_delay() {
//!         println!("rate limited; retry in {delay:?}");
//!     }
//! }
//! ```

use std::time::Duration;

use sift_error::prelude::*;
use tonic::{Code, Status};
use tonic_types::{RetryInfo, StatusExt};

/// Returns the `RetryInfo` if `status` is a Sift rate-limit response.
fn rate_limit_info(status: &Status) -> Option<RetryInfo> {
    if status.code() != Code::ResourceExhausted {
        return None;
    }
    status.get_details_retry_info()
}

/// Returns `true` if `status` is a rate-limit response from Sift: `ResourceExhausted` carrying a
/// `RetryInfo` detail.
pub fn is_rate_limited(status: &Status) -> bool {
    rate_limit_info(status).is_some()
}

/// Returns the server-provided retry delay if `status` is a rate-limit response that includes one.
pub fn retry_delay(status: &Status) -> Option<Duration> {
    rate_limit_info(status).and_then(|info| info.retry_delay)
}

/// Converts a `tonic::Status` into a [`sift_error::Error`].
///
/// Rate-limit responses become [`ErrorKind::RateLimitedError`] carrying the server's retry delay.
/// Everything else, including `ResourceExhausted` without `RetryInfo`, uses `kind`.
pub fn error_from_status(kind: ErrorKind, status: Status) -> Error {
    match rate_limit_info(&status) {
        Some(info) => Error::new_rate_limited(status, info.retry_delay),
        None => Error::new(kind, status),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tonic::codegen::Bytes;
    use tonic_types::ErrorDetails;

    fn rate_limited(retry_delay: Option<Duration>) -> Status {
        Status::with_error_details(
            Code::ResourceExhausted,
            "Rate limit exceeded",
            ErrorDetails::with_retry_info(retry_delay),
        )
    }

    #[test]
    fn resource_exhausted_with_retry_info_is_rate_limited() {
        let status = rate_limited(Some(Duration::from_secs(2)));

        assert!(is_rate_limited(&status));
        assert_eq!(retry_delay(&status), Some(Duration::from_secs(2)));

        let err = error_from_status(ErrorKind::CreateFlowError, status);
        assert_eq!(
            err.kind(),
            ErrorKind::RateLimitedError {
                retry_delay: Some(Duration::from_secs(2))
            }
        );
    }

    #[test]
    fn retry_info_without_delay_is_rate_limited() {
        let status = rate_limited(None);

        assert!(is_rate_limited(&status));
        assert_eq!(retry_delay(&status), None);

        let err = error_from_status(ErrorKind::CreateFlowError, status);
        assert_eq!(
            err.kind(),
            ErrorKind::RateLimitedError { retry_delay: None }
        );
    }

    #[test]
    fn resource_exhausted_without_retry_info_is_not_rate_limited() {
        // Mirrors the server's message size limit, which returns no details.
        let status =
            Status::resource_exhausted("grpc: received message larger than max for this method");

        assert!(!is_rate_limited(&status));
        assert_eq!(retry_delay(&status), None);

        let err = error_from_status(ErrorKind::CreateFlowError, status);
        assert_eq!(err.kind(), ErrorKind::CreateFlowError);
    }

    #[test]
    fn retry_info_on_other_codes_is_not_rate_limited() {
        let status = Status::with_error_details(
            Code::Unavailable,
            "unavailable",
            ErrorDetails::with_retry_info(Some(Duration::from_secs(1))),
        );

        assert!(!is_rate_limited(&status));
        assert_eq!(retry_delay(&status), None);

        let err = error_from_status(ErrorKind::RetrieveRunError, status);
        assert_eq!(err.kind(), ErrorKind::RetrieveRunError);
    }

    #[test]
    fn other_codes_keep_their_kind() {
        for (status, kind) in [
            (
                Status::unavailable("unavailable"),
                ErrorKind::RetrieveAssetError,
            ),
            (Status::not_found("not found"), ErrorKind::RetrieveRunError),
            (
                Status::invalid_argument("bad request"),
                ErrorKind::CreateIngestionConfigError,
            ),
        ] {
            assert!(!is_rate_limited(&status));
            assert_eq!(error_from_status(kind, status).kind(), kind);
        }
    }

    #[test]
    fn malformed_details_are_not_rate_limited() {
        let status = Status::with_details(
            Code::ResourceExhausted,
            "Rate limit exceeded",
            Bytes::from_static(b"\xff\xff"),
        );

        assert!(!is_rate_limited(&status));
        assert_eq!(retry_delay(&status), None);
        assert_eq!(
            error_from_status(ErrorKind::UpdateRunError, status).kind(),
            ErrorKind::UpdateRunError
        );
    }
}
