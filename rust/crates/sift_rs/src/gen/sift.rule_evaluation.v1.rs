// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EvaluateRulesRequest {
    #[prost(message, optional, tag="6")]
    pub annotation_options: ::core::option::Option<EvaluateRulesAnnotationOptions>,
    /// Only required if your user belongs to multiple organizations
    #[prost(string, tag="7")]
    pub organization_id: ::prost::alloc::string::String,
    /// If this request creates a report, this field will be used as the report name.
    #[prost(string, optional, tag="8")]
    pub report_name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(oneof="evaluate_rules_request::Time", tags="1, 2")]
    pub time: ::core::option::Option<evaluate_rules_request::Time>,
    #[prost(oneof="evaluate_rules_request::Mode", tags="3, 4, 5")]
    pub mode: ::core::option::Option<evaluate_rules_request::Mode>,
}
/// Nested message and enum types in `EvaluateRulesRequest`.
pub mod evaluate_rules_request {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Time {
        #[prost(message, tag="1")]
        Run(super::super::super::common::r#type::v1::ResourceIdentifier),
        #[prost(message, tag="2")]
        Assets(super::AssetsTimeRange),
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Mode {
        #[prost(message, tag="3")]
        Rules(super::EvaluateRulesFromCurrentRuleVersions),
        #[prost(message, tag="4")]
        RuleVersions(super::EvaluateRulesFromRuleVersions),
        #[prost(message, tag="5")]
        ReportTemplate(super::EvaluateRulesFromReportTemplate),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AssetsTimeRange {
    #[prost(message, optional, tag="1")]
    pub assets: ::core::option::Option<super::super::common::r#type::v1::NamedResources>,
    #[prost(message, optional, tag="2")]
    pub start_time: ::core::option::Option<::pbjson_types::Timestamp>,
    #[prost(message, optional, tag="3")]
    pub end_time: ::core::option::Option<::pbjson_types::Timestamp>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EvaluateRulesFromCurrentRuleVersions {
    #[prost(message, optional, tag="1")]
    pub rules: ::core::option::Option<super::super::common::r#type::v1::ResourceIdentifiers>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EvaluateRulesFromReportTemplate {
    #[prost(message, optional, tag="1")]
    pub report_template: ::core::option::Option<super::super::common::r#type::v1::ResourceIdentifier>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EvaluateRulesFromRuleVersions {
    #[prost(string, repeated, tag="1")]
    pub rule_version_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EvaluateRulesAnnotationOptions {
    #[prost(message, optional, tag="1")]
    pub tags: ::core::option::Option<super::super::common::r#type::v1::NamedResources>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EvaluateRulesResponse {
    #[prost(int32, tag="1")]
    pub created_annotation_count: i32,
    /// A report will be generated if rules were evaluated against a run.
    #[prost(string, optional, tag="2")]
    pub report_id: ::core::option::Option<::prost::alloc::string::String>,
    /// This will be set if the job has an extended run time and is being processed asynchronously.
    #[prost(string, optional, tag="3")]
    pub job_id: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EvaluateRulesPreviewRequest {
    /// Only required if your user belongs to multiple organizations
    #[prost(string, tag="7")]
    pub organization_id: ::prost::alloc::string::String,
    #[prost(oneof="evaluate_rules_preview_request::Time", tags="1")]
    pub time: ::core::option::Option<evaluate_rules_preview_request::Time>,
    #[prost(oneof="evaluate_rules_preview_request::Mode", tags="3, 4, 5, 6")]
    pub mode: ::core::option::Option<evaluate_rules_preview_request::Mode>,
}
/// Nested message and enum types in `EvaluateRulesPreviewRequest`.
pub mod evaluate_rules_preview_request {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Time {
        #[prost(message, tag="1")]
        Run(super::super::super::common::r#type::v1::ResourceIdentifier),
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Mode {
        #[prost(message, tag="3")]
        Rules(super::EvaluateRulesFromCurrentRuleVersions),
        #[prost(message, tag="4")]
        RuleVersions(super::EvaluateRulesFromRuleVersions),
        #[prost(message, tag="5")]
        ReportTemplate(super::EvaluateRulesFromReportTemplate),
        #[prost(message, tag="6")]
        RuleConfigs(super::EvaluateRulesFromRuleConfigs),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EvaluateRulesFromRuleConfigs {
    #[prost(message, repeated, tag="1")]
    pub configs: ::prost::alloc::vec::Vec<super::super::rules::v1::UpdateRuleRequest>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EvaluateRulesPreviewResponse {
    #[prost(int32, tag="1")]
    pub created_annotation_count: i32,
    #[prost(message, repeated, tag="2")]
    pub dry_run_annotations: ::prost::alloc::vec::Vec<super::super::rules::v1::DryRunAnnotation>,
}
include!("sift.rule_evaluation.v1.tonic.rs");
include!("sift.rule_evaluation.v1.serde.rs");
// @@protoc_insertion_point(module)