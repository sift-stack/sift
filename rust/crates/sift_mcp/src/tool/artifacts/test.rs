use rmcp::{handler::server::wrapper::Parameters, model::ErrorCode};
use sift_rs::{
    artifacts::v1::{
        ArchiveArtifactResponse, Artifact, ArtifactCreatedVia, ArtifactDetails, ArtifactEntityType,
        ArtifactLinkRelation, ArtifactStorageClass, ArtifactVersion, CreateArtifactResponse,
        GetArtifactResponse, ListArtifactVersionsResponse, ListArtifactsResponse,
        UnarchiveArtifactResponse, UpdateArtifactResponse,
        artifact_service_server::ArtifactServiceServer,
    },
    remote_files::v1::{
        GetRemoteFileDownloadUrlResponse, remote_file_service_server::RemoteFileServiceServer,
    },
};
use sift_test_util::{
    grpc::memory_sift_channel,
    mock::{artifacts::v1::MockArtifactServiceImpl, remote_files::v1::MockRemoteFileServiceImpl},
};
use tokio::task::JoinHandle;
use tonic::{Response, Status, transport::Server};

use super::{
    ArtifactArchiveParams, ArtifactVersionListParams, CreateArtifactParams, DownloadArtifactParams,
    UpdateArtifactParams, parse_created_via, parse_storage_class,
};
use crate::{
    server::SiftMcpServer,
    tool::{
        artifacts::ArtifactListParams,
        common::{
            MetadataEntry, MetadataScalar,
            test_support::{structured, structured_field},
        },
    },
};

fn sample_artifact() -> ArtifactDetails {
    sample_with(|_| {}, |_| {})
}

/// The sample pair with either half adjusted, so a test can set a container
/// field and a version field without restating both messages.
fn sample_with(
    container: impl FnOnce(&mut Artifact),
    version: impl FnOnce(&mut ArtifactVersion),
) -> ArtifactDetails {
    let mut artifact = Artifact {
        artifact_id: "art-1".into(),
        organization_id: "org-1".into(),
        current_version_id: "ver-1".into(),
        ..Default::default()
    };
    let mut artifact_version = ArtifactVersion {
        artifact_version_id: "ver-1".into(),
        artifact_id: "art-1".into(),
        version: 1,
        title: Some("report".into()),
        ..Default::default()
    };
    container(&mut artifact);
    version(&mut artifact_version);
    ArtifactDetails {
        artifact: Some(artifact),
        artifact_version: Some(artifact_version),
    }
}

#[test]
fn parse_container_fields_omit_empty_and_missing_values() {
    for value in [None, Some(String::new())] {
        assert_eq!(parse_storage_class(value.clone()).unwrap(), None);
        assert_eq!(parse_created_via(value).unwrap(), None);
    }
    assert!(parse_created_via(Some("sdk".into())).is_err());
}

#[test]
fn include_archived_uses_the_filter_directive() {
    assert_eq!(
        super::with_include_archived(String::new(), true),
        "include_archived == true"
    );
    assert_eq!(
        super::with_include_archived("storage_class == \"FILE\"".into(), true),
        "(storage_class == \"FILE\") && include_archived == true"
    );
    assert_eq!(
        super::with_include_archived("include_archived == false".into(), true),
        "include_archived == false"
    );
}

#[test]
fn parse_entity_types_uses_the_proto_enum() {
    for (value, expected) in [
        ("conversation", ArtifactEntityType::Conversation),
        ("canvas", ArtifactEntityType::Canvas),
        ("run", ArtifactEntityType::Run),
        ("asset", ArtifactEntityType::Asset),
        ("artifact", ArtifactEntityType::Artifact),
        ("tool_use", ArtifactEntityType::ToolUse),
    ] {
        assert_eq!(super::parse_entity_type(value.into()).unwrap(), expected);
    }
}

async fn server_with_mock(
    mock: MockArtifactServiceImpl,
    allow_create: bool,
) -> (SiftMcpServer, JoinHandle<()>) {
    server_with_mocks(
        mock,
        MockRemoteFileServiceImpl::new(),
        allow_create,
        allow_create,
    )
    .await
}

