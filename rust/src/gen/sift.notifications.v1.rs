// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Notification {
    #[prost(string, tag="1")]
    pub notification_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub created_date: ::core::option::Option<::pbjson_types::Timestamp>,
    #[prost(message, optional, tag="3")]
    pub modified_date: ::core::option::Option<::pbjson_types::Timestamp>,
    #[prost(string, tag="4")]
    pub created_by_user_id: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub modified_by_user_id: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub organization_id: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub recipient_user_id: ::prost::alloc::string::String,
    #[prost(bool, tag="8")]
    pub is_read: bool,
    #[prost(string, tag="9")]
    pub full_link: ::prost::alloc::string::String,
    #[prost(enumeration="NotificationKind", tag="10")]
    pub notification_type: i32,
    #[prost(string, tag="11")]
    pub contents: ::prost::alloc::string::String,
    #[prost(string, tag="12")]
    pub entity_id: ::prost::alloc::string::String,
}
/// The request for a call to `NotificationService_ListNotifications` to retrieve notifications.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListNotificationsRequest {
    /// The maximum number of notifications to return.
    /// The service may return fewer than this value.
    /// If unspecified, at most 50 notifications will be returned.
    /// The maximum value is 1000; values above 1000 will be coerced to 1000.
    #[prost(uint32, tag="1")]
    pub page_size: u32,
    /// A page token, received from a previous `ListNotifications` call.
    /// Provide this to retrieve the subsequent page.
    /// When paginating, all other parameters provided to `ListNotifications` must match
    /// the call that provided the page token.
    #[prost(string, tag="2")]
    pub page_token: ::prost::alloc::string::String,
    /// A [Common Expression Language (CEL)](<https://github.com/google/cel-spec>) filter string.
    /// Available fields to filter by are `notification_id`, `created_by_user_id`, `recipient_user_id`,
    /// `created_date`, `notification_type`, and `is_read`.
    /// For further information about how to use CELs, please refer to [this guide](<https://github.com/google/cel-spec/blob/master/doc/langdef.md#standard-definitions>).
    /// For more information about the fields used for filtering, please refer to [this definition](/protocol-buffers/documentation#notification). Optional.
    #[prost(string, tag="3")]
    pub filter: ::prost::alloc::string::String,
}
/// The response of a call to `NotificationService_ListNotifications`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListNotificationsResponse {
    #[prost(message, repeated, tag="1")]
    pub notifications: ::prost::alloc::vec::Vec<Notification>,
    #[prost(string, tag="2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// The request for a call to `NotificationService_BatchUpdateNotifications` to update notifications.
/// A maximum of 1000 notifications can be modified in a batch.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchUpdateNotificationsRequest {
    #[prost(message, repeated, tag="1")]
    pub requests: ::prost::alloc::vec::Vec<UpdateNotificationRequest>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateNotificationRequest {
    /// The notification to update. The notification's `notification_id` field is used to identify the notification to update
    /// and must be provided.
    #[prost(message, optional, tag="1")]
    pub notification: ::core::option::Option<Notification>,
    /// The list of fields to be updated. Currently, the only field that can be updated is `is_read`.
    #[prost(message, optional, tag="2")]
    pub update_mask: ::core::option::Option<::pbjson_types::FieldMask>,
}
/// The response of a call to `NotificationService_BatchUpdateNotifications` containing the updated notifications.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchUpdateNotificationsResponse {
    /// The updated notifications.
    #[prost(message, repeated, tag="1")]
    pub notifications: ::prost::alloc::vec::Vec<Notification>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum NotificationKind {
    Unspecified = 0,
    Text = 1,
    AnnotationAssigned = 2,
    MentionedInAnnotationComment = 3,
    ConditionTriggered = 4,
    AnnotationStateChanged = 5,
}
impl NotificationKind {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            NotificationKind::Unspecified => "NOTIFICATION_KIND_UNSPECIFIED",
            NotificationKind::Text => "NOTIFICATION_KIND_TEXT",
            NotificationKind::AnnotationAssigned => "NOTIFICATION_KIND_ANNOTATION_ASSIGNED",
            NotificationKind::MentionedInAnnotationComment => "NOTIFICATION_KIND_MENTIONED_IN_ANNOTATION_COMMENT",
            NotificationKind::ConditionTriggered => "NOTIFICATION_KIND_CONDITION_TRIGGERED",
            NotificationKind::AnnotationStateChanged => "NOTIFICATION_KIND_ANNOTATION_STATE_CHANGED",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "NOTIFICATION_KIND_UNSPECIFIED" => Some(Self::Unspecified),
            "NOTIFICATION_KIND_TEXT" => Some(Self::Text),
            "NOTIFICATION_KIND_ANNOTATION_ASSIGNED" => Some(Self::AnnotationAssigned),
            "NOTIFICATION_KIND_MENTIONED_IN_ANNOTATION_COMMENT" => Some(Self::MentionedInAnnotationComment),
            "NOTIFICATION_KIND_CONDITION_TRIGGERED" => Some(Self::ConditionTriggered),
            "NOTIFICATION_KIND_ANNOTATION_STATE_CHANGED" => Some(Self::AnnotationStateChanged),
            _ => None,
        }
    }
}
include!("sift.notifications.v1.tonic.rs");
include!("sift.notifications.v1.serde.rs");
// @@protoc_insertion_point(module)