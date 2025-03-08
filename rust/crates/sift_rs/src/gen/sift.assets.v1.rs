// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Asset {
    #[prost(string, tag="1")]
    pub asset_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub organization_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag="5")]
    pub created_date: ::core::option::Option<::pbjson_types::Timestamp>,
    #[prost(string, tag="6")]
    pub created_by_user_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag="7")]
    pub modified_date: ::core::option::Option<::pbjson_types::Timestamp>,
    #[prost(string, tag="8")]
    pub modified_by_user_id: ::prost::alloc::string::String,
    /// The names of the tags to associate with this asset.
    #[prost(string, repeated, tag="9")]
    pub tags: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// The request for a call to `AssetService_ListAssets`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListAssetsRequest {
    /// The maximum number of assets to return.
    /// The service may return fewer than this value.
    /// If unspecified, at most 50 assets will be returned.
    /// The maximum value is 1000; values above 1000 will be coerced to 1000.
    #[prost(uint32, tag="1")]
    pub page_size: u32,
    /// A page token, received from a previous `ListAssets` call.
    /// Provide this to retrieve the subsequent page.
    /// When paginating, all other parameters provided to `ListAssets` must match
    /// the call that provided the page token.
    #[prost(string, tag="2")]
    pub page_token: ::prost::alloc::string::String,
    /// A [Common Expression Language (CEL)](<https://github.com/google/cel-spec>) filter string.
    /// Available fields to filter by are `asset_id`, `created_by_user_id`, `modified_by_user_id`,
    /// `created_date`, `modified_date`, and `name`.
    /// For further information about how to use CELs, please refer to [this guide](<https://github.com/google/cel-spec/blob/master/doc/langdef.md#standard-definitions>).
    /// For more information about the fields used for filtering, please refer to [this definition](/docs/api/grpc/protocol-buffers/assets#asset). Optional.
    #[prost(string, tag="3")]
    pub filter: ::prost::alloc::string::String,
    /// How to order the retrieved assets. Formatted as a comma-separated string i.e. "FIELD_NAME\[ desc\],...".
    /// Available fields to order_by are `created_date` and `modified_date`.
    /// If left empty, items are ordered by `created_date` in ascending order (oldest-first).
    /// For more information about the format of this field, read [this](<https://google.aip.dev/132#ordering>)
    /// Example: "created_date desc,modified_date"
    #[prost(string, tag="4")]
    pub order_by: ::prost::alloc::string::String,
}
/// The result of a call to `AssetService_ListAssets`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListAssetsResponse {
    #[prost(message, repeated, tag="1")]
    pub assets: ::prost::alloc::vec::Vec<Asset>,
    #[prost(string, tag="5")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// The request for a call to `AssetService_DeleteAsset` to delete a single existing annotation by its asset_id.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteAssetRequest {
    /// The id of the asset to be deleted. Required.
    #[prost(string, tag="1")]
    pub asset_id: ::prost::alloc::string::String,
}
/// The response of a call to `AssetService_DeleteAsset`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteAssetResponse {
}
/// The request for a call to `AssetService_GetAsset` to retrieve a single existing asset by its asset_id.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAssetRequest {
    /// The id of the asset to be retrieved. Required.
    #[prost(string, tag="1")]
    pub asset_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAssetResponse {
    #[prost(message, optional, tag="1")]
    pub asset: ::core::option::Option<Asset>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateAssetRequest {
    /// The asset to update. The asset's `asset_id` field is used to identify asset run to update
    /// and is required.
    #[prost(message, optional, tag="1")]
    pub asset: ::core::option::Option<Asset>,
    /// The list of fields to be updated. Currently, the only field available to be updated is `tags`.
    #[prost(message, optional, tag="2")]
    pub update_mask: ::core::option::Option<::pbjson_types::FieldMask>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateAssetResponse {
    #[prost(message, optional, tag="1")]
    pub asset: ::core::option::Option<Asset>,
}
include!("sift.assets.v1.tonic.rs");
include!("sift.assets.v1.serde.rs");
// @@protoc_insertion_point(module)