async fn server_with_mocks(
    artifacts: MockArtifactServiceImpl,
    remote_files: MockRemoteFileServiceImpl,
    allow_create: bool,
    allow_destructive: bool,
) -> (SiftMcpServer, JoinHandle<()>) {
    let (client, server) = tokio::io::duplex(1024);
    let channel = memory_sift_channel(client).await;

    let handle = tokio::spawn(async move {
        Server::builder()
            .add_service(ArtifactServiceServer::new(artifacts))
            .add_service(RemoteFileServiceServer::new(remote_files))
            .serve_with_incoming(tokio_stream::once(Ok::<_, std::io::Error>(server)))
            .await
            .unwrap();
    });

    (
        SiftMcpServer::new(
            channel,
            String::from("https://app.test.local"),
            allow_create,
            allow_destructive,
        ),
        handle,
    )
}

fn get_returns(artifact: ArtifactDetails) -> MockArtifactServiceImpl {
    let mut mock = MockArtifactServiceImpl::new();
    mock.expect_get_artifact().returning(move |_| {
        Ok(Response::new(GetArtifactResponse {
            artifact: Some(artifact.clone()),
        }))
    });
    mock
}

#[tokio::test]
async fn list_artifacts_returns_rows() {
    let mut mock = MockArtifactServiceImpl::new();
    mock.expect_list_artifacts()
        .withf(|request| {
            let request = request.get_ref();
            request.conversation_id.as_deref() == Some("conv-1")
                && request.filter == "storage_class == \"STRUCTURED\""
                && request.order_by == "created_date desc"
        })
        .returning(|_| {
            Ok(Response::new(ListArtifactsResponse {
                artifacts: vec![sample_artifact()],
                next_page_token: String::new(),
            }))
        });

    let (server, _h) = server_with_mock(mock, true).await;
    let resp = server
        .list_artifacts(Parameters(ArtifactListParams {
            conversation_id: Some("conv-1".into()),
            include_archived: None,
            filter: "storage_class == \"STRUCTURED\"".into(),
            order_by: Some("created_date desc".into()),
            limit: None,
            fields: None,
        }))
        .await
        .expect("list");
    let artifacts = structured_field(resp, "artifacts");
    assert_eq!(artifacts.as_array().unwrap().len(), 1);
    assert_eq!(artifacts[0]["artifactId"], "art-1");
}

#[tokio::test]
async fn list_artifacts_rejects_empty_conversation_id() {
    let (server, _h) = server_with_mock(MockArtifactServiceImpl::new(), true).await;
    let err = server
        .list_artifacts(Parameters(ArtifactListParams {
            conversation_id: Some("  ".into()),
            include_archived: None,
            filter: String::new(),
            order_by: None,
            limit: None,
            fields: None,
        }))
        .await
        .expect_err("empty conversation");
    assert_eq!(err.code, ErrorCode::INVALID_PARAMS);
}

#[tokio::test]
async fn get_artifact_rejects_empty_id() {
    let (server, _h) = server_with_mock(MockArtifactServiceImpl::new(), true).await;
    let err = server
        .download_artifact(Parameters(DownloadArtifactParams {
            artifact_id: String::new(),
            artifact_version_id: None,
        }))
        .await
        .expect_err("empty id");
    assert_eq!(err.code, ErrorCode::INVALID_PARAMS);
}

#[tokio::test]
async fn get_artifact_returns_snake_case_download_url() {
    let uploaded = sample_with(
        |_| {},
        |version| version.remote_file_id = Some("rf-1".into()),
    );
    let mut remote_files = MockRemoteFileServiceImpl::new();
    remote_files
        .expect_get_remote_file_download_url()
        .returning(|_| {
            Ok(Response::new(GetRemoteFileDownloadUrlResponse {
                download_url: "https://files.test.local/rf-1".into(),
            }))
        });

    let (server, _h) = server_with_mocks(get_returns(uploaded), remote_files, false, false).await;
    let resp = server
        .download_artifact(Parameters(DownloadArtifactParams {
            artifact_id: "art-1".into(),
            artifact_version_id: None,
        }))
        .await
        .expect("get");
    let artifact = structured_field(resp, "artifact");
    assert_eq!(artifact["artifactId"], "art-1");
    assert_eq!(artifact["download_url"], "https://files.test.local/rf-1");
    assert!(artifact.get("downloadUrl").is_none());
}

