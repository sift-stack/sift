use pyo3::prelude::*;
use pyo3_stub_gen::derive::*;
use sift_stream::{
    DiskBackupPolicy, RecoveryStrategy, RetryPolicy, backup::disk::RollingFilePolicy,
};

// Type Definitions
#[gen_stub_pyclass]
#[pyclass]
#[derive(Clone, Copy, Debug)]
pub struct DurationPy {
    #[pyo3(get, set)]
    secs: u64,
    #[pyo3(get, set)]
    nanos: u32,
}

// Pyo3 doesn't support nested enums, so we need to build RecoveryStrategy differently
#[gen_stub_pyclass]
#[pyclass]
#[derive(Clone, Debug)]
pub struct RecoveryStrategyPy {
    inner: RecoveryStrategy,
}

#[gen_stub_pyclass]
#[pyclass]
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

#[gen_stub_pyclass]
#[pyclass]
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
#[pyclass]
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

impl From<RecoveryStrategyPy> for RecoveryStrategy {
    fn from(strategy: RecoveryStrategyPy) -> Self {
        strategy.inner
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
impl RecoveryStrategyPy {
    #[staticmethod]
    pub fn retry_only(retry_policy: RetryPolicyPy) -> Self {
        Self {
            inner: RecoveryStrategy::RetryOnly(retry_policy.into()),
        }
    }

    #[staticmethod]
    pub fn retry_with_backups(
        retry_policy: RetryPolicyPy,
        disk_backup_policy: DiskBackupPolicyPy,
    ) -> Self {
        Self {
            inner: RecoveryStrategy::RetryWithBackups {
                retry_policy: retry_policy.into(),
                disk_backup_policy: disk_backup_policy.into(),
            },
        }
    }

    #[staticmethod]
    pub fn default() -> Self {
        Self {
            inner: RecoveryStrategy::default(),
        }
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
