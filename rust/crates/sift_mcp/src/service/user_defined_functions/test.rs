use sift_rs::common::r#type::v1::{FunctionDataType, FunctionInput, UserDefinedFunction};
use sift_rs::metadata::v1::{
    MetadataKey, MetadataKeyType, MetadataValue, metadata_value::Value as MetadataValueInner,
};
use sift_rs::user_defined_functions::v1::{
    CreateUserDefinedFunctionResponse, GetUserDefinedFunctionResponse,
    ListUserDefinedFunctionsResponse, UpdateUserDefinedFunctionResponse,
    user_defined_function_service_server::UserDefinedFunctionServiceServer,
};
use sift_test_util::{
    grpc::memory_sift_channel, mock::user_defined_functions::v1::MockUserDefinedFunctionServiceImpl,
};
use tokio::task::JoinHandle;
use tonic::{Response, Status, transport::Server};

use super::{UserDefinedFunctionService, UserDefinedFunctionUpdate};
use crate::service::common::PAGE_SIZE;

async fn service_with_mock(
    mock: MockUserDefinedFunctionServiceImpl,
) -> (UserDefinedFunctionService, JoinHandle<()>) {
    let (client, server) = tokio::io::duplex(1024);
    let channel = memory_sift_channel(client).await;

    let handle = tokio::spawn(async move {
        Server::builder()
            .add_service(UserDefinedFunctionServiceServer::new(mock))
            .serve_with_incoming(tokio_stream::once(Ok::<_, std::io::Error>(server)))
            .await
            .unwrap();
    });

    (UserDefinedFunctionService::new(channel), handle)
}

#[tokio::test]
async fn list_paginates_until_token_empty() {
    let mut mock = MockUserDefinedFunctionServiceImpl::new();
    mock.expect_list_user_defined_functions().returning(|req| {
        let req = req.into_inner();
        assert_eq!(req.page_size, PAGE_SIZE);
        let (udfs, next) = match req.page_token.as_str() {
            "" => (
                vec![UserDefinedFunction {
                    user_defined_function_id: "u1".into(),
                    ..Default::default()
                }],
                "page-2".to_string(),
            ),
            "page-2" => (
                vec![UserDefinedFunction {
                    user_defined_function_id: "u2".into(),
                    ..Default::default()
                }],
                String::new(),
            ),
            other => return Err(Status::invalid_argument(format!("bad token: {other}"))),
        };
        Ok(Response::new(ListUserDefinedFunctionsResponse {
            user_defined_functions: udfs,
            next_page_token: next,
        }))
    });

    let (service, _h) = service_with_mock(mock).await;

    let udfs = service
        .list_user_defined_functions(String::new(), None, None)
        .await
        .expect("list failed");

    let ids: Vec<&str> = udfs
        .iter()
        .map(|u| u.user_defined_function_id.as_str())
        .collect();
    assert_eq!(ids, vec!["u1", "u2"]);
}

#[tokio::test]
async fn get_by_id_returns_function() {
    let mut mock = MockUserDefinedFunctionServiceImpl::new();
    mock.expect_get_user_defined_function()
        .withf(|req| req.get_ref().user_defined_function_id == "u1")
        .returning(|_| {
            Ok(Response::new(GetUserDefinedFunctionResponse {
                user_defined_function: Some(UserDefinedFunction {
                    user_defined_function_id: "u1".into(),
                    name: "my_func".into(),
                    ..Default::default()
                }),
            }))
        });

    let (service, _h) = service_with_mock(mock).await;

    let udf = service
        .get_user_defined_function("u1".into(), String::new())
        .await
        .expect("get failed");

    assert_eq!(udf.name, "my_func");
}

#[tokio::test]
async fn get_missing_function_is_not_found() {
    let mut mock = MockUserDefinedFunctionServiceImpl::new();
    mock.expect_get_user_defined_function().returning(|_| {
        Ok(Response::new(GetUserDefinedFunctionResponse {
            user_defined_function: None,
        }))
    });

    let (service, _h) = service_with_mock(mock).await;

    let err = service
        .get_user_defined_function("missing".into(), String::new())
        .await
        .expect_err("expected not found");

    let status = err.downcast::<Status>().expect("expected tonic Status");
    assert_eq!(status.code(), tonic::Code::NotFound);
}

