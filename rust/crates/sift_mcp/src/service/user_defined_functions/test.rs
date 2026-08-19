use sift_rs::{
    common::r#type::v1::{FunctionDataType, FunctionInput, UserDefinedFunction},
    metadata::v1::{
        MetadataKey, MetadataKeyType, MetadataValue, metadata_value::Value as MetadataValueInner,
    },
    user_defined_functions::v1::{
        CreateUserDefinedFunctionResponse, ListUserDefinedFunctionVersionsResponse,
        ListUserDefinedFunctionsResponse, UpdateUserDefinedFunctionResponse,
        user_defined_function_service_server::UserDefinedFunctionServiceServer,
    },
};
use sift_test_util::{
    grpc::memory_sift_channel,
    mock::user_defined_functions::v1::MockUserDefinedFunctionServiceImpl,
};
use tokio::task::JoinHandle;
use tonic::{Response, Status, transport::Server};

use super::{UdfUpdate, UserDefinedFunctionService};
use crate::policy::RetryPolicy;
use crate::service::common::DEFAULT_LIMIT;

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

fn numeric_input(identifier: &str) -> FunctionInput {
    FunctionInput {
        identifier: identifier.into(),
        data_type: FunctionDataType::Numeric.into(),
        constant: false,
    }
}

fn udf(id: &str, name: &str) -> UserDefinedFunction {
    UserDefinedFunction {
        user_defined_function_id: id.into(),
        name: name.into(),
        ..Default::default()
    }
}

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

    (
        UserDefinedFunctionService::new(channel, RetryPolicy::default()),
        handle,
    )
}

#[tokio::test]
async fn list_user_defined_functions_returns_single_page() {
    let mut mock = MockUserDefinedFunctionServiceImpl::new();
    mock.expect_list_user_defined_functions()
        .times(1)
        .withf(|req| {
            let req = req.get_ref();
            req.filter == "name.matches(\"(?i)rms\")"
                && req.order_by == "name"
                && req.page_size == DEFAULT_LIMIT
                && req.page_token.is_empty()
        })
        .returning(|_| {
            Ok(Response::new(ListUserDefinedFunctionsResponse {
                user_defined_functions: vec![udf("f1", "rms"), udf("f2", "rms_windowed")],
                next_page_token: String::new(),
            }))
        });

    let (service, _h) = service_with_mock(mock).await;

    let functions = service
        .list_user_defined_functions(
            "name.matches(\"(?i)rms\")".to_string(),
            Some("name".to_string()),
            None,
        )
        .await
        .expect("list_user_defined_functions failed");

    assert_eq!(functions.len(), 2);
    assert_eq!(functions[0].user_defined_function_id, "f1");
    assert_eq!(functions[1].user_defined_function_id, "f2");
}

#[tokio::test]
async fn list_user_defined_functions_paginates_until_token_empty() {
    let mut mock = MockUserDefinedFunctionServiceImpl::new();
    mock.expect_list_user_defined_functions().returning(|req| {
        let req = req.into_inner();
        assert_eq!(req.page_size, DEFAULT_LIMIT);
        let (functions, next) = match req.page_token.as_str() {
            "" => (vec![udf("f1", "a")], "page-2".to_string()),
            "page-2" => (vec![udf("f2", "b")], "page-3".to_string()),
            "page-3" => (vec![udf("f3", "c")], String::new()),
            other => return Err(Status::invalid_argument(format!("bad token: {other}"))),
        };
        Ok(Response::new(ListUserDefinedFunctionsResponse {
            user_defined_functions: functions,
            next_page_token: next,
        }))
    });

    let (service, _h) = service_with_mock(mock).await;

    let functions = service
        .list_user_defined_functions(String::new(), None, None)
        .await
        .expect("list_user_defined_functions failed");

    let ids: Vec<&str> = functions
        .iter()
        .map(|f| f.user_defined_function_id.as_str())
        .collect();
    assert_eq!(ids, vec!["f1", "f2", "f3"]);
}

