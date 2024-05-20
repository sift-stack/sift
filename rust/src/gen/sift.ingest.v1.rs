// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IngestWithConfigDataStreamRequest {
    #[prost(string, tag="1")]
    pub ingestion_config_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub flow: ::prost::alloc::string::String,
    #[prost(message, optional, tag="3")]
    pub timestamp: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(message, repeated, tag="4")]
    pub channel_values: ::prost::alloc::vec::Vec<IngestWithConfigDataChannelValue>,
    /// The run_id MUST be included if this data is part of a run.
    #[prost(string, tag="5")]
    pub run_id: ::prost::alloc::string::String,
    /// By default, if this request contains any channel values that do not match
    /// the supplied ingestion config, the request is stored in an error queue and
    /// the stream continues to accept data. This ensures all data is saved, but
    /// only valid data is fully ingested. If this is set to `true`, any validation
    /// errors end the stream and return the error to the client.
    #[prost(bool, tag="6")]
    pub end_stream_on_validation_error: bool,
    #[prost(string, tag="7")]
    pub organization_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IngestWithConfigDataStreamResponse {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IngestWithConfigDataChannelValue {
    #[prost(oneof="ingest_with_config_data_channel_value::Type", tags="1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11")]
    pub r#type: ::core::option::Option<ingest_with_config_data_channel_value::Type>,
}
/// Nested message and enum types in `IngestWithConfigDataChannelValue`.
pub mod ingest_with_config_data_channel_value {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Type {
        #[prost(string, tag="1")]
        String(::prost::alloc::string::String),
        #[prost(double, tag="2")]
        Double(f64),
        #[prost(float, tag="3")]
        Float(f32),
        #[prost(bool, tag="4")]
        Bool(bool),
        #[prost(int32, tag="5")]
        Int32(i32),
        #[prost(uint32, tag="6")]
        Uint32(u32),
        #[prost(int64, tag="7")]
        Int64(i64),
        #[prost(uint64, tag="8")]
        Uint64(u64),
        #[prost(bytes, tag="9")]
        BitField(::prost::alloc::vec::Vec<u8>),
        #[prost(uint32, tag="10")]
        Enum(u32),
        /// If there's not a new data point for a channel at the given timestamp, pass empty to skip it
        #[prost(message, tag="11")]
        Empty(()),
    }
}
include!("sift.ingest.v1.tonic.rs");
// @@protoc_insertion_point(module)