// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Report {
    #[prost(string, tag="1")]
    pub report_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub report_template_id: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub run_id: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub organization_id: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, optional, tag="6")]
    pub description: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, tag="7")]
    pub created_by_user_id: ::prost::alloc::string::String,
    #[prost(string, tag="8")]
    pub modified_by_user_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag="9")]
    pub created_date: ::core::option::Option<::pbjson_types::Timestamp>,
    #[prost(message, optional, tag="10")]
    pub modified_date: ::core::option::Option<::pbjson_types::Timestamp>,
    #[prost(message, repeated, tag="11")]
    pub summaries: ::prost::alloc::vec::Vec<ReportRuleSummary>,
    #[prost(message, repeated, tag="12")]
    pub tags: ::prost::alloc::vec::Vec<ReportTag>,
    #[prost(string, optional, tag="13")]
    pub rerun_from_report_id: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReportRuleSummary {
    #[prost(string, tag="1")]
    pub rule_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub rule_client_key: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub rule_version_id: ::prost::alloc::string::String,
    #[prost(uint32, tag="4")]
    pub rule_version_number: u32,
    #[prost(string, tag="5")]
    pub report_rule_version_id: ::prost::alloc::string::String,
    #[prost(uint32, tag="6")]
    pub num_open: u32,
    #[prost(uint32, tag="7")]
    pub num_failed: u32,
    #[prost(uint32, tag="8")]
    pub num_passed: u32,
    #[prost(enumeration="ReportRuleStatus", tag="9")]
    pub status: i32,
    #[prost(message, optional, tag="10")]
    pub status_details: ::core::option::Option<ReportRuleStatusDetails>,
    #[prost(message, optional, tag="11")]
    pub created_date: ::core::option::Option<::pbjson_types::Timestamp>,
    #[prost(message, optional, tag="12")]
    pub modified_date: ::core::option::Option<::pbjson_types::Timestamp>,
    #[prost(string, tag="13")]
    pub asset_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReportTag {
    #[prost(string, tag="1")]
    pub tag_name: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReportRuleStatusDetails {
    #[prost(oneof="report_rule_status_details::Details", tags="1, 2, 3, 4")]
    pub details: ::core::option::Option<report_rule_status_details::Details>,
}
/// Nested message and enum types in `ReportRuleStatusDetails`.
pub mod report_rule_status_details {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Details {
        #[prost(message, tag="1")]
        Created(super::ReportRuleStatusDetailsCreated),
        #[prost(message, tag="2")]
        Live(super::ReportRuleStatusDetailsLive),
        #[prost(message, tag="3")]
        Finished(super::ReportRuleStatusDetailsFinished),
        #[prost(message, tag="4")]
        Failed(super::ReportRuleStatusDetailsFailed),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReportRuleStatusDetailsCreated {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReportRuleStatusDetailsLive {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReportRuleStatusDetailsFinished {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReportRuleStatusDetailsFailed {
    #[prost(string, tag="1")]
    pub error_message: ::prost::alloc::string::String,
}
/// The request of a call to `ReportService_CreateReport` to create a report. A report can be created either via a report template
/// or an arbitrary report can be constructed depending on the variant of the `request` field.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateReportRequest {
    #[prost(string, tag="3")]
    pub organization_id: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub run_id: ::prost::alloc::string::String,
    /// Override the name of the report. If not provided, the name will be generated based on the report template or run.
    #[prost(string, optional, tag="5")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(oneof="create_report_request::Request", tags="1, 2")]
    pub request: ::core::option::Option<create_report_request::Request>,
}
/// Nested message and enum types in `CreateReportRequest`.
pub mod create_report_request {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Request {
        #[prost(message, tag="1")]
        ReportFromReportTemplateRequest(super::CreateReportFromReportTemplateRequest),
        #[prost(message, tag="2")]
        ReportFromRulesRequest(super::CreateReportFromRulesRequest),
    }
}
/// The response of a call to `ReportService_CreateReport` to create a report.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateReportResponse {
    #[prost(message, optional, tag="1")]
    pub report: ::core::option::Option<Report>,
}
/// Used to create a report from a report template.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateReportFromReportTemplateRequest {
    #[prost(string, tag="1")]
    pub report_template_id: ::prost::alloc::string::String,
}
/// Used to construct an arbitrary report for an arbitrary set of rules. Rules can be specified either by rule ID
/// or client key based on the variant used in the `rule_identifiers` field.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateReportFromRulesRequest {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, optional, tag="2")]
    pub description: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, repeated, tag="3")]
    pub tag_names: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(oneof="create_report_from_rules_request::RuleIdentifiers", tags="4, 5")]
    pub rule_identifiers: ::core::option::Option<create_report_from_rules_request::RuleIdentifiers>,
}
/// Nested message and enum types in `CreateReportFromRulesRequest`.
pub mod create_report_from_rules_request {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum RuleIdentifiers {
        #[prost(message, tag="4")]
        RuleIds(super::CreateReportRequestRuleIds),
        #[prost(message, tag="5")]
        RuleClientKeys(super::CreateReportRequestClientKeys),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateReportRequestRuleIds {
    #[prost(string, repeated, tag="1")]
    pub rule_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateReportRequestClientKeys {
    #[prost(string, repeated, tag="1")]
    pub rule_client_keys: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// The request for a call to `ReportService_GetReport` to retrieve a report template.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetReportRequest {
    #[prost(string, tag="1")]
    pub report_id: ::prost::alloc::string::String,
}
/// The request of a call to `ReportService_GetReport` to retrieve a report template.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetReportResponse {
    #[prost(message, optional, tag="1")]
    pub report: ::core::option::Option<Report>,
}
/// The request for a call to `ReportService_ListReports` to retrieve report.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListReportsRequest {
    /// The maximum number of reports to return. The service may return fewer than this value.
    /// If unspecified, at most 50 reports will be returned. The maximum value is 1000; values above
    /// 1000 will be coerced to 1000. Optional.
    #[prost(uint32, tag="1")]
    pub page_size: u32,
    /// A page token, received from a previous `ListReports` call.
    /// Provide this to retrieve the subsequent page.
    /// When paginating, all other parameters provided to `ListReports` must match
    /// the call that provided the page token. Optional.
    #[prost(string, tag="2")]
    pub page_token: ::prost::alloc::string::String,
    /// A [Common Expression Language (CEL)](<https://github.com/google/cel-spec>) filter string.
    /// Available fields to filter by are `report_id`, `report_template_id`, `tag_name`, `name`, and `run_id`.
    /// For further information about how to use CELs, please refer to [this guide](<https://github.com/google/cel-spec/blob/master/doc/langdef.md#standard-definitions>).
    /// For more information about the fields used for filtering, please refer to [this definition](/api/grpc/protocol_buffers/reports#report). Optional.
    #[prost(string, tag="3")]
    pub filter: ::prost::alloc::string::String,
    /// This field is only required if your user belongs to multiple organizations.
    #[prost(string, tag="4")]
    pub organization_id: ::prost::alloc::string::String,
    /// How to order the retrieved reports. Formatted as a comma-separated string i.e. "<field_name>\[ desc\],...".
    /// Available fields to order_by are `created_date` and `modified_date`.
    /// If left empty, items are ordered by `created_date` in ascending order (oldest-first).
    /// For more information about the format of this field, read [this](<https://google.aip.dev/132#ordering>)
    /// Example: "created_date desc,modified_date"
    #[prost(string, tag="5")]
    pub order_by: ::prost::alloc::string::String,
}
/// The response of a call to `ReportService_ListReportsResponse`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListReportsResponse {
    #[prost(message, repeated, tag="1")]
    pub reports: ::prost::alloc::vec::Vec<Report>,
    #[prost(string, tag="2")]
    pub next_page_token: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RerunReportRequest {
    #[prost(string, tag="1")]
    pub report_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RerunReportResponse {
    #[prost(string, tag="1")]
    pub job_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub report_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CancelReportRequest {
    #[prost(string, tag="1")]
    pub report_id: ::prost::alloc::string::String,
}
/// no response fields
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CancelReportResponse {
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ReportRuleStatus {
    Unspecified = 0,
    Created = 1,
    Live = 2,
    Finished = 3,
    Failed = 4,
    Canceled = 5,
}
impl ReportRuleStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ReportRuleStatus::Unspecified => "REPORT_RULE_STATUS_UNSPECIFIED",
            ReportRuleStatus::Created => "REPORT_RULE_STATUS_CREATED",
            ReportRuleStatus::Live => "REPORT_RULE_STATUS_LIVE",
            ReportRuleStatus::Finished => "REPORT_RULE_STATUS_FINISHED",
            ReportRuleStatus::Failed => "REPORT_RULE_STATUS_FAILED",
            ReportRuleStatus::Canceled => "REPORT_RULE_STATUS_CANCELED",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "REPORT_RULE_STATUS_UNSPECIFIED" => Some(Self::Unspecified),
            "REPORT_RULE_STATUS_CREATED" => Some(Self::Created),
            "REPORT_RULE_STATUS_LIVE" => Some(Self::Live),
            "REPORT_RULE_STATUS_FINISHED" => Some(Self::Finished),
            "REPORT_RULE_STATUS_FAILED" => Some(Self::Failed),
            "REPORT_RULE_STATUS_CANCELED" => Some(Self::Canceled),
            _ => None,
        }
    }
}
include!("sift.reports.v1.tonic.rs");
include!("sift.reports.v1.serde.rs");
// @@protoc_insertion_point(module)