#[tokio::test]
async fn get_artifact_omits_download_url_without_bytes() {
    let (server, _h) = server_with_mock(get_returns(sample_artifact()), false).await;
    let resp = server
        .download_artifact(Parameters(DownloadArtifactParams {
            artifact_id: "art-1".into(),
            artifact_version_id: None,
        }))
        .await
        .expect("get");
    let artifact = structured_field(resp, "artifact");
    assert!(artifact.get("download_url").is_none());
}

#[tokio::test]
async fn get_artifact_returns_structured_payload() {
    let structured = sample_with(
        |artifact| artifact.storage_class = ArtifactStorageClass::Structured as i32,
        |version| {
            version.payload =
                Some(serde_json::from_value(serde_json::json!({ "rows": [[1, 2]] })).unwrap())
        },
    );
    let (server, _h) = server_with_mock(get_returns(structured), false).await;
    let resp = server
        .download_artifact(Parameters(DownloadArtifactParams {
            artifact_id: "art-1".into(),
            artifact_version_id: None,
        }))
        .await
        .expect("get");
    let artifact = structured_field(resp, "artifact");
    assert!(artifact["payload"].is_object());
    assert!(artifact["payload"].get("rows").is_some());
}

#[tokio::test]
async fn get_artifact_surfaces_download_url_failure() {
    let uploaded = sample_with(
        |_| {},
        |version| version.remote_file_id = Some("rf-1".into()),
    );
    let mut remote_files = MockRemoteFileServiceImpl::new();
    remote_files
        .expect_get_remote_file_download_url()
        .returning(|_| Err(Status::not_found("remote file gone")));

    let (server, _h) = server_with_mocks(get_returns(uploaded), remote_files, false, false).await;
    let err = server
        .download_artifact(Parameters(DownloadArtifactParams {
            artifact_id: "art-1".into(),
            artifact_version_id: None,
        }))
        .await
        .expect_err("download failure is an error, not a partial artifact");
    assert_eq!(err.code, ErrorCode::RESOURCE_NOT_FOUND);
}

#[tokio::test]
async fn list_artifact_versions_returns_rows_newest_first() {
    let mut mock = MockArtifactServiceImpl::new();
    mock.expect_list_artifact_versions()
        .withf(|request| request.get_ref().artifact_id == "art-1")
        .returning(|_| {
            Ok(Response::new(ListArtifactVersionsResponse {
                versions: vec![
                    ArtifactVersion {
                        artifact_id: "art-1".into(),
                        artifact_version_id: "ver-2".into(),
                        version: 2,
                        ..Default::default()
                    },
                    ArtifactVersion {
                        artifact_id: "art-1".into(),
                        artifact_version_id: "ver-1".into(),
                        version: 1,
                        ..Default::default()
                    },
                ],
                next_page_token: String::new(),
            }))
        });

    let (server, _h) = server_with_mock(mock, true).await;
    let resp = server
        .list_artifact_versions(Parameters(ArtifactVersionListParams {
            artifact_id: "art-1".into(),
            limit: None,
            fields: None,
        }))
        .await
        .expect("list versions");
    let versions = structured_field(resp, "artifact_versions");
    let rows = versions.as_array().expect("rows");
    assert_eq!(rows.len(), 2);
    assert_eq!(rows[0]["artifactVersionId"], "ver-2");
    assert_eq!(rows[1]["artifactVersionId"], "ver-1");
}

#[tokio::test]
async fn list_artifact_versions_rejects_empty_artifact_id() {
    let (server, _h) = server_with_mock(MockArtifactServiceImpl::new(), true).await;
    let err = server
        .list_artifact_versions(Parameters(ArtifactVersionListParams {
            artifact_id: "  ".into(),
            limit: None,
            fields: None,
        }))
        .await
        .expect_err("empty id");
    assert_eq!(err.code, ErrorCode::INVALID_PARAMS);
}

#[tokio::test]
async fn archive_artifact_blocked_without_allow_destructive() {
    let (server, _h) = server_with_mocks(
        MockArtifactServiceImpl::new(),
        MockRemoteFileServiceImpl::new(),
        true,
        false,
    )
    .await;
    let err = server
        .archive_artifact(Parameters(ArtifactArchiveParams {
            artifact_id: "art-1".into(),
        }))
        .await
        .expect_err("archive gated");
    assert_eq!(err.code, ErrorCode::INVALID_REQUEST);
    assert!(err.message.contains("--allow-destructive"));
}

