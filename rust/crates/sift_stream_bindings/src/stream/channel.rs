use pbjson_types::Empty;
use pyo3::prelude::*;
use pyo3_stub_gen::derive::*;
use sift_rs::common::r#type::v1::{ChannelBitFieldElement, ChannelDataType, ChannelEnumType};
use sift_rs::ingest::v1::ingest_with_config_data_channel_value::Type as ChannelValueType;
use sift_stream::stream::channel::{ChannelValue, Value};

#[gen_stub_pyclass]
#[pyclass]
#[derive(Clone)]
pub struct ChannelValuePy {
    #[pyo3(get, set)]
    name: String,
    #[pyo3(get, set)]
    value: ValuePy,
}

#[gen_stub_pyclass]
#[pyclass]
#[derive(Clone)]
pub struct ChannelEnumPy(pub u32);

#[gen_stub_pyclass]
#[pyclass]
#[derive(Clone)]
pub struct ValuePy {
    inner: Value,
}

#[gen_stub_pyclass]
#[pyclass]
#[derive(Clone)]
pub struct ChannelBitFieldElementPy {
    pub inner: ChannelBitFieldElement,
    #[pyo3(get, set)]
    name: String,
    #[pyo3(get, set)]
    index: i32,
    #[pyo3(get, set)]
    bit_count: u32,
}

#[gen_stub_pyclass]
#[pyclass]
#[derive(Clone)]
pub struct ChannelEnumTypePy {
    pub inner: ChannelEnumType,
    #[pyo3(get, set)]
    name: String,
    #[pyo3(get, set)]
    key: u32,
}

#[gen_stub_pyclass_enum]
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

#[gen_stub_pyclass]
#[pyclass]
#[derive(Clone)]
pub struct ChannelValueTypePy {
    inner: ChannelValueType,
}

// Trait Implementations
impl From<ChannelBitFieldElementPy> for ChannelBitFieldElement {
    fn from(value: ChannelBitFieldElementPy) -> Self {
        value.inner
    }
}

impl From<ChannelBitFieldElement> for ChannelBitFieldElementPy {
    fn from(value: ChannelBitFieldElement) -> Self {
        ChannelBitFieldElementPy {
            inner: value.clone(),
            name: value.name,
            index: value.index,
            bit_count: value.bit_count,
        }
    }
}

impl From<ChannelEnumTypePy> for ChannelEnumType {
    fn from(value: ChannelEnumTypePy) -> Self {
        value.inner
    }
}

impl From<ChannelEnumType> for ChannelEnumTypePy {
    fn from(value: ChannelEnumType) -> Self {
        ChannelEnumTypePy {
            inner: value.clone(),
            name: value.name,
            key: value.key,
        }
    }
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

impl From<ChannelValuePy> for ChannelValue {
    fn from(value: ChannelValuePy) -> Self {
        ChannelValue {
            name: value.name,
            value: value.value.into(),
        }
    }
}

impl From<ValuePy> for Value {
    fn from(value: ValuePy) -> Self {
        value.inner
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

// PyO3 Method Implementations
#[gen_stub_pymethods]
#[pymethods]
impl ValuePy {
    #[staticmethod]
    #[allow(non_snake_case)]
    pub fn Bool(value: bool) -> Self {
        Self {
            inner: Value::Bool(value),
        }
    }

    #[staticmethod]
    #[allow(non_snake_case)]
    pub fn String(value: String) -> Self {
        Self {
            inner: Value::String(value),
        }
    }

    #[staticmethod]
    #[allow(non_snake_case)]
    pub fn Float(value: f32) -> Self {
        Self {
            inner: Value::Float(value),
        }
    }

    #[staticmethod]
    #[allow(non_snake_case)]
    pub fn Double(value: f64) -> Self {
        Self {
            inner: Value::Double(value),
        }
    }

    #[staticmethod]
    #[allow(non_snake_case)]
    pub fn Int32(value: i32) -> Self {
        Self {
            inner: Value::Int32(value),
        }
    }

    #[staticmethod]
    #[allow(non_snake_case)]
    pub fn Int64(value: i64) -> Self {
        Self {
            inner: Value::Int64(value),
        }
    }

    #[staticmethod]
    #[allow(non_snake_case)]
    pub fn Uint32(value: u32) -> Self {
        Self {
            inner: Value::Uint32(value),
        }
    }

    #[staticmethod]
    #[allow(non_snake_case)]
    pub fn Uint64(value: u64) -> Self {
        Self {
            inner: Value::Uint64(value),
        }
    }

    #[staticmethod]
    #[allow(non_snake_case)]
    pub fn Enum(value: u32) -> Self {
        Self {
            inner: Value::Enum(value),
        }
    }

    #[staticmethod]
    #[allow(non_snake_case)]
    pub fn BitField(value: Vec<u8>) -> Self {
        Self {
            inner: Value::BitField(value),
        }
    }

    pub fn is_bool(&self) -> bool {
        matches!(self.inner, Value::Bool(_))
    }

    pub fn is_string(&self) -> bool {
        matches!(self.inner, Value::String(_))
    }

    pub fn is_float(&self) -> bool {
        matches!(self.inner, Value::Float(_))
    }

    pub fn is_double(&self) -> bool {
        matches!(self.inner, Value::Double(_))
    }

    pub fn is_int32(&self) -> bool {
        matches!(self.inner, Value::Int32(_))
    }

    pub fn is_int64(&self) -> bool {
        matches!(self.inner, Value::Int64(_))
    }

    pub fn is_uint32(&self) -> bool {
        matches!(self.inner, Value::Uint32(_))
    }

    pub fn is_uint64(&self) -> bool {
        matches!(self.inner, Value::Uint64(_))
    }

    pub fn is_enum(&self) -> bool {
        matches!(self.inner, Value::Enum(_))
    }

    pub fn is_bitfield(&self) -> bool {
        matches!(self.inner, Value::BitField(_))
    }

    pub fn as_bool(&self) -> PyResult<bool> {
        match &self.inner {
            Value::Bool(v) => Ok(*v),
            _ => Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
                "Value is not a Bool",
            )),
        }
    }

