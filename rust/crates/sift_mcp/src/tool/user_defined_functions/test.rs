use rmcp::{handler::server::wrapper::Parameters, model::ErrorCode};
use sift_rs::{
    common::r#type::v1::{FunctionDataType, FunctionInput, UserDefinedFunction},
    user_defined_functions::v1::{
        CreateUserDefinedFunctionResponse, ListUserDefinedFunctionVersionsResponse,
        ListUserDefinedFunctionsResponse, UpdateUserDefinedFunctionResponse,
        user_defined_function_service_server::UserDefinedFunctionServiceServer,
    },
};
use sift_test_util::{
    grpc::memory_sift_channel, mock::user_defined_functions::v1::MockUserDefinedFunctionServiceImpl,
};
use tokio::task::JoinHandle;
use tonic::{Response, Status, transport::Server};

use crate::{
    server::SiftMcpServer,
    tool::{
        common::test_support::{list_params, list_params_with_fields, structured},
        user_defined_functions::{
            ArchiveUserDefinedFunctionParams, CreateUserDefinedFunctionParams,
            UpdateUserDefinedFunctionParams, UserDefinedFunctionVersionListParams,
        },
    },
};

async fn server_with_mock(
    mock: MockUserDefinedFunctionServiceImpl,
    allow_create: bool,
    allow_destructive: bool,
) -> (SiftMcpServer, JoinHandle<()>) {
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
        SiftMcpServer::new(
            channel,
            String::from("https://app.test.local"),
            allow_create,
            allow_destructive,
        ),
        handle,
    )
}

fn numeric_input(identifier: &str) -> FunctionInput {
    FunctionInput {
        identifier: identifier.into(),
        data_type: FunctionDataType::Numeric.into(),
        constant: false,
    }
}

fn version_list_params(
    user_defined_function_id: Option<&str>,
    name: Option<&str>,
) -> Parameters<UserDefinedFunctionVersionListParams> {
    Parameters(UserDefinedFunctionVersionListParams {
        user_defined_function_id: user_defined_function_id.map(str::to_string),
        name: name.map(str::to_string),
        filter: String::new(),
        order_by: None,
        limit: None,
        fields: None,
    })
}

fn create_params(function_inputs_json: &str) -> Parameters<CreateUserDefinedFunctionParams> {
    Parameters(CreateUserDefinedFunctionParams {
        name: "rms".into(),
        expression: "sqrt(mean($x * $x))".into(),
        function_inputs_json: function_inputs_json.into(),
        description: None,
        user_notes: None,
        metadata: None,
    })
}

fn update_params(
    name: Option<&str>,
    description: Option<&str>,
    function_inputs_json: Option<&str>,
) -> Parameters<UpdateUserDefinedFunctionParams> {
    Parameters(UpdateUserDefinedFunctionParams {
        user_defined_function_id: "f1".into(),
        name: name.map(str::to_string),
        description: description.map(str::to_string),
        expression: None,
        function_inputs_json: function_inputs_json.map(str::to_string),
        metadata: None,
    })
}

fn archive_params(id: &str) -> Parameters<ArchiveUserDefinedFunctionParams> {
    Parameters(ArchiveUserDefinedFunctionParams {
        user_defined_function_id: id.into(),
    })
}

#[tokio::test]
async fn list_user_defined_functions_returns_structured_rows() {
    let mut mock = MockUserDefinedFunctionServiceImpl::new();
    mock.expect_list_user_defined_functions()
        .withf(|req| req.get_ref().filter == "is_archived == false")
        .returning(|_| {
            Ok(Response::new(ListUserDefinedFunctionsResponse {
                user_defined_functions: vec![UserDefinedFunction {
                    user_defined_function_id: "f1".into(),
                    name: "rms".into(),
                    version: 3,
                    ..Default::default()
                }],
                next_page_token: String::new(),
            }))
        });

    let (server, _h) = server_with_mock(mock, false, false).await;

    let resp = server
        .list_user_defined_functions(list_params("is_archived == false", None))
        .await
        .expect("list_user_defined_functions failed");

    let body = structured(resp);
    let functions = body["user_defined_functions"].as_array().expect("array");
    assert_eq!(functions.len(), 1);
    assert_eq!(functions[0]["userDefinedFunctionId"], "f1");
    assert_eq!(functions[0]["name"], "rms");
    assert_eq!(body["count"], 1);
}

