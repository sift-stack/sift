use sift_rs::channels::v3::{
    Channel, ListChannelsResponse, channel_service_server::ChannelServiceServer,
};
use sift_test_util::{grpc::memory_sift_channel, mock::channels::v3::MockChannelServiceImpl};
use tokio::task::JoinHandle;
use tonic::{Response, Status, transport::Server};

use super::ChannelService;
use crate::service::common::PAGE_SIZE;

async fn service_with_mock(mock: MockChannelServiceImpl) -> (ChannelService, JoinHandle<()>) {
    let (client, server) = tokio::io::duplex(1024);
    let channel = memory_sift_channel(client).await;

    let handle = tokio::spawn(async move {
        Server::builder()
            .add_service(ChannelServiceServer::new(mock))
            .serve_with_incoming(tokio_stream::once(Ok::<_, std::io::Error>(server)))
            .await
            .unwrap();
    });

    (
        ChannelService::new(channel, crate::policy::RetryPolicy::default()),
        handle,
    )
}

#[tokio::test]
async fn list_channels_returns_single_page() {
    let mut mock = MockChannelServiceImpl::new();
    mock.expect_list_channels()
        .withf(|req| req.get_ref().filter == "name == \"throttle\"")
        .returning(|_| {
            Ok(Response::new(ListChannelsResponse {
                channels: vec![
                    Channel {
                        channel_id: "c1".into(),
                        name: "throttle".into(),
                        ..Default::default()
                    },
                    Channel {
                        channel_id: "c2".into(),
                        name: "throttle".into(),
                        ..Default::default()
                    },
                ],
                next_page_token: String::new(),
            }))
        });

    let (service, _h) = service_with_mock(mock).await;

    let channels = service
        .list_channels("name == \"throttle\"".to_string(), None, None)
        .await
        .expect("list_channels failed");

    assert_eq!(channels.len(), 2);
    assert_eq!(channels[0].channel_id, "c1");
    assert_eq!(channels[1].channel_id, "c2");
}

#[tokio::test]
async fn list_channels_paginates_until_token_empty() {
    let mut mock = MockChannelServiceImpl::new();
    mock.expect_list_channels().returning(|req| {
        let req = req.into_inner();
        assert_eq!(req.page_size, PAGE_SIZE);
        let (channels, next) = match req.page_token.as_str() {
            "" => (
                vec![Channel {
                    channel_id: "c1".into(),
                    ..Default::default()
                }],
                "page-2".to_string(),
            ),
            "page-2" => (
                vec![Channel {
                    channel_id: "c2".into(),
                    ..Default::default()
                }],
                "page-3".to_string(),
            ),
            "page-3" => (
                vec![Channel {
                    channel_id: "c3".into(),
                    ..Default::default()
                }],
                String::new(),
            ),
            other => return Err(Status::invalid_argument(format!("bad token: {other}"))),
        };
        Ok(Response::new(ListChannelsResponse {
            channels,
            next_page_token: next,
        }))
    });

    let (service, _h) = service_with_mock(mock).await;

    let channels = service
        .list_channels(String::new(), None, None)
        .await
        .expect("list_channels failed");

    let ids: Vec<&str> = channels.iter().map(|c| c.channel_id.as_str()).collect();
    assert_eq!(ids, vec!["c1", "c2", "c3"]);
}

#[tokio::test]
async fn list_channels_respects_limit() {
    let mut mock = MockChannelServiceImpl::new();
    mock.expect_list_channels().times(1).returning(|req| {
        let req = req.into_inner();
        assert_eq!(req.page_size, 2);
        Ok(Response::new(ListChannelsResponse {
            channels: vec![
                Channel {
                    channel_id: "c1".into(),
                    ..Default::default()
                },
                Channel {
                    channel_id: "c2".into(),
                    ..Default::default()
                },
            ],
            next_page_token: "page-2".into(),
        }))
    });

    let (service, _h) = service_with_mock(mock).await;

    let channels = service
        .list_channels(String::new(), None, Some(2))
        .await
        .expect("list_channels failed");

    assert_eq!(channels.len(), 2);
}

#[tokio::test]
async fn list_channels_truncates_to_limit_across_pages() {
    let mut mock = MockChannelServiceImpl::new();
    mock.expect_list_channels().returning(|req| {
        let req = req.into_inner();
        assert_eq!(req.page_size, 3);
        let (channels, next) = match req.page_token.as_str() {
            "" => (
                vec![
                    Channel {
                        channel_id: "c1".into(),
                        ..Default::default()
                    },
                    Channel {
                        channel_id: "c2".into(),
                        ..Default::default()
                    },
                ],
                "page-2".to_string(),
            ),
            "page-2" => (
                vec![
                    Channel {
                        channel_id: "c3".into(),
                        ..Default::default()
                    },
                    Channel {
                        channel_id: "c4".into(),
                        ..Default::default()
                    },
                ],
                String::new(),
            ),
            other => return Err(Status::invalid_argument(format!("bad token: {other}"))),
        };
        Ok(Response::new(ListChannelsResponse {
            channels,
            next_page_token: next,
        }))
    });

    let (service, _h) = service_with_mock(mock).await;

    let channels = service
        .list_channels(String::new(), None, Some(3))
        .await
        .expect("list_channels failed");

    let ids: Vec<&str> = channels.iter().map(|c| c.channel_id.as_str()).collect();
    assert_eq!(ids, vec!["c1", "c2", "c3"]);
}

#[tokio::test]
async fn list_channels_breaks_on_empty_page() {
    let mut mock = MockChannelServiceImpl::new();
    mock.expect_list_channels().times(1).returning(|_| {
        Ok(Response::new(ListChannelsResponse {
            channels: vec![],
            next_page_token: "ignored".into(),
        }))
    });

    let (service, _h) = service_with_mock(mock).await;

    let channels = service
        .list_channels(String::new(), None, None)
        .await
        .expect("list_channels failed");

    assert!(channels.is_empty());
}

#[tokio::test]
async fn list_channels_propagates_grpc_error() {
    let mut mock = MockChannelServiceImpl::new();
    mock.expect_list_channels()
        .returning(|_| Err(Status::invalid_argument("bad filter")));

    let (service, _h) = service_with_mock(mock).await;

    let err = service
        .list_channels("nope".to_string(), None, None)
        .await
        .expect_err("expected error");

    assert!(err.to_string().contains("failed to query channels"));
}
