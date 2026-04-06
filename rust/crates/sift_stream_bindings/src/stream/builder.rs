use crate::sift::metadata::MetadataPy;
use crate::stream::SiftStreamPy;
use crate::stream::config::{IngestionConfigFormPy, RunFormPy};
use crate::stream::retry::{DiskBackupPolicyPy, DurationPy, RetryPolicyPy};
use pyo3::prelude::*;
use pyo3_stub_gen::derive::*;
use sift_stream::{
    Credentials, DiskBackupPolicy, RetryPolicy, SiftStreamBuilder, StreamConfigBuilder,
};

// Mirrored from sift_stream (pub(crate) there, so we duplicate the values)
const DATA_CHANNEL_CAPACITY: usize = 1024 * 100;
const CONTROL_CHANNEL_CAPACITY: usize = 1024;

// Type Definitions
/// Python binding for [`SiftStreamBuilder`](sift_stream::stream::builder::SiftStreamBuilder).
///
/// This is a thin wrapper around the Rust `SiftStreamBuilder` type. For detailed documentation,
/// see [`SiftStreamBuilder`](sift_stream::stream::builder::SiftStreamBuilder).
///
/// The builder provides a fluent API for configuring and creating a [`SiftStreamPy`] instance
/// with various options including ingestion configs, retry policies, checkpoint intervals, and more.
#[gen_stub_pyclass]
#[pyclass]
pub struct SiftStreamBuilderPy {
    #[pyo3(get, set)]
    uri: String,
    #[pyo3(get, set)]
    apikey: String,
    #[pyo3(get, set)]
    enable_tls: bool,
    #[pyo3(get, set)]
    ingestion_config: Option<IngestionConfigFormPy>,
    #[pyo3(get, set)]
    run: Option<RunFormPy>,
    #[pyo3(get, set)]
    run_id: Option<String>,
    #[pyo3(get, set)]
    asset_tags: Option<Vec<String>>,
    #[pyo3(get, set)]
    metadata: Option<Vec<MetadataPy>>,
}

// PyO3 Method Implementations
#[gen_stub_pymethods]
#[pymethods]
impl SiftStreamBuilderPy {
    #[new]
    pub fn new(uri: &str, apikey: &str) -> Self {
        Self {
            uri: uri.into(),
            apikey: apikey.into(),
            enable_tls: true,
            ingestion_config: None,
            run: None,
            run_id: None,
            asset_tags: None,
            metadata: None,
        }
    }

    pub fn build(&mut self, py: Python) -> PyResult<Py<PyAny>> {
        let ingestion_config = self.ingestion_config.clone().ok_or_else(|| {
            pyo3::exceptions::PyValueError::new_err(
                "ingestion_config must be set before calling build()",
            )
        })?;

        let mut builder = SiftStreamBuilder::new(Credentials::Config {
            uri: self.uri.clone(),
            apikey: self.apikey.clone(),
        });

        if !self.enable_tls {
            builder = builder.disable_tls();
        }

        let mut config_builder = builder.ingestion_config(ingestion_config.into());

        if let Some(run) = self.run.as_ref() {
            config_builder = config_builder.attach_run(run.clone().into());
        }

        if let Some(run_id) = self.run_id.as_ref() {
            config_builder = config_builder.attach_run_id(run_id);
        }

        if let Some(tags) = self.asset_tags.clone() {
            config_builder = config_builder.add_asset_tags(tags);
        }

        if let Some(metadata) = self.metadata.clone() {
            let metadata_rs = metadata
                .into_iter()
                .map(|m| m.into())
                .collect::<Vec<sift_rs::metadata::v1::MetadataValue>>();
            config_builder = config_builder.add_asset_metadata(metadata_rs);
        }

        let mode_builder = config_builder.live_only();

        let awaitable = pyo3_async_runtimes::tokio::future_into_py(py, async move {
            match mode_builder.build().await {
                Ok(stream) => Ok(SiftStreamPy::from(stream)),
                Err(e) => Err(crate::error::SiftErrorWrapper(e).into()),
            }
        })?;
        Ok(awaitable.into())
    }