#[tokio::test]
async fn unarchive_artifact_blocked_without_allow_destructive() {
    let (server, _h) = server_with_mocks(
        MockArtifactServiceImpl::new(),
        MockRemoteFileServiceImpl::new(),
        true,
        false,
    )
    .await;
    let err = server
        .unarchive_artifact(Parameters(ArtifactArchiveParams {
            artifact_id: "art-1".into(),
        }))
        .await
        .expect_err("unarchive gated");
    assert_eq!(err.code, ErrorCode::INVALID_REQUEST);
    assert!(err.message.contains("--allow-destructive"));
}

#[tokio::test]
async fn archive_artifact_rejects_empty_artifact_id() {
    let (server, _h) = server_with_mock(MockArtifactServiceImpl::new(), true).await;
    let err = server
        .archive_artifact(Parameters(ArtifactArchiveParams {
            artifact_id: "  ".into(),
        }))
        .await
        .expect_err("empty id");
    assert_eq!(err.code, ErrorCode::INVALID_PARAMS);
}

#[tokio::test]
async fn unarchive_artifact_rejects_empty_artifact_id() {
    let (server, _h) = server_with_mock(MockArtifactServiceImpl::new(), true).await;
    let err = server
        .unarchive_artifact(Parameters(ArtifactArchiveParams {
            artifact_id: "  ".into(),
        }))
        .await
        .expect_err("empty id");
    assert_eq!(err.code, ErrorCode::INVALID_PARAMS);
}

#[tokio::test]
async fn archive_artifact_returns_structured_result() {
    let mut mock = MockArtifactServiceImpl::new();
    mock.expect_archive_artifact()
        .withf(|req| req.get_ref().artifact_id == "art-1")
        .returning(|_| Ok(Response::new(ArchiveArtifactResponse {})));

    let (server, _h) = server_with_mock(mock, true).await;
    let response = server
        .archive_artifact(Parameters(ArtifactArchiveParams {
            artifact_id: "art-1".into(),
        }))
        .await
        .expect("archive");
    let body = structured(response);
    assert_eq!(body["artifact_id"], "art-1");
    assert_eq!(body["archived"], true);
    assert!(body["next_step"].is_string());
}

#[tokio::test]
async fn unarchive_artifact_returns_structured_result() {
    let mut mock = MockArtifactServiceImpl::new();
    mock.expect_unarchive_artifact()
        .withf(|req| req.get_ref().artifact_id == "art-1")
        .returning(|_| Ok(Response::new(UnarchiveArtifactResponse {})));

    let (server, _h) = server_with_mock(mock, true).await;
    let response = server
        .unarchive_artifact(Parameters(ArtifactArchiveParams {
            artifact_id: "art-1".into(),
        }))
        .await
        .expect("unarchive");
    let body = structured(response);
    assert_eq!(body["artifact_id"], "art-1");
    assert_eq!(body["unarchived"], true);
    assert!(body["next_step"].is_string());
}

#[tokio::test]
async fn create_artifact_blocked_without_allow_create() {
    let (server, _h) = server_with_mock(MockArtifactServiceImpl::new(), false).await;
    let err = server
        .create_artifact(Parameters(CreateArtifactParams::default()))
        .await
        .expect_err("gated");
    assert_eq!(err.code, ErrorCode::INVALID_REQUEST);
    assert!(err.message.contains("--allow-create"));
}

#[tokio::test]
async fn update_artifact_blocked_without_allow_destructive() {
    let (server, _h) = server_with_mocks(
        MockArtifactServiceImpl::new(),
        MockRemoteFileServiceImpl::new(),
        true,
        false,
    )
    .await;
    let err = server
        .update_artifact(Parameters(UpdateArtifactParams {
            artifact_id: "art-1".into(),
            title: Some("v2".into()),
            ..Default::default()
        }))
        .await
        .expect_err("update gated");
    assert_eq!(err.code, ErrorCode::INVALID_REQUEST);
    assert!(err.message.contains("--allow-destructive"));
}

