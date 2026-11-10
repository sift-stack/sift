use std::{
    path::{Path, PathBuf},
    time::{Duration, SystemTime},
};

use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpListener,
    task::JoinHandle,
};

use super::{DeclarativeService, HttpError, SCHEMA_FILENAME, SCHEMA_PATH};
use crate::service::remote_files::RestConfig;

/// Deliberately odd spacing: the cache must keep the bytes exactly as served.
const SCHEMA: &str = "{ \"$id\" : \"a\",\n  \"$defs\" : { \"chart\" : {} } }\n";

pub(crate) fn http_response(status: &str, body: &str) -> Vec<u8> {
    format!(
        "HTTP/1.1 {status}\r\ncontent-type: application/json\r\ncontent-length: {}\r\nconnection: close\r\n\r\n{body}",
        body.len()
    )
    .into_bytes()
}

/// Serves one canned response per incoming connection, in order, then returns every request it read.
/// `start_http_server` handles one connection, which cannot show a second download or its absence.
pub(crate) async fn start_sequence_server(
    responses: Vec<Vec<u8>>,
) -> (String, JoinHandle<Vec<Vec<u8>>>) {
    fn request_complete(request: &[u8]) -> bool {
        let Some(header_end) = request.windows(4).position(|window| window == b"\r\n\r\n") else {
            return false;
        };
        let headers = String::from_utf8_lossy(&request[..header_end]);
        let content_length = headers.lines().find_map(|line| {
            let (name, value) = line.split_once(':')?;
            name.eq_ignore_ascii_case("content-length")
                .then(|| value.trim().parse::<usize>().ok())
                .flatten()
        });
        content_length.is_none_or(|length| request.len() >= header_end + 4 + length)
    }

    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = listener.local_addr().unwrap();
    let server = tokio::spawn(async move {
        let mut requests = Vec::new();
        for response in responses {
            let (mut stream, _) = listener.accept().await.unwrap();
            let mut request = Vec::new();
            while !request_complete(&request) {
                let mut buffer = [0; 1024];
                let count = stream.read(&mut buffer).await.unwrap();
                assert!(
                    count > 0,
                    "the client closed before the request was complete"
                );
                request.extend_from_slice(&buffer[..count]);
            }
            requests.push(request);
            stream.write_all(&response).await.unwrap();
        }
        requests
    });
    (format!("http://{address}"), server)
}

fn ok(body: &str) -> Vec<u8> {
    http_response("200 OK", body)
}

pub(crate) fn service(rest_uri: &str, cache_root: &Path) -> DeclarativeService {
    DeclarativeService::new(
        Some(RestConfig::new(rest_uri.into(), "test-key".into())),
        "1.2.3",
    )
    .with_cache_root(cache_root)
}

fn cache_root() -> tempdir::TempDir {
    tempdir::TempDir::new("sift-schema-cache").unwrap()
}

fn request_text(request: &[u8]) -> (String, String) {
    let text = String::from_utf8(request.to_vec()).unwrap();
    let (headers, body) = text.split_once("\r\n\r\n").unwrap();
    (headers.to_owned(), body.to_owned())
}

fn assert_header(headers: &str, expected: &str) {
    assert!(
        headers
            .lines()
            .any(|line| line.eq_ignore_ascii_case(expected)),
        "missing `{expected}` in:\n{headers}"
    );
}

fn set_age(path: &Path, age: Duration) {
    let modified = SystemTime::now() - age;
    std::fs::File::options()
        .write(true)
        .open(path)
        .unwrap()
        .set_modified(modified)
        .unwrap();
}

fn http_error(error: &anyhow::Error) -> &HttpError {
    error
        .downcast_ref::<HttpError>()
        .expect("a typed HTTP error")
}

#[tokio::test]
async fn schema_bytes_are_downloaded_fresh_with_auth_and_kept_exactly() {
    let (uri, server) = start_sequence_server(vec![ok(SCHEMA)]).await;
    let bytes = DeclarativeService::new(
        Some(RestConfig::new(format!("{uri}/"), "test-key".into())),
        "1.2.3",
    )
    .chart_schema_bytes()
    .await
    .unwrap();

    assert_eq!(bytes, SCHEMA.as_bytes());
    let requests = server.await.unwrap();
    let (headers, _) = request_text(&requests[0]);
    assert!(
        headers.starts_with(&format!("GET {SCHEMA_PATH} HTTP/1.1")),
        "{headers}"
    );
    assert_header(&headers, "authorization: Bearer test-key");
    assert_header(&headers, "user-agent: sift_mcp/1.2.3");
}

