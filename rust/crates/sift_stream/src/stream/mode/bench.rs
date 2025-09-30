use crate::stream::mode::ingestion_config::Flow;
use crate::{IngestionConfigMode, SiftStream};
use sift_rs::ingestion_configs::v2::FlowConfig;

/// Unstable wrapper around [SiftStream::message_to_ingest_req] used for benchmarking purposes.
#[inline]
pub fn message_to_ingest_req(
    message: &Flow,
    ingestion_config_id: &str,
    run_id: Option<String>,
    flows: &[FlowConfig],
) -> Option<sift_rs::ingest::v1::IngestWithConfigDataStreamRequest> {
    SiftStream::<IngestionConfigMode>::message_to_ingest_req(
        message,
        ingestion_config_id,
        run_id,
        flows,
    )
}

/// Unstable wrapper around [SiftStream::message_to_ingest_req_direct] used for benchmarking purposes.
#[inline]
pub fn message_to_ingest_req_direct(
    message: &Flow,
    ingestion_config_id: &str,
    run_id: Option<String>,
) -> sift_rs::ingest::v1::IngestWithConfigDataStreamRequest {
    SiftStream::<IngestionConfigMode>::message_to_ingest_req_direct(
        message,
        ingestion_config_id,
        run_id,
    )
}
