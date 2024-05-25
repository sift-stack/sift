// @generated
/// The request for a call to `ChannelSchemaService_CreateChannelSchema`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateChannelSchemaRequest {
    /// The name of the component (case-insensitive).
    #[prost(string, tag="1")]
    pub component: ::prost::alloc::string::String,
    /// The name of the channel (case-insensitive).
    #[prost(string, tag="2")]
    pub channel: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub unit: ::prost::alloc::string::String,
    #[prost(enumeration="super::super::common::r#type::v1::ChannelDataType", tag="4")]
    pub data_type: i32,
    #[prost(message, repeated, tag="5")]
    pub enum_types: ::prost::alloc::vec::Vec<super::super::common::r#type::v1::ChannelEnumType>,
    #[prost(message, repeated, tag="6")]
    pub bit_field_elements: ::prost::alloc::vec::Vec<super::super::common::r#type::v1::ChannelBitFieldElement>,
    /// The name of the asset (case-insensitive).
    #[prost(string, tag="7")]
    pub asset_name: ::prost::alloc::string::String,
    /// This field is optional if the caller belongs to a single organization.
    #[prost(string, tag="8")]
    pub organization_id: ::prost::alloc::string::String,
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
    #[prost(message, repeated, tag="1")]
    pub requests: ::prost::alloc::vec::Vec<CreateChannelSchemaRequest>,
    /// The asset to which the channel schemas belong (case-insensitive).
    /// If this field, and any of the child requests specify an asset, the assets must match.
    #[prost(string, tag="2")]
    pub asset_name: ::prost::alloc::string::String,
    /// The organization to which the channel schemas belong.
    /// If this field, and any of the child requests specify an organization_id, the organization_ids must match.
    /// This field is optional if the caller belongs to a single organization.
    #[prost(string, tag="3")]
    pub organization_id: ::prost::alloc::string::String,
}
/// The response of a call to `ChannelSchemaService_BatchCreateChannelSchemas`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchCreateChannelSchemasResponse {
}
include!("sift.channel_schemas.v1.tonic.rs");
include!("sift.channel_schemas.v1.serde.rs");
// @@protoc_insertion_point(module)