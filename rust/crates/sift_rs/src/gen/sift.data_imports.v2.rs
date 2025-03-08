// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateDataImportFromUrlRequest {
    /// The url to import. HTTP and S3 urls are supported.
    /// If you need to import non-public S3 objects, please contact Sift to set that up.
    #[prost(string, tag="1")]
    pub url: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub csv_config: ::core::option::Option<CsvConfig>,
    #[prost(message, optional, tag="3")]
    pub ch10_config: ::core::option::Option<Ch10Config>,
    #[prost(message, optional, tag="4")]
    pub tdms_config: ::core::option::Option<TdmsConfig>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateDataImportFromUrlResponse {
    #[prost(string, tag="1")]
    pub data_import_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDataImportRequest {
    #[prost(string, tag="1")]
    pub data_import_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDataImportResponse {
    #[prost(message, optional, tag="1")]
    pub data_import: ::core::option::Option<DataImport>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateDataImportFromUploadRequest {
    #[prost(message, optional, tag="1")]
    pub csv_config: ::core::option::Option<CsvConfig>,
    #[prost(message, optional, tag="3")]
    pub ch10_config: ::core::option::Option<Ch10Config>,
    #[prost(message, optional, tag="4")]
    pub tdms_config: ::core::option::Option<TdmsConfig>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateDataImportFromUploadResponse {
    #[prost(string, tag="1")]
    pub upload_url: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub data_import_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CsvConfig {
    #[prost(string, tag="1")]
    pub asset_name: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub run_name: ::prost::alloc::string::String,
    /// The id of the run to add this data to. If set, `run_name` is ignored.
    #[prost(string, tag="3")]
    pub run_id: ::prost::alloc::string::String,
    /// The first row to start reading as data. Can be used to skip header rows.
    /// The first row in the file is 1.
    #[prost(uint32, tag="4")]
    pub first_data_row: u32,
    #[prost(message, optional, tag="5")]
    pub time_column: ::core::option::Option<CsvTimeColumn>,
    /// A map from column number (1-indexed) to the channel configuration for that column.
    #[prost(map="uint32, message", tag="6")]
    pub data_columns: ::std::collections::HashMap<u32, super::super::common::r#type::v1::ChannelConfig>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CsvTimeColumn {
    /// The column number (1-indexed) of the time column.
    #[prost(uint32, tag="1")]
    pub column_number: u32,
    #[prost(enumeration="TimeFormat", tag="2")]
    pub format: i32,
    #[prost(message, optional, tag="3")]
    pub relative_start_time: ::core::option::Option<::pbjson_types::Timestamp>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DetectConfigRequest {
    #[prost(bytes="vec", tag="1")]
    pub data: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DetectConfigResponse {
    #[prost(message, optional, tag="1")]
    pub csv_config: ::core::option::Option<CsvConfig>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Ch10Config {
    #[prost(string, tag="1")]
    pub asset_name: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub run_name: ::prost::alloc::string::String,
    #[prost(bool, tag="3")]
    pub scale_values: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TdmsConfig {
    #[prost(string, tag="1")]
    pub asset_name: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub run_name: ::prost::alloc::string::String,
    /// Override the wf_start_time metadata field for all channels.
    /// Useful if your waveform channels have wf_increment but no wf_start_time (Veristand is guilty of this).
    #[prost(message, optional, tag="3")]
    pub start_time_override: ::core::option::Option<::pbjson_types::Timestamp>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DataImport {
    #[prost(string, tag="1")]
    pub data_import_id: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub source_url: ::prost::alloc::string::String,
    #[prost(enumeration="DataImportStatus", tag="4")]
    pub status: i32,
    #[prost(string, tag="5")]
    pub error_message: ::prost::alloc::string::String,
    #[prost(message, optional, tag="7")]
    pub created_date: ::core::option::Option<::pbjson_types::Timestamp>,
    #[prost(message, optional, tag="8")]
    pub modified_date: ::core::option::Option<::pbjson_types::Timestamp>,
    #[prost(message, optional, tag="6")]
    pub csv_config: ::core::option::Option<CsvConfig>,
    #[prost(message, optional, tag="9")]
    pub ch10_config: ::core::option::Option<Ch10Config>,
    #[prost(message, optional, tag="10")]
    pub tdms_config: ::core::option::Option<TdmsConfig>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListDataImportsRequest {
    /// The maximum number of data imports to return. The service may return fewer than this value.
    /// If unspecified, at most 50 data imports will be returned. The maximum value is 1000; values above
    /// 1000 will be coerced to 1000. Optional.
    #[prost(uint32, tag="1")]
    pub page_size: u32,
    /// A page token, received from a previous `ListDataImports` call.
    /// Provide this to retrieve the subsequent page.
    /// When paginating, all other parameters provided to `ListDataImports` must match
    /// the call that provided the page token. Optional.
    #[prost(string, tag="2")]
    pub page_token: ::prost::alloc::string::String,
    /// A [Common Expression Language (CEL)](<https://github.com/google/cel-spec>) filter string.
    /// Available fields to filter by are `data_import_id`, `source_url`, `status`.
    /// For further information about how to use CELs, please refer to [this guide](<https://github.com/google/cel-spec/blob/master/doc/langdef.md#standard-definitions>).
    #[prost(string, tag="3")]
    pub filter: ::prost::alloc::string::String,
    /// How to order the retrieved data imports. Formatted as a comma-separated string i.e. "FIELD_NAME\[ desc\],...".
    /// Available fields to order_by are `created_date` and `modified_date`.
    /// If left empty, items are ordered by `created_date` in ascending order (oldest-first).
    /// For more information about the format of this field, read [this](<https://google.aip.dev/132#ordering>)
    /// Example: "created_date desc,modified_date"
    #[prost(string, tag="4")]
    pub order_by: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListDataImportsResponse {
    #[prost(message, repeated, tag="1")]
    pub data_imports: ::prost::alloc::vec::Vec<DataImport>,
    #[prost(string, tag="2")]
    pub next_page_token: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RetryDataImportRequest {
    /// data_import_id is the id of the data import to retry.
    /// You can only retry an import that is a "url" based import (created with CreateDataImportFromUrl) and is in a failed state.
    #[prost(string, tag="1")]
    pub data_import_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RetryDataImportResponse {
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum TimeFormat {
    Unspecified = 0,
    RelativeNanoseconds = 1,
    RelativeMicroseconds = 2,
    RelativeMilliseconds = 3,
    RelativeSeconds = 4,
    RelativeMinutes = 5,
    RelativeHours = 6,
    AbsoluteRfc3339 = 10,
    AbsoluteDatetime = 11,
    AbsoluteUnixSeconds = 12,
    AbsoluteUnixMilliseconds = 13,
    AbsoluteUnixMicroseconds = 14,
    AbsoluteUnixNanoseconds = 15,
}
impl TimeFormat {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            TimeFormat::Unspecified => "TIME_FORMAT_UNSPECIFIED",
            TimeFormat::RelativeNanoseconds => "TIME_FORMAT_RELATIVE_NANOSECONDS",
            TimeFormat::RelativeMicroseconds => "TIME_FORMAT_RELATIVE_MICROSECONDS",
            TimeFormat::RelativeMilliseconds => "TIME_FORMAT_RELATIVE_MILLISECONDS",
            TimeFormat::RelativeSeconds => "TIME_FORMAT_RELATIVE_SECONDS",
            TimeFormat::RelativeMinutes => "TIME_FORMAT_RELATIVE_MINUTES",
            TimeFormat::RelativeHours => "TIME_FORMAT_RELATIVE_HOURS",
            TimeFormat::AbsoluteRfc3339 => "TIME_FORMAT_ABSOLUTE_RFC3339",
            TimeFormat::AbsoluteDatetime => "TIME_FORMAT_ABSOLUTE_DATETIME",
            TimeFormat::AbsoluteUnixSeconds => "TIME_FORMAT_ABSOLUTE_UNIX_SECONDS",
            TimeFormat::AbsoluteUnixMilliseconds => "TIME_FORMAT_ABSOLUTE_UNIX_MILLISECONDS",
            TimeFormat::AbsoluteUnixMicroseconds => "TIME_FORMAT_ABSOLUTE_UNIX_MICROSECONDS",
            TimeFormat::AbsoluteUnixNanoseconds => "TIME_FORMAT_ABSOLUTE_UNIX_NANOSECONDS",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "TIME_FORMAT_UNSPECIFIED" => Some(Self::Unspecified),
            "TIME_FORMAT_RELATIVE_NANOSECONDS" => Some(Self::RelativeNanoseconds),
            "TIME_FORMAT_RELATIVE_MICROSECONDS" => Some(Self::RelativeMicroseconds),
            "TIME_FORMAT_RELATIVE_MILLISECONDS" => Some(Self::RelativeMilliseconds),
            "TIME_FORMAT_RELATIVE_SECONDS" => Some(Self::RelativeSeconds),
            "TIME_FORMAT_RELATIVE_MINUTES" => Some(Self::RelativeMinutes),
            "TIME_FORMAT_RELATIVE_HOURS" => Some(Self::RelativeHours),
            "TIME_FORMAT_ABSOLUTE_RFC3339" => Some(Self::AbsoluteRfc3339),
            "TIME_FORMAT_ABSOLUTE_DATETIME" => Some(Self::AbsoluteDatetime),
            "TIME_FORMAT_ABSOLUTE_UNIX_SECONDS" => Some(Self::AbsoluteUnixSeconds),
            "TIME_FORMAT_ABSOLUTE_UNIX_MILLISECONDS" => Some(Self::AbsoluteUnixMilliseconds),
            "TIME_FORMAT_ABSOLUTE_UNIX_MICROSECONDS" => Some(Self::AbsoluteUnixMicroseconds),
            "TIME_FORMAT_ABSOLUTE_UNIX_NANOSECONDS" => Some(Self::AbsoluteUnixNanoseconds),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum DataImportStatus {
    Unspecified = 0,
    Pending = 1,
    InProgress = 2,
    Succeeded = 3,
    Failed = 4,
}
impl DataImportStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            DataImportStatus::Unspecified => "DATA_IMPORT_STATUS_UNSPECIFIED",
            DataImportStatus::Pending => "DATA_IMPORT_STATUS_PENDING",
            DataImportStatus::InProgress => "DATA_IMPORT_STATUS_IN_PROGRESS",
            DataImportStatus::Succeeded => "DATA_IMPORT_STATUS_SUCCEEDED",
            DataImportStatus::Failed => "DATA_IMPORT_STATUS_FAILED",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "DATA_IMPORT_STATUS_UNSPECIFIED" => Some(Self::Unspecified),
            "DATA_IMPORT_STATUS_PENDING" => Some(Self::Pending),
            "DATA_IMPORT_STATUS_IN_PROGRESS" => Some(Self::InProgress),
            "DATA_IMPORT_STATUS_SUCCEEDED" => Some(Self::Succeeded),
            "DATA_IMPORT_STATUS_FAILED" => Some(Self::Failed),
            _ => None,
        }
    }
}
include!("sift.data_imports.v2.tonic.rs");
include!("sift.data_imports.v2.serde.rs");
// @@protoc_insertion_point(module)