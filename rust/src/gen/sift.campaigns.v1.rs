// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Campaign {
    #[prost(string, tag="1")]
    pub campaign_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub organization_id: ::prost::alloc::string::String,
    #[prost(string, optional, tag="3")]
    pub client_key: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, tag="4")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, optional, tag="5")]
    pub description: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, tag="6")]
    pub created_by_user_id: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub modified_by_user_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag="8")]
    pub created_date: ::core::option::Option<::pbjson_types::Timestamp>,
    #[prost(message, optional, tag="9")]
    pub modified_date: ::core::option::Option<::pbjson_types::Timestamp>,
    #[prost(message, optional, tag="10")]
    pub archived_date: ::core::option::Option<::pbjson_types::Timestamp>,
    #[prost(message, repeated, tag="11")]
    pub tags: ::prost::alloc::vec::Vec<super::super::tags::v1::TagRef>,
    /// A campaign, at its core, is a list of reports
    #[prost(message, repeated, tag="12")]
    pub reports: ::prost::alloc::vec::Vec<CampaignReport>,
    /// If this campaign was created by duplicating another campaign, that other campaign will be referenced here
    #[prost(string, optional, tag="13")]
    pub created_from_campaign_id: ::core::option::Option<::prost::alloc::string::String>,
}
/// A top-level summary of the report's rules is provided here so clients needn't aggregate this information themselves.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CampaignReport {
    /// Direct reference to the report. Set this on the client to tell the backend which report you're referring to.
    #[prost(string, tag="1")]
    pub report_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub report_name: ::prost::alloc::string::String,
    /// How many annotations exist under the report's various rules.
    #[prost(uint32, tag="3")]
    pub num_annotations: u32,
    /// How many rules from this report were never triggered and generated NO annotations.
    #[prost(uint32, tag="4")]
    pub num_passed_rules: u32,
    /// How many rules have annotations which are ALL marked as accepted.
    #[prost(uint32, tag="5")]
    pub num_accepted_rules: u32,
    /// How many rules have ANY annotations which are marked as failed.
    #[prost(uint32, tag="6")]
    pub num_failed_rules: u32,
    /// How many rules have annotations of which SOME are marked as open and NONE are marked as failed.
    #[prost(uint32, tag="7")]
    pub num_open_rules: u32,
}
/// The request for a call to `CampaignService_GetCampaign` to retrieve a campaign.
/// If `campaign_id` is provided then all other arguments will be ignored.
/// The `organization_id` argument is only required if using `client_key` and the user belongs to multiple organizations.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCampaignRequest {
    #[prost(string, tag="1")]
    pub campaign_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub client_key: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub organization_id: ::prost::alloc::string::String,
}
/// The response of a call to `CampaignService_GetCampaign` to retrieve a campaign.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCampaignResponse {
    #[prost(message, optional, tag="1")]
    pub campaign: ::core::option::Option<Campaign>,
}
/// The request of a call to `CampaignService_CreateCampaign` to create a campaign.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateCampaignRequest {
    /// The descriptive display name of the created campaign
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// A custom description for the campaign
    #[prost(string, optional, tag="2")]
    pub description: ::core::option::Option<::prost::alloc::string::String>,
    /// Tags to be applied to the new campaign
    #[prost(message, optional, tag="3")]
    pub tags: ::core::option::Option<super::super::common::r#type::v1::NamedResources>,
    /// `organization_id` is only required if your user belongs to multiple organizations
    #[prost(string, tag="4")]
    pub organization_id: ::prost::alloc::string::String,
    /// User-specified unique identifier.
    #[prost(string, optional, tag="5")]
    pub client_key: ::core::option::Option<::prost::alloc::string::String>,
    /// Initialize the campaign, or leave this field empty to create an empty campaign to be populated later
    #[prost(message, optional, tag="6")]
    pub create_from: ::core::option::Option<CreateCampaignFrom>,
}
/// Campaigns can be created from a few different sources.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateCampaignFrom {
    #[prost(oneof="create_campaign_from::Initializer", tags="1, 2, 3")]
    pub initializer: ::core::option::Option<create_campaign_from::Initializer>,
}
/// Nested message and enum types in `CreateCampaignFrom`.
pub mod create_campaign_from {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Initializer {
        /// Create a campaign directly from a set of reports
        #[prost(message, tag="1")]
        Reports(super::super::super::common::r#type::v1::ResourceIdentifiers),
        /// Create a campaign from a set of runs by collecting all the reports generated by those runs
        #[prost(message, tag="2")]
        Runs(super::super::super::common::r#type::v1::ResourceIdentifiers),
        /// Duplicate another campaign
        #[prost(message, tag="3")]
        OtherCampaign(super::super::super::common::r#type::v1::ResourceIdentifier),
    }
}
/// The response of a call to `CampaignService_CreateCampaign` to create a campaign.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateCampaignResponse {
    #[prost(message, optional, tag="1")]
    pub campaign: ::core::option::Option<Campaign>,
}
/// The request for a call to `CampaignService_ListCampaigns` to retrieve campaigns.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListCampaignsRequest {
    /// The maximum number of campaigns to return. The service may return fewer than this value.
    /// If unspecified, at most 50 campaigns will be returned. The maximum value is 1000; values above
    /// 1000 will be coerced to 1000. Optional.
    #[prost(uint32, tag="1")]
    pub page_size: u32,
    /// A page token, received from a previous `ListCampaigns` call.
    /// Provide this to retrieve the subsequent page.
    /// When paginating, all other parameters provided to `ListCampaigns` must match
    /// the call that provided the page token. Optional.
    #[prost(string, tag="2")]
    pub page_token: ::prost::alloc::string::String,
    /// A [Common Expression Language (CEL)](<https://github.com/google/cel-spec>) filter string.
    /// Available fields to filter by are:
    /// `created_by_user_id`, `tag_id`, `tag_name`, `report_id`, `report_name`, `campaign_id`, `client_key`, `description`, `run_id`, and `name`.
    /// For further information about how to use CELs, please refer to [this guide](<https://github.com/google/cel-spec/blob/master/doc/langdef.md#standard-definitions>).
    /// For more information about the fields used for filtering, please refer to this definition. Optional.
    #[prost(string, tag="3")]
    pub filter: ::prost::alloc::string::String,
    /// This field is only required if your user belongs to multiple organizations.
    #[prost(string, tag="4")]
    pub organization_id: ::prost::alloc::string::String,
    /// If `true` then archived campaigns will be included in the query. Defaults to `false`.
    #[prost(bool, tag="5")]
    pub include_archived: bool,
    /// How to order the retrieved campaigns. Formatted as a comma-separated string i.e. "FIELD_NAME\[ desc\],...".
    /// Available fields to order_by are `created_date` and `modified_date`.
    /// If left empty, items are ordered by `created_date` in ascending order (oldest-first).
    /// For more information about the format of this field, read [this](<https://google.aip.dev/132#ordering>)
    /// Example: "created_date desc,modified_date"
    #[prost(string, tag="6")]
    pub order_by: ::prost::alloc::string::String,
}
/// The response of a call to `CampaignService_ListCampaignsResponse`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListCampaignsResponse {
    #[prost(message, repeated, tag="1")]
    pub campaigns: ::prost::alloc::vec::Vec<Campaign>,
    #[prost(string, tag="2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// The request for a call to `CampaignService_UpdateCampaign` to update a campaign. When updating
/// tags or rules, the update will perform a full replace. Additionally, when updating rules, only the rule ID or the rule client key
/// is required, but it is okay to provide both. If some rules contain only client keys and others only rule IDs, they will be consolidated.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateCampaignRequest {
    /// The campaign to update.
    #[prost(message, optional, tag="1")]
    pub campaign: ::core::option::Option<Campaign>,
    /// The list of fields to be updated. The fields available to be updated are `name`, `archived_date`, `description`, `tags`, and `reports`.
    #[prost(message, optional, tag="2")]
    pub update_mask: ::core::option::Option<::pbjson_types::FieldMask>,
}
/// The response of a call to `CampaignService_UpdateCampaign`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateCampaignResponse {
    #[prost(message, optional, tag="1")]
    pub campaign: ::core::option::Option<Campaign>,
}
include!("sift.campaigns.v1.tonic.rs");
include!("sift.campaigns.v1.serde.rs");
// @@protoc_insertion_point(module)