// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Run {
    #[prost(string, tag="1")]
    pub run_id: ::prost::alloc::string::String,
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
    #[prost(message, optional, tag="7")]
    pub start_time: ::core::option::Option<::pbjson_types::Timestamp>,
    #[prost(message, optional, tag="8")]
    pub stop_time: ::core::option::Option<::pbjson_types::Timestamp>,
    #[prost(bool, tag="9")]
    pub is_pinned: bool,
    #[prost(string, tag="10")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag="11")]
    pub description: ::prost::alloc::string::String,
    #[prost(string, repeated, tag="12")]
    pub tags: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// The request for a call to `RunService_GetRun` to retrieve run.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetRunRequest {
    /// The ID of the run to retrieve.
    #[prost(string, tag="1")]
    pub run_id: ::prost::alloc::string::String,
}
/// The response of a call to `RunService_GetRun` containing the requested run.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetRunResponse {
    #[prost(message, optional, tag="1")]
    pub run: ::core::option::Option<Run>,
}
/// The request for a call to `RunService_ListRuns` to retrieve runs.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListRunsRequest {
    /// The maximum number of runs to return.
    /// The service may return fewer than this value.
    /// If unspecified, at most 50 runs will be returned.
    /// The maximum value is 1000; values above 1000 will be coerced to 1000.
    #[prost(uint32, tag="1")]
    pub page_size: u32,
    /// A page token, received from a previous `ListRuns` call.
    /// Provide this to retrieve the subsequent page.
    /// When paginating, all other parameters provided to `ListRuns` must match
    /// the call that provided the page token.
    #[prost(string, tag="2")]
    pub page_token: ::prost::alloc::string::String,
    /// A [Common Expression Language (CEL)](<https://github.com/google/cel-spec>) filter string.
    /// Available fields to filter by are `run_id`, `organization_id`, `name`, `description`, `created_by_user_id`, `modified_by_user_id`,
    /// `created_date`, `modified_date`, `start_time`, `stop_time`, `client_key`, and `is_pinned`.
    /// For further information about how to use CELs, please refer to [this guide](<https://github.com/google/cel-spec/blob/master/doc/langdef.md#standard-definitions>).
    /// For more information about the fields used for filtering, please refer to [this definition](/protocol-buffers/documentation#run). Optional.
    #[prost(string, tag="3")]
    pub filter: ::prost::alloc::string::String,
}
/// The response of a call to `RunService_ListRuns` containing requested runs.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListRunsResponse {
    #[prost(message, repeated, tag="1")]
    pub runs: ::prost::alloc::vec::Vec<Run>,
    #[prost(string, tag="2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// The request of a call to `RunService_CreateRuns` to create a new run.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateRunRequest {
    /// The name that will be assigned to the new run.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// A description about the new run.
    #[prost(string, tag="2")]
    pub description: ::prost::alloc::string::String,
    /// Tags to associate with the new run.
    #[prost(string, repeated, tag="3")]
    pub tags: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// The time at which data ingestion begins for this new run. It must be before the `stop_time`, and it must
    /// be provided if a `stop_time` is provided.
    /// Important note: `start_time` will be automatically computed during data ingestion and will be set
    /// based on the timestamp of the data for this run.
    #[prost(message, optional, tag="4")]
    pub start_time: ::core::option::Option<::pbjson_types::Timestamp>,
    /// The time at which data ingestion for this new run concludes.
    /// Important note: `stop_time` will be automatically computed during data ingestion and will be
    /// set based on the timestamp of the data for this run.
    #[prost(message, optional, tag="5")]
    pub stop_time: ::core::option::Option<::pbjson_types::Timestamp>,
    /// An organization ID is only required if the user belongs to multiple organizations.
    #[prost(string, tag="7")]
    pub organization_id: ::prost::alloc::string::String,
}
/// The response of a call to `RunService_CreateRuns` containing the newly created run.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateRunResponse {
    #[prost(message, optional, tag="1")]
    pub run: ::core::option::Option<Run>,
}
/// The request for a call to `RunService_UpdateRun` to update an existing run.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateRunRequest {
    /// The run to update. The run's `run_id` field is used to identify the run to update
    /// and is required.
    #[prost(message, optional, tag="1")]
    pub run: ::core::option::Option<Run>,
    /// The list of fields to be updated. The fields available to be updated are `name`, `description`,
    /// `start_time`, `stop_time`, `is_pinned`,  and `tags`.
    /// Important Note: When updating the `start_time`, please be aware that if a subsequent data ingestion
    /// commences for this run, the `start_time` will be automatically overwritten and set to the timestamp
    /// corresponding to the beginning of the latest run.
    #[prost(message, optional, tag="2")]
    pub update_mask: ::core::option::Option<::pbjson_types::FieldMask>,
}
/// The response of a call to `RunService_UpdateRun` containing the updated run.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateRunResponse {
    #[prost(message, optional, tag="1")]
    pub run: ::core::option::Option<Run>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateAutomaticRunAssociationForAssetsRequest {
    /// The ID of the run to associate the asset with.
    #[prost(string, tag="1")]
    pub run_id: ::prost::alloc::string::String,
    /// A list of asset names to automatically associate with the run.
    /// Any data that is received for these assets will automatically added to the run.
    /// This applies even if the run has concluded, so long as the new data contains
    /// timestamps that are between the `start_time` and `stop_time`.
    /// If any of the assets are already associated with a different run whose run
    /// period (the period between `start_time` and `end_time`) overlaps with the
    /// requested run period, an error will be returned.
    #[prost(string, repeated, tag="2")]
    pub asset_names: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateAutomaticRunAssociationForAssetsResponse {
}
/// The request for a call to `RunService_DeleteRun`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteRunRequest {
    #[prost(string, tag="1")]
    pub run_id: ::prost::alloc::string::String,
}
/// The response of a call to `RunService_DeleteRun`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteRunResponse {
}
/// The request for a call to `RunService_StopRun` to stop a run.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StopRunRequest {
    #[prost(string, tag="1")]
    pub run_id: ::prost::alloc::string::String,
}
/// The response of a call to `RunService_StopRun` to stop a run.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StopRunResponse {
}
include!("sift.runs.v2.tonic.rs");
include!("sift.runs.v2.serde.rs");
// @@protoc_insertion_point(module)