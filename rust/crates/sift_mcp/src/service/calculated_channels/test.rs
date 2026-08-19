use pbjson_types::Timestamp;
use sift_rs::{
    calculated_channels::{
        v1::{ExpressionChannelReference, ExpressionRequest},
        v2::{
            BatchResolveCalculatedChannelsResponse, CalculatedChannel,
            CalculatedChannelAbstractChannelReference, CalculatedChannelAssetConfiguration,
            CalculatedChannelConfiguration, CalculatedChannelQueryConfiguration,
            CreateCalculatedChannelResponse, GetCalculatedChannelResponse,
            ListCalculatedChannelVersionsResponse, ListCalculatedChannelsResponse,
            ResolveCalculatedChannelResponse, ResolvedCalculatedChannel,
            UnresolvedCalculatedChannel, UpdateCalculatedChannelResponse,
            calculated_channel_asset_configuration::{AssetScope, AssetSelection},
            calculated_channel_query_configuration::{Query, Sel},
            calculated_channel_service_server::CalculatedChannelServiceServer,
            resolve_calculated_channel_request::CalculatedChannel as ResolveTarget,
        },
    },
    common::r#type::v1::{
        ResourceIdentifier, named_resources::Resources, resource_identifier::Identifier,
    },
    metadata::v1::{
        MetadataKey, MetadataKeyType, MetadataValue, metadata_value::Value as MetadataValueInner,
    },
};
use sift_test_util::{
    grpc::memory_sift_channel, mock::calculated_channels::v2::MockCalculatedChannelServiceImpl,
};
use tokio::task::JoinHandle;
use tonic::{Response, Status, transport::Server};

use super::{CalculatedChannelService, CalculatedChannelUpdate, NewCalculatedChannel};
use crate::policy::RetryPolicy;
use crate::service::common::{DEFAULT_LIMIT, PAGE_SIZE};

async fn service_with_mock(
    mock: MockCalculatedChannelServiceImpl,
) -> (CalculatedChannelService, JoinHandle<()>) {
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
        CalculatedChannelService::new(channel, RetryPolicy::default()),
        handle,
    )
}

fn channel_ref(reference: &str, identifier: &str) -> CalculatedChannelAbstractChannelReference {
    CalculatedChannelAbstractChannelReference {
        channel_reference: reference.into(),
        channel_identifier: identifier.into(),
        calculated_channel_reference: None,
    }
}

fn string_metadata(name: &str, value: &str) -> MetadataValue {
    MetadataValue {
        key: Some(MetadataKey {
            name: name.into(),
            r#type: MetadataKeyType::String.into(),
            ..Default::default()
        }),
        value: Some(MetadataValueInner::StringValue(value.into())),
        ..Default::default()
    }
}

/// A stored calculated channel with both halves of its configuration populated,
/// used as the read side of the update read-modify-write path.
fn existing_channel(id: &str) -> CalculatedChannel {
    CalculatedChannel {
        calculated_channel_id: id.into(),
        name: "thrust_margin".into(),
        description: "margin".into(),
        units: Some("N".into()),
        calculated_channel_configuration: Some(CalculatedChannelConfiguration {
            asset_configuration: Some(CalculatedChannelAssetConfiguration {
                asset_scope: Some(AssetScope::Selection(AssetSelection {
                    asset_ids: vec!["asset-1".into()],
                    tag_ids: vec!["tag-1".into()],
                })),
            }),
            query_configuration: Some(CalculatedChannelQueryConfiguration {
                query: Some(Query::Sel(Sel {
                    expression: "$1 - $2".into(),
                    expression_channel_references: vec![
                        channel_ref("$1", "thrust"),
                        channel_ref("$2", "thrust_limit"),
                    ],
                })),
            }),
        }),
        ..Default::default()
    }
}

fn new_channel() -> NewCalculatedChannel {
    NewCalculatedChannel {
        name: "thrust_margin".into(),
        description: Some("headroom".into()),
        user_notes: Some("initial".into()),
        units: Some("N".into()),
        client_key: Some("ck-1".into()),
        metadata: vec![string_metadata("owner", "propulsion")],
        expression: "$1 - $2".into(),
        expression_channel_references: vec![
            channel_ref("$1", "thrust"),
            channel_ref("$2", "thrust_limit"),
        ],
        all_assets: false,
        asset_ids: vec!["asset-1".into()],
        tag_ids: vec!["tag-1".into()],
    }
}

/// Extract the `Sel` query out of a calculated channel's configuration.
fn sel_of(channel: &CalculatedChannel) -> &Sel {
    match channel
        .calculated_channel_configuration
        .as_ref()
        .and_then(|c| c.query_configuration.as_ref())
        .and_then(|q| q.query.as_ref())
    {
        Some(Query::Sel(sel)) => sel,
        None => panic!("expected a sel query configuration"),
    }
}

/// Extract the asset scope out of a calculated channel's configuration.
fn asset_scope_of(channel: &CalculatedChannel) -> &AssetScope {
    channel
        .calculated_channel_configuration
        .as_ref()
        .and_then(|c| c.asset_configuration.as_ref())
        .and_then(|a| a.asset_scope.as_ref())
        .expect("expected an asset scope")
}

#[tokio::test]
async fn list_calculated_channels_returns_single_page() {
    let mut mock = MockCalculatedChannelServiceImpl::new();
    mock.expect_list_calculated_channels()
        .withf(|req| {
            let req = req.get_ref();
            req.filter == "name == \"thrust_margin\"" && req.order_by == "name desc"
        })
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
                        name: "thrust_margin".into(),
                        ..Default::default()
                    },
                ],
                next_page_token: String::new(),
            }))
        });

    let (service, _h) = service_with_mock(mock).await;

    let channels = service
        .list_calculated_channels(
            "name == \"thrust_margin\"".to_string(),
            Some("name desc".to_string()),
            None,
        )
        .await
        .expect("list_calculated_channels failed")
        .items;

    assert_eq!(channels.len(), 2);
    assert_eq!(channels[0].calculated_channel_id, "cc1");
    assert_eq!(channels[1].calculated_channel_id, "cc2");
}

