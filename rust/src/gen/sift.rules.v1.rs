// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Rule {
    #[prost(string, tag="1")]
    pub rule_id: ::prost::alloc::string::String,
    #[deprecated]
    #[prost(string, tag="2")]
    pub asset_id: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub description: ::prost::alloc::string::String,
    #[prost(bool, tag="6")]
    pub is_enabled: bool,
    #[prost(message, optional, tag="7")]
    pub created_date: ::core::option::Option<::pbjson_types::Timestamp>,
    #[prost(message, optional, tag="8")]
    pub modified_date: ::core::option::Option<::pbjson_types::Timestamp>,
    #[prost(string, tag="9")]
    pub created_by_user_id: ::prost::alloc::string::String,
    #[prost(string, tag="10")]
    pub modified_by_user_id: ::prost::alloc::string::String,
    #[prost(string, tag="11")]
    pub organization_id: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="12")]
    pub conditions: ::prost::alloc::vec::Vec<RuleCondition>,
    #[prost(message, optional, tag="13")]
    pub rule_version: ::core::option::Option<RuleVersion>,
    /// client_key is a client provided identifier for the rule. It is immutable after rule creation.
    #[prost(string, tag="14")]
    pub client_key: ::prost::alloc::string::String,
    #[prost(message, optional, tag="15")]
    pub asset_configuration: ::core::option::Option<RuleAssetConfiguration>,
    #[prost(message, optional, tag="16")]
    pub contextual_channels: ::core::option::Option<ContextualChannels>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RuleCondition {
    #[prost(string, tag="1")]
    pub rule_condition_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub rule_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag="3")]
    pub expression: ::core::option::Option<RuleConditionExpression>,
    #[prost(message, optional, tag="5")]
    pub created_date: ::core::option::Option<::pbjson_types::Timestamp>,
    #[prost(message, optional, tag="6")]
    pub modified_date: ::core::option::Option<::pbjson_types::Timestamp>,
    #[prost(string, tag="7")]
    pub created_by_user_id: ::prost::alloc::string::String,
    #[prost(string, tag="8")]
    pub modified_by_user_id: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="9")]
    pub actions: ::prost::alloc::vec::Vec<RuleAction>,
    #[prost(string, tag="10")]
    pub rule_condition_version_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RuleAction {
    #[prost(string, tag="1")]
    pub rule_action_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub rule_condition_id: ::prost::alloc::string::String,
    #[prost(enumeration="ActionKind", tag="3")]
    pub action_type: i32,
    #[prost(message, optional, tag="4")]
    pub configuration: ::core::option::Option<RuleActionConfiguration>,
    #[prost(message, optional, tag="5")]
    pub created_date: ::core::option::Option<::pbjson_types::Timestamp>,
    #[prost(message, optional, tag="6")]
    pub modified_date: ::core::option::Option<::pbjson_types::Timestamp>,
    #[prost(string, tag="7")]
    pub created_by_user_id: ::prost::alloc::string::String,
    #[prost(string, tag="8")]
    pub modified_by_user_id: ::prost::alloc::string::String,
    #[prost(string, tag="9")]
    pub rule_action_version_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RuleAssetConfiguration {
    #[prost(string, repeated, tag="1")]
    pub asset_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag="2")]
    pub tag_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ContextualChannels {
    #[prost(message, repeated, tag="1")]
    pub channels: ::prost::alloc::vec::Vec<ChannelReference>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchRulesRequest {
    /// Max number of rules to return (returns all if omitted).
    #[prost(uint32, optional, tag="1")]
    pub limit: ::core::option::Option<u32>,
    /// Only applies if limit provided.
    #[prost(uint32, tag="2")]
    pub offset: u32,
    /// Order to sort results by (defaults to ascending).
    #[prost(enumeration="SearchOrder", optional, tag="3")]
    pub order: ::core::option::Option<i32>,
    #[prost(string, tag="4")]
    pub name_matches: ::prost::alloc::string::String,
    #[prost(bool, tag="5")]
    pub case_sensitive: bool,
    #[prost(bool, tag="6")]
    pub regexp: bool,
    #[prost(string, optional, tag="7")]
    pub order_by: ::core::option::Option<::prost::alloc::string::String>,
    /// If provided, only returns rules with the given ids
    #[prost(string, repeated, tag="8")]
    pub rule_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// If provided, only returns rules with the given asset ids
    #[prost(string, repeated, tag="9")]
    pub asset_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchRulesResponse {
    #[prost(uint32, tag="1")]
    pub count: u32,
    /// Conditions are not included in the search response. The latest version of the rule is returned.
    #[prost(message, repeated, tag="2")]
    pub rules: ::prost::alloc::vec::Vec<Rule>,
}
/// GetRuleRequest is used to retrieve a rule by rule_id or client_key. If both are provided, only rule_id will be used.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetRuleRequest {
    #[prost(string, tag="1")]
    pub rule_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub client_key: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetRuleResponse {
    #[prost(message, optional, tag="1")]
    pub rule: ::core::option::Option<Rule>,
}
/// BatchGetRulesRequest is used to retrieve rules by rule_ids or client_keys. If both are provided, both will be used to retrieve rules.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchGetRulesRequest {
    #[prost(string, repeated, tag="1")]
    pub rule_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag="2")]
    pub client_keys: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchGetRulesResponse {
    #[prost(message, repeated, tag="1")]
    pub rules: ::prost::alloc::vec::Vec<Rule>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateRuleRequest {
    #[prost(message, optional, tag="1")]
    pub update: ::core::option::Option<UpdateRuleRequest>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateRuleResponse {
    #[prost(string, tag="1")]
    pub rule_id: ::prost::alloc::string::String,
}
/// UpdateRuleRequest is used to create or update a rule. If the rule_id or client_key is provided, the rule will be updated. If not, a new rule will be created.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateRuleRequest {
    #[prost(string, optional, tag="1")]
    pub rule_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub description: ::prost::alloc::string::String,
    /// Deprecated - use asset_configuration instead.
    #[deprecated]
    #[prost(string, tag="4")]
    pub asset_id: ::prost::alloc::string::String,
    #[prost(bool, tag="5")]
    pub is_enabled: bool,
    #[prost(message, repeated, tag="6")]
    pub conditions: ::prost::alloc::vec::Vec<UpdateConditionRequest>,
    #[prost(string, tag="7")]
    pub organization_id: ::prost::alloc::string::String,
    #[prost(string, tag="8")]
    pub version_notes: ::prost::alloc::string::String,
    #[prost(string, optional, tag="9")]
    pub client_key: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag="10")]
    pub asset_configuration: ::core::option::Option<RuleAssetConfiguration>,
    #[prost(message, optional, tag="11")]
    pub contextual_channels: ::core::option::Option<ContextualChannels>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateConditionRequest {
    #[prost(string, optional, tag="1")]
    pub rule_condition_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag="3")]
    pub expression: ::core::option::Option<RuleConditionExpression>,
    #[prost(message, repeated, tag="4")]
    pub actions: ::prost::alloc::vec::Vec<UpdateActionRequest>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateActionRequest {
    #[prost(string, optional, tag="1")]
    pub rule_action_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(enumeration="ActionKind", tag="2")]
    pub action_type: i32,
    #[prost(message, optional, tag="3")]
    pub configuration: ::core::option::Option<RuleActionConfiguration>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateRuleResponse {
    #[prost(string, tag="1")]
    pub rule_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchUpdateRulesRequest {
    /// rules are limited 1000 rules at a time
    #[prost(message, repeated, tag="1")]
    pub rules: ::prost::alloc::vec::Vec<UpdateRuleRequest>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchUpdateRulesResponse {
    #[prost(bool, tag="1")]
    pub success: bool,
    #[prost(int32, tag="2")]
    pub rules_created_count: i32,
    #[prost(int32, tag="3")]
    pub rules_updated_count: i32,
}
/// DeleteRuleRequest is used to delete a rule by rule_id or client_key. If both are provided, only rule_id will be used.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteRuleRequest {
    #[prost(string, tag="1")]
    pub rule_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub client_key: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteRuleResponse {
}
/// Deprecated - use ViewJsonRulesRequest.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ViewHumanFriendlyRulesRequest {
    #[prost(string, tag="1")]
    pub asset_id: ::prost::alloc::string::String,
}
/// Deprecated - use ViewJsonRulesResponse.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ViewHumanFriendlyRulesResponse {
    #[prost(string, tag="1")]
    pub rules_json: ::prost::alloc::string::String,
}
/// Deprecated - use UpdateJsonRulesRequest.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateHumanFriendlyRulesRequest {
    #[prost(string, tag="1")]
    pub asset_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub rules_json: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub organization_id: ::prost::alloc::string::String,
}
/// Deprecated - use UpdateJsonRulesResponse.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateHumanFriendlyRulesResponse {
    #[prost(bool, tag="1")]
    pub success: bool,
    #[prost(int32, tag="2")]
    pub rules_count: i32,
    #[prost(string, tag="3")]
    pub messages: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ViewJsonRulesRequest {
    #[prost(string, tag="1")]
    pub asset_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ViewJsonRulesResponse {
    #[prost(string, tag="1")]
    pub rules_json: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct JsonRulesRequest {
    #[prost(string, tag="1")]
    pub asset_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub rules_json: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub organization_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct JsonRulesResponse {
    #[prost(bool, tag="1")]
    pub success: bool,
    #[prost(int32, tag="2")]
    pub total_rules_count: i32,
    #[prost(int32, tag="3")]
    pub rules_created_count: i32,
    #[prost(int32, tag="4")]
    pub rules_updated_count: i32,
    #[prost(int32, tag="5")]
    pub rules_deleted_count: i32,
    #[prost(string, optional, tag="6")]
    pub error_messages: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ValidateJsonRulesRequest {
    #[prost(message, optional, tag="1")]
    pub request: ::core::option::Option<JsonRulesRequest>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ValidateJsonRulesResponse {
    #[prost(message, optional, tag="1")]
    pub response: ::core::option::Option<JsonRulesResponse>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateJsonRulesRequest {
    #[prost(message, optional, tag="1")]
    pub request: ::core::option::Option<JsonRulesRequest>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateJsonRulesResponse {
    #[prost(message, optional, tag="1")]
    pub response: ::core::option::Option<JsonRulesResponse>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListRuleVersionsRequest {
    #[prost(string, tag="1")]
    pub rule_id: ::prost::alloc::string::String,
    /// The maximum number of Rule Versions to return.
    /// The service may return fewer than this value.
    /// If unspecified, at most 50 Rule Versions will be returned.
    /// The maximum value is 1000; values above 1000 will be coerced to 1000.
    #[prost(uint32, tag="2")]
    pub page_size: u32,
    /// A page token, received from a previous `ListRuleVersions` call.
    /// Provide this to retrieve the subsequent page.
    /// When paginating, all other parameters provided to `ListRuleVersions` must match
    /// the call that provided the page token.
    #[prost(string, tag="3")]
    pub page_token: ::prost::alloc::string::String,
    /// A [Common Expression Language (CEL)](<https://github.com/google/cel-spec>) filter string.
    /// Available fields to filter by are `rule_version_id`, `user_notes`,  and `change_message`.
    /// For further information about how to use CELs, please refer to [this guide](<https://github.com/google/cel-spec/blob/master/doc/langdef.md#standard-definitions>). Optional.
    #[prost(string, tag="4")]
    pub filter: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RuleVersion {
    #[prost(string, tag="1")]
    pub rule_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub rule_version_id: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub version: ::prost::alloc::string::String,
    #[prost(message, optional, tag="4")]
    pub created_date: ::core::option::Option<::pbjson_types::Timestamp>,
    #[prost(string, tag="5")]
    pub created_by_user_id: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub version_notes: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub generated_change_message: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListRuleVersionsResponse {
    #[prost(message, repeated, tag="1")]
    pub rule_versions: ::prost::alloc::vec::Vec<RuleVersion>,
    #[prost(string, tag="2")]
    pub next_page_token: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetRuleVersionRequest {
    #[prost(string, tag="1")]
    pub rule_version_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetRuleVersionResponse {
    #[prost(message, optional, tag="1")]
    pub rule: ::core::option::Option<Rule>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchGetRuleVersionsRequest {
    #[prost(string, repeated, tag="1")]
    pub rule_version_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchGetRuleVersionsResponse {
    #[prost(message, repeated, tag="1")]
    pub rules: ::prost::alloc::vec::Vec<Rule>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RuleConditionExpression {
    #[prost(oneof="rule_condition_expression::Expression", tags="1, 2")]
    pub expression: ::core::option::Option<rule_condition_expression::Expression>,
}
/// Nested message and enum types in `RuleConditionExpression`.
pub mod rule_condition_expression {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Expression {
        #[prost(message, tag="1")]
        SingleChannelComparison(super::SingleChannelComparisonExpression),
        #[prost(message, tag="2")]
        CalculatedChannel(super::CalculatedChannelConfig),
    }
}
/// Deprecated - use CalculatedChannelConfig.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SingleChannelComparisonExpression {
    #[prost(string, tag="1")]
    pub channel_component: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub channel_name: ::prost::alloc::string::String,
    #[prost(enumeration="ConditionComparator", tag="3")]
    pub comparator: i32,
    /// Threshold can be either a double or a string. Boolean values are encoded as 1 or 0. Enum values are stored as the string representation.
    #[prost(oneof="single_channel_comparison_expression::Threshold", tags="4, 5, 6")]
    pub threshold: ::core::option::Option<single_channel_comparison_expression::Threshold>,
}
/// Nested message and enum types in `SingleChannelComparisonExpression`.
pub mod single_channel_comparison_expression {
    /// Threshold can be either a double or a string. Boolean values are encoded as 1 or 0. Enum values are stored as the string representation.
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Threshold {
        #[prost(double, tag="4")]
        Double(f64),
        #[prost(string, tag="5")]
        String(::prost::alloc::string::String),
        #[prost(message, tag="6")]
        LastValue(super::LastValueThreshold),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LastValueThreshold {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CalculatedChannelConfig {
    #[prost(map="string, message", tag="1")]
    pub channel_references: ::std::collections::HashMap<::prost::alloc::string::String, ChannelReference>,
    #[prost(string, tag="2")]
    pub expression: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelReference {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub component: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RuleActionConfiguration {
    #[prost(oneof="rule_action_configuration::Configuration", tags="1, 2")]
    pub configuration: ::core::option::Option<rule_action_configuration::Configuration>,
}
/// Nested message and enum types in `RuleActionConfiguration`.
pub mod rule_action_configuration {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Configuration {
        #[prost(message, tag="1")]
        Notification(super::NotificationActionConfiguration),
        #[prost(message, tag="2")]
        Annotation(super::AnnotationActionConfiguration),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NotificationActionConfiguration {
    #[prost(string, repeated, tag="1")]
    pub recipient_user_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AnnotationActionConfiguration {
    #[prost(string, repeated, tag="1")]
    pub tag_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(enumeration="super::super::annotations::v1::AnnotationType", tag="2")]
    pub annotation_type: i32,
    #[prost(string, optional, tag="3")]
    pub assigned_to_user_id: ::core::option::Option<::prost::alloc::string::String>,
}
/// Deprecated - use RuleEvaluationService instead.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EvaluateRulesRequest {
    #[prost(string, repeated, tag="1")]
    pub rule_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(message, optional, tag="2")]
    pub annotation_options: ::core::option::Option<EvaluatedAnnotationOptions>,
    #[prost(bool, tag="5")]
    pub dry_run: bool,
    #[prost(oneof="evaluate_rules_request::Time", tags="3, 4")]
    pub time: ::core::option::Option<evaluate_rules_request::Time>,
}
/// Nested message and enum types in `EvaluateRulesRequest`.
pub mod evaluate_rules_request {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Time {
        #[prost(string, tag="3")]
        RunId(::prost::alloc::string::String),
        #[prost(message, tag="4")]
        TimeRange(super::TimeRangeQuery),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EvaluatedAnnotationOptions {
    #[prost(string, repeated, tag="1")]
    pub tag_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TimeRangeQuery {
    #[prost(message, optional, tag="1")]
    pub start_time: ::core::option::Option<::pbjson_types::Timestamp>,
    #[prost(message, optional, tag="2")]
    pub end_time: ::core::option::Option<::pbjson_types::Timestamp>,
}
/// Deprecated - use RuleEvaluationService instead.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EvaluateRulesResponse {
    #[prost(int32, tag="1")]
    pub created_annotation_count: i32,
    /// If dry_run is true, this will be populated with the annotations that would be created
    #[prost(message, repeated, tag="2")]
    pub dry_run_annotations: ::prost::alloc::vec::Vec<DryRunAnnotation>,
    /// job_id and report_id will be set if the job has an extended run time and is being processed asynchronously.
    #[prost(string, optional, tag="3")]
    pub job_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="4")]
    pub report_id: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DryRunAnnotation {
    #[prost(string, tag="1")]
    pub condition_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
    #[prost(message, optional, tag="3")]
    pub start_time: ::core::option::Option<::pbjson_types::Timestamp>,
    #[prost(message, optional, tag="4")]
    pub end_time: ::core::option::Option<::pbjson_types::Timestamp>,
    #[prost(string, tag="5")]
    pub condition_version_id: ::prost::alloc::string::String,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SearchOrder {
    Unspecified = 0,
    Asc = 1,
    Desc = 2,
}
impl SearchOrder {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            SearchOrder::Unspecified => "SEARCH_ORDER_UNSPECIFIED",
            SearchOrder::Asc => "SEARCH_ORDER_ASC",
            SearchOrder::Desc => "SEARCH_ORDER_DESC",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "SEARCH_ORDER_UNSPECIFIED" => Some(Self::Unspecified),
            "SEARCH_ORDER_ASC" => Some(Self::Asc),
            "SEARCH_ORDER_DESC" => Some(Self::Desc),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ActionKind {
    Unspecified = 0,
    Notification = 1,
    Annotation = 2,
}
impl ActionKind {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ActionKind::Unspecified => "ACTION_KIND_UNSPECIFIED",
            ActionKind::Notification => "NOTIFICATION",
            ActionKind::Annotation => "ANNOTATION",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "ACTION_KIND_UNSPECIFIED" => Some(Self::Unspecified),
            "NOTIFICATION" => Some(Self::Notification),
            "ANNOTATION" => Some(Self::Annotation),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ConditionComparator {
    Unspecified = 0,
    LessThan = 1,
    LessThanOrEqual = 2,
    GreaterThan = 3,
    GreaterThanOrEqual = 4,
    Equal = 5,
    NotEqual = 6,
}
impl ConditionComparator {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ConditionComparator::Unspecified => "CONDITION_COMPARATOR_UNSPECIFIED",
            ConditionComparator::LessThan => "LESS_THAN",
            ConditionComparator::LessThanOrEqual => "LESS_THAN_OR_EQUAL",
            ConditionComparator::GreaterThan => "GREATER_THAN",
            ConditionComparator::GreaterThanOrEqual => "GREATER_THAN_OR_EQUAL",
            ConditionComparator::Equal => "EQUAL",
            ConditionComparator::NotEqual => "NOT_EQUAL",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "CONDITION_COMPARATOR_UNSPECIFIED" => Some(Self::Unspecified),
            "LESS_THAN" => Some(Self::LessThan),
            "LESS_THAN_OR_EQUAL" => Some(Self::LessThanOrEqual),
            "GREATER_THAN" => Some(Self::GreaterThan),
            "GREATER_THAN_OR_EQUAL" => Some(Self::GreaterThanOrEqual),
            "EQUAL" => Some(Self::Equal),
            "NOT_EQUAL" => Some(Self::NotEqual),
            _ => None,
        }
    }
}
include!("sift.rules.v1.tonic.rs");
include!("sift.rules.v1.serde.rs");
// @@protoc_insertion_point(module)