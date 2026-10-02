use std::time::Duration;

use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpListener,
    task::JoinHandle,
};

use super::{DeclarativeService, Issue, RestRetry, ShareLinkResult};
use crate::client_event::start_http_server;
use crate::service::remote_files::RestConfig;

pub(crate) fn http_response(status: &str, body: &str) -> Vec<u8> {
    format!(
        "HTTP/1.1 {status}\r\ncontent-type: application/json\r\ncontent-length: {}\r\nconnection: close\r\n\r\n{body}",
        body.len()
    )
    .into_bytes()
}

/// Tiny backoff so retry tests finish quickly.
pub(crate) fn fast_retry() -> RestRetry {
    RestRetry {
        max_attempts: 3,
        retry_delay: Duration::from_millis(1),
        request_timeout: Duration::from_secs(5),
    }
}

fn service(rest_uri: String) -> DeclarativeService {
    service_with_retry(rest_uri, fast_retry())
}

fn service_with_retry(rest_uri: String, retry: RestRetry) -> DeclarativeService {
    DeclarativeService::new(Some(RestConfig::new(rest_uri, "test-key".into())), "1.2.3")
        .with_retry(retry)
}

/// How the sequence server answers one connection.
pub(crate) enum Reply {
    Respond(Vec<u8>),
    /// Read the request, then never answer, so the client times out.
    Hang,
}

/// Serves one `Reply` per incoming connection, in order, then returns every
/// request it read. `start_http_server` handles a single connection, which is
/// not enough to observe a retry.
pub(crate) async fn start_sequence_server(
    replies: Vec<Reply>,
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
        for reply in replies {
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
            match reply {
                Reply::Respond(response) => stream.write_all(&response).await.unwrap(),
                Reply::Hang => {
                    // Keep the connection open without answering. The task is
                    // dropped with the test runtime.
                    tokio::spawn(async move {
                        let _stream = stream;
                        tokio::time::sleep(Duration::from_secs(60)).await;
                    });
                }
            }
        }
        requests
    });

    (format!("http://{address}"), server)
}

fn split_request(request: Vec<u8>) -> (String, serde_json::Value) {
    let request = String::from_utf8(request).unwrap();
    let (headers, body) = request.split_once("\r\n\r\n").unwrap();
    (headers.to_string(), serde_json::from_str(body).unwrap())
}

#[tokio::test]
async fn validate_sends_the_spec_verbatim_in_strict_mode() {
    let (rest_uri, server) = start_http_server(http_response(
        "200 OK",
        r#"{"valid":true,"spec":"normalized: spec","warnings":[{"path":"charts[0]","message":"deprecated"}]}"#,
    ))
    .await;

    let result = service(rest_uri)
        .validate("charts:\n  - name: \"a\"\n")
        .await
        .unwrap();

    let (headers, body) = split_request(server.await.unwrap());
    assert!(headers.starts_with("POST /api/v1/declarative:validate HTTP/1.1"));
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
    assert_eq!(
        body,
        serde_json::json!({"spec": "charts:\n  - name: \"a\"\n", "strict": true})
    );

    assert!(result.valid);
    assert_eq!(result.spec, "normalized: spec");
    assert!(result.errors.is_empty());
    assert_eq!(
        result.warnings,
        vec![Issue {
            path: "charts[0]".into(),
            message: "deprecated".into()
        }]
    );
}

#[tokio::test]
async fn validate_returns_errors_for_an_invalid_spec() {
    let (rest_uri, server) = start_http_server(http_response(
        "200 OK",
        r#"{"spec":"bad","errors":[{"path":"charts[0].name","message":"required"},{"path":"","message":"unknown field"}]}"#,
    ))
    .await;

    let result = service(rest_uri).validate("bad").await.unwrap();
    server.await.unwrap();

    assert!(!result.valid);
    assert_eq!(result.errors.len(), 2);
    assert_eq!(result.errors[0].path, "charts[0].name");
    assert_eq!(result.errors[0].message, "required");
    assert_eq!(result.errors[1].path, "");
    assert!(result.warnings.is_empty());
}

#[tokio::test]
async fn validate_treats_omitted_default_fields_as_empty() {
    let (rest_uri, server) = start_http_server(http_response("200 OK", "{}")).await;

    let result = service(rest_uri).validate("x").await.unwrap();
    server.await.unwrap();

    assert!(!result.valid);
    assert_eq!(result.spec, "");
    assert!(result.errors.is_empty());
    assert!(result.warnings.is_empty());
}

