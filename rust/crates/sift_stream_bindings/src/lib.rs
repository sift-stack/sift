use pyo3::prelude::*;
use pyo3_stub_gen::define_stub_info_gatherer;

mod error;
mod metrics;
mod sift;
mod stream;

define_stub_info_gatherer!(stub_info);

// Consider reorganizing in the future. For now keep all classes within the sift_stream_bindings module
// See below for issues with submodules using pyo3
// https://github.com/PyO3/pyo3/issues/759
#[pymodule]
fn sift_stream_bindings(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<sift::metadata::MetadataPy>()?;
    m.add_class::<metrics::SiftStreamMetricsSnapshotPy>()?;
    m.add_class::<metrics::CheckpointMetricsSnapshotPy>()?;
    m.add_class::<metrics::BackupMetricsSnapshotPy>()?;
    m.add_class::<stream::FlowPy>()?;
    m.add_class::<stream::SiftStreamPy>()?;
    m.add_class::<stream::builder::SiftStreamBuilderPy>()?;
    m.add_class::<stream::config::ChannelConfigPy>()?;
    m.add_class::<stream::config::FlowConfigPy>()?;
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
    m.add_class::<stream::time::TimeValuePy>()?;
    
    Ok(())
}
