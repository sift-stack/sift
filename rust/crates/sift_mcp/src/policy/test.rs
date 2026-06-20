use std::sync::Arc;
use std::sync::atomic::{AtomicU32, Ordering};

use tonic::{Code, Status};

use super::{RetryPolicy, with_retry};

fn fast_policy() -> RetryPolicy {
    RetryPolicy {
        max_attempts: 3,
        base_backoff_ms: 1,
    }
}

#[tokio::test]
async fn succeeds_after_transient_failures() {
    let policy = fast_policy();
    let counter = Arc::new(AtomicU32::new(0));
    let result = with_retry(&policy, || {
        let counter = counter.clone();
        async move {
            let n = counter.fetch_add(1, Ordering::SeqCst);
            if n < 2 {
                Err(Status::unavailable("retry me"))
            } else {
                Ok(42i32)
            }
        }
    })
    .await;

    assert_eq!(result.unwrap(), 42);
    assert_eq!(counter.load(Ordering::SeqCst), 3);
}

#[tokio::test]
async fn returns_last_status_on_max_attempts() {
    let policy = fast_policy();
    let counter = Arc::new(AtomicU32::new(0));
    let result: Result<(), Status> = with_retry(&policy, || {
        let counter = counter.clone();
        async move {
            counter.fetch_add(1, Ordering::SeqCst);
            Err(Status::unavailable("nope"))
        }
    })
    .await;

    let err = result.unwrap_err();
    assert_eq!(err.code(), Code::Unavailable);
    assert_eq!(err.message(), "nope");
    assert_eq!(counter.load(Ordering::SeqCst), 3);
}

#[tokio::test]
async fn resource_exhausted_does_not_retry() {
    let policy = fast_policy();
    let counter = Arc::new(AtomicU32::new(0));
    let result: Result<(), Status> = with_retry(&policy, || {
        let counter = counter.clone();
        async move {
            counter.fetch_add(1, Ordering::SeqCst);
            Err(Status::resource_exhausted("slow down"))
        }
    })
    .await;

    assert_eq!(result.unwrap_err().code(), Code::ResourceExhausted);
    assert_eq!(counter.load(Ordering::SeqCst), 1);
}

#[tokio::test]
async fn deadline_exceeded_does_not_retry() {
    let policy = fast_policy();
    let counter = Arc::new(AtomicU32::new(0));
    let result: Result<(), Status> = with_retry(&policy, || {
        let counter = counter.clone();
        async move {
            counter.fetch_add(1, Ordering::SeqCst);
            Err(Status::deadline_exceeded("too slow"))
        }
    })
    .await;

    assert_eq!(result.unwrap_err().code(), Code::DeadlineExceeded);
    assert_eq!(counter.load(Ordering::SeqCst), 1);
}

#[tokio::test]
async fn internal_does_not_retry() {
    let policy = fast_policy();
    let counter = Arc::new(AtomicU32::new(0));
    let result: Result<(), Status> = with_retry(&policy, || {
        let counter = counter.clone();
        async move {
            counter.fetch_add(1, Ordering::SeqCst);
            Err(Status::internal("boom"))
        }
    })
    .await;

    assert_eq!(result.unwrap_err().code(), Code::Internal);
    assert_eq!(counter.load(Ordering::SeqCst), 1);
}

#[tokio::test]
async fn invalid_argument_does_not_retry() {
    let policy = fast_policy();
    let counter = Arc::new(AtomicU32::new(0));
    let result: Result<(), Status> = with_retry(&policy, || {
        let counter = counter.clone();
        async move {
            counter.fetch_add(1, Ordering::SeqCst);
            Err(Status::invalid_argument("bad input"))
        }
    })
    .await;

    assert_eq!(result.unwrap_err().code(), Code::InvalidArgument);
    assert_eq!(counter.load(Ordering::SeqCst), 1);
}
