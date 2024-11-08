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
pub struct ChannelBitFieldElement {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// The index of this element's first bit in the logical bit field array.
    #[prost(int32, tag="2")]
    pub index: i32,
    #[prost(uint32, tag="3")]
    pub bit_count: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelEnumType {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub key: u32,
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
pub struct ResourceIdentifier {
    #[prost(oneof="resource_identifier::Identifier", tags="1, 2")]
    pub identifier: ::core::option::Option<resource_identifier::Identifier>,
}
/// Nested message and enum types in `ResourceIdentifier`.
pub mod resource_identifier {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Identifier {
        #[prost(string, tag="1")]
        Id(::prost::alloc::string::String),
        #[prost(string, tag="2")]
        ClientKey(::prost::alloc::string::String),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NamedResource {
    #[prost(oneof="named_resource::Resource", tags="1, 2")]
    pub resource: ::core::option::Option<named_resource::Resource>,
}
/// Nested message and enum types in `NamedResource`.
pub mod named_resource {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Resource {
        #[prost(string, tag="1")]
        Id(::prost::alloc::string::String),
        #[prost(string, tag="2")]
        Name(::prost::alloc::string::String),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResourceIdentifiers {
    #[prost(oneof="resource_identifiers::Identifiers", tags="1, 2")]
    pub identifiers: ::core::option::Option<resource_identifiers::Identifiers>,
}
/// Nested message and enum types in `ResourceIdentifiers`.
pub mod resource_identifiers {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Identifiers {
        #[prost(message, tag="1")]
        Ids(super::Ids),
        #[prost(message, tag="2")]
        ClientKeys(super::ClientKeys),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NamedResources {
    #[prost(oneof="named_resources::Resources", tags="1, 2")]
    pub resources: ::core::option::Option<named_resources::Resources>,
}
/// Nested message and enum types in `NamedResources`.
pub mod named_resources {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Resources {
        #[prost(message, tag="1")]
        Ids(super::Ids),
        #[prost(message, tag="2")]
        Names(super::Names),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Ids {
    #[prost(string, repeated, tag="1")]
    pub ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClientKeys {
    #[prost(string, repeated, tag="1")]
    pub client_keys: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Names {
    #[prost(string, repeated, tag="1")]
    pub names: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
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
include!("sift.common.type.v1.serde.rs");
// @@protoc_insertion_point(module)