#[tokio::test]
async fn list_calculated_channels_paginates_until_token_empty() {
    let mut mock = MockCalculatedChannelServiceImpl::new();
    mock.expect_list_calculated_channels().returning(|req| {
        let req = req.into_inner();
        assert_eq!(req.page_size, DEFAULT_LIMIT);
        let (calculated_channels, next) = match req.page_token.as_str() {
            "" => (
                vec![CalculatedChannel {
                    calculated_channel_id: "cc1".into(),
                    ..Default::default()
                }],
                "page-2".to_string(),
            ),
            "page-2" => (
                vec![CalculatedChannel {
                    calculated_channel_id: "cc2".into(),
                    ..Default::default()
                }],
                "page-3".to_string(),
            ),
            "page-3" => (
                vec![CalculatedChannel {
                    calculated_channel_id: "cc3".into(),
                    ..Default::default()
                }],
                String::new(),
            ),
            other => return Err(Status::invalid_argument(format!("bad token: {other}"))),
        };
        Ok(Response::new(ListCalculatedChannelsResponse {
            calculated_channels,
            next_page_token: next,
        }))
    });

    let (service, _h) = service_with_mock(mock).await;

    let channels = service
        .list_calculated_channels(String::new(), None, None)
        .await
        .expect("list_calculated_channels failed")
        .items;

    let ids: Vec<&str> = channels
        .iter()
        .map(|c| c.calculated_channel_id.as_str())
        .collect();
    assert_eq!(ids, vec!["cc1", "cc2", "cc3"]);
}

#[tokio::test]
async fn list_calculated_channels_respects_limit() {
    let mut mock = MockCalculatedChannelServiceImpl::new();
    mock.expect_list_calculated_channels()
        .times(1)
        .returning(|req| {
            let req = req.into_inner();
            assert_eq!(req.page_size, 2);
            Ok(Response::new(ListCalculatedChannelsResponse {
                calculated_channels: vec![
                    CalculatedChannel {
                        calculated_channel_id: "cc1".into(),
                        ..Default::default()
                    },
                    CalculatedChannel {
                        calculated_channel_id: "cc2".into(),
                        ..Default::default()
                    },
                ],
                next_page_token: "page-2".into(),
            }))
        });

    let (service, _h) = service_with_mock(mock).await;

    let channels = service
        .list_calculated_channels(String::new(), None, Some(2))
        .await
        .expect("list_calculated_channels failed")
        .items;

    assert_eq!(channels.len(), 2);
}

#[tokio::test]
async fn list_calculated_channels_clamps_limit_to_page_size() {
    let mut mock = MockCalculatedChannelServiceImpl::new();
    mock.expect_list_calculated_channels()
        .times(1)
        .returning(|req| {
            let req = req.into_inner();
            assert_eq!(req.page_size, PAGE_SIZE);
            Ok(Response::new(ListCalculatedChannelsResponse {
                calculated_channels: vec![CalculatedChannel {
                    calculated_channel_id: "cc1".into(),
                    ..Default::default()
                }],
                next_page_token: String::new(),
            }))
        });

    let (service, _h) = service_with_mock(mock).await;

    let channels = service
        .list_calculated_channels(String::new(), None, Some(5_000))
        .await
        .expect("list_calculated_channels failed")
        .items;

    assert_eq!(channels.len(), 1);
}

#[tokio::test]
async fn list_calculated_channels_breaks_on_empty_page() {
    let mut mock = MockCalculatedChannelServiceImpl::new();
    mock.expect_list_calculated_channels()
        .times(1)
        .returning(|_| {
            Ok(Response::new(ListCalculatedChannelsResponse {
                calculated_channels: vec![],
                next_page_token: "ignored".into(),
            }))
        });

    let (service, _h) = service_with_mock(mock).await;

    let channels = service
        .list_calculated_channels(String::new(), None, None)
        .await
        .expect("list_calculated_channels failed")
        .items;

    assert!(channels.is_empty());
}

#[tokio::test]
async fn list_calculated_channels_propagates_grpc_error() {
    let mut mock = MockCalculatedChannelServiceImpl::new();
    mock.expect_list_calculated_channels()
        .returning(|_| Err(Status::invalid_argument("bad filter")));

    let (service, _h) = service_with_mock(mock).await;

    let err = service
        .list_calculated_channels("nope".to_string(), None, None)
        .await
        .expect_err("expected error");

    assert!(
        err.to_string()
            .contains("failed to query calculated channels")
    );
}

#[tokio::test]
async fn list_calculated_channel_versions_builds_request() {
    let mut mock = MockCalculatedChannelServiceImpl::new();
    mock.expect_list_calculated_channel_versions()
        .times(1)
        .withf(|req| {
            let req = req.get_ref();
            req.calculated_channel_id == "cc1"
                && req.filter == "version == 2"
                && req.order_by == "version desc"
                && req.page_size == 10
                && req.page_token.is_empty()
        })
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

    let (service, _h) = service_with_mock(mock).await;

    let versions = service
        .list_calculated_channel_versions(
            "cc1".to_string(),
            "version == 2".to_string(),
            Some("version desc".to_string()),
            Some(10),
        )
        .await
        .expect("list_calculated_channel_versions failed")
        .items;

    assert_eq!(versions.len(), 1);
    assert_eq!(versions[0].version, 2);
}

