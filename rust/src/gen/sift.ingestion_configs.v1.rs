// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IngestionConfig {
    #[prost(string, tag="1")]
    pub ingestion_config_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub asset_id: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub client_key: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FlowConfig {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="2")]
    pub channels: ::prost::alloc::vec::Vec<ChannelConfig>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelConfig {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub component: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub unit: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub description: ::prost::alloc::string::String,
    #[prost(enumeration="super::super::common::r#type::v1::ChannelDataType", tag="5")]
    pub data_type: i32,
    #[prost(message, repeated, tag="6")]
    pub enum_types: ::prost::alloc::vec::Vec<super::super::common::r#type::v1::ChannelEnumType>,
    #[prost(message, repeated, tag="7")]
    pub bit_field_elements: ::prost::alloc::vec::Vec<super::super::common::r#type::v1::ChannelBitFieldElement>,
}
/// The request for a call to `IngestionConfigService_GetIngestionConfig` to retrieve an ingestion config.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetIngestionConfigRequest {
    #[prost(string, tag="1")]
    pub ingestion_config_id: ::prost::alloc::string::String,
}
/// The result of a call to `IngestionConfigService_GetIngestionConfig`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetIngestionConfigResponse {
    #[prost(message, optional, tag="1")]
    pub ingestion_config: ::core::option::Option<IngestionConfig>,
}
/// The request for a call to `IngestionConfigService_ListIngestionConfigs` to retrieve ingestion configs.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateIngestionConfigRequest {
    #[prost(string, tag="1")]
    pub asset_name: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="2")]
    pub flows: ::prost::alloc::vec::Vec<FlowConfig>,
    #[prost(string, tag="3")]
    pub organization_id: ::prost::alloc::string::String,
    /// The `client_key` field is a user-defined string you can supply to uniquely identify
    /// an ingestion config (and retrieve it via `GetIngestionConfig`).
    /// An error is returned if you try to create an ingestion config with a
    /// client_key that already exists.
    #[prost(string, tag="4")]
    pub client_key: ::prost::alloc::string::String,
}
/// The result of a call to `IngestionConfigService_CreateIngestionConfig`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateIngestionConfigResponse {
    #[prost(message, optional, tag="1")]
    pub ingestion_config: ::core::option::Option<IngestionConfig>,
}
/// The request for a call to `IngestionConfigService_ListIngestionConfigs` to retrieve ingestion configs.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListIngestionConfigsRequest {
    /// The maximum number of ingestion configs to return. The service may return fewer than this value.
    /// If unspecified, at most 50 ingestion configs will be returned. The maximum value is 1000; values above
    /// 1000 will be coerced to 1000. Optional.
    #[prost(uint32, tag="1")]
    pub page_size: u32,
    /// A page token, received from a previous `ListIngestionConfigs` call.
    /// Provide this to retrieve the subsequent page.
    /// When paginating, all other parameters provided to `ListIngestionConfigs` must match
    /// the call that provided the page token. Optional.
    #[prost(string, tag="2")]
    pub page_token: ::prost::alloc::string::String,
    /// A [Common Expression Language (CEL)](<https://github.com/google/cel-spec>) filter string.
    /// Available fields to filter by are `ingestion_config_id`, `client_key`, `asset_id`, `created_date`, and `modified_date`.
    /// For further information about how to use CELs, please refer to [this guide](<https://github.com/google/cel-spec/blob/master/doc/langdef.md#standard-definitions>).
    /// For more information about the fields used for filtering, please refer to [this definition](/ingestion/api#sift_ingestion_configs_v1-proto). Optional.
    #[prost(string, tag="3")]
    pub filter: ::prost::alloc::string::String,
}
/// The result of a call to `IngestionConfigService_ListIngestionConfigs`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListIngestionConfigsResponse {
    #[prost(message, repeated, tag="1")]
    pub ingestion_configs: ::prost::alloc::vec::Vec<IngestionConfig>,
    #[prost(string, tag="2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// The request for a call to `IngestionConfigService_CreateIngestionConfigFlows` to create ingestion config flows.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateIngestionConfigFlowsRequest {
    #[prost(string, tag="1")]
    pub ingestion_config_id: ::prost::alloc::string::String,
    /// These flows must have unique names. If you try to send a flow with a name that already exists for an ingestion config, it will return an error.
    #[prost(message, repeated, tag="2")]
    pub flows: ::prost::alloc::vec::Vec<FlowConfig>,
}
/// The result of a call to `IngestionConfigService_CreateIngestionConfigFlows`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateIngestionConfigFlowsResponse {
}
/// The request for a call to `IngestionConfigService_ListIngestionConfigFlows` to retrieve ingestion config flows.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListIngestionConfigFlowsRequest {
    #[prost(string, tag="1")]
    pub ingestion_config_id: ::prost::alloc::string::String,
    /// The maximum number of ingestion config flows to return. The service may return fewer than this value.
    /// If unspecified, at most 50 ingestion config flows will be returned. The maximum value is 1000; values above
    /// 1000 will be coerced to 1000. Optional.
    #[prost(uint32, tag="2")]
    pub page_size: u32,
    /// A page token, received from a previous `ListIngestionConfigFlows` call.
    /// Provide this to retrieve the subsequent page.
    /// When paginating, all other parameters provided to `ListIngestionConfigFlows` must match
    /// the call that provided the page token. Optional.
    #[prost(string, tag="3")]
    pub page_token: ::prost::alloc::string::String,
    /// A [Common Expression Language (CEL)](<https://github.com/google/cel-spec>) filter string.
    /// Available fields to filter by are `flow_name`, `flow_id`, `created_date`, and `modified_date`.
    /// For further information about how to use CELs, please refer to [this guide](<https://github.com/google/cel-spec/blob/master/doc/langdef.md#standard-definitions>).
    /// For more information about the fields used for filtering, please refer to [this definition](/ingestion/api#sift_ingestion_configs_v1-proto). Optional.
    #[prost(string, tag="4")]
    pub filter: ::prost::alloc::string::String,
}
/// The result of a call to `IngestionConfigService_ListIngestionConfigFlows`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListIngestionConfigFlowsResponse {
    #[prost(message, repeated, tag="1")]
    pub flows: ::prost::alloc::vec::Vec<FlowConfig>,
    #[prost(string, tag="2")]
    pub next_page_token: ::prost::alloc::string::String,
}
include!("sift.ingestion_configs.v1.tonic.rs");
// @@protoc_insertion_point(module)