#[tokio::test]
async fn create_sends_inputs_and_returns_function() {
    let mut mock = MockUserDefinedFunctionServiceImpl::new();
    mock.expect_create_user_defined_function()
        .withf(|req| {
            let req = req.get_ref();
            req.name == "scale"
                && req.expression == "x * 2"
                && req.function_inputs.len() == 1
                && req.function_inputs[0].identifier == "x"
                && req.function_inputs[0].data_type == FunctionDataType::Numeric as i32
        })
        .returning(|req| {
            let req = req.into_inner();
            Ok(Response::new(CreateUserDefinedFunctionResponse {
                user_defined_function: Some(UserDefinedFunction {
                    user_defined_function_id: "u9".into(),
                    name: req.name,
                    expression: req.expression,
                    function_inputs: req.function_inputs,
                    ..Default::default()
                }),
            }))
        });

    let (service, _h) = service_with_mock(mock).await;

    let udf = service
        .create_user_defined_function(
            "scale".into(),
            None,
            "x * 2".into(),
            vec![FunctionInput {
                identifier: "x".into(),
                data_type: FunctionDataType::Numeric as i32,
                constant: false,
            }],
            None,
            vec![],
        )
        .await
        .expect("create failed");

    assert_eq!(udf.user_defined_function_id, "u9");
}

#[tokio::test]
async fn create_propagates_invalid_argument() {
    let mut mock = MockUserDefinedFunctionServiceImpl::new();
    mock.expect_create_user_defined_function()
        .returning(|_| Err(Status::invalid_argument("bad expression")));

    let (service, _h) = service_with_mock(mock).await;

    let err = service
        .create_user_defined_function("bad".into(), None, "???".into(), vec![], None, vec![])
        .await
        .expect_err("expected error");

    let status = err.downcast::<Status>().expect("expected tonic Status");
    assert_eq!(status.code(), tonic::Code::InvalidArgument);
}

#[tokio::test]
async fn update_reads_then_writes_only_changed_fields() {
    let mut mock = MockUserDefinedFunctionServiceImpl::new();
    mock.expect_get_user_defined_function().returning(|_| {
        Ok(Response::new(GetUserDefinedFunctionResponse {
            user_defined_function: Some(UserDefinedFunction {
                user_defined_function_id: "u1".into(),
                name: "old_name".into(),
                description: "keep me".into(),
                expression: "x".into(),
                ..Default::default()
            }),
        }))
    });
    mock.expect_update_user_defined_function()
        .withf(|req| {
            let req = req.get_ref();
            let udf = req.user_defined_function.as_ref().unwrap();
            let mask = req.update_mask.as_ref().unwrap();
            // Only description changed; name/expression carried through from the read.
            udf.name == "old_name"
                && udf.description == "new desc"
                && udf.expression == "x"
                && mask.paths == vec!["description".to_string()]
        })
        .returning(|req| {
            let req = req.into_inner();
            Ok(Response::new(UpdateUserDefinedFunctionResponse {
                user_defined_function: req.user_defined_function,
            }))
        });

    let (service, _h) = service_with_mock(mock).await;

    let udf = service
        .update_user_defined_function(
            "u1".into(),
            UserDefinedFunctionUpdate {
                description: Some("new desc".into()),
                ..Default::default()
            },
        )
        .await
        .expect("update failed");

    assert_eq!(udf.description, "new desc");
}

