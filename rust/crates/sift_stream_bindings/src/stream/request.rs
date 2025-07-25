use crate::stream::channel::ChannelValueTypePy;
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
    pub channel_values: Vec<IngestWithConfigDataChannelValuePy>,
    #[pyo3(get, set)]
    pub run_id: String,
    #[pyo3(get, set)]
    pub end_stream_on_validation_error: bool,
    #[pyo3(get, set)]
    pub organization_id: String,
}

#[gen_stub_pyclass]
#[pyclass]
#[derive(Clone)]
pub struct IngestWithConfigDataChannelValuePy {
    #[pyo3(get, set)]
    pub r#type: ChannelValueTypePy,
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
                    r#type: Some(v.r#type.into()),
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
        channel_values: Vec<IngestWithConfigDataChannelValuePy>,
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

#[gen_stub_pymethods]
#[pymethods]
impl IngestWithConfigDataChannelValuePy {
    #[staticmethod]
    pub fn bool(value: bool) -> Self {
        Self {
            r#type: ChannelValueTypePy::bool(value),
        }
    }

    #[staticmethod]
    pub fn string(value: String) -> Self {
        Self {
            r#type: ChannelValueTypePy::string(value),
        }
    }

    #[staticmethod]
    pub fn float(value: f32) -> Self {
        Self {
            r#type: ChannelValueTypePy::float(value),
        }
    }

    #[staticmethod]
    pub fn double(value: f64) -> Self {
        Self {
            r#type: ChannelValueTypePy::double(value),
        }
    }

    #[staticmethod]
    pub fn int32(value: i32) -> Self {
        Self {
            r#type: ChannelValueTypePy::int32(value),
        }
    }

    #[staticmethod]
    pub fn uint32(value: u32) -> Self {
        Self {
            r#type: ChannelValueTypePy::uint32(value),
        }
    }

    #[staticmethod]
    pub fn int64(value: i64) -> Self {
        Self {
            r#type: ChannelValueTypePy::int64(value),
        }
    }

    #[staticmethod]
    pub fn uint64(value: u64) -> Self {
        Self {
            r#type: ChannelValueTypePy::uint64(value),
        }
    }

    #[staticmethod]
    pub fn enum_value(value: u32) -> Self {
        Self {
            r#type: ChannelValueTypePy::enum_value(value),
        }
    }

    #[staticmethod]
    pub fn bitfield(value: Vec<u8>) -> Self {
        Self {
            r#type: ChannelValueTypePy::bitfield(value),
        }
    }

    #[staticmethod]
    pub fn empty() -> Self {
        Self {
            r#type: ChannelValueTypePy::empty(),
        }
    }
}
