use rmcp::{handler::server::wrapper::Parameters, model::ErrorCode};
use sift_rs::metadata::v1::{
    ListMetadataKeysResponse, ListMetadataUsageResponse, ListMetadataValuesResponse, MetadataKey,
    MetadataUsage, MetadataValue, metadata_service_server::MetadataServiceServer,
};
use sift_test_util::{grpc::memory_sift_channel, mock::metadata::v1::MockMetadataServiceImpl};
use tokio::task::JoinHandle;
use tonic::{Response, Status, transport::Server};

use super::ListMetadataValuesParams;
use crate::{
    server::SiftMcpServer,
    service::common::DEFAULT_LIMIT,
    tool::common::test_support::{list_params, structured_field},
};

fn key(name: &str) -> MetadataKey {
    MetadataKey {
        name: name.into(),
        ..Default::default()
    }
}

fn values_params(metadata_key_name: &str) -> Parameters<ListMetadataValuesParams> {
    Parameters(ListMetadataValuesParams {
        filter: String::new(),
        order_by: None,
        limit: None,
        metadata_key_name: metadata_key_name.into(),
    })
}

async fn server_with_mock(mock: MockMetadataServiceImpl) -> (SiftMcpServer, JoinHandle<()>) {
    let (client, server) = tokio::io::duplex(1024);
    let channel = memory_sift_channel(client).await;

    let handle = tokio::spawn(async move {
        Server::builder()
            .add_service(MetadataServiceServer::new(mock))
            .serve_with_incoming(tokio_stream::once(Ok::<_, std::io::Error>(server)))
            .await
            .unwrap();
    });

    (
        SiftMcpServer::new(channel, String::from("https://app.test.local"), true, true),
        handle,
    )
}

// --- list_metadata_keys ---

#[tokio::test]
async fn list_metadata_keys_returns_single_page() {
    let mut mock = MockMetadataServiceImpl::new();
    mock.expect_list_metadata_keys()
        .withf(|req| req.get_ref().filter == "name == \"vehicle_type\"")
        .returning(|_| {
            Ok(Response::new(ListMetadataKeysResponse {
                metadata_keys: vec![key("vehicle_type"), key("mission")],
                next_page_token: String::new(),
            }))
        });

    let (server, _h) = server_with_mock(mock).await;

    let resp = server
        .list_metadata_keys(list_params("name == \"vehicle_type\"", None))
        .await
        .expect("list_metadata_keys failed");

    let keys = structured_field(resp, "metadata_keys");
    assert_eq!(keys.as_array().unwrap().len(), 2);
    assert_eq!(keys[0]["name"], "vehicle_type");
}

#[tokio::test]
async fn list_metadata_keys_paginates_until_token_empty() {
    let mut mock = MockMetadataServiceImpl::new();
    mock.expect_list_metadata_keys().returning(|req| {
        let req = req.into_inner();
        assert_eq!(req.page_size, DEFAULT_LIMIT);
        let (keys, next) = match req.page_token.as_str() {
            "" => (vec![key("k1")], "page-2".to_string()),
            "page-2" => (vec![key("k2")], String::new()),
            other => return Err(Status::invalid_argument(format!("bad token: {other}"))),
        };
        Ok(Response::new(ListMetadataKeysResponse {
            metadata_keys: keys,
            next_page_token: next,
        }))
    });

    let (server, _h) = server_with_mock(mock).await;

    let resp = server
        .list_metadata_keys(list_params("", None))
        .await
        .expect("list_metadata_keys failed");

    let keys = structured_field(resp, "metadata_keys");
    let names: Vec<&str> = keys
        .as_array()
        .unwrap()
        .iter()
        .map(|k| k["name"].as_str().unwrap())
        .collect();
    assert_eq!(names, vec!["k1", "k2"]);
}

#[tokio::test]
async fn list_metadata_keys_respects_limit() {
    let mut mock = MockMetadataServiceImpl::new();
    mock.expect_list_metadata_keys().times(1).returning(|req| {
        let req = req.into_inner();
        assert_eq!(req.page_size, 2);
        Ok(Response::new(ListMetadataKeysResponse {
            metadata_keys: vec![key("k1"), key("k2")],
            next_page_token: "page-2".into(),
        }))
    });

    let (server, _h) = server_with_mock(mock).await;

    let resp = server
        .list_metadata_keys(list_params("", Some(2)))
        .await
        .expect("list_metadata_keys failed");

    assert_eq!(
        structured_field(resp, "metadata_keys")
            .as_array()
            .unwrap()
            .len(),
        2
    );
}

#[tokio::test]
async fn list_metadata_keys_propagates_grpc_error() {
    let mut mock = MockMetadataServiceImpl::new();
    mock.expect_list_metadata_keys()
        .returning(|_| Err(Status::invalid_argument("bad filter")));

    let (server, _h) = server_with_mock(mock).await;

    let err = server
        .list_metadata_keys(list_params("nope", None))
        .await
        .expect_err("expected error");

    assert_eq!(err.code, ErrorCode::INVALID_PARAMS);
    assert!(err.message.contains("bad filter"));
}

// --- list_metadata_values ---

#[tokio::test]
async fn list_metadata_values_returns_single_page() {
    let mut mock = MockMetadataServiceImpl::new();
    mock.expect_list_metadata_values()
        .withf(|req| req.get_ref().metadata_key_name == "vehicle_type")
        .returning(|_| {
            Ok(Response::new(ListMetadataValuesResponse {
                metadata_values: vec![
                    MetadataValue {
                        key: Some(key("vehicle_type")),
                        ..Default::default()
                    },
                    MetadataValue {
                        key: Some(key("vehicle_type")),
                        ..Default::default()
                    },
                ],
                next_page_token: String::new(),
            }))
        });

    let (server, _h) = server_with_mock(mock).await;

    let resp = server
        .list_metadata_values(values_params("vehicle_type"))
        .await
        .expect("list_metadata_values failed");

    assert_eq!(
        structured_field(resp, "metadata_values")
            .as_array()
            .unwrap()
            .len(),
        2
    );
}

