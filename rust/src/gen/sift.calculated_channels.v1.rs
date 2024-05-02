// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExpressionRequest {
    /// A map from the channel reference in the expression string (e.g. $1) to the channel id (uuid).
    #[prost(map="string, string", tag="1")]
    pub channel_references: ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    #[prost(string, tag="2")]
    pub expression: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListExpressionIdentifiersRequest {
    /// Defaults to 1000. Max of 10,000.
    #[prost(uint32, tag="1")]
    pub page_size: u32,
    #[prost(string, tag="2")]
    pub page_token: ::prost::alloc::string::String,
    #[prost(enumeration="ExpressionMode", tag="3")]
    pub mode: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListExpressionIdentifiersResponse {
    #[prost(message, repeated, tag="1")]
    pub identifiers: ::prost::alloc::vec::Vec<ExpressionIdentifier>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExpressionIdentifier {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub description: ::prost::alloc::string::String,
    #[prost(enumeration="ExpressionIdentifierType", tag="3")]
    pub r#type: i32,
    #[prost(string, tag="4")]
    pub display_name: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ValidateExpressionRequest {
    #[prost(message, optional, tag="1")]
    pub expression: ::core::option::Option<ExpressionRequest>,
    #[prost(enumeration="ExpressionMode", tag="2")]
    pub mode: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ValidateExpressionResponse {
    #[prost(oneof="validate_expression_response::Result", tags="1, 2")]
    pub result: ::core::option::Option<validate_expression_response::Result>,
}
/// Nested message and enum types in `ValidateExpressionResponse`.
pub mod validate_expression_response {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Result {
        #[prost(message, tag="1")]
        Error(super::ErrorValidatingExpressionResult),
        #[prost(message, tag="2")]
        Success(super::SuccessValidatingExpressionResult),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ErrorValidatingExpressionResult {
    #[prost(string, tag="1")]
    pub error_message: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SuccessValidatingExpressionResult {
    #[prost(enumeration="super::super::common::r#type::v1::ChannelDataType", tag="1")]
    pub data_type: i32,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ExpressionIdentifierType {
    Unspecified = 0,
    Function = 1,
    Channel = 2,
}
impl ExpressionIdentifierType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ExpressionIdentifierType::Unspecified => "EXPRESSION_IDENTIFIER_TYPE_UNSPECIFIED",
            ExpressionIdentifierType::Function => "EXPRESSION_IDENTIFIER_TYPE_FUNCTION",
            ExpressionIdentifierType::Channel => "EXPRESSION_IDENTIFIER_TYPE_CHANNEL",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "EXPRESSION_IDENTIFIER_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
            "EXPRESSION_IDENTIFIER_TYPE_FUNCTION" => Some(Self::Function),
            "EXPRESSION_IDENTIFIER_TYPE_CHANNEL" => Some(Self::Channel),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ExpressionMode {
    Unspecified = 0,
    Rules = 1,
    CalculatedChannels = 2,
}
impl ExpressionMode {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ExpressionMode::Unspecified => "EXPRESSION_MODE_UNSPECIFIED",
            ExpressionMode::Rules => "EXPRESSION_MODE_RULES",
            ExpressionMode::CalculatedChannels => "EXPRESSION_MODE_CALCULATED_CHANNELS",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "EXPRESSION_MODE_UNSPECIFIED" => Some(Self::Unspecified),
            "EXPRESSION_MODE_RULES" => Some(Self::Rules),
            "EXPRESSION_MODE_CALCULATED_CHANNELS" => Some(Self::CalculatedChannels),
            _ => None,
        }
    }
}
include!("sift.calculated_channels.v1.tonic.rs");
// @@protoc_insertion_point(module)