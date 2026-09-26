use std::future::Future;
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

/// Run a gRPC call that is not safe to repeat, with one attempt and no retry.
///
/// AIP-194 permits retrying `Unavailable` only for idempotent methods. A server
/// can answer `Unavailable` after it has already committed — the connection
/// drops between the write and the response — so retrying a create or an append
/// mints a second artifact or a second version that nothing asked for. One
/// attempt turns that into an error the caller can report instead.
pub async fn once<T, Fut>(op: impl FnOnce() -> Fut) -> Result<T, Status>
where
    Fut: Future<Output = Result<T, Status>>,
{
    op().await
}

/// Wrap an idempotent gRPC call in retry-with-backoff policy. Per AIP-194, only
/// `Unavailable` is automatically retried (with exponential backoff and full
/// jitter). All other codes — including `ResourceExhausted`, which is a
/// server-side rate-limit signal — return immediately so the caller can surface
/// the failure rather than amplifying load.
///
/// Calls that are not safe to repeat use [`once`] instead.
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
                Code::Unavailable => {
                    attempt += 1;
                    if attempt >= policy.max_attempts {
                        return Err(s);
                    }
                }
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
