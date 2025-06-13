use pyo3::prelude::*;
use sift_error::Error as SiftError;
use sift_rs::common::r#type::v1::{ChannelBitFieldElement, ChannelDataType, ChannelEnumType};
use sift_rs::ingest::v1::ingest_with_config_data_channel_value::Type as ChannelValueType;
use sift_rs::ingest::v1::{IngestWithConfigDataChannelValue, IngestWithConfigDataStreamRequest};
use sift_rs::ingestion_configs::v2::{ChannelConfig, FlowConfig};
use sift_stream::stream::channel::{ChannelValue, Value};
use sift_stream::stream::time::TimeValue;
use sift_stream::{
    Credentials, Flow, IngestionConfigForm, IngestionConfigMode, RecoveryStrategy, RetryPolicy,
    RunForm, SiftStream, SiftStreamBuilder,
};
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::Mutex;

// Newtype wrapper for sift_error::Error
#[derive(Debug)]
pub struct SiftErrorWrapper(pub SiftError);

impl From<SiftError> for SiftErrorWrapper {
    fn from(err: SiftError) -> Self {
        Self(err)
    }
}

impl From<SiftErrorWrapper> for PyErr {
    fn from(err: SiftErrorWrapper) -> Self {
        PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("{}", err.0))
    }
}

#[pyclass]
#[derive(Clone)]
pub struct ChannelBitFieldElementPy {
    inner: ChannelBitFieldElement,
    #[pyo3(get, set)]
    name: String,
    #[pyo3(get, set)]
    index: i32,
    #[pyo3(get, set)]
    bit_count: u32,
}

#[pymethods]
impl ChannelBitFieldElementPy {
    #[new]
    pub fn new(name: &str, index: i32, bit_count: u32) -> Self {
        Self {
            inner: ChannelBitFieldElement {
                name: name.to_string(),
                index,
                bit_count,
            },
            name: name.to_string(),
            index,
            bit_count,
        }
    }
}

impl From<ChannelBitFieldElementPy> for ChannelBitFieldElement {
    fn from(value: ChannelBitFieldElementPy) -> Self {
        value.inner
    }
}

#[pyclass]
#[derive(Clone)]
pub struct ChannelEnumTypePy {
    inner: ChannelEnumType,
    #[pyo3(get, set)]
    name: String,
    #[pyo3(get, set)]
    key: u32,
}

#[pymethods]
impl ChannelEnumTypePy {
    #[new]
    pub fn new(name: &str, key: u32) -> Self {
        Self {
            inner: ChannelEnumType {
                name: name.to_string(),
                key,
            },
            name: name.to_string(),
            key,
        }
    }
}

impl From<ChannelEnumTypePy> for ChannelEnumType {
    fn from(value: ChannelEnumTypePy) -> Self {
        value.inner
    }
}

#[pyclass]
#[derive(Clone)]
pub enum ChannelDataTypePy {
    Unspecified,
    Double,
    String,
    Enum,
    BitField,
    Bool,
    Float,
    Int32,
    Uint32,
    Int64,
    Uint64,
    Bytes,
}

impl From<ChannelDataType> for ChannelDataTypePy {
    fn from(data_type: ChannelDataType) -> Self {
        match data_type {
            ChannelDataType::Unspecified => ChannelDataTypePy::Unspecified,
            ChannelDataType::Double => ChannelDataTypePy::Double,
            ChannelDataType::String => ChannelDataTypePy::String,
            ChannelDataType::Enum => ChannelDataTypePy::Enum,
            ChannelDataType::BitField => ChannelDataTypePy::BitField,
            ChannelDataType::Bool => ChannelDataTypePy::Bool,
            ChannelDataType::Float => ChannelDataTypePy::Float,
            ChannelDataType::Int32 => ChannelDataTypePy::Int32,
            ChannelDataType::Uint32 => ChannelDataTypePy::Uint32,
            ChannelDataType::Int64 => ChannelDataTypePy::Int64,
            ChannelDataType::Uint64 => ChannelDataTypePy::Uint64,
            ChannelDataType::Bytes => ChannelDataTypePy::Bytes,
        }
    }
}

