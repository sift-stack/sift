use rmcp::{handler::server::wrapper::Parameters, model::ErrorCode};
use serde_json::json;
use sift_rs::rules::v1::{ListRulesResponse, Rule, rule_service_server::RuleServiceServer};
use sift_test_util::{grpc::memory_sift_channel, mock::rules::v1::MockRuleServiceImpl};
use tokio::task::JoinHandle;
use tonic::{Response, transport::Server};

use super::{parse_rule_definition, rule_identifier};
use crate::{
    server::SiftMcpServer,
    tool::common::{ListParams, test_support::structured},
};

async fn server_with_mock(mock: MockRuleServiceImpl) -> (SiftMcpServer, JoinHandle<()>) {
    let (client, server) = tokio::io::duplex(1024);
    let channel = memory_sift_channel(client).await;

    let handle = tokio::spawn(async move {
        Server::builder()
            .add_service(RuleServiceServer::new(mock))
            .serve_with_incoming(tokio_stream::once(Ok::<_, std::io::Error>(server)))
            .await
            .unwrap();
    });

    (
        SiftMcpServer::new(
            channel,
            String::from("https://app.test.local"),
            false,
            false,
        ),
        handle,
    )
}

#[tokio::test]
async fn list_rules_count_only_returns_the_match_count() {
    let mut mock = MockRuleServiceImpl::new();
    mock.expect_list_rules().times(2).returning(|req| {
        let (count, next) = match req.get_ref().page_token.as_str() {
            "" => (200, "page-2"),
            _ => (50, ""),
        };
        Ok(Response::new(ListRulesResponse {
            rules: (0..count)
                .map(|i| Rule {
                    rule_id: format!("rule{i}"),
                    ..Default::default()
                })
                .collect(),
            next_page_token: next.to_string(),
        }))
    });

    let (server, _h) = server_with_mock(mock).await;
    let resp = server
        .list_rules(Parameters(ListParams {
            filter: "is_live_evaluation_enabled == true".into(),
            order_by: None,
            limit: Some(10),
            fields: None,
            count_only: Some(true),
        }))
        .await
        .expect("list_rules failed");

    assert_eq!(structured(resp), json!({ "count": 250, "has_more": false }));
}

#[test]
fn rule_identifier_accepts_rule_id_only() {
    let (rule_id, client_key) =
        rule_identifier(Some("rule-1".to_string()), None).expect("should accept rule_id");
    assert_eq!(rule_id, "rule-1");
    assert_eq!(client_key, "");
}

#[test]
fn rule_identifier_accepts_client_key_only() {
    let (rule_id, client_key) =
        rule_identifier(None, Some("ck-1".to_string())).expect("should accept client_key");
    assert_eq!(rule_id, "");
    assert_eq!(client_key, "ck-1");
}

#[test]
fn rule_identifier_rejects_both() {
    let err = rule_identifier(Some("rule-1".to_string()), Some("ck-1".to_string()))
        .expect_err("should reject both");
    assert_eq!(err.code, ErrorCode::INVALID_PARAMS);
}

#[test]
fn rule_identifier_rejects_neither() {
    let err = rule_identifier(None, None).expect_err("should reject neither");
    assert_eq!(err.code, ErrorCode::INVALID_PARAMS);
}

#[test]
fn parse_rule_definition_parses_valid_json() {
    let json = r#"{ "name": "overtemp", "description": "engine over temperature" }"#;
    let update = parse_rule_definition(json).expect("should parse");
    assert_eq!(update.name, "overtemp");
    assert_eq!(update.description, "engine over temperature");
}

#[test]
fn parse_rule_definition_rejects_invalid_json() {
    let err = parse_rule_definition("not json").expect_err("should reject");
    assert_eq!(err.code, ErrorCode::INVALID_PARAMS);
}
