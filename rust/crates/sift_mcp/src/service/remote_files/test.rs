use std::io::Write;

use sift_rs::remote_files::v1::{
    GetRemoteFileDownloadUrlResponse, ListRemoteFilesResponse, RemoteFile,
    remote_file_service_server::RemoteFileServiceServer,
};
use sift_test_util::{
    grpc::memory_sift_channel, mock::remote_files::v1::MockRemoteFileServiceImpl,
};
use tempdir::TempDir;
use tokio::task::JoinHandle;
use tonic::{Code, Response, Status, transport::Server};

use super::{
    AttachmentEntityType, RemoteFileService, RemoteFileUploader, RestConfig, attachment_filter,
    mime_type_for,
};
use crate::client_event::start_http_server;
use crate::policy::RetryPolicy;

async fn service_with_mock(mock: MockRemoteFileServiceImpl) -> (RemoteFileService, JoinHandle<()>) {
    let (client, server) = tokio::io::duplex(1024);
    let channel = memory_sift_channel(client).await;

    let handle = tokio::spawn(async move {
        Server::builder()
            .add_service(RemoteFileServiceServer::new(mock))
            .serve_with_incoming(tokio_stream::once(Ok::<_, std::io::Error>(server)))
            .await
            .unwrap();
    });

    (
        RemoteFileService::new(channel, RetryPolicy::default()),
        handle,
    )
}

fn remote_file(remote_file_id: &str) -> RemoteFile {
    RemoteFile {
        remote_file_id: remote_file_id.into(),
        entity_id: "run-1".into(),
        file_name: format!("{remote_file_id}.csv"),
        ..Default::default()
    }
}

fn ids(page: &crate::service::common::Page<RemoteFile>) -> Vec<&str> {
    page.items
        .iter()
        .map(|file| file.remote_file_id.as_str())
        .collect()
}

const ATTACHMENT_TYPES: &str = "(entity_type == \"runs\" || entity_type == \"assets\" || \
     entity_type == \"annotations\" || entity_type == \"annotation_logs\" || \
     entity_type == \"test_reports\" || entity_type == \"test_steps\")";

#[test]
fn unscoped_filter_holds_the_listing_to_attachment_types() {
    assert_eq!(attachment_filter(&[], None, ""), ATTACHMENT_TYPES);
    assert_eq!(attachment_filter(&[], None, "   "), ATTACHMENT_TYPES);
}

#[test]
fn filter_ands_ids_type_and_the_callers_expression() {
    let ids = vec!["run-1".to_string(), " run-2 ".to_string()];
    assert_eq!(
        attachment_filter(
            &ids,
            Some(AttachmentEntityType::Run),
            "file_name.matches(\"(?i)log\") || file_name == \"a.csv\""
        ),
        "entity_id in [\"run-1\", \"run-2\"] && entity_type == \"runs\" && \
         (file_name.matches(\"(?i)log\") || file_name == \"a.csv\")"
    );
}

#[test]
fn filter_escapes_quotes_in_ids() {
    let ids = vec!["a\"b".to_string()];
    assert_eq!(
        attachment_filter(&ids, Some(AttachmentEntityType::AnnotationLog), ""),
        "entity_id in [\"a\\\"b\"] && entity_type == \"annotation_logs\""
    );
}

#[test]
fn every_attachment_type_has_a_distinct_name_and_filter_value() {
    let names: std::collections::HashSet<_> = AttachmentEntityType::ALL
        .map(AttachmentEntityType::name)
        .into();
    let values: std::collections::HashSet<_> = AttachmentEntityType::ALL
        .map(AttachmentEntityType::filter_value)
        .into();
    assert_eq!(names.len(), AttachmentEntityType::ALL.len());
    assert_eq!(values.len(), AttachmentEntityType::ALL.len());
}

#[tokio::test]
async fn list_attachments_sends_the_scoped_filter_and_limit() {
    let mut mock = MockRemoteFileServiceImpl::new();
    mock.expect_list_remote_files()
        .withf(|req| {
            let req = req.get_ref();
            req.filter == "entity_id in [\"run-1\"] && entity_type == \"runs\""
                && req.page_size == 50
                && req.page_token.is_empty()
        })
        .times(1)
        .returning(|_| {
            Ok(Response::new(ListRemoteFilesResponse {
                remote_files: vec![remote_file("rf-1")],
                next_page_token: String::new(),
            }))
        });

    let (service, _h) = service_with_mock(mock).await;
    let page = service
        .list_attachments(
            vec!["run-1".into()],
            Some(AttachmentEntityType::Run),
            String::new(),
            None,
        )
        .await
        .expect("list");
    assert_eq!(ids(&page), ["rf-1"]);
    assert!(!page.has_more);
}

