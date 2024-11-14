// @generated
/// The request for a call to `AnnotationLogService_CreateAnnotationLog` to create an annotation log.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateAnnotationLogRequest {
    /// The ID of the parent annotation associated to associate with the new annotation log.
    #[prost(string, tag="1")]
    pub annotation_id: ::prost::alloc::string::String,
    /// The kind of annotation log to create.
    #[prost(enumeration="AnnotationLogKind", tag="2")]
    pub kind: i32,
    /// The properties associated with the specific kind of annotation log.
    #[prost(oneof="create_annotation_log_request::Properties", tags="3, 4, 5")]
    pub properties: ::core::option::Option<create_annotation_log_request::Properties>,
}
/// Nested message and enum types in `CreateAnnotationLogRequest`.
pub mod create_annotation_log_request {
    /// The properties associated with the specific kind of annotation log.
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Properties {
        #[prost(message, tag="3")]
        Assigned(super::AnnotationLogAssignedProperties),
        #[prost(message, tag="4")]
        StateUpdate(super::AnnotationLogStateUpdateProperties),
        #[prost(message, tag="5")]
        Comment(super::AnnotationLogCommentProperties),
    }
}
/// The response of a call to `AnnotationLogService_CreateAnnotationLog`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateAnnotationLogResponse {
    #[prost(message, optional, tag="1")]
    pub annotation_log: ::core::option::Option<AnnotationLogSearchResult>,
}
/// The request for a call to `AnnotationLogService_ListAnnotationLogs` to retrieve annotation logs.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListAnnotationLogsRequest {
    /// The ID of the parent annotation associated with the annotation logs.
    #[prost(string, tag="1")]
    pub annotation_id: ::prost::alloc::string::String,
    /// The maximum number of annotation logs to return.
    /// The service may return fewer than this value.
    /// If unspecified, at most 50 annotation logs will be returned.
    /// The maximum value is 1000; values above 1000 will be coerced to 1000.
    #[prost(uint32, tag="2")]
    pub page_size: u32,
    /// A page token, received from a previous `ListAnnotationLogs` call.
    /// Provide this to retrieve the subsequent page.
    /// When paginating, all other parameters provided to `ListAnnotationLogs` must match
    /// the call that provided the page token.
    #[prost(string, tag="3")]
    pub page_token: ::prost::alloc::string::String,
    /// A [Common Expression Language (CEL)](<https://github.com/google/cel-spec>) filter string.
    /// Available fields to filter by are `annotation_log_id`, `annotation_id`, `created_by_user_id`,
    /// `created_date`, `modified_date`, and `kind`.
    /// For further information about how to use CELs, please refer to [this guide](<https://github.com/google/cel-spec/blob/master/doc/langdef.md#standard-definitions>).
    /// For more information about the fields used for filtering, please refer to [this definition](/api/grpc/protocol_buffers/annotation_logs#annotationlogsearchresult). Optional.
    #[prost(string, tag="4")]
    pub filter: ::prost::alloc::string::String,
}
/// The response of a call to `AnnotationLogService_ListAnnotationLogs`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListAnnotationLogsResponse {
    #[prost(message, repeated, tag="1")]
    pub annotation_logs: ::prost::alloc::vec::Vec<AnnotationLogSearchResult>,
    #[prost(string, tag="2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// The request for a call to `AnnotationLogService_DeleteAnnotationLog` to delete an annotation log.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteAnnotationLogRequest {
    /// The ID of the parent annotation associated with the annotation log to be deleted.
    #[prost(string, tag="1")]
    pub annotation_id: ::prost::alloc::string::String,
    /// ID of the annotation log to be deleted.
    #[prost(string, tag="2")]
    pub annotation_log_id: ::prost::alloc::string::String,
}
/// The Response of a call to `AnnotationLogService_DeleteAnnotationLog`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteAnnotationLogResponse {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AnnotationLogSearchResult {
    #[prost(string, tag="1")]
    pub annotation_log_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub created_date: ::core::option::Option<::pbjson_types::Timestamp>,
    #[prost(message, optional, tag="3")]
    pub modified_date: ::core::option::Option<::pbjson_types::Timestamp>,
    #[prost(string, tag="4")]
    pub annotation_id: ::prost::alloc::string::String,
    #[prost(enumeration="AnnotationLogKind", tag="5")]
    pub kind: i32,
    #[prost(string, tag="6")]
    pub created_by_user_id: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub created_by_user_name: ::prost::alloc::string::String,
    #[prost(oneof="annotation_log_search_result::Properties", tags="8, 9, 10")]
    pub properties: ::core::option::Option<annotation_log_search_result::Properties>,
}
/// Nested message and enum types in `AnnotationLogSearchResult`.
pub mod annotation_log_search_result {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Properties {
        #[prost(message, tag="8")]
        Assigned(super::AnnotationLogAssignedProperties),
        #[prost(message, tag="9")]
        StateUpdate(super::AnnotationLogStateUpdateProperties),
        #[prost(message, tag="10")]
        Comment(super::AnnotationLogCommentProperties),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AnnotationLogAssignedProperties {
    #[prost(string, tag="1")]
    pub assigned_to_user_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub assigned_to_user_email: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AnnotationLogStateUpdateProperties {
    #[prost(enumeration="AnnotationLogState", tag="1")]
    pub state: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AnnotationLogCommentProperties {
    #[prost(message, repeated, tag="1")]
    pub body: ::prost::alloc::vec::Vec<AnnotationCommentBodyElement>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AnnotationCommentBodyElement {
    #[prost(enumeration="AnnotationCommentBodyElementType", tag="1")]
    pub r#type: i32,
    #[prost(string, tag="2")]
    pub text: ::prost::alloc::string::String,
    #[prost(message, optional, tag="3")]
    pub user_mention: ::core::option::Option<AnnotationCommentUserMention>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AnnotationCommentUserMention {
    #[prost(string, tag="1")]
    pub user_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub user_email: ::prost::alloc::string::String,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum AnnotationLogKind {
    Unspecified = 0,
    Comment = 1,
    StateUpdate = 2,
    Assigned = 3,
}
impl AnnotationLogKind {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            AnnotationLogKind::Unspecified => "ANNOTATION_LOG_KIND_UNSPECIFIED",
            AnnotationLogKind::Comment => "ANNOTATION_LOG_KIND_COMMENT",
            AnnotationLogKind::StateUpdate => "ANNOTATION_LOG_KIND_STATE_UPDATE",
            AnnotationLogKind::Assigned => "ANNOTATION_LOG_KIND_ASSIGNED",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "ANNOTATION_LOG_KIND_UNSPECIFIED" => Some(Self::Unspecified),
            "ANNOTATION_LOG_KIND_COMMENT" => Some(Self::Comment),
            "ANNOTATION_LOG_KIND_STATE_UPDATE" => Some(Self::StateUpdate),
            "ANNOTATION_LOG_KIND_ASSIGNED" => Some(Self::Assigned),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum AnnotationLogState {
    Unspecified = 0,
    Created = 1,
    Open = 2,
    Flagged = 3,
    Resolved = 4,
}
impl AnnotationLogState {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            AnnotationLogState::Unspecified => "ANNOTATION_LOG_STATE_UNSPECIFIED",
            AnnotationLogState::Created => "ANNOTATION_LOG_STATE_CREATED",
            AnnotationLogState::Open => "ANNOTATION_LOG_STATE_OPEN",
            AnnotationLogState::Flagged => "ANNOTATION_LOG_STATE_FLAGGED",
            AnnotationLogState::Resolved => "ANNOTATION_LOG_STATE_RESOLVED",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "ANNOTATION_LOG_STATE_UNSPECIFIED" => Some(Self::Unspecified),
            "ANNOTATION_LOG_STATE_CREATED" => Some(Self::Created),
            "ANNOTATION_LOG_STATE_OPEN" => Some(Self::Open),
            "ANNOTATION_LOG_STATE_FLAGGED" => Some(Self::Flagged),
            "ANNOTATION_LOG_STATE_RESOLVED" => Some(Self::Resolved),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum AnnotationCommentBodyElementType {
    Unspecified = 0,
    Text = 1,
    UserMention = 2,
}
impl AnnotationCommentBodyElementType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            AnnotationCommentBodyElementType::Unspecified => "ANNOTATION_COMMENT_BODY_ELEMENT_TYPE_UNSPECIFIED",
            AnnotationCommentBodyElementType::Text => "ANNOTATION_COMMENT_BODY_ELEMENT_TYPE_TEXT",
            AnnotationCommentBodyElementType::UserMention => "ANNOTATION_COMMENT_BODY_ELEMENT_TYPE_USER_MENTION",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "ANNOTATION_COMMENT_BODY_ELEMENT_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
            "ANNOTATION_COMMENT_BODY_ELEMENT_TYPE_TEXT" => Some(Self::Text),
            "ANNOTATION_COMMENT_BODY_ELEMENT_TYPE_USER_MENTION" => Some(Self::UserMention),
            _ => None,
        }
    }
}
include!("sift.annotation_logs.v1.tonic.rs");
include!("sift.annotation_logs.v1.serde.rs");
// @@protoc_insertion_point(module)