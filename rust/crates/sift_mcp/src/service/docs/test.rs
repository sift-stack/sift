use sift_rs::docs::v1::{
    DocHit, ReadDocResponse, SearchDocsResponse, docs_service_server::DocsServiceServer,
};
use sift_test_util::{grpc::memory_sift_channel, mock::docs::v1::MockDocsServiceImpl};
use tokio::task::JoinHandle;
use tonic::{Response, Status, transport::Server};

use super::DocsService;
use crate::policy::RetryPolicy;

async fn service_with_mock(mock: MockDocsServiceImpl) -> (DocsService, JoinHandle<()>) {
    let (client, server) = tokio::io::duplex(1024);
    let channel = memory_sift_channel(client).await;

    let handle = tokio::spawn(async move {
        Server::builder()
            .add_service(DocsServiceServer::new(mock))
            .serve_with_incoming(tokio_stream::once(Ok::<_, std::io::Error>(server)))
            .await
            .unwrap();
    });

    (DocsService::new(channel, RetryPolicy::default()), handle)
}

#[tokio::test]
async fn search_docs_returns_hits() {
    let mut mock = MockDocsServiceImpl::new();
    mock.expect_search_docs()
        .withf(|req| req.get_ref().query == "asset channels")
        .returning(|_| {
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

    let (service, _h) = service_with_mock(mock).await;

    let resp = service
        .search_docs("asset channels".into(), Some(5))
        .await
        .expect("search_docs failed");

    assert_eq!(resp.total_scanned, 7);
    assert_eq!(resp.hits.len(), 1);
    let hit = &resp.hits[0];
    assert_eq!(hit.path, "documentation/ingest/asset-channels.mdx");
    assert_eq!(hit.match_line, 12);
    assert_eq!(hit.total_lines, 120);
}

#[tokio::test]
async fn search_docs_forwards_max_results() {
    // `Option<u32>` maps to the proto int32; the service owns the default/cap,
    // so the client forwards the value verbatim.
    let mut mock = MockDocsServiceImpl::new();
    mock.expect_search_docs()
        .withf(|req| req.get_ref().max_results == 5)
        .returning(|_| Ok(Response::new(SearchDocsResponse::default())));

    let (service, _h) = service_with_mock(mock).await;
    service
        .search_docs("anything".into(), Some(5))
        .await
        .expect("search_docs failed");
}

#[tokio::test]
async fn read_doc_returns_page_and_forwards_offset_limit() {
    let mut mock = MockDocsServiceImpl::new();
    mock.expect_read_doc()
        .withf(|req| {
            let req = req.get_ref();
            req.path == "documentation/ingest/asset-channels.mdx"
                && req.offset == 10
                && req.limit == 50
        })
        .returning(|_| {
            Ok(Response::new(ReadDocResponse {
                path: "documentation/ingest/asset-channels.mdx".into(),
                title: "Asset Channels".into(),
                total_lines: 120,
                start_line: 10,
                content: "10\tHello".into(),
            }))
        });

    let (service, _h) = service_with_mock(mock).await;

    let resp = service
        .read_doc(
            "documentation/ingest/asset-channels.mdx".into(),
            Some(10),
            Some(50),
        )
        .await
        .expect("read_doc failed");

    assert_eq!(resp.total_lines, 120);
    assert_eq!(resp.start_line, 10);
    assert_eq!(resp.title, "Asset Channels");
}

#[tokio::test]
async fn search_docs_propagates_grpc_error() {
    let mut mock = MockDocsServiceImpl::new();
    mock.expect_search_docs()
        .returning(|_| Err(Status::invalid_argument("empty query")));

    let (service, _h) = service_with_mock(mock).await;

    let err = service
        .search_docs(String::new(), None)
        .await
        .expect_err("expected error");

    assert!(err.to_string().contains("failed to search docs"));
}

#[tokio::test]
async fn read_doc_propagates_not_found() {
    let mut mock = MockDocsServiceImpl::new();
    mock.expect_read_doc()
        .returning(|_| Err(Status::not_found("no such doc")));

    let (service, _h) = service_with_mock(mock).await;

    let err = service
        .read_doc("missing".into(), None, None)
        .await
        .expect_err("expected error");

    assert!(err.to_string().contains("failed to read doc"));
}
