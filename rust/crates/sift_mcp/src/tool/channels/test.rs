use rmcp::model::ErrorCode;
use sift_rs::channels::v3::{
    Channel, ListChannelsResponse, channel_service_server::ChannelServiceServer,
};
use sift_test_util::{grpc::memory_sift_channel, mock::channels::v3::MockChannelServiceImpl};
use tokio::task::JoinHandle;
use tonic::{Response, Status, transport::Server};

use crate::{
    server::SiftMcpServer,
    service::common::DEFAULT_LIMIT,
    tool::common::test_support::{
        assert_keys_before, content_texts, list_params, list_params_with_fields, structured,
        structured_field,
    },
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
        SiftMcpServer::new(channel, String::from("https://app.test.local"), true, true),
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
        assert_eq!(req.page_size, DEFAULT_LIMIT);
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

#[tokio::test]
async fn list_channels_projects_requested_fields() {
    let mut channel_mock = MockChannelServiceImpl::new();
    channel_mock.expect_list_channels().returning(|_| {
        Ok(Response::new(ListChannelsResponse {
            channels: vec![Channel {
                channel_id: "c1".into(),
                name: "throttle".into(),
                description: "main throttle".into(),
                ..Default::default()
            }],
            next_page_token: String::new(),
        }))
    });

    let (server, _h) = server_with_mock(channel_mock).await;

    let resp = server
        .list_channels(list_params_with_fields("", &["name"]))
        .await
        .expect("list_channels failed");

    let channels = structured_field(resp, "channels");
    assert_eq!(channels, serde_json::json!([{ "name": "throttle" }]));
}

#[tokio::test]
async fn list_channels_reports_a_field_that_matched_nothing() {
    let mut channel_mock = MockChannelServiceImpl::new();
    channel_mock.expect_list_channels().returning(|_| {
        Ok(Response::new(ListChannelsResponse {
            channels: vec![Channel {
                channel_id: "c1".into(),
                name: "throttle".into(),
                ..Default::default()
            }],
            next_page_token: String::new(),
        }))
    });

    let (server, _h) = server_with_mock(channel_mock).await;

    let resp = server
        .list_channels(list_params_with_fields("", &["name", "nmae"]))
        .await
        .expect("list_channels failed");

    // `channels` sorts before every metadata key, so this fails if the handler
    // returns the body without `ListBody::into_result`.
    let texts = content_texts(&resp);
    assert_eq!(texts.len(), 1);
    assert_keys_before(
        &texts[0],
        &["count", "has_more", "unmatched_fields"],
        "channels",
    );

    let body = structured(resp);
    assert_eq!(
        body["channels"],
        serde_json::json!([{ "name": "throttle" }])
    );
    assert_eq!(body["unmatched_fields"], serde_json::json!(["nmae"]));
}

#[test]
fn tool_description_warns_that_unscoped_name_searches_can_time_out() {
    let tools = SiftMcpServer::channels_router().list_all();
    let tool = tools
        .iter()
        .find(|tool| tool.name == "list_channels")
        .expect("list_channels is routed");
    let description = tool
        .description
        .as_deref()
        .expect("list_channels has a description");

    assert!(
        description
            .contains("Unscoped name searches scan every channel in the org and can time out"),
        "the description must warn that unscoped name searches can time out"
    );
    assert!(
        description.contains("candidate assets with `list_assets` first"),
        "the description must tell the agent to resolve assets before searching"
    );
    assert!(
        description.contains(
            "A timeout means the filter was too broad, not that the parameters were wrong"
        ),
        "the description must say a timeout calls for a narrower filter"
    );
}

#[tokio::test]
async fn list_channels_projects_asset_id_and_unit_id_but_not_asset_name() {
    let mut channel_mock = MockChannelServiceImpl::new();
    channel_mock.expect_list_channels().returning(|_| {
        Ok(Response::new(ListChannelsResponse {
            channels: vec![Channel {
                channel_id: "c1".into(),
                asset_id: "a1".into(),
                unit_id: "u1".into(),
                ..Default::default()
            }],
            next_page_token: String::new(),
        }))
    });

    let (server, _h) = server_with_mock(channel_mock).await;

    let resp = server
        .list_channels(list_params_with_fields(
            "",
            &["asset_id", "unit_id", "asset_name"],
        ))
        .await
        .expect("list_channels failed");

    let body = structured(resp);
    assert_eq!(
        body["channels"],
        serde_json::json!([{ "assetId": "a1", "unitId": "u1" }])
    );
    assert_eq!(body["unmatched_fields"], serde_json::json!(["asset_name"]));
}
