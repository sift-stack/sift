use pyo3::prelude::*;
use pyo3_stub_gen::derive::*;
use sift_stream::{
    DiskBackupPolicy, RateLimitPolicy, RetryPolicy, backup::disk::RollingFilePolicy,
};

// Type Definitions
#[gen_stub_pyclass]
#[pyclass(from_py_object)]
#[derive(Clone, Copy, Debug)]
pub struct DurationPy {
    #[pyo3(get, set)]
    pub secs: u64,
    #[pyo3(get, set)]
    pub nanos: u32,
}

impl DurationPy {
    pub(crate) fn from_secs(secs: u64) -> Self {
        Self { secs, nanos: 0 }
    }

    pub(crate) fn from_millis(ms: u64) -> Self {
        Self {
            secs: ms / 1000,
            nanos: ((ms % 1000) * 1_000_000) as u32,
        }
    }
}

/// Python binding for [`RetryPolicy`](sift_stream::RetryPolicy).
///
/// This is a thin wrapper around the Rust `RetryPolicy` type. For detailed documentation,
/// see [`RetryPolicy`](sift_stream::RetryPolicy).
///
/// A retry policy configures the retry behavior of a Sift stream, including the number
/// of attempts and exponential backoff parameters.
#[gen_stub_pyclass]
#[pyclass(from_py_object)]
#[derive(Clone, Debug)]
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

/// Python binding for [`RateLimitPolicy`](sift_stream::RateLimitPolicy).
///
/// Controls how a stream handles Sift rate limits on control-plane calls: connecting, creating
/// or fetching the ingestion config, flows, run, and asset during `build()`, and later
/// `attach_run()` and `add_new_flows()` calls. Streaming ingestion is governed by the retry
/// policy instead.
///
/// Create one with a static constructor:
///
/// - `RateLimitPolicyPy.retry()` (default): retry after waiting at least the delay Sift requests.
/// - `RateLimitPolicyPy.retry_if_delay_at_most(max_delay)`: retry only if the wait is at most
///   `max_delay`; otherwise raise immediately.
/// - `RateLimitPolicyPy.return_error()`: never retry; raise immediately.
///
/// A rate-limit error is raised as a `RuntimeError` whose message starts with
/// `[RateLimitedError]` and includes the delay Sift requested, when provided.
#[gen_stub_pyclass]
#[pyclass(from_py_object)]
#[derive(Clone, Copy, Debug, Default)]
pub struct RateLimitPolicyPy {
    inner: RateLimitPolicy,
}

/// Python binding for [`DiskBackupPolicy`](sift_stream::backup::DiskBackupPolicy).
///
/// This is a thin wrapper around the Rust `DiskBackupPolicy` type. For detailed documentation,
/// see [`DiskBackupPolicy`](sift_stream::backup::DiskBackupPolicy).
///
/// A disk backup policy configures how telemetry data is backed up to disk, including
/// backup directory, file size limits, and retention policies.
#[gen_stub_pyclass]
#[pyclass(from_py_object)]
#[derive(Clone, Debug)]
pub struct DiskBackupPolicyPy {
    #[pyo3(get, set)]
    backups_dir: Option<String>,
    #[pyo3(get, set)]
    max_backup_file_size: usize,
    #[pyo3(get, set)]
    rolling_file_policy: RollingFilePolicyPy,
    #[pyo3(get, set)]
    retain_backups: bool,
}

