use std::time::Duration;

/// A retry policy that is used to configure the retry behavior of a Sift stream. Most users should
/// opt to use the default retry policy provided by [RetryPolicy::default], however, they are able
/// to completely configure their own.
#[derive(Debug, Clone)]
pub struct RetryPolicy {
    pub max_attempts: u8,
    pub initial_backoff: Duration,
    pub max_backoff: Duration,
    pub backoff_multiplier: u8,
}

impl Default for RetryPolicy {
    /// The default [RetryPolicy] that is configured to retry 5 times with exponential backoff.
    fn default() -> Self {
        Self {
            max_attempts: 5,
            initial_backoff: Duration::from_millis(50),
            max_backoff: Duration::from_secs(5),
            backoff_multiplier: 5,
        }
    }
}
