use axum::{Router, extract::Query, http::HeaderMap, routing::get};
use std::collections::HashMap;
use tokio::net::TcpListener;
use tokio::task::JoinHandle;

use super::{DocsService, ReadDocResponse, SearchDocsResponse};

/// Spin an ephemeral HTTP server that serves the two docs endpoints with the
/// supplied JSON bodies, capturing the inbound query params and auth header for
/// assertion. Returns the service pointed at it plus the captured request state.
async fn service_with_routes(
    search_body: &str,
    read_body: &str,
) -> (DocsService, JoinHandle<()>, String) {
    let search_body = search_body.to_string();
    let read_body = read_body.to_string();
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    let base = format!("http://{addr}");

    let app = Router::new()
        .route(
            "/api/v1/docs:search",
            get(
                move |headers: HeaderMap, Query(q): Query<HashMap<String, String>>| async move {
                    // Assert auth + required query param round-trip.
                    assert_eq!(
                        headers.get("authorization").and_then(|v| v.to_str().ok()),
                        Some("Bearer test-key")
                    );
                    assert_eq!(q.get("query").map(String::as_str), Some("asset channels"));
                    axum::http::Response::builder()
                        .header("content-type", "application/json")
                        .body(search_body.to_string())
                        .unwrap()
                },
            ),
        )
        .route(
            "/api/v1/docs:read",
            get(move |Query(q): Query<HashMap<String, String>>| async move {
                assert_eq!(
                    q.get("path").map(String::as_str),
                    Some("documentation/ingest/asset-channels.mdx")
                );
                axum::http::Response::builder()
                    .header("content-type", "application/json")
                    .body(read_body.to_string())
                    .unwrap()
            }),
        );

    let handle = tokio::spawn(async move {
        axum::serve(listener, app).await.unwrap();
    });

    (
        DocsService::new(base.clone(), "test-key".into()),
        handle,
        base,
    )
}

#[tokio::test]
async fn search_docs_sends_auth_and_parses_camelcase() {
    let body = r#"{"hits":[{"path":"documentation/ingest/asset-channels.mdx","title":"Asset Channels","score":42,"matchLine":12,"totalLines":120,"content":"1\tAsset channels"}],"totalScanned":7}"#;
    let (service, _h, _base) = service_with_routes(body, "{}").await;

    let resp = service
        .search_docs("asset channels".into(), Some(5))
        .await
        .expect("search failed");

    assert_eq!(resp.total_scanned, 7);
    assert_eq!(resp.hits.len(), 1);
    let hit = &resp.hits[0];
    assert_eq!(hit.path, "documentation/ingest/asset-channels.mdx");
    assert_eq!(hit.score, 42);
    // The embedded read payload parses from camelCase.
    assert_eq!(hit.match_line, 12);
    assert_eq!(hit.total_lines, 120);
    assert_eq!(hit.content, "1\tAsset channels");
}

#[tokio::test]
async fn search_docs_parses_snake_case_alias() {
    // The gateway might emit snake_case; the aliases must still parse, both for
    // the response's `total_scanned` and each hit's `match_line`/`total_lines`.
    let body = r#"{"hits":[{"path":"a.mdx","title":"A","score":1,"match_line":5,"total_lines":9,"content":"x"}],"total_scanned":3}"#;
    let (service, _h, _base) = service_with_routes(body, "{}").await;

    let resp = service
        .search_docs("asset channels".into(), None)
        .await
        .expect("search failed");

    assert_eq!(resp.total_scanned, 3);
    assert_eq!(resp.hits.len(), 1);
    assert_eq!(resp.hits[0].match_line, 5);
    assert_eq!(resp.hits[0].total_lines, 9);
}

#[tokio::test]
async fn read_doc_parses_response() {
    let body = r#"{"path":"documentation/ingest/asset-channels.mdx","title":"Asset Channels","totalLines":120,"startLine":1,"content":"1\tHello"}"#;
    let (service, _h, _base) = service_with_routes("{}", body).await;

    let resp: ReadDocResponse = service
        .read_doc("documentation/ingest/asset-channels.mdx".into(), None, None)
        .await
        .expect("read failed");

    assert_eq!(resp.total_lines, 120);
    assert_eq!(resp.start_line, 1);
    assert_eq!(resp.title, "Asset Channels");
}