#[gen_stub_pyclass]
#[pyclass(from_py_object)]
#[derive(Debug, Clone)]
pub struct RollingFilePolicyPy {
    max_file_count: Option<usize>,
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

impl From<RetryPolicy> for RetryPolicyPy {
    fn from(policy: RetryPolicy) -> Self {
        RetryPolicyPy {
            max_attempts: policy.max_attempts,
            initial_backoff: policy.initial_backoff.into(),
            max_backoff: policy.max_backoff.into(),
            backoff_multiplier: policy.backoff_multiplier,
        }
    }
}

impl From<RateLimitPolicyPy> for RateLimitPolicy {
    fn from(policy: RateLimitPolicyPy) -> Self {
        policy.inner
    }
}

impl From<RateLimitPolicy> for RateLimitPolicyPy {
    fn from(policy: RateLimitPolicy) -> Self {
        RateLimitPolicyPy { inner: policy }
    }
}

impl From<DiskBackupPolicyPy> for DiskBackupPolicy {
    fn from(policy: DiskBackupPolicyPy) -> Self {
        DiskBackupPolicy {
            backups_dir: policy.backups_dir.map(|p| p.into()),
            max_backup_file_size: policy.max_backup_file_size,
            rolling_file_policy: policy.rolling_file_policy.into(),
            retain_backups: policy.retain_backups,
        }
    }
}

impl From<DiskBackupPolicy> for DiskBackupPolicyPy {
    fn from(policy: DiskBackupPolicy) -> Self {
        DiskBackupPolicyPy {
            backups_dir: policy.backups_dir.map(|p| p.to_string_lossy().to_string()),
            max_backup_file_size: policy.max_backup_file_size,
            rolling_file_policy: policy.rolling_file_policy.into(),
            retain_backups: policy.retain_backups,
        }
    }
}

impl From<RollingFilePolicyPy> for RollingFilePolicy {
    fn from(policy: RollingFilePolicyPy) -> Self {
        RollingFilePolicy {
            max_file_count: policy.max_file_count,
        }
    }
}

impl From<RollingFilePolicy> for RollingFilePolicyPy {
    fn from(policy: RollingFilePolicy) -> Self {
        RollingFilePolicyPy {
            max_file_count: policy.max_file_count,
        }
    }
}

// PyO3 Method Implementations
#[gen_stub_pymethods]
#[pymethods]
impl DurationPy {
    #[new]
    pub fn new(secs: u64, nanos: u32) -> Self {
        Self { secs, nanos }
    }
}

#[gen_stub_pymethods]
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
        RetryPolicy::default().into()
    }
}

#[gen_stub_pymethods]
#[pymethods]
impl RateLimitPolicyPy {
    /// Retry rate-limited calls after waiting at least the delay Sift requests. This is the
    /// default.
    #[staticmethod]
    pub fn retry() -> Self {
        RateLimitPolicy::Retry.into()
    }

    /// Retry rate-limited calls only if the wait is at most `max_delay`; otherwise raise the
    /// rate-limit error immediately.
    #[staticmethod]
    pub fn retry_if_delay_at_most(max_delay: DurationPy) -> Self {
        RateLimitPolicy::RetryIfDelayAtMost(max_delay.into()).into()
    }

    /// Never retry rate-limited calls; raise the rate-limit error immediately.
    #[staticmethod]
    pub fn return_error() -> Self {
        RateLimitPolicy::ReturnError.into()
    }

    #[staticmethod]
    pub fn default() -> Self {
        RateLimitPolicy::default().into()
    }

    pub fn __repr__(&self) -> String {
        match self.inner {
            RateLimitPolicy::Retry => "RateLimitPolicyPy.retry()".to_string(),
            RateLimitPolicy::RetryIfDelayAtMost(max_delay) => {
                let max_delay = DurationPy::from(max_delay);
                format!(
                    "RateLimitPolicyPy.retry_if_delay_at_most(DurationPy(secs={}, nanos={}))",
                    max_delay.secs, max_delay.nanos
                )
            }
            RateLimitPolicy::ReturnError => "RateLimitPolicyPy.return_error()".to_string(),
        }
    }
}

#[gen_stub_pymethods]
#[pymethods]
impl DiskBackupPolicyPy {
    #[new]
    pub fn new(
        backups_dir: Option<String>,
        max_backup_file_size: usize,
        rolling_file_policy: RollingFilePolicyPy,
        retain_backups: bool,
    ) -> Self {
        Self {
            backups_dir,
            max_backup_file_size,
            rolling_file_policy,
            retain_backups,
        }
    }

    #[staticmethod]
    pub fn default() -> Self {
        DiskBackupPolicy::default().into()
    }
}

#[gen_stub_pymethods]
#[pymethods]
impl RollingFilePolicyPy {
    #[new]
    pub fn new(max_file_count: Option<usize>) -> Self {
        Self { max_file_count }
    }

    #[staticmethod]
    pub fn default() -> Self {
        RollingFilePolicy::default().into()
    }
}