#[tokio::test]
async fn migrate_returns_the_migrated_spec_and_versions() {
    let (rest_uri, server) = start_http_server(http_response(
        "200 OK",
        r#"{"originalSpec":"old","migratedSpec":"new","sourceVersion":"DECLARATIVE_SCHEMA_VERSION_V0","targetVersion":"DECLARATIVE_SCHEMA_VERSION_V1","warnings":[{"path":"a","message":"renamed"}],"valid":true}"#,
    ))
    .await;

    let result = service(rest_uri).migrate("old").await.unwrap();

    let (headers, body) = split_request(server.await.unwrap());
    assert!(headers.starts_with("POST /api/v1/declarative:migrate HTTP/1.1"));
    assert_eq!(body, serde_json::json!({"spec": "old"}));

    assert!(result.valid);
    assert_eq!(result.original_spec, "old");
    assert_eq!(result.migrated_spec, "new");
    assert_eq!(result.source_version, "DECLARATIVE_SCHEMA_VERSION_V0");
    assert_eq!(result.target_version, "DECLARATIVE_SCHEMA_VERSION_V1");
    assert_eq!(result.warnings.len(), 1);
    assert!(result.errors.is_empty());
}

#[tokio::test]
async fn migrate_with_errors_and_missing_default_fields() {
    let (rest_uri, server) = start_http_server(http_response(
        "200 OK",
        r#"{"originalSpec":"old","errors":[{"path":"b","message":"cannot migrate"}]}"#,
    ))
    .await;

    let result = service(rest_uri).migrate("old").await.unwrap();
    server.await.unwrap();

    assert!(!result.valid);
    assert_eq!(result.migrated_spec, "");
    assert_eq!(result.source_version, "");
    assert_eq!(result.target_version, "");
    assert!(result.warnings.is_empty());
    assert_eq!(result.errors[0].message, "cannot migrate");
}

#[tokio::test]
async fn a_non_2xx_response_reports_the_status_and_detail() {
    let (rest_uri, server) = start_http_server(http_response(
        "403 Forbidden",
        r#"{"message":"permission denied"}"#,
    ))
    .await;

    let error = service(rest_uri).validate("x").await.unwrap_err();
    server.await.unwrap();

    let message = format!("{error:#}");
    assert!(message.contains("403"), "{message}");
    assert!(message.contains("permission denied"), "{message}");
}

#[tokio::test]
async fn a_migrate_400_is_an_invalid_result_with_the_backend_message() {
    let (rest_uri, server) = start_sequence_server(vec![Reply::Respond(http_response(
        "400 Bad Request",
        r#"{"message":"spec must not be empty"}"#,
    ))])
    .await;

    let result = service(rest_uri).migrate(" ").await.unwrap();
    let requests = server.await.unwrap();

    assert!(!result.valid);
    assert_eq!(result.migrated_spec, "");
    assert_eq!(
        result.errors,
        vec![Issue {
            path: String::new(),
            message: "spec must not be empty".into()
        }]
    );
    // A 400 is final: no retry.
    assert_eq!(requests.len(), 1);
}

#[tokio::test]
async fn a_validate_400_is_an_invalid_result_with_the_backend_message() {
    let (rest_uri, server) =
        start_http_server(http_response("400 Bad Request", "spec must not be empty")).await;

    let result = service(rest_uri).validate(" ").await.unwrap();
    server.await.unwrap();

    assert!(!result.valid);
    assert_eq!(
        result.errors,
        vec![Issue {
            path: String::new(),
            message: "spec must not be empty".into()
        }]
    );
}

#[tokio::test]
async fn a_non_2xx_migrate_response_is_an_error() {
    let (rest_uri, server) = start_http_server(http_response(
        "404 Not Found",
        r#"{"message":"no such endpoint"}"#,
    ))
    .await;

    let error = service(rest_uri).migrate("x").await.unwrap_err();
    server.await.unwrap();

    let message = format!("{error:#}");
    assert!(message.contains("migrate"), "{message}");
    assert!(message.contains("404"), "{message}");
}

#[tokio::test]
async fn a_long_error_body_is_truncated_to_500_chars() {
    let body = "x".repeat(800);
    let (rest_uri, server) = start_http_server(http_response("403 Forbidden", &body)).await;

    let error = service(rest_uri).validate("x").await.unwrap_err();
    server.await.unwrap();

    let message = format!("{error:#}");
    assert!(message.contains(&"x".repeat(500)), "{message}");
    assert!(!message.contains(&"x".repeat(501)), "{message}");
}

