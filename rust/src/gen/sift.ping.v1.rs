// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PingRequest {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PingResponse {
    #[prost(string, tag="1")]
    pub response: ::prost::alloc::string::String,
}
include!("sift.ping.v1.tonic.rs");
include!("sift.ping.v1.serde.rs");
// @@protoc_insertion_point(module)