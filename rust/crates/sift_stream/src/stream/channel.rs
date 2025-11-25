use sift_rs::{
    common::r#type::v1::ChannelDataType, ingest::v1::ingest_with_config_data_channel_value::Type,
};

/// Represents the value emitted by a named channel.
#[derive(Debug, PartialEq, Clone)]
pub struct ChannelValue {
    pub name: String,
    pub value: Value,
}

/// Represents a specific enumeration of an enum channel.
#[derive(Debug, PartialEq)]
pub struct ChannelEnum(pub u32);

/// Represents a typed-value emitted by a channel.
#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    Bool(bool),
    String(String),
    Float(f32),
    Double(f64),
    Int32(i32),
    Int64(i64),
    Uint32(u32),
    Uint64(u64),
    Enum(u32),
    BitField(Vec<u8>),
}

impl Value {
    pub(crate) fn pb_data_type(&self) -> ChannelDataType {
        match self {
            Value::Bool(_) => ChannelDataType::Bool,
            Value::String(_) => ChannelDataType::String,
            Value::Double(_) => ChannelDataType::Double,
            Value::Float(_) => ChannelDataType::Float,
            Value::Int32(_) => ChannelDataType::Int32,
            Value::Int64(_) => ChannelDataType::Int64,
            Value::Uint32(_) => ChannelDataType::Uint32,
            Value::Uint64(_) => ChannelDataType::Uint64,
            Value::Enum(_) => ChannelDataType::Enum,
            Value::BitField(_) => ChannelDataType::BitField,
        }
    }

    pub(crate) fn pb_value(&self) -> Type {
        match self {
            Value::Bool(val) => Type::Bool(*val),
            Value::String(val) => Type::String(val.clone()),
            Value::Double(val) => Type::Double(*val),
            Value::Float(val) => Type::Float(*val),
            Value::Int32(val) => Type::Int32(*val),
            Value::Int64(val) => Type::Int64(*val),
            Value::Uint32(val) => Type::Uint32(*val),
            Value::Uint64(val) => Type::Uint64(*val),
            Value::Enum(val) => Type::Enum(*val),
            Value::BitField(val) => Type::BitField(val.clone()),
        }
    }
}

impl ChannelValue {
    /// Creates a [ChannelValue] for a channel of name `name`.
    ///
    /// Example:
    /// ```ignore
    /// ChannelValue::new("arm-joint", 3_i32);
    /// ChannelValue::new("navigation", 3.14_f32);
    /// ```
    pub fn new<T: Into<Value>>(name: &str, val: T) -> Self {
        Self {
            name: name.to_string(),
            value: val.into(),
        }
    }

    pub(crate) fn pb_value(&self) -> Type {
        self.value.pb_value()
    }
}

impl From<bool> for Value {
    fn from(value: bool) -> Self {
        Value::Bool(value)
    }
}

impl From<String> for Value {
    fn from(value: String) -> Self {
        Value::String(value)
    }
}

impl From<&str> for Value {
    fn from(value: &str) -> Self {
        Value::String(value.to_string())
    }
}

impl From<f32> for Value {
    fn from(value: f32) -> Self {
        Value::Float(value)
    }
}

impl From<f64> for Value {
    fn from(value: f64) -> Self {
        Value::Double(value)
    }
}

impl From<i32> for Value {
    fn from(value: i32) -> Self {
        Value::Int32(value)
    }
}

impl From<i64> for Value {
    fn from(value: i64) -> Self {
        Value::Int64(value)
    }
}

impl From<ChannelEnum> for Value {
    fn from(value: ChannelEnum) -> Self {
        Value::Enum(value.0)
    }
}

impl From<u32> for Value {
    fn from(value: u32) -> Self {
        Value::Uint32(value)
    }
}

impl From<u64> for Value {
    fn from(value: u64) -> Self {
        Value::Uint64(value)
    }
}

impl From<Vec<u8>> for Value {
    fn from(value: Vec<u8>) -> Self {
        Value::BitField(value)
    }
}

impl From<&[u8]> for Value {
    fn from(value: &[u8]) -> Self {
        Value::BitField(value.to_vec())
    }
}

#[test]
fn test_channel_value_conversion() {
    let bool_value = ChannelValue::new("channel", true);
    assert_eq!(
        ChannelValue {
            name: String::from("channel"),
            value: Value::Bool(true)
        },
        bool_value
    );

    let string_value = ChannelValue::new("channel", "value".to_string());
    assert_eq!(
        ChannelValue {
            name: String::from("channel"),
            value: Value::String("value".to_string())
        },
        string_value
    );

    let str_value = ChannelValue::new("channel", "value");
    assert_eq!(
        ChannelValue {
            name: String::from("channel"),
            value: Value::String("value".to_string())
        },
        str_value
    );

    let float_val = ChannelValue::new("channel", 1.0_f32);
    assert_eq!(
        ChannelValue {
            name: String::from("channel"),
            value: Value::Float(1.0)
        },
        float_val
    );

    let double_val = ChannelValue::new("channel", 1.0_f64);
    assert_eq!(
        ChannelValue {
            name: String::from("channel"),
            value: Value::Double(1.0)
        },
        double_val
    );

    let int32_val = ChannelValue::new("channel", 1_i32);
    assert_eq!(
        ChannelValue {
            name: String::from("channel"),
            value: Value::Int32(1_i32)
        },
        int32_val
    );

    let int64_val = ChannelValue::new("channel", 1_i64);
    assert_eq!(
        ChannelValue {
            name: String::from("channel"),
            value: Value::Int64(1_i64)
        },
        int64_val
    );

    let u32_val = ChannelValue::new("channel", 1_u32);
    assert_eq!(
        ChannelValue {
            name: String::from("channel"),
            value: Value::Uint32(1_u32)
        },
        u32_val
    );

    let u64_val = ChannelValue::new("channel", 1_u64);
    assert_eq!(
        ChannelValue {
            name: String::from("channel"),
            value: Value::Uint64(1_u64)
        },
        u64_val
    );

    let enum_val = ChannelValue::new("channel", ChannelEnum(1));
    assert_eq!(
        ChannelValue {
            name: String::from("channel"),
            value: Value::Enum(1)
        },
        enum_val
    );

    let bitfield_val = ChannelValue::new("channel", vec![1]);
    assert_eq!(
        ChannelValue {
            name: String::from("channel"),
            value: Value::BitField(vec![1])
        },
        bitfield_val
    );

    let bytes: Vec<u8> = vec![1];
    let bitfield_val = ChannelValue::new("channel", bytes.as_slice());
    assert_eq!(
        ChannelValue {
            name: String::from("channel"),
            value: Value::BitField(bytes)
        },
        bitfield_val
    );
}
