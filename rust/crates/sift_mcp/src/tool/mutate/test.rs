use rmcp::{handler::server::wrapper::Parameters, model::ErrorCode};
use sift_rs::{
    common::r#type::v1::{FunctionDataType, UserDefinedFunction},
    user_defined_functions::v1::{
        CreateUserDefinedFunctionResponse, GetUserDefinedFunctionResponse,
        UpdateUserDefinedFunctionResponse,
        user_defined_function_service_server::UserDefinedFunctionServiceServer,
    },
};
use sift_test_util::{
    grpc::memory_sift_channel, mock::user_defined_functions::v1::MockUserDefinedFunctionServiceImpl,
};
use tokio::task::JoinHandle;
use tonic::{Response, transport::Server};

use sift_rs::metadata::v1::{MetadataKeyType, metadata_value::Value as MetadataValueInner};

use super::{CreateUserDefinedFunctionParams, UpdateUserDefinedFunctionParams};
use crate::server::SiftMcpServer;
use crate::tool::common::{MetadataEntry, MetadataScalar};

async fn server_with_udf_mock(
    mock: MockUserDefinedFunctionServiceImpl,
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
        SiftMcpServer::new(channel, String::from("https://api.test.local")),
        handle,
    )
}

fn create_params() -> CreateUserDefinedFunctionParams {
    CreateUserDefinedFunctionParams {
        name: "scale".into(),
        description: None,
        expression: "x * 2".into(),
        input_identifiers: vec!["x".into()],
        input_data_types: vec!["numeric".into()],
        input_constants: vec![false],
        user_notes: None,
        metadata: None,
    }
}

#[tokio::test]
async fn create_maps_inputs_and_returns_function() {
    let mut mock = MockUserDefinedFunctionServiceImpl::new();
    mock.expect_create_user_defined_function()
        .withf(|req| {
            let req = req.get_ref();
            req.name == "scale"
                && req.function_inputs.len() == 1
                && req.function_inputs[0].identifier == "x"
                && req.function_inputs[0].data_type == FunctionDataType::Numeric as i32
                && !req.function_inputs[0].constant
        })
        .returning(|_| {
            Ok(Response::new(CreateUserDefinedFunctionResponse {
                user_defined_function: Some(UserDefinedFunction {
                    user_defined_function_id: "u9".into(),
                    name: "scale".into(),
                    ..Default::default()
                }),
            }))
        });

    let (server, _h) = server_with_udf_mock(mock).await;

    let resp = server
        .create_user_defined_function(Parameters(create_params()))
        .await
        .expect("create failed");

    let value = resp.structured_content.expect("structured content");
    assert_eq!(
        value["user_defined_function"]["userDefinedFunctionId"],
        "u9"
    );
}

