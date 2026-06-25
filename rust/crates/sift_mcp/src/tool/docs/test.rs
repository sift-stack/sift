use axum::{Router, routing::get};
use rmcp::handler::server::wrapper::Parameters;
use rmcp::model::ErrorCode;
use sift_test_util::grpc::memory_sift_channel;
use tokio::io::DuplexStream;
use tokio::net::TcpListener;
use tokio::task::JoinHandle;

use super::SearchDocsParams;
use crate::server::SiftMcpServer;

/// Build a server whose docs HTTP client points at an ephemeral axum server.
/// The gRPC channel is required by `SiftMcpServer::new` but unused by the docs
/// tool; the returned `DuplexStream` is the channel's peer and is held alive by
/// the caller so the (lazily-connected, never-used) channel stays valid.
async fn server_with_docs(
    search_body: &'static str,
    read_body: &'static str,
) -> (SiftMcpServer, JoinHandle<()>, DuplexStream) {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    let app = Router::new()
        .route(
            "/api/v1/docs:search",
            get(move || async move {
                axum::http::Response::builder()
                    .header("content-type", "application/json")
                    .body(search_body.to_string())
                    .unwrap()
            }),
        )
        .route(
            "/api/v1/docs:read",
            get(move || async move {
                axum::http::Response::builder()
                    .header("content-type", "application/json")
                    .body(read_body.to_string())
                    .unwrap()
            }),
        );
    let docs_handle = tokio::spawn(async move {
        axum::serve(listener, app).await.unwrap();
    });

    let (client, grpc_server) = tokio::io::duplex(1024);
    let channel = memory_sift_channel(client).await;

    let server = SiftMcpServer::new(channel, format!("http://{addr}"), "test-key".into());
    (server, docs_handle, grpc_server)
}

fn params(query: Option<&str>, path: Option<&str>) -> SearchDocsParams {
    SearchDocsParams {
        query: query.map(Into::into),
        path: path.map(Into::into),
        max_results: None,
        index: None,
        lines: None,
    }
}

#[tokio::test]
async fn search_mode_returns_structured_hits() {
    let body = r#"{"hits":[{"path":"documentation/ingest/asset-channels.mdx","title":"Asset Channels","score":42,"matchLine":12,"totalLines":120,"content":"1\tAsset channels"}],"totalScanned":7}"#;
    let (server, _d, _g) = server_with_docs(body, "{}").await;

    let resp = server
        .search_docs(Parameters(params(Some("asset channels"), None)))
        .await
        .expect("search failed");

    let value = resp.structured_content.expect("structured content");
    assert_eq!(value["mode"], "search");
    assert_eq!(value["total_scanned"], 7);
    let hit = &value["hits"][0];
    assert_eq!(hit["path"], "documentation/ingest/asset-channels.mdx");
    // Hits expose the embedded read payload in snake_case, matching read mode.
    assert_eq!(hit["match_line"], 12);
    assert_eq!(hit["total_lines"], 120);
    assert_eq!(hit["content"], "1\tAsset channels");
}

#[tokio::test]
async fn read_mode_returns_structured_content() {
    let body = r#"{"path":"documentation/ingest/asset-channels.mdx","title":"Asset Channels","totalLines":120,"startLine":1,"content":"1\tHello"}"#;
    let (server, _d, _g) = server_with_docs("{}", body).await;

    let resp = server
        .search_docs(Parameters(params(
            None,
            Some("documentation/ingest/asset-channels.mdx"),
        )))
        .await
        .expect("read failed");

    let value = resp.structured_content.expect("structured content");
    assert_eq!(value["mode"], "read");
    assert_eq!(value["title"], "Asset Channels");
    assert_eq!(value["total_lines"], 120);
    assert_eq!(value["content"], "1\tHello");
}

#[tokio::test]
async fn rejects_both_query_and_path() {
    let (server, _d, _g) = server_with_docs("{}", "{}").await;

    let err = server
        .search_docs(Parameters(params(Some("x"), Some("y.mdx"))))
        .await
        .expect_err("expected INVALID_PARAMS when both query and path are set");

    assert_eq!(err.code, ErrorCode::INVALID_PARAMS);
}

#[tokio::test]
async fn rejects_neither_query_nor_path() {
    let (server, _d, _g) = server_with_docs("{}", "{}").await;

    let err = server
        .search_docs(Parameters(params(None, None)))
        .await
        .expect_err("expected INVALID_PARAMS when neither query nor path is set");

    assert_eq!(err.code, ErrorCode::INVALID_PARAMS);
}

#[tokio::test]
async fn read_mode_forwards_index_and_lines() {
    // The handler maps read-mode `index`/`lines` onto the docs API's
    // `offset`/`limit`. Exercise that wiring through the tool layer so a
    // swapped or dropped argument is caught, not just at the service level.
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    let app = Router::new().route(
        "/api/v1/docs:read",
        get(
            move |axum::extract::Query(q): axum::extract::Query<
                std::collections::HashMap<String, String>,
            >| async move {
                assert_eq!(q.get("offset").map(String::as_str), Some("10"));
                assert_eq!(q.get("limit").map(String::as_str), Some("50"));
                axum::http::Response::builder()
                    .header("content-type", "application/json")
                    .body("{}".to_string())
                    .unwrap()
            },
        ),
    );
    let _docs_handle = tokio::spawn(async move {
        axum::serve(listener, app).await.unwrap();
    });

    let (client, _grpc_peer) = tokio::io::duplex(1024);
    let channel = memory_sift_channel(client).await;
    let server = SiftMcpServer::new(channel, format!("http://{addr}"), "test-key".into());

    server
        .search_docs(Parameters(SearchDocsParams {
            query: None,
            path: Some("documentation/x.mdx".into()),
            max_results: None,
            index: Some(10),
            lines: Some(50),
        }))
        .await
        .expect("read failed");
}
