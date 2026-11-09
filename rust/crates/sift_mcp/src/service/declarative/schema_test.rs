use std::time::Duration;

use super::DeclarativeService;
use super::test::{
    Reply, chart_schema_fixture, chart_schema_responses, http_response, service,
    start_sequence_server,
};

#[tokio::test]
async fn chart_schema_uses_get_auth_retries_and_redirects() {
    let mut responses = chart_schema_responses();
    responses.insert(0, http_response("503 Service Unavailable", "{}"));
    responses.insert(2, http_response("429 Too Many Requests", "{}"));
    responses.insert(
        3,
        b"HTTP/1.1 302 Found\r\nlocation: /schema.json\r\ncontent-length: 0\r\nconnection: close\r\n\r\n"
            .to_vec(),
    );
    let (rest_uri, server) =
        start_sequence_server(responses.into_iter().map(Reply::Respond).collect()).await;

    let schema = service(format!("{rest_uri}/profile/"))
        .chart_schema()
        .await
        .unwrap();
    let requests = server.await.unwrap();

    assert_eq!(schema.schema.id, "https://example.test/chart.schema.json");
    assert_eq!(requests.len(), 5);
    assert_eq!(requests[0], requests[1]);
    assert_eq!(requests[2], requests[3]);
    let expected_paths = [
        "/profile/api/packages/v1/schemas/declarative-chart/versions",
        "/profile/api/packages/v1/schemas/declarative-chart/versions",
        "/profile/download/schemas/declarative-chart/0/sift.declarative.schema.v0.json",
        "/profile/download/schemas/declarative-chart/0/sift.declarative.schema.v0.json",
        "/schema.json",
    ];
    for (request, path) in requests.iter().zip(expected_paths) {
        let request = String::from_utf8_lossy(request);
        assert!(request.starts_with(&format!("GET {path} HTTP/1.1\r\n")));
        assert!(
            request
                .lines()
                .any(|line| line.eq_ignore_ascii_case("authorization: Bearer test-key"))
        );
        assert!(
            request
                .lines()
                .any(|line| line.eq_ignore_ascii_case("user-agent: sift_mcp/1.2.3"))
        );
        assert_eq!(request.split_once("\r\n\r\n").unwrap().1, "");
    }
}

#[tokio::test]
async fn chart_schema_reports_a_400_on_versions() {
    for body in [
        r#"{"error":"schema unavailable"}"#,
        "an arbitrary response body",
    ] {
        let (rest_uri, server) =
            start_sequence_server(vec![Reply::Respond(http_response("400 Bad Request", body))])
                .await;

        let error = service(rest_uri).chart_schema().await.unwrap_err();
        assert_eq!(
            error.to_string(),
            "This deployment does not serve the declarative chart schema."
        );
        assert_eq!(server.await.unwrap().len(), 1);
    }
}

#[tokio::test]
async fn chart_schema_reports_a_404_on_versions() {
    let (rest_uri, server) = start_sequence_server(vec![Reply::Respond(http_response(
        "404 Not Found",
        "an arbitrary response body",
    ))])
    .await;

    let error = service(rest_uri).chart_schema().await.unwrap_err();
    assert_eq!(
        error.to_string(),
        "This deployment does not serve the declarative chart schema."
    );
    assert_eq!(server.await.unwrap().len(), 1);
}

#[tokio::test]
async fn chart_schema_reports_a_missing_version_zero() {
    for versions in [r#"{"versions":[]}"#, r#"{"versions":[{"version":"1"}]}"#] {
        let (rest_uri, server) =
            start_sequence_server(vec![Reply::Respond(http_response("200 OK", versions))]).await;

        let error = service(rest_uri).chart_schema().await.unwrap_err();
        assert_eq!(
            error.to_string(),
            "This deployment does not serve the declarative chart schema."
        );
        assert_eq!(server.await.unwrap().len(), 1);
    }
}

fn schema_replies(body: &str) -> Vec<Reply> {
    let mut responses = chart_schema_responses();
    responses[1] = http_response("200 OK", body);
    responses.into_iter().map(Reply::Respond).collect()
}

fn fixture_body() -> String {
    chart_schema_fixture().to_string()
}

/// Deliberately not what `serde_json` would emit, so any re-serialization
/// shows up as a byte difference.
const ODD_SCHEMA_BODY: &str = "{\n  \"title\" :  \"Odd \\u00e9 bytes\",\n\t\"$defs\": { \"z\": {}, \"a\": {} },\n  \"type\":\"object\"  }\n";

