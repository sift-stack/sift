// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoteFile {
    #[prost(string, tag="1")]
    pub remote_file_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub organization_id: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub entity_id: ::prost::alloc::string::String,
    #[prost(enumeration="EntityType", tag="4")]
    pub entity_type: i32,
    #[prost(string, tag="5")]
    pub file_name: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub file_mime_type: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub file_content_encoding: ::prost::alloc::string::String,
    #[prost(string, tag="8")]
    pub storage_key: ::prost::alloc::string::String,
    #[prost(uint64, tag="9")]
    pub file_size: u64,
    #[prost(string, optional, tag="10")]
    pub description: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, tag="13")]
    pub created_by_user_id: ::prost::alloc::string::String,
    #[prost(string, tag="14")]
    pub modified_by_user_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag="15")]
    pub created_date: ::core::option::Option<::pbjson_types::Timestamp>,
    #[prost(message, optional, tag="16")]
    pub modified_date: ::core::option::Option<::pbjson_types::Timestamp>,
    #[prost(oneof="remote_file::Metadata", tags="11, 12")]
    pub metadata: ::core::option::Option<remote_file::Metadata>,
}
/// Nested message and enum types in `RemoteFile`.
pub mod remote_file {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Metadata {
        #[prost(message, tag="11")]
        VideoMetadata(super::VideoMetadata),
        #[prost(message, tag="12")]
        ImageMetadata(super::ImageMetadata),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VideoMetadata {
    #[prost(uint32, tag="1")]
    pub height: u32,
    #[prost(uint32, tag="2")]
    pub width: u32,
    #[prost(float, tag="3")]
    pub duration_seconds: f32,
    #[prost(message, optional, tag="4")]
    pub timestamp: ::core::option::Option<::pbjson_types::Timestamp>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImageMetadata {
    #[prost(uint32, tag="1")]
    pub height: u32,
    #[prost(uint32, tag="2")]
    pub width: u32,
}
/// The request for a call to `RemoteFileService_GetRemoteFile` to retrieve a remote file;
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetRemoteFileRequest {
    #[prost(string, tag="1")]
    pub remote_file_id: ::prost::alloc::string::String,
}
/// The response of a call to `RemoteFileService_GetRemoteFile`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetRemoteFileResponse {
    #[prost(message, optional, tag="1")]
    pub remote_file: ::core::option::Option<RemoteFile>,
}
/// The request for a call to `RemoteFileService_ListRemoteFiles` to retrieve remote files.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListRemoteFilesRequest {
    /// The maximum number of remote files to return. The service may return fewer than this value.
    /// If unspecified, at most 50 remote files will be returned. The maximum value is 1000; values above
    /// 1000 will be coerced to 1000. Optional.
    #[prost(uint32, tag="1")]
    pub page_size: u32,
    /// A page token, received from a previous `ListRemoteFiles` call.
    /// Provide this to retrieve the subsequent page.
    /// When paginating, all other parameters provided to `ListRemoteFiles` must match
    /// the call that provided the page token. Optional.
    #[prost(string, tag="2")]
    pub page_token: ::prost::alloc::string::String,
    /// A [Common Expression Language (CEL)](<https://github.com/google/cel-spec>) filter string.
    /// Available fields to filter by are `remote_file_id`, `entity_id`, `entity_type`, and `file_name`.
    /// For further information about how to use CELs, please refer to [this guide](<https://github.com/google/cel-spec/blob/master/doc/langdef.md#standard-definitions>).
    /// For more information about the fields used for filtering, please refer to [this definition](/protocol-buffers/documentation#remote_files). Optional.
    #[prost(string, tag="3")]
    pub filter: ::prost::alloc::string::String,
    /// This field is only required if your user belongs to multiple organizations.
    #[prost(string, tag="4")]
    pub organization_id: ::prost::alloc::string::String,
}
/// The response of a call to `RemoteFileService_ListRemoteFilesResponse`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListRemoteFilesResponse {
    #[prost(message, repeated, tag="1")]
    pub remote_files: ::prost::alloc::vec::Vec<RemoteFile>,
    #[prost(string, tag="2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// The request for a call to `RemoteFileService_CreateRemoteFile` to create a remote file.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateRemoteFileRequest {
    #[prost(string, tag="1")]
    pub file_name: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub entity_id: ::prost::alloc::string::String,
    #[prost(enumeration="EntityType", tag="3")]
    pub entity_type: i32,
    #[prost(string, tag="4")]
    pub file_mime_type: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub file_content_encoding: ::prost::alloc::string::String,
    #[prost(uint64, tag="6")]
    pub file_size: u64,
    #[prost(string, optional, tag="7")]
    pub description: ::core::option::Option<::prost::alloc::string::String>,
    /// This field is only required if your user belongs to multiple organizations.
    #[prost(string, tag="8")]
    pub organization_id: ::prost::alloc::string::String,
    /// A custom UUID used to generate the object key. Recommended to be left unset.
    #[prost(string, optional, tag="11")]
    pub custom_uuid: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(oneof="create_remote_file_request::Metadata", tags="9, 10")]
    pub metadata: ::core::option::Option<create_remote_file_request::Metadata>,
}
/// Nested message and enum types in `CreateRemoteFileRequest`.
pub mod create_remote_file_request {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Metadata {
        #[prost(message, tag="9")]
        VideoMetadata(super::VideoMetadata),
        #[prost(message, tag="10")]
        ImageMetadata(super::ImageMetadata),
    }
}
/// The response for a call to `RemoteFileService_CreateRemoteFile`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateRemoteFileResponse {
    #[prost(message, optional, tag="1")]
    pub remote_file: ::core::option::Option<RemoteFile>,
}
/// The request for a call to `RemoteFileService_DeleteRemoteFile` to delete a remote file.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteRemoteFileRequest {
    #[prost(string, tag="1")]
    pub remote_file_id: ::prost::alloc::string::String,
}
/// The response of a call to `RemoteFileService_DeleteRemoteFile`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteRemoteFileResponse {
}
/// The request for a call to `RemoteFileService_BatchDeleteRemoteFiles` to delete remote files.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchDeleteRemoteFilesRequest {
    /// Up to 1000 remote file IDs can be provided per request.
    #[prost(string, repeated, tag="1")]
    pub remote_file_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// The response of a call to `RemoteFileService_BatchDeleteRemoteFiles`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchDeleteRemoteFilesResponse {
}
/// The request for a call to `RemoteFileService_UpdateRemoteFile` to update a remote file.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateRemoteFileRequest {
    /// The remote file to update.
    #[prost(message, optional, tag="1")]
    pub remote_file: ::core::option::Option<RemoteFile>,
    /// The list of fields to be updated. The fields available to be updated are `description` and `metadata`.
    #[prost(message, optional, tag="2")]
    pub update_mask: ::core::option::Option<::pbjson_types::FieldMask>,
}
/// The response of a call to `RemoteFileService_UpdateRemoteFile`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateRemoteFileResponse {
    #[prost(message, optional, tag="1")]
    pub remote_file: ::core::option::Option<RemoteFile>,
}
/// The request for a call to `RemoteFileService_GetRemoteFileDownloadUrl`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetRemoteFileDownloadUrlRequest {
    #[prost(string, tag="1")]
    pub remote_file_id: ::prost::alloc::string::String,
}
/// The response of a call to `RemoteFileService_GetRemoteFileDownloadUrl`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetRemoteFileDownloadUrlResponse {
    #[prost(string, tag="1")]
    pub download_url: ::prost::alloc::string::String,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum EntityType {
    Unspecified = 0,
    Run = 1,
    Annotation = 2,
    Asset = 3,
    AnnotationLog = 4,
}
impl EntityType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            EntityType::Unspecified => "ENTITY_TYPE_UNSPECIFIED",
            EntityType::Run => "ENTITY_TYPE_RUN",
            EntityType::Annotation => "ENTITY_TYPE_ANNOTATION",
            EntityType::Asset => "ENTITY_TYPE_ASSET",
            EntityType::AnnotationLog => "ENTITY_TYPE_ANNOTATION_LOG",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "ENTITY_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
            "ENTITY_TYPE_RUN" => Some(Self::Run),
            "ENTITY_TYPE_ANNOTATION" => Some(Self::Annotation),
            "ENTITY_TYPE_ASSET" => Some(Self::Asset),
            "ENTITY_TYPE_ANNOTATION_LOG" => Some(Self::AnnotationLog),
            _ => None,
        }
    }
}
include!("sift.remote_files.v1.tonic.rs");
include!("sift.remote_files.v1.serde.rs");
// @@protoc_insertion_point(module)