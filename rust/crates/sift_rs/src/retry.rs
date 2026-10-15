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
//!
//! ## Rate Limiting
//!
//! When Sift rate limits a request it returns `ResourceExhausted` with a `google.rpc.RetryInfo`
//! detail, which the wrappers surface as
//! [`ErrorKind::RateLimitedError`](sift_error::ErrorKind::RateLimitedError). By default, [`Retrying`]
//! retries these errors after waiting the larger of the server's `retry_delay` and the configured
//! backoff. Use [`Retrying::with_rate_limit_policy`] to cap or disable it:
//!
//! ```no_run
//! use sift_rs::retry::{RateLimitPolicy, RetryConfig, RetryExt};
//! use sift_rs::wrappers::assets::{AssetServiceWrapper, new_asset_service};
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! # let channel = todo!();
//! let svc = new_asset_service(channel)
//!     .retrying(RetryConfig::default())
//!     .with_rate_limit_policy(RateLimitPolicy::ReturnError);
//!
//! match svc.call(|mut w| async move { w.try_get_asset_by_id("asset-123").await }).await {
//!     Ok(asset) => println!("{}", asset.name),
//!     Err(err) if err.is_rate_limited() => println!("rate limited; retry in {:?}", err.retry_delay()),
//!     Err(err) => return Err(err.into()),
//! }
//! # Ok(())
//! # }
//! ```

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

/// How [`Retrying`] handles errors that its decider reports as rate limited.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum RateLimitPolicy {
    /// Retry after waiting the larger of the server's `retry_delay` and the configured backoff.
    /// The server's delay is not capped by [`RetryConfig::max_delay`]. Rate-limited retries count
    /// against [`RetryConfig::max_attempts`].
    #[default]
    Retry,
    /// Retry only if the wait computed as for [`RateLimitPolicy::Retry`] is at most this duration;
    /// otherwise return the error immediately.
    RetryIfDelayAtMost(Duration),
    /// Never retry rate-limited errors; return them immediately so the caller can decide what to
    /// do, for example using [`sift_error::Error::retry_delay`].
    ReturnError,
}

/// Trait for determining whether an error should trigger a retry.
pub trait RetryDecider<E> {
    /// Returns `true` if the error should trigger a retry attempt.
    fn should_retry(&self, err: &E) -> bool;

    /// Returns `true` if the error is a server rate-limit signal. Rate-limited errors are handled
    /// by the [`RateLimitPolicy`] of the [`Retrying`] adapter instead of
    /// [`should_retry`](RetryDecider::should_retry).
    fn is_rate_limited(&self, _err: &E) -> bool {
        false
    }

    /// Returns the minimum delay the server requested before retrying, if known.
    fn retry_delay(&self, _err: &E) -> Option<Duration> {
        None
    }
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
                | ErrorKind::RateLimitedError { .. }
        )
    }

    fn is_rate_limited(&self, err: &sift_error::Error) -> bool {
        err.is_rate_limited()
    }

    fn retry_delay(&self, err: &sift_error::Error) -> Option<Duration> {
        err.retry_delay()
    }
}

/// Retry decider that only retries rate-limited errors.
///
/// Use this for calls that should fail fast on ordinary errors but still honor a server's
/// rate limit. Rate-limited errors are handled by the [`RateLimitPolicy`] of the [`Retrying`]
/// adapter.
#[derive(Debug, Clone, Copy, Default)]
pub struct RateLimitOnly;

impl RetryDecider<sift_error::Error> for RateLimitOnly {
    fn should_retry(&self, _err: &sift_error::Error) -> bool {
        false
    }

    fn is_rate_limited(&self, err: &sift_error::Error) -> bool {
        err.is_rate_limited()
    }

    fn retry_delay(&self, err: &sift_error::Error) -> Option<Duration> {
        err.retry_delay()
    }
}

/// Adapter that wraps a type and provides retry functionality.
#[derive(Clone, Debug)]
pub struct Retrying<T, D = DefaultGrpcRetry> {
    inner: T,
    cfg: RetryConfig,
    decider: D,
    rate_limit_policy: RateLimitPolicy,
}

