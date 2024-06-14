// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Rule {
    #[prost(string, tag="1")]
    pub rule_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub asset_id: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub description: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub current_status: ::prost::alloc::string::String,
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
    #[prost(string, tag="4")]
    pub status: ::prost::alloc::string::String,
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
    /// Conditions are not included in the search response
    #[prost(message, repeated, tag="2")]
    pub rules: ::prost::alloc::vec::Vec<Rule>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetRuleRequest {
    #[prost(string, tag="1")]
    pub rule_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetRuleResponse {
    #[prost(message, optional, tag="1")]
    pub rule: ::core::option::Option<Rule>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchGetRulesRequest {
    #[prost(string, repeated, tag="1")]
    pub rule_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
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
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateRuleRequest {
    #[prost(string, optional, tag="1")]
    pub rule_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub description: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub asset_id: ::prost::alloc::string::String,
    #[prost(bool, tag="5")]
    pub is_enabled: bool,
    #[prost(message, repeated, tag="6")]
    pub conditions: ::prost::alloc::vec::Vec<UpdateConditionRequest>,
    #[prost(string, tag="7")]
    pub organization_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateConditionRequest {
    #[prost(string, optional, tag="1")]
    pub rule_condition_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, tag="2")]
    pub status: ::prost::alloc::string::String,
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
pub struct DeleteRuleRequest {
    #[prost(string, tag="1")]
    pub rule_id: ::prost::alloc::string::String,
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
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EvaluateRulesRequest {
    #[prost(string, repeated, tag="1")]
    pub rule_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(message, optional, tag="2")]
    pub annotation_options: ::core::option::Option<EvaluatedAnnotationOptions>,
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
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EvaluateRulesResponse {
    #[prost(int32, tag="1")]
    pub created_annotation_count: i32,
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