#[tokio::test]
async fn list_user_defined_functions_truncates_to_limit_across_pages() {
    let mut mock = MockUserDefinedFunctionServiceImpl::new();
    mock.expect_list_user_defined_functions().returning(|req| {
        let req = req.into_inner();
        assert_eq!(req.page_size, 3);
        let (functions, next) = match req.page_token.as_str() {
            "" => (vec![udf("f1", "a"), udf("f2", "b")], "page-2".to_string()),
            "page-2" => (vec![udf("f3", "c"), udf("f4", "d")], String::new()),
            other => return Err(Status::invalid_argument(format!("bad token: {other}"))),
        };
        Ok(Response::new(ListUserDefinedFunctionsResponse {
            user_defined_functions: functions,
            next_page_token: next,
        }))
    });

    let (service, _h) = service_with_mock(mock).await;

    let functions = service
        .list_user_defined_functions(String::new(), None, Some(3))
        .await
        .expect("list_user_defined_functions failed");

    let ids: Vec<&str> = functions
        .iter()
        .map(|f| f.user_defined_function_id.as_str())
        .collect();
    assert_eq!(ids, vec!["f1", "f2", "f3"]);
}

#[tokio::test]
async fn list_user_defined_functions_clamps_limit_to_page_size() {
    let mut mock = MockUserDefinedFunctionServiceImpl::new();
    mock.expect_list_user_defined_functions()
        .times(1)
        .withf(|req| req.get_ref().page_size == 200)
        .returning(|_| {
            Ok(Response::new(ListUserDefinedFunctionsResponse {
                user_defined_functions: vec![udf("f1", "a")],
                next_page_token: String::new(),
            }))
        });

    let (service, _h) = service_with_mock(mock).await;

    service
        .list_user_defined_functions(String::new(), None, Some(5_000))
        .await
        .expect("list_user_defined_functions failed");
}

#[tokio::test]
async fn list_user_defined_functions_breaks_on_empty_page() {
    let mut mock = MockUserDefinedFunctionServiceImpl::new();
    mock.expect_list_user_defined_functions()
        .times(1)
        .returning(|_| {
            Ok(Response::new(ListUserDefinedFunctionsResponse {
                user_defined_functions: vec![],
                next_page_token: "ignored".into(),
            }))
        });

    let (service, _h) = service_with_mock(mock).await;

    let functions = service
        .list_user_defined_functions(String::new(), None, None)
        .await
        .expect("list_user_defined_functions failed");

    assert!(functions.is_empty());
}

#[tokio::test]
async fn list_user_defined_functions_propagates_grpc_error() {
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
            .contains("failed to query user defined functions")
    );
}

#[tokio::test]
async fn list_versions_sends_id_filter_and_order_by() {
    let mut mock = MockUserDefinedFunctionServiceImpl::new();
    mock.expect_list_user_defined_function_versions()
        .times(1)
        .withf(|req| {
            let req = req.get_ref();
            req.user_defined_function_id == "f1"
                && req.name.is_empty()
                && req.filter == "version == 2"
                && req.order_by == "version desc"
                && req.page_size == DEFAULT_LIMIT
                && req.page_token.is_empty()
        })
        .returning(|_| {
            Ok(Response::new(ListUserDefinedFunctionVersionsResponse {
                user_defined_functions: vec![UserDefinedFunction {
                    user_defined_function_id: "f1".into(),
                    user_defined_function_version_id: "v2".into(),
                    version: 2,
                    ..Default::default()
                }],
                next_page_token: String::new(),
            }))
        });

    let (service, _h) = service_with_mock(mock).await;

    let versions = service
        .list_user_defined_function_versions(
            "f1".to_string(),
            String::new(),
            "version == 2".to_string(),
            Some("version desc".to_string()),
            None,
        )
        .await
        .expect("list_user_defined_function_versions failed");

    assert_eq!(versions.len(), 1);
    assert_eq!(versions[0].version, 2);
}

