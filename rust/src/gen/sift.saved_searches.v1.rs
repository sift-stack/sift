// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SavedSearch {
    #[prost(string, tag="1")]
    pub saved_search_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub organization_id: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub name: ::prost::alloc::string::String,
    #[prost(message, optional, tag="4")]
    pub properties: ::core::option::Option<SavedSearchProperties>,
    #[prost(string, tag="5")]
    pub created_by_user_id: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub modified_by_user_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag="7")]
    pub created_date: ::core::option::Option<::pbjson_types::Timestamp>,
    #[prost(message, optional, tag="8")]
    pub modified_date: ::core::option::Option<::pbjson_types::Timestamp>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SavedSearchProperties {
    #[prost(string, tag="1")]
    pub overview_mode: ::prost::alloc::string::String,
    #[prost(string, optional, tag="2")]
    pub search_term: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag="3")]
    pub from_date_time: ::core::option::Option<::pbjson_types::Timestamp>,
    #[prost(message, optional, tag="4")]
    pub to_date_time: ::core::option::Option<::pbjson_types::Timestamp>,
    #[prost(message, repeated, tag="5")]
    pub asset_items: ::prost::alloc::vec::Vec<SavedSearchFilterItem>,
    #[prost(message, repeated, tag="6")]
    pub user_items: ::prost::alloc::vec::Vec<SavedSearchFilterItem>,
    #[prost(message, repeated, tag="7")]
    pub tag_items: ::prost::alloc::vec::Vec<SavedSearchFilterItem>,
    #[prost(message, repeated, tag="8")]
    pub annotation_items: ::prost::alloc::vec::Vec<SavedSearchFilterItem>,
    #[prost(message, repeated, tag="9")]
    pub run_items: ::prost::alloc::vec::Vec<SavedSearchFilterItem>,
    #[prost(message, repeated, tag="10")]
    pub report_template_items: ::prost::alloc::vec::Vec<SavedSearchFilterItem>,
    #[prost(bool, optional, tag="11")]
    pub show_advanced_filters: ::core::option::Option<bool>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SavedSearchFilterItem {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
}
/// The request for a call to `SavedSearchService_GetSavedSearch` to retrieve a saved search;
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetSavedSearchRequest {
    #[prost(string, tag="1")]
    pub saved_search_id: ::prost::alloc::string::String,
}
/// The response of a call to `SavedSearchService_GetSavedSearch`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetSavedSearchResponse {
    #[prost(message, optional, tag="1")]
    pub saved_search: ::core::option::Option<SavedSearch>,
}
/// The request for a call to `SavedSearchService_ListSavedSearches` to retrieve saved searches.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListSavedSearchesRequest {
    /// The maximum number of saved searches to return. The service may return fewer than this value.
    /// If unspecified, at most 50 saved searches will be returned. The maximum value is 1000; values above
    /// 1000 will be coerced to 1000. Optional.
    #[prost(uint32, tag="1")]
    pub page_size: u32,
    /// A page token, received from a previous `ListSavedSearches` call.
    /// Provide this to retrieve the subsequent page.
    /// When paginating, all other parameters provided to `ListSavedSearches` must match
    /// the call that provided the page token. Optional.
    #[prost(string, tag="2")]
    pub page_token: ::prost::alloc::string::String,
    /// A [Common Expression Language (CEL)](<https://github.com/google/cel-spec>) filter string.
    /// Available fields to filter by are 'name' and 'saved_search_id'.
    /// For further information about how to use CELs, please refer to [this guide](<https://github.com/google/cel-spec/blob/master/doc/langdef.md#standard-definitions>).
    /// For more information about the fields used for filtering, please refer to [this definition](/protocol-buffers/documentation#saved_searches). Optional.
    #[prost(string, tag="3")]
    pub filter: ::prost::alloc::string::String,
    /// This field is only required if your user belongs to multiple organizations.
    #[prost(string, tag="4")]
    pub organization_id: ::prost::alloc::string::String,
}
/// The response of a call to `SavedSearchService_ListSavedSearchesResponse`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListSavedSearchesResponse {
    #[prost(message, repeated, tag="1")]
    pub saved_searches: ::prost::alloc::vec::Vec<SavedSearch>,
    #[prost(string, tag="2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// The request for a call to `SavedSearchService_CreateSavedSearch` to create a saved search.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateSavedSearchRequest {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub properties: ::core::option::Option<SavedSearchProperties>,
    /// This field is only required if your user belongs to multiple organizations.
    #[prost(string, tag="3")]
    pub organization_id: ::prost::alloc::string::String,
}
/// The response for a call to `SavedSearchService_CreateSavedResponse`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateSavedSearchResponse {
    #[prost(message, optional, tag="1")]
    pub saved_search: ::core::option::Option<SavedSearch>,
}
/// The request for a call to `SavedSearchService_DeleteSavedSearch` to delete a saved search.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteSavedSearchRequest {
    #[prost(string, tag="1")]
    pub saved_search_id: ::prost::alloc::string::String,
}
/// The response of a call to `SavedSearchService_DeleteSavedSearch`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteSavedSearchResponse {
}
/// The request for a call to `SavedSearchService_BatchDeleteSavedSearches` to delete saved searches.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchDeleteSavedSearchesRequest {
    /// Limit of 1000 searches per batch
    #[prost(string, repeated, tag="1")]
    pub saved_search_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// The response of a call to `SavedSearchService_BatchDeleteSavedSearches`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchDeleteSavedSearchesResponse {
}
/// The request for a call to `SavedSearchService_UpdateSavedSearch` to update a saved search.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateSavedSearchRequest {
    /// The saved search to update.
    #[prost(message, optional, tag="1")]
    pub saved_search: ::core::option::Option<SavedSearch>,
    /// The list of fields to be updated. The fields available to be updated are `name` and `properties`.
    #[prost(message, optional, tag="2")]
    pub update_mask: ::core::option::Option<::pbjson_types::FieldMask>,
}
/// The response of a call to `SavedSearchService_UpdateSavedSearch`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateSavedSearchResponse {
    #[prost(message, optional, tag="1")]
    pub saved_search: ::core::option::Option<SavedSearch>,
}
include!("sift.saved_searches.v1.tonic.rs");
include!("sift.saved_searches.v1.serde.rs");
// @@protoc_insertion_point(module)