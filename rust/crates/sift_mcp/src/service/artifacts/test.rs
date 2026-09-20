use sift_rs::{
    artifacts::v1::{
        ArchiveArtifactResponse, Artifact, ArtifactCreatedVia, ArtifactDetails, ArtifactEntityType,
        ArtifactLinkInput, ArtifactLinkRelation, ArtifactStorageClass, ArtifactVersion,
        CreateArtifactResponse, GetArtifactResponse, ListArtifactVersionsResponse,
        ListArtifactsResponse, UnarchiveArtifactResponse, UpdateArtifactResponse,
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
use tonic::{Code, Response, Status, transport::Server};

use super::{ArtifactService, CreateArtifactInput, UpdateArtifactInput};
use crate::policy::RetryPolicy;

/// A listing entry: the container plus its resolved version.
fn listed(artifact_id: &str, artifact_version_id: &str, version: u32) -> ArtifactDetails {
    ArtifactDetails {
        artifact: Some(Artifact {
            artifact_id: artifact_id.into(),
            current_version_id: artifact_version_id.into(),
            ..Default::default()
        }),
        artifact_version: Some(ArtifactVersion {
            artifact_version_id: artifact_version_id.into(),
            artifact_id: artifact_id.into(),
            version,
            ..Default::default()
        }),
    }
}

fn sample_artifact() -> ArtifactDetails {
    ArtifactDetails {
        artifact: Some(Artifact {
            artifact_id: "art-1".into(),
            organization_id: "org-1".into(),
            created_by_user_id: "user-1".into(),
            current_version_id: "ver-1".into(),
            ..Default::default()
        }),
        artifact_version: Some(ArtifactVersion {
            artifact_version_id: "ver-1".into(),
            artifact_id: "art-1".into(),
            version: 1,
            title: Some("report".into()),
            file_name: Some("report.md".into()),
            file_mime_type: Some("text/markdown".into()),
            ..Default::default()
        }),
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
                && req.filter == "storage_class == \"STRUCTURED\""
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
            "storage_class == \"STRUCTURED\"".into(),
            Some("created_date desc".into()),
            None,
        )
        .await
        .expect("list");
    assert_eq!(page.items.len(), 1);
    assert_eq!(
        page.items[0].artifact.as_ref().unwrap().artifact_id,
        "art-1"
    );
    assert!(!page.has_more);
}

#[tokio::test]
async fn list_artifacts_paginates_until_token_empty() {
    let mut mock = MockArtifactServiceImpl::new();
    mock.expect_list_artifacts().returning(|req| {
        let req = req.into_inner();
        assert_eq!(req.page_size, 200);
        let (artifacts, next) = match req.page_token.as_str() {
            "" => (vec![listed("a1", "v1", 1)], "50".to_string()),
            "50" => (vec![listed("a2", "v2", 1)], String::new()),
            other => return Err(Status::invalid_argument(format!("bad token: {other}"))),
        };
        Ok(Response::new(ListArtifactsResponse {
            artifacts,
            next_page_token: next,
        }))
    });

    let (service, _h) = service_with_mock(mock).await;
    let page = service
        .list_artifacts(None, String::new(), None, Some(200))
        .await
        .expect("list");
    assert_eq!(
        page.items
            .iter()
            .map(|a| a.artifact.as_ref().unwrap().artifact_id.as_str())
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
                listed("a1", "v1", 1),
                listed("a2", "v2", 1),
                listed("a3", "v3", 1),
            ],
            next_page_token: String::new(),
        }))
    });

    let (service, _h) = service_with_mock(mock).await;
    let page = service
        .list_artifacts(None, String::new(), None, Some(2))
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
        .list_artifacts(Some("missing".into()), String::new(), None, None)
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
    assert_eq!(artifact.artifact_id(), "art-1");
    assert_eq!(artifact.version().version, 1);
    assert!(artifact.download_url.is_none());
}

