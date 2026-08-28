use sift_rs::artifacts::v1::{
    Artifact, ArtifactAuthoringKind, CreateArtifactResponse, GetArtifactResponse,
    ListArtifactsResponse, artifact_service_server::ArtifactServiceServer,
};
use sift_test_util::{grpc::memory_sift_channel, mock::artifacts::v1::MockArtifactServiceImpl};
use tokio::task::JoinHandle;
use tonic::{Code, Response, Status, transport::Server};

use super::{ArtifactService, AuthoringKind};
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
    let (client, server) = tokio::io::duplex(1024);
    let channel = memory_sift_channel(client).await;

    let handle = tokio::spawn(async move {
        Server::builder()
            .add_service(ArtifactServiceServer::new(mock))
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
        .withf(|req| req.get_ref().conversation_id.as_deref() == Some("conv-1"))
        .returning(|_| {
            Ok(Response::new(ListArtifactsResponse {
                artifacts: vec![sample_artifact()],
                next_page_token: String::new(),
            }))
        });

    let (service, _h) = service_with_mock(mock).await;
    let page = service
        .list_artifacts(Some("conv-1".into()), false, None)
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
        .list_artifacts(None, false, Some(200))
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
        .list_artifacts(None, false, Some(2))
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
        .list_artifacts(Some("missing".into()), false, None)
        .await
        .expect_err("expected error");
    let status = err.downcast_ref::<tonic::Status>().expect("status");
    assert_eq!(status.code(), Code::NotFound);
}

#[tokio::test]
async fn get_artifact_returns_latest() {
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
        .get_artifact("art-1".into(), None)
        .await
        .expect("get");
    assert_eq!(artifact.inner.artifact_id, "art-1");
    assert_eq!(artifact.inner.version, 1);
    assert!(artifact.download_url.is_none());
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
        })
        .returning(|_| {
            Ok(Response::new(CreateArtifactResponse {
                artifact: Some(sample_artifact()),
            }))
        });

    let (service, _h) = service_with_mock(mock).await;
    let artifact = service
        .create_artifact(
            Some("report".into()),
            Some("summary".into()),
            Some("conv-1".into()),
            None,
            AuthoringKind::Agent,
        )
        .await
        .expect("create");
    assert_eq!(artifact.inner.artifact_id, "art-1");
    assert_eq!(artifact.inner.version, 1);
}
