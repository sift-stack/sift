use sift_rs::{
    common::r#type::v1::ChannelDataType, ingest::v1::ingest_with_config_data_channel_value::Type,
};

#[derive(Debug)]
pub struct ChannelDataPoint {
    pub(crate) name: String,
    pub(crate) value: ChannelValue,
}

#[derive(Debug)]
pub enum ChannelValue {
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

impl ChannelDataPoint {
    pub(crate) fn pb_data_type(&self) -> i32 {
        match self.value {
            ChannelValue::Bool(_) => i32::from(ChannelDataType::Bool),
            ChannelValue::String(_) => i32::from(ChannelDataType::String),
            ChannelValue::Double(_) => i32::from(ChannelDataType::Double),
            ChannelValue::Float(_) => i32::from(ChannelDataType::Float),
            ChannelValue::Int32(_) => i32::from(ChannelDataType::Int32),
            ChannelValue::Int64(_) => i32::from(ChannelDataType::Int64),
            ChannelValue::Uint32(_) => i32::from(ChannelDataType::Uint32),
            ChannelValue::Uint64(_) => i32::from(ChannelDataType::Uint64),
            ChannelValue::Enum(_) => i32::from(ChannelDataType::Enum),
            ChannelValue::BitField(_) => i32::from(ChannelDataType::BitField),
        }
    }

    pub(crate) fn pb_value(&self) -> Type {
        match self.value {
            ChannelValue::Bool(val) => Type::Bool(val),
            ChannelValue::String(ref val) => Type::String(val.clone()),
            ChannelValue::Double(val) => Type::Double(val),
            ChannelValue::Float(val) => Type::Float(val),
            ChannelValue::Int32(val) => Type::Int32(val),
            ChannelValue::Int64(val) => Type::Int64(val),
            ChannelValue::Uint32(val) => Type::Uint32(val),
            ChannelValue::Uint64(val) => Type::Uint64(val),
            ChannelValue::Enum(val) => Type::Enum(val),
            ChannelValue::BitField(ref val) => Type::BitField(val.clone()),
        }
    }
}
