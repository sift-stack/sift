use pyo3::prelude::*;
use pyo3_stub_gen::define_stub_info_gatherer;
use tracing::Level;
use tracing_subscriber::{Layer, filter, layer::SubscriberExt};
use std::sync::{Once, Mutex};

mod error;
mod stream;

define_stub_info_gatherer!(stub_info);

static INIT_TRACING: Once = Once::new();
static FILE_APPENDER_GUARD: Mutex<Option<tracing_appender::non_blocking::WorkerGuard>> = Mutex::new(None);

/// Initialize tracing to stdout for Python environments.
/// This function can be called multiple times safely.
#[pyfunction]
fn init_tracing() -> PyResult<()> {
    INIT_TRACING.call_once(|| {
        let filter = filter::Targets::new()
            .with_target("sift_stream", Level::TRACE)
            .with_default(Level::INFO);

        let stdout_fmt_layer = tracing_subscriber::fmt::layer().with_filter(filter.clone());

        let subscriber = tracing_subscriber::Registry::default()
            .with(stdout_fmt_layer);

        tracing::subscriber::set_global_default(subscriber)
            .expect("Unable to setup tracing");
    });
    Ok(())
}

/// Initialize tracing to output and rolling logs for Python environments.
/// This function can be called multiple times safely.
#[pyfunction]
fn init_tracing_with_file() -> PyResult<()> {
    INIT_TRACING.call_once(|| {
        let filter = filter::Targets::new()
            .with_target("sift_stream", Level::TRACE)
            .with_default(Level::INFO);

        let stdout_fmt_layer = tracing_subscriber::fmt::layer().with_filter(filter.clone());

        let file_appender = tracing_appender::rolling::Builder::new()
            .rotation(tracing_appender::rolling::Rotation::DAILY)
            .filename_prefix("soak_test_py_bindings.log")
            .max_log_files(7)
            .build("./logs")
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

        tracing::subscriber::set_global_default(subscriber)
            .expect("Unable to setup tracing");
    });
    Ok(())
}

#[pymodule]
fn sift_stream_bindings(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<stream::SiftStreamPy>()?;
    m.add_class::<stream::FlowPy>()?;
    m.add_class::<stream::builder::SiftStreamBuilderPy>()?;
    m.add_class::<stream::config::IngestionConfigFormPy>()?;
    m.add_class::<stream::config::FlowConfigPy>()?;
    m.add_class::<stream::config::ChannelConfigPy>()?;
    m.add_class::<stream::channel::ChannelDataTypePy>()?;
    m.add_class::<stream::channel::ChannelEnumTypePy>()?;
    m.add_class::<stream::channel::ChannelBitFieldElementPy>()?;
    m.add_class::<stream::retry::DurationPy>()?;
    m.add_class::<stream::retry::RecoveryStrategyPy>()?;
    m.add_class::<stream::retry::RetryPolicyPy>()?;
    m.add_class::<stream::config::RunFormPy>()?;
    m.add_class::<stream::time::TimeValuePy>()?;
    m.add_class::<stream::channel::ChannelValuePy>()?;
    m.add_class::<stream::channel::ChannelValueTypePy>()?;
    m.add_class::<stream::request::IngestWithConfigDataStreamRequestPy>()?;
    m.add_class::<stream::request::IngestWithConfigDataChannelValuePy>()?;
    m.add_function(wrap_pyfunction!(init_tracing, m)?)?;
    m.add_function(wrap_pyfunction!(init_tracing_with_file, m)?)?;
    Ok(())
}
