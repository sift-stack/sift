use rmcp::{handler::server::wrapper::Parameters, model::ErrorCode};
use sift_rs::calculated_channels::v2::{
    CalculatedChannel, CalculatedChannelValidationResult, CreateCalculatedChannelResponse,
    GetCalculatedChannelResponse, ListCalculatedChannelVersionsResponse,
    ListCalculatedChannelsResponse, UpdateCalculatedChannelResponse,
    calculated_channel_service_server::CalculatedChannelServiceServer,
};
use sift_test_util::{
    grpc::memory_sift_channel, mock::calculated_channels::v2::MockCalculatedChannelServiceImpl,
};
use tokio::task::JoinHandle;
use tonic::{Response, Status, transport::Server};

use super::{
    CalculatedChannelArchiveParams, CalculatedChannelVersionListParams,
    CreateCalculatedChannelParams, UpdateCalculatedChannelParams,
};
use crate::{
    server::SiftMcpServer,
    tool::common::test_support::{list_params, list_params_with_fields, structured},
};

const REFERENCES_JSON: &str = r#"[
    { "channel_reference": "$1", "channel_identifier": "thrust" },
    { "channel_reference": "$2", "channel_identifier": "thrust_limit" }
]"#;

async fn server_with_mock(
    mock: MockCalculatedChannelServiceImpl,
) -> (SiftMcpServer, JoinHandle<()>) {
    server_with_mock_and_flags(mock, true, true).await
}

async fn server_with_mock_and_flags(
    mock: MockCalculatedChannelServiceImpl,
    allow_create: bool,
    allow_destructive: bool,
) -> (SiftMcpServer, JoinHandle<()>) {
    let (client, server) = tokio::io::duplex(1024);
    let channel = memory_sift_channel(client).await;

    let handle = tokio::spawn(async move {
        Server::builder()
            .add_service(CalculatedChannelServiceServer::new(mock))
            .serve_with_incoming(tokio_stream::once(Ok::<_, std::io::Error>(server)))
            .await
            .unwrap();
    });

    (
        SiftMcpServer::new(
            channel,
            String::from("https://app.test.local"),
            allow_create,
            allow_destructive,
        ),
        handle,
    )
}

fn create_params() -> CreateCalculatedChannelParams {
    CreateCalculatedChannelParams {
        name: "thrust_margin".into(),
        expression: "$1 - $2".into(),
        expression_channel_references_json: REFERENCES_JSON.into(),
        description: None,
        user_notes: None,
        units: None,
        client_key: None,
        all_assets: Some(true),
        asset_ids: None,
        tag_ids: None,
        metadata: None,
    }
}

fn update_params() -> UpdateCalculatedChannelParams {
    UpdateCalculatedChannelParams {
        calculated_channel_id: "cc1".into(),
        name: None,
        description: None,
        units: None,
        expression: None,
        expression_channel_references_json: None,
        all_assets: None,
        asset_ids: None,
        tag_ids: None,
        metadata: None,
        user_notes: None,
    }
}

fn stored_channel() -> CalculatedChannel {
    CalculatedChannel {
        calculated_channel_id: "cc1".into(),
        name: "thrust_margin".into(),
        version: 3,
        ..Default::default()
    }
}

#[tokio::test]
async fn list_calculated_channels_returns_single_page() {
    let mut mock = MockCalculatedChannelServiceImpl::new();
    mock.expect_list_calculated_channels()
        .withf(|req| req.get_ref().filter == "is_archived == false")
        .returning(|_| {
            Ok(Response::new(ListCalculatedChannelsResponse {
                calculated_channels: vec![
                    CalculatedChannel {
                        calculated_channel_id: "cc1".into(),
                        name: "thrust_margin".into(),
                        ..Default::default()
                    },
                    CalculatedChannel {
                        calculated_channel_id: "cc2".into(),
                        name: "chamber_dp".into(),
                        ..Default::default()
                    },
                ],
                next_page_token: String::new(),
            }))
        });

    let (server, _h) = server_with_mock(mock).await;

    let resp = server
        .list_calculated_channels(list_params("is_archived == false", None))
        .await
        .expect("list_calculated_channels failed");

    let body = structured(resp);
    let channels = body["calculated_channels"]
        .as_array()
        .expect("expected an array");
    assert_eq!(channels.len(), 2);
    assert_eq!(channels[0]["calculatedChannelId"], "cc1");
    assert_eq!(channels[1]["name"], "chamber_dp");
    assert_eq!(body["count"], 2);
}

