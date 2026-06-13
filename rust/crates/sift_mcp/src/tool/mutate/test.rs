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

use sift_rs::assets::v1::{Asset, ListAssetsResponse, asset_service_server::AssetServiceServer};
use sift_rs::calculated_channels::v2::{
    CalculatedChannel, CreateCalculatedChannelResponse,
    calculated_channel_asset_configuration::AssetScope,
    calculated_channel_query_configuration::Query,
    calculated_channel_service_server::CalculatedChannelServiceServer,
};
use sift_rs::metadata::v1::{MetadataKeyType, metadata_value::Value as MetadataValueInner};
use sift_test_util::mock::{
    assets::v1::MockAssetServiceImpl, calculated_channels::v2::MockCalculatedChannelServiceImpl,
};

use super::{
    CreateCalculatedChannelParams, CreateUserDefinedFunctionParams, UpdateCalculatedChannelParams,
    UpdateUserDefinedFunctionParams,
};
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

async fn server_with_cc_mocks(
    cc_mock: MockCalculatedChannelServiceImpl,
    asset_mock: MockAssetServiceImpl,
) -> (SiftMcpServer, JoinHandle<()>) {
    let (client, server) = tokio::io::duplex(1024);
    let channel = memory_sift_channel(client).await;

    let handle = tokio::spawn(async move {
        Server::builder()
            .add_service(CalculatedChannelServiceServer::new(cc_mock))
            .add_service(AssetServiceServer::new(asset_mock))
            .serve_with_incoming(tokio_stream::once(Ok::<_, std::io::Error>(server)))
            .await
            .unwrap();
    });

    (
        SiftMcpServer::new(channel, String::from("https://api.test.local")),
        handle,
    )
}