impl From<ChannelDataTypePy> for ChannelDataType {
    fn from(data_type: ChannelDataTypePy) -> Self {
        match data_type {
            ChannelDataTypePy::Unspecified => ChannelDataType::Unspecified,
            ChannelDataTypePy::Double => ChannelDataType::Double,
            ChannelDataTypePy::String => ChannelDataType::String,
            ChannelDataTypePy::Enum => ChannelDataType::Enum,
            ChannelDataTypePy::BitField => ChannelDataType::BitField,
            ChannelDataTypePy::Bool => ChannelDataType::Bool,
            ChannelDataTypePy::Float => ChannelDataType::Float,
            ChannelDataTypePy::Int32 => ChannelDataType::Int32,
            ChannelDataTypePy::Uint32 => ChannelDataType::Uint32,
            ChannelDataTypePy::Int64 => ChannelDataType::Int64,
            ChannelDataTypePy::Uint64 => ChannelDataType::Uint64,
            ChannelDataTypePy::Bytes => ChannelDataType::Bytes,
        }
    }
}

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
                data_type: <ChannelDataTypePy as Into<ChannelDataType>>::into(data_type.clone())
                    as i32,
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

#[pyclass]
#[derive(Clone)]
pub struct FlowConfigPy {
    inner: FlowConfig,
    #[pyo3(get, set)]
    name: String,
    #[pyo3(get, set)]
    channels: Vec<ChannelConfigPy>,
}

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

#[pyclass]
#[derive(Clone, Copy)]
pub struct DurationPy {
    #[pyo3(get, set)]
    secs: u64,
    #[pyo3(get, set)]
    nanos: u32,
}

#[pymethods]
impl DurationPy {
    #[new]
    pub fn new(secs: u64, nanos: u32) -> Self {
        Self { secs, nanos }
    }
}

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

#[pyclass]
#[derive(Clone)]
pub struct RecoveryStrategyPy {
    #[pyo3(get, set)]
    strategy_type: String,
    #[pyo3(get, set)]
    retry_policy: Option<RetryPolicyPy>,
    #[pyo3(get, set)]
    max_buffer_size: Option<usize>,
    #[pyo3(get, set)]
    backups_dir: Option<String>,
    #[pyo3(get, set)]
    max_backups_file_size: Option<usize>,
}

#[pymethods]
impl RecoveryStrategyPy {
    #[new]
    pub fn new(
        strategy_type: &str,
        retry_policy: Option<RetryPolicyPy>,
        max_buffer_size: Option<usize>,
        backups_dir: Option<String>,
        max_backups_file_size: Option<usize>,
    ) -> Self {
        Self {
            strategy_type: strategy_type.to_string(),
            retry_policy,
            max_buffer_size,
            backups_dir,
            max_backups_file_size,
        }
    }

    #[staticmethod]
    pub fn retry_only(retry_policy: RetryPolicyPy) -> Self {
        Self {
            strategy_type: "RetryOnly".to_string(),
            retry_policy: Some(retry_policy),
            max_buffer_size: None,
            backups_dir: None,
            max_backups_file_size: None,
        }
    }

    #[staticmethod]
    pub fn retry_with_in_memory_backups(
        retry_policy: RetryPolicyPy,
        max_buffer_size: Option<usize>,
    ) -> Self {
        Self {
            strategy_type: "RetryWithInMemoryBackups".to_string(),
            retry_policy: Some(retry_policy),
            max_buffer_size,
            backups_dir: None,
            max_backups_file_size: None,
        }
    }

    #[staticmethod]
    pub fn retry_with_disk_backups(
        retry_policy: RetryPolicyPy,
        backups_dir: Option<String>,
        max_backups_file_size: Option<usize>,
    ) -> Self {
        Self {
            strategy_type: "RetryWithDiskBackups".to_string(),
            retry_policy: Some(retry_policy),
            max_buffer_size: None,
            backups_dir,
            max_backups_file_size,
        }
    }

    #[staticmethod]
    pub fn default() -> Self {
        Self::retry_only(RetryPolicyPy::default())
    }