#[tokio::test]
async fn list_calculated_channels_propagates_grpc_error() {
    let mut mock = MockCalculatedChannelServiceImpl::new();
    mock.expect_list_calculated_channels()
        .returning(|_| Err(Status::invalid_argument("bad filter")));

    let (server, _h) = server_with_mock(mock).await;

    let err = server
        .list_calculated_channels(list_params("nope", None))
        .await
        .expect_err("expected error");

    assert_eq!(err.code, ErrorCode::INVALID_PARAMS);
    assert!(err.message.contains("bad filter"));
}

#[tokio::test]
async fn list_calculated_channel_versions_returns_versions() {
    let mut mock = MockCalculatedChannelServiceImpl::new();
    mock.expect_list_calculated_channel_versions()
        .withf(|req| req.get_ref().calculated_channel_id == "cc1")
        .returning(|_| {
            Ok(Response::new(ListCalculatedChannelVersionsResponse {
                calculated_channel_versions: vec![CalculatedChannel {
                    calculated_channel_id: "cc1".into(),
                    version: 2,
                    ..Default::default()
                }],
                next_page_token: String::new(),
            }))
        });

    let (server, _h) = server_with_mock(mock).await;

    let resp = server
        .list_calculated_channel_versions(Parameters(CalculatedChannelVersionListParams {
            calculated_channel_id: "cc1".into(),
            filter: String::new(),
            order_by: None,
            limit: None,
            fields: None,
        }))
        .await
        .expect("list_calculated_channel_versions failed");

    let body = structured(resp);
    let versions = body["calculated_channel_versions"]
        .as_array()
        .expect("expected an array");
    assert_eq!(versions.len(), 1);
    assert_eq!(versions[0]["version"], 2);
    assert_eq!(body["count"], 1);
    assert!(body["next_step"].is_string());
}

#[tokio::test]
async fn list_calculated_channel_versions_rejects_empty_id() {
    let (server, _h) = server_with_mock(MockCalculatedChannelServiceImpl::new()).await;

    let err = server
        .list_calculated_channel_versions(Parameters(CalculatedChannelVersionListParams {
            calculated_channel_id: String::new(),
            filter: String::new(),
            order_by: None,
            limit: None,
            fields: None,
        }))
        .await
        .expect_err("expected error");

    assert_eq!(err.code, ErrorCode::INVALID_PARAMS);
}

#[tokio::test]
async fn list_calculated_channels_projects_requested_fields() {
    let mut mock = MockCalculatedChannelServiceImpl::new();
    mock.expect_list_calculated_channels().returning(|_| {
        Ok(Response::new(ListCalculatedChannelsResponse {
            calculated_channels: vec![CalculatedChannel {
                calculated_channel_id: "cc1".into(),
                name: "thrust_margin".into(),
                description: "engine headroom".into(),
                ..Default::default()
            }],
            next_page_token: String::new(),
        }))
    });

    let (server, _h) = server_with_mock(mock).await;

    let resp = server
        .list_calculated_channels(list_params_with_fields("", &["name"]))
        .await
        .expect("list_calculated_channels failed");

    let body = structured(resp);
    assert_eq!(
        body["calculated_channels"],
        serde_json::json!([{ "name": "thrust_margin" }])
    );
    assert_eq!(body["count"], 1);
}

#[tokio::test]
async fn list_calculated_channel_versions_projects_requested_fields() {
    let mut mock = MockCalculatedChannelServiceImpl::new();
    mock.expect_list_calculated_channel_versions()
        .returning(|_| {
            Ok(Response::new(ListCalculatedChannelVersionsResponse {
                calculated_channel_versions: vec![CalculatedChannel {
                    calculated_channel_id: "cc1".into(),
                    version: 2,
                    name: "thrust_margin".into(),
                    ..Default::default()
                }],
                next_page_token: String::new(),
            }))
        });

    let (server, _h) = server_with_mock(mock).await;

    let resp = server
        .list_calculated_channel_versions(Parameters(CalculatedChannelVersionListParams {
            calculated_channel_id: "cc1".into(),
            filter: String::new(),
            order_by: None,
            limit: None,
            fields: Some(vec!["version".into()]),
        }))
        .await
        .expect("list_calculated_channel_versions failed");

    let body = structured(resp);
    assert_eq!(
        body["calculated_channel_versions"],
        serde_json::json!([{ "version": 2 }])
    );
    assert_eq!(body["count"], 1);
}

#[tokio::test]
async fn create_calculated_channel_blocked_without_allow_create() {
    // No expectations on the mock: the gate must fire before any RPC.
    let mock = MockCalculatedChannelServiceImpl::new();
    let (server, _h) = server_with_mock_and_flags(mock, false, false).await;

    let err = server
        .create_calculated_channel(Parameters(create_params()))
        .await
        .expect_err("expected create gate to reject the call");

    assert_eq!(err.code, ErrorCode::INVALID_REQUEST);
    assert!(err.message.contains("--allow-create"));
    assert!(err.message.contains("sift-cli agent update --allow-create"));
}