#[tokio::test]
async fn non_success_status_maps_to_tonic_status() {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    let app: Router = Router::new().route(
        "/api/v1/docs:read",
        get(|| async {
            axum::http::Response::builder()
                .status(404)
                .body("no such doc".to_string())
                .unwrap()
        }),
    );
    let _h = tokio::spawn(async move {
        axum::serve(listener, app).await.unwrap();
    });

    let service = DocsService::new(format!("http://{addr}"), "test-key".into());
    let err = service
        .read_doc("missing".into(), None, None)
        .await
        .expect_err("expected error");

    // 404 -> tonic NotFound so the tool layer maps it to RESOURCE_NOT_FOUND.
    let status = err
        .downcast::<tonic::Status>()
        .expect("expected tonic Status");
    assert_eq!(status.code(), tonic::Code::NotFound);
}

#[tokio::test]
async fn bad_request_maps_to_invalid_argument() {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    let app: Router = Router::new().route(
        "/api/v1/docs:read",
        get(|| async {
            axum::http::Response::builder()
                .status(400)
                .body("bad path".to_string())
                .unwrap()
        }),
    );
    let _h = tokio::spawn(async move {
        axum::serve(listener, app).await.unwrap();
    });

    let service = DocsService::new(format!("http://{addr}"), "test-key".into());
    let err = service
        .read_doc("..".into(), None, None)
        .await
        .expect_err("expected error");

    // 400 -> tonic InvalidArgument so the tool layer maps it to INVALID_PARAMS.
    let status = err
        .downcast::<tonic::Status>()
        .expect("expected tonic Status");
    assert_eq!(status.code(), tonic::Code::InvalidArgument);
}

#[tokio::test]
async fn success_status_with_unparseable_body_errors() {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    let app: Router = Router::new().route(
        "/api/v1/docs:read",
        get(|| async { "not json".to_string() }),
    );
    let _h = tokio::spawn(async move {
        axum::serve(listener, app).await.unwrap();
    });

    let service = DocsService::new(format!("http://{addr}"), "test-key".into());
    let err = service
        .read_doc("documentation/x.mdx".into(), None, None)
        .await
        .expect_err("expected deserialize error");

    // A 2xx with a non-JSON body fails at deserialization, so the tool layer
    // classifies it as INTERNAL_ERROR. Assert both the mapped code and that the
    // deserialize context is preserved, which also rules out an earlier
    // transport error passing this test for the wrong reason.
    let mapped = crate::error::from_anyhow(err);
    assert_eq!(mapped.code, rmcp::model::ErrorCode::INTERNAL_ERROR);
    assert!(
        mapped
            .message
            .contains("failed to deserialize docs API response"),
        "expected deserialize context, got: {}",
        mapped.message
    );
}

#[tokio::test]
async fn search_docs_forwards_max_results() {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    let app = Router::new().route(
        "/api/v1/docs:search",
        get(move |Query(q): Query<HashMap<String, String>>| async move {
            assert_eq!(q.get("max_results").map(String::as_str), Some("5"));
            axum::http::Response::builder()
                .header("content-type", "application/json")
                .body("{}".to_string())
                .unwrap()
        }),
    );
    let _h = tokio::spawn(async move {
        axum::serve(listener, app).await.unwrap();
    });

    let service = DocsService::new(format!("http://{addr}"), "test-key".into());
    service
        .search_docs("anything".into(), Some(5))
        .await
        .expect("search failed");
}

