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
    CalculatedChannelVersionListParams, CreateCalculatedChannelItem, UpdateCalculatedChannelItem,
};
use crate::{
    server::SiftMcpServer,
    tool::common::test_support::{
        assert_keys_before, bulk_params, content_texts, list_params, list_params_with_fields,
        structured,
    },
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

fn create_params() -> CreateCalculatedChannelItem {
    CreateCalculatedChannelItem {
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

mod bulk {
    use super::*;
    use serde_json::{Value, json};

    const TOOLS: [&str; 3] = ["update", "archive", "unarchive"];

    async fn call(server: &SiftMcpServer, tool: &str, value: Value) -> crate::error::McpResult {
        match tool {
            "update" => {
                server
                    .update_calculated_channel(Parameters(serde_json::from_value(value).unwrap()))
                    .await
            }
            "archive" => {
                server
                    .archive_calculated_channel(Parameters(serde_json::from_value(value).unwrap()))
                    .await
            }
            _ => {
                server
                    .unarchive_calculated_channel(Parameters(
                        serde_json::from_value(value).unwrap(),
                    ))
                    .await
            }
        }
    }

    fn item(id: &str, tool: &str) -> Value {
        if tool == "update" {
            json!({"calculated_channel_id": id, "name": "renamed"})
        } else {
            json!({"calculated_channel_id": id})
        }
    }

    fn mock_reads(mock: &mut MockCalculatedChannelServiceImpl) {
        mock.expect_get_calculated_channel().returning(|req| {
            let id = req.into_inner().calculated_channel_id;
            let mut channel = stored_channel();
            channel.calculated_channel_id = id;
            Ok(Response::new(GetCalculatedChannelResponse {
                calculated_channel: if channel.calculated_channel_id == "missing" {
                    None
                } else {
                    Some(channel)
                },
            }))
        });
    }

    #[tokio::test]
    async fn repeated_archive_and_unarchive_are_successes() {
        for tool in ["archive", "unarchive"] {
            let mut mock = MockCalculatedChannelServiceImpl::new();
            mock.expect_update_calculated_channel()
                .times(2)
                .returning(|req| {
                    Ok(Response::new(UpdateCalculatedChannelResponse {
                        calculated_channel: req.into_inner().calculated_channel,
                        inapplicable_assets: vec![],
                    }))
                });
            let (server, handle) = server_with_mock(mock).await;
            for _ in 0..2 {
                let result = call(&server, tool, json!({"items": [item("same", tool)]}))
                    .await
                    .unwrap();
                assert!(!result.is_error.unwrap_or(false));
                assert_eq!(structured(result)["successes"].as_array().unwrap().len(), 1);
            }
            handle.abort();
        }
    }

    #[tokio::test]
    async fn selected_subset_writes_only_selected_items() {
        for tool in TOOLS {
            let mut mock = MockCalculatedChannelServiceImpl::new();
            mock_reads(&mut mock);
            mock.expect_update_calculated_channel()
                .times(1)
                .withf(|req| {
                    req.get_ref()
                        .calculated_channel
                        .as_ref()
                        .unwrap()
                        .calculated_channel_id
                        == "selected"
                })
                .returning(|req| {
                    Ok(Response::new(UpdateCalculatedChannelResponse {
                        calculated_channel: req.into_inner().calculated_channel,
                        inapplicable_assets: vec![],
                    }))
                });
            let (server, handle) = server_with_mock(mock).await;
            let result = call(&server, tool, json!({"items": [item("skip", tool), item("selected", tool)], "selected_indices": [1]})).await.unwrap();
            let body = structured(result);
            assert_eq!(body["successes"][0]["index"], 1);
            assert_eq!(body["not_selected"], json!([0]));
            handle.abort();
        }
    }

    #[tokio::test]
    async fn not_found_continues_and_unavailable_stops_tail() {
        for tool in TOOLS {
            let mut mock = MockCalculatedChannelServiceImpl::new();
            mock_reads(&mut mock);
            mock.expect_update_calculated_channel().returning(|req| {
                let channel = req.into_inner().calculated_channel.unwrap();
                match channel.calculated_channel_id.as_str() {
                    "missing" => Err(Status::not_found("missing channel")),
                    "down" => Err(Status::unavailable("backend down")),
                    "never" => panic!("tail must not run"),
                    _ => Ok(Response::new(UpdateCalculatedChannelResponse {
                        calculated_channel: Some(channel),
                        inapplicable_assets: vec![],
                    })),
                }
            });
            let (server, handle) = server_with_mock(mock).await;
            let body = structured(call(&server, tool, json!({"items": [item("missing", tool), item("ok", tool), item("down", tool), item("never", tool)]})).await.unwrap());
            assert_eq!(body["successes"][0]["index"], 1);
            assert_eq!(
                body["failures"][0]["error"]["code"],
                ErrorCode::RESOURCE_NOT_FOUND.0
            );
            assert_eq!(body["failures"][1]["index"], 2);
            assert_eq!(body["not_attempted"], json!([3]));
            handle.abort();
        }
    }

    #[tokio::test]
    async fn invalid_duplicate_and_missing_chat_mask_fail_before_rpc() {
        for tool in TOOLS {
            let (server, handle) = server_with_mock(MockCalculatedChannelServiceImpl::new()).await;
            let server = server.with_require_selection(true);
            for items in [
                json!([item("a", tool), item("a", tool)]),
                json!([item("a", tool), item(" ", tool)]),
                json!([{"calculated_channel_id": "b", "unknown": true}]),
            ] {
                let error = call(
                    &server,
                    tool,
                    json!({"items": items, "selected_indices": [0]}),
                )
                .await
                .unwrap_err();
                assert_eq!(error.code, ErrorCode::INVALID_PARAMS);
            }
            let error = call(&server, tool, json!({"items": [item("a", tool)]}))
                .await
                .unwrap_err();
            assert!(error.message.contains("nothing was written"));
            handle.abort();
        }
    }
}

fn update_call(
    item: UpdateCalculatedChannelItem,
) -> Parameters<super::BulkItems<UpdateCalculatedChannelItem>> {
    assert!(item.metadata.is_none());
    Parameters(
        serde_json::from_value(serde_json::json!({"items": [{
            "calculated_channel_id": item.calculated_channel_id,
            "name": item.name, "description": item.description, "units": item.units,
            "expression": item.expression,
            "expression_channel_references_json": item.expression_channel_references_json,
            "all_assets": item.all_assets, "asset_ids": item.asset_ids, "tag_ids": item.tag_ids,
            "user_notes": item.user_notes,
        }]}))
        .unwrap(),
    )
}

fn archive_call(id: &str) -> Parameters<super::BulkItems<super::CalculatedChannelArchiveItem>> {
    bulk_params(serde_json::json!([{ "calculated_channel_id": id }]), None)
}

fn update_params() -> UpdateCalculatedChannelItem {
    UpdateCalculatedChannelItem {
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

    let texts = content_texts(&resp);
    assert_eq!(texts.len(), 2);
    assert!(texts[0].starts_with("Listed 1 calculated channel versions."));
    assert_keys_before(
        &texts[1],
        &["count", "has_more", "next_step"],
        "calculated_channel_versions",
    );

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
        .create_calculated_channel_item(Parameters(create_params()))
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
        .create_calculated_channel_item(Parameters(create_params()))
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
    assert!(
        body["warning"]
            .as_str()
            .expect("warning")
            .contains("1 in-scope asset(s)")
    );
}

#[tokio::test]
async fn create_calculated_channel_rejects_malformed_references_json() {
    let (server, _h) = server_with_mock(MockCalculatedChannelServiceImpl::new()).await;

    let mut params = create_params();
    params.expression_channel_references_json = "{not json".into();

    let err = server
        .create_calculated_channel_item(Parameters(params))
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
        .create_calculated_channel_item(Parameters(params))
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
        .create_calculated_channel_item(Parameters(params))
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
        .create_calculated_channel_item(Parameters(params))
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
        .create_calculated_channel_item(Parameters(params))
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
        .update_calculated_channel(update_call(params))
        .await
        .expect_err("expected destructive gate to reject the call");

    assert_eq!(err.code, ErrorCode::INVALID_REQUEST);
    assert!(err.message.contains("--allow-destructive"));
}

#[tokio::test]
async fn update_calculated_channel_rejects_empty_update() {
    let (server, _h) = server_with_mock(MockCalculatedChannelServiceImpl::new()).await;

    let err = server
        .update_calculated_channel(update_call(update_params()))
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
        .update_calculated_channel(update_call(params))
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
        .update_calculated_channel(update_call(params))
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
        .update_calculated_channel(update_call(params))
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
        .update_calculated_channel(update_call(params))
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
        .update_calculated_channel(update_call(params))
        .await
        .expect("update_calculated_channel failed");

    let body = structured(resp)["successes"][0]["result"].clone();
    assert_eq!(body["calculated_channel"]["name"], "renamed");
    assert_eq!(body["calculated_channel"]["version"], 4);
    assert!(body.get("next_step").is_none(), "{body}");
}

#[tokio::test]
async fn archive_calculated_channel_blocked_without_allow_destructive() {
    let mock = MockCalculatedChannelServiceImpl::new();
    let (server, _h) = server_with_mock_and_flags(mock, false, false).await;

    let err = server
        .archive_calculated_channel(archive_call("cc1"))
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
        .unarchive_calculated_channel(archive_call("cc1"))
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
        .archive_calculated_channel(archive_call("cc1"))
        .await
        .expect("archive_calculated_channel failed");

    let body = structured(resp)["successes"][0]["result"].clone();
    assert_eq!(body["archived"], true);
    assert_eq!(body["calculated_channel"]["calculatedChannelId"], "cc1");
    assert!(body.get("warning").is_none(), "{body}");
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
        .unarchive_calculated_channel(archive_call("cc1"))
        .await
        .expect("unarchive_calculated_channel failed");

    let body = structured(resp)["successes"][0]["result"].clone();
    assert_eq!(body["unarchived"], true);
    assert!(body.get("warning").is_none(), "{body}");
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
        .archive_calculated_channel(archive_call("cc1"))
        .await
        .expect("archive_calculated_channel failed");
    assert_eq!(resp.is_error, Some(true));
    let body = structured(resp);
    assert_eq!(body["archived_count"], 0);
    let failure = &body["failures"][0];
    assert_eq!(failure["outcome_unknown"], true);
    let message = failure["error"]["message"].as_str().expect("message");
    assert!(message.contains("`is_archived: false`"), "{message}");
    assert!(message.contains("may not be archived"), "{message}");
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
        .unarchive_calculated_channel(archive_call("cc1"))
        .await
        .expect("unarchive_calculated_channel failed");
    assert_eq!(resp.is_error, Some(true));
    let body = structured(resp);
    assert_eq!(body["unarchived_count"], 0);
    let failure = &body["failures"][0];
    assert_eq!(failure["outcome_unknown"], true);
    let message = failure["error"]["message"].as_str().expect("message");
    assert!(message.contains("`is_archived: true`"), "{message}");
    assert!(message.contains("may not be unarchived"), "{message}");
}

#[tokio::test]
async fn archive_calculated_channel_rejects_empty_id() {
    let (server, _h) = server_with_mock(MockCalculatedChannelServiceImpl::new()).await;

    let err = server
        .archive_calculated_channel(archive_call(""))
        .await
        .expect_err("expected error");

    assert_eq!(err.code, ErrorCode::INVALID_PARAMS);
}
