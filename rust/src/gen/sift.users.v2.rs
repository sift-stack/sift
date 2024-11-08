// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateUserOrganizationActiveRequest {
    #[prost(string, tag="1")]
    pub user_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub organization_id: ::prost::alloc::string::String,
    #[prost(bool, tag="3")]
    pub active: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateUserOrganizationActiveResponse {
}
/// The request for a call to retrieve a single existing user by its user_id.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetUserRequest {
    /// The id of the asset to be retrieved. Required.
    #[prost(string, tag="1")]
    pub user_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetUserResponse {
    #[prost(message, optional, tag="1")]
    pub user: ::core::option::Option<super::super::common::r#type::v1::User>,
}
/// The request for a call to `UserService_ListActiveUsers` to retrieve users.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListActiveUsersRequest {
    /// The maximum number of users to return. The service may return fewer than this value.
    /// If unspecified, at most 50 users will be returned. The maximum value is 1000; values above
    /// 1000 will be coerced to 1000. Optional.
    #[prost(uint32, tag="1")]
    pub page_size: u32,
    /// A page token, received from a previous `ListActiveUsers` call.
    /// Provide this to retrieve the subsequent page.
    /// When paginating, all other parameters provided to `ListActiveUsers` must match
    /// the call that provided the page token. Optional.
    #[prost(string, tag="2")]
    pub page_token: ::prost::alloc::string::String,
    /// A [Common Expression Language (CEL)](<https://github.com/google/cel-spec>) filter string.
    /// Available fields to filter by are `user_id` and `name`.
    /// For further information about how to use CELs, please refer to [this guide](<https://github.com/google/cel-spec/blob/master/doc/langdef.md#standard-definitions>).
    /// For more information about the fields used for filtering, please refer to [this definition](/protocol-buffers/documentation#users). Optional.
    #[prost(string, tag="3")]
    pub filter: ::prost::alloc::string::String,
    /// Optional. If provided, this will scope down the user search to just those in the organization ID provided.
    #[prost(string, tag="4")]
    pub organization_id: ::prost::alloc::string::String,
}
/// The response of a call to `UserService_ListActiveUsersResponse`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListActiveUsersResponse {
    #[prost(message, repeated, tag="1")]
    pub users: ::prost::alloc::vec::Vec<super::super::common::r#type::v1::User>,
    #[prost(string, tag="2")]
    pub next_page_token: ::prost::alloc::string::String,
}
include!("sift.users.v2.tonic.rs");
include!("sift.users.v2.serde.rs");
// @@protoc_insertion_point(module)