    #[staticmethod]
    pub fn default_retry_policy_in_memory_backups() -> Self {
        Self::retry_with_in_memory_backups(RetryPolicyPy::default(), None)
    }

    #[staticmethod]
    pub fn default_retry_policy_disk_backups() -> Self {
        Self::retry_with_disk_backups(RetryPolicyPy::default(), None, None)
    }
}

impl From<RecoveryStrategyPy> for RecoveryStrategy {
    fn from(strategy: RecoveryStrategyPy) -> Self {
        match strategy.strategy_type.as_str() {
            "RetryOnly" => RecoveryStrategy::RetryOnly(strategy.retry_policy.unwrap().into()),
            "RetryWithInMemoryBackups" => RecoveryStrategy::RetryWithInMemoryBackups {
                retry_policy: strategy.retry_policy.unwrap().into(),
                max_buffer_size: strategy.max_buffer_size,
            },
            "RetryWithDiskBackups" => RecoveryStrategy::RetryWithDiskBackups {
                retry_policy: strategy.retry_policy.unwrap().into(),
                backups_dir: strategy.backups_dir.map(PathBuf::from),
                max_backups_file_size: strategy.max_backups_file_size,
            },
            _ => panic!("Invalid strategy type"),
        }
    }
}

#[pyclass]
#[derive(Clone)]
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
        Self {
            max_attempts: 5,
            initial_backoff: DurationPy::new(0, 50_000_000), // 50ms
            max_backoff: DurationPy::new(5, 0),              // 5s
            backoff_multiplier: 5,
        }
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
}

#[pymethods]
impl RunFormPy {
    #[new]
    pub fn new(
        name: &str,
        client_key: &str,
        description: Option<&str>,
        tags: Option<Vec<String>>,
    ) -> Self {
        Self {
            name: name.to_string(),
            client_key: client_key.to_string(),
            description: description.map(|s| s.to_string()),
            tags,
        }
    }
}

impl From<RunFormPy> for RunForm {
    fn from(form: RunFormPy) -> Self {
        RunForm {
            name: form.name,
            client_key: form.client_key,
            description: form.description,
            tags: form.tags,
        }
    }
}

#[pyclass]
pub struct SiftStreamBuilderPy {
    inner: Arc<Mutex<Option<SiftStreamBuilder<IngestionConfigMode>>>>,
    #[pyo3(get, set)]
    uri: String,
    #[pyo3(get, set)]
    apikey: String,
    #[pyo3(get, set)]
    enable_tls: bool,
    #[pyo3(get, set)]
    ingestion_config: Option<IngestionConfigFormPy>,
    #[pyo3(get, set)]
    recovery_strategy: Option<RecoveryStrategyPy>,
    #[pyo3(get, set)]
    checkpoint_interval: DurationPy,
    #[pyo3(get, set)]
    run: Option<RunFormPy>,
    #[pyo3(get, set)]
    run_id: Option<String>,
}

#[pymethods]
impl SiftStreamBuilderPy {
    #[new]
    pub fn new(uri: &str, apikey: &str) -> Self {
        Self {
            inner: Arc::new(Mutex::new(Some(SiftStreamBuilder::new(
                Credentials::Config {
                    uri: uri.into(),
                    apikey: apikey.into(),
                },
            )))),
            uri: uri.into(),
            apikey: apikey.into(),
            enable_tls: true,
            ingestion_config: None,
            recovery_strategy: None,
            checkpoint_interval: DurationPy::new(60, 0),
            run: None,
            run_id: None,
        }
    }