#[tokio::test]
async fn list_calculated_channel_versions_paginates_until_token_empty() {
    let mut mock = MockCalculatedChannelServiceImpl::new();
    mock.expect_list_calculated_channel_versions()
        .returning(|req| {
            let req = req.into_inner();
            let (calculated_channel_versions, next) = match req.page_token.as_str() {
                "" => (
                    vec![CalculatedChannel {
                        version: 1,
                        ..Default::default()
                    }],
                    "page-2".to_string(),
                ),
                "page-2" => (
                    vec![CalculatedChannel {
                        version: 2,
                        ..Default::default()
                    }],
                    String::new(),
                ),
                other => return Err(Status::invalid_argument(format!("bad token: {other}"))),
            };
            Ok(Response::new(ListCalculatedChannelVersionsResponse {
                calculated_channel_versions,
                next_page_token: next,
            }))
        });

    let (service, _h) = service_with_mock(mock).await;

    let versions = service
        .list_calculated_channel_versions("cc1".to_string(), String::new(), None, None)
        .await
        .expect("list_calculated_channel_versions failed")
        .items;

    let numbers: Vec<u32> = versions.iter().map(|v| v.version).collect();
    assert_eq!(numbers, vec![1, 2]);
}

#[tokio::test]
async fn list_calculated_channel_versions_propagates_grpc_error() {
    let mut mock = MockCalculatedChannelServiceImpl::new();
    mock.expect_list_calculated_channel_versions()
        .returning(|_| Err(Status::not_found("no such calculated channel")));

    let (service, _h) = service_with_mock(mock).await;

    let err = service
        .list_calculated_channel_versions("missing".to_string(), String::new(), None, None)
        .await
        .expect_err("expected error");

    assert!(
        err.to_string()
            .contains("failed to query calculated channel versions")
    );
}

#[tokio::test]
async fn create_calculated_channel_builds_request_with_asset_selection() {
    let mut mock = MockCalculatedChannelServiceImpl::new();
    mock.expect_create_calculated_channel()
        .times(1)
        .withf(|req| {
            let req = req.get_ref();
            let config = req
                .calculated_channel_configuration
                .as_ref()
                .expect("configuration present");
            let sel = match config
                .query_configuration
                .as_ref()
                .and_then(|q| q.query.as_ref())
            {
                Some(Query::Sel(sel)) => sel,
                None => return false,
            };
            let scope = config
                .asset_configuration
                .as_ref()
                .and_then(|a| a.asset_scope.as_ref())
                .expect("asset scope present");

            req.name == "thrust_margin"
                && req.description == "headroom"
                && req.user_notes == "initial"
                && req.units.as_deref() == Some("N")
                && req.client_key.as_deref() == Some("ck-1")
                && req.metadata.len() == 1
                && sel.expression == "$1 - $2"
                && sel.expression_channel_references.len() == 2
                && sel.expression_channel_references[0].channel_reference == "$1"
                && sel.expression_channel_references[0].channel_identifier == "thrust"
                && *scope
                    == AssetScope::Selection(AssetSelection {
                        asset_ids: vec!["asset-1".to_string()],
                        tag_ids: vec!["tag-1".to_string()],
                    })
        })
        .returning(|_| {
            Ok(Response::new(CreateCalculatedChannelResponse {
                calculated_channel: Some(CalculatedChannel {
                    calculated_channel_id: "cc1".into(),
                    name: "thrust_margin".into(),
                    ..Default::default()
                }),
                inapplicable_assets: vec![],
            }))
        });

    let (service, _h) = service_with_mock(mock).await;

    let written = service
        .create_calculated_channel(new_channel())
        .await
        .expect("create_calculated_channel failed");

    assert_eq!(written.calculated_channel.calculated_channel_id, "cc1");
    assert!(written.inapplicable_assets.is_empty());
}

#[tokio::test]
async fn create_calculated_channel_builds_all_assets_scope() {
    let mut mock = MockCalculatedChannelServiceImpl::new();
    mock.expect_create_calculated_channel()
        .times(1)
        .withf(|req| {
            let scope = req
                .get_ref()
                .calculated_channel_configuration
                .as_ref()
                .and_then(|c| c.asset_configuration.as_ref())
                .and_then(|a| a.asset_scope.as_ref())
                .expect("asset scope present");
            *scope == AssetScope::AllAssets(true)
        })
        .returning(|_| {
            Ok(Response::new(CreateCalculatedChannelResponse {
                calculated_channel: Some(CalculatedChannel {
                    calculated_channel_id: "cc2".into(),
                    ..Default::default()
                }),
                inapplicable_assets: vec![],
            }))
        });

    let (service, _h) = service_with_mock(mock).await;

    let mut new = new_channel();
    new.all_assets = true;
    new.asset_ids = vec![];
    new.tag_ids = vec![];

    let written = service
        .create_calculated_channel(new)
        .await
        .expect("create_calculated_channel failed");

    assert_eq!(written.calculated_channel.calculated_channel_id, "cc2");
}

#[tokio::test]
async fn create_calculated_channel_surfaces_inapplicable_assets() {
    let mut mock = MockCalculatedChannelServiceImpl::new();
    mock.expect_create_calculated_channel().returning(|_| {
        Ok(Response::new(CreateCalculatedChannelResponse {
            calculated_channel: Some(CalculatedChannel {
                calculated_channel_id: "cc3".into(),
                ..Default::default()
            }),
            inapplicable_assets: vec![
                sift_rs::calculated_channels::v2::CalculatedChannelValidationResult {
                    asset_id: "asset-9".into(),
                    asset_name: Some("rover-09".into()),
                    tag_names: vec![],
                    missing_channels: vec!["thrust".into()],
                },
            ],
        }))
    });

    let (service, _h) = service_with_mock(mock).await;

    let written = service
        .create_calculated_channel(new_channel())
        .await
        .expect("create_calculated_channel failed");

    assert_eq!(written.inapplicable_assets.len(), 1);
    assert_eq!(written.inapplicable_assets[0].asset_id, "asset-9");
}

