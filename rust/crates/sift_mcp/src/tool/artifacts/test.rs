use rmcp::{handler::server::wrapper::Parameters, model::ErrorCode};
use sift_rs::{
    artifacts::v1::{
        Artifact, ArtifactAuthoringKind, CreateArtifactResponse, GetArtifactResponse,
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
use tonic::{Response, Status, transport::Server};

use super::{CreateArtifactParams, DownloadArtifactParams};
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

fn get_returns(artifact: Artifact) -> MockArtifactServiceImpl {
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
    let uploaded = Artifact {
        remote_file_id: Some("rf-1".into()),
        ..sample_artifact()
    };
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
async fn get_artifact_surfaces_download_url_failure() {
    let uploaded = Artifact {
        remote_file_id: Some("rf-1".into()),
        ..sample_artifact()
    };
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
async fn create_artifact_blocked_without_allow_create() {
    let (server, _h) = server_with_mock(MockArtifactServiceImpl::new(), false).await;
    let err = server
        .create_artifact(Parameters(CreateArtifactParams {
            title: None,
            summary: None,
            conversation_id: None,
            artifact_id: None,
            authoring_kind: None,
            file_path: None,
        }))
        .await
        .expect_err("gated");
    assert_eq!(err.code, ErrorCode::INVALID_REQUEST);
    assert!(err.message.contains("--allow-create"));
}

#[tokio::test]
async fn create_artifact_append_blocked_without_allow_destructive() {
    let (server, _h) = server_with_mocks(
        MockArtifactServiceImpl::new(),
        MockRemoteFileServiceImpl::new(),
        true,
        false,
    )
    .await;
    let err = server
        .create_artifact(Parameters(CreateArtifactParams {
            title: Some("v2".into()),
            summary: None,
            conversation_id: None,
            artifact_id: Some("art-1".into()),
            authoring_kind: None,
            file_path: None,
        }))
        .await
        .expect_err("append gated");
    assert_eq!(err.code, ErrorCode::INVALID_REQUEST);
    assert!(err.message.contains("--allow-destructive"));
}

#[tokio::test]
async fn create_artifact_append_reports_appended_version() {
    let mut mock = MockArtifactServiceImpl::new();
    mock.expect_create_artifact()
        .withf(|req| {
            let req = req.get_ref();
            req.artifact_id.as_deref() == Some("art-1") && req.conversation_id.is_none()
        })
        .returning(|_| {
            Ok(Response::new(CreateArtifactResponse {
                artifact: Some(Artifact {
                    artifact_version_id: "ver-2".into(),
                    version: 2,
                    ..sample_artifact()
                }),
            }))
        });

    let (server, _h) = server_with_mock(mock, true).await;
    let resp = server
        .create_artifact(Parameters(CreateArtifactParams {
            title: Some("v2".into()),
            summary: None,
            conversation_id: None,
            artifact_id: Some("art-1".into()),
            authoring_kind: None,
            file_path: None,
        }))
        .await
        .expect("append");
    let next_step = structured_field(resp, "next_step");
    let next_step = next_step.as_str().unwrap();
    assert!(
        next_step.starts_with("Appended version 2 to artifact art-1"),
        "{next_step}"
    );
    assert!(!next_step.contains("Created"), "{next_step}");
}

#[tokio::test]
async fn create_artifact_accepts_authoring_kind_in_any_case() {
    for (input, expected) in [
        ("Agent", ArtifactAuthoringKind::Agent),
        ("AGENT", ArtifactAuthoringKind::Agent),
        (
            "ARTIFACT_AUTHORING_KIND_AGENT",
            ArtifactAuthoringKind::Agent,
        ),
        ("User", ArtifactAuthoringKind::User),
        ("artifact_authoring_kind_user", ArtifactAuthoringKind::User),
    ] {
        let mut mock = MockArtifactServiceImpl::new();
        mock.expect_create_artifact()
            .withf(move |req| req.get_ref().authoring_kind == Some(expected as i32))
            .returning(|_| {
                Ok(Response::new(CreateArtifactResponse {
                    artifact: Some(sample_artifact()),
                }))
            });
        let (server, _h) = server_with_mock(mock, true).await;
        server
            .create_artifact(Parameters(CreateArtifactParams {
                title: None,
                summary: None,
                conversation_id: None,
                artifact_id: None,
                authoring_kind: Some(input.into()),
                file_path: None,
            }))
            .await
            .unwrap_or_else(|err| panic!("{input}: {err:?}"));
    }
}

#[tokio::test]
async fn create_artifact_rejects_unknown_authoring_kind() {
    let (server, _h) = server_with_mock(MockArtifactServiceImpl::new(), true).await;
    let err = server
        .create_artifact(Parameters(CreateArtifactParams {
            title: None,
            summary: None,
            conversation_id: None,
            artifact_id: None,
            authoring_kind: Some("robot".into()),
            file_path: None,
        }))
        .await
        .expect_err("unknown kind");
    assert_eq!(err.code, ErrorCode::INVALID_PARAMS);
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
            file_path: None,
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
            file_path: None,
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
        let mut uploaded = sample_artifact();
        uploaded.remote_file_id = Some("rf-1".into());
        uploaded.file_name = Some("report.md".into());
        Ok(Response::new(GetArtifactResponse {
            artifact: Some(uploaded),
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
            artifact_id: None,
            authoring_kind: Some("agent".into()),
            file_path: Some(path.to_string_lossy().into_owned()),
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
            artifact_id: None,
            authoring_kind: None,
            file_path: Some("   ".into()),
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
            artifact_id: None,
            authoring_kind: Some("agent".into()),
            file_path: Some(path.to_string_lossy().into_owned()),
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
            artifact_id: None,
            authoring_kind: Some("agent".into()),
            file_path: Some(path.to_string_lossy().into_owned()),
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