    pub fn build(&mut self, py: Python) -> PyResult<Py<PyAny>> {
        let mut inner = self.inner.lock().unwrap().take().unwrap();
        if let Some(config) = self.ingestion_config.as_ref() {
            inner = inner.ingestion_config(config.clone().into());
        }

        if !self.enable_tls {
            inner = inner.disable_tls();
        }

        if let Some(strategy) = self.recovery_strategy.as_ref() {
            inner = inner.recovery_strategy(strategy.clone().into());
        }

        inner = inner.checkpoint_interval(self.checkpoint_interval.into());

        if let Some(run) = self.run.as_ref() {
            inner = inner.attach_run(run.clone().into());
        }

        if let Some(run_id) = self.run_id.as_ref() {
            inner = inner.attach_run_id(run_id);
        }

        let awaitable = pyo3_async_runtimes::tokio::future_into_py(py, async move {
            match inner.build().await {
                Ok(stream) => Ok(SiftStreamPy {
                    inner: Arc::new(Mutex::new(Some(stream))),
                }),
                Err(e) => Err(SiftErrorWrapper(e).into()),
            }
        })?;
        Ok(awaitable.into())
    }
}

#[pyclass]
pub struct SiftStreamPy {
    inner: Arc<Mutex<Option<SiftStream<IngestionConfigMode>>>>,
}

#[pymethods]
impl SiftStreamPy {
    pub fn send(&mut self, py: Python, flow: FlowPy) -> PyResult<Py<PyAny>> {
        let mut inner = self.inner.lock().unwrap().take().unwrap();
        let awaitable = pyo3_async_runtimes::tokio::future_into_py(py, async move {
            match inner.send(flow.into()).await {
                Ok(_) => Ok(SiftStreamPy {
                    inner: Arc::new(Mutex::new(Some(inner))),
                }),
                Err(e) => Err(SiftErrorWrapper(e).into()),
            }
        })?;
        Ok(awaitable.into())
    }

    pub fn send_requests(
        &mut self,
        py: Python,
        requests: Vec<IngestWithConfigDataStreamRequestPy>,
    ) -> PyResult<Py<PyAny>> {
        let mut inner = self.inner.lock().unwrap().take().unwrap();
        let awaitable = pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let requests: Vec<IngestWithConfigDataStreamRequest> =
                requests.into_iter().map(|r| r.into()).collect();
            match inner.send_requests(requests).await {
                Ok(_) => Ok(SiftStreamPy {
                    inner: Arc::new(Mutex::new(Some(inner))),
                }),
                Err(e) => Err(SiftErrorWrapper(e).into()),
            }
        })?;
        Ok(awaitable.into())
    }

    pub fn finish(&mut self, py: Python) -> PyResult<Py<PyAny>> {
        let inner = self.inner.lock().unwrap().take().unwrap();
        let awaitable = pyo3_async_runtimes::tokio::future_into_py(py, async move {
            match inner.finish().await {
                Ok(_) => Ok(()),
                Err(e) => Err(SiftErrorWrapper(e).into()),
            }
        })?;
        Ok(awaitable.into())
    }
}

impl From<SiftStream<IngestionConfigMode>> for SiftStreamPy {
    fn from(stream: SiftStream<IngestionConfigMode>) -> Self {
        Self {
            inner: Arc::new(Mutex::new(Some(stream))),
        }
    }
}

impl From<SiftStreamPy> for SiftStream<IngestionConfigMode> {
    fn from(stream: SiftStreamPy) -> Self {
        stream.inner.lock().unwrap().take().unwrap()
    }
}

#[pyclass]
pub struct IngestionConfigModePy {
    inner: Arc<Mutex<IngestionConfigMode>>,
}

impl From<IngestionConfigMode> for IngestionConfigModePy {
    fn from(mode: IngestionConfigMode) -> Self {
        Self {
            inner: Arc::new(Mutex::new(mode)),
        }
    }
}

impl From<IngestionConfigModePy> for IngestionConfigMode {
    fn from(mode: IngestionConfigModePy) -> Self {
        // Take ownership of the inner value
        Arc::try_unwrap(mode.inner)
            .unwrap_or_else(|_| panic!("Failed to unwrap Arc"))
            .into_inner()
            .unwrap_or_else(|_| panic!("Failed to unwrap Mutex"))
    }
}

#[pyclass]
#[derive(Clone)]
pub struct TimeValuePy {
    inner: TimeValue,
}

#[pymethods]
impl TimeValuePy {
    #[new]
    pub fn new() -> Self {
        Self {
            inner: TimeValue::now(),
        }
    }