#[tokio::test]
async fn list_attachments_reads_has_more_from_the_token_without_following_it() {
    let mut mock = MockRemoteFileServiceImpl::new();
    mock.expect_list_remote_files()
        .withf(|req| req.get_ref().page_size == 5)
        .times(1)
        .returning(|_| {
            Ok(Response::new(ListRemoteFilesResponse {
                remote_files: vec![remote_file("rf-1"), remote_file("rf-2")],
                next_page_token: "next".into(),
            }))
        });

    let (service, _h) = service_with_mock(mock).await;
    let page = service
        .list_attachments(Vec::new(), None, String::new(), Some(5))
        .await
        .expect("list");
    assert_eq!(ids(&page), ["rf-1", "rf-2"]);
    assert!(page.has_more);
}

#[tokio::test]
async fn list_attachments_reports_more_behind_an_empty_page() {
    // The backend drops files the caller can't view after paging, so a page
    // can be empty and still carry a token.
    let mut mock = MockRemoteFileServiceImpl::new();
    mock.expect_list_remote_files().times(1).returning(|_| {
        Ok(Response::new(ListRemoteFilesResponse {
            remote_files: Vec::new(),
            next_page_token: "next".into(),
        }))
    });

    let (service, _h) = service_with_mock(mock).await;
    let page = service
        .list_attachments(Vec::new(), None, String::new(), None)
        .await
        .expect("list");
    assert!(page.items.is_empty());
    assert!(page.has_more);
}

#[tokio::test]
async fn list_attachments_truncates_to_limit() {
    let mut mock = MockRemoteFileServiceImpl::new();
    mock.expect_list_remote_files().returning(|_| {
        Ok(Response::new(ListRemoteFilesResponse {
            remote_files: vec![
                remote_file("rf-1"),
                remote_file("rf-2"),
                remote_file("rf-3"),
            ],
            next_page_token: String::new(),
        }))
    });

    let (service, _h) = service_with_mock(mock).await;
    let page = service
        .list_attachments(Vec::new(), None, String::new(), Some(2))
        .await
        .expect("list");
    assert_eq!(ids(&page), ["rf-1", "rf-2"]);
    assert!(page.has_more);
}

#[tokio::test]
async fn list_attachments_propagates_invalid_argument() {
    let mut mock = MockRemoteFileServiceImpl::new();
    mock.expect_list_remote_files()
        .returning(|_| Err(Status::invalid_argument("bad filter")));

    let (service, _h) = service_with_mock(mock).await;
    let err = service
        .list_attachments(Vec::new(), None, "file_name ==".into(), None)
        .await
        .expect_err("expected error");
    let status = err.downcast_ref::<tonic::Status>().expect("status");
    assert_eq!(status.code(), Code::InvalidArgument);
}

#[tokio::test]
async fn download_url_returns_the_signed_url() {
    let mut mock = MockRemoteFileServiceImpl::new();
    mock.expect_get_remote_file_download_url()
        .withf(|req| req.get_ref().remote_file_id == "rf-1")
        .times(1)
        .returning(|_| {
            Ok(Response::new(GetRemoteFileDownloadUrlResponse {
                download_url: "https://files.test.local/rf-1?sig=abc".into(),
            }))
        });

    let (service, _h) = service_with_mock(mock).await;
    let url = service.download_url("rf-1".into()).await.expect("url");
    assert_eq!(url, "https://files.test.local/rf-1?sig=abc");
}

#[tokio::test]
async fn download_url_rejects_an_empty_url() {
    let mut mock = MockRemoteFileServiceImpl::new();
    mock.expect_get_remote_file_download_url().returning(|_| {
        Ok(Response::new(GetRemoteFileDownloadUrlResponse {
            download_url: String::new(),
        }))
    });

    let (service, _h) = service_with_mock(mock).await;
    let err = service
        .download_url("rf-1".into())
        .await
        .expect_err("empty url is an error");
    assert!(err.to_string().contains("download url response was empty"));
}

#[tokio::test]
async fn download_url_propagates_not_found() {
    let mut mock = MockRemoteFileServiceImpl::new();
    mock.expect_get_remote_file_download_url()
        .returning(|_| Err(Status::not_found("remote file not found")));

    let (service, _h) = service_with_mock(mock).await;
    let err = service
        .download_url("missing".into())
        .await
        .expect_err("expected error");
    let status = err.downcast_ref::<tonic::Status>().expect("status");
    assert_eq!(status.code(), Code::NotFound);
}

fn write_file(dir: &TempDir, name: &str, contents: &[u8]) -> std::path::PathBuf {
    let path = dir.path().join(name);
    let mut file = std::fs::File::create(&path).unwrap();
    file.write_all(contents).unwrap();
    path
}

