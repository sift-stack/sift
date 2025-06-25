use crate::stream::channel::{ChannelValuePy, ChannelValueTypePy};
use crate::stream::time::TimeValuePy;
use pyo3::prelude::*;
use pyo3_stub_gen::derive::*;
use sift_rs::ingest::v1::{IngestWithConfigDataChannelValue, IngestWithConfigDataStreamRequest};

// Type Definitions
#[gen_stub_pyclass]
#[pyclass]
#[derive(Clone)]
pub struct IngestWithConfigDataStreamRequestPy {
    #[pyo3(get, set)]
    pub ingestion_config_id: String,
    #[pyo3(get, set)]
    pub flow: String,
    #[pyo3(get, set)]
    pub timestamp: Option<TimeValuePy>,
    #[pyo3(get, set)]
    pub channel_values: Vec<ChannelValuePy>,
    #[pyo3(get, set)]
    pub run_id: String,
    #[pyo3(get, set)]
    pub end_stream_on_validation_error: bool,
    #[pyo3(get, set)]
    pub organization_id: String,
}

// Trait Implementations
impl From<IngestWithConfigDataStreamRequestPy> for IngestWithConfigDataStreamRequest {
    fn from(request: IngestWithConfigDataStreamRequestPy) -> Self {
        IngestWithConfigDataStreamRequest {
            ingestion_config_id: request.ingestion_config_id,
            flow: request.flow,
            timestamp: request.timestamp.map(|t| *t.inner),
            channel_values: request
                .channel_values
                .into_iter()
                .map(|v| IngestWithConfigDataChannelValue {
                    r#type: Some(ChannelValueTypePy::from(v.inner.value).into()),
                })
                .collect(),
            run_id: request.run_id,
            end_stream_on_validation_error: request.end_stream_on_validation_error,
            organization_id: request.organization_id,
        }
    }
}

// PyO3 Method Implementations
#[gen_stub_pymethods]
#[pymethods]
impl IngestWithConfigDataStreamRequestPy {
    #[new]
    pub fn new(
        ingestion_config_id: String,
        flow: String,
        timestamp: Option<TimeValuePy>,
        channel_values: Vec<ChannelValuePy>,
        run_id: String,
        end_stream_on_validation_error: bool,
        organization_id: String,
    ) -> Self {
        Self {
            ingestion_config_id,
            flow,
            timestamp,
            channel_values,
            run_id,
            end_stream_on_validation_error,
            organization_id,
        }
    }
}