#[tokio::test]
async fn create_calculated_channel_propagates_grpc_error() {
    let mut mock = MockCalculatedChannelServiceImpl::new();
    mock.expect_create_calculated_channel()
        .returning(|_| Err(Status::invalid_argument("bad expression")));

    let (service, _h) = service_with_mock(mock).await;

    let err = service
        .create_calculated_channel(new_channel())
        .await
        .expect_err("expected error");

    assert!(
        err.to_string()
            .contains("failed to create calculated channel")
    );
}

#[tokio::test]
async fn create_calculated_channel_errors_when_response_missing_channel() {
    let mut mock = MockCalculatedChannelServiceImpl::new();
    mock.expect_create_calculated_channel().returning(|_| {
        Ok(Response::new(CreateCalculatedChannelResponse {
            calculated_channel: None,
            inapplicable_assets: vec![],
        }))
    });

    let (service, _h) = service_with_mock(mock).await;

    let err = service
        .create_calculated_channel(new_channel())
        .await
        .expect_err("expected error");

    assert!(
        err.to_string()
            .contains("create_calculated_channel response missing calculated channel")
    );
}

#[tokio::test]
async fn update_calculated_channel_name_only_sets_name_mask() {
    let mut mock = MockCalculatedChannelServiceImpl::new();
    mock.expect_get_calculated_channel()
        .times(1)
        .withf(|req| req.get_ref().calculated_channel_id == "cc1")
        .returning(|_| {
            Ok(Response::new(GetCalculatedChannelResponse {
                calculated_channel: Some(existing_channel("cc1")),
            }))
        });
    mock.expect_update_calculated_channel()
        .times(1)
        .withf(|req| {
            let req = req.get_ref();
            let channel = req.calculated_channel.as_ref().expect("channel present");
            let mask = req.update_mask.as_ref().expect("mask present");

            // Only `name` is in the mask, and the untouched configuration
            // survives the read-modify-write round trip.
            mask.paths == vec!["name".to_string()]
                && channel.calculated_channel_id == "cc1"
                && channel.name == "renamed"
                && sel_of(channel).expression == "$1 - $2"
                && *asset_scope_of(channel)
                    == AssetScope::Selection(AssetSelection {
                        asset_ids: vec!["asset-1".to_string()],
                        tag_ids: vec!["tag-1".to_string()],
                    })
        })
        .returning(|req| {
            Ok(Response::new(UpdateCalculatedChannelResponse {
                calculated_channel: req.into_inner().calculated_channel,
                inapplicable_assets: vec![],
            }))
        });

    let (service, _h) = service_with_mock(mock).await;

    let written = service
        .update_calculated_channel(
            "cc1".to_string(),
            CalculatedChannelUpdate {
                name: Some("renamed".into()),
                ..Default::default()
            },
        )
        .await
        .expect("update_calculated_channel failed");

    assert_eq!(written.calculated_channel.name, "renamed");
}

#[tokio::test]
async fn update_calculated_channel_sets_every_provided_field_in_mask() {
    let mut mock = MockCalculatedChannelServiceImpl::new();
    mock.expect_get_calculated_channel().returning(|_| {
        Ok(Response::new(GetCalculatedChannelResponse {
            calculated_channel: Some(existing_channel("cc1")),
        }))
    });
    mock.expect_update_calculated_channel()
        .times(1)
        .withf(|req| {
            let req = req.get_ref();
            let mask = req.update_mask.as_ref().expect("mask present");
            let channel = req.calculated_channel.as_ref().expect("channel present");

            mask.paths
                == vec![
                    "name".to_string(),
                    "description".to_string(),
                    "units".to_string(),
                    "metadata".to_string(),
                    "query_configuration".to_string(),
                    "asset_configuration".to_string(),
                ]
                && channel.description == "new description"
                && channel.units.as_deref() == Some("kN")
                && channel.metadata.len() == 1
                && req.user_notes.as_deref() == Some("bumped scaling")
        })
        .returning(|req| {
            Ok(Response::new(UpdateCalculatedChannelResponse {
                calculated_channel: req.into_inner().calculated_channel,
                inapplicable_assets: vec![],
            }))
        });

    let (service, _h) = service_with_mock(mock).await;

    service
        .update_calculated_channel(
            "cc1".to_string(),
            CalculatedChannelUpdate {
                name: Some("renamed".into()),
                description: Some("new description".into()),
                units: Some("kN".into()),
                metadata: Some(vec![string_metadata("owner", "avionics")]),
                expression: Some("$1 * 2".into()),
                expression_channel_references: Some(vec![channel_ref("$1", "thrust")]),
                all_assets: Some(true),
                user_notes: Some("bumped scaling".into()),
                ..Default::default()
            },
        )
        .await
        .expect("update_calculated_channel failed");
}

