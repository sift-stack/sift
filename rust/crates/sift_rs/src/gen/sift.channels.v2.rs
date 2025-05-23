// @generated
// This file is @generated by prost-build.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Channel {
    #[prost(string, tag="1")]
    pub channel_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub component: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub asset_id: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub description: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub unit_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag="7")]
    pub created_date: ::core::option::Option<::pbjson_types::Timestamp>,
    #[prost(message, optional, tag="8")]
    pub modified_date: ::core::option::Option<::pbjson_types::Timestamp>,
    #[prost(string, tag="9")]
    pub created_by_user_id: ::prost::alloc::string::String,
    #[prost(string, tag="10")]
    pub modified_by_user_id: ::prost::alloc::string::String,
    #[prost(string, tag="11")]
    pub organization_id: ::prost::alloc::string::String,
    #[prost(enumeration="super::super::common::r#type::v1::ChannelDataType", tag="12")]
    pub data_type: i32,
    #[prost(message, repeated, tag="13")]
    pub enum_types: ::prost::alloc::vec::Vec<super::super::common::r#type::v1::ChannelEnumType>,
    #[prost(message, repeated, tag="14")]
    pub bit_field_elements: ::prost::alloc::vec::Vec<super::super::common::r#type::v1::ChannelBitFieldElement>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetChannelRequest {
    #[prost(string, tag="1")]
    pub channel_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetChannelResponse {
    #[prost(message, optional, tag="1")]
    pub channel: ::core::option::Option<Channel>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListChannelsRequest {
    #[prost(uint32, tag="1")]
    pub page_size: u32,
    #[prost(string, tag="2")]
    pub page_token: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub filter: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub organization_id: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub order_by: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListChannelsResponse {
    #[prost(message, repeated, tag="1")]
    pub channels: ::prost::alloc::vec::Vec<Channel>,
    #[prost(string, tag="2")]
    pub next_page_token: ::prost::alloc::string::String,
}
include!("sift.channels.v2.tonic.rs");
include!("sift.channels.v2.serde.rs");
// @@protoc_insertion_point(module)