#[tokio::test]
async fn update_with_no_fields_is_invalid() {
    let mut mock = MockUserDefinedFunctionServiceImpl::new();
    mock.expect_get_user_defined_function().returning(|_| {
        Ok(Response::new(GetUserDefinedFunctionResponse {
            user_defined_function: Some(UserDefinedFunction {
                user_defined_function_id: "u1".into(),
                ..Default::default()
            }),
        }))
    });

    let (service, _h) = service_with_mock(mock).await;

    let err = service
        .update_user_defined_function("u1".into(), UserDefinedFunctionUpdate::default())
        .await
        .expect_err("expected invalid argument");

    let status = err.downcast::<Status>().expect("expected tonic Status");
    assert_eq!(status.code(), tonic::Code::InvalidArgument);
}

#[tokio::test]
async fn list_forwards_limit_as_page_size() {
    let mut mock = MockUserDefinedFunctionServiceImpl::new();
    mock.expect_list_user_defined_functions()
        .times(1)
        .returning(|req| {
            assert_eq!(req.get_ref().page_size, 2);
            Ok(Response::new(ListUserDefinedFunctionsResponse {
                user_defined_functions: vec![
                    UserDefinedFunction {
                        user_defined_function_id: "u1".into(),
                        ..Default::default()
                    },
                    UserDefinedFunction {
                        user_defined_function_id: "u2".into(),
                        ..Default::default()
                    },
                ],
                next_page_token: "page-2".into(),
            }))
        });

    let (service, _h) = service_with_mock(mock).await;

    let udfs = service
        .list_user_defined_functions(String::new(), None, Some(2))
        .await
        .expect("list failed");

    assert_eq!(udfs.len(), 2);
}

#[tokio::test]
async fn list_truncates_to_limit_across_pages() {
    let mut mock = MockUserDefinedFunctionServiceImpl::new();
    mock.expect_list_user_defined_functions().returning(|req| {
        let req = req.into_inner();
        assert_eq!(req.page_size, 3);
        let (udfs, next) = match req.page_token.as_str() {
            "" => (
                vec![
                    UserDefinedFunction {
                        user_defined_function_id: "u1".into(),
                        ..Default::default()
                    },
                    UserDefinedFunction {
                        user_defined_function_id: "u2".into(),
                        ..Default::default()
                    },
                ],
                "page-2".to_string(),
            ),
            "page-2" => (
                vec![
                    UserDefinedFunction {
                        user_defined_function_id: "u3".into(),
                        ..Default::default()
                    },
                    UserDefinedFunction {
                        user_defined_function_id: "u4".into(),
                        ..Default::default()
                    },
                ],
                String::new(),
            ),
            other => return Err(Status::invalid_argument(format!("bad token: {other}"))),
        };
        Ok(Response::new(ListUserDefinedFunctionsResponse {
            user_defined_functions: udfs,
            next_page_token: next,
        }))
    });

    let (service, _h) = service_with_mock(mock).await;

    let udfs = service
        .list_user_defined_functions(String::new(), None, Some(3))
        .await
        .expect("list failed");

    let ids: Vec<&str> = udfs
        .iter()
        .map(|u| u.user_defined_function_id.as_str())
        .collect();
    assert_eq!(ids, vec!["u1", "u2", "u3"]);
}

#[tokio::test]
async fn list_propagates_grpc_error() {
    let mut mock = MockUserDefinedFunctionServiceImpl::new();
    mock.expect_list_user_defined_functions()
        .returning(|_| Err(Status::invalid_argument("bad filter")));

    let (service, _h) = service_with_mock(mock).await;

    let err = service
        .list_user_defined_functions("nope".to_string(), None, None)
        .await
        .expect_err("expected error");

    assert!(
        err.to_string()
            .contains("failed to query user-defined functions")
    );
}

