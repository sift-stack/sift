use rmcp::{handler::server::wrapper::Parameters, model::ErrorCode};
use sift_rs::artifacts::v1::{
    Artifact, ArtifactAuthoringKind, CreateArtifactResponse, ListArtifactsResponse,
    artifact_service_server::ArtifactServiceServer,
};
use sift_test_util::{grpc::memory_sift_channel, mock::artifacts::v1::MockArtifactServiceImpl};
use tokio::task::JoinHandle;
use tonic::{Response, transport::Server};

use super::{CreateArtifactParams, GetArtifactParams};
use crate::{
    server::SiftMcpServer,
    tool::{artifacts::ArtifactListParams, common::test_support::structured_field},
};

fn sample_artifact() -> Artifact {
    Artifact {
        artifact_id: "art-1".into(),
        organization_id: "org-1".into(),
        artifact_version_id: "ver-1".into(),
        version: 1,
        title: Some("report".into()),
        authoring_kind: ArtifactAuthoringKind::Agent as i32,
        ..Default::default()
    }
}

async fn server_with_mock(
    mock: MockArtifactServiceImpl,
    allow_create: bool,
) -> (SiftMcpServer, JoinHandle<()>) {
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
        SiftMcpServer::new(
            channel,
            String::from("https://app.test.local"),
            allow_create,
            allow_create,
        ),
        handle,
    )
}

#[tokio::test]
async fn list_artifacts_returns_rows() {
    let mut mock = MockArtifactServiceImpl::new();
    mock.expect_list_artifacts().returning(|_| {
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
            limit: None,
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
            limit: None,
        }))
        .await
        .expect_err("empty conversation");
    assert_eq!(err.code, ErrorCode::INVALID_PARAMS);
}

#[tokio::test]
async fn get_artifact_rejects_empty_id() {
    let (server, _h) = server_with_mock(MockArtifactServiceImpl::new(), true).await;
    let err = server
        .get_artifact(Parameters(GetArtifactParams {
            artifact_id: String::new(),
            artifact_version_id: None,
        }))
        .await
        .expect_err("empty id");
    assert_eq!(err.code, ErrorCode::INVALID_PARAMS);
}

#[tokio::test]
async fn create_artifact_blocked_without_allow_create() {
    let (server, _h) = server_with_mock(MockArtifactServiceImpl::new(), false).await;
    let err = server
        .create_artifact(Parameters(CreateArtifactParams {
            title: None,
            summary: None,
            conversation_id: None,
            artifact_id: None,
            authoring_kind: None,
        }))
        .await
        .expect_err("gated");
    assert_eq!(err.code, ErrorCode::INVALID_REQUEST);
    assert!(err.message.contains("--allow-create"));
}

#[tokio::test]
async fn create_artifact_rejects_append_with_conversation() {
    let (server, _h) = server_with_mock(MockArtifactServiceImpl::new(), true).await;
    let err = server
        .create_artifact(Parameters(CreateArtifactParams {
            title: None,
            summary: None,
            conversation_id: Some("conv-1".into()),
            artifact_id: Some("art-1".into()),
            authoring_kind: None,
        }))
        .await
        .expect_err("illegal combo");
    assert_eq!(err.code, ErrorCode::INVALID_PARAMS);
}

#[tokio::test]
async fn create_artifact_happy_path() {
    let mut mock = MockArtifactServiceImpl::new();
    mock.expect_create_artifact().returning(|_| {
        Ok(Response::new(CreateArtifactResponse {
            artifact: Some(sample_artifact()),
        }))
    });

    let (server, _h) = server_with_mock(mock, true).await;
    let resp = server
        .create_artifact(Parameters(CreateArtifactParams {
            title: Some("report".into()),
            summary: None,
            conversation_id: Some("conv-1".into()),
            artifact_id: None,
            authoring_kind: Some("agent".into()),
        }))
        .await
        .expect("create");
    let artifact = structured_field(resp, "artifact");
    assert_eq!(artifact["artifactId"], "art-1");
}
