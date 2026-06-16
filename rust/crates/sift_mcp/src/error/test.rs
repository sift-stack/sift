use anyhow::Context as _;
use rmcp::model::ErrorCode;
use tonic::Status;

use super::{err_into_mcp_result, into_tool_result};

fn extract_reason(value: &serde_json::Value) -> &str {
    value
        .get("reason")
        .and_then(|v| v.as_str())
        .expect("missing reason field")
}

#[test]
fn soft_signal_for_resource_exhausted() {
    let err = anyhow::Error::from(Status::resource_exhausted("slow down"))
        .context("failed to query assets");

    let result = err_into_mcp_result(err).expect("expected Ok with structured guidance");
    let content = result
        .structured_content
        .expect("expected structured content");
    assert_eq!(extract_reason(&content), "rate_limited");
}

#[test]
fn soft_signal_for_unavailable() {
    let err =
        anyhow::Error::from(Status::unavailable("backend gone")).context("failed to query runs");

    let result = err_into_mcp_result(err).expect("expected Ok with structured guidance");
    let content = result
        .structured_content
        .expect("expected structured content");
    assert_eq!(extract_reason(&content), "backend_unreachable");
}

#[test]
fn soft_signal_for_deadline_exceeded() {
    let err = anyhow::Error::from(Status::deadline_exceeded("too slow"))
        .context("failed to get data");

    let result = err_into_mcp_result(err).expect("expected Ok with structured guidance");
    let content = result
        .structured_content
        .expect("expected structured content");
    assert_eq!(extract_reason(&content), "query_too_expensive");
}

#[test]
fn soft_signal_for_internal() {
    let err =
        anyhow::Error::from(Status::internal("backend bug")).context("failed to query channels");

    let result = err_into_mcp_result(err).expect("expected Ok with structured guidance");
    let content = result
        .structured_content
        .expect("expected structured content");
    assert_eq!(extract_reason(&content), "backend_error");
}

#[test]
fn invalid_argument_still_errors() {
    let err = anyhow::Error::from(Status::invalid_argument("bad filter"))
        .context("failed to query assets");

    let data = err_into_mcp_result(err).expect_err("expected Err for non-soft-signal code");
    assert_eq!(data.code, ErrorCode::INVALID_PARAMS);
}

#[test]
fn not_found_still_errors() {
    let err = anyhow::Error::from(Status::not_found("missing")).context("lookup failed");

    let data = err_into_mcp_result(err).expect_err("expected Err for not-found");
    assert_eq!(data.code, ErrorCode::RESOURCE_NOT_FOUND);
}

#[test]
fn into_tool_result_passes_through_success() {
    let value = serde_json::json!({ "assets": [] });
    let result =
        into_tool_result(Ok(value.clone())).expect("expected Ok with structured success");
    assert_eq!(
        result
            .structured_content
            .expect("expected structured content"),
        value
    );
}

#[test]
fn soft_signal_emits_text_content() {
    let err = anyhow::Error::from(Status::resource_exhausted("slow down"))
        .context("failed to query assets");

    let result = err_into_mcp_result(err).expect("expected Ok");
    assert!(!result.content.is_empty(), "expected text content block");
}