#[tokio::test]
async fn schema_bytes_reject_bad_responses_and_missing_rest_endpoint() {
    for body in ["not json", "[]", "{}", r#"{"$defs":[]}"#] {
        let (uri, _server) = start_sequence_server(vec![ok(body)]).await;
        let root = cache_root();
        let error = service(&uri, root.path())
            .chart_schema_bytes()
            .await
            .unwrap_err();
        assert!(
            error.downcast_ref::<HttpError>().is_none(),
            "{body}: {error:#}"
        );
    }

    let (uri, _server) = start_sequence_server(vec![http_response("404 Not Found", "")]).await;
    let root = cache_root();
    let error = service(&uri, root.path())
        .chart_schema_bytes()
        .await
        .unwrap_err();
    assert_eq!(http_error(&error).status.as_u16(), 404);

    let error = DeclarativeService::new(None, "1.2.3")
        .chart_schema_bytes()
        .await
        .unwrap_err();
    assert!(format!("{error:#}").contains("REST endpoint"), "{error:#}");
}

#[tokio::test]
async fn schema_path_downloads_once_and_is_reused_by_other_processes() {
    let (uri, server) = start_sequence_server(vec![ok(SCHEMA)]).await;
    let root = cache_root();

    let path = service(&uri, root.path())
        .chart_schema_path()
        .await
        .unwrap();
    assert!(path.is_absolute());
    assert!(path.starts_with(root.path()));
    assert_eq!(path.file_name().unwrap(), SCHEMA_FILENAME);
    assert_eq!(std::fs::read(&path).unwrap(), SCHEMA.as_bytes());
    assert_eq!(
        std::fs::read_dir(path.parent().unwrap()).unwrap().count(),
        1,
        "no temp files"
    );
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mode = |p: &Path| std::fs::metadata(p).unwrap().permissions().mode() & 0o777;
        assert_eq!(mode(&path), 0o600);
        assert_eq!(mode(path.parent().unwrap()), 0o700);
    }

    // The server has answered its only request, so another service (a second MCP process)
    // can only succeed from the file.
    let again = service(&uri, root.path())
        .chart_schema_path()
        .await
        .unwrap();
    assert_eq!(again, path);
    assert_eq!(server.await.unwrap().len(), 1);
}

#[tokio::test]
async fn schema_path_refreshes_stale_future_corrupt_and_deleted_files() {
    let bodies = [
        "{\"$defs\":{\"v\":1}}",
        "{\"$defs\":{\"v\":2}}",
        "{\"$defs\":{\"v\":3}}",
        "{\"$defs\":{\"v\":4}}",
        "{\"$defs\":{\"v\":5}}",
    ];
    let (uri, server) = start_sequence_server(bodies.iter().map(|body| ok(body)).collect()).await;
    let root = cache_root();
    let svc = service(&uri, root.path());
    let hour = Duration::from_secs(3600);

    let path = svc.chart_schema_path().await.unwrap();
    assert_eq!(std::fs::read_to_string(&path).unwrap(), bodies[0]);

    set_age(&path, 23 * hour);
    svc.chart_schema_path().await.unwrap();
    assert_eq!(
        std::fs::read_to_string(&path).unwrap(),
        bodies[0],
        "still fresh"
    );

    set_age(&path, 25 * hour);
    svc.chart_schema_path().await.unwrap();
    assert_eq!(std::fs::read_to_string(&path).unwrap(), bodies[1], "stale");

    let future = SystemTime::now() + hour;
    std::fs::File::options()
        .write(true)
        .open(&path)
        .unwrap()
        .set_modified(future)
        .unwrap();
    svc.chart_schema_path().await.unwrap();
    assert_eq!(
        std::fs::read_to_string(&path).unwrap(),
        bodies[2],
        "mtime in the future"
    );

    std::fs::write(&path, "{ corrupt").unwrap();
    svc.chart_schema_path().await.unwrap();
    assert_eq!(
        std::fs::read_to_string(&path).unwrap(),
        bodies[3],
        "corrupt"
    );

    std::fs::remove_file(&path).unwrap();
    svc.chart_schema_path().await.unwrap();
    assert_eq!(
        std::fs::read_to_string(&path).unwrap(),
        bodies[4],
        "deleted"
    );
    assert_eq!(server.await.unwrap().len(), 5);
}

#[tokio::test]
async fn failed_refresh_keeps_the_previous_file() {
    let (uri, _server) = start_sequence_server(vec![
        ok(SCHEMA),
        http_response("500 Internal Server Error", "{}"),
        ok("{\"no_defs\":true}"),
    ])
    .await;
    let root = cache_root();
    let svc = service(&uri, root.path());
    let path = svc.chart_schema_path().await.unwrap();
    set_age(&path, Duration::from_secs(25 * 3600));

    let error = svc.chart_schema_path().await.unwrap_err();
    assert_eq!(http_error(&error).status.as_u16(), 500);
    svc.chart_schema_path().await.unwrap_err();

    assert_eq!(std::fs::read(&path).unwrap(), SCHEMA.as_bytes());
    assert_eq!(
        std::fs::read_dir(path.parent().unwrap()).unwrap().count(),
        1,
        "no temp files"
    );
}

