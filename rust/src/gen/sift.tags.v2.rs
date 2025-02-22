// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Tag {
    #[prost(string, tag="1")]
    pub tag_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub created_by_user_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag="4")]
    pub created_date: ::core::option::Option<::pbjson_types::Timestamp>,
}
/// The request for a call to TagService.CreateTag.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateTagRequest {
    /// The name for the new tag.
    /// If the tag already exists, an error is returned.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
/// The result of a call to TagService.CreateTag.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateTagResponse {
    #[prost(message, optional, tag="1")]
    pub tag: ::core::option::Option<Tag>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListTagsRequest {
    /// The maximum number of tags to return.
    /// The service may return fewer than this value.
    /// If unspecified, at most 50 tags will be returned.
    /// The maximum value is 1000; values above 1000 will be coerced to 1000.
    #[prost(uint32, tag="1")]
    pub page_size: u32,
    /// A page token, received from a previous `ListTags` call.
    /// Provide this to retrieve the subsequent page.
    /// When paginating, all other parameters provided to `ListTags` must match
    /// the call that provided the page token.
    #[prost(string, tag="2")]
    pub page_token: ::prost::alloc::string::String,
    /// A [Common Expression Language (CEL)](<https://github.com/google/cel-spec>) filter string
    /// Available fields to filter by are 'name' and 'tag_id'.
    /// For further information about how to use CELs, please refer to [this guide](<https://github.com/google/cel-spec/blob/master/doc/langdef.md#standard-definitions>).
    /// For more information about the fields used for filtering, please refer to [this definition](/protocol-buffers/documentation#tag). Optional.
    #[prost(string, tag="3")]
    pub filter: ::prost::alloc::string::String,
    /// How to order the retrieved campaigns. Formatted as a comma-separated string i.e. "FIELD_NAME\[ desc\],...".
    /// Available fields to order_by are `created_date` and `name`.
    /// If left empty, items are ordered by `created_date` in ascending order (oldest-first).
    /// For more information about the format of this field, read [this](<https://google.aip.dev/132#ordering>)
    /// Example: "created_date desc,name"
    #[prost(string, tag="4")]
    pub order_by: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListTagsResponse {
    #[prost(message, repeated, tag="1")]
    pub tags: ::prost::alloc::vec::Vec<Tag>,
    /// A token, which can be sent as `page_token` to retrieve the next page.
    /// If this field is empty, there are no subsequent pages.
    #[prost(string, tag="2")]
    pub next_page_token: ::prost::alloc::string::String,
}
include!("sift.tags.v2.tonic.rs");
include!("sift.tags.v2.serde.rs");
// @@protoc_insertion_point(module)