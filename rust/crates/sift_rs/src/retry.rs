//! Generic retry extension for gRPC wrapper services.
//!
//! This module provides a retry mechanism that can be applied to any wrapper service
//! without modifying the wrapper traits themselves. The retry logic intelligently
//! extracts `tonic::Status` from `sift_error::Error` types to make retry decisions.
//!
//! ## Usage
//!
//! The retry mechanism uses a closure pattern to work around Rust's borrow checker
//! and the `&mut self` requirement of tonic clients. Each retry attempt clones the
//! wrapper and calls the closure, allowing the closure to use `&mut self` internally.
//!
//! ```no_run
//! use sift_rs::retry::{RetryExt, RetryConfig};
//! use sift_rs::wrappers::assets::new_asset_service;
//! use sift_rs::wrappers::assets::AssetServiceWrapper;
//! use std::time::Duration;
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! # let channel = todo!();
//! let wrapper = new_asset_service(channel);
//! let cfg = RetryConfig {
//!     max_attempts: 3,
//!     base_delay: Duration::from_millis(100),
//!     max_delay: Duration::from_secs(5),
//!     backoff_multiplier: 2.0,
//! };
//!
//! let svc = wrapper.retrying(cfg);
//! let asset = svc.call(|mut w| async move {
//!     w.try_get_asset_by_id("asset-123").await
//! }).await?;
//! # Ok(())
//! # }
//! ```
//!
//! ## Important Notes
//!
//! - **Idempotency**: Only use retries for idempotent operations. Non-idempotent
//!   operations may be executed multiple times if retries occur.
//! - **Streaming RPCs**: This retry mechanism does not support streaming RPCs.
//!   Streaming calls require recreating the stream and may have side effects.

use std::error::Error as StdError;
use std::future::Future;
use std::result::Result as StdResult;
use std::time::Duration;

use sift_error::prelude::*;
use tonic;

/// Configuration for retry behavior.
#[derive(Debug, Clone)]
pub struct RetryConfig {
    /// Maximum number of retry attempts (including the initial attempt).
    pub max_attempts: usize,
    /// Base delay for exponential backoff.
    pub base_delay: Duration,
    /// Maximum delay cap for exponential backoff.
    pub max_delay: Duration,
    /// Multiplier for exponential backoff (e.g., 2.0 for doubling each attempt).
    pub backoff_multiplier: f64,
}

impl Default for RetryConfig {
    /// Creates a default retry configuration with conservative settings:
    /// - 3 attempts total
    /// - 100ms base delay
    /// - 5s maximum delay
    /// - 2.0 backoff multiplier (exponential)
    fn default() -> Self {
        Self {
            max_attempts: 3,
            base_delay: Duration::from_millis(100),
            max_delay: Duration::from_secs(5),
            backoff_multiplier: 2.0,
        }
    }
}

impl RetryConfig {
    /// Calculates the backoff delay for a given attempt number.
    ///
    /// The delay is calculated as: `base_delay * (backoff_multiplier ^ (attempt - 1))`
    /// and is capped at `max_delay`.
    ///
    /// # Arguments
    ///
    /// * `attempt` - The attempt number (1-indexed). For attempt 1, returns `base_delay`.
    pub fn backoff(&self, attempt: usize) -> Duration {
        if attempt <= 1 {
            return self.base_delay;
        }

        let exponent = (attempt - 1) as f64;
        let delay_ms = self.base_delay.as_millis() as f64 * self.backoff_multiplier.powf(exponent);
        let delay = Duration::from_millis(delay_ms as u64);

        delay.min(self.max_delay)
    }
}

/// Trait for determining whether an error should trigger a retry.
pub trait RetryDecider<E> {
    /// Returns `true` if the error should trigger a retry attempt.
    fn should_retry(&self, err: &E) -> bool;
}

/// Default retry decider for gRPC errors wrapped in `sift_error::Error`.
///
/// This decider uses a two-strategy approach:
/// 1. First, attempts to extract `tonic::Status` from the error's source chain
/// 2. Falls back to `ErrorKind`-based heuristics if no `tonic::Status` is found
pub struct DefaultGrpcRetry;

impl RetryDecider<sift_error::Error> for DefaultGrpcRetry {
    fn should_retry(&self, err: &sift_error::Error) -> bool {
        // Strategy 1: Try to extract tonic::Status from error source chain
        let mut source = err.source();
        while let Some(err_ref) = source {
            if let Some(status) = err_ref.downcast_ref::<tonic::Status>() {
                return matches!(
                    status.code(),
                    tonic::Code::Unavailable
                        | tonic::Code::ResourceExhausted
                        | tonic::Code::DeadlineExceeded
                );
            }
            source = err_ref.source();
        }

        // Strategy 2: Fallback to ErrorKind-based heuristics
        matches!(
            err.kind(),
            ErrorKind::GrpcConnectError
                | ErrorKind::RetrieveAssetError
                | ErrorKind::RetrieveIngestionConfigError
                | ErrorKind::RetrieveRunError
        )
    }
}