#[tokio::test]
async fn list_versions_sends_name_when_id_is_empty() {
    let mut mock = MockUserDefinedFunctionServiceImpl::new();
    mock.expect_list_user_defined_function_versions()
        .times(1)
        .withf(|req| {
            let req = req.get_ref();
            req.user_defined_function_id.is_empty() && req.name == "rms"
        })
        .returning(|_| {
            Ok(Response::new(ListUserDefinedFunctionVersionsResponse {
                user_defined_functions: vec![udf("f1", "rms")],
                next_page_token: String::new(),
            }))
        });

    let (service, _h) = service_with_mock(mock).await;

    service
        .list_user_defined_function_versions(
            String::new(),
            "rms".to_string(),
            String::new(),
            None,
            None,
        )
        .await
        .expect("list_user_defined_function_versions failed");
}

#[tokio::test]
async fn list_versions_paginates_and_truncates_to_limit() {
    let mut mock = MockUserDefinedFunctionServiceImpl::new();
    mock.expect_list_user_defined_function_versions()
        .returning(|req| {
            let req = req.into_inner();
            assert_eq!(req.page_size, 2);
            let (functions, next) = match req.page_token.as_str() {
                "" => (
                    vec![
                        UserDefinedFunction {
                            version: 1,
                            ..Default::default()
                        },
                        UserDefinedFunction {
                            version: 2,
                            ..Default::default()
                        },
                    ],
                    "page-2".to_string(),
                ),
                "page-2" => (
                    vec![UserDefinedFunction {
                        version: 3,
                        ..Default::default()
                    }],
                    String::new(),
                ),
                other => return Err(Status::invalid_argument(format!("bad token: {other}"))),
            };
            Ok(Response::new(ListUserDefinedFunctionVersionsResponse {
                user_defined_functions: functions,
                next_page_token: next,
            }))
        });

    let (service, _h) = service_with_mock(mock).await;

    let versions = service
        .list_user_defined_function_versions(
            "f1".to_string(),
            String::new(),
            String::new(),
            None,
            Some(2),
        )
        .await
        .expect("list_user_defined_function_versions failed");

    let numbers: Vec<u32> = versions.iter().map(|v| v.version).collect();
    assert_eq!(numbers, vec![1, 2]);
}

#[tokio::test]
async fn list_versions_propagates_grpc_error() {
    let mut mock = MockUserDefinedFunctionServiceImpl::new();
    mock.expect_list_user_defined_function_versions()
        .returning(|_| Err(Status::not_found("no such function")));

    let (service, _h) = service_with_mock(mock).await;

    let err = service
        .list_user_defined_function_versions(
            "missing".to_string(),
            String::new(),
            String::new(),
            None,
            None,
        )
        .await
        .expect_err("expected error");

    assert!(
        err.to_string()
            .contains("failed to query user defined function versions")
    );
}

#[tokio::test]
async fn create_sends_every_provided_field() {
    let mut mock = MockUserDefinedFunctionServiceImpl::new();
    mock.expect_create_user_defined_function()
        .times(1)
        .withf(|req| {
            let req = req.get_ref();
            req.name == "rms"
                && req.description.as_deref() == Some("root mean square")
                && req.expression == "sqrt(mean($x * $x))"
                && req.function_inputs.len() == 1
                && req.function_inputs[0].identifier == "x"
                && req.function_inputs[0].data_type == i32::from(FunctionDataType::Numeric)
                && req.user_notes.as_deref() == Some("initial version")
                && req.metadata.len() == 1
        })
        .returning(|req| {
            let req = req.into_inner();
            Ok(Response::new(CreateUserDefinedFunctionResponse {
                user_defined_function: Some(UserDefinedFunction {
                    user_defined_function_id: "f1".into(),
                    user_defined_function_version_id: "v1".into(),
                    version: 1,
                    name: req.name,
                    expression: req.expression,
                    function_inputs: req.function_inputs,
                    ..Default::default()
                }),
            }))
        });

    let (service, _h) = service_with_mock(mock).await;

    let created = service
        .create_user_defined_function(
            "rms".to_string(),
            Some("root mean square".to_string()),
            "sqrt(mean($x * $x))".to_string(),
            vec![numeric_input("x")],
            Some("initial version".to_string()),
            vec![string_metadata("owner", "avionics")],
        )
        .await
        .expect("create_user_defined_function failed");

    assert_eq!(created.user_defined_function_id, "f1");
    assert_eq!(created.version, 1);
}