    pub fn as_string(&self) -> PyResult<String> {
        match &self.inner {
            Value::String(v) => Ok(v.clone()),
            _ => Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
                "Value is not a String",
            )),
        }
    }

    pub fn as_float(&self) -> PyResult<f32> {
        match &self.inner {
            Value::Float(v) => Ok(*v),
            _ => Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
                "Value is not a Float",
            )),
        }
    }

    pub fn as_double(&self) -> PyResult<f64> {
        match &self.inner {
            Value::Double(v) => Ok(*v),
            _ => Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
                "Value is not a Double",
            )),
        }
    }

    pub fn as_int32(&self) -> PyResult<i32> {
        match &self.inner {
            Value::Int32(v) => Ok(*v),
            _ => Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
                "Value is not an Int32",
            )),
        }
    }

    pub fn as_int64(&self) -> PyResult<i64> {
        match &self.inner {
            Value::Int64(v) => Ok(*v),
            _ => Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
                "Value is not an Int64",
            )),
        }
    }

    pub fn as_uint32(&self) -> PyResult<u32> {
        match &self.inner {
            Value::Uint32(v) => Ok(*v),
            _ => Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
                "Value is not a Uint32",
            )),
        }
    }

    pub fn as_uint64(&self) -> PyResult<u64> {
        match &self.inner {
            Value::Uint64(v) => Ok(*v),
            _ => Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
                "Value is not a Uint64",
            )),
        }
    }

    pub fn as_enum(&self) -> PyResult<u32> {
        match &self.inner {
            Value::Enum(v) => Ok(*v),
            _ => Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
                "Value is not an Enum",
            )),
        }
    }

    pub fn as_bitfield(&self) -> PyResult<Vec<u8>> {
        match &self.inner {
            Value::BitField(v) => Ok(v.clone()),
            _ => Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
                "Value is not a BitField",
            )),
        }
    }
}

#[gen_stub_pymethods]
#[pymethods]
impl ChannelValuePy {
    #[new]
    pub fn new(name: String, value: ValuePy) -> Self {
        Self { name, value }
    }
}

#[gen_stub_pymethods]
#[pymethods]
impl ChannelEnumPy {
    #[new]
    pub fn new(val: u32) -> Self {
        Self(val)
    }
}

#[gen_stub_pymethods]
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

#[gen_stub_pymethods]
#[pymethods]
impl ChannelEnumTypePy {
    #[new]
    pub fn new(name: &str, key: u32) -> Self {
        Self {
            inner: ChannelEnumType {
                name: name.to_string(),
                key,
                ..Default::default()
            },
            name: name.to_string(),
            key,
        }
    }
}

#[gen_stub_pymethods]
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

    #[staticmethod]
    pub fn empty() -> Self {
        Self {
            inner: ChannelValueType::Empty(Empty {}),
        }
    }
}
