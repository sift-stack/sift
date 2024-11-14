// @generated
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
/// The request for a call to `ChannelService_GetChannel`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetChannelRequest {
    #[prost(string, tag="1")]
    pub channel_id: ::prost::alloc::string::String,
}
/// The response of a call to `ChannelService_GetChannel`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetChannelResponse {
    #[prost(message, optional, tag="1")]
    pub channel: ::core::option::Option<Channel>,
}
/// The request for a call to `ChannelService_ListChannels` to retrieve channels.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListChannelsRequest {
    /// The maximum number of channels to return. The service may return fewer than this value.
    /// If unspecified, at most 50 channels will be returned. The maximum value is 10,000; values above
    /// 10,000 will be coerced to 10,000. Optional.
    #[prost(uint32, tag="1")]
    pub page_size: u32,
    /// A page token, received from a previous `ListChannels` call.
    /// Provide this to retrieve the subsequent page.
    /// When paginating, all other parameters provided to `ListChannels` must match
    /// the call that provided the page token. Optional.
    #[prost(string, tag="2")]
    pub page_token: ::prost::alloc::string::String,
    /// A [Common Expression Language (CEL)](<https://github.com/google/cel-spec>) filter string.
    /// Available fields to filter by are `channel_id`, `asset_id`, `name`, `component`, `description`, `active`, `created_date`, and `modified_date`.
    /// For further information about how to use CELs, please refer to [this guide](<https://github.com/google/cel-spec/blob/master/doc/langdef.md#standard-definitions>).
    /// For more information about the fields used for filtering, please refer to [this definition](/api/grpc/protocol_buffers/channels#channel). Optional.
    #[prost(string, tag="3")]
    pub filter: ::prost::alloc::string::String,
    /// This field is only required if your user belongs to multiple organizations.
    #[prost(string, tag="4")]
    pub organization_id: ::prost::alloc::string::String,
    /// How to order the retrieved channels. Formatted as a comma-separated string i.e. "<field_name>\[ desc\],...".
    /// Available fields to order_by are `created_date` and `modified_date`.
    /// If left empty, items are ordered by `created_date` in ascending order (oldest-first).
    /// For more information about the format of this field, read [this](<https://google.aip.dev/132#ordering>)
    /// Example: "created_date desc,modified_date"
    #[prost(string, tag="5")]
    pub order_by: ::prost::alloc::string::String,
}
/// The result of a call to `ChannelService_ListChannels`.
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