    /// Sets the ingestion config and advances to [`StreamConfigBuilderPy`] where shared options
    /// (run, asset tags/metadata) and the streaming mode can be configured.
    pub fn ingestion_config(&self, config: IngestionConfigFormPy) -> StreamConfigBuilderPy {
        StreamConfigBuilderPy {
            uri: self.uri.clone(),
            apikey: self.apikey.clone(),
            enable_tls: self.enable_tls,
            ingestion_config: config,
            run: None,
            run_id: None,
            asset_tags: None,
            metadata: None,
        }
    }
}

// ──────────────────────────────────────────────────────────────────────────────
// Shared helper
// ──────────────────────────────────────────────────────────────────────────────

/// Constructs a [`StreamConfigBuilder`] from the shared base configuration stored in
/// each mode builder. Consumes the [`StreamConfigBuilderPy`] so callers should clone
/// `self.base` before passing it in.
fn make_stream_config_builder(base: StreamConfigBuilderPy) -> PyResult<StreamConfigBuilder> {
    let mut builder = SiftStreamBuilder::new(Credentials::Config {
        uri: base.uri,
        apikey: base.apikey,
    });

    if !base.enable_tls {
        builder = builder.disable_tls();
    }

    let mut config_builder = builder.ingestion_config(base.ingestion_config.into());

    if let Some(run) = base.run {
        config_builder = config_builder.attach_run(run.into());
    }

    if let Some(run_id) = base.run_id {
        config_builder = config_builder.attach_run_id(&run_id);
    }

    if let Some(tags) = base.asset_tags {
        config_builder = config_builder.add_asset_tags(tags);
    }

    if let Some(metadata) = base.metadata {
        let metadata_rs = metadata
            .into_iter()
            .map(|m| m.into())
            .collect::<Vec<sift_rs::metadata::v1::MetadataValue>>();
        config_builder = config_builder.add_asset_metadata(metadata_rs);
    }

    Ok(config_builder)
}

// ──────────────────────────────────────────────────────────────────────────────
// StreamConfigBuilderPy
// ──────────────────────────────────────────────────────────────────────────────

/// Holds shared configuration (run, asset tags/metadata) and provides mode selection.
///
/// Created by [`SiftStreamBuilderPy.ingestion_config()`]. Call one of the mode selectors
/// to advance to a mode-specific builder:
///
/// - [`StreamConfigBuilderPy.live_only()`] — single channel, direct backpressure, no disk backups
/// - [`StreamConfigBuilderPy.live_with_backups()`] — dual channel with checkpointing and disk backups
/// - [`StreamConfigBuilderPy.file_backup()`] — writes directly to disk, no network ingestion
#[gen_stub_pyclass]
#[pyclass(skip_from_py_object)]
#[derive(Clone)]
pub struct StreamConfigBuilderPy {
    // Credentials carried forward from SiftStreamBuilderPy
    pub(crate) uri: String,
    pub(crate) apikey: String,
    pub(crate) enable_tls: bool,
    pub(crate) ingestion_config: IngestionConfigFormPy,
    // Optional shared options
    #[pyo3(get, set)]
    pub run: Option<RunFormPy>,
    #[pyo3(get, set)]
    pub run_id: Option<String>,
    #[pyo3(get, set)]
    pub asset_tags: Option<Vec<String>>,
    #[pyo3(get, set)]
    pub metadata: Option<Vec<MetadataPy>>,
}

#[gen_stub_pymethods]
#[pymethods]
impl StreamConfigBuilderPy {
    /// Selects `LiveStreamingOnly` mode: a single bounded ingestion channel with direct
    /// backpressure. No disk backups, no checkpointing, no retry policy.
    pub fn live_only(&self) -> LiveOnlyBuilderPy {
        LiveOnlyBuilderPy {
            base: self.clone(),
            enable_compression_for_ingestion: false,
            metrics_streaming_interval: Some(DurationPy::from_millis(500)),
            ingestion_data_channel_capacity: DATA_CHANNEL_CAPACITY,
            control_channel_capacity: CONTROL_CHANNEL_CAPACITY,
        }
    }

