//! Names need not be unique and failed creates have no id; retain input indices.
use std::future::Future;

#[derive(Debug)]
pub struct BatchResult<T, E> {
    pub successes: Vec<(usize, T)>,
    pub failures: Vec<(usize, E)>,
    pub not_attempted: Vec<usize>,
}

pub async fn execute<T, R, E, F, Fut>(
    items: Vec<T>,
    mut operation: F,
    should_stop: impl Fn(&E) -> bool,
) -> BatchResult<R, E>
where
    F: FnMut(T) -> Fut,
    Fut: Future<Output = Result<R, E>>,
{
    let count = items.len();
    let mut result = BatchResult {
        successes: Vec::new(),
        failures: Vec::new(),
        not_attempted: Vec::new(),
    };
    for (index, item) in items.into_iter().enumerate() {
        match operation(item).await {
            Ok(value) => result.successes.push((index, value)),
            Err(error) => {
                let stop = should_stop(&error);
                result.failures.push((index, error));
                if stop {
                    result.not_attempted.extend(index + 1..count);
                    break;
                }
            }
        }
    }
    result
}

#[cfg(test)]
mod test {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};

    #[tokio::test]
    async fn preserves_indices_and_continues_after_item_failure() {
        let result = execute(
            vec![0, 1, 2],
            |item| async move {
                if item == 1 {
                    Err("invalid")
                } else {
                    Ok(item * 10)
                }
            },
            |_| false,
        )
        .await;
        assert_eq!(result.successes, vec![(0, 0), (2, 20)]);
        assert_eq!(result.failures, vec![(1, "invalid")]);
        assert!(result.not_attempted.is_empty());
    }

    #[tokio::test]
    async fn stops_without_attempting_or_retrying_the_tail() {
        let calls = AtomicUsize::new(0);
        let result = execute(
            vec![0, 1, 2, 3],
            |item| {
                calls.fetch_add(1, Ordering::SeqCst);
                async move {
                    if item == 1 {
                        Err("unavailable")
                    } else {
                        Ok(item)
                    }
                }
            },
            |_| true,
        )
        .await;
        assert_eq!(calls.load(Ordering::SeqCst), 2);
        assert_eq!(result.successes, vec![(0, 0)]);
        assert_eq!(result.failures, vec![(1, "unavailable")]);
        assert_eq!(result.not_attempted, vec![2, 3]);
    }
}