#[tokio::test]
async fn update_calculated_channel_expression_replaces_query_configuration() {
    let mut mock = MockCalculatedChannelServiceImpl::new();
    mock.expect_get_calculated_channel().returning(|_| {
        Ok(Response::new(GetCalculatedChannelResponse {
            calculated_channel: Some(existing_channel("cc1")),
        }))
    });
    mock.expect_update_calculated_channel()
        .times(1)
        .withf(|req| {
            let req = req.get_ref();
            let channel = req.calculated_channel.as_ref().expect("channel present");
            let mask = req.update_mask.as_ref().expect("mask present");
            let sel = sel_of(channel);

            mask.paths == vec!["query_configuration".to_string()]
                && sel.expression == "$1 * 2"
                && sel.expression_channel_references.len() == 1
                && sel.expression_channel_references[0].channel_identifier == "thrust"
        })
        .returning(|req| {
            Ok(Response::new(UpdateCalculatedChannelResponse {
                calculated_channel: req.into_inner().calculated_channel,
                inapplicable_assets: vec![],
            }))
        });

    let (service, _h) = service_with_mock(mock).await;

    service
        .update_calculated_channel(
            "cc1".to_string(),
            CalculatedChannelUpdate {
                expression: Some("$1 * 2".into()),
                expression_channel_references: Some(vec![channel_ref("$1", "thrust")]),
                ..Default::default()
            },
        )
        .await
        .expect("update_calculated_channel failed");
}

#[tokio::test]
async fn update_calculated_channel_asset_ids_preserve_existing_tag_ids() {
    let mut mock = MockCalculatedChannelServiceImpl::new();
    mock.expect_get_calculated_channel().returning(|_| {
        Ok(Response::new(GetCalculatedChannelResponse {
            calculated_channel: Some(existing_channel("cc1")),
        }))
    });
    mock.expect_update_calculated_channel()
        .times(1)
        .withf(|req| {
            let req = req.get_ref();
            let channel = req.calculated_channel.as_ref().expect("channel present");
            let mask = req.update_mask.as_ref().expect("mask present");

            mask.paths == vec!["asset_configuration".to_string()]
                && *asset_scope_of(channel)
                    == AssetScope::Selection(AssetSelection {
                        asset_ids: vec!["asset-2".to_string(), "asset-3".to_string()],
                        tag_ids: vec!["tag-1".to_string()],
                    })
        })
        .returning(|req| {
            Ok(Response::new(UpdateCalculatedChannelResponse {
                calculated_channel: req.into_inner().calculated_channel,
                inapplicable_assets: vec![],
            }))
        });

    let (service, _h) = service_with_mock(mock).await;

    service
        .update_calculated_channel(
            "cc1".to_string(),
            CalculatedChannelUpdate {
                asset_ids: Some(vec!["asset-2".into(), "asset-3".into()]),
                ..Default::default()
            },
        )
        .await
        .expect("update_calculated_channel failed");
}

#[tokio::test]
async fn update_calculated_channel_all_assets_replaces_selection_scope() {
    let mut mock = MockCalculatedChannelServiceImpl::new();
    mock.expect_get_calculated_channel().returning(|_| {
        Ok(Response::new(GetCalculatedChannelResponse {
            calculated_channel: Some(existing_channel("cc1")),
        }))
    });
    mock.expect_update_calculated_channel()
        .times(1)
        .withf(|req| {
            let channel = req
                .get_ref()
                .calculated_channel
                .as_ref()
                .expect("channel present");
            *asset_scope_of(channel) == AssetScope::AllAssets(true)
        })
        .returning(|req| {
            Ok(Response::new(UpdateCalculatedChannelResponse {
                calculated_channel: req.into_inner().calculated_channel,
                inapplicable_assets: vec![],
            }))
        });

    let (service, _h) = service_with_mock(mock).await;

    service
        .update_calculated_channel(
            "cc1".to_string(),
            CalculatedChannelUpdate {
                all_assets: Some(true),
                ..Default::default()
            },
        )
        .await
        .expect("update_calculated_channel failed");
}

#[tokio::test]
async fn update_calculated_channel_errors_when_channel_missing() {
    let mut mock = MockCalculatedChannelServiceImpl::new();
    mock.expect_get_calculated_channel().returning(|_| {
        Ok(Response::new(GetCalculatedChannelResponse {
            calculated_channel: None,
        }))
    });

    let (service, _h) = service_with_mock(mock).await;

    let err = service
        .update_calculated_channel(
            "missing".to_string(),
            CalculatedChannelUpdate {
                name: Some("renamed".into()),
                ..Default::default()
            },
        )
        .await
        .expect_err("expected error");

    assert!(err.to_string().contains("calculated channel"));
    assert!(err.to_string().contains("missing"));
}

#[tokio::test]
async fn update_calculated_channel_propagates_grpc_error() {
    let mut mock = MockCalculatedChannelServiceImpl::new();
    mock.expect_get_calculated_channel().returning(|_| {
        Ok(Response::new(GetCalculatedChannelResponse {
            calculated_channel: Some(existing_channel("cc1")),
        }))
    });
    mock.expect_update_calculated_channel()
        .returning(|_| Err(Status::permission_denied("no write access")));

    let (service, _h) = service_with_mock(mock).await;

    let err = service
        .update_calculated_channel(
            "cc1".to_string(),
            CalculatedChannelUpdate {
                name: Some("renamed".into()),
                ..Default::default()
            },
        )
        .await
        .expect_err("expected error");

    assert!(
        err.to_string()
            .contains("failed to update calculated channel")
    );
}

