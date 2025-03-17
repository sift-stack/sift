use std::time::Duration;

#[derive(Debug)]
pub struct RetryPolicy {
    pub max_attempts: u8,
    pub initial_backoff: Duration,
    pub max_backoff: Duration,
    pub backoff_multiplier: u8,
}

impl Default for RetryPolicy {
    fn default() -> Self {
        Self {
            max_attempts: 5,
            initial_backoff: Duration::from_millis(50),
            max_backoff: Duration::from_secs(5),
            backoff_multiplier: 4,
        }
    }
}
