use std::time::Duration;

use rmcp::{ServerHandler, ServiceExt};
use serde_json::Value;
use sift_rs::assets::v1::{ListAssetsResponse, asset_service_server::AssetServiceServer};
use sift_test_util::{grpc::memory_sift_channel, mock::assets::v1::MockAssetServiceImpl};
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader, DuplexStream},
    sync::watch,
    task::JoinHandle,
};
use tonic::{Response, transport::Server};

use crate::UpdateCheck;

use super::SiftMcpServer;

const CLI_VERSION: &str = "7.8.9";
const INSTALL_COMMAND: &str =
    "curl --proto '=https' --tlsv1.2 -LsSf https://example.test/sift_cli-installer.sh | sh";
const EXPECTED_BASE_INSTRUCTIONS: &str = concat!(
    "Call `check_for_updates` once before the first other Sift tool in each session. ",
    "If it reports `update_available`, relay its `message` and exact ",
    "`install_command` to the user. If the check is unavailable, continue with ",
    "the requested Sift task. Use Sift tools for telemetry discovery, analysis, ",
    "and ingestion. Run `sift-cli agent doctor` for read-only integration ",
    "diagnosis, `sift-cli agent install` for first setup, and `sift-cli agent ",
    "update` to refresh every detected client together. Never enable destructive ",
    "tools without explicit user approval. Result objects follow proto3 JSON ",
    "rules: fields at their default value (false, 0, empty string/list) are ",
    "omitted, so a missing boolean key means false, not unknown."
);

fn update_available() -> UpdateCheck {
    UpdateCheck::UpdateAvailable {
        current_version: "0.3.0".to_string(),
        latest_version: "0.4.0".to_string(),
        install_command: INSTALL_COMMAND.to_string(),
        message: format!(
            "sift-cli 0.3.0 is outdated; latest is 0.4.0\nUpdate with:\n\n  {INSTALL_COMMAND}"
        ),
    }
}

fn receiver(status: UpdateCheck) -> watch::Receiver<UpdateCheck> {
    watch::channel(status).1
}

async fn server_with_update_check(
    update_check: Option<watch::Receiver<UpdateCheck>>,
    asset_tool_calls: usize,
) -> (SiftMcpServer, JoinHandle<()>) {
    let mut mock = MockAssetServiceImpl::new();
    mock.expect_list_assets()
        .times(asset_tool_calls)
        .returning(|_| Ok(Response::new(ListAssetsResponse::default())));

    let (client, server) = tokio::io::duplex(1024);
    let channel = memory_sift_channel(client).await;
    let handle = tokio::spawn(async move {
        Server::builder()
            .add_service(AssetServiceServer::new(mock))
            .serve_with_incoming(tokio_stream::once(Ok::<_, std::io::Error>(server)))
            .await
            .unwrap();
    });

    (
        SiftMcpServer::new_with_update_check(
            channel,
            "https://app.test.local".to_string(),
            false,
            false,
            CLI_VERSION.to_string(),
            update_check,
        ),
        handle,
    )
}

async fn initialized_client(
    update_check: Option<watch::Receiver<UpdateCheck>>,
    asset_tool_calls: usize,
) -> (
    BufReader<tokio::io::ReadHalf<DuplexStream>>,
    tokio::io::WriteHalf<DuplexStream>,
    JoinHandle<()>,
) {
    let (server, grpc_handle) = server_with_update_check(update_check, asset_tool_calls).await;
    let (server_transport, client_transport) = tokio::io::duplex(8192);
    let mcp_handle = tokio::spawn(async move {
        let service = server.serve(server_transport).await.unwrap();
        service.waiting().await.unwrap();
        grpc_handle.abort();
    });
    let (reader, mut writer) = tokio::io::split(client_transport);
    let reader = BufReader::new(reader);

    let request = serde_json::json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "initialize",
        "params": {
            "protocolVersion": "2025-11-25",
            "capabilities": {},
            "clientInfo": { "name": "test-client", "version": "0.0.1" }
        }
    });
    writer
        .write_all(format!("{request}\n").as_bytes())
        .await
        .unwrap();

    (reader, writer, mcp_handle)
}

async fn read_json(reader: &mut BufReader<tokio::io::ReadHalf<DuplexStream>>) -> Value {
    let mut response = String::new();
    reader.read_line(&mut response).await.unwrap();
    serde_json::from_str(&response).unwrap()
}

async fn finish(
    reader: BufReader<tokio::io::ReadHalf<DuplexStream>>,
    writer: tokio::io::WriteHalf<DuplexStream>,
    server: JoinHandle<()>,
) {
    drop(writer);
    drop(reader);
    server.await.unwrap();
}

#[tokio::test]
async fn initialize_prepends_a_cached_update_notice() {
    let expected = update_available();
    let expected_message = expected.message();
    let (mut reader, writer, server) = initialized_client(Some(receiver(expected)), 0).await;

    let response = read_json(&mut reader).await;
    let instructions = response["result"]["instructions"].as_str().unwrap();

    assert!(instructions.starts_with(&expected_message));
    assert!(instructions[..instructions.len().min(512)].contains(INSTALL_COMMAND));
    assert!(instructions.ends_with(EXPECTED_BASE_INSTRUCTIONS));
    assert!(instructions.len() < 2048);

    finish(reader, writer, server).await;
}

