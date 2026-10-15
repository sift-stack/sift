use rmcp::{handler::server::wrapper::Parameters, model::ErrorCode};
use sift_rs::{
    rules::v1::{
        ActionKind, CreateRuleResponse, GetRuleResponse, Rule, RuleAction, RuleActionConfiguration,
        RuleCondition, UpdateRuleResponse, WebhookActionConfiguration,
        rule_action_configuration::Configuration, rule_service_server::RuleServiceServer,
    },
    webhooks::v1::{ListWebhooksResponse, Webhook, webhook_service_server::WebhookServiceServer},
};
use sift_test_util::{
    grpc::memory_sift_channel,
    mock::{rules::v1::MockRuleServiceImpl, webhooks::v1::MockWebhookServiceImpl},
};
use tokio::task::JoinHandle;
use tonic::{Response, transport::Server};

use super::{
    RuleDefinitionParams, UpdateRuleParams, check_webhooks, parse_rule_definition, rule_identifier,
};
use crate::{server::SiftMcpServer, tool::common::test_support::structured};

/// A rule definition in the shape `create_rule` documents, with one annotation action.
const RULE_JSON: &str = r#"{
    "name": "overtemp",
    "description": "engine over temperature",
    "conditions": [{
        "expression": {},
        "actions": [{
            "action_type": "ANNOTATION",
            "configuration": {
                "annotation": { "tag_ids": [], "annotation_type": "ANNOTATION_TYPE_DATA_REVIEW" }
            }
        }]
    }],
    "metadata": []
}"#;

async fn server_with_mocks(
    rule_mock: MockRuleServiceImpl,
    webhook_mock: MockWebhookServiceImpl,
) -> (SiftMcpServer, JoinHandle<()>) {
    let (client, server) = tokio::io::duplex(1024);
    let channel = memory_sift_channel(client).await;

    let handle = tokio::spawn(async move {
        Server::builder()
            .add_service(RuleServiceServer::new(rule_mock))
            .add_service(WebhookServiceServer::new(webhook_mock))
            .serve_with_incoming(tokio_stream::once(Ok::<_, std::io::Error>(server)))
            .await
            .unwrap();
    });

    (
        SiftMcpServer::new(channel, String::from("https://app.test.local"), true, true),
        handle,
    )
}

fn webhook(id: &str, name: &str) -> Webhook {
    Webhook {
        webhook_id: id.into(),
        name: name.into(),
        ..Default::default()
    }
}

fn webhook_mock_returning(webhooks: Vec<Webhook>) -> MockWebhookServiceImpl {
    let mut mock = MockWebhookServiceImpl::new();
    mock.expect_list_webhooks().returning(move |_| {
        Ok(Response::new(ListWebhooksResponse {
            webhooks: webhooks.clone(),
            next_page_token: String::new(),
        }))
    });
    mock
}

fn create_params(webhook_ids: Option<Vec<String>>) -> Parameters<RuleDefinitionParams> {
    Parameters(RuleDefinitionParams {
        rule_json: RULE_JSON.to_string(),
        webhook_ids,
    })
}

fn update_params(webhook_ids: Option<Vec<String>>) -> Parameters<UpdateRuleParams> {
    Parameters(UpdateRuleParams {
        rule_id: "rule-1".to_string(),
        name: None,
        description: None,
        version_notes: None,
        is_live_evaluation_enabled: None,
        asset_ids: None,
        tag_ids: None,
        conditions_json: None,
        webhook_ids,
    })
}

/// A webhook action as `create_rule` would refuse to find it in `rule_json`.
const WEBHOOK_ACTION_JSON: &str = r#"{
    "action_type": "WEBHOOK",
    "configuration": { "webhook": { "webhook_id": "wh1" } }
}"#;

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

#[test]
fn parse_rule_definition_accepts_the_documented_annotation_action() {
    let update = parse_rule_definition(RULE_JSON).expect("should parse");
    let action = &update.conditions[0].actions[0];
    assert_eq!(action.action_type, ActionKind::Annotation as i32);
    assert!(matches!(
        action.configuration.as_ref().unwrap().configuration,
        Some(Configuration::Annotation(_))
    ));
}

#[test]
fn check_webhooks_accepts_live_webhooks() {
    let webhooks = check_webhooks(&["wh1".to_string()], vec![webhook("wh1", "alerts")])
        .expect("should accept");
    assert_eq!(webhooks[0].name, "alerts");
}