#[tokio::test]
async fn search_docs_clamps_max_results_to_25() {
    // The tool documents `max 25` / `values above 25 are coerced to 25`. Pin
    // that boundary so removing or omitting the clamp fails loudly.
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    let app = Router::new().route(
        "/api/v1/docs:search",
        get(move |Query(q): Query<HashMap<String, String>>| async move {
            assert_eq!(q.get("max_results").map(String::as_str), Some("25"));
            axum::http::Response::builder()
                .header("content-type", "application/json")
                .body("{}".to_string())
                .unwrap()
        }),
    );
    let _h = tokio::spawn(async move {
        axum::serve(listener, app).await.unwrap();
    });

    let service = DocsService::new(format!("http://{addr}"), "test-key".into());
    service
        .search_docs("anything".into(), Some(1000))
        .await
        .expect("search failed");
}

#[tokio::test]
async fn search_docs_defaults_max_results_to_10() {
    // No `max_results` supplied still sends the documented default of 10.
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    let app = Router::new().route(
        "/api/v1/docs:search",
        get(move |Query(q): Query<HashMap<String, String>>| async move {
            assert_eq!(q.get("max_results").map(String::as_str), Some("10"));
            axum::http::Response::builder()
                .header("content-type", "application/json")
                .body("{}".to_string())
                .unwrap()
        }),
    );
    let _h = tokio::spawn(async move {
        axum::serve(listener, app).await.unwrap();
    });

    let service = DocsService::new(format!("http://{addr}"), "test-key".into());
    service
        .search_docs("anything".into(), None)
        .await
        .expect("search failed");
}

#[tokio::test]
async fn read_doc_forwards_offset_and_limit() {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    let app = Router::new().route(
        "/api/v1/docs:read",
        get(move |Query(q): Query<HashMap<String, String>>| async move {
            assert_eq!(q.get("offset").map(String::as_str), Some("10"));
            assert_eq!(q.get("limit").map(String::as_str), Some("50"));
            axum::http::Response::builder()
                .header("content-type", "application/json")
                .body("{}".to_string())
                .unwrap()
        }),
    );
    let _h = tokio::spawn(async move {
        axum::serve(listener, app).await.unwrap();
    });

    let service = DocsService::new(format!("http://{addr}"), "test-key".into());
    service
        .read_doc("documentation/x.mdx".into(), Some(10), Some(50))
        .await
        .expect("read failed");
}

#[test]
fn search_response_deserializes_with_defaults() {
    // Missing fields default rather than failing.
    let resp: SearchDocsResponse = serde_json::from_str("{}").unwrap();
    assert_eq!(resp.total_scanned, 0);
    assert!(resp.hits.is_empty());
}

#[tokio::test]
async fn server_error_maps_to_internal_status() {
    // Any non-400/404 status maps to tonic Internal, which the tool layer
    // surfaces as INTERNAL_ERROR. This path differs from the 400/404 mapping.
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    let app: Router = Router::new().route(
        "/api/v1/docs:read",
        get(|| async {
            axum::http::Response::builder()
                .status(500)
                .body("boom".to_string())
                .unwrap()
        }),
    );
    let _h = tokio::spawn(async move {
        axum::serve(listener, app).await.unwrap();
    });

    let service = DocsService::new(format!("http://{addr}"), "test-key".into());
    let err = service
        .read_doc("documentation/x.mdx".into(), None, None)
        .await
        .expect_err("expected error");

    let status = err
        .downcast::<tonic::Status>()
        .expect("expected tonic Status");
    assert_eq!(status.code(), tonic::Code::Internal);
}

#[tokio::test]
async fn search_docs_error_maps_to_internal_status() {
    // search_docs builds a distinct URL and wraps errors with its own context,
    // so cover its error path independently of read_doc.
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    let app: Router = Router::new().route(
        "/api/v1/docs:search",
        get(|| async {
            axum::http::Response::builder()
                .status(500)
                .body("boom".to_string())
                .unwrap()
        }),
    );
    let _h = tokio::spawn(async move {
        axum::serve(listener, app).await.unwrap();
    });

    let service = DocsService::new(format!("http://{addr}"), "test-key".into());
    let err = service
        .search_docs("anything".into(), None)
        .await
        .expect_err("expected error");

    let status = err
        .downcast::<tonic::Status>()
        .expect("expected tonic Status");
    assert_eq!(status.code(), tonic::Code::Internal);
}