#[tokio::test]
async fn archive_calculated_channel_masks_archived_date_only() {
    let mut mock = MockCalculatedChannelServiceImpl::new();
    mock.expect_update_calculated_channel()
        .times(1)
        .withf(|req| {
            let req = req.get_ref();
            let channel = req.calculated_channel.as_ref().expect("channel present");
            let mask = req.update_mask.as_ref().expect("mask present");

            mask.paths == vec!["archived_date".to_string()]
                && channel.calculated_channel_id == "cc1"
                && channel.archived_date.is_some()
        })
        .returning(|req| {
            Ok(Response::new(UpdateCalculatedChannelResponse {
                calculated_channel: req.into_inner().calculated_channel,
                inapplicable_assets: vec![],
            }))
        });

    let (service, _h) = service_with_mock(mock).await;

    let written = service
        .archive_calculated_channel("cc1".to_string())
        .await
        .expect("archive_calculated_channel failed");

    assert_eq!(written.calculated_channel.calculated_channel_id, "cc1");
}

#[tokio::test]
async fn archive_calculated_channel_does_not_read_first() {
    // Archive is a masked field write; it must not need the read-modify-write
    // round trip that `update_calculated_channel` performs.
    let mut mock = MockCalculatedChannelServiceImpl::new();
    mock.expect_get_calculated_channel().times(0);
    mock.expect_update_calculated_channel().returning(|req| {
        Ok(Response::new(UpdateCalculatedChannelResponse {
            calculated_channel: req.into_inner().calculated_channel,
            inapplicable_assets: vec![],
        }))
    });

    let (service, _h) = service_with_mock(mock).await;

    service
        .archive_calculated_channel("cc1".to_string())
        .await
        .expect("archive_calculated_channel failed");
}

#[tokio::test]
async fn unarchive_calculated_channel_clears_archived_date_with_mask_set() {
    let mut mock = MockCalculatedChannelServiceImpl::new();
    mock.expect_update_calculated_channel()
        .times(1)
        .withf(|req| {
            let req = req.get_ref();
            let channel = req.calculated_channel.as_ref().expect("channel present");
            let mask = req.update_mask.as_ref().expect("mask present");

            mask.paths == vec!["archived_date".to_string()]
                && channel.calculated_channel_id == "cc1"
                && channel.archived_date.is_none()
        })
        .returning(|req| {
            let mut channel = req
                .into_inner()
                .calculated_channel
                .expect("channel present");
            channel.archived_date = None;
            Ok(Response::new(UpdateCalculatedChannelResponse {
                calculated_channel: Some(channel),
                inapplicable_assets: vec![],
            }))
        });

    let (service, _h) = service_with_mock(mock).await;

    let written = service
        .unarchive_calculated_channel("cc1".to_string())
        .await
        .expect("unarchive_calculated_channel failed");

    assert!(written.calculated_channel.archived_date.is_none());
}

#[tokio::test]
async fn unarchive_calculated_channel_propagates_grpc_error() {
    let mut mock = MockCalculatedChannelServiceImpl::new();
    mock.expect_update_calculated_channel()
        .returning(|_| Err(Status::not_found("no such calculated channel")));

    let (service, _h) = service_with_mock(mock).await;

    let err = service
        .unarchive_calculated_channel("missing".to_string())
        .await
        .expect_err("expected error");

    assert!(
        err.to_string()
            .contains("failed to unarchive calculated channel")
    );
}

#[tokio::test]
async fn archive_calculated_channel_errors_when_response_missing_channel() {
    let mut mock = MockCalculatedChannelServiceImpl::new();
    mock.expect_update_calculated_channel().returning(|_| {
        Ok(Response::new(UpdateCalculatedChannelResponse {
            calculated_channel: None,
            inapplicable_assets: vec![],
        }))
    });

    let (service, _h) = service_with_mock(mock).await;

    let err = service
        .archive_calculated_channel("cc1".to_string())
        .await
        .expect_err("expected error");

    // `{:#}` renders the whole context chain; archive wraps the shared update path.
    assert!(
        format!("{err:#}")
            .contains("update_calculated_channel response missing calculated channel")
    );
}

#[tokio::test]
async fn archive_calculated_channel_stamps_a_current_timestamp() {
    let mut mock = MockCalculatedChannelServiceImpl::new();
    mock.expect_update_calculated_channel().returning(|req| {
        Ok(Response::new(UpdateCalculatedChannelResponse {
            calculated_channel: req.into_inner().calculated_channel,
            inapplicable_assets: vec![],
        }))
    });

    let (service, _h) = service_with_mock(mock).await;

    let written = service
        .archive_calculated_channel("cc1".to_string())
        .await
        .expect("archive_calculated_channel failed");

    let Timestamp { seconds, .. } = written
        .calculated_channel
        .archived_date
        .expect("archived_date stamped");
    // Sanity floor: 2020-01-01T00:00:00Z. Guards against a zero-valued default
    // being sent as the archive timestamp.
    assert!(seconds > 1_577_836_800);
}

/// Helper: a stored calculated channel carrying only the fields the resolution
/// path reads.
fn stored_channel(id: &str, name: &str) -> CalculatedChannel {
    CalculatedChannel {
        calculated_channel_id: id.into(),
        name: name.into(),
        ..Default::default()
    }
}

fn expression_request(expression: &str, channel_id: &str) -> ExpressionRequest {
    ExpressionRequest {
        expression: expression.into(),
        expression_channel_references: vec![ExpressionChannelReference {
            channel_reference: "$1".into(),
            channel_id: channel_id.into(),
            calculated_channel_reference: None,
        }],
        ..Default::default()
    }
}

