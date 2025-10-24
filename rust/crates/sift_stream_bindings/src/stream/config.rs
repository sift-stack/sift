use crate::{
    sift::metadata::MetadataPy,
    stream::channel::{ChannelBitFieldElementPy, ChannelDataTypePy, ChannelEnumTypePy},
};
use pyo3::prelude::*;
use pyo3_stub_gen::derive::*;
use sift_rs::ingestion_configs::v2::{ChannelConfig, FlowConfig};
use sift_stream::stream::run::RunSelector;
use sift_stream::{IngestionConfigForm, RunForm};

// Type Definitions
#[gen_stub_pyclass]
#[pyclass]
#[derive(Clone)]
pub struct ChannelConfigPy {
    inner: ChannelConfig,
    #[pyo3(get, set)]
    name: String,
    #[pyo3(get, set)]
    unit: String,
    #[pyo3(get, set)]
    description: String,
    #[pyo3(get, set)]
    data_type: ChannelDataTypePy,
    #[pyo3(get, set)]
    enum_types: Vec<ChannelEnumTypePy>,
    #[pyo3(get, set)]
    bit_field_elements: Vec<ChannelBitFieldElementPy>,
}

#[gen_stub_pyclass]
#[pyclass]
#[derive(Clone)]
pub struct FlowConfigPy {
    inner: FlowConfig,
    #[pyo3(get, set)]
    name: String,
    #[pyo3(get, set)]
    channels: Vec<ChannelConfigPy>,
}

#[gen_stub_pyclass]
#[pyclass]
#[derive(Clone)]
pub struct IngestionConfigFormPy {
    #[pyo3(get, set)]
    asset_name: String,
    #[pyo3(get, set)]
    flows: Vec<FlowConfigPy>,
    #[pyo3(get, set)]
    client_key: String,
}

#[gen_stub_pyclass]
#[pyclass]
#[derive(Clone)]
pub struct RunFormPy {
    #[pyo3(get, set)]
    name: String,
    #[pyo3(get, set)]
    client_key: String,
    #[pyo3(get, set)]
    description: Option<String>,
    #[pyo3(get, set)]
    tags: Option<Vec<String>>,
    #[pyo3(get, set)]
    metadata: Option<Vec<MetadataPy>>,
}

#[gen_stub_pyclass]
#[pyclass]
#[derive(Clone)]
pub struct RunSelectorPy {
    run_id: Option<String>,
    run_form: Option<RunFormPy>,
}

// Trait Implementations
impl From<RunSelectorPy> for RunSelector {
    fn from(selector: RunSelectorPy) -> Self {
        if let Some(run_id) = selector.run_id {
            RunSelector::ById(run_id)
        } else if let Some(form) = selector.run_form {
            RunSelector::ByForm(form.into())
        } else {
            // This shouldn't happen if constructed correctly
            panic!("Invalid RunSelectorPy: must have either run_id or run_form")
        }
    }
}

impl IngestionConfigFormPy {
    pub fn to_inner(&self) -> IngestionConfigForm {
        IngestionConfigForm {
            asset_name: self.asset_name.clone(),
            client_key: self.client_key.clone(),
            flows: self.flows.iter().map(|f| f.inner.clone()).collect(),
        }
    }
}

impl From<IngestionConfigFormPy> for IngestionConfigForm {
    fn from(config: IngestionConfigFormPy) -> Self {
        config.to_inner()
    }
}

impl From<RunFormPy> for RunForm {
    fn from(form: RunFormPy) -> Self {
        let metadata = form
            .metadata
            .map(|v| v.into_iter().map(|m| m.into()).collect());

        RunForm {
            name: form.name,
            client_key: form.client_key,
            description: form.description,
            tags: form.tags,
            metadata,
        }
    }
}

impl From<FlowConfigPy> for sift_rs::ingestion_configs::v2::FlowConfig {
    fn from(config: FlowConfigPy) -> Self {
        config.inner
    }
}

// PyO3 Method Implementations
#[gen_stub_pymethods]
#[pymethods]
impl ChannelConfigPy {
    #[new]
    pub fn new(
        name: &str,
        unit: &str,
        description: &str,
        data_type: ChannelDataTypePy,
        enum_types: Vec<ChannelEnumTypePy>,
        bit_field_elements: Vec<ChannelBitFieldElementPy>,
    ) -> Self {
        Self {
            inner: ChannelConfig {
                name: name.to_string(),
                unit: unit.to_string(),
                description: description.to_string(),
                data_type: <ChannelDataTypePy as Into<
                    sift_rs::common::r#type::v1::ChannelDataType,
                >>::into(data_type.clone()) as i32,
                enum_types: enum_types.iter().map(|e| e.inner.clone()).collect(),
                bit_field_elements: bit_field_elements.iter().map(|b| b.inner.clone()).collect(),
            },
            name: name.to_string(),
            unit: unit.to_string(),
            description: description.to_string(),
            data_type,
            enum_types,
            bit_field_elements,
        }
    }
}

#[gen_stub_pymethods]
#[pymethods]
impl FlowConfigPy {
    #[new]
    pub fn new(name: &str, channels: Vec<ChannelConfigPy>) -> Self {
        Self {
            inner: FlowConfig {
                name: name.to_string(),
                channels: channels.iter().map(|c| c.inner.clone()).collect(),
            },
            name: name.to_string(),
            channels,
        }
    }
}

#[gen_stub_pymethods]
#[pymethods]
impl IngestionConfigFormPy {
    #[new]
    pub fn new(asset_name: &str, client_key: &str, flows: Vec<FlowConfigPy>) -> Self {
        Self {
            asset_name: asset_name.to_string(),
            client_key: client_key.to_string(),
            flows,
        }
    }
}

#[gen_stub_pymethods]
#[pymethods]
impl RunFormPy {
    #[new]
    pub fn new(
        name: &str,
        client_key: &str,
        description: Option<&str>,
        tags: Option<Vec<String>>,
        metadata: Option<Vec<MetadataPy>>,
    ) -> Self {
        Self {
            name: name.to_string(),
            client_key: client_key.to_string(),
            description: description.map(|s| s.to_string()),
            tags,
            metadata,
        }
    }
}

#[gen_stub_pymethods]
#[pymethods]
impl RunSelectorPy {
    #[staticmethod]
    pub fn by_id(run_id: String) -> Self {
        Self {
            run_id: Some(run_id),
            run_form: None,
        }
    }

    #[staticmethod]
    pub fn by_form(form: RunFormPy) -> Self {
        Self {
            run_id: None,
            run_form: Some(form),
        }
    }
}