#[tokio::test]
async fn update_artifact_masks_only_what_was_passed() {
    let mut mock = MockArtifactServiceImpl::new();
    mock.expect_update_artifact()
        .withf(|req| {
            let req = req.get_ref();
            let version = req
                .artifact
                .as_ref()
                .unwrap()
                .artifact_version
                .as_ref()
                .unwrap();
            req.artifact_id == "art-1"
                && req.update_mask.as_ref().unwrap().paths == ["artifact_version.title"]
                && version.title.as_deref() == Some("Renamed")
                // The payload it did not send is the point: the server carries it forward.
                && version.payload.is_none()
                && version.summary.is_none()
                && version.metadata.is_empty()
        })
        .returning(|_| {
            Ok(Response::new(UpdateArtifactResponse {
                artifact: Some(sample_with(
                    |_| {},
                    |version| {
                        version.artifact_version_id = "ver-2".into();
                        version.version = 2;
                        version.title = Some("Renamed".into());
                    },
                )),
            }))
        });

    let (server, _h) = server_with_mock(mock, true).await;
    let resp = server
        .update_artifact(Parameters(UpdateArtifactParams {
            artifact_id: "art-1".into(),
            title: Some("Renamed".into()),
            ..Default::default()
        }))
        .await
        .expect("update");
    let artifact = structured_field(resp.clone(), "artifact");
    assert_eq!(artifact["version"], 2);
    assert_eq!(artifact["title"], "Renamed");
    let next_step = structured_field(resp, "next_step");
    let next_step = next_step.as_str().unwrap();
    assert!(
        next_step.starts_with("Wrote version 2 of artifact art-1"),
        "{next_step}"
    );
}

#[tokio::test]
async fn update_artifact_masks_a_payload_replacement() {
    let mut mock = MockArtifactServiceImpl::new();
    mock.expect_update_artifact()
        .withf(|req| {
            let req = req.get_ref();
            req.update_mask.as_ref().unwrap().paths == ["artifact_version.payload"]
                && req
                    .artifact
                    .as_ref()
                    .unwrap()
                    .artifact_version
                    .as_ref()
                    .unwrap()
                    .payload
                    .is_some()
        })
        .returning(|_| {
            Ok(Response::new(UpdateArtifactResponse {
                artifact: Some(sample_with(
                    |artifact| artifact.storage_class = ArtifactStorageClass::Structured as i32,
                    |version| {
                        version.artifact_version_id = "ver-2".into();
                        version.version = 2;
                    },
                )),
            }))
        });

    let (server, _h) = server_with_mock(mock, true).await;
    let resp = server
        .update_artifact(Parameters(UpdateArtifactParams {
            artifact_id: "art-1".into(),
            payload: Some(serde_json::json!({ "step": 2 })),
            ..Default::default()
        }))
        .await
        .expect("update with payload");
    let next_step = structured_field(resp, "next_step");
    let next_step = next_step.as_str().unwrap();
    assert!(
        next_step.contains("It carries its JSON payload."),
        "{next_step}"
    );
}

#[tokio::test]
async fn update_artifact_surfaces_the_server_storage_class_check() {
    let mut mock = MockArtifactServiceImpl::new();
    mock.expect_update_artifact().returning(|_| {
        Err(Status::invalid_argument(
            "payload is legal only for STRUCTURED artifacts",
        ))
    });

    let (server, _h) = server_with_mock(mock, true).await;
    let err = server
        .update_artifact(Parameters(UpdateArtifactParams {
            artifact_id: "art-1".into(),
            payload: Some(serde_json::json!({ "step": 2 })),
            ..Default::default()
        }))
        .await
        .expect_err("payload on a file artifact");
    assert_eq!(err.code, ErrorCode::INVALID_PARAMS);
    assert!(
        err.message
            .contains("payload is legal only for STRUCTURED artifacts"),
        "{}",
        err.message
    );
}

#[tokio::test]
async fn update_artifact_requires_an_id_and_something_to_change() {
    let (server, _h) = server_with_mock(MockArtifactServiceImpl::new(), true).await;
    for params in [
        UpdateArtifactParams {
            artifact_id: "  ".into(),
            title: Some("Renamed".into()),
            ..Default::default()
        },
        UpdateArtifactParams {
            artifact_id: "art-1".into(),
            ..Default::default()
        },
    ] {
        let err = server
            .update_artifact(Parameters(params))
            .await
            .expect_err("rejected");
        assert_eq!(err.code, ErrorCode::INVALID_PARAMS);
    }
}