#[tokio::test]
async fn create_omits_optional_fields_when_absent() {
    let mut mock = MockUserDefinedFunctionServiceImpl::new();
    mock.expect_create_user_defined_function()
        .times(1)
        .withf(|req| {
            let req = req.get_ref();
            req.description.is_none() && req.user_notes.is_none() && req.metadata.is_empty()
        })
        .returning(|_| {
            Ok(Response::new(CreateUserDefinedFunctionResponse {
                user_defined_function: Some(udf("f1", "rms")),
            }))
        });

    let (service, _h) = service_with_mock(mock).await;

    service
        .create_user_defined_function(
            "rms".to_string(),
            None,
            "$x".to_string(),
            vec![numeric_input("x")],
            None,
            Vec::new(),
        )
        .await
        .expect("create_user_defined_function failed");
}

#[tokio::test]
async fn create_errors_when_response_missing_function() {
    let mut mock = MockUserDefinedFunctionServiceImpl::new();
    mock.expect_create_user_defined_function().returning(|_| {
        Ok(Response::new(CreateUserDefinedFunctionResponse {
            user_defined_function: None,
        }))
    });

    let (service, _h) = service_with_mock(mock).await;

    let err = service
        .create_user_defined_function(
            "rms".to_string(),
            None,
            "$x".to_string(),
            vec![numeric_input("x")],
            None,
            Vec::new(),
        )
        .await
        .expect_err("expected error");

    assert!(
        err.to_string()
            .contains("create_user_defined_function response missing user defined function")
    );
}

#[tokio::test]
async fn create_propagates_grpc_error() {
    let mut mock = MockUserDefinedFunctionServiceImpl::new();
    mock.expect_create_user_defined_function()
        .returning(|_| Err(Status::invalid_argument("bad expression")));

    let (service, _h) = service_with_mock(mock).await;

    let err = service
        .create_user_defined_function(
            "rms".to_string(),
            None,
            "nonsense(".to_string(),
            vec![],
            None,
            Vec::new(),
        )
        .await
        .expect_err("expected error");

    assert!(
        err.to_string()
            .contains("failed to create user defined function")
    );
}

#[tokio::test]
async fn update_masks_only_the_provided_fields() {
    let mut mock = MockUserDefinedFunctionServiceImpl::new();
    mock.expect_update_user_defined_function()
        .times(1)
        .withf(|req| {
            let req = req.get_ref();
            let function = req.user_defined_function.as_ref().expect("function present");
            let mask = req.update_mask.as_ref().expect("mask present");
            function.user_defined_function_id == "f1"
                && function.description == "updated"
                && function.expression.is_empty()
                && function.function_inputs.is_empty()
                && function.metadata.is_empty()
                && mask.paths == vec!["description".to_string()]
        })
        .returning(|req| {
            let req = req.into_inner();
            Ok(Response::new(UpdateUserDefinedFunctionResponse {
                user_defined_function: req.user_defined_function,
            }))
        });

    let (service, _h) = service_with_mock(mock).await;

    let updated = service
        .update_user_defined_function(
            "f1".to_string(),
            UdfUpdate {
                description: Some("updated".to_string()),
                ..Default::default()
            },
        )
        .await
        .expect("update_user_defined_function failed");

    assert_eq!(updated.user_defined_function_id, "f1");
}

