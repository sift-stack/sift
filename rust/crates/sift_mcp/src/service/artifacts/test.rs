use sift_rs::{
    artifacts::v1::{
        Artifact, ArtifactAuthoringKind, ArtifactCreatedVia, ArtifactLinkInput,
        ArtifactLinkRelation, ArtifactStorageClass, CreateArtifactResponse, GetArtifactResponse,
        ListArtifactsResponse, artifact_service_server::ArtifactServiceServer,
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
use tonic::{Code, Response, Status, transport::Server};

use super::{ArtifactService, CreateArtifactInput};
use crate::policy::RetryPolicy;

fn sample_artifact() -> Artifact {
    Artifact {
        artifact_id: "art-1".into(),
        organization_id: "org-1".into(),
        created_by_user_id: "user-1".into(),
        authoring_kind: ArtifactAuthoringKind::Agent as i32,
        artifact_version_id: "ver-1".into(),
        version: 1,
        title: Some("report".into()),
        file_name: Some("report.md".into()),
        file_mime_type: Some("text/markdown".into()),
        ..Default::default()
    }
}

async fn service_with_mock(mock: MockArtifactServiceImpl) -> (ArtifactService, JoinHandle<()>) {
    service_with_mocks(mock, MockRemoteFileServiceImpl::new()).await
}

async fn service_with_mocks(
    artifacts: MockArtifactServiceImpl,
    remote_files: MockRemoteFileServiceImpl,
) -> (ArtifactService, JoinHandle<()>) {
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
        ArtifactService::new(channel, RetryPolicy::default()),
        handle,
    )
}

#[tokio::test]
async fn list_artifacts_returns_single_page() {
    let mut mock = MockArtifactServiceImpl::new();
    mock.expect_list_artifacts()
        .withf(|req| {
            let req = req.get_ref();
            req.conversation_id.as_deref() == Some("conv-1")
                && req.filter == "kind == \"table\""
                && req.order_by == "created_date desc"
        })
        .returning(|_| {
            Ok(Response::new(ListArtifactsResponse {
                artifacts: vec![sample_artifact()],
                next_page_token: String::new(),
            }))
        });

    let (service, _h) = service_with_mock(mock).await;
    let page = service
        .list_artifacts(
            Some("conv-1".into()),
            false,
            "kind == \"table\"".into(),
            Some("created_date desc".into()),
            None,
        )
        .await
        .expect("list");
    assert_eq!(page.items.len(), 1);
    assert_eq!(page.items[0].artifact_id, "art-1");
    assert!(!page.has_more);
}

#[tokio::test]
async fn list_artifacts_paginates_until_token_empty() {
    let mut mock = MockArtifactServiceImpl::new();
    mock.expect_list_artifacts().returning(|req| {
        let req = req.into_inner();
        assert_eq!(req.page_size, 200);
        let (artifacts, next) = match req.page_token.as_str() {
            "" => (
                vec![Artifact {
                    artifact_id: "a1".into(),
                    artifact_version_id: "v1".into(),
                    version: 1,
                    ..Default::default()
                }],
                "50".to_string(),
            ),
            "50" => (
                vec![Artifact {
                    artifact_id: "a2".into(),
                    artifact_version_id: "v2".into(),
                    version: 1,
                    ..Default::default()
                }],
                String::new(),
            ),
            other => return Err(Status::invalid_argument(format!("bad token: {other}"))),
        };
        Ok(Response::new(ListArtifactsResponse {
            artifacts,
            next_page_token: next,
        }))
    });

    let (service, _h) = service_with_mock(mock).await;
    let page = service
        .list_artifacts(None, false, String::new(), None, Some(200))
        .await
        .expect("list");
    assert_eq!(
        page.items
            .iter()
            .map(|a| a.artifact_id.as_str())
            .collect::<Vec<_>>(),
        ["a1", "a2"]
    );
    assert!(!page.has_more);
}

#[tokio::test]
async fn list_artifacts_limit_truncates() {
    let mut mock = MockArtifactServiceImpl::new();
    mock.expect_list_artifacts().returning(|_| {
        Ok(Response::new(ListArtifactsResponse {
            artifacts: vec![
                Artifact {
                    artifact_id: "a1".into(),
                    ..Default::default()
                },
                Artifact {
                    artifact_id: "a2".into(),
                    ..Default::default()
                },
                Artifact {
                    artifact_id: "a3".into(),
                    ..Default::default()
                },
            ],
            next_page_token: String::new(),
        }))
    });

    let (service, _h) = service_with_mock(mock).await;
    let page = service
        .list_artifacts(None, false, String::new(), None, Some(2))
        .await
        .expect("list");
    assert_eq!(page.items.len(), 2);
    assert!(page.has_more);
}

#[tokio::test]
async fn list_artifacts_propagates_not_found() {
    let mut mock = MockArtifactServiceImpl::new();
    mock.expect_list_artifacts()
        .returning(|_| Err(Status::not_found("conversation not found")));

    let (service, _h) = service_with_mock(mock).await;
    let err = service
        .list_artifacts(Some("missing".into()), false, String::new(), None, None)
        .await
        .expect_err("expected error");
    let status = err.downcast_ref::<tonic::Status>().expect("status");
    assert_eq!(status.code(), Code::NotFound);
}

#[tokio::test]
async fn download_artifact_returns_latest() {
    let mut mock = MockArtifactServiceImpl::new();
    mock.expect_get_artifact()
        .withf(|req| {
            let req = req.get_ref();
            req.artifact_id == "art-1" && req.artifact_version_id.is_none()
        })
        .returning(|_| {
            Ok(Response::new(GetArtifactResponse {
                artifact: Some(sample_artifact()),
            }))
        });

    let (service, _h) = service_with_mock(mock).await;
    let artifact = service
        .download_artifact("art-1".into(), None)
        .await
        .expect("get");
    assert_eq!(artifact.inner.artifact_id, "art-1");
    assert_eq!(artifact.inner.version, 1);
    assert!(artifact.download_url.is_none());
}

fn uploaded_artifact() -> Artifact {
    Artifact {
        remote_file_id: Some("rf-1".into()),
        ..sample_artifact()
    }
}

fn get_returns_uploaded() -> MockArtifactServiceImpl {
    let mut mock = MockArtifactServiceImpl::new();
    mock.expect_get_artifact().returning(|_| {
        Ok(Response::new(GetArtifactResponse {
            artifact: Some(uploaded_artifact()),
        }))
    });
    mock
}

#[tokio::test]
async fn download_artifact_attaches_download_url_when_bytes_uploaded() {
    let mut remote_files = MockRemoteFileServiceImpl::new();
    remote_files
        .expect_get_remote_file_download_url()
        .withf(|req| req.get_ref().remote_file_id == "rf-1")
        .times(1)
        .returning(|_| {
            Ok(Response::new(GetRemoteFileDownloadUrlResponse {
                download_url: "https://files.test.local/rf-1?sig=abc".into(),
            }))
        });

    let (service, _h) = service_with_mocks(get_returns_uploaded(), remote_files).await;
    let artifact = service
        .download_artifact("art-1".into(), None)
        .await
        .expect("get");
    assert_eq!(artifact.inner.remote_file_id.as_deref(), Some("rf-1"));
    assert_eq!(
        artifact.download_url.as_deref(),
        Some("https://files.test.local/rf-1?sig=abc")
    );
}

#[tokio::test]
async fn download_artifact_propagates_download_url_error() {
    let mut remote_files = MockRemoteFileServiceImpl::new();
    remote_files
        .expect_get_remote_file_download_url()
        .returning(|_| Err(Status::permission_denied("no access to remote file")));

    let (service, _h) = service_with_mocks(get_returns_uploaded(), remote_files).await;
    let err = service
        .download_artifact("art-1".into(), None)
        .await
        .expect_err("download url failure must not be swallowed");
    let status = err.downcast_ref::<tonic::Status>().expect("status");
    assert_eq!(status.code(), Code::PermissionDenied);
}

#[tokio::test]
async fn download_artifact_rejects_empty_download_url() {
    let mut remote_files = MockRemoteFileServiceImpl::new();
    remote_files
        .expect_get_remote_file_download_url()
        .returning(|_| {
            Ok(Response::new(GetRemoteFileDownloadUrlResponse {
                download_url: String::new(),
            }))
        });

    let (service, _h) = service_with_mocks(get_returns_uploaded(), remote_files).await;
    let err = service
        .download_artifact("art-1".into(), None)
        .await
        .expect_err("empty url is an error");
    assert!(err.to_string().contains("download url response was empty"));
}

#[test]
fn artifact_view_serializes_flat_with_snake_case_download_url() {
    let with_url = super::ArtifactView {
        inner: uploaded_artifact(),
        download_url: Some("https://files.test.local/rf-1".into()),
    };
    let value = serde_json::to_value(&with_url).expect("serialize");
    assert_eq!(value["artifactId"], "art-1");
    assert_eq!(value["remoteFileId"], "rf-1");
    assert_eq!(value["download_url"], "https://files.test.local/rf-1");
    assert!(value.get("downloadUrl").is_none());
    assert!(value.get("inner").is_none());

    let without_url = super::ArtifactView {
        inner: sample_artifact(),
        download_url: None,
    };
    let value = serde_json::to_value(&without_url).expect("serialize");
    assert_eq!(value["artifactId"], "art-1");
    assert!(value.get("download_url").is_none());
}

#[test]
fn artifact_view_serialization_error_propagates() {
    let unknown_kind = super::ArtifactView {
        inner: Artifact {
            authoring_kind: 999,
            ..sample_artifact()
        },
        download_url: None,
    };
    let err = serde_json::to_value(&unknown_kind).expect_err("unknown enum variant is an error");
    assert!(err.to_string().contains("999"), "{err}");
}

#[tokio::test]
async fn create_artifact_returns_created_row() {
    let mut mock = MockArtifactServiceImpl::new();
    mock.expect_create_artifact()
        .withf(|req| {
            let req = req.get_ref();
            req.conversation_id.as_deref() == Some("conv-1")
                && req.title.as_deref() == Some("report")
                && req.summary.as_deref() == Some("summary")
                && req.authoring_kind == Some(ArtifactAuthoringKind::Agent as i32)
                && req.storage_class == Some(ArtifactStorageClass::Structured as i32)
                && req.created_via == Some(ArtifactCreatedVia::Chat as i32)
                && req.kind.as_deref() == Some("table")
                && serde_json::to_value(req.payload.as_ref().unwrap()).unwrap()
                    == serde_json::json!({ "rows": [] })
                && req.links[0].relation == ArtifactLinkRelation::AttachedTo as i32
        })
        .returning(|_| {
            Ok(Response::new(CreateArtifactResponse {
                artifact: Some(sample_artifact()),
            }))
        });

    let (service, _h) = service_with_mock(mock).await;
    let artifact = service
        .create_artifact(
            CreateArtifactInput {
                title: Some("report".into()),
                summary: Some("summary".into()),
                conversation_id: Some("conv-1".into()),
                artifact_id: None,
                authoring_kind: ArtifactAuthoringKind::Agent,
                storage_class: Some(ArtifactStorageClass::Structured),
                created_via: Some(ArtifactCreatedVia::Chat),
                kind: Some("table".into()),
                payload: Some(serde_json::from_value(serde_json::json!({ "rows": [] })).unwrap()),
                metadata: vec![],
                links: vec![ArtifactLinkInput {
                    relation: ArtifactLinkRelation::AttachedTo as i32,
                    entity_type: "conversations".into(),
                    entity_id: "conv-1".into(),
                }],
            },
            None,
        )
        .await
        .expect("create");
    assert_eq!(artifact.inner.artifact_id, "art-1");
    assert_eq!(artifact.inner.version, 1);
}
