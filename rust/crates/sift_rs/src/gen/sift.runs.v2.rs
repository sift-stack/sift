// @generated
// This file is @generated by prost-build.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Run {
    #[prost(string, tag="1")]
    pub run_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub created_date: ::core::option::Option<::pbjson_types::Timestamp>,
    #[prost(message, optional, tag="3")]
    pub modified_date: ::core::option::Option<::pbjson_types::Timestamp>,
    #[prost(string, tag="4")]
    pub created_by_user_id: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub modified_by_user_id: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub organization_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag="7")]
    pub start_time: ::core::option::Option<::pbjson_types::Timestamp>,
    #[prost(message, optional, tag="8")]
    pub stop_time: ::core::option::Option<::pbjson_types::Timestamp>,
    #[prost(bool, tag="9")]
    pub is_pinned: bool,
    #[prost(string, tag="10")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag="11")]
    pub description: ::prost::alloc::string::String,
    #[prost(string, repeated, tag="12")]
    pub tags: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, tag="13")]
    pub default_report_id: ::prost::alloc::string::String,
    #[prost(string, optional, tag="14")]
    pub client_key: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetRunRequest {
    #[prost(string, tag="1")]
    pub run_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetRunResponse {
    #[prost(message, optional, tag="1")]
    pub run: ::core::option::Option<Run>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListRunsRequest {
    #[prost(uint32, tag="1")]
    pub page_size: u32,
    #[prost(string, tag="2")]
    pub page_token: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub filter: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub order_by: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListRunsResponse {
    #[prost(message, repeated, tag="1")]
    pub runs: ::prost::alloc::vec::Vec<Run>,
    #[prost(string, tag="2")]
    pub next_page_token: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateRunRequest {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub description: ::prost::alloc::string::String,
    #[prost(string, repeated, tag="3")]
    pub tags: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(message, optional, tag="4")]
    pub start_time: ::core::option::Option<::pbjson_types::Timestamp>,
    #[prost(message, optional, tag="5")]
    pub stop_time: ::core::option::Option<::pbjson_types::Timestamp>,
    #[prost(string, tag="7")]
    pub organization_id: ::prost::alloc::string::String,
    #[prost(string, optional, tag="8")]
    pub client_key: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateRunResponse {
    #[prost(message, optional, tag="1")]
    pub run: ::core::option::Option<Run>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateRunRequest {
    #[prost(message, optional, tag="1")]
    pub run: ::core::option::Option<Run>,
    #[prost(message, optional, tag="2")]
    pub update_mask: ::core::option::Option<::pbjson_types::FieldMask>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateRunResponse {
    #[prost(message, optional, tag="1")]
    pub run: ::core::option::Option<Run>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateAutomaticRunAssociationForAssetsRequest {
    #[prost(string, tag="1")]
    pub run_id: ::prost::alloc::string::String,
    #[prost(string, repeated, tag="2")]
    pub asset_names: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CreateAutomaticRunAssociationForAssetsResponse {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteRunRequest {
    #[prost(string, tag="1")]
    pub run_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct DeleteRunResponse {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StopRunRequest {
    #[prost(string, tag="1")]
    pub run_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct StopRunResponse {
}
include!("sift.runs.v2.tonic.rs");
include!("sift.runs.v2.serde.rs");
// @@protoc_insertion_point(module)