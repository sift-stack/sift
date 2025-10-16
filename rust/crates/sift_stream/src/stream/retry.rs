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

impl RetryPolicy {
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