fn set_age(path: &std::path::Path, age: Duration) {
    set_mtime(path, std::time::SystemTime::now() - age);
}

fn set_mtime(path: &std::path::Path, time: std::time::SystemTime) {
    std::fs::OpenOptions::new()
        .write(true)
        .open(path)
        .unwrap()
        .set_modified(time)
        .unwrap();
}

#[tokio::test]
async fn chart_schema_downloads_once_per_instance_and_clones_share_it() {
    let (rest_uri, server) = start_sequence_server(schema_replies(&fixture_body())).await;
    let service = service(rest_uri);

    let first = service.chart_schema().await.unwrap();
    let from_clone = service.clone().chart_schema().await.unwrap();
    let again = service.chart_schema().await.unwrap();

    assert_eq!(first.path, from_clone.path);
    assert_eq!(first.path, again.path);
    assert_eq!(first.schema.title, "Declarative chart");
    assert_eq!(server.await.unwrap().len(), 2);
}

#[tokio::test]
async fn chart_schema_cold_concurrent_calls_download_once() {
    let (rest_uri, server) = start_sequence_server(schema_replies(&fixture_body())).await;
    let service = service(rest_uri);

    let mut tasks = tokio::task::JoinSet::new();
    for _ in 0..16 {
        let service = service.clone();
        tasks.spawn(async move { service.chart_schema().await.unwrap().path.clone() });
    }
    while let Some(path) = tasks.join_next().await {
        assert_eq!(path.unwrap(), service.chart_schema().await.unwrap().path);
    }
    assert_eq!(server.await.unwrap().len(), 2);
}

#[tokio::test]
async fn chart_schema_concurrent_services_share_one_startup_download() {
    let (rest_uri, server) = start_sequence_server(schema_replies(&fixture_body())).await;
    let first = service(rest_uri.clone());
    let others: Vec<_> = (0..7)
        .map(|_| service(rest_uri.clone()).with_shared_cache_root(&first))
        .collect();

    let mut tasks = tokio::task::JoinSet::new();
    for service in std::iter::once(first).chain(others) {
        tasks.spawn(async move { service.chart_schema().await.unwrap().path.clone() });
    }
    let mut paths = Vec::new();
    while let Some(path) = tasks.join_next().await {
        paths.push(path.unwrap());
    }

    assert!(paths.iter().all(|path| *path == paths[0]));
    assert_eq!(server.await.unwrap().len(), 2);
}

#[tokio::test]
async fn chart_schema_is_a_private_file_with_the_exact_downloaded_bytes() {
    let (rest_uri, server) = start_sequence_server(schema_replies(ODD_SCHEMA_BODY)).await;
    let service = service(rest_uri);

    let cached = service.chart_schema().await.unwrap();

    assert!(cached.path.is_absolute(), "{:?}", cached.path);
    assert_eq!(
        std::fs::read(&cached.path).unwrap(),
        ODD_SCHEMA_BODY.as_bytes()
    );
    assert_eq!(cached.schema.title, "Odd \u{e9} bytes");
    assert_eq!(cached.schema.root["type"], "object");
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mode =
            |path: &std::path::Path| std::fs::metadata(path).unwrap().permissions().mode() & 0o777;
        assert_eq!(mode(&cached.path), 0o600);
        assert_eq!(mode(cached.path.parent().unwrap()), 0o700);
    }
    assert_eq!(server.await.unwrap().len(), 2);
}

#[tokio::test]
async fn chart_schema_path_is_stable_keyless_and_survives_the_service() {
    let (rest_uri, server) = start_sequence_server(schema_replies(&fixture_body())).await;
    let first = service(rest_uri.clone());
    let path = first.chart_schema().await.unwrap().path.clone();
    server.await.unwrap();

    let text = path.to_string_lossy();
    assert!(text.contains("sift-cli/schemas") || text.contains("sift-cli\\schemas"));
    assert!(path.ends_with("sift.declarative.schema.v0.json"));
    assert!(!text.contains("test-key"));
    assert!(!std::fs::read_to_string(&path).unwrap().contains("test-key"));

    let second = service(rest_uri).with_shared_cache_root(&first);
    drop(first);
    // The mock is closed: only the persistent file can answer.
    assert_eq!(second.chart_schema().await.unwrap().path, path);
}

