// @generated
/// The request for a call to `ChannelSchemaService_CreateChannelSchema`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateChannelSchemaRequest {
    #[prost(string, tag="1")]
    pub channel: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub unit: ::prost::alloc::string::String,
    #[prost(enumeration="super::super::common::r#type::v1::ChannelDataType", tag="3")]
    pub data_type: i32,
    #[prost(message, repeated, tag="4")]
    pub enum_types: ::prost::alloc::vec::Vec<super::super::common::r#type::v1::ChannelEnumType>,
    #[prost(message, repeated, tag="5")]
    pub bit_field_elements: ::prost::alloc::vec::Vec<super::super::common::r#type::v1::ChannelBitFieldElement>,
    /// The name of the asset (case-insensitive).
    #[prost(string, tag="6")]
    pub asset_name: ::prost::alloc::string::String,
}
/// The response of a call to `ChannelSchemaService_CreateChannelSchema`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateChannelSchemaResponse {
}
/// The request for a call to `ChannelSchemaService_BatchCreateChannelSchemas`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchCreateChannelSchemasRequest {
    /// Limit of 1000 channels schemas per batch
    #[prost(message, repeated, tag="1")]
    pub requests: ::prost::alloc::vec::Vec<CreateChannelSchemaRequest>,
    /// The asset to which the channel schemas belong (case-insensitive).
    /// If this field, and any of the child requests specify an asset, the assets must match.
    #[prost(string, tag="2")]
    pub asset_name: ::prost::alloc::string::String,
}
/// The response of a call to `ChannelSchemaService_BatchCreateChannelSchemas`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchCreateChannelSchemasResponse {
}
include!("sift.channel_schemas.v2.tonic.rs");
include!("sift.channel_schemas.v2.serde.rs");
// @@protoc_insertion_point(module)