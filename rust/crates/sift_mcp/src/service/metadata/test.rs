use sift_rs::metadata::v1::{
    ListMetadataKeysResponse, ListMetadataUsageResponse, ListMetadataValuesResponse, MetadataKey,
    MetadataUsage, MetadataValue, metadata_service_server::MetadataServiceServer,
};
use sift_test_util::{grpc::memory_sift_channel, mock::metadata::v1::MockMetadataServiceImpl};
use tokio::task::JoinHandle;
use tonic::{Response, Status, transport::Server};

use super::MetadataService;
use crate::policy::RetryPolicy;
use crate::service::common::DEFAULT_LIMIT;

async fn service_with_mock(mock: MockMetadataServiceImpl) -> (MetadataService, JoinHandle<()>) {
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
        MetadataService::new(channel, RetryPolicy::default()),
        handle,
    )
}

fn key(name: &str) -> MetadataKey {
    MetadataKey {
        name: name.into(),
        ..Default::default()
    }
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

    let (service, _h) = service_with_mock(mock).await;

    let keys = service
        .list_metadata_keys("name == \"vehicle_type\"".to_string(), None, None)
        .await
        .expect("list_metadata_keys failed");

    assert_eq!(keys.len(), 2);
    assert_eq!(keys[0].name, "vehicle_type");
    assert_eq!(keys[1].name, "mission");
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

    let (service, _h) = service_with_mock(mock).await;

    let keys = service
        .list_metadata_keys(String::new(), None, None)
        .await
        .expect("list_metadata_keys failed");

    let names: Vec<&str> = keys.iter().map(|k| k.name.as_str()).collect();
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

    let (service, _h) = service_with_mock(mock).await;

    let keys = service
        .list_metadata_keys(String::new(), None, Some(2))
        .await
        .expect("list_metadata_keys failed");

    assert_eq!(keys.len(), 2);
}

#[tokio::test]
async fn list_metadata_keys_propagates_grpc_error() {
    let mut mock = MockMetadataServiceImpl::new();
    mock.expect_list_metadata_keys()
        .returning(|_| Err(Status::invalid_argument("bad filter")));

    let (service, _h) = service_with_mock(mock).await;

    let err = service
        .list_metadata_keys("nope".to_string(), None, None)
        .await
        .expect_err("expected error");

    assert!(err.to_string().contains("failed to query metadata keys"));
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

    let (service, _h) = service_with_mock(mock).await;

    let values = service
        .list_metadata_values(String::new(), None, None, "vehicle_type".to_string())
        .await
        .expect("list_metadata_values failed");

    assert_eq!(values.len(), 2);
}

#[tokio::test]
async fn list_metadata_values_paginates_until_token_empty() {
    let mut mock = MockMetadataServiceImpl::new();
    mock.expect_list_metadata_values().returning(|req| {
        let req = req.into_inner();
        assert_eq!(req.page_size, DEFAULT_LIMIT);
        assert_eq!(req.metadata_key_name, "vehicle_type");
        let (values, next) = match req.page_token.as_str() {
            "" => (
                vec![MetadataValue {
                    key: Some(key("vehicle_type")),
                    ..Default::default()
                }],
                "page-2".to_string(),
            ),
            "page-2" => (
                vec![MetadataValue {
                    key: Some(key("vehicle_type")),
                    ..Default::default()
                }],
                String::new(),
            ),
            other => return Err(Status::invalid_argument(format!("bad token: {other}"))),
        };
        Ok(Response::new(ListMetadataValuesResponse {
            metadata_values: values,
            next_page_token: next,
        }))
    });

    let (service, _h) = service_with_mock(mock).await;

    let values = service
        .list_metadata_values(String::new(), None, None, "vehicle_type".to_string())
        .await
        .expect("list_metadata_values failed");

    assert_eq!(values.len(), 2);
}

#[tokio::test]
async fn list_metadata_values_respects_limit() {
    let mut mock = MockMetadataServiceImpl::new();
    mock.expect_list_metadata_values()
        .times(1)
        .returning(|req| {
            let req = req.into_inner();
            assert_eq!(req.page_size, 1);
            Ok(Response::new(ListMetadataValuesResponse {
                metadata_values: vec![MetadataValue {
                    key: Some(key("vehicle_type")),
                    ..Default::default()
                }],
                next_page_token: "page-2".into(),
            }))
        });

    let (service, _h) = service_with_mock(mock).await;

    let values = service
        .list_metadata_values(String::new(), None, Some(1), "vehicle_type".to_string())
        .await
        .expect("list_metadata_values failed");

    assert_eq!(values.len(), 1);
}

#[tokio::test]
async fn list_metadata_values_propagates_grpc_error() {
    let mut mock = MockMetadataServiceImpl::new();
    mock.expect_list_metadata_values()
        .returning(|_| Err(Status::not_found("unknown key")));

    let (service, _h) = service_with_mock(mock).await;

    let err = service
        .list_metadata_values(String::new(), None, None, "nope".to_string())
        .await
        .expect_err("expected error");

    assert!(err.to_string().contains("failed to query metadata values"));
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

    let (service, _h) = service_with_mock(mock).await;

    let usages = service
        .list_metadata_usage("key_name == \"vehicle_type\"".to_string(), None, None)
        .await
        .expect("list_metadata_usage failed");

    assert_eq!(usages.len(), 2);
    assert_eq!(usages[0].entity_id, "a1");
    assert_eq!(usages[1].entity_id, "r1");
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

    let (service, _h) = service_with_mock(mock).await;

    let usages = service
        .list_metadata_usage(String::new(), None, None)
        .await
        .expect("list_metadata_usage failed");

    let ids: Vec<&str> = usages.iter().map(|u| u.entity_id.as_str()).collect();
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

    let (service, _h) = service_with_mock(mock).await;

    let usages = service
        .list_metadata_usage(String::new(), None, Some(2))
        .await
        .expect("list_metadata_usage failed");

    assert_eq!(usages.len(), 2);
}

#[tokio::test]
async fn list_metadata_usage_propagates_grpc_error() {
    let mut mock = MockMetadataServiceImpl::new();
    mock.expect_list_metadata_usage()
        .returning(|_| Err(Status::invalid_argument("bad filter")));

    let (service, _h) = service_with_mock(mock).await;

    let err = service
        .list_metadata_usage("nope".to_string(), None, None)
        .await
        .expect_err("expected error");

    assert!(err.to_string().contains("failed to query metadata usage"));
}
