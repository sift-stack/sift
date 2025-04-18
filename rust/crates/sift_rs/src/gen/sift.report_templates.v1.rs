// @generated
// This file is @generated by prost-build.
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
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetReportTemplateResponse {
    #[prost(message, optional, tag="1")]
    pub report_template: ::core::option::Option<ReportTemplate>,
}
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
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateReportTemplateResponse {
    #[prost(message, optional, tag="1")]
    pub report_template: ::core::option::Option<ReportTemplate>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListReportTemplatesRequest {
    #[prost(uint32, tag="1")]
    pub page_size: u32,
    #[prost(string, tag="2")]
    pub page_token: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub filter: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub organization_id: ::prost::alloc::string::String,
    #[prost(bool, tag="5")]
    pub include_archived: bool,
    #[prost(string, tag="6")]
    pub order_by: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListReportTemplatesResponse {
    #[prost(message, repeated, tag="1")]
    pub report_templates: ::prost::alloc::vec::Vec<ReportTemplate>,
    #[prost(string, tag="2")]
    pub next_page_token: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateReportTemplateRequest {
    #[prost(message, optional, tag="1")]
    pub report_template: ::core::option::Option<ReportTemplate>,
    #[prost(message, optional, tag="2")]
    pub update_mask: ::core::option::Option<::pbjson_types::FieldMask>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateReportTemplateResponse {
    #[prost(message, optional, tag="1")]
    pub report_template: ::core::option::Option<ReportTemplate>,
}
include!("sift.report_templates.v1.tonic.rs");
include!("sift.report_templates.v1.serde.rs");
// @@protoc_insertion_point(module)