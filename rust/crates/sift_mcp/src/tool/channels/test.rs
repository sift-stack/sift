use rmcp::model::ErrorCode;
use sift_rs::channels::v3::{
    Channel, ListChannelsResponse, channel_service_server::ChannelServiceServer,
};
use sift_test_util::{grpc::memory_sift_channel, mock::channels::v3::MockChannelServiceImpl};
use tokio::task::JoinHandle;
use tonic::{Response, Status, transport::Server};

use crate::{
    server::SiftMcpServer,
    service::common::PAGE_SIZE,
    tool::common::test_support::{list_params, structured_field},
};

async fn server_with_mock(mock: MockChannelServiceImpl) -> (SiftMcpServer, JoinHandle<()>) {
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
        SiftMcpServer::new(channel, String::from("https://api.test.local")),
        handle,
    )
}

#[tokio::test]
async fn list_channels_returns_single_page() {
    let mut channel_mock = MockChannelServiceImpl::new();
    channel_mock
        .expect_list_channels()
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

    let (server, _h) = server_with_mock(channel_mock).await;

    let resp = server
        .list_channels(list_params("name == \"throttle\"", None))
        .await
        .expect("list_channels failed");

    let channels = structured_field(resp, "channels");
    assert_eq!(channels.as_array().unwrap().len(), 2);
    assert_eq!(channels[0]["channelId"], "c1");
    assert_eq!(channels[1]["channelId"], "c2");
}

#[tokio::test]
async fn list_channels_paginates_until_token_empty() {
    let mut channel_mock = MockChannelServiceImpl::new();
    channel_mock.expect_list_channels().returning(|req| {
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
                String::new(),
            ),
            other => return Err(Status::invalid_argument(format!("bad token: {other}"))),
        };
        Ok(Response::new(ListChannelsResponse {
            channels,
            next_page_token: next,
        }))
    });

    let (server, _h) = server_with_mock(channel_mock).await;

    let resp = server
        .list_channels(list_params("", None))
        .await
        .expect("list_channels failed");

    let channels = structured_field(resp, "channels");
    let ids: Vec<&str> = channels
        .as_array()
        .unwrap()
        .iter()
        .map(|c| c["channelId"].as_str().unwrap())
        .collect();
    assert_eq!(ids, vec!["c1", "c2"]);
}

#[tokio::test]
async fn list_channels_respects_limit() {
    let mut channel_mock = MockChannelServiceImpl::new();
    channel_mock
        .expect_list_channels()
        .times(1)
        .returning(|req| {
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

    let (server, _h) = server_with_mock(channel_mock).await;

    let resp = server
        .list_channels(list_params("", Some(2)))
        .await
        .expect("list_channels failed");

    let channels = structured_field(resp, "channels");
    assert_eq!(channels.as_array().unwrap().len(), 2);
}

#[tokio::test]
async fn list_channels_breaks_on_empty_page() {
    let mut channel_mock = MockChannelServiceImpl::new();
    channel_mock.expect_list_channels().times(1).returning(|_| {
        Ok(Response::new(ListChannelsResponse {
            channels: vec![],
            next_page_token: "ignored".into(),
        }))
    });

    let (server, _h) = server_with_mock(channel_mock).await;

    let resp = server
        .list_channels(list_params("", None))
        .await
        .expect("list_channels failed");

    assert!(
        structured_field(resp, "channels")
            .as_array()
            .unwrap()
            .is_empty()
    );
}

#[tokio::test]
async fn list_channels_propagates_grpc_error() {
    let mut channel_mock = MockChannelServiceImpl::new();
    channel_mock
        .expect_list_channels()
        .returning(|_| Err(Status::invalid_argument("bad filter")));

    let (server, _h) = server_with_mock(channel_mock).await;

    let err = server
        .list_channels(list_params("nope", None))
        .await
        .expect_err("expected error");

    assert_eq!(err.code, ErrorCode::INVALID_PARAMS);
    assert!(err.message.contains("bad filter"));
}
