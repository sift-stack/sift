// @generated
/// Message representing a tag target
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TagTarget {
    /// Setting this value indicates the allowed sources for tag values.
    /// If set to SIBLING_SOURCES, tag_sources from descendant proto fields will not be applied.
    /// If set to DESCENDANT_SOURCES, sibling tag_sources will not be applied.
    #[prost(enumeration="TagSourceType", optional, tag="1")]
    pub allowed_tag_source: ::core::option::Option<i32>,
}
/// Message representing a tag source
///
/// Setting either of these values indicates that the field is a tag source
/// and the value can be applied to allowed tag_targets.
/// Tag sources apply only to the nearest ancestor (if any) and do not propagate
/// to ancestors that are lists or maps or beyond.
/// They can apply to both ancestor and sibling fields.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TagSource {
    /// Indicates which tag_target relationships are allowed to have this tag.
    /// If set to ANCESTOR_TARGETS, sibling tag_targets will not have this value.
    /// If set to SIBLING_TARGETS, it will not apply to tag_targets that are ancestors.
    #[prost(enumeration="TagTargetType", optional, tag="1")]
    pub allowed_tag_target: ::core::option::Option<i32>,
    /// The name of the tag. It defaults to the field name but can be overridden here.
    #[prost(string, optional, tag="2")]
    pub tag_name: ::core::option::Option<::prost::alloc::string::String>,
}
/// Enum for different types of tag sources
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum TagSourceType {
    /// Tags can be sourced from descendant fields
    DescendantSources = 0,
    /// Tags can be sourced from sibling fields
    SiblingSources = 1,
    /// Tags can be sourced from both descendant and sibling fields
    DescendantAndSiblingSources = 2,
}
impl TagSourceType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            TagSourceType::DescendantSources => "DESCENDANT_SOURCES",
            TagSourceType::SiblingSources => "SIBLING_SOURCES",
            TagSourceType::DescendantAndSiblingSources => "DESCENDANT_AND_SIBLING_SOURCES",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "DESCENDANT_SOURCES" => Some(Self::DescendantSources),
            "SIBLING_SOURCES" => Some(Self::SiblingSources),
            "DESCENDANT_AND_SIBLING_SOURCES" => Some(Self::DescendantAndSiblingSources),
            _ => None,
        }
    }
}
/// Enum for different types of tag targets
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum TagTargetType {
    /// Tags can be applied to ancestor fields
    AncestorTargets = 0,
    /// Tags can be applied to sibling fields
    SiblingTargets = 1,
    /// Tags can be applied to both ancestor and sibling fields
    AncestorAndSiblingTargets = 2,
}
impl TagTargetType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            TagTargetType::AncestorTargets => "ANCESTOR_TARGETS",
            TagTargetType::SiblingTargets => "SIBLING_TARGETS",
            TagTargetType::AncestorAndSiblingTargets => "ANCESTOR_AND_SIBLING_TARGETS",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "ANCESTOR_TARGETS" => Some(Self::AncestorTargets),
            "SIBLING_TARGETS" => Some(Self::SiblingTargets),
            "ANCESTOR_AND_SIBLING_TARGETS" => Some(Self::AncestorAndSiblingTargets),
            _ => None,
        }
    }
}
/// Enum for the source and destination of a map key override
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum MapKeyOverrideType {
    /// Default value, will be ignored
    MapKeyOverrideUnspecified = 0,
    /// Map key will be replaced on fields with this type specified
    MapKeyOverrideTarget = 1,
    /// Map key will be sourced from fields with this type specified
    MapKeyOverrideSource = 2,
    /// Map key will be removed from fields with this type specified
    MapKeyOverrideRemoveKey = 3,
    /// Map key will be replaced with the enum name specified by the display_override_enum value on the field
    MapKeyOverrideEnum = 4,
}
impl MapKeyOverrideType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            MapKeyOverrideType::MapKeyOverrideUnspecified => "MAP_KEY_OVERRIDE_UNSPECIFIED",
            MapKeyOverrideType::MapKeyOverrideTarget => "MAP_KEY_OVERRIDE_TARGET",
            MapKeyOverrideType::MapKeyOverrideSource => "MAP_KEY_OVERRIDE_SOURCE",
            MapKeyOverrideType::MapKeyOverrideRemoveKey => "MAP_KEY_OVERRIDE_REMOVE_KEY",
            MapKeyOverrideType::MapKeyOverrideEnum => "MAP_KEY_OVERRIDE_ENUM",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "MAP_KEY_OVERRIDE_UNSPECIFIED" => Some(Self::MapKeyOverrideUnspecified),
            "MAP_KEY_OVERRIDE_TARGET" => Some(Self::MapKeyOverrideTarget),
            "MAP_KEY_OVERRIDE_SOURCE" => Some(Self::MapKeyOverrideSource),
            "MAP_KEY_OVERRIDE_REMOVE_KEY" => Some(Self::MapKeyOverrideRemoveKey),
            "MAP_KEY_OVERRIDE_ENUM" => Some(Self::MapKeyOverrideEnum),
            _ => None,
        }
    }
}
/// Enum for the source and destination of an array index override
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ArrayIndexOverrideType {
    /// Default value, will be ignored
    ArrayIndexOverrideUnspecified = 0,
    /// Array Index will be replaced on fields with this type specified
    ArrayIndexOverrideTarget = 1,
    /// Array Index  will be sourced from fields with this type specified
    ArrayIndexOverrideSource = 2,
    /// Array Index will be removed from fields with this type specified
    ArrayIndexOverrideRemoveIndex = 3,
    /// Array Index will be replaced with the enum name specified by the display_override_enum value on the field
    ArrayIndexOverrideEnum = 4,
}
impl ArrayIndexOverrideType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ArrayIndexOverrideType::ArrayIndexOverrideUnspecified => "ARRAY_INDEX_OVERRIDE_UNSPECIFIED",
            ArrayIndexOverrideType::ArrayIndexOverrideTarget => "ARRAY_INDEX_OVERRIDE_TARGET",
            ArrayIndexOverrideType::ArrayIndexOverrideSource => "ARRAY_INDEX_OVERRIDE_SOURCE",
            ArrayIndexOverrideType::ArrayIndexOverrideRemoveIndex => "ARRAY_INDEX_OVERRIDE_REMOVE_INDEX",
            ArrayIndexOverrideType::ArrayIndexOverrideEnum => "ARRAY_INDEX_OVERRIDE_ENUM",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "ARRAY_INDEX_OVERRIDE_UNSPECIFIED" => Some(Self::ArrayIndexOverrideUnspecified),
            "ARRAY_INDEX_OVERRIDE_TARGET" => Some(Self::ArrayIndexOverrideTarget),
            "ARRAY_INDEX_OVERRIDE_SOURCE" => Some(Self::ArrayIndexOverrideSource),
            "ARRAY_INDEX_OVERRIDE_REMOVE_INDEX" => Some(Self::ArrayIndexOverrideRemoveIndex),
            "ARRAY_INDEX_OVERRIDE_ENUM" => Some(Self::ArrayIndexOverrideEnum),
            _ => None,
        }
    }
}
/// Enum for how to decode bytes fields
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum BytesDecodingType {
    /// Default value, will be ignored
    Unspecified = 0,
    /// Decode bytes as UTF-8
    Utf8 = 1,
}
impl BytesDecodingType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            BytesDecodingType::Unspecified => "BYTES_DECODING_TYPE_UNSPECIFIED",
            BytesDecodingType::Utf8 => "BYTES_DECODING_TYPE_UTF8",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "BYTES_DECODING_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
            "BYTES_DECODING_TYPE_UTF8" => Some(Self::Utf8),
            _ => None,
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteProtobufDescriptorsRequest {
    #[deprecated]
    #[prost(string, tag="1")]
    pub message_type_full_name: ::prost::alloc::string::String,
    #[deprecated]
    #[prost(string, tag="2")]
    pub namespace: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub organization_id: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub protobuf_descriptor_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteProtobufDescriptorsResponse {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddProtobufDescriptorRequest {
    #[prost(message, optional, tag="1")]
    pub protobuf_descriptor: ::core::option::Option<ProtobufDescriptor>,
    /// If set to true, the service will ignore checks that this descriptor has already been registered
    /// and will ensure that it is registered as the latest proto descriptor for the message type.
    #[prost(bool, tag="2")]
    pub force_duplicate_registration: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddProtobufDescriptorResponse {
    #[prost(message, optional, tag="1")]
    pub protobuf_descriptor: ::core::option::Option<ProtobufDescriptor>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProtobufDescriptor {
    #[prost(string, tag="1")]
    pub message_type_full_name: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="2")]
    pub file_descriptor_set: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="3")]
    pub proto_file_name: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub namespace: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub protobuf_descriptor_id: ::prost::alloc::string::String,
}
/// The request for a call to `ProtobufDescriptorService_ListProtobufDescriptors` to retrieve protobuf descriptors.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListProtobufDescriptorsRequest {
    /// The maximum number of protobuf descriptors to return.
    /// The service may return fewer than this value.
    /// If unspecified, at most 50 protobuf descriptors will be returned.
    /// The maximum value is 1000; values above 1000 will be coerced to 1000.
    #[prost(uint32, tag="1")]
    pub page_size: u32,
    /// A page token, received from a previous `ListProtobufDescriptors` call.
    /// Provide this to retrieve the subsequent page.
    /// When paginating, all other parameters provided to `ListProtobufDescriptors` must match
    /// the call that provided the page token.
    #[prost(string, tag="2")]
    pub page_token: ::prost::alloc::string::String,
    /// A [Common Expression Language (CEL)](<https://github.com/google/cel-spec>) filter string.
    /// Available fields to filter by are `protobuf_descriptor_id`, `proto_file_name`, `namespace`, and `message_type_full_name`.
    /// For further information about how to use CELs, please refer to [this guide](<https://github.com/google/cel-spec/blob/master/doc/langdef.md#standard-definitions>).
    /// For more information about the fields used for filtering, please refer to [this definition](/protocol-buffers/documentation#protobufdescriptor). Optional.
    #[prost(string, tag="3")]
    pub filter: ::prost::alloc::string::String,
    /// How to order the retrieved protobuf descriptors. Formatted as a comma-separated string i.e. "FIELD_NAME\[ desc\],...".
    /// Available fields to order_by are `created_date`, `modified_date`, and `message_type_full_name` and `namespace`.
    /// If left empty, items are ordered by `created_date` in ascending order (oldest-first).
    /// For more information about the format of this field, read [this](<https://google.aip.dev/132#ordering>)
    /// Example: "created_date desc,modified_date"
    #[prost(string, tag="4")]
    pub order_by: ::prost::alloc::string::String,
}
/// The response of a call to `ProtobufDescriptorService_ListProtobufDescriptors`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListProtobufDescriptorsResponse {
    #[prost(message, repeated, tag="1")]
    pub protobuf_descriptors: ::prost::alloc::vec::Vec<ProtobufDescriptor>,
    #[prost(string, tag="2")]
    pub next_page_token: ::prost::alloc::string::String,
}
include!("sift.protobuf_descriptors.v2.tonic.rs");
include!("sift.protobuf_descriptors.v2.serde.rs");
// @@protoc_insertion_point(module)