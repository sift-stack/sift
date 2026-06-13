use sift_rs::calculated_channels::v2::{
    CalculatedChannel, CalculatedChannelAbstractChannelReference, CalculatedChannelConfiguration,
    CalculatedChannelQueryConfiguration, CreateCalculatedChannelResponse,
    GetCalculatedChannelResponse, ListCalculatedChannelsResponse, UpdateCalculatedChannelResponse,
    calculated_channel_query_configuration::{Query, Sel},
    calculated_channel_service_server::CalculatedChannelServiceServer,
};
use sift_rs::metadata::v1::MetadataValue;
use sift_test_util::{
    grpc::memory_sift_channel, mock::calculated_channels::v2::MockCalculatedChannelServiceImpl,
};
use tokio::task::JoinHandle;
use tonic::{Response, Status, transport::Server};

use super::{CalculatedChannelService, CalculatedChannelUpdate};
use crate::service::common::PAGE_SIZE;

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

    (CalculatedChannelService::new(channel), handle)
}

/// Build a channel whose query configuration holds the given expression and refs.
fn channel_with_sel(id: &str, expression: &str, refs: Vec<&str>) -> CalculatedChannel {
    CalculatedChannel {
        calculated_channel_id: id.into(),
        name: "keep".into(),
        calculated_channel_configuration: Some(CalculatedChannelConfiguration {
            asset_configuration: None,
            query_configuration: Some(CalculatedChannelQueryConfiguration {
                query: Some(Query::Sel(Sel {
                    expression: expression.into(),
                    expression_channel_references: refs
                        .into_iter()
                        .map(|r| CalculatedChannelAbstractChannelReference {
                            channel_reference: r.into(),
                            ..Default::default()
                        })
                        .collect(),
                })),
            }),
        }),
        ..Default::default()
    }
}

#[tokio::test]
async fn list_truncates_to_limit_across_pages() {
    let mut mock = MockCalculatedChannelServiceImpl::new();
    mock.expect_list_calculated_channels().returning(|req| {
        let req = req.into_inner();
        assert_eq!(req.page_size, 3);
        let (channels, next) = match req.page_token.as_str() {
            "" => (
                vec![
                    CalculatedChannel {
                        calculated_channel_id: "c1".into(),
                        ..Default::default()
                    },
                    CalculatedChannel {
                        calculated_channel_id: "c2".into(),
                        ..Default::default()
                    },
                ],
                "page-2".to_string(),
            ),
            "page-2" => (
                vec![
                    CalculatedChannel {
                        calculated_channel_id: "c3".into(),
                        ..Default::default()
                    },
                    CalculatedChannel {
                        calculated_channel_id: "c4".into(),
                        ..Default::default()
                    },
                ],
                String::new(),
            ),
            other => return Err(Status::invalid_argument(format!("bad token: {other}"))),
        };
        Ok(Response::new(ListCalculatedChannelsResponse {
            calculated_channels: channels,
            next_page_token: next,
        }))
    });

    let (service, _h) = service_with_mock(mock).await;

    let channels = service
        .list_calculated_channels(String::new(), None, Some(3))
        .await
        .expect("list failed");

    let ids: Vec<&str> = channels
        .iter()
        .map(|c| c.calculated_channel_id.as_str())
        .collect();
    assert_eq!(ids, vec!["c1", "c2", "c3"]);
}

#[tokio::test]
async fn list_unbounded_uses_page_size_cap() {
    let mut mock = MockCalculatedChannelServiceImpl::new();
    mock.expect_list_calculated_channels()
        .times(1)
        .returning(|req| {
            assert_eq!(req.get_ref().page_size, PAGE_SIZE);
            Ok(Response::new(ListCalculatedChannelsResponse {
                calculated_channels: vec![CalculatedChannel {
                    calculated_channel_id: "c1".into(),
                    ..Default::default()
                }],
                next_page_token: String::new(),
            }))
        });

    let (service, _h) = service_with_mock(mock).await;

    let channels = service
        .list_calculated_channels(String::new(), None, None)
        .await
        .expect("list failed");

    assert_eq!(channels.len(), 1);
}