#[tokio::test]
async fn chart_schema_file_persists_after_the_service_drops_while_the_root_lives() {
    let (rest_uri, server) = start_sequence_server(schema_replies(&fixture_body())).await;
    let owner = service("http://unused.test.local".into());
    let first = service(rest_uri).with_shared_cache_root(&owner);
    let path = first.chart_schema().await.unwrap().path.clone();
    server.await.unwrap();
    drop(first);
    assert!(path.is_file());
}

#[tokio::test]
async fn chart_schema_fresh_file_is_reused_by_a_new_instance_without_http() {
    let uri = "http://fresh.test.local".to_owned();
    let first = service(uri.clone());
    let file = seed(&first, ODD_SCHEMA_BODY.as_bytes());
    set_age(&file, Duration::from_secs(24 * 60 * 60 - 60));

    let second = service(uri).with_shared_cache_root(&first);
    let cached = second.chart_schema().await.unwrap();

    assert_eq!(cached.path, file);
    assert_eq!(cached.schema.title, "Odd \u{e9} bytes");
    assert_eq!(std::fs::read(&file).unwrap(), ODD_SCHEMA_BODY.as_bytes());
}

/// Seeds `service`'s cache file with `contents`, then returns its path.
fn seed(service: &DeclarativeService, contents: &[u8]) -> std::path::PathBuf {
    let dir = service.schema_cache_dir().unwrap();
    std::fs::create_dir_all(&dir).unwrap();
    let file = dir.join("sift.declarative.schema.v0.json");
    std::fs::write(&file, contents).unwrap();
    file
}

#[tokio::test]
async fn chart_schema_refreshes_a_file_at_least_24_hours_old() {
    let mut updated = chart_schema_fixture();
    updated["title"] = serde_json::json!("Updated chart schema");
    let (rest_uri, server) = start_sequence_server(schema_replies(&updated.to_string())).await;
    let owner = service(rest_uri.clone());
    let file = seed(&owner, ODD_SCHEMA_BODY.as_bytes());
    set_age(&file, Duration::from_secs(24 * 60 * 60 + 1));

    let cached = service(rest_uri)
        .with_shared_cache_root(&owner)
        .chart_schema()
        .await
        .unwrap();

    assert_eq!(cached.schema.title, "Updated chart schema");
    assert_eq!(
        std::fs::read(&file).unwrap(),
        updated.to_string().as_bytes()
    );
    assert_eq!(server.await.unwrap().len(), 2);
}

#[tokio::test]
async fn chart_schema_repairs_a_corrupt_file() {
    let (rest_uri, server) = start_sequence_server(schema_replies(ODD_SCHEMA_BODY)).await;
    let owner = service(rest_uri.clone());
    let file = seed(&owner, b"{ not json");

    let cached = service(rest_uri)
        .with_shared_cache_root(&owner)
        .chart_schema()
        .await
        .unwrap();

    assert_eq!(cached.schema.title, "Odd \u{e9} bytes");
    assert_eq!(std::fs::read(&file).unwrap(), ODD_SCHEMA_BODY.as_bytes());
    assert_eq!(server.await.unwrap().len(), 2);
}

#[tokio::test]
async fn chart_schema_refreshes_a_file_dated_in_the_future() {
    let (rest_uri, server) = start_sequence_server(schema_replies(ODD_SCHEMA_BODY)).await;
    let owner = service(rest_uri.clone());
    let file = seed(&owner, fixture_body().as_bytes());
    set_mtime(
        &file,
        std::time::SystemTime::now() + Duration::from_secs(3600),
    );

    let cached = service(rest_uri)
        .with_shared_cache_root(&owner)
        .chart_schema()
        .await
        .unwrap();

    assert_eq!(cached.schema.title, "Odd \u{e9} bytes");
    assert_eq!(server.await.unwrap().len(), 2);
}