fn cc_create_params() -> CreateCalculatedChannelParams {
    CreateCalculatedChannelParams {
        name: "derived".into(),
        description: None,
        units: None,
        client_key: None,
        user_notes: None,
        expression: "$1 * 2".into(),
        all_assets: Some(true),
        asset_ids: None,
        asset_names: None,
        tag_ids: None,
        channel_references_json: "[{\"channel_reference\":\"$1\",\"channel_identifier\":\"rpm\"}]"
            .into(),
        metadata: None,
    }
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

#[tokio::test]
async fn create_calculated_channel_builds_config_and_returns_channel() {
    let mut cc_mock = MockCalculatedChannelServiceImpl::new();
    cc_mock
        .expect_create_calculated_channel()
        .withf(|req| {
            let req = req.get_ref();
            let config = req.calculated_channel_configuration.as_ref().unwrap();
            let all_assets = matches!(
                config
                    .asset_configuration
                    .as_ref()
                    .and_then(|a| a.asset_scope.as_ref()),
                Some(AssetScope::AllAssets(true))
            );
            let sel_ok = match config
                .query_configuration
                .as_ref()
                .and_then(|q| q.query.as_ref())
            {
                Some(Query::Sel(sel)) => {
                    sel.expression == "$1 * 2"
                        && sel.expression_channel_references.len() == 1
                        && sel.expression_channel_references[0].channel_reference == "$1"
                        && sel.expression_channel_references[0].channel_identifier == "rpm"
                }
                None => false,
            };
            req.name == "derived" && all_assets && sel_ok
        })
        .returning(|_| {
            Ok(Response::new(CreateCalculatedChannelResponse {
                calculated_channel: Some(CalculatedChannel {
                    calculated_channel_id: "c9".into(),
                    ..Default::default()
                }),
                inapplicable_assets: vec![],
            }))
        });

    let (server, _h) = server_with_cc_mocks(cc_mock, MockAssetServiceImpl::new()).await;

    let resp = server
        .create_calculated_channel(Parameters(cc_create_params()))
        .await
        .expect("create failed");

    let value = resp.structured_content.expect("structured content");
    assert_eq!(value["calculated_channel"]["calculatedChannelId"], "c9");
}

#[tokio::test]
async fn create_calculated_channel_without_asset_scope_is_invalid() {
    let (server, _h) = server_with_cc_mocks(
        MockCalculatedChannelServiceImpl::new(),
        MockAssetServiceImpl::new(),
    )
    .await;

    let mut params = cc_create_params();
    params.all_assets = None;

    let err = server
        .create_calculated_channel(Parameters(params))
        .await
        .expect_err("expected invalid params");

    assert_eq!(err.code, ErrorCode::INVALID_PARAMS);
}

#[tokio::test]
async fn create_calculated_channel_with_both_scopes_is_invalid() {
    let (server, _h) = server_with_cc_mocks(
        MockCalculatedChannelServiceImpl::new(),
        MockAssetServiceImpl::new(),
    )
    .await;

    let mut params = cc_create_params();
    params.asset_ids = Some(vec!["a1".into()]);

    let err = server
        .create_calculated_channel(Parameters(params))
        .await
        .expect_err("expected invalid params");

    assert_eq!(err.code, ErrorCode::INVALID_PARAMS);
}

#[tokio::test]
async fn create_calculated_channel_with_malformed_references_is_invalid() {
    let (server, _h) = server_with_cc_mocks(
        MockCalculatedChannelServiceImpl::new(),
        MockAssetServiceImpl::new(),
    )
    .await;

    let mut params = cc_create_params();
    params.channel_references_json = "not json".into();

    let err = server
        .create_calculated_channel(Parameters(params))
        .await
        .expect_err("expected invalid params");

    assert_eq!(err.code, ErrorCode::INVALID_PARAMS);
}

#[tokio::test]
async fn create_calculated_channel_reference_with_both_targets_is_invalid() {
    let (server, _h) = server_with_cc_mocks(
        MockCalculatedChannelServiceImpl::new(),
        MockAssetServiceImpl::new(),
    )
    .await;

    let mut params = cc_create_params();
    params.channel_references_json =
        "[{\"channel_reference\":\"$1\",\"channel_identifier\":\"rpm\",\"calculated_channel_version_id\":\"v1\"}]"
            .into();

    let err = server
        .create_calculated_channel(Parameters(params))
        .await
        .expect_err("expected invalid params");

    assert_eq!(err.code, ErrorCode::INVALID_PARAMS);
}

#[tokio::test]
async fn create_calculated_channel_resolves_asset_names() {
    let mut cc_mock = MockCalculatedChannelServiceImpl::new();
    cc_mock
        .expect_create_calculated_channel()
        .withf(|req| {
            let config = req
                .get_ref()
                .calculated_channel_configuration
                .as_ref()
                .unwrap();
            matches!(
                config.asset_configuration.as_ref().and_then(|a| a.asset_scope.as_ref()),
                Some(AssetScope::Selection(sel)) if sel.asset_ids == vec!["resolved-id".to_string()]
            )
        })
        .returning(|_| {
            Ok(Response::new(CreateCalculatedChannelResponse {
                calculated_channel: Some(CalculatedChannel {
                    calculated_channel_id: "c9".into(),
                    ..Default::default()
                }),
                inapplicable_assets: vec![],
            }))
        });

    let mut asset_mock = MockAssetServiceImpl::new();
    asset_mock
        .expect_list_assets()
        .withf(|req| req.get_ref().filter == "name in [\"engine\"]")
        .returning(|_| {
            Ok(Response::new(ListAssetsResponse {
                assets: vec![Asset {
                    asset_id: "resolved-id".into(),
                    name: "engine".into(),
                    ..Default::default()
                }],
                next_page_token: String::new(),
            }))
        });

    let (server, _h) = server_with_cc_mocks(cc_mock, asset_mock).await;

    let mut params = cc_create_params();
    params.all_assets = None;
    params.asset_names = Some(vec!["engine".into()]);

    server
        .create_calculated_channel(Parameters(params))
        .await
        .expect("create failed");
}

#[tokio::test]
async fn update_calculated_channel_without_identifier_is_invalid() {
    let (server, _h) = server_with_cc_mocks(
        MockCalculatedChannelServiceImpl::new(),
        MockAssetServiceImpl::new(),
    )
    .await;

    let err = server
        .update_calculated_channel(Parameters(UpdateCalculatedChannelParams {
            calculated_channel_id: None,
            client_key: None,
            name: Some("x".into()),
            description: None,
            units: None,
            expression: None,
            channel_references_json: None,
            all_assets: None,
            asset_ids: None,
            asset_names: None,
            tag_ids: None,
            metadata: None,
            user_notes: None,
        }))
        .await
        .expect_err("expected invalid params");

    assert_eq!(err.code, ErrorCode::INVALID_PARAMS);
}

#[tokio::test]
async fn create_calculated_channel_with_unresolvable_asset_name_is_invalid() {
    let cc_mock = MockCalculatedChannelServiceImpl::new();
    let mut asset_mock = MockAssetServiceImpl::new();
    // The named asset does not exist: list returns no matches.
    asset_mock.expect_list_assets().returning(|_| {
        Ok(Response::new(ListAssetsResponse {
            assets: vec![],
            next_page_token: String::new(),
        }))
    });

    let (server, _h) = server_with_cc_mocks(cc_mock, asset_mock).await;

    let mut params = cc_create_params();
    params.all_assets = None;
    params.asset_names = Some(vec!["ghost".into()]);

    let err = server
        .create_calculated_channel(Parameters(params))
        .await
        .expect_err("expected invalid params");

    assert_eq!(err.code, ErrorCode::INVALID_PARAMS);
}