fn uploaded_artifact() -> ArtifactDetails {
    let mut details = sample_artifact();
    if let Some(version) = details.artifact_version.as_mut() {
        version.remote_file_id = Some("rf-1".into());
    }
    details
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
    assert_eq!(artifact.version().remote_file_id.as_deref(), Some("rf-1"));
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

#[tokio::test]
async fn list_artifact_versions_paginates_until_token_empty() {
    let mut mock = MockArtifactServiceImpl::new();
    mock.expect_list_artifact_versions().returning(|req| {
        let req = req.into_inner();
        assert_eq!(req.artifact_id, "art-1");
        assert_eq!(req.page_size, 200);
        let (versions, next) = match req.page_token.as_str() {
            "" => (
                vec![ArtifactVersion {
                    artifact_version_id: "ver-2".into(),
                    version: 2,
                    ..Default::default()
                }],
                "50".to_string(),
            ),
            "50" => (
                vec![ArtifactVersion {
                    artifact_version_id: "ver-1".into(),
                    version: 1,
                    ..Default::default()
                }],
                String::new(),
            ),
            other => return Err(Status::invalid_argument(format!("bad token: {other}"))),
        };
        Ok(Response::new(ListArtifactVersionsResponse {
            versions,
            next_page_token: next,
        }))
    });

    let (service, _h) = service_with_mock(mock).await;
    let page = service
        .list_artifact_versions("art-1".into(), Some(200))
        .await
        .expect("list versions");
    assert_eq!(
        page.items.iter().map(|v| v.version).collect::<Vec<_>>(),
        [2, 1]
    );
    assert!(!page.has_more);
}

#[tokio::test]
async fn list_artifact_versions_limit_truncates() {
    let mut mock = MockArtifactServiceImpl::new();
    mock.expect_list_artifact_versions().returning(|_| {
        Ok(Response::new(ListArtifactVersionsResponse {
            versions: vec![
                ArtifactVersion {
                    version: 3,
                    ..Default::default()
                },
                ArtifactVersion {
                    version: 2,
                    ..Default::default()
                },
                ArtifactVersion {
                    version: 1,
                    ..Default::default()
                },
            ],
            next_page_token: String::new(),
        }))
    });

    let (service, _h) = service_with_mock(mock).await;
    let page = service
        .list_artifact_versions("art-1".into(), Some(2))
        .await
        .expect("list versions");
    assert_eq!(page.items.len(), 2);
    assert!(page.has_more);
}

#[tokio::test]
async fn list_artifact_versions_propagates_not_found() {
    let mut mock = MockArtifactServiceImpl::new();
    mock.expect_list_artifact_versions()
        .returning(|_| Err(Status::not_found("artifact not found")));

    let (service, _h) = service_with_mock(mock).await;
    let err = service
        .list_artifact_versions("missing".into(), None)
        .await
        .expect_err("expected error");
    let status = err.downcast_ref::<tonic::Status>().expect("status");
    assert_eq!(status.code(), Code::NotFound);
}

#[tokio::test]
async fn archive_artifact_forwards_artifact_id() {
    let mut mock = MockArtifactServiceImpl::new();
    mock.expect_archive_artifact()
        .withf(|req| req.get_ref().artifact_id == "art-1")
        .returning(|_| Ok(Response::new(ArchiveArtifactResponse {})));

    let (service, _h) = service_with_mock(mock).await;
    service
        .archive_artifact("art-1".into())
        .await
        .expect("archive");
}

#[tokio::test]
async fn unarchive_artifact_forwards_artifact_id() {
    let mut mock = MockArtifactServiceImpl::new();
    mock.expect_unarchive_artifact()
        .withf(|req| req.get_ref().artifact_id == "art-1")
        .returning(|_| Ok(Response::new(UnarchiveArtifactResponse {})));

    let (service, _h) = service_with_mock(mock).await;
    service
        .unarchive_artifact("art-1".into())
        .await
        .expect("unarchive");
}

#[tokio::test]
async fn update_artifact_forwards_the_mask_and_version() {
    let mut mock = MockArtifactServiceImpl::new();
    mock.expect_update_artifact()
        .withf(|req| {
            let req = req.get_ref();
            let details = req.artifact.as_ref().unwrap();
            req.artifact_id == "art-1"
                && details.artifact.is_none()
                && details.artifact_version.as_ref().unwrap().title.as_deref() == Some("Renamed")
                && req.update_mask.as_ref().unwrap().paths == ["artifact_version.title"]
        })
        .returning(|_| {
            Ok(Response::new(UpdateArtifactResponse {
                artifact: Some(sample_artifact()),
            }))
        });

    let (service, _h) = service_with_mock(mock).await;
    let artifact = service
        .update_artifact(
            UpdateArtifactInput {
                artifact_id: "art-1".into(),
                version: ArtifactVersion {
                    title: Some("Renamed".into()),
                    ..Default::default()
                },
                links: vec![],
                update_mask: vec!["artifact_version.title".into()],
            },
            None,
        )
        .await
        .expect("update");
    assert_eq!(artifact.artifact_id(), "art-1");
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
                && req.storage_class == Some(ArtifactStorageClass::Structured as i32)
                && req.created_via == Some(ArtifactCreatedVia::Agent as i32)
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
                storage_class: Some(ArtifactStorageClass::Structured),
                created_via: Some(ArtifactCreatedVia::Agent),
                metadata: vec![],
                payload: Some(serde_json::from_value(serde_json::json!({ "rows": [] })).unwrap()),
                links: vec![ArtifactLinkInput {
                    relation: ArtifactLinkRelation::AttachedTo as i32,
                    entity_type: ArtifactEntityType::Conversation as i32,
                    entity_id: "conv-1".into(),
                }],
            },
            None,
        )
        .await
        .expect("create");
    assert_eq!(artifact.artifact_id(), "art-1");
    assert_eq!(artifact.version().version, 1);
}