#[tokio::test]
async fn create_calculated_channel_returns_structured_result() {
    let mut mock = MockCalculatedChannelServiceImpl::new();
    mock.expect_create_calculated_channel()
        .times(1)
        .returning(|_| {
            Ok(Response::new(CreateCalculatedChannelResponse {
                calculated_channel: Some(stored_channel()),
                inapplicable_assets: vec![CalculatedChannelValidationResult {
                    asset_id: "asset-9".into(),
                    asset_name: Some("rover-09".into()),
                    tag_names: vec![],
                    missing_channels: vec!["thrust".into()],
                }],
            }))
        });

    let (server, _h) = server_with_mock(mock).await;

    let resp = server
        .create_calculated_channel(Parameters(create_params()))
        .await
        .expect("create_calculated_channel failed");

    let body = structured(resp);
    assert_eq!(body["calculated_channel"]["calculatedChannelId"], "cc1");
    assert_eq!(
        body["inapplicable_assets"]
            .as_array()
            .expect("expected an array")
            .len(),
        1
    );
    assert!(body["next_step"].is_string());
}

#[tokio::test]
async fn create_calculated_channel_rejects_malformed_references_json() {
    let (server, _h) = server_with_mock(MockCalculatedChannelServiceImpl::new()).await;

    let mut params = create_params();
    params.expression_channel_references_json = "{not json".into();

    let err = server
        .create_calculated_channel(Parameters(params))
        .await
        .expect_err("expected error");

    assert_eq!(err.code, ErrorCode::INVALID_PARAMS);
    assert!(err.message.contains("expression_channel_references_json"));
}

#[tokio::test]
async fn create_calculated_channel_rejects_all_assets_with_asset_ids() {
    let (server, _h) = server_with_mock(MockCalculatedChannelServiceImpl::new()).await;

    let mut params = create_params();
    params.all_assets = Some(true);
    params.asset_ids = Some(vec!["asset-1".into()]);

    let err = server
        .create_calculated_channel(Parameters(params))
        .await
        .expect_err("expected error");

    assert_eq!(err.code, ErrorCode::INVALID_PARAMS);
}

#[tokio::test]
async fn create_calculated_channel_requires_an_asset_scope() {
    let (server, _h) = server_with_mock(MockCalculatedChannelServiceImpl::new()).await;

    let mut params = create_params();
    params.all_assets = None;

    let err = server
        .create_calculated_channel(Parameters(params))
        .await
        .expect_err("expected error");

    assert_eq!(err.code, ErrorCode::INVALID_PARAMS);
}

#[tokio::test]
async fn create_calculated_channel_rejects_empty_name() {
    let (server, _h) = server_with_mock(MockCalculatedChannelServiceImpl::new()).await;

    let mut params = create_params();
    params.name = String::new();

    let err = server
        .create_calculated_channel(Parameters(params))
        .await
        .expect_err("expected error");

    assert_eq!(err.code, ErrorCode::INVALID_PARAMS);
}

#[tokio::test]
async fn create_calculated_channel_rejects_empty_expression() {
    let (server, _h) = server_with_mock(MockCalculatedChannelServiceImpl::new()).await;

    let mut params = create_params();
    params.expression = String::new();

    let err = server
        .create_calculated_channel(Parameters(params))
        .await
        .expect_err("expected error");

    assert_eq!(err.code, ErrorCode::INVALID_PARAMS);
}

#[tokio::test]
async fn update_calculated_channel_blocked_without_allow_destructive() {
    let mock = MockCalculatedChannelServiceImpl::new();
    let (server, _h) = server_with_mock_and_flags(mock, false, false).await;

    let mut params = update_params();
    params.name = Some("renamed".into());

    let err = server
        .update_calculated_channel(Parameters(params))
        .await
        .expect_err("expected destructive gate to reject the call");

    assert_eq!(err.code, ErrorCode::INVALID_REQUEST);
    assert!(err.message.contains("--allow-destructive"));
}

#[tokio::test]
async fn update_calculated_channel_rejects_empty_update() {
    let (server, _h) = server_with_mock(MockCalculatedChannelServiceImpl::new()).await;

    let err = server
        .update_calculated_channel(Parameters(update_params()))
        .await
        .expect_err("expected error");

    assert_eq!(err.code, ErrorCode::INVALID_PARAMS);
}

