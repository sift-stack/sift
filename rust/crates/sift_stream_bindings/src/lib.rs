use pyo3::prelude::*;
use pyo3_stub_gen::define_stub_info_gatherer;
use std::sync::{Mutex, Once};
use tracing::Level;
use tracing_subscriber::{Layer, filter, layer::SubscriberExt};

mod error;
mod metrics;
mod sift;
mod stream;

define_stub_info_gatherer!(stub_info);

static INIT_TRACING: Once = Once::new();
static FILE_APPENDER_GUARD: Mutex<Option<tracing_appender::non_blocking::WorkerGuard>> =
    Mutex::new(None);

/// Initialize tracing to stdout for Python environments.
/// Can only be called once per process. Subsequent calls will raise an error.
/// Call `is_tracing_initialized()` to check if already initialized.
///
/// Args:
///     level: Logging level as string - one of "trace", "debug", "info", "warn", "error" (default: "info")
#[pyfunction]
#[pyo3(signature = (level = "info"))]
fn init_tracing(level: &str) -> PyResult<()> {
    if INIT_TRACING.is_completed() {
        return Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(
            "Tracing has already been initialized. It can only be initialized once per process.",
        ));
    }

    let parsed_level = match level.to_lowercase().as_str() {
        "trace" => Level::TRACE,
        "debug" => Level::DEBUG,
        "info" => Level::INFO,
        "warn" => Level::WARN,
        "error" => Level::ERROR,
        _ => {
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
                "Invalid log level '{}'. Must be one of: trace, debug, info, warn, error",
                level
            )));
        }
    };

    INIT_TRACING.call_once(|| {
        let filter = filter::Targets::new()
            .with_target("sift_stream", parsed_level)
            .with_default(Level::WARN);

        let stdout_fmt_layer = tracing_subscriber::fmt::layer().with_filter(filter.clone());

        let subscriber = tracing_subscriber::Registry::default().with(stdout_fmt_layer);

        tracing::subscriber::set_global_default(subscriber).expect("Unable to setup tracing");
    });
    Ok(())
}

/// Initialize tracing to output and rolling logs for Python environments.
/// Can only be called once per process. Subsequent calls will raise an error.
/// Call `is_tracing_initialized()` to check if already initialized.
///
/// Args:
///     level: Logging level as string - one of "trace", "debug", "info", "warn", "error" (default: "info")
///     log_dir: Directory path for log files (default: "./logs")
///     filename_prefix: Prefix for log filenames (default: "sift_stream_bindings.log")
///     max_log_files: Maximum number of log files to keep (default: 7)
#[pyfunction]
#[pyo3(signature = (level = "info", log_dir = "./logs", filename_prefix = "sift_stream_bindings.log", max_log_files = 7))]
fn init_tracing_with_file(
    level: &str,
    log_dir: &str,
    filename_prefix: &str,
    max_log_files: usize,
) -> PyResult<()> {
    if INIT_TRACING.is_completed() {
        return Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(
            "Tracing has already been initialized. It can only be initialized once per process.",
        ));
    }

    let parsed_level = match level.to_lowercase().as_str() {
        "trace" => Level::TRACE,
        "debug" => Level::DEBUG,
        "info" => Level::INFO,
        "warn" => Level::WARN,
        "error" => Level::ERROR,
        _ => {
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
                "Invalid log level '{}'. Must be one of: trace, debug, info, warn, error",
                level
            )));
        }
    };

    INIT_TRACING.call_once(|| {
        let filter = filter::Targets::new()
            .with_target("sift_stream", parsed_level)
            .with_default(Level::WARN);

        let stdout_fmt_layer = tracing_subscriber::fmt::layer().with_filter(filter.clone());

        let file_appender = tracing_appender::rolling::Builder::new()
            .rotation(tracing_appender::rolling::Rotation::DAILY)
            .filename_prefix(filename_prefix)
            .max_log_files(max_log_files)
            .build(log_dir)
            .expect("Unable to set up file appender");
        let (non_blocking_appender, guard) = tracing_appender::non_blocking(file_appender);

        // Store the guard to prevent it from being dropped
        if let Ok(mut guard_storage) = FILE_APPENDER_GUARD.lock() {
            *guard_storage = Some(guard);
        }

        // Create file layer
        let log_layer = tracing_subscriber::fmt::layer()
            .with_ansi(false)
            .with_writer(non_blocking_appender)
            .with_filter(filter);

        let subscriber = tracing_subscriber::Registry::default()
            .with(stdout_fmt_layer)
            .with(log_layer);

        tracing::subscriber::set_global_default(subscriber).expect("Unable to setup tracing");
    });
    Ok(())
}

#[pyfunction]
fn is_tracing_initialized() -> bool {
    INIT_TRACING.is_completed()
}

// Cannot organize into submodules right now
// See below for issues with submodules using pyo3
// https://github.com/PyO3/pyo3/issues/759
#[pymodule]
fn sift_stream_bindings(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<sift::metadata::MetadataPy>()?;
    m.add_class::<sift::metadata::MetadataValuePy>()?;
    m.add_class::<metrics::SiftStreamMetricsSnapshotPy>()?;
    m.add_class::<metrics::CheckpointMetricsSnapshotPy>()?;
    m.add_class::<metrics::BackupMetricsSnapshotPy>()?;
    m.add_class::<stream::FlowPy>()?;
    m.add_class::<stream::SiftStreamPy>()?;
    m.add_class::<stream::builder::SiftStreamBuilderPy>()?;
    m.add_class::<stream::config::ChannelConfigPy>()?;
    m.add_class::<stream::config::ChannelIndexPy>()?;
    m.add_class::<stream::config::FlowConfigPy>()?;
    m.add_class::<stream::config::FlowDescriptorBuilderPy>()?;
    m.add_class::<stream::config::FlowDescriptorPy>()?;
    m.add_class::<stream::config::FlowBuilderPy>()?;
    m.add_class::<stream::config::IngestionConfigFormPy>()?;
    m.add_class::<stream::config::RunFormPy>()?;
    m.add_class::<stream::config::RunSelectorPy>()?;
    m.add_class::<stream::channel::ChannelBitFieldElementPy>()?;
    m.add_class::<stream::channel::ChannelDataTypePy>()?;
    m.add_class::<stream::channel::ChannelEnumPy>()?;
    m.add_class::<stream::channel::ChannelEnumTypePy>()?;
    m.add_class::<stream::channel::ChannelValuePy>()?;
    m.add_class::<stream::channel::ChannelValueTypePy>()?;
    m.add_class::<stream::channel::ValuePy>()?;
    m.add_class::<stream::retry::DiskBackupPolicyPy>()?;
    m.add_class::<stream::retry::DurationPy>()?;
    m.add_class::<stream::retry::RecoveryStrategyPy>()?;
    m.add_class::<stream::retry::RetryPolicyPy>()?;
    m.add_class::<stream::retry::RollingFilePolicyPy>()?;
    m.add_class::<stream::request::IngestWithConfigDataChannelValuePy>()?;
    m.add_class::<stream::request::IngestWithConfigDataStreamRequestPy>()?;
    m.add_class::<stream::request::IngestWithConfigDataStreamRequestWrapperPy>()?;
    m.add_class::<stream::time::TimeValuePy>()?;
    m.add_function(wrap_pyfunction!(init_tracing, m)?)?;
    m.add_function(wrap_pyfunction!(init_tracing_with_file, m)?)?;
    m.add_function(wrap_pyfunction!(is_tracing_initialized, m)?)?;
    Ok(())
}
