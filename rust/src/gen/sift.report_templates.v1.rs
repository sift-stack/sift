// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReportTemplate {
    #[prost(string, tag="1")]
    pub report_template_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub organization_id: ::prost::alloc::string::String,
    #[prost(string, optional, tag="3")]
    pub client_key: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, tag="4")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, optional, tag="5")]
    pub description: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag="6")]
    pub archived_date: ::core::option::Option<::pbjson_types::Timestamp>,
    #[prost(string, tag="7")]
    pub created_by_user_id: ::prost::alloc::string::String,
    #[prost(string, tag="8")]
    pub modified_by_user_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag="9")]
    pub created_date: ::core::option::Option<::pbjson_types::Timestamp>,
    #[prost(message, optional, tag="10")]
    pub modified_date: ::core::option::Option<::pbjson_types::Timestamp>,
    #[prost(message, repeated, tag="11")]
    pub rules: ::prost::alloc::vec::Vec<ReportTemplateRule>,
    #[prost(message, repeated, tag="12")]
    pub tags: ::prost::alloc::vec::Vec<ReportTemplateTag>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReportTemplateRule {
    #[prost(string, tag="1")]
    pub rule_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub rule_version_id: ::prost::alloc::string::String,
    #[prost(uint32, tag="3")]
    pub rule_version_number: u32,
    #[prost(string, tag="4")]
    pub client_key: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReportTemplateTag {
    #[prost(string, tag="1")]
    pub tag_name: ::prost::alloc::string::String,
}
/// The request for a call to `ReportTemplateService_GetReportTemplate` to retrieve a report template.
/// If `report_template_id` is provided then all other arguments will be ignored. The argument `report_template_id`
/// should not be used together with `client_key`. The `organization_id` argument is only required
/// if using `client_key` and the user belongs to multiple organizations.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetReportTemplateRequest {
    #[prost(string, tag="1")]
    pub report_template_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub client_key: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub organization_id: ::prost::alloc::string::String,
}
/// The request of a call to `ReportTemplateService_GetReportTemplate` to retrieve a report template.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetReportTemplateResponse {
    #[prost(message, optional, tag="1")]
    pub report_template: ::core::option::Option<ReportTemplate>,
}
/// The request of a call to `ReportTemplateService_CreateReportTemplate` to create a report template.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateReportTemplateRequest {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, optional, tag="2")]
    pub client_key: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="3")]
    pub description: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, repeated, tag="4")]
    pub tag_names: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// `organization_id` is only required if your user belongs to multiple organizations
    #[prost(string, tag="6")]
    pub organization_id: ::prost::alloc::string::String,
    #[prost(oneof="create_report_template_request::RuleIdentifiers", tags="7, 8")]
    pub rule_identifiers: ::core::option::Option<create_report_template_request::RuleIdentifiers>,
}
/// Nested message and enum types in `CreateReportTemplateRequest`.
pub mod create_report_template_request {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum RuleIdentifiers {
        #[prost(message, tag="7")]
        RuleIds(super::CreateReportTemplateRequestRuleIds),
        #[prost(message, tag="8")]
        RuleClientKeys(super::CreateReportTemplateRequestClientKeys),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateReportTemplateRequestRuleIds {
    #[prost(string, repeated, tag="1")]
    pub rule_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateReportTemplateRequestClientKeys {
    #[prost(string, repeated, tag="1")]
    pub rule_client_keys: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// The response of a call to `ReportTemplateService_CreateReportTemplate` to create a report template.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateReportTemplateResponse {
    #[prost(message, optional, tag="1")]
    pub report_template: ::core::option::Option<ReportTemplate>,
}
/// The request for a call to `ReportTemplateService_ListReportTemplates` to retrieve report templates.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListReportTemplatesRequest {
    /// The maximum number of report templates to return. The service may return fewer than this value.
    /// If unspecified, at most 50 report templates will be returned. The maximum value is 1000; values above
    /// 1000 will be coerced to 1000. Optional.
    #[prost(uint32, tag="1")]
    pub page_size: u32,
    /// A page token, received from a previous `ListReportTemplates` call.
    /// Provide this to retrieve the subsequent page.
    /// When paginating, all other parameters provided to `ListReportTemplates` must match
    /// the call that provided the page token. Optional.
    #[prost(string, tag="2")]
    pub page_token: ::prost::alloc::string::String,
    /// A [Common Expression Language (CEL)](<https://github.com/google/cel-spec>) filter string.
    /// Available fields to filter by are `report_template_id`, `tag_id`, `tag_name`, `client_key`, and `name`.
    /// For further information about how to use CELs, please refer to [this guide](<https://github.com/google/cel-spec/blob/master/doc/langdef.md#standard-definitions>).
    /// For more information about the fields used for filtering, please refer to [this definition](/protocol-buffers/documentation#report_templates). Optional.
    #[prost(string, tag="3")]
    pub filter: ::prost::alloc::string::String,
    /// This field is only required if your user belongs to multiple organizations.
    #[prost(string, tag="4")]
    pub organization_id: ::prost::alloc::string::String,
    /// If `true` then archived report templates will be included in the query. Defaults to `false`.
    #[prost(bool, tag="5")]
    pub include_archived: bool,
    /// How to order the retrieved report templates. Formatted as a comma-separated string i.e. "<field_name>\[ desc\],...".
    /// Available fields to order_by are `created_date` and `modified_date`.
    /// If left empty, items are ordered by `created_date` in ascending order (oldest-first).
    /// For more information about the format of this field, read [this](<https://google.aip.dev/132#ordering>)
    /// Example: "created_date desc,modified_date"
    #[prost(string, tag="6")]
    pub order_by: ::prost::alloc::string::String,
}
/// The response of a call to `ReportTemplateService_ListReportTemplatesResponse`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListReportTemplatesResponse {
    #[prost(message, repeated, tag="1")]
    pub report_templates: ::prost::alloc::vec::Vec<ReportTemplate>,
    #[prost(string, tag="2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// The request for a call to `ReportTemplateService_UpdateReportTemplate` to update a report template. When updating
/// tags or rules, the update will perform a full replace. Additionally, when updating rules, only the rule ID or the rule client key
/// is required, but it is okay to provide both. If some rules contain only client keys and others only rule IDs, they will be consolidated.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateReportTemplateRequest {
    /// The report template to update.
    #[prost(message, optional, tag="1")]
    pub report_template: ::core::option::Option<ReportTemplate>,
    /// The list of fields to be updated. The fields available to be updated are `name`, `archived_date`, `description`, `tags`, and `rules`.
    #[prost(message, optional, tag="2")]
    pub update_mask: ::core::option::Option<::pbjson_types::FieldMask>,
}
/// The response of a call to `ReportTemplateService_UpdateReportTemplate`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateReportTemplateResponse {
    #[prost(message, optional, tag="1")]
    pub report_template: ::core::option::Option<ReportTemplate>,
}
include!("sift.report_templates.v1.tonic.rs");
include!("sift.report_templates.v1.serde.rs");
// @@protoc_insertion_point(module)