#[tokio::test]
async fn resolve_calculated_channels_builds_lookup_and_resolve_requests() {
    let mut mock = MockCalculatedChannelServiceImpl::new();
    mock.expect_list_calculated_channels()
        .times(1)
        .withf(|req| {
            let filter = &req.get_ref().filter;
            filter.contains("is_archived == false")
                && filter.contains("name in [\"thrust_margin\", \"chamber_delta\"]")
        })
        .returning(|_| {
            Ok(Response::new(ListCalculatedChannelsResponse {
                calculated_channels: vec![
                    stored_channel("cc2", "chamber_delta"),
                    stored_channel("cc1", "thrust_margin"),
                ],
                next_page_token: String::new(),
            }))
        });
    mock.expect_batch_resolve_calculated_channels()
        .times(1)
        .withf(|req| {
            let requests = &req.get_ref().requests;
            if requests.len() != 2 {
                return false;
            }
            // Requests keep the caller's name order so responses can be mapped
            // back by index.
            let ids = requests
                .iter()
                .map(|r| match r.calculated_channel.as_ref() {
                    Some(ResolveTarget::Identifier(ResourceIdentifier {
                        identifier: Some(Identifier::Id(id)),
                    })) => id.clone(),
                    _ => String::new(),
                })
                .collect::<Vec<_>>();
            let assets = match requests[0].assets.as_ref().and_then(|a| a.resources.as_ref()) {
                Some(Resources::Ids(ids)) => ids.ids.clone(),
                _ => Vec::new(),
            };
            let run = match requests[0].run.as_ref().and_then(|r| r.identifier.as_ref()) {
                Some(Identifier::Id(id)) => id.clone(),
                _ => String::new(),
            };
            ids == vec!["cc1".to_string(), "cc2".to_string()]
                && assets == vec!["asset-1".to_string()]
                && run == "run-1"
        })
        .returning(|req| {
            let responses = req
                .into_inner()
                .requests
                .into_iter()
                .enumerate()
                .map(|(i, _)| ResolveCalculatedChannelResponse {
                    calculated_channel_id: None,
                    resolved: vec![ResolvedCalculatedChannel {
                        asset_name: "bench".into(),
                        asset_id: "asset-1".into(),
                        expression_request: Some(expression_request(
                            "$1 * 2",
                            &format!("ch-{i}"),
                        )),
                        output_data_type: 0,
                    }],
                    unresolved: vec![],
                })
                .collect();
            Ok(Response::new(BatchResolveCalculatedChannelsResponse { responses }))
        });

    let (service, _h) = service_with_mock(mock).await;

    let resolution = service
        .resolve_calculated_channels(
            vec!["thrust_margin".to_string(), "chamber_delta".to_string()],
            "asset-1".to_string(),
            Some("run-1".to_string()),
        )
        .await
        .expect("resolve_calculated_channels failed");

    assert!(resolution.unresolved.is_empty());
    assert_eq!(
        resolution
            .resolved
            .iter()
            .map(|r| r.name.as_str())
            .collect::<Vec<_>>(),
        vec!["thrust_margin", "chamber_delta"],
    );
    assert_eq!(resolution.resolved[0].asset_name, "bench");
    assert_eq!(
        resolution.resolved[0]
            .expression_request
            .expression_channel_references[0]
            .channel_id,
        "ch-0",
    );
}

#[tokio::test]
async fn resolve_calculated_channels_omits_run_when_absent() {
    let mut mock = MockCalculatedChannelServiceImpl::new();
    mock.expect_list_calculated_channels().returning(|_| {
        Ok(Response::new(ListCalculatedChannelsResponse {
            calculated_channels: vec![stored_channel("cc1", "thrust_margin")],
            next_page_token: String::new(),
        }))
    });
    mock.expect_batch_resolve_calculated_channels()
        .times(1)
        .withf(|req| req.get_ref().requests[0].run.is_none())
        .returning(|_| {
            Ok(Response::new(BatchResolveCalculatedChannelsResponse {
                responses: vec![ResolveCalculatedChannelResponse {
                    calculated_channel_id: None,
                    resolved: vec![ResolvedCalculatedChannel {
                        asset_name: "bench".into(),
                        asset_id: "asset-1".into(),
                        expression_request: Some(expression_request("$1", "ch-0")),
                        output_data_type: 0,
                    }],
                    unresolved: vec![],
                }],
            }))
        });

    let (service, _h) = service_with_mock(mock).await;

    let resolution = service
        .resolve_calculated_channels(
            vec!["thrust_margin".to_string()],
            "asset-1".to_string(),
            None,
        )
        .await
        .expect("resolve_calculated_channels failed");

    assert_eq!(resolution.resolved.len(), 1);
}

/// One requested channel applies to the asset and one does not. The
/// inapplicable one must come back named, with the API's reason, instead of
/// being dropped so the caller only sees the applicable channel.
#[tokio::test]
async fn resolve_calculated_channels_reports_inapplicable_channel() {
    let mut mock = MockCalculatedChannelServiceImpl::new();
    mock.expect_list_calculated_channels().returning(|_| {
        Ok(Response::new(ListCalculatedChannelsResponse {
            calculated_channels: vec![
                stored_channel("cc1", "thrust_margin"),
                stored_channel("cc2", "chamber_delta"),
            ],
            next_page_token: String::new(),
        }))
    });
    mock.expect_batch_resolve_calculated_channels()
        .returning(|_| {
            Ok(Response::new(BatchResolveCalculatedChannelsResponse {
                responses: vec![
                    ResolveCalculatedChannelResponse {
                        calculated_channel_id: None,
                        resolved: vec![ResolvedCalculatedChannel {
                            asset_name: "bench".into(),
                            asset_id: "asset-1".into(),
                            expression_request: Some(expression_request("$1", "ch-0")),
                            output_data_type: 0,
                        }],
                        unresolved: vec![],
                    },
                    ResolveCalculatedChannelResponse {
                        calculated_channel_id: None,
                        resolved: vec![],
                        unresolved: vec![UnresolvedCalculatedChannel {
                            asset_name: "bench".into(),
                            error_message: "asset is missing channel chamber_pressure".into(),
                        }],
                    },
                ],
            }))
        });

    let (service, _h) = service_with_mock(mock).await;

    let resolution = service
        .resolve_calculated_channels(
            vec!["thrust_margin".to_string(), "chamber_delta".to_string()],
            "asset-1".to_string(),
            Some("run-1".to_string()),
        )
        .await
        .expect("resolve_calculated_channels failed");

    assert_eq!(resolution.resolved.len(), 1);
    assert_eq!(resolution.resolved[0].name, "thrust_margin");
    assert_eq!(resolution.unresolved.len(), 1);
    assert_eq!(resolution.unresolved[0].name, "chamber_delta");
    assert!(
        resolution.unresolved[0]
            .reason
            .contains("missing channel chamber_pressure"),
        "reason should carry the API message: {}",
        resolution.unresolved[0].reason,
    );
}