#[tokio::test]
async fn chart_schema_keeps_a_stale_file_when_the_refresh_fails() {
    let mut responses = vec![Reply::Respond(http_response("404 Not Found", ""))];
    responses.extend(schema_replies(ODD_SCHEMA_BODY));
    let (rest_uri, server) = start_sequence_server(responses).await;
    let owner = service(rest_uri.clone());
    let file = seed(&owner, fixture_body().as_bytes());
    set_age(&file, Duration::from_secs(48 * 60 * 60));
    let stale_modified = std::fs::metadata(&file).unwrap().modified().unwrap();
    let service = service(rest_uri).with_shared_cache_root(&owner);

    let error = service.chart_schema().await.unwrap_err();
    assert!(format!("{error:#}").contains("does not serve"), "{error:#}");
    assert_eq!(std::fs::read(&file).unwrap(), fixture_body().as_bytes());
    assert_eq!(
        std::fs::metadata(&file).unwrap().modified().unwrap(),
        stale_modified
    );
    let leftovers: Vec<_> = std::fs::read_dir(file.parent().unwrap())
        .unwrap()
        .map(|entry| entry.unwrap().file_name().to_string_lossy().into_owned())
        .filter(|name| name.ends_with(".tmp"))
        .collect();
    assert!(leftovers.is_empty(), "{leftovers:?}");

    // The next call retries and now succeeds.
    let cached = service.chart_schema().await.unwrap();
    assert_eq!(cached.schema.title, "Odd \u{e9} bytes");
    server.await.unwrap();
}

#[tokio::test]
async fn chart_schema_keeps_a_stale_file_when_the_download_is_undecodable() {
    let (rest_uri, server) = start_sequence_server(schema_replies("{ not json")).await;
    let owner = service(rest_uri.clone());
    let file = seed(&owner, fixture_body().as_bytes());
    set_age(&file, Duration::from_secs(48 * 60 * 60));

    let result = service(rest_uri)
        .with_shared_cache_root(&owner)
        .chart_schema()
        .await;

    assert!(result.is_err());
    assert_eq!(std::fs::read(&file).unwrap(), fixture_body().as_bytes());
    server.await.unwrap();
}

#[tokio::test]
async fn chart_schema_does_not_share_files_between_deployments() {
    let mut other = chart_schema_fixture();
    other["title"] = serde_json::json!("Other deployment");
    let (first_uri, first_server) = start_sequence_server(schema_replies(&fixture_body())).await;
    let (other_uri, other_server) = start_sequence_server(schema_replies(&other.to_string())).await;
    let first_service = service(first_uri.clone());
    let other_service = service(other_uri).with_shared_cache_root(&first_service);

    let first = first_service.chart_schema().await.unwrap();
    let second = other_service.chart_schema().await.unwrap();

    assert_eq!(first.schema.title, "Declarative chart");
    assert_eq!(second.schema.title, "Other deployment");
    assert_ne!(first.path, second.path);
    assert_eq!(first_server.await.unwrap().len(), 2);
    assert_eq!(other_server.await.unwrap().len(), 2);

    // Scheme, port, and base path each change the key; a trailing slash does not.
    let key = |uri: &str| {
        service(uri.into())
            .with_shared_cache_root(&first_service)
            .schema_cache_dir()
            .unwrap()
    };
    let base = key("https://app.example.test");
    assert_eq!(base, key("https://app.example.test/"));
    for different in [
        "http://app.example.test",
        "https://app.example.test:8443",
        "https://app.example.test/proxy",
    ] {
        assert_ne!(base, key(different), "{different}");
    }
}

#[tokio::test]
async fn chart_schema_sees_an_external_replacement_in_a_running_service() {
    let (rest_uri, server) = start_sequence_server(schema_replies(&fixture_body())).await;
    let service = service(rest_uri);
    let before = service.chart_schema().await.unwrap();
    assert_eq!(before.schema.title, "Declarative chart");
    server.await.unwrap();

    // Another MCP process refreshes the shared file.
    let mut updated = chart_schema_fixture();
    updated["title"] = serde_json::json!("Refreshed elsewhere");
    updated["$defs"] = serde_json::json!({"onlyNew": {"description": "New."}});
    std::fs::write(&before.path, updated.to_string()).unwrap();

    let after = service.chart_schema().await.unwrap();
    assert_eq!(after.path, before.path);
    assert_eq!(after.schema.title, "Refreshed elsewhere");
    assert!(after.schema.definitions.contains_key("onlyNew"));
}

