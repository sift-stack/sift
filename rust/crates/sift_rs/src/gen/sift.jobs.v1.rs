// @generated
// This file is @generated by prost-build.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Job {
    #[prost(string, tag="1")]
    pub job_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub organization_id: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub created_by_user_id: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub modified_by_user_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag="6")]
    pub created_date: ::core::option::Option<::pbjson_types::Timestamp>,
    #[prost(message, optional, tag="7")]
    pub modified_date: ::core::option::Option<::pbjson_types::Timestamp>,
    #[prost(message, optional, tag="8")]
    pub started_date: ::core::option::Option<::pbjson_types::Timestamp>,
    #[prost(message, optional, tag="9")]
    pub completed_date: ::core::option::Option<::pbjson_types::Timestamp>,
    #[prost(enumeration="JobType", tag="10")]
    pub job_type: i32,
    #[prost(enumeration="JobStatus", tag="11")]
    pub job_status: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListJobsRequest {
    #[prost(uint32, tag="1")]
    pub page_size: u32,
    #[prost(string, tag="2")]
    pub page_token: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub filter: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub organization_id: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub order_by: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListJobsResponse {
    #[prost(message, repeated, tag="1")]
    pub jobs: ::prost::alloc::vec::Vec<Job>,
    #[prost(string, tag="2")]
    pub next_page_token: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CancelJobRequest {
    #[prost(string, tag="1")]
    pub job_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CancelJobResponse {
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum JobType {
    Unspecified = 0,
    RuleEvaluation = 1,
    DataImport = 2,
}
impl JobType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            JobType::Unspecified => "JOB_TYPE_UNSPECIFIED",
            JobType::RuleEvaluation => "JOB_TYPE_RULE_EVALUATION",
            JobType::DataImport => "JOB_TYPE_DATA_IMPORT",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "JOB_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
            "JOB_TYPE_RULE_EVALUATION" => Some(Self::RuleEvaluation),
            "JOB_TYPE_DATA_IMPORT" => Some(Self::DataImport),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum JobStatus {
    Unspecified = 0,
    Created = 1,
    Running = 2,
    Finished = 3,
    Failed = 4,
    Cancelled = 5,
    CancelRequested = 6,
}
impl JobStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            JobStatus::Unspecified => "JOB_STATUS_UNSPECIFIED",
            JobStatus::Created => "JOB_STATUS_CREATED",
            JobStatus::Running => "JOB_STATUS_RUNNING",
            JobStatus::Finished => "JOB_STATUS_FINISHED",
            JobStatus::Failed => "JOB_STATUS_FAILED",
            JobStatus::Cancelled => "JOB_STATUS_CANCELLED",
            JobStatus::CancelRequested => "JOB_STATUS_CANCEL_REQUESTED",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "JOB_STATUS_UNSPECIFIED" => Some(Self::Unspecified),
            "JOB_STATUS_CREATED" => Some(Self::Created),
            "JOB_STATUS_RUNNING" => Some(Self::Running),
            "JOB_STATUS_FINISHED" => Some(Self::Finished),
            "JOB_STATUS_FAILED" => Some(Self::Failed),
            "JOB_STATUS_CANCELLED" => Some(Self::Cancelled),
            "JOB_STATUS_CANCEL_REQUESTED" => Some(Self::CancelRequested),
            _ => None,
        }
    }
}
include!("sift.jobs.v1.tonic.rs");
include!("sift.jobs.v1.serde.rs");
// @@protoc_insertion_point(module)