#[tokio::test]
async fn cache_paths_isolate_deployments_and_roots_without_leaking_credentials() {
    let (root_a, root_b) = (cache_root(), cache_root());
    let path =
        |uri: &str, root: &Path| -> PathBuf { service(uri, root).schema_cache_path().unwrap() };

    let base = path("https://a.example.test", root_a.path());
    assert_eq!(base, path("https://a.example.test/", root_a.path()));
    assert_ne!(base, path("https://b.example.test", root_a.path()));
    let other_root = path("https://a.example.test", root_b.path());
    assert_ne!(base, other_root);
    assert!(other_root.starts_with(root_b.path()));
    assert!(!base.to_string_lossy().contains("test-key"));

    let no_root = DeclarativeService::new(
        Some(RestConfig::new("https://a.example.test".into(), "k".into())),
        "1.2.3",
    );
    assert!(no_root.chart_schema_path().await.is_err());
    let no_rest = DeclarativeService::new(None, "1.2.3").with_cache_root(root_a.path());
    assert!(no_rest.chart_schema_path().await.is_err());
}

#[tokio::test]
async fn concurrent_callers_in_one_process_share_one_download() {
    let (uri, server) = start_sequence_server(vec![ok(SCHEMA)]).await;
    let root = cache_root();
    let svc = service(&uri, root.path());

    let calls = (0..8).map(|_| {
        let svc = svc.clone();
        async move { svc.chart_schema_path().await }
    });
    let paths = futures::future::join_all(calls).await;

    let first = paths[0].as_ref().unwrap();
    assert!(paths.iter().all(|path| path.as_ref().unwrap() == first));
    assert_eq!(server.await.unwrap().len(), 1);
}

#[tokio::test]
async fn share_link_posts_the_exact_spec_and_returns_the_short_link() {
    let (uri, server) = start_sequence_server(vec![ok(r#"{"shortLink":"abc 123"}"#)]).await;
    let root = cache_root();
    let spec = "charts:\n  - name: \"a\"\n";

    let link = service(&uri, root.path())
        .create_share_link(spec)
        .await
        .unwrap();

    assert_eq!(link, "abc 123");
    let (headers, body) = request_text(&server.await.unwrap()[0]);
    assert!(
        headers.starts_with("POST /api/v1/declarative:sharelink HTTP/1.1"),
        "{headers}"
    );
    assert_header(&headers, "authorization: Bearer test-key");
    assert_header(&headers, "user-agent: sift_mcp/1.2.3");
    assert_eq!(
        serde_json::from_str::<serde_json::Value>(&body).unwrap(),
        serde_json::json!({ "spec": spec })
    );
}

#[tokio::test]
async fn share_link_failures_are_typed_and_never_return_a_link() {
    let long_body = "x".repeat(2_000);
    let cases = [
        (
            http_response(
                "400 Bad Request",
                r#"{"code":3,"message":"charts[0]: bad"}"#,
            ),
            400,
            Some(3),
            "charts[0]: bad",
        ),
        (
            http_response("403 Forbidden", "denied"),
            403,
            None,
            "denied",
        ),
        (
            http_response("503 Service Unavailable", &long_body),
            503,
            None,
            &long_body[..500],
        ),
    ];
    for (reply, status, grpc_code, message) in cases {
        let (uri, _server) = start_sequence_server(vec![reply]).await;
        let root = cache_root();
        let error = service(&uri, root.path())
            .create_share_link("spec")
            .await
            .unwrap_err();
        let http = http_error(&error);
        assert_eq!(
            (http.status.as_u16(), http.grpc_code, http.message.as_str()),
            (status, grpc_code, message)
        );
    }

    for body in ["{}", r#"{"shortLink":""}"#, "not json"] {
        let (uri, _server) = start_sequence_server(vec![ok(body)]).await;
        let root = cache_root();
        let error = service(&uri, root.path())
            .create_share_link("spec")
            .await
            .unwrap_err();
        assert!(error.downcast_ref::<HttpError>().is_none(), "{body}");
    }

    let error = DeclarativeService::new(None, "1.2.3")
        .create_share_link("spec")
        .await
        .unwrap_err();
    assert!(format!("{error:#}").contains("REST endpoint"), "{error:#}");
}