impl<T> Retrying<T> {
    /// Creates a new `Retrying` adapter with the default gRPC retry decider and the default
    /// [`RateLimitPolicy`].
    pub fn new(inner: T, cfg: RetryConfig) -> Self {
        Self {
            inner,
            cfg,
            decider: DefaultGrpcRetry,
            rate_limit_policy: RateLimitPolicy::default(),
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
            rate_limit_policy: self.rate_limit_policy,
        }
    }

    /// Sets how rate-limited errors are handled. Defaults to [`RateLimitPolicy::Retry`].
    pub fn with_rate_limit_policy(mut self, policy: RateLimitPolicy) -> Self {
        self.rate_limit_policy = policy;
        self
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
    /// The closure is called up to `max_attempts` times, and at least once. If it returns an
    /// error, the function decides whether to retry:
    ///
    /// - If the decider reports the error as rate limited, the [`RateLimitPolicy`] applies. When
    ///   it allows a retry, the function waits the larger of the server's retry delay and the
    ///   calculated backoff delay.
    /// - Otherwise, if the decider indicates the error is retryable, the function waits for the
    ///   calculated backoff delay before retrying.
    ///
    /// # Arguments
    ///
    /// * `f` - A closure that takes a cloned wrapper and returns a future
    ///   that produces a `Result`. The closure can use `&mut self` internally
    ///   since each attempt gets a fresh clone.
    ///
    /// # Returns
    ///
    /// Returns `Ok(result)` if any attempt succeeds, or `Err(error)` with the last error if all
    /// attempts fail, the error is not retryable, or the [`RateLimitPolicy`] declines to retry.
    pub async fn call<F, Fut, R, E>(&self, mut f: F) -> StdResult<R, E>
    where
        F: FnMut(T) -> Fut,
        Fut: Future<Output = StdResult<R, E>>,
        D: RetryDecider<E>,
    {
        let mut attempt = 1;

        loop {
            let err = match f(self.inner.clone()).await {
                Ok(result) => return Ok(result),
                Err(e) => e,
            };

            if attempt >= self.cfg.max_attempts {
                return Err(err);
            }

            let delay = if self.decider.is_rate_limited(&err) {
                let backoff = self.cfg.backoff(attempt);
                // The server's delay is a floor, so it is not capped by `max_delay`.
                let delay = self
                    .decider
                    .retry_delay(&err)
                    .map_or(backoff, |server_delay| server_delay.max(backoff));

                match self.rate_limit_policy {
                    RateLimitPolicy::Retry => delay,
                    RateLimitPolicy::RetryIfDelayAtMost(limit) if delay <= limit => delay,
                    _ => return Err(err),
                }
            } else if self.decider.should_retry(&err) {
                self.cfg.backoff(attempt)
            } else {
                return Err(err);
            };

            tokio::time::sleep(delay).await;
            attempt += 1;
        }
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

    fn test_cfg(max_attempts: usize) -> RetryConfig {
        RetryConfig {
            max_attempts,
            base_delay: Duration::from_millis(100),
            max_delay: Duration::from_secs(5),
            backoff_multiplier: 2.0,
        }
    }

    fn rate_limited(retry_delay: Option<Duration>) -> sift_error::Error {
        Error::new_rate_limited(
            tonic::Status::resource_exhausted("Rate limit exceeded"),
            retry_delay,
        )
    }

    /// Runs `retrying.call` where attempt `n` (1-indexed) fails with `errors(n)`, or succeeds once
    /// `errors` returns `None`. Returns the result, the number of attempts, and the elapsed time.
    async fn run<D>(
        retrying: &Retrying<(), D>,
        errors: impl Fn(usize) -> Option<sift_error::Error>,
    ) -> (sift_error::Result<()>, usize, Duration)
    where
        D: RetryDecider<sift_error::Error>,
    {
        let attempts = AtomicUsize::new(0);
        let start = tokio::time::Instant::now();
        let result = retrying
            .call(|_| {
                let attempt = attempts.fetch_add(1, Ordering::SeqCst) + 1;
                let outcome = errors(attempt).map_or(Ok(()), Err);
                async move { outcome }
            })
            .await;
        (result, attempts.load(Ordering::SeqCst), start.elapsed())
    }

    /// Paused-clock timers can round a deadline up to the next millisecond.
    fn assert_elapsed(actual: Duration, expected: Duration) {
        assert!(
            actual >= expected && actual <= expected + Duration::from_millis(5),
            "expected about {expected:?}, waited {actual:?}"
        );
    }

    #[tokio::test(start_paused = true)]
    async fn test_rate_limited_waits_for_server_delay() {
        let retrying = Retrying::new((), test_cfg(3));
        let (result, attempts, elapsed) = run(&retrying, |n| {
            (n == 1).then(|| rate_limited(Some(Duration::from_secs(3))))
        })
        .await;

        assert!(result.is_ok());
        assert_eq!(attempts, 2);
        assert_elapsed(elapsed, Duration::from_secs(3));
    }

    #[tokio::test(start_paused = true)]
    async fn test_rate_limited_server_delay_is_not_capped_by_max_delay() {
        let retrying = Retrying::new((), test_cfg(3));
        let (result, _, elapsed) = run(&retrying, |n| {
            (n == 1).then(|| rate_limited(Some(Duration::from_secs(10))))
        })
        .await;

        assert!(result.is_ok());
        assert_elapsed(elapsed, Duration::from_secs(10));
    }

    #[tokio::test(start_paused = true)]
    async fn test_rate_limited_uses_backoff_when_larger() {
        let retrying = Retrying::new((), test_cfg(3));
        let (result, _, elapsed) = run(&retrying, |n| {
            (n == 1).then(|| rate_limited(Some(Duration::from_millis(10))))
        })
        .await;

        assert!(result.is_ok());
        assert_elapsed(elapsed, Duration::from_millis(100));
    }

    #[tokio::test(start_paused = true)]
    async fn test_rate_limited_without_delay_uses_backoff() {
        let retrying = Retrying::new((), test_cfg(3));
        let (result, _, elapsed) = run(&retrying, |n| (n == 1).then(|| rate_limited(None))).await;

        assert!(result.is_ok());
        assert_elapsed(elapsed, Duration::from_millis(100));
    }

    #[tokio::test(start_paused = true)]
    async fn test_rate_limited_each_wait_is_max_of_delay_and_backoff() {
        let retrying = Retrying::new((), test_cfg(3));
        let (result, attempts, elapsed) = run(&retrying, |n| {
            (n <= 2).then(|| rate_limited(Some(Duration::from_millis(150))))
        })
        .await;

        // max(150ms, 100ms) + max(150ms, 200ms)
        assert!(result.is_ok());
        assert_eq!(attempts, 3);
        assert_elapsed(elapsed, Duration::from_millis(350));
    }

    #[tokio::test(start_paused = true)]
    async fn test_rate_limited_exhausts_attempts_and_returns_last_error() {
        let retrying = Retrying::new((), test_cfg(3));
        let (result, attempts, elapsed) = run(&retrying, |_| {
            Some(rate_limited(Some(Duration::from_secs(1))))
        })
        .await;

        let err = result.unwrap_err();
        assert!(err.is_rate_limited());
        assert_eq!(err.retry_delay(), Some(Duration::from_secs(1)));
        assert_eq!(attempts, 3);
        assert_elapsed(elapsed, Duration::from_secs(2));
    }

    #[tokio::test(start_paused = true)]
    async fn test_return_error_policy_does_not_retry() {
        let retrying =
            Retrying::new((), test_cfg(3)).with_rate_limit_policy(RateLimitPolicy::ReturnError);
        let (result, attempts, elapsed) = run(&retrying, |_| {
            Some(rate_limited(Some(Duration::from_secs(3))))
        })
        .await;

        let err = result.unwrap_err();
        assert_eq!(err.retry_delay(), Some(Duration::from_secs(3)));
        assert_eq!(attempts, 1);
        assert_eq!(elapsed, Duration::ZERO);
    }

    #[tokio::test(start_paused = true)]
    async fn test_retry_if_delay_at_most_policy() {
        let retrying = Retrying::new((), test_cfg(3))
            .with_rate_limit_policy(RateLimitPolicy::RetryIfDelayAtMost(Duration::from_secs(1)));

        let (result, attempts, _) = run(&retrying, |n| {
            (n == 1).then(|| rate_limited(Some(Duration::from_secs(3))))
        })
        .await;
        assert_eq!(
            result.unwrap_err().retry_delay(),
            Some(Duration::from_secs(3))
        );
        assert_eq!(attempts, 1);

        let (result, attempts, elapsed) = run(&retrying, |n| {
            (n == 1).then(|| rate_limited(Some(Duration::from_millis(500))))
        })
        .await;
        assert!(result.is_ok());
        assert_eq!(attempts, 2);
        assert_elapsed(elapsed, Duration::from_millis(500));
    }

    #[tokio::test(start_paused = true)]
    async fn test_resource_exhausted_without_retry_info_is_not_retried_for_creates() {
        // A message-size rejection is `ResourceExhausted` without `RetryInfo`, so it keeps its
        // create kind and is not retried.
        let retrying = Retrying::new((), test_cfg(3));
        let (result, attempts, _) = run(&retrying, |_| {
            Some(Error::new(
                ErrorKind::CreateFlowError,
                tonic::Status::resource_exhausted("grpc: received message larger than max"),
            ))
        })
        .await;

        assert_eq!(result.unwrap_err().kind(), ErrorKind::CreateFlowError);
        assert_eq!(attempts, 1);
    }

    #[tokio::test(start_paused = true)]
    async fn test_rate_limit_only_decider() {
        let retrying = Retrying::new((), test_cfg(3)).with_decider(RateLimitOnly);

        let (result, attempts, _) = run(&retrying, |_| {
            Some(Error::new(
                ErrorKind::RetrieveAssetError,
                tonic::Status::unavailable("unavailable"),
            ))
        })
        .await;
        assert!(result.is_err());
        assert_eq!(attempts, 1);

        let (result, attempts, elapsed) = run(&retrying, |n| {
            (n == 1).then(|| rate_limited(Some(Duration::from_secs(2))))
        })
        .await;
        assert!(result.is_ok());
        assert_eq!(attempts, 2);
        assert_elapsed(elapsed, Duration::from_secs(2));
    }

    #[tokio::test(start_paused = true)]
    async fn test_with_decider_keeps_rate_limit_policy() {
        let retrying = Retrying::new((), test_cfg(3))
            .with_rate_limit_policy(RateLimitPolicy::ReturnError)
            .with_decider(RateLimitOnly);
        let (result, attempts, _) = run(&retrying, |_| {
            Some(rate_limited(Some(Duration::from_secs(1))))
        })
        .await;

        assert!(result.unwrap_err().is_rate_limited());
        assert_eq!(attempts, 1);
    }

    #[tokio::test(start_paused = true)]
    async fn test_custom_decider_without_rate_limit_methods_is_unchanged() {
        struct AlwaysRetry;

        impl RetryDecider<sift_error::Error> for AlwaysRetry {
            fn should_retry(&self, _err: &sift_error::Error) -> bool {
                true
            }
        }

        // The default trait methods report no rate limit, so the server delay is ignored and the
        // plain backoff applies, as before this decider API existed.
        let retrying = Retrying::new((), test_cfg(3)).with_decider(AlwaysRetry);
        let (result, attempts, elapsed) = run(&retrying, |n| {
            (n == 1).then(|| rate_limited(Some(Duration::from_secs(3))))
        })
        .await;

        assert!(result.is_ok());
        assert_eq!(attempts, 2);
        assert_elapsed(elapsed, Duration::from_millis(100));
    }

    #[tokio::test(start_paused = true)]
    async fn test_zero_max_attempts_makes_one_attempt() {
        let retrying = Retrying::new((), test_cfg(0));
        let (result, attempts, _) = run(&retrying, |_| Some(rate_limited(None))).await;

        assert!(result.is_err());
        assert_eq!(attempts, 1);
    }

    #[test]
    fn test_default_grpc_retry_rate_limit_methods() {
        let decider = DefaultGrpcRetry;

        let err = rate_limited(Some(Duration::from_secs(2)));
        assert!(decider.is_rate_limited(&err));
        assert!(decider.should_retry(&err));
        assert_eq!(decider.retry_delay(&err), Some(Duration::from_secs(2)));

        let err = Error::new(
            ErrorKind::RetrieveAssetError,
            tonic::Status::unavailable("unavailable"),
        );
        assert!(!decider.is_rate_limited(&err));
        assert_eq!(decider.retry_delay(&err), None);
    }
}