#[tokio::test]
async fn create_artifact_validates_storage_and_payload() {
    for params in [
        CreateArtifactParams {
            storage_class: Some("structured".into()),
            ..Default::default()
        },
        CreateArtifactParams {
            storage_class: Some("file".into()),
            payload: Some(serde_json::json!({ "rows": [] })),
            ..Default::default()
        },
        CreateArtifactParams {
            payload: Some(serde_json::json!({ "rows": [] })),
            ..Default::default()
        },
        CreateArtifactParams {
            storage_class: Some("unknown".into()),
            ..Default::default()
        },
        CreateArtifactParams {
            storage_class: Some("structured".into()),
            payload: Some(serde_json::json!({ "rows": [] })),
            file_path: Some("report.csv".into()),
            ..Default::default()
        },
        CreateArtifactParams {
            storage_class: Some("structured".into()),
            payload: Some(serde_json::json!(["not an object"])),
            ..Default::default()
        },
        CreateArtifactParams {
            storage_class: Some("structured".into()),
            payload: Some(serde_json::json!("{ not json")),
            ..Default::default()
        },
        CreateArtifactParams {
            links: Some(vec![super::ArtifactLinkParam {
                relation: "attached_to".into(),
                entity_type: String::new(),
                entity_id: "conv-1".into(),
            }]),
            ..Default::default()
        },
        CreateArtifactParams {
            links: Some(vec![super::ArtifactLinkParam {
                relation: "attached_to".into(),
                entity_type: "conversation".into(),
                entity_id: String::new(),
            }]),
            ..Default::default()
        },
        CreateArtifactParams {
            links: Some(vec![super::ArtifactLinkParam {
                relation: "invalid".into(),
                entity_type: "conversation".into(),
                entity_id: "conv-1".into(),
            }]),
            ..Default::default()
        },
        CreateArtifactParams {
            links: Some(vec![super::ArtifactLinkParam {
                relation: "attached_to".into(),
                entity_type: "conversations".into(),
                entity_id: "conv-1".into(),
            }]),
            ..Default::default()
        },
    ] {
        let (server, _h) = server_with_mock(MockArtifactServiceImpl::new(), true).await;
        let err = server
            .create_artifact(Parameters(params))
            .await
            .expect_err("invalid storage input");
        assert_eq!(err.code, ErrorCode::INVALID_PARAMS);
    }
}