#[tokio::test]
async fn update_calculated_channel_rejects_user_notes_only_update() {
    // `user_notes` is a request field, not a mask path. On its own it would send
    // an empty mask — a no-op the tool would then report as a new version. No
    // mock expectations: nothing may reach the wire.
    let (server, _h) = server_with_mock(MockCalculatedChannelServiceImpl::new()).await;

    let mut params = update_params();
    params.user_notes = Some("just a note".into());

    let err = server
        .update_calculated_channel(Parameters(params))
        .await
        .expect_err("expected error");

    assert_eq!(err.code, ErrorCode::INVALID_PARAMS);
    assert!(
        err.message.contains("user_notes"),
        "error should name `user_notes` as needing a maskable field: {}",
        err.message
    );
}

#[tokio::test]
async fn update_calculated_channel_rejects_expression_without_references() {
    let (server, _h) = server_with_mock(MockCalculatedChannelServiceImpl::new()).await;

    let mut params = update_params();
    params.expression = Some("$1 * 2".into());

    let err = server
        .update_calculated_channel(Parameters(params))
        .await
        .expect_err("expected error");

    assert_eq!(err.code, ErrorCode::INVALID_PARAMS);
}

#[tokio::test]
async fn update_calculated_channel_rejects_malformed_references_json() {
    let (server, _h) = server_with_mock(MockCalculatedChannelServiceImpl::new()).await;

    let mut params = update_params();
    params.expression = Some("$1 * 2".into());
    params.expression_channel_references_json = Some("{not json".into());

    let err = server
        .update_calculated_channel(Parameters(params))
        .await
        .expect_err("expected error");

    assert_eq!(err.code, ErrorCode::INVALID_PARAMS);
}

#[tokio::test]
async fn update_calculated_channel_rejects_all_assets_with_asset_ids() {
    let (server, _h) = server_with_mock(MockCalculatedChannelServiceImpl::new()).await;

    let mut params = update_params();
    params.all_assets = Some(true);
    params.asset_ids = Some(vec!["asset-1".into()]);

    let err = server
        .update_calculated_channel(Parameters(params))
        .await
        .expect_err("expected error");

    assert_eq!(err.code, ErrorCode::INVALID_PARAMS);
}

#[tokio::test]
async fn update_calculated_channel_returns_structured_result() {
    let mut mock = MockCalculatedChannelServiceImpl::new();
    mock.expect_get_calculated_channel().returning(|_| {
        Ok(Response::new(GetCalculatedChannelResponse {
            calculated_channel: Some(stored_channel()),
        }))
    });
    mock.expect_update_calculated_channel()
        .times(1)
        .returning(|req| {
            let mut channel = req
                .into_inner()
                .calculated_channel
                .expect("channel present");
            channel.version = 4;
            Ok(Response::new(UpdateCalculatedChannelResponse {
                calculated_channel: Some(channel),
                inapplicable_assets: vec![],
            }))
        });

    let (server, _h) = server_with_mock(mock).await;

    let mut params = update_params();
    params.name = Some("renamed".into());

    let resp = server
        .update_calculated_channel(Parameters(params))
        .await
        .expect("update_calculated_channel failed");

    let body = structured(resp);
    assert_eq!(body["calculated_channel"]["name"], "renamed");
    assert_eq!(body["calculated_channel"]["version"], 4);
    assert!(body["next_step"].is_string());
}

#[tokio::test]
async fn archive_calculated_channel_blocked_without_allow_destructive() {
    let mock = MockCalculatedChannelServiceImpl::new();
    let (server, _h) = server_with_mock_and_flags(mock, false, false).await;

    let err = server
        .archive_calculated_channel(Parameters(CalculatedChannelArchiveParams {
            calculated_channel_id: "cc1".into(),
        }))
        .await
        .expect_err("expected destructive gate to reject the call");

    assert_eq!(err.code, ErrorCode::INVALID_REQUEST);
    assert!(err.message.contains("--allow-destructive"));
}

#[tokio::test]
async fn unarchive_calculated_channel_blocked_without_allow_destructive() {
    let mock = MockCalculatedChannelServiceImpl::new();
    let (server, _h) = server_with_mock_and_flags(mock, false, false).await;

    let err = server
        .unarchive_calculated_channel(Parameters(CalculatedChannelArchiveParams {
            calculated_channel_id: "cc1".into(),
        }))
        .await
        .expect_err("expected destructive gate to reject the call");

    assert_eq!(err.code, ErrorCode::INVALID_REQUEST);
    assert!(err.message.contains("--allow-destructive"));
}

