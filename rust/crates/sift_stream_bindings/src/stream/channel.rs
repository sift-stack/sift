use pyo3::prelude::*;
use sift_rs::common::r#type::v1::{ChannelBitFieldElement, ChannelDataType, ChannelEnumType};
use sift_rs::ingest::v1::ingest_with_config_data_channel_value::Type as ChannelValueType;
use sift_stream::stream::channel::{ChannelValue, Value};

// Type Definitions
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

#[pyclass]
#[derive(Clone)]
pub struct ChannelEnumTypePy {
    pub inner: ChannelEnumType,
    #[pyo3(get, set)]
    name: String,
    #[pyo3(get, set)]
    key: u32,
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

#[pyclass]
#[derive(Clone)]
pub struct ChannelValuePy {
    pub inner: ChannelValue,
}

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

impl From<ChannelEnumTypePy> for ChannelEnumType {
    fn from(value: ChannelEnumTypePy) -> Self {
        value.inner
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