#[tokio::test]
async fn create_artifact_accepts_a_stringified_payload() {
    let mut mock = MockArtifactServiceImpl::new();
    mock.expect_create_artifact()
        .withf(|request| {
            request
                .get_ref()
                .payload
                .as_ref()
                .is_some_and(|payload| payload.fields.contains_key("rows"))
        })
        .returning(|_| {
            Ok(Response::new(CreateArtifactResponse {
                artifact: Some(sample_artifact()),
            }))
        });

    let (server, _h) = server_with_mock(mock, true).await;
    server
        .create_artifact(Parameters(CreateArtifactParams {
            storage_class: Some("structured".into()),
            payload: Some(serde_json::json!(r#"{"rows": []}"#)),
            ..Default::default()
        }))
        .await
        .expect("create");
}

#[tokio::test]
async fn create_artifact_sends_generic_fields() {
    let mut mock = MockArtifactServiceImpl::new();
    mock.expect_create_artifact()
        .withf(|request| {
            let request = request.get_ref();
            request.storage_class == Some(ArtifactStorageClass::Structured as i32)
                && request.created_via == Some(ArtifactCreatedVia::Agent as i32)
                && request
                    .payload
                    .as_ref()
                    .is_some_and(|payload| payload.fields.contains_key("rows"))
                && request.metadata.len() == 1
                && request.links.len() == 1
                && request.links[0].relation == ArtifactLinkRelation::AttachedTo as i32
                && request.links[0].entity_type == ArtifactEntityType::Conversation as i32
                && request.links[0].entity_id == "conv-1"
        })
        .returning(|_| {
            Ok(Response::new(CreateArtifactResponse {
                artifact: Some(sample_with(
                    |artifact| artifact.storage_class = ArtifactStorageClass::Structured as i32,
                    |_| {},
                )),
            }))
        });

    let (server, _h) = server_with_mock(mock, true).await;
    let response = server
        .create_artifact(Parameters(CreateArtifactParams {
            storage_class: Some("structured".into()),
            created_via: Some("agent".into()),
            payload: Some(serde_json::json!({ "rows": [[1, 2]] })),
            metadata: Some(vec![MetadataEntry {
                name: "source".into(),
                value: MetadataScalar::String("computed".into()),
            }]),
            links: Some(vec![super::ArtifactLinkParam {
                relation: "attached_to".into(),
                entity_type: "conversation".into(),
                entity_id: "conv-1".into(),
            }]),
            ..Default::default()
        }))
        .await
        .expect("create");
    let next_step = structured_field(response, "next_step");
    assert!(next_step.as_str().unwrap().contains("JSON payload"));
}

#[tokio::test]
async fn create_artifact_happy_path() {
    let mut mock = MockArtifactServiceImpl::new();
    mock.expect_create_artifact()
        .withf(|request| request.get_ref().created_via.is_none())
        .returning(|_| {
            Ok(Response::new(CreateArtifactResponse {
                artifact: Some(sample_artifact()),
            }))
        });

    let (server, _h) = server_with_mock(mock, true).await;
    let resp = server
        .create_artifact(Parameters(CreateArtifactParams {
            title: Some("report".into()),
            conversation_id: Some("conv-1".into()),
            ..Default::default()
        }))
        .await
        .expect("create");
    let artifact = structured_field(resp, "artifact");
    assert_eq!(artifact["artifactId"], "art-1");
    assert!(artifact.get("download_url").is_none());
}

#[tokio::test]
async fn create_artifact_with_file_path_uploads_and_returns_the_refreshed_artifact() {
    use std::io::Write as _;

    use crate::client_event::start_http_server;
    use crate::service::remote_files::{RemoteFileUploader, RestConfig};

    let dir = tempdir::TempDir::new("artifact-tool-upload").unwrap();
    let path = dir.path().join("report.md");
    std::fs::File::create(&path)
        .unwrap()
        .write_all(b"# Battery Report\n")
        .unwrap();

    let (rest_uri, rest_server) = start_http_server(
        b"HTTP/1.1 200 OK\r\ncontent-type: application/json\r\ncontent-length: 2\r\nconnection: close\r\n\r\n{}"
            .to_vec(),
    )
    .await;

    let mut mock = MockArtifactServiceImpl::new();
    mock.expect_create_artifact().returning(|_| {
        Ok(Response::new(CreateArtifactResponse {
            artifact: Some(sample_artifact()),
        }))
    });
    // The refresh after the upload returns the version with its file fields.
    mock.expect_get_artifact().returning(|_| {
        Ok(Response::new(GetArtifactResponse {
            artifact: Some(sample_with(
                |_| {},
                |version| {
                    version.remote_file_id = Some("rf-1".into());
                    version.file_name = Some("report.md".into());
                },
            )),
        }))
    });
    let mut remote_files = MockRemoteFileServiceImpl::new();
    remote_files
        .expect_get_remote_file_download_url()
        .returning(|_| {
            Ok(Response::new(GetRemoteFileDownloadUrlResponse {
                download_url: "https://files.test.local/rf-1".into(),
            }))
        });

    let (server, _h) = server_with_mocks(mock, remote_files, true, true).await;
    let server = server.with_artifact_uploader(RemoteFileUploader::new(
        RestConfig::new(rest_uri, "test-key".into()),
        "1.2.3",
    ));

    let resp = server
        .create_artifact(Parameters(CreateArtifactParams {
            title: Some("report".into()),
            summary: None,
            conversation_id: None,
            file_path: Some(path.to_string_lossy().into_owned()),
            ..Default::default()
        }))
        .await
        .expect("create with file");

    let request = String::from_utf8(rest_server.await.unwrap()).unwrap();
    assert!(request.contains("name=\"entityId\""));
    assert!(request.contains("ver-1"));
    assert!(request.contains("# Battery Report"));

    let artifact = structured_field(resp.clone(), "artifact");
    assert_eq!(artifact["remoteFileId"], "rf-1");
    assert_eq!(artifact["fileName"], "report.md");
    assert_eq!(artifact["download_url"], "https://files.test.local/rf-1");
    let next_step = structured_field(resp, "next_step");
    assert!(
        next_step
            .as_str()
            .unwrap()
            .contains("file content was uploaded"),
        "{next_step}"
    );
}

#[tokio::test]
async fn create_artifact_rejects_an_empty_file_path() {
    let (server, _h) = server_with_mock(MockArtifactServiceImpl::new(), true).await;
    let err = server
        .create_artifact(Parameters(CreateArtifactParams {
            title: None,
            summary: None,
            conversation_id: None,
            file_path: Some("   ".into()),
            ..Default::default()
        }))
        .await
        .expect_err("empty file_path");
    assert_eq!(err.code, ErrorCode::INVALID_PARAMS);
}

#[tokio::test]
async fn create_artifact_names_the_created_artifact_when_the_upload_fails() {
    use std::io::Write as _;

    use crate::client_event::start_http_server;
    use crate::service::remote_files::{RemoteFileUploader, RestConfig};

    let dir = tempdir::TempDir::new("artifact-tool-upload-fail").unwrap();
    let path = dir.path().join("report.md");
    std::fs::File::create(&path)
        .unwrap()
        .write_all(b"# Battery Report\n")
        .unwrap();

    let (rest_uri, rest_server) = start_http_server(
        b"HTTP/1.1 500 Internal Server Error\r\ncontent-length: 0\r\nconnection: close\r\n\r\n"
            .to_vec(),
    )
    .await;

    let mut mock = MockArtifactServiceImpl::new();
    mock.expect_create_artifact().returning(|_| {
        Ok(Response::new(CreateArtifactResponse {
            artifact: Some(sample_artifact()),
        }))
    });

    let (server, _h) = server_with_mocks(mock, MockRemoteFileServiceImpl::new(), true, true).await;
    let server = server.with_artifact_uploader(RemoteFileUploader::new(
        RestConfig::new(rest_uri, "test-key".into()),
        "1.2.3",
    ));

    let err = server
        .create_artifact(Parameters(CreateArtifactParams {
            title: None,
            summary: None,
            conversation_id: None,
            file_path: Some(path.to_string_lossy().into_owned()),
            ..Default::default()
        }))
        .await
        .expect_err("upload failed");
    rest_server.await.unwrap();

    let message = format!("{err:?}");
    assert!(message.contains("art-1"), "{message}");
    assert!(
        message.contains("do NOT create the artifact again"),
        "{message}"
    );
}

#[tokio::test]
async fn create_artifact_with_file_path_says_so_when_the_download_link_is_missing() {
    use std::io::Write as _;

    use crate::client_event::start_http_server;
    use crate::service::remote_files::{RemoteFileUploader, RestConfig};

    let dir = tempdir::TempDir::new("artifact-tool-upload-nolink").unwrap();
    let path = dir.path().join("report.md");
    std::fs::File::create(&path)
        .unwrap()
        .write_all(b"# Battery Report\n")
        .unwrap();

    let (rest_uri, rest_server) = start_http_server(
        b"HTTP/1.1 200 OK\r\ncontent-type: application/json\r\ncontent-length: 2\r\nconnection: close\r\n\r\n{}"
            .to_vec(),
    )
    .await;

    let mut mock = MockArtifactServiceImpl::new();
    mock.expect_create_artifact().returning(|_| {
        Ok(Response::new(CreateArtifactResponse {
            artifact: Some(sample_artifact()),
        }))
    });
    // The post-upload refresh fails, so the response has no file fields or link.
    mock.expect_get_artifact()
        .returning(|_| Err(tonic::Status::not_found("gone")));

    let (server, _h) = server_with_mocks(mock, MockRemoteFileServiceImpl::new(), true, true).await;
    let server = server.with_artifact_uploader(RemoteFileUploader::new(
        RestConfig::new(rest_uri, "test-key".into()),
        "1.2.3",
    ));

    let resp = server
        .create_artifact(Parameters(CreateArtifactParams {
            title: Some("report".into()),
            summary: None,
            conversation_id: None,
            file_path: Some(path.to_string_lossy().into_owned()),
            ..Default::default()
        }))
        .await
        .expect("upload succeeded even though the refresh failed");
    rest_server.await.unwrap();

    let artifact = structured_field(resp.clone(), "artifact");
    assert!(artifact.get("download_url").is_none());
    let next_step = structured_field(resp, "next_step");
    let next_step = next_step.as_str().unwrap();
    assert!(
        next_step.contains("call `download_artifact`"),
        "{next_step}"
    );
    assert!(
        !next_step.contains("can preview and download"),
        "{next_step}"
    );
}
