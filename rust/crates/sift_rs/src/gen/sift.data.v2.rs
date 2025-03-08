// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDataRequest {
    #[prost(message, repeated, tag="1")]
    pub queries: ::prost::alloc::vec::Vec<Query>,
    /// Required. The starting timestamp of the data to retrieve. This is an inclusive bound.
    #[prost(message, optional, tag="2")]
    pub start_time: ::core::option::Option<::pbjson_types::Timestamp>,
    /// Required. The end timestamp of the data to retrieve. This is an exclusive bound.
    #[prost(message, optional, tag="3")]
    pub end_time: ::core::option::Option<::pbjson_types::Timestamp>,
    /// The rate to sample the returned data at. The data is sampled using [LTTB](<https://github.com/sveinn-steinarsson/flot-downsample>)
    /// which will return one point approximately every sample_ms milliseconds that retains the shape of the raw data.
    /// Sampling is only supported for numeric data types, if sample_ms is provided for non-numeric data, it will be
    /// ignored and the full-fidelity data will be returned.
    #[prost(uint32, tag="4")]
    pub sample_ms: u32,
    /// The maximum number of channel values to return.
    /// The service may return fewer than this value.
    /// If unspecified, at most 10,000 values will be returned.
    /// The maximum value is 100,000; values above 100,000 will be coerced to 100,000.
    /// For variable data types (i.e. string channels), at most page_size elements
    /// will be read, or 1MB, whichever occurs first.
    #[prost(uint32, tag="5")]
    pub page_size: u32,
    /// A page token, received from a previous `GetData` call.
    /// Provide this to retrieve the subsequent page.
    /// When paginating, all other parameters provided to `GetData` must match
    /// the call that provided the page token.
    #[prost(string, tag="6")]
    pub page_token: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Query {
    #[prost(oneof="query::Query", tags="1, 2")]
    pub query: ::core::option::Option<query::Query>,
}
/// Nested message and enum types in `Query`.
pub mod query {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Query {
        #[prost(message, tag="1")]
        Channel(super::ChannelQuery),
        #[prost(message, tag="2")]
        CalculatedChannel(super::CalculatedChannelQuery),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelQuery {
    /// channel_id is the uuid of the channel.
    #[prost(string, tag="1")]
    pub channel_id: ::prost::alloc::string::String,
    /// Optional.
    /// If set, only data associated with the specified run is returned.
    /// If set to the empty string, only non-run data is returned.
    /// If unset, all run / non-run data is returned.
    #[prost(string, optional, tag="2")]
    pub run_id: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CalculatedChannelQuery {
    #[prost(string, tag="1")]
    pub channel_key: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub expression: ::core::option::Option<super::super::calculated_channels::v1::ExpressionRequest>,
    /// Optional.
    /// If set, only data for the specified run is returned
    /// If set to the empty string, only non-run data is returned.
    /// If unset, all run / non-run data is returned.
    #[prost(string, optional, tag="3")]
    pub run_id: ::core::option::Option<::prost::alloc::string::String>,
    /// Optional. If unset, will default to EXPRESSION_MODE_CALCULATED_CHANNELS.
    #[prost(enumeration="super::super::calculated_channels::v1::ExpressionMode", optional, tag="4")]
    pub mode: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDataResponse {
    #[prost(string, tag="1")]
    pub next_page_token: ::prost::alloc::string::String,
    /// data contains the result of the supplied queries.
    /// Be aware that each query can generate multiple data responses.
    /// For example, if run_id is omitted from a ChannelQuery, the query returns
    /// data for all runs containing that channel. Channel data for each run is
    /// returned in a separate data object.
    /// Possible message types:
    ///    sift.data.v2.DoubleValues
    ///    sift.data.v2.FloatValues
    ///    sift.data.v2.StringValues
    ///    sift.data.v2.EnumValues
    ///    sift.data.v2.BitFieldValues
    ///    sift.data.v2.BoolValues
    ///    sift.data.v2.Int32Values
    ///    sift.data.v2.Int64Values
    ///    sift.data.v2.Uint32Values
    ///    sift.data.v2.Uint64Values
    #[prost(message, repeated, tag="2")]
    pub data: ::prost::alloc::vec::Vec<::pbjson_types::Any>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Metadata {
    #[prost(enumeration="super::super::common::r#type::v1::ChannelDataType", tag="1")]
    pub data_type: i32,
    #[prost(uint32, tag="2")]
    pub sampled_ms: u32,
    #[prost(message, optional, tag="3")]
    pub asset: ::core::option::Option<metadata::Asset>,
    #[prost(message, optional, tag="4")]
    pub run: ::core::option::Option<metadata::Run>,
    #[prost(message, optional, tag="5")]
    pub channel: ::core::option::Option<metadata::Channel>,
}
/// Nested message and enum types in `Metadata`.
pub mod metadata {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Asset {
        #[prost(string, tag="1")]
        pub asset_id: ::prost::alloc::string::String,
        #[prost(string, tag="2")]
        pub name: ::prost::alloc::string::String,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Run {
        /// The run_id that was sent with the data during ingestion (if any).
        /// Note that this may be different from the run_id that was requested in the query.
        #[prost(string, tag="1")]
        pub run_id: ::prost::alloc::string::String,
        #[prost(string, tag="2")]
        pub name: ::prost::alloc::string::String,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Channel {
        /// For channel queries, this will contain the requested backing channel id.
        /// For calculated channel queries, this will contain the requested channel key.
        #[prost(string, tag="1")]
        pub channel_id: ::prost::alloc::string::String,
        #[prost(string, tag="2")]
        pub name: ::prost::alloc::string::String,
        #[prost(message, optional, tag="3")]
        pub unit: ::core::option::Option<channel::Unit>,
        #[prost(message, repeated, tag="4")]
        pub enum_types: ::prost::alloc::vec::Vec<super::super::super::common::r#type::v1::ChannelEnumType>,
        #[prost(message, repeated, tag="5")]
        pub bit_field_elements: ::prost::alloc::vec::Vec<super::super::super::common::r#type::v1::ChannelBitFieldElement>,
    }
    /// Nested message and enum types in `Channel`.
    pub mod channel {
        #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Unit {
            #[prost(string, tag="1")]
            pub name: ::prost::alloc::string::String,
            #[prost(string, tag="2")]
            pub abbreviated_name: ::prost::alloc::string::String,
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DoubleValue {
    #[prost(message, optional, tag="1")]
    pub timestamp: ::core::option::Option<::pbjson_types::Timestamp>,
    #[prost(double, tag="2")]
    pub value: f64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DoubleValues {
    #[prost(message, optional, tag="1")]
    pub metadata: ::core::option::Option<Metadata>,
    #[prost(message, repeated, tag="2")]
    pub values: ::prost::alloc::vec::Vec<DoubleValue>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StringValue {
    #[prost(message, optional, tag="1")]
    pub timestamp: ::core::option::Option<::pbjson_types::Timestamp>,
    #[prost(string, tag="2")]
    pub value: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StringValues {
    #[prost(message, optional, tag="1")]
    pub metadata: ::core::option::Option<Metadata>,
    #[prost(message, repeated, tag="2")]
    pub values: ::prost::alloc::vec::Vec<StringValue>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EnumValue {
    #[prost(message, optional, tag="1")]
    pub timestamp: ::core::option::Option<::pbjson_types::Timestamp>,
    #[prost(uint32, tag="2")]
    pub value: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EnumValues {
    #[prost(message, optional, tag="1")]
    pub metadata: ::core::option::Option<Metadata>,
    #[prost(message, repeated, tag="2")]
    pub values: ::prost::alloc::vec::Vec<EnumValue>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BitFieldValue {
    #[prost(message, optional, tag="1")]
    pub timestamp: ::core::option::Option<::pbjson_types::Timestamp>,
    #[prost(uint32, tag="2")]
    pub value: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BitFieldElementValues {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="2")]
    pub values: ::prost::alloc::vec::Vec<BitFieldValue>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BitFieldValues {
    #[prost(message, optional, tag="1")]
    pub metadata: ::core::option::Option<Metadata>,
    #[prost(message, repeated, tag="2")]
    pub values: ::prost::alloc::vec::Vec<BitFieldElementValues>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BoolValue {
    #[prost(message, optional, tag="1")]
    pub timestamp: ::core::option::Option<::pbjson_types::Timestamp>,
    #[prost(bool, tag="2")]
    pub value: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BoolValues {
    #[prost(message, optional, tag="1")]
    pub metadata: ::core::option::Option<Metadata>,
    #[prost(message, repeated, tag="2")]
    pub values: ::prost::alloc::vec::Vec<BoolValue>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FloatValue {
    #[prost(message, optional, tag="1")]
    pub timestamp: ::core::option::Option<::pbjson_types::Timestamp>,
    #[prost(float, tag="2")]
    pub value: f32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FloatValues {
    #[prost(message, optional, tag="1")]
    pub metadata: ::core::option::Option<Metadata>,
    #[prost(message, repeated, tag="2")]
    pub values: ::prost::alloc::vec::Vec<FloatValue>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Int32Value {
    #[prost(message, optional, tag="1")]
    pub timestamp: ::core::option::Option<::pbjson_types::Timestamp>,
    #[prost(int32, tag="2")]
    pub value: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Int32Values {
    #[prost(message, optional, tag="1")]
    pub metadata: ::core::option::Option<Metadata>,
    #[prost(message, repeated, tag="2")]
    pub values: ::prost::alloc::vec::Vec<Int32Value>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Uint32Value {
    #[prost(message, optional, tag="1")]
    pub timestamp: ::core::option::Option<::pbjson_types::Timestamp>,
    #[prost(uint32, tag="2")]
    pub value: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Uint32Values {
    #[prost(message, optional, tag="1")]
    pub metadata: ::core::option::Option<Metadata>,
    #[prost(message, repeated, tag="2")]
    pub values: ::prost::alloc::vec::Vec<Uint32Value>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Int64Value {
    #[prost(message, optional, tag="1")]
    pub timestamp: ::core::option::Option<::pbjson_types::Timestamp>,
    #[prost(int64, tag="2")]
    pub value: i64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Int64Values {
    #[prost(message, optional, tag="1")]
    pub metadata: ::core::option::Option<Metadata>,
    #[prost(message, repeated, tag="2")]
    pub values: ::prost::alloc::vec::Vec<Int64Value>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Uint64Value {
    #[prost(message, optional, tag="1")]
    pub timestamp: ::core::option::Option<::pbjson_types::Timestamp>,
    #[prost(uint64, tag="2")]
    pub value: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Uint64Values {
    #[prost(message, optional, tag="1")]
    pub metadata: ::core::option::Option<Metadata>,
    #[prost(message, repeated, tag="2")]
    pub values: ::prost::alloc::vec::Vec<Uint64Value>,
}
include!("sift.data.v2.tonic.rs");
include!("sift.data.v2.serde.rs");
// @@protoc_insertion_point(module)