#[tokio::test]
async fn update_masks_every_provided_field_in_declaration_order() {
    let mut mock = MockUserDefinedFunctionServiceImpl::new();
    mock.expect_update_user_defined_function()
        .times(1)
        .withf(|req| {
            let req = req.get_ref();
            let function = req.user_defined_function.as_ref().expect("function present");
            let mask = req.update_mask.as_ref().expect("mask present");
            function.name == "rms_v2"
                && function.description == "updated"
                && function.expression == "$x * 2"
                && function.function_inputs.len() == 1
                && function.metadata.len() == 1
                && mask.paths
                    == vec![
                        "name".to_string(),
                        "description".to_string(),
                        "expression".to_string(),
                        "function_inputs".to_string(),
                        "metadata".to_string(),
                    ]
        })
        .returning(|req| {
            let req = req.into_inner();
            Ok(Response::new(UpdateUserDefinedFunctionResponse {
                user_defined_function: req.user_defined_function,
            }))
        });

    let (service, _h) = service_with_mock(mock).await;

    service
        .update_user_defined_function(
            "f1".to_string(),
            UdfUpdate {
                name: Some("rms_v2".to_string()),
                description: Some("updated".to_string()),
                expression: Some("$x * 2".to_string()),
                function_inputs: Some(vec![numeric_input("x")]),
                metadata: Some(vec![string_metadata("owner", "avionics")]),
            },
        )
        .await
        .expect("update_user_defined_function failed");
}

#[tokio::test]
async fn update_sends_no_version_precondition() {
    // `UpdateUserDefinedFunctionRequest` has no version precondition field: the
    // service always creates a new version and returns it. Assert the request
    // carries no stale version identifiers that could be mistaken for one, and
    // that the caller sees the new version from the response.
    let mut mock = MockUserDefinedFunctionServiceImpl::new();
    mock.expect_update_user_defined_function()
        .times(1)
        .withf(|req| {
            let function = req
                .get_ref()
                .user_defined_function
                .as_ref()
                .expect("function present");
            function.version == 0
                && function.user_defined_function_version_id.is_empty()
                && function.function_dependencies.is_empty()
        })
        .returning(|req| {
            let req = req.into_inner();
            let mut function = req.user_defined_function.expect("function present");
            function.version = 7;
            function.user_defined_function_version_id = "v7".into();
            Ok(Response::new(UpdateUserDefinedFunctionResponse {
                user_defined_function: Some(function),
            }))
        });

    let (service, _h) = service_with_mock(mock).await;

    let updated = service
        .update_user_defined_function(
            "f1".to_string(),
            UdfUpdate {
                expression: Some("$x + 1".to_string()),
                ..Default::default()
            },
        )
        .await
        .expect("update_user_defined_function failed");

    assert_eq!(updated.version, 7);
    assert_eq!(updated.user_defined_function_version_id, "v7");
}

#[tokio::test]
async fn update_with_no_fields_sends_an_empty_mask() {
    // The tool handler rejects this case; the service contract is to send
    // exactly what it was given.
    let mut mock = MockUserDefinedFunctionServiceImpl::new();
    mock.expect_update_user_defined_function()
        .times(1)
        .withf(|req| {
            let req = req.get_ref();
            let function = req.user_defined_function.as_ref().expect("function present");
            let mask = req.update_mask.as_ref().expect("mask present");
            function.user_defined_function_id == "f1" && mask.paths.is_empty()
        })
        .returning(|req| {
            let req = req.into_inner();
            Ok(Response::new(UpdateUserDefinedFunctionResponse {
                user_defined_function: req.user_defined_function,
            }))
        });

    let (service, _h) = service_with_mock(mock).await;

    service
        .update_user_defined_function("f1".to_string(), UdfUpdate::default())
        .await
        .expect("update_user_defined_function failed");
}

