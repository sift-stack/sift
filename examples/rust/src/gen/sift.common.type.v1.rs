// @generated
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ChannelDataType {
    Unspecified = 0,
    Double = 1,
    String = 2,
    Enum = 3,
    BitField = 4,
    Bool = 5,
    Float = 6,
    Int32 = 7,
    Uint32 = 8,
    Int64 = 9,
    Uint64 = 10,
}
impl ChannelDataType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ChannelDataType::Unspecified => "CHANNEL_DATA_TYPE_UNSPECIFIED",
            ChannelDataType::Double => "CHANNEL_DATA_TYPE_DOUBLE",
            ChannelDataType::String => "CHANNEL_DATA_TYPE_STRING",
            ChannelDataType::Enum => "CHANNEL_DATA_TYPE_ENUM",
            ChannelDataType::BitField => "CHANNEL_DATA_TYPE_BIT_FIELD",
            ChannelDataType::Bool => "CHANNEL_DATA_TYPE_BOOL",
            ChannelDataType::Float => "CHANNEL_DATA_TYPE_FLOAT",
            ChannelDataType::Int32 => "CHANNEL_DATA_TYPE_INT_32",
            ChannelDataType::Uint32 => "CHANNEL_DATA_TYPE_UINT_32",
            ChannelDataType::Int64 => "CHANNEL_DATA_TYPE_INT_64",
            ChannelDataType::Uint64 => "CHANNEL_DATA_TYPE_UINT_64",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "CHANNEL_DATA_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
            "CHANNEL_DATA_TYPE_DOUBLE" => Some(Self::Double),
            "CHANNEL_DATA_TYPE_STRING" => Some(Self::String),
            "CHANNEL_DATA_TYPE_ENUM" => Some(Self::Enum),
            "CHANNEL_DATA_TYPE_BIT_FIELD" => Some(Self::BitField),
            "CHANNEL_DATA_TYPE_BOOL" => Some(Self::Bool),
            "CHANNEL_DATA_TYPE_FLOAT" => Some(Self::Float),
            "CHANNEL_DATA_TYPE_INT_32" => Some(Self::Int32),
            "CHANNEL_DATA_TYPE_UINT_32" => Some(Self::Uint32),
            "CHANNEL_DATA_TYPE_INT_64" => Some(Self::Int64),
            "CHANNEL_DATA_TYPE_UINT_64" => Some(Self::Uint64),
            _ => None,
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Organization {
    #[prost(string, tag="1")]
    pub organization_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub organization_name: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct User {
    #[prost(string, tag="1")]
    pub user_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub user_name: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="3")]
    pub organizations: ::prost::alloc::vec::Vec<Organization>,
}
// @@protoc_insertion_point(module)
