use anyhow::Context as _;
use rmcp::model::ErrorCode;
use tonic::Status;

use super::from_anyhow;

fn reason_from_data(data: &rmcp::ErrorData) -> &str {
    data.data
        .as_ref()
        .and_then(|v| v.get("reason"))
        .and_then(|v| v.as_str())
        .expect("missing reason field in error data")
}

#[test]
fn soft_signal_for_resource_exhausted() {
    let err = anyhow::Error::from(Status::resource_exhausted("slow down"))
        .context("failed to query assets");

    let data = from_anyhow(err);
    assert_eq!(reason_from_data(&data), "rate_limited");
    assert!(data.message.contains("rate-limiting"));
}

#[test]
fn soft_signal_for_unavailable() {
    let err =
        anyhow::Error::from(Status::unavailable("backend gone")).context("failed to query runs");

    let data = from_anyhow(err);
    assert_eq!(reason_from_data(&data), "backend_unreachable");
}

#[test]
fn soft_signal_for_deadline_exceeded() {
    let err =
        anyhow::Error::from(Status::deadline_exceeded("too slow")).context("failed to get data");

    let data = from_anyhow(err);
    assert_eq!(reason_from_data(&data), "query_too_expensive");
}

#[test]
fn soft_signal_for_internal() {
    let err =
        anyhow::Error::from(Status::internal("backend bug")).context("failed to query channels");

    let data = from_anyhow(err);
    assert_eq!(reason_from_data(&data), "backend_error");
}

#[test]
fn invalid_argument_keeps_existing_mapping() {
    let err = anyhow::Error::from(Status::invalid_argument("bad filter"))
        .context("failed to query assets");

    let data = from_anyhow(err);
    assert_eq!(data.code, ErrorCode::INVALID_PARAMS);
    assert!(data.data.is_none());
}

#[test]
fn not_found_keeps_existing_mapping() {
    let err = anyhow::Error::from(Status::not_found("missing")).context("lookup failed");

    let data = from_anyhow(err);
    assert_eq!(data.code, ErrorCode::RESOURCE_NOT_FOUND);
}
