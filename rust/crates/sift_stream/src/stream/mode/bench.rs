use crate::stream::flow::FlowDescriptor;
use crate::stream::helpers;
use crate::stream::mode::ingestion_config::Flow;
use sift_rs::runs::v2::Run;

/// Unstable wrapper around [helpers::message_to_ingest_req] used for benchmarking purposes.
#[inline]
pub fn message_to_ingest_req(
    message: &Flow,
    run: Option<&Run>,
    descriptor: &FlowDescriptor<String>,
) -> Option<sift_rs::ingest::v1::IngestWithConfigDataStreamRequest> {
    helpers::message_to_ingest_req(message, run, descriptor)
}

/// Unstable wrapper around [helpers::message_to_ingest_req_direct] used for benchmarking purposes.
#[inline]
pub fn message_to_ingest_req_direct(
    message: &Flow,
    ingestion_config_id: &str,
    run: Option<&Run>,
) -> sift_rs::ingest::v1::IngestWithConfigDataStreamRequest {
    helpers::message_to_ingest_req_direct(message, ingestion_config_id, run)
}
