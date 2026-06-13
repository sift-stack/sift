use rmcp::{handler::server::wrapper::Parameters, model::ErrorCode};
use sift_rs::{
    common::r#type::v1::UserDefinedFunction,
    user_defined_functions::v1::{
        GetUserDefinedFunctionResponse,
        user_defined_function_service_server::UserDefinedFunctionServiceServer,
    },
};
use sift_test_util::{
    grpc::memory_sift_channel, mock::user_defined_functions::v1::MockUserDefinedFunctionServiceImpl,
};
use tokio::task::JoinHandle;
use tonic::{Response, transport::Server};

use super::GetUserDefinedFunctionParams;
use crate::server::SiftMcpServer;

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

#[tokio::test]
async fn get_by_id_returns_structured_function() {
    let mut mock = MockUserDefinedFunctionServiceImpl::new();
    mock.expect_get_user_defined_function()
        .withf(|req| {
            let req = req.get_ref();
            req.user_defined_function_id == "u1" && req.name.is_empty()
        })
        .returning(|_| {
            Ok(Response::new(GetUserDefinedFunctionResponse {
                user_defined_function: Some(UserDefinedFunction {
                    user_defined_function_id: "u1".into(),
                    name: "my_func".into(),
                    ..Default::default()
                }),
            }))
        });

    let (server, _h) = server_with_udf_mock(mock).await;

    let resp = server
        .get_user_defined_function(Parameters(GetUserDefinedFunctionParams {
            user_defined_function_id: Some("u1".into()),
            name: None,
        }))
        .await
        .expect("get failed");

    let value = resp.structured_content.expect("structured content");
    assert_eq!(value["user_defined_function"]["name"], "my_func");
}

#[tokio::test]
async fn get_with_neither_id_nor_name_is_invalid() {
    let (server, _h) = server_with_udf_mock(MockUserDefinedFunctionServiceImpl::new()).await;

    let err = server
        .get_user_defined_function(Parameters(GetUserDefinedFunctionParams {
            user_defined_function_id: None,
            name: None,
        }))
        .await
        .expect_err("expected invalid params");

    assert_eq!(err.code, ErrorCode::INVALID_PARAMS);
}

#[tokio::test]
async fn get_with_both_id_and_name_is_invalid() {
    let (server, _h) = server_with_udf_mock(MockUserDefinedFunctionServiceImpl::new()).await;

    let err = server
        .get_user_defined_function(Parameters(GetUserDefinedFunctionParams {
            user_defined_function_id: Some("u1".into()),
            name: Some("my_func".into()),
        }))
        .await
        .expect_err("expected invalid params");

    assert_eq!(err.code, ErrorCode::INVALID_PARAMS);
}

#[tokio::test]
async fn get_missing_function_maps_to_resource_not_found() {
    let mut mock = MockUserDefinedFunctionServiceImpl::new();
    mock.expect_get_user_defined_function().returning(|_| {
        Ok(Response::new(GetUserDefinedFunctionResponse {
            user_defined_function: None,
        }))
    });

    let (server, _h) = server_with_udf_mock(mock).await;

    let err = server
        .get_user_defined_function(Parameters(GetUserDefinedFunctionParams {
            user_defined_function_id: Some("missing".into()),
            name: None,
        }))
        .await
        .expect_err("expected not found");

    assert_eq!(err.code, ErrorCode::RESOURCE_NOT_FOUND);
}