#[tokio::test]
async fn list_metadata_values_empty_key_name_is_invalid_params() {
    let mock = MockMetadataServiceImpl::new();
    let (server, _h) = server_with_mock(mock).await;

    let err = server
        .list_metadata_values(values_params(""))
        .await
        .expect_err("expected error");

    assert_eq!(err.code, ErrorCode::INVALID_PARAMS);
}

#[tokio::test]
async fn list_metadata_values_propagates_grpc_error() {
    let mut mock = MockMetadataServiceImpl::new();
    mock.expect_list_metadata_values()
        .returning(|_| Err(Status::not_found("unknown key")));

    let (server, _h) = server_with_mock(mock).await;

    let err = server
        .list_metadata_values(values_params("nope"))
        .await
        .expect_err("expected error");

    assert_eq!(err.code, ErrorCode::RESOURCE_NOT_FOUND);
    assert!(err.message.contains("unknown key"));
}

// --- list_metadata_usage ---

#[tokio::test]
async fn list_metadata_usage_returns_single_page() {
    let mut mock = MockMetadataServiceImpl::new();
    mock.expect_list_metadata_usage()
        .withf(|req| req.get_ref().filter == "key_name == \"vehicle_type\"")
        .returning(|_| {
            Ok(Response::new(ListMetadataUsageResponse {
                metadata_usages: vec![
                    MetadataUsage {
                        entity_id: "a1".into(),
                        entity_type: "asset".into(),
                        value: Some(MetadataValue {
                            key: Some(key("vehicle_type")),
                            ..Default::default()
                        }),
                    },
                    MetadataUsage {
                        entity_id: "r1".into(),
                        entity_type: "run".into(),
                        value: Some(MetadataValue {
                            key: Some(key("vehicle_type")),
                            ..Default::default()
                        }),
                    },
                ],
                next_page_token: String::new(),
            }))
        });

    let (server, _h) = server_with_mock(mock).await;

    let resp = server
        .list_metadata_usage(list_params("key_name == \"vehicle_type\"", None))
        .await
        .expect("list_metadata_usage failed");

    let usages = structured_field(resp, "metadata_usages");
    assert_eq!(usages.as_array().unwrap().len(), 2);
    assert_eq!(usages[0]["entityId"], "a1");
    assert_eq!(usages[1]["entityId"], "r1");
}

#[tokio::test]
async fn list_metadata_usage_paginates_until_token_empty() {
    let mut mock = MockMetadataServiceImpl::new();
    mock.expect_list_metadata_usage().returning(|req| {
        let req = req.into_inner();
        assert_eq!(req.page_size, DEFAULT_LIMIT);
        let (usages, next) = match req.page_token.as_str() {
            "" => (
                vec![MetadataUsage {
                    entity_id: "a1".into(),
                    entity_type: "asset".into(),
                    value: None,
                }],
                "page-2".to_string(),
            ),
            "page-2" => (
                vec![MetadataUsage {
                    entity_id: "a2".into(),
                    entity_type: "asset".into(),
                    value: None,
                }],
                String::new(),
            ),
            other => return Err(Status::invalid_argument(format!("bad token: {other}"))),
        };
        Ok(Response::new(ListMetadataUsageResponse {
            metadata_usages: usages,
            next_page_token: next,
        }))
    });

    let (server, _h) = server_with_mock(mock).await;

    let resp = server
        .list_metadata_usage(list_params("", None))
        .await
        .expect("list_metadata_usage failed");

    let usages = structured_field(resp, "metadata_usages");
    let ids: Vec<&str> = usages
        .as_array()
        .unwrap()
        .iter()
        .map(|u| u["entityId"].as_str().unwrap())
        .collect();
    assert_eq!(ids, vec!["a1", "a2"]);
}

#[tokio::test]
async fn list_metadata_usage_respects_limit() {
    let mut mock = MockMetadataServiceImpl::new();
    mock.expect_list_metadata_usage().times(1).returning(|req| {
        let req = req.into_inner();
        assert_eq!(req.page_size, 2);
        Ok(Response::new(ListMetadataUsageResponse {
            metadata_usages: vec![
                MetadataUsage {
                    entity_id: "a1".into(),
                    entity_type: "asset".into(),
                    value: None,
                },
                MetadataUsage {
                    entity_id: "a2".into(),
                    entity_type: "asset".into(),
                    value: None,
                },
            ],
            next_page_token: "page-2".into(),
        }))
    });

    let (server, _h) = server_with_mock(mock).await;

    let resp = server
        .list_metadata_usage(list_params("", Some(2)))
        .await
        .expect("list_metadata_usage failed");

    assert_eq!(
        structured_field(resp, "metadata_usages")
            .as_array()
            .unwrap()
            .len(),
        2
    );
}

#[tokio::test]
async fn list_metadata_usage_propagates_grpc_error() {
    let mut mock = MockMetadataServiceImpl::new();
    mock.expect_list_metadata_usage()
        .returning(|_| Err(Status::invalid_argument("bad filter")));

    let (server, _h) = server_with_mock(mock).await;

    let err = server
        .list_metadata_usage(list_params("nope", None))
        .await
        .expect_err("expected error");

    assert_eq!(err.code, ErrorCode::INVALID_PARAMS);
    assert!(err.message.contains("bad filter"));
}