#[tokio::test]
async fn list_user_defined_functions_propagates_grpc_error() {
    let mut mock = MockUserDefinedFunctionServiceImpl::new();
    mock.expect_list_user_defined_functions()
        .returning(|_| Err(Status::invalid_argument("bad filter")));

    let (server, _h) = server_with_mock(mock, false, false).await;

    let err = server
        .list_user_defined_functions(list_params("nope", None))
        .await
        .expect_err("expected error");

    assert_eq!(err.code, ErrorCode::INVALID_PARAMS);
    assert!(err.message.contains("bad filter"));
}

#[tokio::test]
async fn list_user_defined_functions_projects_requested_fields() {
    let mut mock = MockUserDefinedFunctionServiceImpl::new();
    mock.expect_list_user_defined_functions().returning(|_| {
        Ok(Response::new(ListUserDefinedFunctionsResponse {
            user_defined_functions: vec![UserDefinedFunction {
                user_defined_function_id: "f1".into(),
                name: "rms".into(),
                description: "root mean square".into(),
                ..Default::default()
            }],
            next_page_token: String::new(),
        }))
    });

    let (server, _h) = server_with_mock(mock, false, false).await;

    let resp = server
        .list_user_defined_functions(list_params_with_fields("", &["name"]))
        .await
        .expect("list_user_defined_functions failed");

    let body = structured(resp);
    assert_eq!(
        body["user_defined_functions"],
        serde_json::json!([{ "name": "rms" }])
    );
    assert_eq!(body["count"], 1);
}