    #[staticmethod]
    pub fn from_timestamp(secs: i64, nsecs: u32) -> PyResult<Self> {
        Ok(Self {
            inner: TimeValue::try_from_timestamp(secs, nsecs)
                .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("{}", e)))?,
        })
    }

    #[staticmethod]
    pub fn from_timestamp_millis(millis: i64) -> PyResult<Self> {
        Ok(Self {
            inner: TimeValue::try_from_timestamp_millis(millis)
                .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("{}", e)))?,
        })
    }

    #[staticmethod]
    pub fn from_timestamp_micros(micros: i64) -> PyResult<Self> {
        Ok(Self {
            inner: TimeValue::try_from_timestamp_micros(micros)
                .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("{}", e)))?,
        })
    }

    #[staticmethod]
    pub fn from_timestamp_nanos(nanos: i64) -> Self {
        Self {
            inner: TimeValue::from_timestamp_nanos(nanos),
        }
    }

    #[staticmethod]
    pub fn from_rfc3339(val: &str) -> PyResult<Self> {
        Ok(Self {
            inner: TimeValue::try_from_rfc3339(val)
                .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("{}", e)))?,
        })
    }
}

impl From<TimeValuePy> for TimeValue {
    fn from(time: TimeValuePy) -> Self {
        time.inner
    }
}

#[pyclass]
#[derive(Clone)]
pub struct ChannelValuePy {
    inner: ChannelValue,
}

#[pymethods]
impl ChannelValuePy {
    #[staticmethod]
    pub fn bool(name: &str, value: bool) -> Self {
        Self {
            inner: ChannelValue {
                name: name.to_string(),
                value: Value::Bool(value),
            },
        }
    }

    #[staticmethod]
    pub fn string(name: &str, value: String) -> Self {
        Self {
            inner: ChannelValue {
                name: name.to_string(),
                value: Value::String(value),
            },
        }
    }

    #[staticmethod]
    pub fn float(name: &str, value: f32) -> Self {
        Self {
            inner: ChannelValue {
                name: name.to_string(),
                value: Value::Float(value),
            },
        }
    }

    #[staticmethod]
    pub fn double(name: &str, value: f64) -> Self {
        Self {
            inner: ChannelValue {
                name: name.to_string(),
                value: Value::Double(value),
            },
        }
    }

    #[staticmethod]
    pub fn int32(name: &str, value: i32) -> Self {
        Self {
            inner: ChannelValue {
                name: name.to_string(),
                value: Value::Int32(value),
            },
        }
    }

    #[staticmethod]
    pub fn uint32(name: &str, value: u32) -> Self {
        Self {
            inner: ChannelValue {
                name: name.to_string(),
                value: Value::Uint32(value),
            },
        }
    }

    #[staticmethod]
    pub fn int64(name: &str, value: i64) -> Self {
        Self {
            inner: ChannelValue {
                name: name.to_string(),
                value: Value::Int64(value),
            },
        }
    }

    #[staticmethod]
    pub fn uint64(name: &str, value: u64) -> Self {
        Self {
            inner: ChannelValue {
                name: name.to_string(),
                value: Value::Uint64(value),
            },
        }
    }

    #[staticmethod]
    pub fn enum_value(name: &str, value: ChannelEnumTypePy) -> Self {
        Self {
            inner: ChannelValue {
                name: name.to_string(),
                value: Value::Enum(value.key),
            },
        }
    }

    #[staticmethod]
    pub fn bitfield(name: &str, value: Vec<ChannelBitFieldElementPy>) -> Self {
        Self {
            inner: ChannelValue {
                name: name.to_string(),
                value: Value::BitField(value.into_iter().map(|e| e.index as u8).collect()),
            },
        }
    }
}

impl From<ChannelValuePy> for ChannelValue {
    fn from(value: ChannelValuePy) -> Self {
        value.inner
    }
}

#[pyclass]
#[derive(Clone, Debug)]
pub struct FlowPy {
    inner: Flow,
}