#[tokio::test]
async fn uploads_the_file_as_one_multipart_request() {
    let (rest_uri, server) = start_http_server(
        b"HTTP/1.1 200 OK\r\ncontent-type: application/json\r\ncontent-length: 2\r\nconnection: close\r\n\r\n{}"
            .to_vec(),
    )
    .await;
    let dir = TempDir::new("artifact-upload").unwrap();
    let path = write_file(&dir, "report.md", b"# Battery Report\n");

    let uploader = RemoteFileUploader::new(RestConfig::new(rest_uri, "test-key".into()), "1.2.3");
    uploader
        .upload_artifact_version_file("org-1", "ver-1", &path)
        .await
        .unwrap();

    let request = String::from_utf8(server.await.unwrap()).unwrap();
    let (headers, body) = request.split_once("\r\n\r\n").unwrap();

    assert!(headers.starts_with("POST /api/v0/remote-files/upload HTTP/1.1"));
    assert!(
        headers
            .lines()
            .any(|line| line.eq_ignore_ascii_case("authorization: Bearer test-key"))
    );
    assert!(
        headers
            .lines()
            .any(|line| line.eq_ignore_ascii_case("user-agent: sift_mcp/1.2.3"))
    );

    // Multipart form fields the upload handler parses.
    assert!(body.contains("name=\"organizationId\""));
    assert!(body.contains("org-1"));
    assert!(body.contains("name=\"entityId\""));
    assert!(body.contains("ver-1"));
    assert!(body.contains("name=\"entityType\""));
    assert!(body.contains("artifact_versions"));
    assert!(body.contains("name=\"file\"; filename=\"report.md\""));
    // The server's own extension table does not know `.md`, so the part must
    // declare the type itself.
    assert!(
        body.to_ascii_lowercase()
            .contains("content-type: text/markdown"),
        "{body}"
    );
    assert!(body.contains("# Battery Report"));
}

#[tokio::test]
async fn a_chat_upload_identifies_as_chat() {
    let (rest_uri, server) = start_http_server(
        b"HTTP/1.1 200 OK\r\ncontent-type: application/json\r\ncontent-length: 2\r\nconnection: close\r\n\r\n{}"
            .to_vec(),
    )
    .await;
    let dir = TempDir::new("artifact-upload").unwrap();
    let path = write_file(&dir, "report.md", b"# Battery Report\n");

    let config = RestConfig {
        client_name: crate::ClientName::Chat,
        ..RestConfig::new(rest_uri, "test-key".into())
    };
    RemoteFileUploader::new(config, "1.2.3")
        .upload_artifact_version_file("org-1", "ver-1", &path)
        .await
        .unwrap();

    let request = String::from_utf8(server.await.unwrap()).unwrap();
    let (headers, _) = request.split_once("\r\n\r\n").unwrap();
    assert!(
        headers
            .lines()
            .any(|line| line.eq_ignore_ascii_case("user-agent: chat/1.2.3"))
    );
}

#[tokio::test]
async fn a_failed_upload_reports_the_status_and_detail() {
    let (rest_uri, server) = start_http_server(
        b"HTTP/1.1 413 Payload Too Large\r\ncontent-type: application/json\r\ncontent-length: 24\r\nconnection: close\r\n\r\n{\"error\":\"file too big\"}"
            .to_vec(),
    )
    .await;
    let dir = TempDir::new("artifact-upload").unwrap();
    let path = write_file(&dir, "export.csv", b"a,b\n1,2\n");

    let uploader = RemoteFileUploader::new(RestConfig::new(rest_uri, "test-key".into()), "1.2.3");
    let error = uploader
        .upload_artifact_version_file("org-1", "ver-1", &path)
        .await
        .unwrap_err();
    server.await.unwrap();

    let message = format!("{error:#}");
    assert!(message.contains("413"), "{message}");
    assert!(message.contains("file too big"), "{message}");
}

#[tokio::test]
async fn rejects_missing_empty_and_directory_paths() {
    let dir = TempDir::new("artifact-upload").unwrap();
    let uploader = RemoteFileUploader::new(
        RestConfig::new("http://unused.test.local".into(), "test-key".into()),
        "1.2.3",
    );

    let missing = dir.path().join("nope.md");
    let error = uploader
        .upload_artifact_version_file("org-1", "ver-1", &missing)
        .await
        .unwrap_err();
    assert!(format!("{error:#}").contains("does not exist"));

    let empty = write_file(&dir, "empty.md", b"");
    let error = uploader
        .upload_artifact_version_file("org-1", "ver-1", &empty)
        .await
        .unwrap_err();
    assert!(format!("{error:#}").contains("is empty"));

    let error = uploader
        .upload_artifact_version_file("org-1", "ver-1", dir.path())
        .await
        .unwrap_err();
    assert!(format!("{error:#}").contains("not a regular file"));
}

#[test]
fn mime_type_comes_from_the_extension_with_a_binary_fallback() {
    assert_eq!(mime_type_for("report.md"), "text/markdown");
    assert_eq!(mime_type_for("export.csv"), "text/csv");
    assert_eq!(mime_type_for("plot.png"), "image/png");
    assert_eq!(mime_type_for("analysis.py"), "text/plain");
    assert_eq!(
        mime_type_for("data.parquet"),
        "application/vnd.apache.parquet"
    );
    assert_eq!(mime_type_for("frames.sift"), "application/octet-stream");
    assert_eq!(mime_type_for("no_extension"), "application/octet-stream");
}
