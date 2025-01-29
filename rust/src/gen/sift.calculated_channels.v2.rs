// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CalculatedChannel {
    #[prost(string, tag="1")]
    pub calculated_channel_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub organization_id: ::prost::alloc::string::String,
    #[prost(string, optional, tag="3")]
    pub client_key: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag="4")]
    pub archived_date: ::core::option::Option<::pbjson_types::Timestamp>,
    #[prost(string, tag="5")]
    pub version_id: ::prost::alloc::string::String,
    #[prost(uint32, tag="6")]
    pub version: u32,
    #[prost(string, tag="7")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag="8")]
    pub description: ::prost::alloc::string::String,
    #[prost(string, tag="9")]
    pub change_message: ::prost::alloc::string::String,
    #[prost(string, tag="10")]
    pub user_notes: ::prost::alloc::string::String,
    #[prost(string, optional, tag="18")]
    pub units: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag="11")]
    pub created_date: ::core::option::Option<::pbjson_types::Timestamp>,
    #[prost(message, optional, tag="12")]
    pub modified_date: ::core::option::Option<::pbjson_types::Timestamp>,
    #[prost(message, optional, tag="15")]
    pub calculated_channel_configuration: ::core::option::Option<CalculatedChannelConfiguration>,
    #[prost(string, tag="16")]
    pub created_by_user_id: ::prost::alloc::string::String,
    #[prost(string, tag="17")]
    pub modified_by_user_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CalculatedChannelConfiguration {
    #[prost(message, optional, tag="1")]
    pub asset_configuration: ::core::option::Option<CalculatedChannelAssetConfiguration>,
    #[prost(message, optional, tag="2")]
    pub query_configuration: ::core::option::Option<CalculatedChannelQueryConfiguration>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CalculatedChannelAssetConfiguration {
    #[prost(oneof="calculated_channel_asset_configuration::AssetScope", tags="1, 2")]
    pub asset_scope: ::core::option::Option<calculated_channel_asset_configuration::AssetScope>,
}
/// Nested message and enum types in `CalculatedChannelAssetConfiguration`.
pub mod calculated_channel_asset_configuration {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct AssetSelection {
        #[prost(string, repeated, tag="1")]
        pub asset_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        #[prost(string, repeated, tag="2")]
        pub tag_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum AssetScope {
        #[prost(bool, tag="1")]
        AllAssets(bool),
        #[prost(message, tag="2")]
        Selection(AssetSelection),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CalculatedChannelQueryConfiguration {
    #[prost(oneof="calculated_channel_query_configuration::Query", tags="1")]
    pub query: ::core::option::Option<calculated_channel_query_configuration::Query>,
}
/// Nested message and enum types in `CalculatedChannelQueryConfiguration`.
pub mod calculated_channel_query_configuration {
    /// Sift Expression Language.
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Sel {
        #[prost(string, tag="1")]
        pub expression: ::prost::alloc::string::String,
        #[prost(message, repeated, tag="2")]
        pub expression_channel_references: ::prost::alloc::vec::Vec<super::CalculatedChannelAbstractChannelReference>,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Query {
        #[prost(message, tag="1")]
        Sel(Sel),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CalculatedChannelAbstractChannelReference {
    #[prost(string, tag="1")]
    pub channel_reference: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub channel_identifier: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CalculatedChannelValidationResult {
    #[prost(string, tag="1")]
    pub asset_id: ::prost::alloc::string::String,
    /// Only included if asset was named in the request
    #[prost(string, optional, tag="2")]
    pub asset_name: ::core::option::Option<::prost::alloc::string::String>,
    /// Includes all tags that have this asset
    #[prost(string, repeated, tag="3")]
    pub tag_names: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Names of the channels that asset does not have
    #[prost(string, repeated, tag="4")]
    pub missing_channels: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// The request for a call to `CalculatedChannelService_GetCalculatedChannel` to retrieve the latest version of a calculated channel.
/// If `calculated_channel_id` is provided then all other arguments will be ignored. The argument `calculated_channel_id`
/// should not be used together with `client_key`. The `organization_id` argument is only required
/// if using `client_key` and the user belongs to multiple organizations.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCalculatedChannelRequest {
    #[prost(string, tag="1")]
    pub calculated_channel_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub client_key: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub organization_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCalculatedChannelResponse {
    #[prost(message, optional, tag="1")]
    pub calculated_channel: ::core::option::Option<CalculatedChannel>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateCalculatedChannelRequest {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub description: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub user_notes: ::prost::alloc::string::String,
    #[prost(string, optional, tag="7")]
    pub units: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="4")]
    pub client_key: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag="5")]
    pub calculated_channel_configuration: ::core::option::Option<CalculatedChannelConfiguration>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateCalculatedChannelResponse {
    #[prost(message, optional, tag="1")]
    pub calculated_channel: ::core::option::Option<CalculatedChannel>,
    #[prost(message, repeated, tag="2")]
    pub inapplicable_assets: ::prost::alloc::vec::Vec<CalculatedChannelValidationResult>,
}
/// The request for a call to `CalculatedChannelService_ListCalculatedChannels` to retrieve lateset vesrions of calculated channels.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListCalculatedChannelsRequest {
    /// The maximum number of calculated channels to return. The service may return fewer than this value.
    /// If unspecified, at most 50 calculated channels will be returned. The maximum value is 1000; values above
    /// 1000 will be coerced to 1000. Optional.
    #[prost(uint32, tag="1")]
    pub page_size: u32,
    /// A page token, received from a previous `ListCalculatedChannels` call.
    /// Provide this to retrieve the subsequent page.
    /// When paginating, all other parameters provided to `ListCalculatedChannels` must match
    /// the call that provided the page token. Optional.
    #[prost(string, tag="2")]
    pub page_token: ::prost::alloc::string::String,
    /// A [Common Expression Language (CEL)](<https://github.com/google/cel-spec>) filter string.
    /// Available fields to filter by are `calculated_channel_id`, `client_key`, `name`, `asset_id`, `asset_name`, `tag_id`, `tag_name`, `version`, and `archived_date.
    /// For further information about how to use CELs, please refer to [this guide](<https://github.com/google/cel-spec/blob/master/doc/langdef.md#standard-definitions>).
    /// For more information about the fields used for filtering, please refer to [this definition](/docs/api/grpc/protocol-buffers/calculated_channels#calculated_channel). Optional.
    #[prost(string, tag="3")]
    pub filter: ::prost::alloc::string::String,
    /// This field is only required if your user belongs to multiple organizations.
    #[prost(string, tag="4")]
    pub organization_id: ::prost::alloc::string::String,
    /// How to order the retrieved calculated channels. Formatted as a comma-separated string i.e. "FIELD_NAME\[ desc\],...".
    /// Available fields to order_by are `created_date` and `modified_date`.
    /// If left empty, items are ordered by `created_date` in ascending order (oldest-first).
    /// For more information about the format of this field, read [this](<https://google.aip.dev/132#ordering>)
    /// Example: "created_date desc,modified_date"
    #[prost(string, tag="5")]
    pub order_by: ::prost::alloc::string::String,
}
/// The response of a call to `CalculatedChannelService_ListCalculatedChannelsResponse`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListCalculatedChannelsResponse {
    #[prost(message, repeated, tag="1")]
    pub calculated_channels: ::prost::alloc::vec::Vec<CalculatedChannel>,
    #[prost(string, tag="2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// The request for a call to `CalculatedChannelService_UpdateCalculatedChannel` to update a calculated channel. Updating a calculated
/// channel creates a new version of the calculated channel, leaving the previous untouched. If no update is deemed necessary, then the
/// the current version is returned. To archive calculated channel, specify `archived_date` in the `update mask` as well as a non-null
/// value for `archived_date` in the `calculated_channel` object. To unarchive a calculated channel, specify `archived_date` in the
/// `update mask` and a `null` value for `archived_date` in the `calculated_channel` object.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateCalculatedChannelRequest {
    /// The calculated channel to update.
    #[prost(message, optional, tag="1")]
    pub calculated_channel: ::core::option::Option<CalculatedChannel>,
    /// The list of fields to be updated. The fields available to be updated are `name`, `description`, `units`,
    /// `query_configuration`, `archived_date`, and `asset_configuration`.
    #[prost(message, optional, tag="2")]
    pub update_mask: ::core::option::Option<::pbjson_types::FieldMask>,
    /// Optional user notes to describe changes.
    #[prost(string, optional, tag="3")]
    pub user_notes: ::core::option::Option<::prost::alloc::string::String>,
}
/// The response of a call to `CalculatedChannelService_UpdateCalculatedChannel`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateCalculatedChannelResponse {
    #[prost(message, optional, tag="1")]
    pub calculated_channel: ::core::option::Option<CalculatedChannel>,
    #[prost(message, repeated, tag="2")]
    pub inapplicable_assets: ::prost::alloc::vec::Vec<CalculatedChannelValidationResult>,
}
/// The request for a call to `CalculatedChannelService_ListCalculatedChannelVersions` to retrieve versions
/// of a particular calculated channel. If `calculated_channel_id` is provided then `client_key` is ignored.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListCalculatedChannelVersionsRequest {
    #[prost(string, tag="1")]
    pub calculated_channel_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub client_key: ::prost::alloc::string::String,
    /// The maximum number of calculated channel versions to return. The service may return fewer than this value.
    /// If unspecified, at most 50 calculated channels will be returned. The maximum value is 1000; values above
    /// 1000 will be coerced to 1000. Optional.
    #[prost(uint32, tag="3")]
    pub page_size: u32,
    /// A page token, received from a previous `ListCalculatedChannelVersions` call.
    /// Provide this to retrieve the subsequent page.
    /// When paginating, all other parameters provided to `ListCalculatedChannelVersions` must match
    /// the call that provided the page token. Optional.
    #[prost(string, tag="4")]
    pub page_token: ::prost::alloc::string::String,
    /// A [Common Expression Language (CEL)](<https://github.com/google/cel-spec>) filter string.
    /// Available fields to filter by are `calculated_channel_id`, `client_key`, `name`, `asset_id`, `asset_name`, `tag_id`, `tag_name`, `version`, and `archived_date.
    /// For further information about how to use CELs, please refer to [this guide](<https://github.com/google/cel-spec/blob/master/doc/langdef.md#standard-definitions>).
    /// For more information about the fields used for filtering, please refer to [this definition](/docs/api/grpc/protocol-buffers/calculated_channels#calculated_channel). Optional.
    #[prost(string, tag="5")]
    pub filter: ::prost::alloc::string::String,
    /// This field is only required if your user belongs to multiple organizations.
    #[prost(string, tag="6")]
    pub organization_id: ::prost::alloc::string::String,
    /// How to order the retrieved calculated channel versions. Formatted as a comma-separated string i.e. "FIELD_NAME\[ desc\],...".
    /// Available fields to order_by are `created_date`, `modified_date`, and `version`.
    /// If left empty, items are ordered by `created_date` in ascending order (oldest-first).
    /// For more information about the format of this field, read [this](<https://google.aip.dev/132#ordering>)
    /// Example: "created_date desc,modified_date".
    #[prost(string, tag="7")]
    pub order_by: ::prost::alloc::string::String,
}
/// The response of a call to `CalculatedChannelService_ListCalculatedChannelVersionsResponse`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListCalculatedChannelVersionsResponse {
    #[prost(message, repeated, tag="1")]
    pub calculated_channel_versions: ::prost::alloc::vec::Vec<CalculatedChannel>,
    #[prost(string, tag="2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// The request for a call to `CalculatedChannelService_ResolveCalculatedChannel` to get the all possible calculated channels.
/// 1) If the calculated channel has a selection of assets and tags then those assets will be used as a base and then filtered down by the given assets and assets from the run.
/// 2) If the calculated channel is enabled for all assets then:
///    a) If the request has run and assets from the run will be used as a base and then filtered down by the given assets.
///    b) If the request has run and no assets then those assets will be used.
///    c) If the request has only assets then those assets will be used.
/// The `organization_id` argument is only required if using `client_key` and the user belongs to multiple organizations.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResolveCalculatedChannelRequest {
    /// This field is only required if your user belongs to multiple organizations.
    #[prost(string, tag="3")]
    pub organization_id: ::prost::alloc::string::String,
    /// Assets to be included or filtered through.  See above.
    #[prost(message, optional, tag="4")]
    pub assets: ::core::option::Option<super::super::common::r#type::v1::NamedResources>,
    /// Run to get assets to be included or filtered through.  See above.
    #[prost(message, optional, tag="5")]
    pub run: ::core::option::Option<super::super::common::r#type::v1::ResourceIdentifier>,
    #[prost(oneof="resolve_calculated_channel_request::CalculatedChannel", tags="1, 2")]
    pub calculated_channel: ::core::option::Option<resolve_calculated_channel_request::CalculatedChannel>,
}
/// Nested message and enum types in `ResolveCalculatedChannelRequest`.
pub mod resolve_calculated_channel_request {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum CalculatedChannel {
        /// The calculated channel to resolve.
        #[prost(message, tag="1")]
        CalculatedChannelId(super::super::super::common::r#type::v1::ResourceIdentifier),
        /// A configuration for the calculated channel to resolve.
        #[prost(message, tag="2")]
        CalculatedChannelConfiguration(super::CalculatedChannelConfiguration),
    }
}
/// The response of a call to `CalculatedChannelService_ResolveCalculatedChannel`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResolveCalculatedChannelResponse {
    /// If provided in the request, the calculated channel resolved.
    #[prost(message, optional, tag="1")]
    pub calculated_channel_id: ::core::option::Option<super::super::common::r#type::v1::ResourceIdentifier>,
    /// All resolved calculated channels.
    #[prost(message, repeated, tag="2")]
    pub resolved: ::prost::alloc::vec::Vec<resolve_calculated_channel_response::ResolvedCalculatedChannel>,
    /// All assets with any issues in resolution.
    #[prost(message, repeated, tag="3")]
    pub unresolved: ::prost::alloc::vec::Vec<resolve_calculated_channel_response::UnresolvedCalculatedChannel>,
}
/// Nested message and enum types in `ResolveCalculatedChannelResponse`.
pub mod resolve_calculated_channel_response {
    /// A specific calculated channel including the asset and exact channels to query.
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ResolvedCalculatedChannel {
        /// The name of the specific asset that was resolved.
        #[prost(string, tag="1")]
        pub asset_name: ::prost::alloc::string::String,
        /// The expression resolved including channel references.
        #[prost(message, optional, tag="2")]
        pub expression_request: ::core::option::Option<super::super::v1::ExpressionRequest>,
        #[prost(enumeration="super::super::super::common::r#type::v1::ChannelDataType", tag="3")]
        pub output_data_type: i32,
    }
    /// Any failure in resolution.
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct UnresolvedCalculatedChannel {
        /// The name of the specific asset that was not resolved.
        #[prost(string, tag="1")]
        pub asset_name: ::prost::alloc::string::String,
        #[prost(string, tag="2")]
        pub error_message: ::prost::alloc::string::String,
    }
}
/// The request of a call to `CalculatedChannelService_BatchResolveCalculatedChannels`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchResolveCalculatedChannelsRequest {
    /// All calculated channels to resolve.
    #[prost(message, repeated, tag="1")]
    pub requests: ::prost::alloc::vec::Vec<ResolveCalculatedChannelRequest>,
}
/// The response of a call to `CalculatedChannelService_BatchResolveCalculatedChannels`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchResolveCalculatedChannelsResponse {
    /// All calculated channels that attempted resolution.
    #[prost(message, repeated, tag="1")]
    pub responses: ::prost::alloc::vec::Vec<ResolveCalculatedChannelResponse>,
}
include!("sift.calculated_channels.v2.tonic.rs");
include!("sift.calculated_channels.v2.serde.rs");
// @@protoc_insertion_point(module)