#[pymethods]
impl FlowPy {
    #[new]
    pub fn new(flow_name: &str, timestamp: TimeValuePy, values: Vec<ChannelValuePy>) -> Self {
        Self {
            inner: Flow::new(
                flow_name,
                timestamp.into(),
                &values.into_iter().map(|v| v.into()).collect::<Vec<_>>(),
            ),
        }
    }
}

impl From<FlowPy> for Flow {
    fn from(flow: FlowPy) -> Self {
        flow.inner
    }
}

#[pyclass]
#[derive(Clone)]
pub struct ChannelValueTypePy {
    inner: ChannelValueType,
}

#[pymethods]
impl ChannelValueTypePy {
    #[staticmethod]
    pub fn bool(value: bool) -> Self {
        Self {
            inner: ChannelValueType::Bool(value),
        }
    }

    #[staticmethod]
    pub fn string(value: String) -> Self {
        Self {
            inner: ChannelValueType::String(value),
        }
    }

    #[staticmethod]
    pub fn float(value: f32) -> Self {
        Self {
            inner: ChannelValueType::Float(value),
        }
    }

    #[staticmethod]
    pub fn double(value: f64) -> Self {
        Self {
            inner: ChannelValueType::Double(value),
        }
    }

    #[staticmethod]
    pub fn int32(value: i32) -> Self {
        Self {
            inner: ChannelValueType::Int32(value),
        }
    }

    #[staticmethod]
    pub fn uint32(value: u32) -> Self {
        Self {
            inner: ChannelValueType::Uint32(value),
        }
    }

    #[staticmethod]
    pub fn int64(value: i64) -> Self {
        Self {
            inner: ChannelValueType::Int64(value),
        }
    }

    #[staticmethod]
    pub fn uint64(value: u64) -> Self {
        Self {
            inner: ChannelValueType::Uint64(value),
        }
    }

    #[staticmethod]
    pub fn enum_value(value: u32) -> Self {
        Self {
            inner: ChannelValueType::Enum(value),
        }
    }

    #[staticmethod]
    pub fn bitfield(value: Vec<u8>) -> Self {
        Self {
            inner: ChannelValueType::BitField(value),
        }
    }

    #[staticmethod]
    pub fn bytes(value: Vec<u8>) -> Self {
        Self {
            inner: ChannelValueType::Bytes(value),
        }
    }
}

impl From<Value> for ChannelValueTypePy {
    fn from(value: Value) -> Self {
        match value {
            Value::Bool(val) => Self::bool(val),
            Value::String(val) => Self::string(val),
            Value::Float(val) => Self::float(val),
            Value::Double(val) => Self::double(val),
            Value::Int32(val) => Self::int32(val),
            Value::Int64(val) => Self::int64(val),
            Value::Uint32(val) => Self::uint32(val),
            Value::Uint64(val) => Self::uint64(val),
            Value::Enum(val) => Self::enum_value(val),
            Value::BitField(val) => Self::bitfield(val),
        }
    }
}

impl From<ChannelValueTypePy> for ChannelValueType {
    fn from(value: ChannelValueTypePy) -> Self {
        value.inner
    }
}

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

#[pymodule]
fn sift_stream_bindings(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<DurationPy>()?;
    m.add_class::<SiftStreamBuilderPy>()?;
    m.add_class::<IngestionConfigFormPy>()?;
    m.add_class::<FlowConfigPy>()?;
    m.add_class::<ChannelConfigPy>()?;
    m.add_class::<ChannelDataTypePy>()?;
    m.add_class::<ChannelEnumTypePy>()?;
    m.add_class::<ChannelBitFieldElementPy>()?;
    m.add_class::<RecoveryStrategyPy>()?;
    m.add_class::<RetryPolicyPy>()?;
    m.add_class::<RunFormPy>()?;
    m.add_class::<IngestionConfigModePy>()?;
    m.add_class::<SiftStreamPy>()?;
    m.add_class::<FlowPy>()?;
    m.add_class::<TimeValuePy>()?;
    m.add_class::<ChannelValuePy>()?;
    m.add_class::<ChannelValueTypePy>()?;
    m.add_class::<IngestWithConfigDataStreamRequestPy>()?;
    Ok(())
}
