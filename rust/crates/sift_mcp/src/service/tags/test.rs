use sift_rs::tags::v2::{ListTagsResponse, Tag, tag_service_server::TagServiceServer};
use sift_test_util::{grpc::memory_sift_channel, mock::tags::v2::MockTagServiceImpl};
use tokio::task::JoinHandle;
use tonic::{Response, Status, transport::Server};

use super::TagService;
use crate::policy::RetryPolicy;
use crate::service::common::DEFAULT_LIMIT;

async fn service_with_mock(mock: MockTagServiceImpl) -> (TagService, JoinHandle<()>) {
    let (client, server) = tokio::io::duplex(1024);
    let channel = memory_sift_channel(client).await;

    let handle = tokio::spawn(async move {
        Server::builder()
            .add_service(TagServiceServer::new(mock))
            .serve_with_incoming(tokio_stream::once(Ok::<_, std::io::Error>(server)))
            .await
            .unwrap();
    });

    (TagService::new(channel, RetryPolicy::default()), handle)
}

#[tokio::test]
async fn list_tags_returns_single_page() {
    let mut mock = MockTagServiceImpl::new();
    mock.expect_list_tags()
        .withf(|req| req.get_ref().filter == "name == \"prod\"")
        .returning(|_| {
            Ok(Response::new(ListTagsResponse {
                tags: vec![
                    Tag {
                        tag_id: "t1".into(),
                        name: "prod".into(),
                        ..Default::default()
                    },
                    Tag {
                        tag_id: "t2".into(),
                        name: "prod".into(),
                        ..Default::default()
                    },
                ],
                next_page_token: String::new(),
            }))
        });

    let (service, _h) = service_with_mock(mock).await;

    let tags = service
        .list_tags("name == \"prod\"".to_string(), None, None)
        .await
        .expect("list_tags failed");

    assert_eq!(tags.len(), 2);
    assert_eq!(tags[0].tag_id, "t1");
    assert_eq!(tags[1].tag_id, "t2");
}

#[tokio::test]
async fn list_tags_paginates_until_token_empty() {
    let mut mock = MockTagServiceImpl::new();
    mock.expect_list_tags().returning(|req| {
        let req = req.into_inner();
        assert_eq!(req.page_size, DEFAULT_LIMIT);
        let (tags, next) = match req.page_token.as_str() {
            "" => (
                vec![Tag {
                    tag_id: "t1".into(),
                    ..Default::default()
                }],
                "page-2".to_string(),
            ),
            "page-2" => (
                vec![Tag {
                    tag_id: "t2".into(),
                    ..Default::default()
                }],
                String::new(),
            ),
            other => return Err(Status::invalid_argument(format!("bad token: {other}"))),
        };
        Ok(Response::new(ListTagsResponse {
            tags,
            next_page_token: next,
        }))
    });

    let (service, _h) = service_with_mock(mock).await;

    let tags = service
        .list_tags(String::new(), None, None)
        .await
        .expect("list_tags failed");

    let ids: Vec<&str> = tags.iter().map(|t| t.tag_id.as_str()).collect();
    assert_eq!(ids, vec!["t1", "t2"]);
}

#[tokio::test]
async fn list_tags_respects_limit() {
    let mut mock = MockTagServiceImpl::new();
    mock.expect_list_tags().times(1).returning(|req| {
        let req = req.into_inner();
        assert_eq!(req.page_size, 2);
        Ok(Response::new(ListTagsResponse {
            tags: vec![
                Tag {
                    tag_id: "t1".into(),
                    ..Default::default()
                },
                Tag {
                    tag_id: "t2".into(),
                    ..Default::default()
                },
            ],
            next_page_token: "page-2".into(),
        }))
    });

    let (service, _h) = service_with_mock(mock).await;

    let tags = service
        .list_tags(String::new(), None, Some(2))
        .await
        .expect("list_tags failed");

    assert_eq!(tags.len(), 2);
}

#[tokio::test]
async fn list_tags_propagates_grpc_error() {
    let mut mock = MockTagServiceImpl::new();
    mock.expect_list_tags()
        .returning(|_| Err(Status::invalid_argument("bad filter")));

    let (service, _h) = service_with_mock(mock).await;

    let err = service
        .list_tags("nope".to_string(), None, None)
        .await
        .expect_err("expected error");

    assert!(err.to_string().contains("failed to query tags"));
}
