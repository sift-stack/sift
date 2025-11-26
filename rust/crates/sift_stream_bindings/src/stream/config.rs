use crate::{
    error::SiftErrorWrapper,
    sift::metadata::MetadataPy,
    stream::{
        channel::{ChannelBitFieldElementPy, ChannelDataTypePy, ChannelEnumTypePy, ValuePy},
        request::IngestWithConfigDataStreamRequestWrapperPy,
        time::TimeValuePy,
    },
};
use pyo3::prelude::*;
use pyo3_stub_gen::derive::*;
use sift_rs::ingestion_configs::v2::{ChannelConfig, FlowConfig};
use sift_stream::{
    ChannelIndex, FlowBuilder, FlowDescriptor, FlowDescriptorBuilder, stream::run::RunSelector,
};
use sift_stream::{IngestionConfigForm, RunForm};
use std::collections::HashMap;
use std::sync::Arc;

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
pub struct ChannelIndexPy {
    inner: ChannelIndex,
}

#[gen_stub_pyclass]
#[pyclass]
pub struct FlowDescriptorBuilderPy {
    inner: Option<FlowDescriptorBuilder<String>>,
}

#[gen_stub_pyclass]
#[pyclass]
#[derive(Clone)]
pub struct FlowDescriptorPy {
    // Use Arc to make cloning cheap - cloning Arc just increments a reference count
    inner: Arc<FlowDescriptor<String>>,
}

#[gen_stub_pyclass]
#[pyclass]
pub struct FlowBuilderPy {
    // We store the descriptor to ensure it lives as long as FlowBuilderPy.
    // Since FlowDescriptorPy uses Arc internally, cloning is cheap (just increments a reference count).
    // The field is intentionally unused (we only need it for lifetime management).
    _descriptor: FlowDescriptorPy,
    builder: Option<FlowBuilder<'static, String>>,
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

impl From<sift_rs::ingestion_configs::v2::FlowConfig> for FlowConfigPy {
    fn from(config: sift_rs::ingestion_configs::v2::FlowConfig) -> Self {
        FlowConfigPy {
            inner: config.clone(),
            name: config.name,
            channels: config.channels.into_iter().map(|c| c.into()).collect(),
        }
    }
}

impl From<ChannelConfig> for ChannelConfigPy {
    fn from(config: ChannelConfig) -> Self {
        let data_type_py = config.data_type().into();
        ChannelConfigPy {
            inner: config.clone(),
            name: config.name,
            unit: config.unit,
            description: config.description,
            data_type: data_type_py,
            enum_types: config.enum_types.into_iter().map(|ce| ce.into()).collect(),
            bit_field_elements: config
                .bit_field_elements
                .into_iter()
                .map(|bfe| bfe.into())
                .collect(),
        }
    }
}

impl From<FlowDescriptor<String>> for FlowDescriptorPy {
    fn from(descriptor: FlowDescriptor<String>) -> Self {
        FlowDescriptorPy {
            inner: Arc::new(descriptor),
        }
    }
}

impl From<ChannelIndexPy> for ChannelIndex {
    fn from(index: ChannelIndexPy) -> Self {
        index.inner
    }
}

impl From<ChannelIndex> for ChannelIndexPy {
    fn from(index: ChannelIndex) -> Self {
        ChannelIndexPy { inner: index }
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

#[gen_stub_pymethods]
#[pymethods]
impl FlowDescriptorBuilderPy {
    #[new]
    pub fn new(ingestion_config_id: &str, name: &str) -> Self {
        Self {
            inner: Some(FlowDescriptorBuilder::new(ingestion_config_id, name)),
        }
    }

    /// Adds a new channel to the flow.
    ///
    /// This returns the index of the channel in the flow.
    pub fn add(&mut self, key: &str, field_type: ChannelDataTypePy) -> PyResult<ChannelIndexPy> {
        let Some(builder) = self.inner.as_mut() else {
            return Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(
                "Builder has already been consumed",
            ));
        };
        Ok(builder.add(key.to_string(), field_type.into()).into())
    }

    /// Builds the FlowDescriptor from the builder.
    pub fn build(&mut self) -> PyResult<FlowDescriptorPy> {
        let Some(builder) = self.inner.take() else {
            return Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(
                "Builder has already been consumed",
            ));
        };
        Ok(builder.build().into())
    }
}

#[gen_stub_pymethods]
#[pymethods]
impl FlowDescriptorPy {
    /// Gets the type of the channel with the given key.
    pub fn get(&self, key: &str) -> Option<ChannelDataTypePy> {
        self.inner.as_ref().get(key).map(|dt| dt.into())
    }