#[tokio::test]
async fn create_passes_converted_metadata_to_request() {
    let mut mock = MockUserDefinedFunctionServiceImpl::new();
    mock.expect_create_user_defined_function()
        .withf(|req| {
            let req = req.get_ref();
            req.metadata.len() == 1
                && req.metadata[0].key.as_ref().map(|k| k.name.as_str()) == Some("team")
                && req.metadata[0].key.as_ref().map(|k| k.r#type)
                    == Some(MetadataKeyType::String as i32)
                && matches!(
                    &req.metadata[0].value,
                    Some(MetadataValueInner::StringValue(s)) if s == "controls"
                )
        })
        .returning(|_| {
            Ok(Response::new(CreateUserDefinedFunctionResponse {
                user_defined_function: Some(UserDefinedFunction {
                    user_defined_function_id: "u9".into(),
                    ..Default::default()
                }),
            }))
        });

    let (server, _h) = server_with_udf_mock(mock).await;

    let mut params = create_params();
    params.metadata = Some(vec![MetadataEntry {
        name: "team".into(),
        value: MetadataScalar::String("controls".into()),
    }]);

    server
        .create_user_defined_function(Parameters(params))
        .await
        .expect("create failed");
}

#[tokio::test]
async fn create_parses_data_types_case_insensitively_with_aliases() {
    let mut mock = MockUserDefinedFunctionServiceImpl::new();
    mock.expect_create_user_defined_function()
        .withf(|req| {
            let inputs = &req.get_ref().function_inputs;
            inputs.len() == 3
                && inputs[0].data_type == FunctionDataType::Bool as i32 // "Boolean"
                && inputs[1].data_type == FunctionDataType::String as i32 // "STRING"
                && inputs[2].data_type == FunctionDataType::Bool as i32 // "bool"
        })
        .returning(|_| {
            Ok(Response::new(CreateUserDefinedFunctionResponse {
                user_defined_function: Some(UserDefinedFunction {
                    user_defined_function_id: "u9".into(),
                    ..Default::default()
                }),
            }))
        });

    let (server, _h) = server_with_udf_mock(mock).await;

    let mut params = create_params();
    params.input_identifiers = vec!["a".into(), "b".into(), "c".into()];
    params.input_data_types = vec!["Boolean".into(), "STRING".into(), "bool".into()];
    params.input_constants = vec![false, false, false];

    server
        .create_user_defined_function(Parameters(params))
        .await
        .expect("create failed");
}

#[tokio::test]
async fn create_with_mismatched_input_arrays_is_invalid() {
    let (server, _h) = server_with_udf_mock(MockUserDefinedFunctionServiceImpl::new()).await;

    let mut params = create_params();
    params.input_constants = vec![]; // length 0 vs identifiers length 1

    let err = server
        .create_user_defined_function(Parameters(params))
        .await
        .expect_err("expected invalid params");

    assert_eq!(err.code, ErrorCode::INVALID_PARAMS);
}

#[tokio::test]
async fn create_with_unknown_data_type_is_invalid() {
    let (server, _h) = server_with_udf_mock(MockUserDefinedFunctionServiceImpl::new()).await;

    let mut params = create_params();
    params.input_data_types = vec!["float".into()];

    let err = server
        .create_user_defined_function(Parameters(params))
        .await
        .expect_err("expected invalid params");

    assert_eq!(err.code, ErrorCode::INVALID_PARAMS);
}

#[tokio::test]
async fn update_with_partial_input_arrays_is_invalid() {
    let (server, _h) = server_with_udf_mock(MockUserDefinedFunctionServiceImpl::new()).await;

    let err = server
        .update_user_defined_function(Parameters(UpdateUserDefinedFunctionParams {
            user_defined_function_id: "u1".into(),
            name: None,
            description: None,
            expression: None,
            input_identifiers: Some(vec!["x".into()]),
            input_data_types: None,
            input_constants: None,
            metadata: None,
        }))
        .await
        .expect_err("expected invalid params");

    assert_eq!(err.code, ErrorCode::INVALID_PARAMS);
}

#[tokio::test]
async fn update_changes_single_field() {
    let mut mock = MockUserDefinedFunctionServiceImpl::new();
    mock.expect_get_user_defined_function().returning(|_| {
        Ok(Response::new(GetUserDefinedFunctionResponse {
            user_defined_function: Some(UserDefinedFunction {
                user_defined_function_id: "u1".into(),
                name: "old".into(),
                description: "keep".into(),
                ..Default::default()
            }),
        }))
    });
    mock.expect_update_user_defined_function()
        .withf(|req| {
            let req = req.get_ref();
            let mask = req.update_mask.as_ref().unwrap();
            mask.paths == vec!["name".to_string()]
                && req.user_defined_function.as_ref().unwrap().name == "new"
        })
        .returning(|req| {
            Ok(Response::new(UpdateUserDefinedFunctionResponse {
                user_defined_function: req.into_inner().user_defined_function,
            }))
        });

    let (server, _h) = server_with_udf_mock(mock).await;

    let resp = server
        .update_user_defined_function(Parameters(UpdateUserDefinedFunctionParams {
            user_defined_function_id: "u1".into(),
            name: Some("new".into()),
            description: None,
            expression: None,
            input_identifiers: None,
            input_data_types: None,
            input_constants: None,
            metadata: None,
        }))
        .await
        .expect("update failed");

    let value = resp.structured_content.expect("structured content");
    assert_eq!(value["user_defined_function"]["name"], "new");
}
