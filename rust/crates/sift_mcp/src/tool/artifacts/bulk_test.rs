use super::*;
use crate::{error, tool::common::bulk::BulkItems};
use serde_json::{Value, json};

async fn call(
    server: &SiftMcpServer,
    op: &str,
    items: Value,
    selected: Option<Vec<usize>>,
) -> error::McpResult {
    let args = json!({"items": items, "selected_indices": selected});
    match op {
        "archive" => {
            server
                .archive_artifact(Parameters(
                    serde_json::from_value::<BulkItems<super::super::ArtifactArchiveItem>>(args)
                        .unwrap(),
                ))
                .await
        }
        "unarchive" => {
            server
                .unarchive_artifact(Parameters(
                    serde_json::from_value::<BulkItems<super::super::ArtifactArchiveItem>>(args)
                        .unwrap(),
                ))
                .await
        }
        _ => {
            server
                .update_artifact(Parameters(
                    serde_json::from_value::<BulkItems<super::super::UpdateArtifactItem>>(args)
                        .unwrap(),
                ))
                .await
        }
    }
}

fn item(op: &str, id: &str) -> Value {
    if op == "update" {
        json!({"artifact_id":id,"title":"new title"})
    } else {
        json!({"artifact_id":id})
    }
}

fn updated(id: &str) -> UpdateArtifactResponse {
    UpdateArtifactResponse {
        artifact: Some(sample_with(
            |artifact| artifact.artifact_id = id.into(),
            |version| {
                version.artifact_id = id.into();
                version.version = 2;
                version.title = Some("new title".into());
            },
        )),
    }
}

#[tokio::test]
async fn selection_preserves_positions_and_artifact_relay_text() {
    for op in ["archive", "unarchive", "update"] {
        let mut mock = MockArtifactServiceImpl::new();
        match op {
            "archive" => {
                mock.expect_archive_artifact()
                    .times(2)
                    .withf(|req| req.get_ref().artifact_id != "skip")
                    .returning(|_| Ok(Response::new(ArchiveArtifactResponse {})));
            }
            "unarchive" => {
                mock.expect_unarchive_artifact()
                    .times(2)
                    .withf(|req| req.get_ref().artifact_id != "skip")
                    .returning(|_| Ok(Response::new(UnarchiveArtifactResponse {})));
            }
            _ => {
                mock.expect_update_artifact()
                    .times(2)
                    .withf(|req| req.get_ref().artifact_id != "skip")
                    .returning(|req| Ok(Response::new(updated(&req.get_ref().artifact_id))));
            }
        }
        let (server, _h) = server_with_mock(mock, true).await;
        let body = structured(
            call(
                &server,
                op,
                json!([item(op, "a"), item(op, "skip"), item(op, "c")]),
                Some(vec![0, 2]),
            )
            .await
            .unwrap(),
        );
        let count = match op {
            "archive" => "archived_count",
            "unarchive" => "unarchived_count",
            _ => "updated_count",
        };
        assert_eq!(body[count], 2);
        assert_eq!(body["not_selected"], json!([1]));
        assert_eq!(body["successes"][1]["index"], 2);
        if op == "update" {
            assert!(
                body["successes"][0]["result"]["next_step"]
                    .as_str()
                    .unwrap()
                    .starts_with("Wrote version 2 of artifact a")
            );
            assert_eq!(
                body["successes"][1]["result"]["artifact"]["artifactId"],
                "c"
            );
        }
    }
}

#[tokio::test]
async fn resource_errors_continue_and_backend_errors_stop() {
    for op in ["archive", "unarchive", "update"] {
        for code in [
            tonic::Code::NotFound,
            tonic::Code::PermissionDenied,
            tonic::Code::FailedPrecondition,
            tonic::Code::Unavailable,
        ] {
            let mut mock = MockArtifactServiceImpl::new();
            let stop = code == tonic::Code::Unavailable;
            match op {
                "archive" => {
                    mock.expect_archive_artifact()
                        .withf(|req| req.get_ref().artifact_id == "bad")
                        .returning(move |_| Err(Status::new(code, "failed")));
                    if !stop {
                        mock.expect_archive_artifact()
                            .times(1)
                            .withf(|req| req.get_ref().artifact_id == "good")
                            .returning(|_| Ok(Response::new(ArchiveArtifactResponse {})));
                    }
                }
                "unarchive" => {
                    mock.expect_unarchive_artifact()
                        .withf(|req| req.get_ref().artifact_id == "bad")
                        .returning(move |_| Err(Status::new(code, "failed")));
                    if !stop {
                        mock.expect_unarchive_artifact()
                            .times(1)
                            .withf(|req| req.get_ref().artifact_id == "good")
                            .returning(|_| Ok(Response::new(UnarchiveArtifactResponse {})));
                    }
                }
                _ => {
                    mock.expect_update_artifact()
                        .withf(|req| req.get_ref().artifact_id == "bad")
                        .returning(move |_| Err(Status::new(code, "failed")));
                    if !stop {
                        mock.expect_update_artifact()
                            .times(1)
                            .withf(|req| req.get_ref().artifact_id == "good")
                            .returning(|req| {
                                Ok(Response::new(updated(&req.get_ref().artifact_id)))
                            });
                    }
                }
            }
            let (server, _h) = server_with_mock(mock, true).await;
            let result = call(
                &server,
                op,
                json!([item(op, "bad"), item(op, "good")]),
                None,
            )
            .await
            .unwrap();
            assert_eq!(result.is_error, Some(true));
            let body = structured(result);
            assert_eq!(body["failures"].as_array().unwrap().len(), 1);
            if op == "update" {
                assert_eq!(body["failures"][0]["outcome_unknown"], true);
            }
            assert_eq!(
                body["successes"].as_array().unwrap().len(),
                usize::from(!stop)
            );
            assert_eq!(
                body["not_attempted"].as_array().unwrap().len(),
                usize::from(stop)
            );
        }
    }
}

#[tokio::test]
async fn invalid_items_duplicates_and_missing_masks_write_nothing() {
    for op in ["archive", "unarchive", "update"] {
        let (server, _h) = server_with_mock(MockArtifactServiceImpl::new(), true).await;
        for items in [
            json!([item(op, "a"), item(op, " a ")]),
            json!([item(op, "a"), item(op, " ")]),
            json!([{"artifact_id":"a","unknown":true}]),
        ] {
            assert_eq!(
                call(&server, op, items, None).await.unwrap_err().code,
                ErrorCode::INVALID_PARAMS
            );
        }
        if op == "update" {
            for bad in [
                json!({"artifact_id":"b"}),
                json!({"artifact_id":"b","file_path":""}),
                json!({"artifact_id":"b","file_path":"/nonexistent/bulk-artifact"}),
                json!({"artifact_id":"b","links":[{"entity_id":"x","entity_type":"invalid","relation":"source"}]}),
            ] {
                assert_eq!(
                    call(&server, op, json!([item(op, "a"), bad]), None)
                        .await
                        .unwrap_err()
                        .code,
                    ErrorCode::INVALID_PARAMS
                );
            }
        }
        let server = server.with_require_selection(true);
        assert!(
            call(&server, op, json!([item(op, "a")]), None)
                .await
                .unwrap_err()
                .message
                .contains("approval host")
        );
    }
}