    /// Gets the mapping of keys to channel indices.
    pub fn mapping(&self) -> HashMap<String, ChannelIndexPy> {
        self.inner
            .as_ref()
            .mapping()
            .iter()
            .map(|(k, v)| (k.clone(), (*v).into()))
            .collect()
    }
}

#[gen_stub_pymethods]
#[pymethods]
impl FlowBuilderPy {
    #[new]
    pub fn new(descriptor: FlowDescriptorPy) -> Self {
        // Since FlowDescriptorPy uses Arc internally, cloning is cheap (just increments a reference count).
        // We can safely create a 'static reference because:
        // 1. We store `descriptor` in the same struct, ensuring it lives as long as `FlowBuilderPy`
        // 2. The `FlowBuilder` only holds a reference and doesn't outlive `FlowBuilderPy`
        // 3. The reference is never used after `FlowBuilderPy` is dropped
        // 4. The descriptor's inner `FlowDescriptor<String>` is wrapped in Arc, making it shareable
        //
        // SAFETY: We extend the lifetime of the reference to 'static using transmute.
        // This is safe because the Arc ensures the data lives as long as needed.
        let descriptor_ref: &'static FlowDescriptor<String> = unsafe {
            // Get a reference to the inner FlowDescriptor through the Arc
            std::mem::transmute(&*Arc::as_ptr(&descriptor.inner))
        };
        let flow_builder = FlowBuilder::new(descriptor_ref);
        Self {
            _descriptor: descriptor,
            builder: Some(flow_builder),
        }
    }

    /// Attaches a run ID to the flow.
    pub fn attach_run_id(&mut self, run_id: &str) -> PyResult<()> {
        let Some(builder) = self.builder.as_mut() else {
            return Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(
                "Builder has already been consumed",
            ));
        };
        builder.attach_run_id(run_id);
        Ok(())
    }

    /// Sets the value of the channel with the given index.
    pub fn set(&mut self, index: ChannelIndexPy, value: ValuePy) -> PyResult<()> {
        let Some(builder) = self.builder.as_mut() else {
            return Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(
                "Builder has already been consumed",
            ));
        };
        match builder.set(index.into(), value) {
            Ok(()) => Ok(()),
            Err(e) => Err(SiftErrorWrapper(e).into()),
        }
    }

    /// Sets the value of the channel with the given key.
    pub fn set_with_key(&mut self, key: &str, value: ValuePy) -> PyResult<()> {
        let Some(builder) = self.builder.as_mut() else {
            return Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(
                "Builder has already been consumed",
            ));
        };
        match builder.set_with_key(key, value) {
            Ok(()) => Ok(()),
            Err(e) => Err(SiftErrorWrapper(e).into()),
        }
    }

    /// Builds an IngestWithConfigDataStreamRequest, consuming the builder.
    pub fn request(
        &mut self,
        now: TimeValuePy,
    ) -> PyResult<IngestWithConfigDataStreamRequestWrapperPy> {
        let Some(builder) = self.builder.take() else {
            return Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(
                "Builder has already been consumed",
            ));
        };
        Ok(builder.request(now.into()).into())
    }
}