#[tokio::test]
async fn initialize_instructs_the_agent_to_check_once() {
    let current = UpdateCheck::Current {
        current_version: "0.4.0".to_string(),
        latest_version: "0.4.0".to_string(),
    };
    let (mut reader, writer, server) = initialized_client(Some(receiver(current)), 0).await;

    let response = read_json(&mut reader).await;

    assert_eq!(
        response["result"]["instructions"],
        EXPECTED_BASE_INSTRUCTIONS
    );

    finish(reader, writer, server).await;
}

#[tokio::test]
async fn get_info_reports_the_injected_cli_version() {
    let current = UpdateCheck::Current {
        current_version: "0.4.0".to_string(),
        latest_version: "0.4.0".to_string(),
    };
    let (server, grpc_handle) = server_with_update_check(Some(receiver(current)), 0).await;

    assert_eq!(
        ServerHandler::get_info(&server).server_info.version,
        CLI_VERSION
    );

    grpc_handle.abort();
}

#[tokio::test]
async fn update_tool_waits_for_the_background_check_and_returns_the_install_command() {
    let (sender, update_check) = watch::channel(UpdateCheck::Checking {
        current_version: "0.3.0".to_string(),
    });
    let (mut reader, mut writer, server) = initialized_client(Some(update_check), 0).await;
    let _initialize = read_json(&mut reader).await;
    writer
        .write_all(b"{\"jsonrpc\":\"2.0\",\"method\":\"notifications/initialized\"}\n")
        .await
        .unwrap();
    writer
        .write_all(
            b"{\"jsonrpc\":\"2.0\",\"id\":2,\"method\":\"tools/call\",\"params\":{\"name\":\"check_for_updates\",\"arguments\":{}}}\n",
        )
        .await
        .unwrap();

    tokio::spawn(async move {
        tokio::time::sleep(Duration::from_millis(25)).await;
        sender.send(update_available()).unwrap();
    });
    let response = read_json(&mut reader).await;

    assert_eq!(
        response["result"]["structuredContent"]["status"],
        "update_available"
    );
    assert_eq!(
        response["result"]["structuredContent"]["install_command"],
        INSTALL_COMMAND
    );
    assert!(
        response["result"]["content"][0]["text"]
            .as_str()
            .unwrap()
            .contains(INSTALL_COMMAND)
    );

    finish(reader, writer, server).await;
}

#[tokio::test]
async fn update_tool_fails_open_when_the_background_check_closes() {
    let (sender, update_check) = watch::channel(UpdateCheck::Checking {
        current_version: "0.3.0".to_string(),
    });
    let (mut reader, mut writer, server) = initialized_client(Some(update_check), 1).await;
    let _initialize = read_json(&mut reader).await;
    writer
        .write_all(b"{\"jsonrpc\":\"2.0\",\"method\":\"notifications/initialized\"}\n")
        .await
        .unwrap();
    drop(sender);
    writer
        .write_all(
            b"{\"jsonrpc\":\"2.0\",\"id\":2,\"method\":\"tools/call\",\"params\":{\"name\":\"check_for_updates\",\"arguments\":{}}}\n",
        )
        .await
        .unwrap();

    let response = read_json(&mut reader).await;
    assert_eq!(
        response["result"]["structuredContent"]["status"],
        "unavailable"
    );
    assert_eq!(
        response["result"]["structuredContent"]["current_version"],
        "0.3.0"
    );

    writer
        .write_all(
            b"{\"jsonrpc\":\"2.0\",\"id\":3,\"method\":\"tools/call\",\"params\":{\"name\":\"list_assets\",\"arguments\":{\"filter\":\"\"}}}\n",
        )
        .await
        .unwrap();
    let assets = read_json(&mut reader).await;
    assert_eq!(
        assets["result"]["structuredContent"],
        serde_json::json!({ "assets": [] })
    );

    finish(reader, writer, server).await;
}

#[tokio::test]
async fn update_notice_does_not_change_unrelated_tool_results() {
    let (mut reader, mut writer, server) =
        initialized_client(Some(receiver(update_available())), 1).await;
    let _initialize = read_json(&mut reader).await;
    writer
        .write_all(b"{\"jsonrpc\":\"2.0\",\"method\":\"notifications/initialized\"}\n")
        .await
        .unwrap();
    writer
        .write_all(
            b"{\"jsonrpc\":\"2.0\",\"id\":2,\"method\":\"tools/call\",\"params\":{\"name\":\"list_assets\",\"arguments\":{\"filter\":\"\"}}}\n",
        )
        .await
        .unwrap();

    let response = read_json(&mut reader).await;

    assert_eq!(
        response["result"]["structuredContent"],
        serde_json::json!({ "assets": [] })
    );
    assert_eq!(response["result"]["content"].as_array().unwrap().len(), 1);

    finish(reader, writer, server).await;
}

#[tokio::test]
async fn disabled_update_check_is_not_advertised() {
    let (mut reader, mut writer, server) = initialized_client(None, 0).await;
    let initialize = read_json(&mut reader).await;
    let instructions = initialize["result"]["instructions"].as_str().unwrap();
    assert!(!instructions.contains("check_for_updates"));

    writer
        .write_all(b"{\"jsonrpc\":\"2.0\",\"method\":\"notifications/initialized\"}\n")
        .await
        .unwrap();
    writer
        .write_all(b"{\"jsonrpc\":\"2.0\",\"id\":2,\"method\":\"tools/list\",\"params\":{}}\n")
        .await
        .unwrap();
    let tools = read_json(&mut reader).await;
    assert!(
        tools["result"]["tools"]
            .as_array()
            .unwrap()
            .iter()
            .all(|tool| tool["name"] != "check_for_updates")
    );

    finish(reader, writer, server).await;
}