#[test]
fn check_webhooks_rejects_an_unknown_id() {
    let err = check_webhooks(
        &["wh1".to_string(), "wh-missing".to_string()],
        vec![webhook("wh1", "alerts")],
    )
    .expect_err("should reject");
    assert_eq!(err.code, ErrorCode::RESOURCE_NOT_FOUND);
    assert!(err.message.contains("wh-missing"));
}

#[test]
fn check_webhooks_rejects_an_archived_webhook() {
    let archived = Webhook {
        is_archived: true,
        ..webhook("wh1", "old alerts")
    };
    let err = check_webhooks(&["wh1".to_string()], vec![archived]).expect_err("should reject");
    assert_eq!(err.code, ErrorCode::INVALID_PARAMS);
    assert!(err.message.contains("old alerts"));
}

#[tokio::test]
async fn create_rule_attaches_webhooks_and_names_them() {
    let mut rule_mock = MockRuleServiceImpl::new();
    rule_mock
        .expect_create_rule()
        .withf(|req| {
            let actions = &req.get_ref().update.as_ref().unwrap().conditions[0].actions;
            actions.len() == 2 && actions[1].action_type == ActionKind::Webhook as i32
        })
        .returning(|_| {
            Ok(Response::new(CreateRuleResponse {
                rule_id: "new-rule".into(),
            }))
        });

    let (server, _h) = server_with_mocks(
        rule_mock,
        webhook_mock_returning(vec![webhook("wh1", "slack alerts")]),
    )
    .await;

    let result = server
        .create_rule(create_params(Some(vec!["wh1".to_string()])))
        .await
        .expect("create_rule failed");

    let next_step = structured(result)["next_step"]
        .as_str()
        .unwrap()
        .to_string();
    assert!(next_step.contains("`slack alerts`"), "{next_step}");
}

#[tokio::test]
async fn create_rule_refuses_an_archived_webhook_before_writing() {
    let mut rule_mock = MockRuleServiceImpl::new();
    rule_mock.expect_create_rule().times(0);

    let archived = Webhook {
        is_archived: true,
        ..webhook("wh1", "old alerts")
    };
    let (server, _h) = server_with_mocks(rule_mock, webhook_mock_returning(vec![archived])).await;

    let err = server
        .create_rule(create_params(Some(vec!["wh1".to_string()])))
        .await
        .expect_err("should refuse");

    assert_eq!(err.code, ErrorCode::INVALID_PARAMS);
}

#[tokio::test]
async fn create_rule_without_webhooks_skips_the_lookup() {
    let mut rule_mock = MockRuleServiceImpl::new();
    rule_mock.expect_create_rule().times(1).returning(|_| {
        Ok(Response::new(CreateRuleResponse {
            rule_id: "new-rule".into(),
        }))
    });
    let mut webhook_mock = MockWebhookServiceImpl::new();
    webhook_mock.expect_list_webhooks().times(0);

    let (server, _h) = server_with_mocks(rule_mock, webhook_mock).await;

    server
        .create_rule(create_params(None))
        .await
        .expect("create_rule failed");
}

