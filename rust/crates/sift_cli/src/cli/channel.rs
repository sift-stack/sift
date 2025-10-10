use clap::ValueEnum;
use sift_rs::common::r#type::v1::ChannelDataType;

#[derive(Debug, Clone, ValueEnum)]
pub enum DataType {
    /// Asks the program to infer the type so user can just focus on setting things like unit,
    /// description, etc.
    Infer,
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

impl From<DataType> for ChannelDataType {
    fn from(dt: DataType) -> Self {
        match dt {
            DataType::Double => Self::Double,
            DataType::String => Self::String,
            DataType::Enum => Self::Enum,
            DataType::BitField => Self::BitField,
            DataType::Bool => Self::Bool,
            DataType::Float => Self::Float,
            DataType::Int32 => Self::Int32,
            DataType::Uint32 => Self::Uint32,
            DataType::Int64 => Self::Int64,
            DataType::Uint64 => Self::Uint64,
            DataType::Bytes => Self::Bytes,
            DataType::Infer => Self::Unspecified,
        }
    }
}
