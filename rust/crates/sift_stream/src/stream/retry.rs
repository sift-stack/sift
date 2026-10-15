use sift_rs::retry::{RateLimitPolicy, RetryConfig, RetryExt, Retrying};
use std::time::Duration;

/// Wraps a service for control-plane calls (ingestion configs, flows, runs, assets) with the
/// default retry configuration and the stream's rate-limit policy.
pub(crate) fn control_plane_retry<T>(inner: T, policy: RateLimitPolicy) -> Retrying<T> {
    inner
        .retrying(RetryConfig::default())
        .with_rate_limit_policy(policy)
}

/// A retry policy that configures the stream retry behavior of a Sift stream
/// instance.
///
/// Most users should opt to use the default retry policy provided by [`RetryPolicy::default`].
///
/// The retry policy uses exponential backoff with configurable parameters. When a retryable
/// error occurs, the stream will wait for the calculated backoff duration before retrying.
///
/// When Sift rate limits a stream, the next attempt waits the larger of Sift's requested delay
/// plus [`RateLimitPolicy::DEFAULT_MARGIN`] and the backoff, but never longer than
/// `max_backoff`. If Sift asks to wait longer than `max_backoff`, the stream still reconnects
/// after `max_backoff` and keeps retrying as it does for any other failure.
///
/// # Example
///
/// ```
/// use sift_stream::RetryPolicy;
/// use std::time::Duration;
///
/// // Use default policy
/// let default_policy = RetryPolicy::default();
///
/// // Create custom policy
/// let custom_policy = RetryPolicy {
///     max_attempts: 10,
///     initial_backoff: Duration::from_millis(100),
///     max_backoff: Duration::from_secs(10),
///     backoff_multiplier: 2,
/// };
/// ```
#[derive(Debug, Clone)]
pub struct RetryPolicy {
    /// Maximum number of retry attempts (including the initial attempt).
    pub max_attempts: u8,
    /// Initial backoff duration for the first retry.
    pub initial_backoff: Duration,
    /// Maximum backoff duration cap.
    pub max_backoff: Duration,
    /// Multiplier for exponential backoff (applied to current wait time).
    pub backoff_multiplier: u8,
}

impl Default for RetryPolicy {
    /// The default [`RetryPolicy`] configured to retry 5 times with exponential backoff.
    ///
    /// Default settings:
    /// - `max_attempts`: 5
    /// - `initial_backoff`: 50ms
    /// - `max_backoff`: 5s
    /// - `backoff_multiplier`: 5
    fn default() -> Self {
        Self {
            max_attempts: 5,
            initial_backoff: Duration::from_millis(50),
            max_backoff: Duration::from_secs(5),
            backoff_multiplier: 5,
        }
    }
}

impl RetryPolicy {
    /// Calculates the next backoff duration based on the current wait time.
    ///
    /// The backoff calculation:
    /// - If `current_wait` is zero, returns `initial_backoff`
    /// - Otherwise, multiplies `current_wait` by `backoff_multiplier` and caps at `max_backoff`
    ///
    /// # Arguments
    ///
    /// * `current_wait` - The current wait duration (use `Duration::ZERO` for the first retry)
    ///
    /// # Returns
    ///
    /// The calculated backoff duration.
    ///
    /// # Example
    ///
    /// ```
    /// use sift_stream::RetryPolicy;
    /// use std::time::Duration;
    ///
    /// let policy = RetryPolicy::default();
    /// let first_backoff = policy.backoff(Duration::ZERO);
    /// let second_backoff = policy.backoff(first_backoff);
    /// ```
    pub fn backoff(&self, current_wait: Duration) -> Duration {
        if current_wait == Duration::ZERO {
            return self.initial_backoff;
        }
        (current_wait * u32::from(self.backoff_multiplier)).min(self.max_backoff)
    }

    /// How long to wait before the next streaming attempt, given this attempt's `backoff` and
    /// the delay Sift requested if it rate limited the attempt.
    ///
    /// This is [`RateLimitPolicy::retry`] with `max_backoff` as the maximum delay: a
    /// rate-limited attempt waits the larger of Sift's delay plus
    /// [`RateLimitPolicy::DEFAULT_MARGIN`] and the backoff, but no longer than the backoff can
    /// reach. If Sift asked for longer, the next attempt may be rate limited again and is retried
    /// like any other failure.
    pub(crate) fn wait_for_retry(
        &self,
        backoff: Duration,
        server_delay: Option<Duration>,
    ) -> Duration {
        // `initial_backoff` is not capped by `max_backoff`, so the first backoff can exceed it.
        let max_delay = self.max_backoff.max(backoff);
        // `RateLimitPolicy::Retry` always retries, so the fallback is never used.
        RateLimitPolicy::retry()
            .with_max_delay(max_delay)
            .retry_wait(server_delay, backoff)
            .unwrap_or(backoff)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_backoff() {
        let policy = RetryPolicy::default();

        let mut current_wait = Duration::ZERO;

        // First backoff should be the initial backoff.
        current_wait = policy.backoff(current_wait);
        assert_eq!(current_wait, policy.initial_backoff);

        // Subsequent backoffs should use the multiplier until the max backoff is reached.
        current_wait = policy.backoff(current_wait);
        assert_eq!(current_wait, Duration::from_millis(250));

        current_wait = policy.backoff(current_wait);
        assert_eq!(current_wait, Duration::from_millis(1250));

        // The max backoff should be returned.
        current_wait = policy.backoff(current_wait);
        assert_eq!(current_wait, policy.max_backoff);

        // No internal state should be affected by the backoff function,
        // so if the current wait is zero, the initial backoff should be returned.
        current_wait = policy.backoff(Duration::ZERO);
        assert_eq!(current_wait, policy.initial_backoff);
    }

    #[test]
    fn test_wait_for_retry() {
        let ms = Duration::from_millis;
        // Default policy: 5s max backoff.
        let policy = RetryPolicy::default();

        // Without a rate limit, the backoff is used as is.
        assert_eq!(policy.wait_for_retry(ms(250), None), ms(250));

        // A rate limit waits the larger of Sift's delay plus the margin and the backoff.
        assert_eq!(policy.wait_for_retry(ms(250), Some(ms(1000))), ms(1100));
        assert_eq!(policy.wait_for_retry(ms(250), Some(ms(100))), ms(250));

        // The wait, margin included, is capped at `max_backoff`.
        assert_eq!(policy.wait_for_retry(ms(250), Some(ms(4950))), ms(5000));
        assert_eq!(policy.wait_for_retry(ms(250), Some(ms(60_000))), ms(5000));

        // An `initial_backoff` above `max_backoff` raises the cap to the backoff.
        let policy = RetryPolicy {
            initial_backoff: ms(8000),
            ..RetryPolicy::default()
        };
        assert_eq!(policy.wait_for_retry(ms(8000), Some(ms(6000))), ms(8000));
        assert_eq!(policy.wait_for_retry(ms(8000), Some(ms(60_000))), ms(8000));
    }
}
