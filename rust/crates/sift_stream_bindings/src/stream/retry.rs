use pyo3::prelude::*;
use sift_stream::{RecoveryStrategy, RetryPolicy};
use std::path::PathBuf;

// Type Definitions
#[pyclass]
#[derive(Clone, Copy)]
pub struct DurationPy {
    #[pyo3(get, set)]
    secs: u64,
    #[pyo3(get, set)]
    nanos: u32,
}

#[pyclass]
#[derive(Clone)]
pub struct RecoveryStrategyPy {
    #[pyo3(get, set)]
    strategy_type: String,
    #[pyo3(get, set)]
    retry_policy: Option<RetryPolicyPy>,
    #[pyo3(get, set)]
    max_buffer_size: Option<usize>,
    #[pyo3(get, set)]
    backups_dir: Option<String>,
    #[pyo3(get, set)]
    max_backups_file_size: Option<usize>,
}

#[pyclass]
#[derive(Clone)]
pub struct RetryPolicyPy {
    #[pyo3(get, set)]
    max_attempts: u8,
    #[pyo3(get, set)]
    initial_backoff: DurationPy,
    #[pyo3(get, set)]
    max_backoff: DurationPy,
    #[pyo3(get, set)]
    backoff_multiplier: u8,
}

// Trait Implementations
impl From<std::time::Duration> for DurationPy {
    fn from(duration: std::time::Duration) -> Self {
        Self {
            secs: duration.as_secs(),
            nanos: duration.subsec_nanos(),
        }
    }
}

impl From<DurationPy> for std::time::Duration {
    fn from(duration: DurationPy) -> Self {
        std::time::Duration::new(duration.secs, duration.nanos)
    }
}

impl From<RecoveryStrategyPy> for RecoveryStrategy {
    fn from(strategy: RecoveryStrategyPy) -> Self {
        match strategy.strategy_type.as_str() {
            "RetryOnly" => RecoveryStrategy::RetryOnly(strategy.retry_policy.unwrap().into()),
            "RetryWithInMemoryBackups" => RecoveryStrategy::RetryWithInMemoryBackups {
                retry_policy: strategy.retry_policy.unwrap().into(),
                max_buffer_size: strategy.max_buffer_size,
            },
            "RetryWithDiskBackups" => RecoveryStrategy::RetryWithDiskBackups {
                retry_policy: strategy.retry_policy.unwrap().into(),
                backups_dir: strategy.backups_dir.map(PathBuf::from),
                max_backups_file_size: strategy.max_backups_file_size,
            },
            _ => panic!("Invalid strategy type"),
        }
    }
}

impl From<RetryPolicyPy> for RetryPolicy {
    fn from(policy: RetryPolicyPy) -> Self {
        RetryPolicy {
            max_attempts: policy.max_attempts,
            initial_backoff: std::time::Duration::new(
                policy.initial_backoff.secs,
                policy.initial_backoff.nanos,
            ),
            max_backoff: std::time::Duration::new(
                policy.max_backoff.secs,
                policy.max_backoff.nanos,
            ),
            backoff_multiplier: policy.backoff_multiplier,
        }
    }
}

// PyO3 Method Implementations
#[pymethods]
impl DurationPy {
    #[new]
    pub fn new(secs: u64, nanos: u32) -> Self {
        Self { secs, nanos }
    }
}

#[pymethods]
impl RecoveryStrategyPy {
    #[new]
    pub fn new(
        strategy_type: &str,
        retry_policy: Option<RetryPolicyPy>,
        max_buffer_size: Option<usize>,
        backups_dir: Option<String>,
        max_backups_file_size: Option<usize>,
    ) -> Self {
        Self {
            strategy_type: strategy_type.to_string(),
            retry_policy,
            max_buffer_size,
            backups_dir,
            max_backups_file_size,
        }
    }

    #[staticmethod]
    pub fn retry_only(retry_policy: RetryPolicyPy) -> Self {
        Self {
            strategy_type: "RetryOnly".to_string(),
            retry_policy: Some(retry_policy),
            max_buffer_size: None,
            backups_dir: None,
            max_backups_file_size: None,
        }
    }

    #[staticmethod]
    pub fn retry_with_in_memory_backups(
        retry_policy: RetryPolicyPy,
        max_buffer_size: Option<usize>,
    ) -> Self {
        Self {
            strategy_type: "RetryWithInMemoryBackups".to_string(),
            retry_policy: Some(retry_policy),
            max_buffer_size,
            backups_dir: None,
            max_backups_file_size: None,
        }
    }

    #[staticmethod]
    pub fn retry_with_disk_backups(
        retry_policy: RetryPolicyPy,
        backups_dir: Option<String>,
        max_backups_file_size: Option<usize>,
    ) -> Self {
        Self {
            strategy_type: "RetryWithDiskBackups".to_string(),
            retry_policy: Some(retry_policy),
            max_buffer_size: None,
            backups_dir,
            max_backups_file_size,
        }
    }

    #[staticmethod]
    pub fn default() -> Self {
        Self::retry_only(RetryPolicyPy::default())
    }

    #[staticmethod]
    pub fn default_retry_policy_in_memory_backups() -> Self {
        Self::retry_with_in_memory_backups(RetryPolicyPy::default(), None)
    }

    #[staticmethod]
    pub fn default_retry_policy_disk_backups() -> Self {
        Self::retry_with_disk_backups(RetryPolicyPy::default(), None, None)
    }
}

#[pymethods]
impl RetryPolicyPy {
    #[new]
    pub fn new(
        max_attempts: u8,
        initial_backoff: DurationPy,
        max_backoff: DurationPy,
        backoff_multiplier: u8,
    ) -> Self {
        Self {
            max_attempts,
            initial_backoff,
            max_backoff,
            backoff_multiplier,
        }
    }

    #[staticmethod]
    pub fn default() -> Self {
        Self {
            max_attempts: 5,
            initial_backoff: DurationPy::new(0, 50_000_000), // 50ms
            max_backoff: DurationPy::new(5, 0),              // 5s
            backoff_multiplier: 5,
        }
    }
}