#[tokio::test]
async fn chart_schema_never_exposes_a_partial_file_to_readers() {
    let mut big = chart_schema_fixture();
    big["description"] = serde_json::json!("x".repeat(2_000_000));
    let (rest_uri, server) = start_sequence_server(schema_replies(&big.to_string())).await;
    let owner = service(rest_uri.clone());
    let file = seed(&owner, fixture_body().as_bytes());
    set_age(&file, Duration::from_secs(48 * 60 * 60));

    let (old_len, new_len) = (fixture_body().len(), big.to_string().len());
    let done = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
    let reader = {
        let (file, done) = (file.clone(), done.clone());
        std::thread::spawn(move || {
            let mut reads = 0;
            while !done.load(std::sync::atomic::Ordering::SeqCst) {
                let len = std::fs::read(&file).unwrap().len();
                assert!(len == old_len || len == new_len, "partial file: {len}");
                reads += 1;
            }
            reads
        })
    };
    service(rest_uri)
        .with_shared_cache_root(&owner)
        .chart_schema()
        .await
        .unwrap();
    done.store(true, std::sync::atomic::Ordering::SeqCst);
    assert!(reader.join().unwrap() > 0);
    assert_eq!(std::fs::read(&file).unwrap(), big.to_string().as_bytes());
    server.await.unwrap();
}

#[tokio::test]
async fn chart_schema_lock_wait_does_not_block_the_runtime() {
    let owner = service("http://127.0.0.1:1".into());
    let dir = owner.schema_cache_dir().unwrap();
    let lock = {
        let dir = dir.clone();
        tokio::task::spawn_blocking(move || super::lock_schema_dir(&dir).unwrap())
            .await
            .unwrap()
    };
    let waiting = service("http://127.0.0.1:1".into()).with_shared_cache_root(&owner);
    let call = tokio::spawn(async move { waiting.chart_schema().await.is_err() });

    // A timer on the same runtime keeps firing while the call waits.
    tokio::time::sleep(Duration::from_millis(100)).await;
    assert!(!call.is_finished());
    // Releasing the lock lets the call proceed; with no REST server it fails.
    drop(lock);
    assert!(
        tokio::time::timeout(Duration::from_secs(10), call)
            .await
            .unwrap()
            .unwrap()
    );
}

#[tokio::test]
async fn chart_schema_without_a_rest_endpoint_fails_without_caching() {
    let service = DeclarativeService::new(None, "1.2.3");

    for _ in 0..2 {
        let error = service.chart_schema().await.unwrap_err();
        assert!(format!("{error:#}").contains("without a REST endpoint"));
    }
}

#[tokio::test]
async fn chart_schema_bytes_is_always_fresh_and_never_reads_the_cache() {
    let body = fixture_body();
    let mut responses = Vec::new();
    for _ in 0..4 {
        responses.extend(schema_replies(&body));
    }
    let (rest_uri, server) = start_sequence_server(responses).await;
    let service = service(rest_uri);

    assert_eq!(service.chart_schema_bytes().await.unwrap(), body.as_bytes());
    assert_eq!(service.chart_schema_bytes().await.unwrap(), body.as_bytes());
    service.chart_schema().await.unwrap();
    // A fresh cache file now exists, but the export still downloads.
    assert_eq!(service.chart_schema_bytes().await.unwrap(), body.as_bytes());
    assert_eq!(server.await.unwrap().len(), 8);
}

#[tokio::test]
async fn chart_schema_does_not_cache_failures() {
    let success = chart_schema_responses();
    let failures = [
        vec![http_response("404 Not Found", "")],
        vec![http_response("200 OK", r#"{"versions":[]}"#)],
        vec![http_response("503 Service Unavailable", "{}"); 3],
        vec![http_response("200 OK", "invalid JSON")],
        vec![success[0].clone(), http_response("403 Forbidden", "{}")],
        vec![success[0].clone(), http_response("200 OK", "invalid JSON")],
        vec![success[0].clone(), http_response("200 OK", "{}")],
    ];
    for mut responses in failures {
        responses.extend(success.clone());
        let expected_requests = responses.len();
        let (rest_uri, server) =
            start_sequence_server(responses.into_iter().map(Reply::Respond).collect()).await;
        let service = service(rest_uri);

        assert!(service.chart_schema().await.is_err());
        let schema = service.chart_schema().await.unwrap();
        assert_eq!(schema.schema.title, "Declarative chart");
        let cached = service.chart_schema().await.unwrap();
        assert_eq!(schema.path, cached.path);
        let requests = server.await.unwrap();
        assert_eq!(requests.len(), expected_requests);
        assert!(
            String::from_utf8_lossy(&requests[expected_requests - 2])
                .starts_with("GET /api/packages/v1/schemas/declarative-chart/versions ")
        );
        assert!(
            String::from_utf8_lossy(&requests[expected_requests - 1]).starts_with(
                "GET /download/schemas/declarative-chart/0/sift.declarative.schema.v0.json "
            )
        );
    }
}
