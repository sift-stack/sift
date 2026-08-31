use std::io::Write;

use tempdir::TempDir;

use super::{RemoteFileUploader, RestConfig};
use crate::client_event::start_http_server;

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
    assert!(body.contains("# Battery Report"));
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
