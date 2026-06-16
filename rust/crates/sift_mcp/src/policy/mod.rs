use std::time::Duration;

use rand::RngExt;
use tokio::time::sleep;
use tonic::{Code, Status};

#[cfg(test)]
mod test;

#[derive(Debug, Clone)]
pub struct RetryPolicy {
    pub max_attempts: u32,
    pub base_backoff_ms: u64,
}

impl Default for RetryPolicy {
    fn default() -> Self {
        Self {
            max_attempts: 3,
            base_backoff_ms: 250,
        }
    }
}

/// Wrap a gRPC call in retry-with-backoff policy. Retries on transient codes
/// (`Unavailable`, `ResourceExhausted`) using exponential backoff with full
/// jitter. Returns the most recent `Status` when retries exhaust or when a
/// non-retriable code is observed.
pub async fn with_retry<T, F, Fut>(policy: &RetryPolicy, op: F) -> Result<T, Status>
where
    F: Fn() -> Fut,
    Fut: Future<Output = Result<T, Status>>,
{
    let mut attempt: u32 = 0;

    loop {
        match op().await {
            Ok(v) => return Ok(v),
            Err(s) => match s.code() {
                Code::Unavailable | Code::ResourceExhausted => {
                    attempt += 1;
                    if attempt >= policy.max_attempts {
                        return Err(s);
                    }
                }
                Code::DeadlineExceeded => return Err(s),
                Code::Internal => return Err(s),
                _ => return Err(s),
            },
        }

        let backoff_ms = policy
            .base_backoff_ms
            .saturating_mul(2u64.saturating_pow(attempt - 1));
        let jittered = rand::rng().random_range(0..=backoff_ms);
        sleep(Duration::from_millis(jittered)).await;
    }
}