    /// Selects `LiveStreamingWithBackups` mode: dual channel with checkpointing, retry policy,
    /// and optional disk backups.
    pub fn live_with_backups(&self) -> LiveWithBackupsBuilderPy {
        LiveWithBackupsBuilderPy {
            base: self.clone(),
            checkpoint_interval: DurationPy::from_secs(60),
            retry_policy: RetryPolicyPy::default(),
            disk_backup_policy: DiskBackupPolicyPy::default(),
            enable_compression_for_ingestion: false,
            metrics_streaming_interval: Some(DurationPy::from_millis(500)),
            ingestion_data_channel_capacity: DATA_CHANNEL_CAPACITY,
            backup_data_channel_capacity: DATA_CHANNEL_CAPACITY,
            control_channel_capacity: CONTROL_CHANNEL_CAPACITY,
        }
    }

    /// Selects `FileBackup` mode: writes all data to disk files without network ingestion.
    /// Requires `disk_backup_policy.backups_dir` to be set on the returned builder.
    pub fn file_backup(&self) -> FileBackupBuilderPy {
        FileBackupBuilderPy {
            base: self.clone(),
            disk_backup_policy: DiskBackupPolicyPy::default(),
            backup_data_channel_capacity: DATA_CHANNEL_CAPACITY,
            control_channel_capacity: CONTROL_CHANNEL_CAPACITY,
            metrics_streaming_interval: Some(DurationPy::from_millis(500)),
        }
    }
}

// ──────────────────────────────────────────────────────────────────────────────
// LiveOnlyBuilderPy
// ──────────────────────────────────────────────────────────────────────────────

/// Builder for `LiveStreamingOnly` mode.
///
/// Created by [`StreamConfigBuilderPy.live_only()`]. Call [`LiveOnlyBuilderPy.build()`]
/// to finalize.
#[gen_stub_pyclass]
#[pyclass]
pub struct LiveOnlyBuilderPy {
    pub(crate) base: StreamConfigBuilderPy,
    #[pyo3(get, set)]
    pub enable_compression_for_ingestion: bool,
    #[pyo3(get, set)]
    pub metrics_streaming_interval: Option<DurationPy>,
    #[pyo3(get, set)]
    pub ingestion_data_channel_capacity: usize,
    #[pyo3(get, set)]
    pub control_channel_capacity: usize,
}

#[gen_stub_pymethods]
#[pymethods]
impl LiveOnlyBuilderPy {
    pub fn build(&self, py: Python) -> PyResult<Py<PyAny>> {
        let base = self.base.clone();
        let enable_compression = self.enable_compression_for_ingestion;
        let metrics_interval = self
            .metrics_streaming_interval
            .map(std::time::Duration::from);
        let ingestion_cap = self.ingestion_data_channel_capacity;
        let control_cap = self.control_channel_capacity;

        let awaitable = pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let config_builder = make_stream_config_builder(base)?;

            let mode_builder = config_builder
                .live_only()
                .enable_compression_for_ingestion(enable_compression)
                .metrics_streaming_interval(metrics_interval)
                .ingestion_data_channel_capacity(ingestion_cap)
                .control_channel_capacity(control_cap);

            mode_builder
                .build()
                .await
                .map(SiftStreamPy::from)
                .map_err(|e| crate::error::SiftErrorWrapper(e).into())
        })?;

        Ok(awaitable.into())
    }
}

// ──────────────────────────────────────────────────────────────────────────────
// LiveWithBackupsBuilderPy
// ──────────────────────────────────────────────────────────────────────────────

