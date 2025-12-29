use std::time::Duration;

/// A retry policy that configures the stream retry behavior of a Sift stream
/// instance.
///
/// Most users should opt to use the default retry policy provided by [`RetryPolicy::default`].
///
/// The retry policy uses exponential backoff with configurable parameters. When a retryable
/// error occurs, the stream will wait for the calculated backoff duration before retrying.
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
}