#[tokio::test]
async fn a_transient_status_is_retried_until_it_succeeds() {
    let (rest_uri, server) = start_sequence_server(vec![
        Reply::Respond(http_response(
            "503 Service Unavailable",
            r#"{"message":"try later"}"#,
        )),
        Reply::Respond(http_response("200 OK", r#"{"valid":true,"spec":"ok"}"#)),
    ])
    .await;

    let result = service(rest_uri).validate("x").await.unwrap();
    let requests = server.await.unwrap();

    assert!(result.valid);
    assert_eq!(requests.len(), 2);
    // Every attempt resends the same request.
    assert_eq!(requests[0], requests[1]);
}

#[tokio::test]
async fn a_429_and_a_408_are_retried() {
    let (rest_uri, server) = start_sequence_server(vec![
        Reply::Respond(http_response("429 Too Many Requests", "{}")),
        Reply::Respond(http_response("408 Request Timeout", "{}")),
        Reply::Respond(http_response(
            "200 OK",
            r#"{"migratedSpec":"new","valid":true}"#,
        )),
    ])
    .await;

    let result = service(rest_uri).migrate("old").await.unwrap();
    let requests = server.await.unwrap();

    assert_eq!(result.migrated_spec, "new");
    assert_eq!(requests.len(), 3);
}

#[tokio::test]
async fn a_non_retryable_status_fails_without_retrying() {
    let (rest_uri, server) = start_sequence_server(vec![Reply::Respond(http_response(
        "404 Not Found",
        r#"{"message":"no such endpoint"}"#,
    ))])
    .await;

    let error = service(rest_uri).validate("x").await.unwrap_err();
    let requests = server.await.unwrap();

    let message = format!("{error:#}");
    assert!(message.contains("404"), "{message}");
    assert!(message.contains("no such endpoint"), "{message}");
    assert!(!message.contains("attempts"), "{message}");
    assert_eq!(requests.len(), 1);
}

#[tokio::test]
async fn a_persistent_5xx_gives_up_after_three_attempts() {
    let (rest_uri, server) = start_sequence_server(vec![
        Reply::Respond(http_response(
            "500 Internal Server Error",
            r#"{"message":"boom"}"#,
        )),
        Reply::Respond(http_response(
            "500 Internal Server Error",
            r#"{"message":"boom"}"#,
        )),
        Reply::Respond(http_response(
            "500 Internal Server Error",
            r#"{"message":"boom"}"#,
        )),
    ])
    .await;

    let error = service(rest_uri).migrate("x").await.unwrap_err();
    let requests = server.await.unwrap();

    let message = format!("{error:#}");
    assert!(message.contains("migrate"), "{message}");
    assert!(message.contains("failed after 3 attempts"), "{message}");
    assert!(message.contains("500"), "{message}");
    assert!(message.contains("boom"), "{message}");
    assert_eq!(requests.len(), 3);
}

#[tokio::test]
async fn max_attempts_is_configurable() {
    let (rest_uri, server) = start_sequence_server(vec![
        Reply::Respond(http_response("502 Bad Gateway", "{}")),
        Reply::Respond(http_response("502 Bad Gateway", "{}")),
    ])
    .await;
    let retry = RestRetry {
        max_attempts: 2,
        ..fast_retry()
    };

    let error = service_with_retry(rest_uri, retry)
        .validate("x")
        .await
        .unwrap_err();
    let requests = server.await.unwrap();

    assert!(
        format!("{error:#}").contains("failed after 2 attempts"),
        "{error:#}"
    );
    assert_eq!(requests.len(), 2);
}

#[tokio::test]
async fn a_timed_out_request_is_retried() {
    let (rest_uri, server) = start_sequence_server(vec![
        Reply::Hang,
        Reply::Respond(http_response("200 OK", r#"{"valid":true}"#)),
    ])
    .await;
    let retry = RestRetry {
        request_timeout: Duration::from_millis(100),
        ..fast_retry()
    };

    let result = service_with_retry(rest_uri, retry)
        .validate("x")
        .await
        .unwrap();
    let requests = server.await.unwrap();

    assert!(result.valid);
    assert_eq!(requests.len(), 2);
}

#[tokio::test]
async fn a_request_that_always_times_out_reports_the_timeout() {
    let (rest_uri, server) =
        start_sequence_server(vec![Reply::Hang, Reply::Hang, Reply::Hang]).await;
    let retry = RestRetry {
        request_timeout: Duration::from_millis(50),
        ..fast_retry()
    };

    let error = service_with_retry(rest_uri, retry)
        .validate("x")
        .await
        .unwrap_err();
    let requests = server.await.unwrap();

    let message = format!("{error:#}");
    assert!(message.contains("timed out after 50ms"), "{message}");
    assert!(message.contains("failed after 3 attempts"), "{message}");
    assert_eq!(requests.len(), 3);
}

#[tokio::test]
async fn share_link_returns_the_short_link() {
    let (rest_uri, server) =
        start_http_server(http_response("200 OK", r#"{"shortLink":"aB3dE9x"}"#)).await;

    let result = service(rest_uri)
        .create_share_link("charts:\n  - name: \"a\"\n")
        .await
        .unwrap();

    let (headers, body) = split_request(server.await.unwrap());
    assert!(headers.starts_with("POST /api/v1/declarative:sharelink HTTP/1.1"));
    assert!(
        headers
            .lines()
            .any(|line| line.eq_ignore_ascii_case("authorization: Bearer test-key"))
    );
    assert_eq!(
        body,
        serde_json::json!({"spec": "charts:\n  - name: \"a\"\n"})
    );
    assert_eq!(
        result,
        ShareLinkResult::Ok {
            short_link: "aB3dE9x".into()
        }
    );
}

#[tokio::test]
async fn share_link_400_is_an_invalid_spec_not_an_error() {
    let (rest_uri, server) = start_sequence_server(vec![Reply::Respond(http_response(
        "400 Bad Request",
        "charts[0].name is required",
    ))])
    .await;

    let result = service(rest_uri).create_share_link("bad").await.unwrap();
    let requests = server.await.unwrap();

    assert_eq!(
        result,
        ShareLinkResult::Invalid {
            message: "charts[0].name is required".into()
        }
    );
    // A 400 is final: no retry.
    assert_eq!(requests.len(), 1);
}

#[tokio::test]
async fn share_link_400_prefers_the_json_message() {
    let (rest_uri, server) = start_http_server(http_response(
        "400 Bad Request",
        r#"{"code":3,"message":"unknown field"}"#,
    ))
    .await;

    let result = service(rest_uri).create_share_link("bad").await.unwrap();
    server.await.unwrap();

    assert_eq!(
        result,
        ShareLinkResult::Invalid {
            message: "unknown field".into()
        }
    );
}

#[tokio::test]
async fn share_link_400_message_is_truncated_to_500_chars() {
    let body = "y".repeat(800);
    let (rest_uri, server) = start_http_server(http_response("400 Bad Request", &body)).await;

    let result = service(rest_uri).create_share_link("bad").await.unwrap();
    server.await.unwrap();

    assert_eq!(
        result,
        ShareLinkResult::Invalid {
            message: "y".repeat(500)
        }
    );
}

#[tokio::test]
async fn share_link_400_with_an_empty_body_uses_a_fallback_message() {
    let (rest_uri, server) = start_http_server(http_response("400 Bad Request", "")).await;

    let result = service(rest_uri).create_share_link("bad").await.unwrap();
    server.await.unwrap();

    assert_eq!(
        result,
        ShareLinkResult::Invalid {
            message: "declarative spec failed validation".into()
        }
    );
}

#[tokio::test]
async fn share_link_without_a_short_link_is_an_error() {
    for body in ["{}", r#"{"shortLink":""}"#] {
        let (rest_uri, server) = start_http_server(http_response("200 OK", body)).await;

        let error = service(rest_uri).create_share_link("x").await.unwrap_err();
        server.await.unwrap();

        assert!(
            format!("{error:#}").contains("did not include shortLink"),
            "{body}: {error:#}"
        );
    }
}

#[tokio::test]
async fn share_link_retries_a_transient_failure() {
    let (rest_uri, server) = start_sequence_server(vec![
        Reply::Respond(http_response("503 Service Unavailable", "{}")),
        Reply::Respond(http_response("200 OK", r#"{"shortLink":"Zk4mQ7p"}"#)),
    ])
    .await;

    let result = service(rest_uri).create_share_link("x").await.unwrap();
    let requests = server.await.unwrap();

    assert_eq!(
        result,
        ShareLinkResult::Ok {
            short_link: "Zk4mQ7p".into()
        }
    );
    assert_eq!(requests.len(), 2);
}

#[tokio::test]
async fn a_transport_error_is_reported() {
    // Bind then drop a listener so the port is free and refuses connections.
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let rest_uri = format!("http://{}", listener.local_addr().unwrap());
    drop(listener);

    let error = service(rest_uri).validate("x").await.unwrap_err();

    assert!(
        format!("{error:#}").contains("failed to reach the declarative validate endpoint"),
        "{error:#}"
    );
}

#[tokio::test]
async fn without_a_rest_endpoint_calls_fail_clearly() {
    let error = DeclarativeService::new(None, "1.2.3")
        .validate("x")
        .await
        .unwrap_err();

    assert!(format!("{error:#}").contains("without a REST endpoint"));
}