#[tokio::test]
async fn archive_calculated_channel_returns_structured_result() {
    let mut mock = MockCalculatedChannelServiceImpl::new();
    mock.expect_update_calculated_channel()
        .times(1)
        .withf(|req| {
            let req = req.get_ref();
            let channel = req.calculated_channel.as_ref().expect("channel present");
            let mask = req.update_mask.as_ref().expect("mask present");

            mask.paths == vec!["is_archived".to_string()]
                && channel.calculated_channel_id == "cc1"
                && channel.is_archived
        })
        .returning(|req| {
            Ok(Response::new(UpdateCalculatedChannelResponse {
                calculated_channel: req.into_inner().calculated_channel,
                inapplicable_assets: vec![],
            }))
        });

    let (server, _h) = server_with_mock(mock).await;

    let resp = server
        .archive_calculated_channel(Parameters(CalculatedChannelArchiveParams {
            calculated_channel_id: "cc1".into(),
        }))
        .await
        .expect("archive_calculated_channel failed");

    let body = structured(resp);
    assert_eq!(body["archived"], true);
    assert_eq!(body["calculated_channel"]["calculatedChannelId"], "cc1");
    assert!(body["next_step"].is_string());
}

#[tokio::test]
async fn unarchive_calculated_channel_returns_structured_result() {
    let mut mock = MockCalculatedChannelServiceImpl::new();
    mock.expect_update_calculated_channel()
        .times(1)
        .withf(|req| {
            let req = req.get_ref();
            let channel = req.calculated_channel.as_ref().expect("channel present");
            let mask = req.update_mask.as_ref().expect("mask present");

            mask.paths == vec!["is_archived".to_string()]
                && channel.calculated_channel_id == "cc1"
                && !channel.is_archived
        })
        .returning(|req| {
            Ok(Response::new(UpdateCalculatedChannelResponse {
                calculated_channel: req.into_inner().calculated_channel,
                inapplicable_assets: vec![],
            }))
        });

    let (server, _h) = server_with_mock(mock).await;

    let resp = server
        .unarchive_calculated_channel(Parameters(CalculatedChannelArchiveParams {
            calculated_channel_id: "cc1".into(),
        }))
        .await
        .expect("unarchive_calculated_channel failed");

    let body = structured(resp);
    assert_eq!(body["unarchived"], true);
    assert!(body["next_step"].is_string());
}

#[tokio::test]
async fn archive_calculated_channel_surfaces_a_returned_unarchived_state() {
    let mut mock = MockCalculatedChannelServiceImpl::new();
    mock.expect_update_calculated_channel().returning(|req| {
        let mut channel = req
            .into_inner()
            .calculated_channel
            .expect("channel present");
        channel.is_archived = false;
        Ok(Response::new(UpdateCalculatedChannelResponse {
            calculated_channel: Some(channel),
            inapplicable_assets: vec![],
        }))
    });

    let (server, _h) = server_with_mock(mock).await;

    let resp = server
        .archive_calculated_channel(Parameters(CalculatedChannelArchiveParams {
            calculated_channel_id: "cc1".into(),
        }))
        .await
        .expect("archive_calculated_channel failed");

    let body = structured(resp);
    assert_eq!(body["archived"], false);
    assert!(body["next_step"].as_str().unwrap().contains("unarchived"));
}

#[tokio::test]
async fn unarchive_calculated_channel_surfaces_a_returned_archived_state() {
    let mut mock = MockCalculatedChannelServiceImpl::new();
    mock.expect_update_calculated_channel().returning(|req| {
        let mut channel = req
            .into_inner()
            .calculated_channel
            .expect("channel present");
        channel.is_archived = true;
        Ok(Response::new(UpdateCalculatedChannelResponse {
            calculated_channel: Some(channel),
            inapplicable_assets: vec![],
        }))
    });

    let (server, _h) = server_with_mock(mock).await;

    let resp = server
        .unarchive_calculated_channel(Parameters(CalculatedChannelArchiveParams {
            calculated_channel_id: "cc1".into(),
        }))
        .await
        .expect("unarchive_calculated_channel failed");

    let body = structured(resp);
    assert_eq!(body["unarchived"], false);
    assert!(body["next_step"].as_str().unwrap().contains("archived"));
}

#[tokio::test]
async fn archive_calculated_channel_rejects_empty_id() {
    let (server, _h) = server_with_mock(MockCalculatedChannelServiceImpl::new()).await;

    let err = server
        .archive_calculated_channel(Parameters(CalculatedChannelArchiveParams {
            calculated_channel_id: String::new(),
        }))
        .await
        .expect_err("expected error");

    assert_eq!(err.code, ErrorCode::INVALID_PARAMS);
}
