use rmcp::handler::server::wrapper::Parameters;
use rmcp::model::ErrorCode;
use sift_rs::docs::v1::{
    DocHit, ReadDocResponse, SearchDocsResponse, docs_service_server::DocsServiceServer,
};
use sift_test_util::{grpc::memory_sift_channel, mock::docs::v1::MockDocsServiceImpl};
use tokio::task::JoinHandle;
use tonic::{Response, transport::Server};

use super::SearchDocsParams;
use crate::server::SiftMcpServer;

/// Build a server whose docs RPCs route to a mocked `DocsService` over an
/// in-memory gRPC channel. Other services share the channel but go unused.
async fn server_with_mock(mock: MockDocsServiceImpl) -> (SiftMcpServer, JoinHandle<()>) {
    let (client, server) = tokio::io::duplex(1024);
    let channel = memory_sift_channel(client).await;

    let handle = tokio::spawn(async move {
        Server::builder()
            .add_service(DocsServiceServer::new(mock))
            .serve_with_incoming(tokio_stream::once(Ok::<_, std::io::Error>(server)))
            .await
            .unwrap();
    });

    let server = SiftMcpServer::new(channel, "https://api.test.local".into());
    (server, handle)
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
    let mut mock = MockDocsServiceImpl::new();
    mock.expect_search_docs().returning(|_| {
        Ok(Response::new(SearchDocsResponse {
            hits: vec![DocHit {
                path: "documentation/ingest/asset-channels.mdx".into(),
                title: "Asset Channels".into(),
                score: 42,
                match_line: 12,
                total_lines: 120,
                content: "1\tAsset channels".into(),
            }],
            total_scanned: 7,
        }))
    });
    let (server, _h) = server_with_mock(mock).await;

    let resp = server
        .search_docs(Parameters(params(Some("asset channels"), None)))
        .await
        .expect("search failed");

    let value = resp.structured_content.expect("structured content");
    assert_eq!(value["mode"], "search");
    assert_eq!(value["total_scanned"], 7);
    let hit = &value["hits"][0];
    assert_eq!(hit["path"], "documentation/ingest/asset-channels.mdx");
    // Hits expose the payload in snake_case, matching read mode.
    assert_eq!(hit["match_line"], 12);
    assert_eq!(hit["total_lines"], 120);
    assert_eq!(hit["content"], "1\tAsset channels");
}

#[tokio::test]
async fn read_mode_returns_structured_content() {
    let mut mock = MockDocsServiceImpl::new();
    mock.expect_read_doc().returning(|_| {
        Ok(Response::new(ReadDocResponse {
            path: "documentation/ingest/asset-channels.mdx".into(),
            title: "Asset Channels".into(),
            total_lines: 120,
            start_line: 1,
            content: "1\tHello".into(),
        }))
    });
    let (server, _h) = server_with_mock(mock).await;

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
async fn read_mode_forwards_index_and_lines() {
    // The handler maps read-mode `index`/`lines` onto the proto's
    // `offset`/`limit`. Exercise that wiring through the tool layer so a
    // swapped or dropped argument is caught.
    let mut mock = MockDocsServiceImpl::new();
    mock.expect_read_doc()
        .withf(|req| {
            let req = req.get_ref();
            req.offset == 10 && req.limit == 50
        })
        .returning(|_| Ok(Response::new(ReadDocResponse::default())));
    let (server, _h) = server_with_mock(mock).await;

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

#[tokio::test]
async fn rejects_both_query_and_path() {
    let (server, _h) = server_with_mock(MockDocsServiceImpl::new()).await;

    let err = server
        .search_docs(Parameters(params(Some("x"), Some("y.mdx"))))
        .await
        .expect_err("expected INVALID_PARAMS when both query and path are set");

    assert_eq!(err.code, ErrorCode::INVALID_PARAMS);
}

#[tokio::test]
async fn rejects_neither_query_nor_path() {
    let (server, _h) = server_with_mock(MockDocsServiceImpl::new()).await;

    let err = server
        .search_docs(Parameters(params(None, None)))
        .await
        .expect_err("expected INVALID_PARAMS when neither query nor path is set");

    assert_eq!(err.code, ErrorCode::INVALID_PARAMS);
}