#[tokio::test]
async fn list_versions_returns_structured_rows() {
    let mut mock = MockUserDefinedFunctionServiceImpl::new();
    mock.expect_list_user_defined_function_versions()
        // An empty `filter` lists every version; it is a required String, not an Option.
        .withf(|req| {
            let req = req.get_ref();
            req.user_defined_function_id == "f1" && req.filter.is_empty()
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

    let (server, _h) = server_with_mock(mock, false, false).await;

    let resp = server
        .list_user_defined_function_versions(version_list_params(Some("f1"), None))
        .await
        .expect("list_user_defined_function_versions failed");

    let body = structured(resp);
    let versions = body["user_defined_function_versions"]
        .as_array()
        .expect("array");
    assert_eq!(versions.len(), 1);
    assert_eq!(versions[0]["userDefinedFunctionVersionId"], "v2");
    assert_eq!(body["count"], 1);
}

#[tokio::test]
async fn list_versions_forwards_the_required_filter() {
    let mut mock = MockUserDefinedFunctionServiceImpl::new();
    mock.expect_list_user_defined_function_versions()
        .times(1)
        .withf(|req| req.get_ref().filter == "version == 2")
        .returning(|_| {
            Ok(Response::new(ListUserDefinedFunctionVersionsResponse {
                user_defined_functions: vec![UserDefinedFunction {
                    version: 2,
                    ..Default::default()
                }],
                next_page_token: String::new(),
            }))
        });

    let (server, _h) = server_with_mock(mock, false, false).await;

    server
        .list_user_defined_function_versions(Parameters(UserDefinedFunctionVersionListParams {
            user_defined_function_id: Some("f1".into()),
            name: None,
            filter: "version == 2".into(),
            order_by: None,
            limit: None,
            fields: None,
        }))
        .await
        .expect("list_user_defined_function_versions failed");
}

#[tokio::test]
async fn list_versions_projects_requested_fields() {
    let mut mock = MockUserDefinedFunctionServiceImpl::new();
    mock.expect_list_user_defined_function_versions()
        .returning(|_| {
            Ok(Response::new(ListUserDefinedFunctionVersionsResponse {
                user_defined_functions: vec![UserDefinedFunction {
                    user_defined_function_id: "f1".into(),
                    user_defined_function_version_id: "v2".into(),
                    version: 2,
                    name: "rms".into(),
                    ..Default::default()
                }],
                next_page_token: String::new(),
            }))
        });

    let (server, _h) = server_with_mock(mock, false, false).await;

    let resp = server
        .list_user_defined_function_versions(Parameters(UserDefinedFunctionVersionListParams {
            user_defined_function_id: Some("f1".into()),
            name: None,
            filter: String::new(),
            order_by: None,
            limit: None,
            fields: Some(vec!["version".into()]),
        }))
        .await
        .expect("list_user_defined_function_versions failed");

    let body = structured(resp);
    assert_eq!(
        body["user_defined_function_versions"],
        serde_json::json!([{ "version": 2 }])
    );
    assert_eq!(body["count"], 1);
}

#[tokio::test]
async fn list_versions_rejects_both_identifiers() {
    let mock = MockUserDefinedFunctionServiceImpl::new();
    let (server, _h) = server_with_mock(mock, false, false).await;

    let err = server
        .list_user_defined_function_versions(version_list_params(Some("f1"), Some("rms")))
        .await
        .expect_err("expected mutually exclusive params to be rejected");

    assert_eq!(err.code, ErrorCode::INVALID_PARAMS);
    assert!(err.message.contains("exactly one"));
}

#[tokio::test]
async fn list_versions_rejects_missing_identifier() {
    let mock = MockUserDefinedFunctionServiceImpl::new();
    let (server, _h) = server_with_mock(mock, false, false).await;

    let err = server
        .list_user_defined_function_versions(version_list_params(None, None))
        .await
        .expect_err("expected missing identifier to be rejected");

    assert_eq!(err.code, ErrorCode::INVALID_PARAMS);
}

#[tokio::test]
async fn create_blocked_without_allow_create() {
    // No expectations on the mock: the gate must fire before any RPC.
    let mock = MockUserDefinedFunctionServiceImpl::new();
    let (server, _h) = server_with_mock(mock, false, false).await;

    let err = server
        .create_user_defined_function(create_params(
            "[{\"identifier\":\"x\",\"data_type\":\"numeric\"}]",
        ))
        .await
        .expect_err("expected create gate to reject the call");

    assert_eq!(err.code, ErrorCode::INVALID_REQUEST);
    assert!(err.message.contains("--allow-create"));
    let data = err
        .data
        .expect("create gate should return remediation data");
    assert_eq!(
        data["remediation_command"],
        "sift-cli agent update --allow-create"
    );
}

#[tokio::test]
async fn create_rejects_malformed_function_inputs_json() {
    let mock = MockUserDefinedFunctionServiceImpl::new();
    let (server, _h) = server_with_mock(mock, true, false).await;

    let err = server
        .create_user_defined_function(create_params("{not json"))
        .await
        .expect_err("expected malformed JSON to be rejected");

    assert_eq!(err.code, ErrorCode::INVALID_PARAMS);
    assert!(err.message.contains("`function_inputs_json`"));
}

#[tokio::test]
async fn create_rejects_unknown_data_type() {
    let mock = MockUserDefinedFunctionServiceImpl::new();
    let (server, _h) = server_with_mock(mock, true, false).await;

    let err = server
        .create_user_defined_function(create_params(
            "[{\"identifier\":\"x\",\"data_type\":\"complex\"}]",
        ))
        .await
        .expect_err("expected unknown data_type to be rejected");

    assert_eq!(err.code, ErrorCode::INVALID_PARAMS);
    assert!(err.message.contains("complex"));
}

#[tokio::test]
async fn create_returns_the_new_function_and_next_step() {
    let mut mock = MockUserDefinedFunctionServiceImpl::new();
    mock.expect_create_user_defined_function()
        .times(1)
        .withf(|req| {
            let req = req.get_ref();
            req.name == "rms"
                && req.function_inputs.len() == 1
                && req.function_inputs[0].identifier == "x"
                && req.function_inputs[0].data_type == i32::from(FunctionDataType::Numeric)
                && !req.function_inputs[0].constant
        })
        .returning(|req| {
            let req = req.into_inner();
            Ok(Response::new(CreateUserDefinedFunctionResponse {
                user_defined_function: Some(UserDefinedFunction {
                    user_defined_function_id: "f1".into(),
                    user_defined_function_version_id: "v1".into(),
                    version: 1,
                    name: req.name,
                    function_inputs: req.function_inputs,
                    ..Default::default()
                }),
            }))
        });

    let (server, _h) = server_with_mock(mock, true, false).await;

    let resp = server
        .create_user_defined_function(create_params(
            "[{\"identifier\":\"x\",\"data_type\":\"numeric\",\"constant\":false}]",
        ))
        .await
        .expect("create_user_defined_function failed");

    let body = structured(resp);
    assert_eq!(body["user_defined_function_id"], "f1");
    assert_eq!(body["user_defined_function"]["name"], "rms");
    assert!(
        body["next_step"]
            .as_str()
            .expect("next_step")
            .contains("f1")
    );
}

#[tokio::test]
async fn update_blocked_without_allow_destructive() {
    let mock = MockUserDefinedFunctionServiceImpl::new();
    let (server, _h) = server_with_mock(mock, false, false).await;

    let err = server
        .update_user_defined_function(update_params(None, Some("updated"), None))
        .await
        .expect_err("expected destructive gate to reject the call");

    assert_eq!(err.code, ErrorCode::INVALID_REQUEST);
    assert!(err.message.contains("--allow-destructive"));
    let data = err
        .data
        .expect("destructive gate should return remediation data");
    assert_eq!(
        data["remediation_command"],
        "sift-cli agent update --allow-destructive"
    );
}

#[tokio::test]
async fn update_rejects_an_empty_mask() {
    let mock = MockUserDefinedFunctionServiceImpl::new();
    let (server, _h) = server_with_mock(mock, true, true).await;

    let err = server
        .update_user_defined_function(update_params(None, None, None))
        .await
        .expect_err("expected an empty update to be rejected");

    assert_eq!(err.code, ErrorCode::INVALID_PARAMS);
    assert!(err.message.contains("at least one"));
}

#[tokio::test]
async fn update_rejects_name_combined_with_other_fields() {
    // The API applies a name change on its own and ignores the rest, so a mixed
    // request would silently drop half the caller's intent.
    let mock = MockUserDefinedFunctionServiceImpl::new();
    let (server, _h) = server_with_mock(mock, true, true).await;

    let err = server
        .update_user_defined_function(update_params(Some("rms_v2"), Some("updated"), None))
        .await
        .expect_err("expected a mixed name update to be rejected");

    assert_eq!(err.code, ErrorCode::INVALID_PARAMS);
    assert!(err.message.contains("`name`"));
}

#[tokio::test]
async fn update_rejects_malformed_function_inputs_json() {
    let mock = MockUserDefinedFunctionServiceImpl::new();
    let (server, _h) = server_with_mock(mock, true, true).await;

    let err = server
        .update_user_defined_function(update_params(None, None, Some("[{\"identifier\":}]")))
        .await
        .expect_err("expected malformed JSON to be rejected");

    assert_eq!(err.code, ErrorCode::INVALID_PARAMS);
    assert!(err.message.contains("`function_inputs_json`"));
}

#[tokio::test]
async fn update_returns_the_new_version() {
    let mut mock = MockUserDefinedFunctionServiceImpl::new();
    mock.expect_update_user_defined_function()
        .times(1)
        .withf(|req| {
            let mask = req.get_ref().update_mask.as_ref().expect("mask present");
            mask.paths == vec!["description".to_string()]
        })
        .returning(|req| {
            let req = req.into_inner();
            let mut function = req.user_defined_function.expect("function present");
            function.version = 4;
            function.user_defined_function_version_id = "v4".into();
            Ok(Response::new(UpdateUserDefinedFunctionResponse {
                user_defined_function: Some(function),
            }))
        });

    let (server, _h) = server_with_mock(mock, true, true).await;

    let resp = server
        .update_user_defined_function(update_params(None, Some("updated"), None))
        .await
        .expect("update_user_defined_function failed");

    let body = structured(resp);
    assert_eq!(body["user_defined_function"]["version"], 4);
    assert_eq!(
        body["user_defined_function"]["userDefinedFunctionVersionId"],
        "v4"
    );
    assert!(body["next_step"].as_str().expect("next_step").contains("4"));
}

#[tokio::test]
async fn archive_blocked_without_allow_destructive() {
    let mock = MockUserDefinedFunctionServiceImpl::new();
    let (server, _h) = server_with_mock(mock, true, false).await;

    let err = server
        .archive_user_defined_function(archive_params("f1"))
        .await
        .expect_err("expected destructive gate to reject the call");

    assert_eq!(err.code, ErrorCode::INVALID_REQUEST);
    assert!(err.message.contains("--allow-destructive"));
}

#[tokio::test]
async fn unarchive_blocked_without_allow_destructive() {
    let mock = MockUserDefinedFunctionServiceImpl::new();
    let (server, _h) = server_with_mock(mock, true, false).await;

    let err = server
        .unarchive_user_defined_function(archive_params("f1"))
        .await
        .expect_err("expected destructive gate to reject the call");

    assert_eq!(err.code, ErrorCode::INVALID_REQUEST);
    assert!(err.message.contains("--allow-destructive"));
}

#[tokio::test]
async fn archive_rejects_an_empty_id() {
    let mock = MockUserDefinedFunctionServiceImpl::new();
    let (server, _h) = server_with_mock(mock, true, true).await;

    let err = server
        .archive_user_defined_function(archive_params(""))
        .await
        .expect_err("expected an empty id to be rejected");

    assert_eq!(err.code, ErrorCode::INVALID_PARAMS);
}

#[tokio::test]
async fn archive_sets_the_archive_flag_and_reports_it() {
    let mut mock = MockUserDefinedFunctionServiceImpl::new();
    mock.expect_update_user_defined_function()
        .times(1)
        .withf(|req| {
            let req = req.get_ref();
            let function = req
                .user_defined_function
                .as_ref()
                .expect("function present");
            let mask = req.update_mask.as_ref().expect("mask present");
            function.is_archived && mask.paths == vec!["is_archived".to_string()]
        })
        .returning(|req| {
            let req = req.into_inner();
            Ok(Response::new(UpdateUserDefinedFunctionResponse {
                user_defined_function: req.user_defined_function,
            }))
        });

    let (server, _h) = server_with_mock(mock, true, true).await;

    let resp = server
        .archive_user_defined_function(archive_params("f1"))
        .await
        .expect("archive_user_defined_function failed");

    let body = structured(resp);
    assert_eq!(body["archived"], true);
    assert_eq!(body["user_defined_function"]["isArchived"], true);
}

#[tokio::test]
async fn unarchive_clears_the_archive_flag_and_reports_it() {
    let mut mock = MockUserDefinedFunctionServiceImpl::new();
    mock.expect_update_user_defined_function()
        .times(1)
        .withf(|req| {
            let req = req.get_ref();
            let function = req
                .user_defined_function
                .as_ref()
                .expect("function present");
            let mask = req.update_mask.as_ref().expect("mask present");
            !function.is_archived && mask.paths == vec!["is_archived".to_string()]
        })
        .returning(|req| {
            let req = req.into_inner();
            Ok(Response::new(UpdateUserDefinedFunctionResponse {
                user_defined_function: req.user_defined_function,
            }))
        });

    let (server, _h) = server_with_mock(mock, true, true).await;

    let resp = server
        .unarchive_user_defined_function(archive_params("f2"))
        .await
        .expect("unarchive_user_defined_function failed");

    let body = structured(resp);
    assert_eq!(body["unarchived"], true);
}

#[tokio::test]
async fn create_rejects_the_proto_enum_spelling_of_data_type() {
    // The description documents `numeric`, `string`, and `bool` only. Accepting
    // the raw proto enum names as well would be undocumented behavior.
    let mock = MockUserDefinedFunctionServiceImpl::new();
    let (server, _h) = server_with_mock(mock, true, false).await;

    let err = server
        .create_user_defined_function(create_params(
            "[{\"identifier\":\"x\",\"data_type\":\"FUNCTION_DATA_TYPE_NUMERIC\"}]",
        ))
        .await
        .expect_err("expected the proto enum spelling to be rejected");

    assert_eq!(err.code, ErrorCode::INVALID_PARAMS);
    assert!(
        err.message
            .contains("expected `numeric`, `string`, or `bool`")
    );
}

#[tokio::test]
async fn create_rejects_a_camel_case_data_type_key() {
    // Only the documented `data_type` key is accepted.
    let mock = MockUserDefinedFunctionServiceImpl::new();
    let (server, _h) = server_with_mock(mock, true, false).await;

    let err = server
        .create_user_defined_function(create_params(
            "[{\"identifier\":\"x\",\"dataType\":\"numeric\"}]",
        ))
        .await
        .expect_err("expected a camelCase key to be rejected");

    assert_eq!(err.code, ErrorCode::INVALID_PARAMS);
    assert!(err.message.contains("`function_inputs_json`"));
}

#[tokio::test]
async fn create_maps_every_documented_data_type() {
    let mut mock = MockUserDefinedFunctionServiceImpl::new();
    mock.expect_create_user_defined_function()
        .times(1)
        .withf(|req| {
            let inputs = &req.get_ref().function_inputs;
            inputs
                == &vec![
                    numeric_input("x"),
                    FunctionInput {
                        identifier: "label".into(),
                        data_type: FunctionDataType::String.into(),
                        constant: true,
                    },
                    FunctionInput {
                        identifier: "flag".into(),
                        data_type: FunctionDataType::Bool.into(),
                        constant: false,
                    },
                ]
        })
        .returning(|_| {
            Ok(Response::new(CreateUserDefinedFunctionResponse {
                user_defined_function: Some(UserDefinedFunction {
                    user_defined_function_id: "f1".into(),
                    ..Default::default()
                }),
            }))
        });

    let (server, _h) = server_with_mock(mock, true, false).await;

    server
        .create_user_defined_function(create_params(
            "[{\"identifier\":\"x\",\"data_type\":\"numeric\"},\
              {\"identifier\":\"label\",\"data_type\":\"STRING\",\"constant\":true},\
              {\"identifier\":\"flag\",\"data_type\":\"bool\",\"constant\":false}]",
        ))
        .await
        .expect("create_user_defined_function failed");
}