/// Adapter that wraps a type and provides retry functionality.
#[derive(Clone, Debug)]
pub struct Retrying<T, D = DefaultGrpcRetry> {
    inner: T,
    cfg: RetryConfig,
    decider: D,
}

impl<T> Retrying<T> {
    /// Creates a new `Retrying` adapter with the default gRPC retry decider.
    pub fn new(inner: T, cfg: RetryConfig) -> Self {
        Self {
            inner,
            cfg,
            decider: DefaultGrpcRetry,
        }
    }
}

impl<T, D> Retrying<T, D> {
    /// Replaces the retry decider with a custom one.
    pub fn with_decider<D2>(self, decider: D2) -> Retrying<T, D2> {
        Retrying {
            inner: self.inner,
            cfg: self.cfg,
            decider,
        }
    }

    /// Returns a reference to the inner wrapped value.
    pub fn inner(&self) -> &T {
        &self.inner
    }
}

impl<T, D> Retrying<T, D>
where
    T: Clone,
{
    /// Executes a closure with retry logic.
    ///
    /// The closure is called up to `max_attempts` times. If it returns an error
    /// and the decider indicates the error is retryable, the function waits for
    /// the calculated backoff delay before retrying.
    ///
    /// # Arguments
    ///
    /// * `f` - A closure that takes a cloned wrapper and returns a future
    ///   that produces a `Result`. The closure can use `&mut self` internally
    ///   since each attempt gets a fresh clone.
    ///
    /// # Returns
    ///
    /// Returns `Ok(result)` if any attempt succeeds, or `Err(error)` if all
    /// attempts fail or the error is not retryable.
    pub async fn call<F, Fut, R, E>(&self, mut f: F) -> StdResult<R, E>
    where
        F: FnMut(T) -> Fut,
        Fut: Future<Output = StdResult<R, E>>,
        D: RetryDecider<E>,
    {
        let mut last_err = None;

        for attempt in 1..=self.cfg.max_attempts {
            let wrapper = self.inner.clone();
            match f(wrapper).await {
                Ok(result) => return Ok(result),
                Err(e) => {
                    last_err = Some(e);
                    if attempt < self.cfg.max_attempts
                        && self.decider.should_retry(last_err.as_ref().unwrap())
                    {
                        let delay = self.cfg.backoff(attempt);
                        tokio::time::sleep(delay).await;
                        continue;
                    }
                    break;
                }
            }
        }

        Err(last_err.expect("retry loop invariant violated"))
    }
}

/// Extension trait that provides the `.retrying()` method for any type.
pub trait RetryExt: Sized {
    /// Wraps `self` in a `Retrying` adapter with the given configuration.
    fn retrying(self, cfg: RetryConfig) -> Retrying<Self> {
        Retrying::new(self, cfg)
    }
}

