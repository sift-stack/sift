// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Annotation {
    #[prost(string, tag="1")]
    pub annotation_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub description: ::prost::alloc::string::String,
    #[prost(message, optional, tag="4")]
    pub start_time: ::core::option::Option<::pbjson_types::Timestamp>,
    #[prost(message, optional, tag="5")]
    pub end_time: ::core::option::Option<::pbjson_types::Timestamp>,
    #[prost(string, tag="6")]
    pub created_by_user_id: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub modified_by_user_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag="8")]
    pub created_date: ::core::option::Option<::pbjson_types::Timestamp>,
    #[prost(message, optional, tag="9")]
    pub modified_date: ::core::option::Option<::pbjson_types::Timestamp>,
    #[prost(string, optional, tag="10")]
    pub run_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(enumeration="AnnotationState", optional, tag="11")]
    pub state: ::core::option::Option<i32>,
    #[prost(string, tag="12")]
    pub organization_id: ::prost::alloc::string::String,
    #[prost(string, tag="13")]
    pub assigned_to_user_id: ::prost::alloc::string::String,
    #[prost(enumeration="AnnotationType", tag="14")]
    pub annotation_type: i32,
    #[prost(string, repeated, tag="15")]
    pub tags: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, optional, tag="16")]
    pub legend_config: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="17")]
    pub created_by_condition_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="18")]
    pub created_by_rule_condition_version_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="19")]
    pub report_rule_version_id: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AnnotationLinkedChannelsChannel {
    #[prost(string, tag="1")]
    pub channel_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AnnotationLinkedChannelsBitFieldElement {
    #[prost(string, tag="1")]
    pub channel_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub bit_field_name: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AnnotationLinkedChannel {
    #[prost(oneof="annotation_linked_channel::Type", tags="1, 2")]
    pub r#type: ::core::option::Option<annotation_linked_channel::Type>,
}
/// Nested message and enum types in `AnnotationLinkedChannel`.
pub mod annotation_linked_channel {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Type {
        #[prost(message, tag="1")]
        Channel(super::AnnotationLinkedChannelsChannel),
        #[prost(message, tag="2")]
        BitFieldElement(super::AnnotationLinkedChannelsBitFieldElement),
    }
}
/// The request for a call to `AnnotationService_CreateAnnotation` to create a new annotation.
/// At least 1 asset, tag, or channel must be specified.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateAnnotationRequest {
    /// The name assigned to the new annotation.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// A short description about the new annotation.
    #[prost(string, tag="2")]
    pub description: ::prost::alloc::string::String,
    /// When the annotation starts.
    #[prost(message, optional, tag="3")]
    pub start_time: ::core::option::Option<::pbjson_types::Timestamp>,
    /// When the annotation ends.
    #[prost(message, optional, tag="4")]
    pub end_time: ::core::option::Option<::pbjson_types::Timestamp>,
    /// The names of the assets to associate with this annotation.
    #[prost(string, repeated, tag="5")]
    pub assets: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// The channels to associate with this annotation.
    #[prost(message, repeated, tag="6")]
    pub linked_channels: ::prost::alloc::vec::Vec<AnnotationLinkedChannel>,
    /// The names of the tags to associate with this annotation.
    #[prost(string, repeated, tag="7")]
    pub tags: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// The ID of the run that this annotation is associated with.
    #[prost(string, optional, tag="8")]
    pub run_id: ::core::option::Option<::prost::alloc::string::String>,
    /// The ID of the user that this annotation is assigned to.
    #[prost(string, optional, tag="9")]
    pub assign_to_user_id: ::core::option::Option<::prost::alloc::string::String>,
    /// The organization associated with this annotation. An organization ID is only required
    /// if the user belongs to multiple organizations.
    #[prost(string, tag="10")]
    pub organization_id: ::prost::alloc::string::String,
    /// The state of the annotation. If an annotation has an `annotation_type` of `ANNOTATION_TYPE_PHASE`, then state must be
    /// unset, otherwise an error will be returned.
    #[prost(enumeration="AnnotationState", optional, tag="11")]
    pub state: ::core::option::Option<i32>,
    /// The type of the annotation.
    #[prost(enumeration="AnnotationType", tag="12")]
    pub annotation_type: i32,
    /// The ID of the rule condition that created this annotation.
    #[prost(string, optional, tag="14")]
    pub created_by_condition_id: ::core::option::Option<::prost::alloc::string::String>,
    /// A JSON string containing the axes configuration of the annotation's linked channels.
    #[prost(string, optional, tag="13")]
    pub legend_config: ::core::option::Option<::prost::alloc::string::String>,
    /// The ID of the rule condition version that created this annotation.
    #[prost(string, optional, tag="15")]
    pub created_by_rule_condition_version_id: ::core::option::Option<::prost::alloc::string::String>,
}
/// The result of a call to `AnnotationService_CreateAnnotation`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateAnnotationResponse {
    #[prost(message, optional, tag="1")]
    pub annotation: ::core::option::Option<Annotation>,
}
/// The request for a call to `AnnotationService_DeleteAnnotation`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteAnnotationRequest {
    #[prost(string, tag="1")]
    pub annotation_id: ::prost::alloc::string::String,
}
/// The response of a call to `AnnotationService_DeleteAnnotation`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteAnnotationResponse {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchDeleteAnnotationsRequest {
    /// Limit of 1000 annotations per batch
    #[prost(string, repeated, tag="1")]
    pub annotation_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchDeleteAnnotationsResponse {
}
/// The request for a call to `AnnotationService_GetAnnotation`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAnnotationRequest {
    #[prost(string, tag="1")]
    pub annotation_id: ::prost::alloc::string::String,
}
/// The response of a call to `AnnotationService_GetAnnotation`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAnnotationResponse {
    #[prost(message, optional, tag="1")]
    pub annotation: ::core::option::Option<Annotation>,
}
/// The request for a call to `AnnotationService_ListAnnotations` to retrieve annotations.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListAnnotationsRequest {
    /// The maximum number of annotations to return. The service may return fewer than this value.
    /// If unspecified, at most 50 annotations will be returned. The maximum value is 1000; values above
    /// 1000 will be coerced to 1000. Optional.
    #[prost(uint32, tag="1")]
    pub page_size: u32,
    /// A page token, received from a previous `ListAnnotations` call.
    /// Provide this to retrieve the subsequent page.
    /// When paginating, all other parameters provided to `ListAnnotations` must match
    /// the call that provided the page token. Optional.
    #[prost(string, tag="2")]
    pub page_token: ::prost::alloc::string::String,
    /// A [Common Expression Language (CEL)](<https://github.com/google/cel-spec>) filter string.
    /// Available fields to filter by are `annotation_id`, `start_time`, `end_time`,
    /// `created_date`, `modified_date`, `run_id`, `name`, `description`, `state`, `created_by_user_id`, `created_by_rule_condition_version_id`,
    /// `annotation_type`, `tag_name`, and `assignee`.
    /// For further information about how to use CELs, please refer to [this guide](<https://github.com/google/cel-spec/blob/master/doc/langdef.md#standard-definitions>).
    /// For more information about the fields used for filtering, please refer to [this definition](/protocol-buffers/documentation#annotation). Optional.
    #[prost(string, tag="3")]
    pub filter: ::prost::alloc::string::String,
    /// This field is only required if your user belongs to multiple organizations.
    #[prost(string, tag="4")]
    pub organization_id: ::prost::alloc::string::String,
    /// How to order the retrieved annotations. Formatted as a comma-separated string i.e. "<field_name>\[ desc\],...".
    /// Available fields to order_by are `created_date`, `modified_date`, `start_time`, and `end_time`.
    /// If left empty, items are ordered by `created_date` in ascending order (oldest-first).
    /// For more information about the format of this field, read [this](<https://google.aip.dev/132#ordering>)
    /// Example: "created_date desc,modified_date"
    #[prost(string, tag="5")]
    pub order_by: ::prost::alloc::string::String,
}
/// The result of a call to `AnnotationService_ListAnnotations`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListAnnotationsResponse {
    #[prost(message, repeated, tag="1")]
    pub annotations: ::prost::alloc::vec::Vec<Annotation>,
    /// Oops, we skipped to index 5! No reason for that; the indices between aren't reserved or anything.
    #[prost(string, tag="5")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// The request for a call to `AnnotationService_UpdateAnnotation` to update an annotation.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateAnnotationRequest {
    /// The annotation to update.
    #[prost(message, optional, tag="1")]
    pub annotation: ::core::option::Option<Annotation>,
    /// The list of fields to be updated. The fields available to be updated are `name`, `description`, `start_time`,
    /// `end_time`, `assigned_to_user_id`, `state`, and `tags`.
    /// Important Note: if `tags` is specified in the update mask and `annotation.tags` is an empty list then all associated tags on the annotation
    /// will be removed.
    #[prost(message, optional, tag="2")]
    pub update_mask: ::core::option::Option<::pbjson_types::FieldMask>,
}
/// The response of a call to `AnnotationService_UpdateAnnotation`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateAnnotationResponse {
    #[prost(message, optional, tag="1")]
    pub annotation: ::core::option::Option<Annotation>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum AnnotationState {
    Unspecified = 0,
    Open = 1,
    Flagged = 2,
    Resolved = 3,
}
impl AnnotationState {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            AnnotationState::Unspecified => "ANNOTATION_STATE_UNSPECIFIED",
            AnnotationState::Open => "ANNOTATION_STATE_OPEN",
            AnnotationState::Flagged => "ANNOTATION_STATE_FLAGGED",
            AnnotationState::Resolved => "ANNOTATION_STATE_RESOLVED",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "ANNOTATION_STATE_UNSPECIFIED" => Some(Self::Unspecified),
            "ANNOTATION_STATE_OPEN" => Some(Self::Open),
            "ANNOTATION_STATE_FLAGGED" => Some(Self::Flagged),
            "ANNOTATION_STATE_RESOLVED" => Some(Self::Resolved),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum AnnotationType {
    Unspecified = 0,
    DataReview = 1,
    Phase = 2,
}
impl AnnotationType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            AnnotationType::Unspecified => "ANNOTATION_TYPE_UNSPECIFIED",
            AnnotationType::DataReview => "ANNOTATION_TYPE_DATA_REVIEW",
            AnnotationType::Phase => "ANNOTATION_TYPE_PHASE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "ANNOTATION_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
            "ANNOTATION_TYPE_DATA_REVIEW" => Some(Self::DataReview),
            "ANNOTATION_TYPE_PHASE" => Some(Self::Phase),
            _ => None,
        }
    }
}
include!("sift.annotations.v1.tonic.rs");
include!("sift.annotations.v1.serde.rs");
// @@protoc_insertion_point(module)