#[tokio::test]
async fn update_errors_when_response_missing_function() {
    let mut mock = MockUserDefinedFunctionServiceImpl::new();
    mock.expect_update_user_defined_function().returning(|_| {
        Ok(Response::new(UpdateUserDefinedFunctionResponse {
            user_defined_function: None,
        }))
    });

    let (service, _h) = service_with_mock(mock).await;

    let err = service
        .update_user_defined_function(
            "f1".to_string(),
            UdfUpdate {
                description: Some("x".to_string()),
                ..Default::default()
            },
        )
        .await
        .expect_err("expected error");

    assert!(
        err.to_string()
            .contains("update_user_defined_function response missing user defined function")
    );
}

#[tokio::test]
async fn update_propagates_grpc_error() {
    let mut mock = MockUserDefinedFunctionServiceImpl::new();
    mock.expect_update_user_defined_function()
        .returning(|_| Err(Status::failed_precondition("function has dependents")));

    let (service, _h) = service_with_mock(mock).await;

    let err = service
        .update_user_defined_function(
            "f1".to_string(),
            UdfUpdate {
                function_inputs: Some(vec![numeric_input("x")]),
                ..Default::default()
            },
        )
        .await
        .expect_err("expected error");

    assert!(
        err.to_string()
            .contains("failed to update user defined function")
    );
}

#[tokio::test]
async fn archive_sets_is_archived_true_through_the_mask() {
    let mut mock = MockUserDefinedFunctionServiceImpl::new();
    mock.expect_update_user_defined_function()
        .times(1)
        .withf(|req| {
            let req = req.get_ref();
            let function = req.user_defined_function.as_ref().expect("function present");
            let mask = req.update_mask.as_ref().expect("mask present");
            function.user_defined_function_id == "f1"
                && function.is_archived
                && function.archived_date.is_none()
                && mask.paths == vec!["is_archived".to_string()]
        })
        .returning(|req| {
            let req = req.into_inner();
            Ok(Response::new(UpdateUserDefinedFunctionResponse {
                user_defined_function: req.user_defined_function,
            }))
        });

    let (service, _h) = service_with_mock(mock).await;

    let archived = service
        .set_user_defined_function_archived("f1".to_string(), true)
        .await
        .expect("set_user_defined_function_archived failed");

    assert!(archived.is_archived);
}

#[tokio::test]
async fn unarchive_sets_is_archived_false_through_the_mask() {
    let mut mock = MockUserDefinedFunctionServiceImpl::new();
    mock.expect_update_user_defined_function()
        .times(1)
        .withf(|req| {
            let req = req.get_ref();
            let function = req.user_defined_function.as_ref().expect("function present");
            let mask = req.update_mask.as_ref().expect("mask present");
            function.user_defined_function_id == "f2"
                && !function.is_archived
                && mask.paths == vec!["is_archived".to_string()]
        })
        .returning(|req| {
            let req = req.into_inner();
            Ok(Response::new(UpdateUserDefinedFunctionResponse {
                user_defined_function: req.user_defined_function,
            }))
        });

    let (service, _h) = service_with_mock(mock).await;

    let unarchived = service
        .set_user_defined_function_archived("f2".to_string(), false)
        .await
        .expect("set_user_defined_function_archived failed");

    assert!(!unarchived.is_archived);
}

#[tokio::test]
async fn set_archived_propagates_grpc_error() {
    let mut mock = MockUserDefinedFunctionServiceImpl::new();
    mock.expect_update_user_defined_function()
        .returning(|_| Err(Status::not_found("function missing")));

    let (service, _h) = service_with_mock(mock).await;

    let err = service
        .set_user_defined_function_archived("missing".to_string(), true)
        .await
        .expect_err("expected error");

    assert!(
        err.to_string()
            .contains("failed to update user defined function")
    );
}
