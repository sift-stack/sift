// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Tag {
    #[prost(string, tag="1")]
    pub tag_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub organization_id: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub created_by_user_id: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub modified_by_user_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag="6")]
    pub created_date: ::core::option::Option<::pbjson_types::Timestamp>,
    #[prost(message, optional, tag="7")]
    pub modified_date: ::core::option::Option<::pbjson_types::Timestamp>,
}
include!("sift.tags.v1.serde.rs");
// @@protoc_insertion_point(module)