/// Builder for `LiveStreamingWithBackups` mode.
///
/// Created by [`StreamConfigBuilderPy.live_with_backups()`]. Call
/// [`LiveWithBackupsBuilderPy.build()`] to finalize.
#[gen_stub_pyclass]
#[pyclass]
pub struct LiveWithBackupsBuilderPy {
    pub(crate) base: StreamConfigBuilderPy,
    #[pyo3(get, set)]
    pub checkpoint_interval: DurationPy,
    #[pyo3(get, set)]
    pub retry_policy: RetryPolicyPy,
    #[pyo3(get, set)]
    pub disk_backup_policy: DiskBackupPolicyPy,
    #[pyo3(get, set)]
    pub enable_compression_for_ingestion: bool,
    #[pyo3(get, set)]
    pub metrics_streaming_interval: Option<DurationPy>,
    #[pyo3(get, set)]
    pub ingestion_data_channel_capacity: usize,
    #[pyo3(get, set)]
    pub backup_data_channel_capacity: usize,
    #[pyo3(get, set)]
    pub control_channel_capacity: usize,
}

#[gen_stub_pymethods]
#[pymethods]
impl LiveWithBackupsBuilderPy {
    pub fn build(&self, py: Python) -> PyResult<Py<PyAny>> {
        let base = self.base.clone();
        let checkpoint_interval = std::time::Duration::from(self.checkpoint_interval);
        let retry_policy: RetryPolicy = self.retry_policy.clone().into();
        let disk_backup_policy: DiskBackupPolicy = self.disk_backup_policy.clone().into();
        let enable_compression = self.enable_compression_for_ingestion;
        let metrics_interval = self
            .metrics_streaming_interval
            .map(std::time::Duration::from);
        let ingestion_cap = self.ingestion_data_channel_capacity;
        let backup_cap = self.backup_data_channel_capacity;
        let control_cap = self.control_channel_capacity;

        let awaitable = pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let config_builder = make_stream_config_builder(base)?;

            let mode_builder = config_builder
                .live_with_backups()
                .checkpoint_interval(checkpoint_interval)
                .retry_policy(retry_policy)
                .disk_backup_policy(disk_backup_policy)
                .enable_compression_for_ingestion(enable_compression)
                .metrics_streaming_interval(metrics_interval)
                .ingestion_data_channel_capacity(ingestion_cap)
                .backup_data_channel_capacity(backup_cap)
                .control_channel_capacity(control_cap);

            mode_builder
                .build()
                .await
                .map(SiftStreamPy::from)
                .map_err(|e| crate::error::SiftErrorWrapper(e).into())
        })?;

        Ok(awaitable.into())
    }
}

// ──────────────────────────────────────────────────────────────────────────────
// FileBackupBuilderPy
// ──────────────────────────────────────────────────────────────────────────────

/// Builder for `FileBackup` mode.
///
/// Created by [`StreamConfigBuilderPy.file_backup()`]. Call [`FileBackupBuilderPy.build()`]
/// to finalize. Requires `disk_backup_policy.backups_dir` to be set.
#[gen_stub_pyclass]
#[pyclass]
pub struct FileBackupBuilderPy {
    base: StreamConfigBuilderPy,
    #[pyo3(get, set)]
    pub disk_backup_policy: DiskBackupPolicyPy,
    #[pyo3(get, set)]
    pub backup_data_channel_capacity: usize,
    #[pyo3(get, set)]
    pub control_channel_capacity: usize,
    #[pyo3(get, set)]
    pub metrics_streaming_interval: Option<DurationPy>,
}

#[gen_stub_pymethods]
#[pymethods]
impl FileBackupBuilderPy {
    pub fn build(&self, py: Python) -> PyResult<Py<PyAny>> {
        let base = self.base.clone();
        let disk_backup_policy: DiskBackupPolicy = self.disk_backup_policy.clone().into();
        let backup_cap = self.backup_data_channel_capacity;
        let control_cap = self.control_channel_capacity;
        let metrics_interval = self
            .metrics_streaming_interval
            .map(std::time::Duration::from);

        let awaitable = pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let config_builder = make_stream_config_builder(base)?;

            let mode_builder = config_builder
                .file_backup()
                .disk_backup_policy(disk_backup_policy)
                .backup_data_channel_capacity(backup_cap)
                .control_channel_capacity(control_cap)
                .metrics_streaming_interval(metrics_interval);

            mode_builder
                .build()
                .await
                .map(SiftStreamPy::from)
                .map_err(|e| crate::error::SiftErrorWrapper(e).into())
        })?;

        Ok(awaitable.into())
    }
}