#[tokio::test]
async fn create_rule_refuses_a_webhook_action_in_rule_json() {
    let (server, _h) =
        server_with_mocks(MockRuleServiceImpl::new(), MockWebhookServiceImpl::new()).await;

    let rule_json = RULE_JSON.replacen(
        r#""actions": ["#,
        &format!(r#""actions": [{WEBHOOK_ACTION_JSON}, "#),
        1,
    );
    let err = server
        .create_rule(Parameters(RuleDefinitionParams {
            rule_json,
            webhook_ids: None,
        }))
        .await
        .expect_err("should refuse");

    assert_eq!(err.code, ErrorCode::INVALID_PARAMS);
    assert!(err.message.contains("webhook_ids"), "{}", err.message);
}

#[tokio::test]
async fn update_rule_refuses_a_webhook_action_in_conditions_json() {
    let (server, _h) =
        server_with_mocks(MockRuleServiceImpl::new(), MockWebhookServiceImpl::new()).await;

    let mut params = update_params(None);
    params.0.conditions_json = Some(format!(
        r#"[{{ "expression": {{}}, "actions": [{WEBHOOK_ACTION_JSON}] }}]"#
    ));
    let err = server.update_rule(params).await.expect_err("should refuse");

    assert_eq!(err.code, ErrorCode::INVALID_PARAMS);
}

#[tokio::test]
async fn update_rule_replaces_webhooks_and_names_the_detached_one() {
    let mut rule_mock = MockRuleServiceImpl::new();
    rule_mock.expect_get_rule().returning(|_| {
        Ok(Response::new(GetRuleResponse {
            rule: Some(Rule {
                rule_id: "rule-1".into(),
                conditions: vec![RuleCondition {
                    actions: vec![RuleAction {
                        rule_action_id: "act-old".into(),
                        action_type: ActionKind::Webhook as i32,
                        configuration: Some(RuleActionConfiguration {
                            configuration: Some(Configuration::Webhook(
                                WebhookActionConfiguration {
                                    webhook_id: "wh-old".into(),
                                },
                            )),
                        }),
                        ..Default::default()
                    }],
                    ..Default::default()
                }],
                ..Default::default()
            }),
        }))
    });
    rule_mock
        .expect_update_rule()
        .withf(|req| {
            let actions = &req.get_ref().conditions[0].actions;
            actions.len() == 1
                && matches!(
                    actions[0].configuration.as_ref().unwrap().configuration.as_ref(),
                    Some(Configuration::Webhook(w)) if w.webhook_id == "wh-new"
                )
        })
        .returning(|_| {
            Ok(Response::new(UpdateRuleResponse {
                rule_id: "rule-1".into(),
            }))
        });

    // One lookup validates the new set; a second names the webhook detached.
    let mut webhook_mock = MockWebhookServiceImpl::new();
    webhook_mock
        .expect_list_webhooks()
        .times(2)
        .returning(|req| {
            let filter = &req.get_ref().filter;
            let webhooks = if filter.contains("wh-new") {
                vec![webhook("wh-new", "new alerts")]
            } else {
                vec![webhook("wh-old", "old alerts")]
            };
            Ok(Response::new(ListWebhooksResponse {
                webhooks,
                next_page_token: String::new(),
            }))
        });

    let (server, _h) = server_with_mocks(rule_mock, webhook_mock).await;

    let result = server
        .update_rule(update_params(Some(vec!["wh-new".to_string()])))
        .await
        .expect("update_rule failed");

    let next_step = structured(result)["next_step"]
        .as_str()
        .unwrap()
        .to_string();
    assert!(
        next_step.contains("now calls webhook `new alerts`"),
        "{next_step}"
    );
    assert!(
        next_step.contains("no longer calls `old alerts`"),
        "{next_step}"
    );
}

#[tokio::test]
async fn update_rule_refuses_an_archived_webhook_before_writing() {
    let mut rule_mock = MockRuleServiceImpl::new();
    rule_mock.expect_get_rule().times(0);
    rule_mock.expect_update_rule().times(0);

    let archived = Webhook {
        is_archived: true,
        ..webhook("wh1", "old alerts")
    };
    let (server, _h) = server_with_mocks(rule_mock, webhook_mock_returning(vec![archived])).await;

    let err = server
        .update_rule(update_params(Some(vec!["wh1".to_string()])))
        .await
        .expect_err("should refuse");

    assert_eq!(err.code, ErrorCode::INVALID_PARAMS);
}

#[tokio::test]
async fn create_rule_refuses_a_rule_without_exactly_one_condition() {
    let (server, _h) =
        server_with_mocks(MockRuleServiceImpl::new(), MockWebhookServiceImpl::new()).await;

    let err = server
        .create_rule(Parameters(RuleDefinitionParams {
            rule_json: r#"{ "name": "overtemp", "description": "d", "metadata": [] }"#.to_string(),
            webhook_ids: None,
        }))
        .await
        .expect_err("should refuse");

    assert_eq!(err.code, ErrorCode::INVALID_PARAMS);
    assert!(err.message.contains("exactly one"), "{}", err.message);
}

#[tokio::test]
async fn update_rule_refuses_empty_conditions_json() {
    let (server, _h) =
        server_with_mocks(MockRuleServiceImpl::new(), MockWebhookServiceImpl::new()).await;

    let mut params = update_params(None);
    params.0.conditions_json = Some("[]".to_string());
    let err = server.update_rule(params).await.expect_err("should refuse");

    assert_eq!(err.code, ErrorCode::INVALID_PARAMS);
}