#[tokio::test]
async fn list_propagates_grpc_error() {
    let mut mock = MockCalculatedChannelServiceImpl::new();
    mock.expect_list_calculated_channels()
        .returning(|_| Err(Status::invalid_argument("bad filter")));

    let (service, _h) = service_with_mock(mock).await;

    let err = service
        .list_calculated_channels("nope".into(), None, None)
        .await
        .expect_err("expected error");

    assert!(
        err.to_string()
            .contains("failed to query calculated channels")
    );
}

#[tokio::test]
async fn get_by_id_returns_channel() {
    let mut mock = MockCalculatedChannelServiceImpl::new();
    mock.expect_get_calculated_channel()
        .withf(|req| {
            let req = req.get_ref();
            req.calculated_channel_id == "c1" && req.client_key.is_empty()
        })
        .returning(|_| {
            Ok(Response::new(GetCalculatedChannelResponse {
                calculated_channel: Some(CalculatedChannel {
                    calculated_channel_id: "c1".into(),
                    name: "speed".into(),
                    ..Default::default()
                }),
            }))
        });

    let (service, _h) = service_with_mock(mock).await;

    let channel = service
        .get_calculated_channel("c1".into(), String::new())
        .await
        .expect("get failed");

    assert_eq!(channel.name, "speed");
}

#[tokio::test]
async fn get_missing_channel_is_not_found() {
    let mut mock = MockCalculatedChannelServiceImpl::new();
    mock.expect_get_calculated_channel().returning(|_| {
        Ok(Response::new(GetCalculatedChannelResponse {
            calculated_channel: None,
        }))
    });

    let (service, _h) = service_with_mock(mock).await;

    let err = service
        .get_calculated_channel("missing".into(), String::new())
        .await
        .expect_err("expected not found");

    let status = err.downcast::<Status>().expect("expected tonic Status");
    assert_eq!(status.code(), tonic::Code::NotFound);
}

#[tokio::test]
async fn create_sends_configuration_and_returns_channel() {
    let mut mock = MockCalculatedChannelServiceImpl::new();
    mock.expect_create_calculated_channel()
        .withf(|req| {
            let req = req.get_ref();
            req.name == "derived" && req.calculated_channel_configuration.is_some()
        })
        .returning(|req| {
            let req = req.into_inner();
            Ok(Response::new(CreateCalculatedChannelResponse {
                calculated_channel: Some(CalculatedChannel {
                    calculated_channel_id: "c9".into(),
                    name: req.name,
                    ..Default::default()
                }),
                inapplicable_assets: vec![],
            }))
        });

    let (service, _h) = service_with_mock(mock).await;

    let resp = service
        .create_calculated_channel(
            "derived".into(),
            String::new(),
            None,
            None,
            String::new(),
            CalculatedChannelConfiguration::default(),
            vec![],
        )
        .await
        .expect("create failed");

    assert_eq!(resp.calculated_channel.unwrap().calculated_channel_id, "c9");
}