#[tokio::test]
async fn update_replaces_function_inputs_with_masked_path() {
    let mut mock = MockUserDefinedFunctionServiceImpl::new();
    mock.expect_get_user_defined_function().returning(|_| {
        Ok(Response::new(GetUserDefinedFunctionResponse {
            user_defined_function: Some(UserDefinedFunction {
                user_defined_function_id: "u1".into(),
                name: "keep".into(),
                function_inputs: vec![FunctionInput {
                    identifier: "old".into(),
                    data_type: FunctionDataType::Numeric as i32,
                    constant: false,
                }],
                ..Default::default()
            }),
        }))
    });
    mock.expect_update_user_defined_function()
        .withf(|req| {
            let req = req.get_ref();
            let udf = req.user_defined_function.as_ref().unwrap();
            let mask = req.update_mask.as_ref().unwrap();
            // name preserved from the read; inputs replaced; only function_inputs masked.
            udf.name == "keep"
                && udf.function_inputs.len() == 1
                && udf.function_inputs[0].identifier == "new"
                && mask.paths == vec!["function_inputs".to_string()]
        })
        .returning(|req| {
            Ok(Response::new(UpdateUserDefinedFunctionResponse {
                user_defined_function: req.into_inner().user_defined_function,
            }))
        });

    let (service, _h) = service_with_mock(mock).await;

    let udf = service
        .update_user_defined_function(
            "u1".into(),
            UserDefinedFunctionUpdate {
                function_inputs: Some(vec![FunctionInput {
                    identifier: "new".into(),
                    data_type: FunctionDataType::String as i32,
                    constant: true,
                }]),
                ..Default::default()
            },
        )
        .await
        .expect("update failed");

    assert_eq!(udf.function_inputs[0].identifier, "new");
}

#[tokio::test]
async fn update_replaces_metadata_with_masked_path() {
    let mut mock = MockUserDefinedFunctionServiceImpl::new();
    mock.expect_get_user_defined_function().returning(|_| {
        Ok(Response::new(GetUserDefinedFunctionResponse {
            user_defined_function: Some(UserDefinedFunction {
                user_defined_function_id: "u1".into(),
                name: "keep".into(),
                ..Default::default()
            }),
        }))
    });
    mock.expect_update_user_defined_function()
        .withf(|req| {
            let req = req.get_ref();
            let udf = req.user_defined_function.as_ref().unwrap();
            let mask = req.update_mask.as_ref().unwrap();
            udf.name == "keep"
                && udf.metadata.len() == 1
                && udf.metadata[0].key.as_ref().map(|k| k.name.as_str()) == Some("team")
                && matches!(
                    &udf.metadata[0].value,
                    Some(MetadataValueInner::StringValue(s)) if s == "controls"
                )
                && mask.paths == vec!["metadata".to_string()]
        })
        .returning(|req| {
            Ok(Response::new(UpdateUserDefinedFunctionResponse {
                user_defined_function: req.into_inner().user_defined_function,
            }))
        });

    let (service, _h) = service_with_mock(mock).await;

    service
        .update_user_defined_function(
            "u1".into(),
            UserDefinedFunctionUpdate {
                metadata: Some(vec![MetadataValue {
                    key: Some(MetadataKey {
                        name: "team".into(),
                        r#type: MetadataKeyType::String as i32,
                        ..Default::default()
                    }),
                    value: Some(MetadataValueInner::StringValue("controls".into())),
                    ..Default::default()
                }]),
                ..Default::default()
            },
        )
        .await
        .expect("update failed");
}

#[tokio::test]
async fn update_rejects_name_combined_with_other_fields() {
    let mut mock = MockUserDefinedFunctionServiceImpl::new();
    mock.expect_get_user_defined_function().returning(|_| {
        Ok(Response::new(GetUserDefinedFunctionResponse {
            user_defined_function: Some(UserDefinedFunction {
                user_defined_function_id: "u1".into(),
                ..Default::default()
            }),
        }))
    });
    // update must never be sent when name is combined with another field.
    mock.expect_update_user_defined_function().never();

    let (service, _h) = service_with_mock(mock).await;

    let err = service
        .update_user_defined_function(
            "u1".into(),
            UserDefinedFunctionUpdate {
                name: Some("renamed".into()),
                description: Some("also changed".into()),
                ..Default::default()
            },
        )
        .await
        .expect_err("expected invalid argument");

    let status = err.downcast::<Status>().expect("expected tonic Status");
    assert_eq!(status.code(), tonic::Code::InvalidArgument);
}