impl<T> RetryExt for T {}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use std::sync::atomic::{AtomicUsize, Ordering};

    #[test]
    fn test_backoff_calculation() {
        let cfg = RetryConfig::default();

        // First attempt should return base delay
        assert_eq!(cfg.backoff(1), Duration::from_millis(100));

        // Second attempt: 100ms * 2^1 = 200ms
        assert_eq!(cfg.backoff(2), Duration::from_millis(200));

        // Third attempt: 100ms * 2^2 = 400ms
        assert_eq!(cfg.backoff(3), Duration::from_millis(400));

        // Fourth attempt: 100ms * 2^3 = 800ms
        assert_eq!(cfg.backoff(4), Duration::from_millis(800));
    }

    #[test]
    fn test_backoff_caps_at_max() {
        let cfg = RetryConfig {
            max_attempts: 10,
            base_delay: Duration::from_millis(100),
            max_delay: Duration::from_millis(500),
            backoff_multiplier: 2.0,
        };

        // Should cap at max_delay
        let delay = cfg.backoff(10);
        assert_eq!(delay, Duration::from_millis(500));
    }

    #[tokio::test]
    async fn test_retry_loop_succeeds_after_failures() {
        let counter = Arc::new(AtomicUsize::new(0));

        let cfg = RetryConfig {
            max_attempts: 3,
            base_delay: Duration::from_millis(10),
            max_delay: Duration::from_secs(5),
            backoff_multiplier: 2.0,
        };

        let retrying = Retrying::new((), cfg);

        let result = retrying
            .call(|_| {
                let counter = counter.clone();
                async move {
                    let attempts = counter.fetch_add(1, Ordering::SeqCst) + 1;
                    if attempts < 3 {
                        Err::<(), sift_error::Error>(Error::new_msg(
                            ErrorKind::RetrieveAssetError,
                            "temporary failure",
                        ))
                    } else {
                        Ok(())
                    }
                }
            })
            .await;

        assert!(result.is_ok());
        assert_eq!(counter.load(Ordering::SeqCst), 3);
    }

    #[tokio::test]
    async fn test_retry_loop_exhausts_attempts() {
        let counter = Arc::new(AtomicUsize::new(0));

        let cfg = RetryConfig {
            max_attempts: 3,
            base_delay: Duration::from_millis(10),
            max_delay: Duration::from_secs(5),
            backoff_multiplier: 2.0,
        };

        let retrying = Retrying::new((), cfg);

        let result = retrying
            .call(|_| {
                let counter = counter.clone();
                async move {
                    counter.fetch_add(1, Ordering::SeqCst);
                    Err::<(), sift_error::Error>(Error::new_msg(
                        ErrorKind::RetrieveAssetError,
                        "persistent failure",
                    ))
                }
            })
            .await;

        assert!(result.is_err());
        assert_eq!(counter.load(Ordering::SeqCst), 3);
    }

    #[test]
    fn test_default_grpc_retry_with_tonic_status() {
        let decider = DefaultGrpcRetry;

        // Test retryable status codes
        let unavailable = Error::new(
            ErrorKind::RetrieveAssetError,
            tonic::Status::unavailable("service unavailable"),
        );
        assert!(decider.should_retry(&unavailable));

        let resource_exhausted = Error::new(
            ErrorKind::RetrieveAssetError,
            tonic::Status::resource_exhausted("resource exhausted"),
        );
        assert!(decider.should_retry(&resource_exhausted));

        let deadline_exceeded = Error::new(
            ErrorKind::RetrieveAssetError,
            tonic::Status::deadline_exceeded("deadline exceeded"),
        );
        assert!(decider.should_retry(&deadline_exceeded));

        // Test non-retryable status codes
        let invalid_argument = Error::new(
            ErrorKind::ArgumentValidationError,
            tonic::Status::invalid_argument("invalid argument"),
        );
        assert!(!decider.should_retry(&invalid_argument));

        let not_found = Error::new(
            ErrorKind::NotFoundError,
            tonic::Status::not_found("not found"),
        );
        assert!(!decider.should_retry(&not_found));
    }

    #[test]
    fn test_default_grpc_retry_with_error_kind_fallback() {
        let decider = DefaultGrpcRetry;

        // Test retryable error kinds (without tonic::Status)
        let grpc_connect_error = Error::new_msg(ErrorKind::GrpcConnectError, "connection failed");
        assert!(decider.should_retry(&grpc_connect_error));

        let retrieve_asset_error =
            Error::new_msg(ErrorKind::RetrieveAssetError, "retrieval failed");
        assert!(decider.should_retry(&retrieve_asset_error));

        let retrieve_ingestion_config_error =
            Error::new_msg(ErrorKind::RetrieveIngestionConfigError, "retrieval failed");
        assert!(decider.should_retry(&retrieve_ingestion_config_error));

        let retrieve_run_error = Error::new_msg(ErrorKind::RetrieveRunError, "retrieval failed");
        assert!(decider.should_retry(&retrieve_run_error));

        // Test non-retryable error kinds
        let argument_error = Error::new_msg(ErrorKind::ArgumentValidationError, "bad argument");
        assert!(!decider.should_retry(&argument_error));

        let not_found_error = Error::new_msg(ErrorKind::NotFoundError, "not found");
        assert!(!decider.should_retry(&not_found_error));
    }

    #[tokio::test]
    async fn test_no_retry_on_non_retryable_error() {
        let counter = Arc::new(AtomicUsize::new(0));

        let cfg = RetryConfig {
            max_attempts: 3,
            base_delay: Duration::from_millis(10),
            max_delay: Duration::from_secs(5),
            backoff_multiplier: 2.0,
        };

        let retrying = Retrying::new((), cfg);

        let result = retrying
            .call(|_| {
                let counter = counter.clone();
                async move {
                    counter.fetch_add(1, Ordering::SeqCst);
                    // InvalidArgument is not retryable
                    Err::<(), sift_error::Error>(Error::new(
                        ErrorKind::ArgumentValidationError,
                        tonic::Status::invalid_argument("invalid argument"),
                    ))
                }
            })
            .await;

        assert!(result.is_err());
        // Should only attempt once since error is not retryable
        assert_eq!(counter.load(Ordering::SeqCst), 1);
    }
}