#[tokio::test]
async fn update_expression_only_preserves_existing_references() {
    let mut mock = MockCalculatedChannelServiceImpl::new();
    mock.expect_get_calculated_channel().returning(|_| {
        Ok(Response::new(GetCalculatedChannelResponse {
            calculated_channel: Some(channel_with_sel("c1", "$1 + 1", vec!["$1"])),
        }))
    });
    mock.expect_update_calculated_channel()
        .withf(|req| {
            let req = req.get_ref();
            let mask = req.update_mask.as_ref().unwrap();
            let channel = req.calculated_channel.as_ref().unwrap();
            let sel = match channel
                .calculated_channel_configuration
                .as_ref()
                .and_then(|c| c.query_configuration.as_ref())
                .and_then(|q| q.query.as_ref())
            {
                Some(Query::Sel(sel)) => sel,
                None => return false,
            };
            // expression replaced; the original $1 reference carried through.
            mask.paths == vec!["query_configuration".to_string()]
                && sel.expression == "$1 * 2"
                && sel.expression_channel_references.len() == 1
                && sel.expression_channel_references[0].channel_reference == "$1"
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
            "c1".into(),
            String::new(),
            CalculatedChannelUpdate {
                expression: Some("$1 * 2".into()),
                ..Default::default()
            },
        )
        .await
        .expect("update failed");
}

#[tokio::test]
async fn update_with_no_fields_is_invalid() {
    let mut mock = MockCalculatedChannelServiceImpl::new();
    mock.expect_get_calculated_channel().returning(|_| {
        Ok(Response::new(GetCalculatedChannelResponse {
            calculated_channel: Some(CalculatedChannel {
                calculated_channel_id: "c1".into(),
                ..Default::default()
            }),
        }))
    });
    mock.expect_update_calculated_channel().never();

    let (service, _h) = service_with_mock(mock).await;

    let err = service
        .update_calculated_channel(
            "c1".into(),
            String::new(),
            CalculatedChannelUpdate::default(),
        )
        .await
        .expect_err("expected invalid argument");

    let status = err.downcast::<Status>().expect("expected tonic Status");
    assert_eq!(status.code(), tonic::Code::InvalidArgument);
}

#[tokio::test]
async fn update_references_only_preserves_existing_expression() {
    let mut mock = MockCalculatedChannelServiceImpl::new();
    mock.expect_get_calculated_channel().returning(|_| {
        Ok(Response::new(GetCalculatedChannelResponse {
            calculated_channel: Some(channel_with_sel("c1", "$1 + $2", vec!["$1"])),
        }))
    });
    mock.expect_update_calculated_channel()
        .withf(|req| {
            let req = req.get_ref();
            let mask = req.update_mask.as_ref().unwrap();
            let channel = req.calculated_channel.as_ref().unwrap();
            let sel = match channel
                .calculated_channel_configuration
                .as_ref()
                .and_then(|c| c.query_configuration.as_ref())
                .and_then(|q| q.query.as_ref())
            {
                Some(Query::Sel(sel)) => sel,
                None => return false,
            };
            // references replaced; the original expression carried through unchanged.
            mask.paths == vec!["query_configuration".to_string()]
                && sel.expression == "$1 + $2"
                && sel.expression_channel_references.len() == 2
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
            "c1".into(),
            String::new(),
            CalculatedChannelUpdate {
                channel_references: Some(vec![
                    CalculatedChannelAbstractChannelReference {
                        channel_reference: "$1".into(),
                        ..Default::default()
                    },
                    CalculatedChannelAbstractChannelReference {
                        channel_reference: "$2".into(),
                        ..Default::default()
                    },
                ]),
                ..Default::default()
            },
        )
        .await
        .expect("update failed");
}

#[tokio::test]
async fn update_scalar_and_asset_scope_branches_set_expected_mask() {
    use sift_rs::calculated_channels::v2::{
        CalculatedChannelAssetConfiguration, calculated_channel_asset_configuration::AssetScope,
    };

    let mut mock = MockCalculatedChannelServiceImpl::new();
    mock.expect_get_calculated_channel().returning(|_| {
        Ok(Response::new(GetCalculatedChannelResponse {
            calculated_channel: Some(CalculatedChannel {
                calculated_channel_id: "c1".into(),
                name: "old".into(),
                ..Default::default()
            }),
        }))
    });
    mock.expect_update_calculated_channel()
        .withf(|req| {
            let req = req.get_ref();
            let channel = req.calculated_channel.as_ref().unwrap();
            let mask = req.update_mask.as_ref().unwrap();
            let scope_all = matches!(
                channel
                    .calculated_channel_configuration
                    .as_ref()
                    .and_then(|c| c.asset_configuration.as_ref())
                    .and_then(|a| a.asset_scope.as_ref()),
                Some(AssetScope::AllAssets(true))
            );
            channel.name == "new"
                && channel.units.as_deref() == Some("rpm")
                && channel.metadata.len() == 1
                && scope_all
                && mask.paths
                    == vec![
                        "name".to_string(),
                        "units".to_string(),
                        "metadata".to_string(),
                        "asset_configuration".to_string(),
                    ]
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
            "c1".into(),
            String::new(),
            CalculatedChannelUpdate {
                name: Some("new".into()),
                units: Some("rpm".into()),
                metadata: Some(vec![MetadataValue::default()]),
                asset_configuration: Some(CalculatedChannelAssetConfiguration {
                    asset_scope: Some(AssetScope::AllAssets(true)),
                }),
                ..Default::default()
            },
        )
        .await
        .expect("update failed");
}