/// A resolution that returns only other assets' entries does not apply to the
/// requested asset, so it must be reported instead of queried.
#[tokio::test]
async fn resolve_calculated_channels_rejects_other_asset_resolution() {
    let mut mock = MockCalculatedChannelServiceImpl::new();
    mock.expect_list_calculated_channels().returning(|_| {
        Ok(Response::new(ListCalculatedChannelsResponse {
            calculated_channels: vec![stored_channel("cc1", "thrust_margin")],
            next_page_token: String::new(),
        }))
    });
    mock.expect_batch_resolve_calculated_channels()
        .returning(|_| {
            Ok(Response::new(BatchResolveCalculatedChannelsResponse {
                responses: vec![ResolveCalculatedChannelResponse {
                    calculated_channel_id: None,
                    resolved: vec![ResolvedCalculatedChannel {
                        asset_name: "other-bench".into(),
                        asset_id: "asset-9".into(),
                        expression_request: Some(expression_request("$1", "ch-9")),
                        output_data_type: 0,
                    }],
                    unresolved: vec![],
                }],
            }))
        });

    let (service, _h) = service_with_mock(mock).await;

    let resolution = service
        .resolve_calculated_channels(
            vec!["thrust_margin".to_string()],
            "asset-1".to_string(),
            None,
        )
        .await
        .expect("resolve_calculated_channels failed");

    assert!(resolution.resolved.is_empty());
    assert_eq!(resolution.unresolved.len(), 1);
    assert_eq!(resolution.unresolved[0].name, "thrust_margin");
}

/// A name with no stored calculated channel never reaches the resolve RPC and
/// comes back as unresolved with a reason the caller can act on.
#[tokio::test]
async fn resolve_calculated_channels_flags_unknown_name() {
    let mut mock = MockCalculatedChannelServiceImpl::new();
    mock.expect_list_calculated_channels().returning(|_| {
        Ok(Response::new(ListCalculatedChannelsResponse {
            calculated_channels: vec![],
            next_page_token: String::new(),
        }))
    });
    mock.expect_batch_resolve_calculated_channels().times(0);

    let (service, _h) = service_with_mock(mock).await;

    let resolution = service
        .resolve_calculated_channels(
            vec!["not_a_channel".to_string()],
            "asset-1".to_string(),
            None,
        )
        .await
        .expect("resolve_calculated_channels failed");

    assert!(resolution.resolved.is_empty());
    assert_eq!(resolution.unresolved.len(), 1);
    assert_eq!(resolution.unresolved[0].name, "not_a_channel");
    assert!(
        resolution.unresolved[0].reason.contains("no active saved"),
        "reason should say the name is unknown: {}",
        resolution.unresolved[0].reason,
    );
}

#[tokio::test]
async fn resolve_calculated_channels_propagates_grpc_error() {
    let mut mock = MockCalculatedChannelServiceImpl::new();
    mock.expect_list_calculated_channels().returning(|_| {
        Ok(Response::new(ListCalculatedChannelsResponse {
            calculated_channels: vec![stored_channel("cc1", "thrust_margin")],
            next_page_token: String::new(),
        }))
    });
    mock.expect_batch_resolve_calculated_channels()
        .returning(|_| Err(Status::permission_denied("no access")));

    let (service, _h) = service_with_mock(mock).await;

    let err = service
        .resolve_calculated_channels(
            vec!["thrust_margin".to_string()],
            "asset-1".to_string(),
            None,
        )
        .await
        .expect_err("expected the gRPC error to propagate");

    assert!(err.to_string().contains("failed to resolve calculated channels"));
}

/// A response count that does not line up with the requests would silently
/// mis-assign expressions to names, so it must fail loudly.
#[tokio::test]
async fn resolve_calculated_channels_errors_on_response_count_mismatch() {
    let mut mock = MockCalculatedChannelServiceImpl::new();
    mock.expect_list_calculated_channels().returning(|_| {
        Ok(Response::new(ListCalculatedChannelsResponse {
            calculated_channels: vec![
                stored_channel("cc1", "thrust_margin"),
                stored_channel("cc2", "chamber_delta"),
            ],
            next_page_token: String::new(),
        }))
    });
    mock.expect_batch_resolve_calculated_channels()
        .returning(|_| {
            Ok(Response::new(BatchResolveCalculatedChannelsResponse {
                responses: vec![ResolveCalculatedChannelResponse {
                    calculated_channel_id: None,
                    resolved: vec![],
                    unresolved: vec![],
                }],
            }))
        });

    let (service, _h) = service_with_mock(mock).await;

    let err = service
        .resolve_calculated_channels(
            vec!["thrust_margin".to_string(), "chamber_delta".to_string()],
            "asset-1".to_string(),
            None,
        )
        .await
        .expect_err("expected a mismatched response count to error");

    assert!(err.to_string().contains("resolve"));
}
