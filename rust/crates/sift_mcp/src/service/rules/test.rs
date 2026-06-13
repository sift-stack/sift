use sift_rs::metadata::v1::MetadataValue;
use sift_rs::rules::v1::{
    ContextualChannels, CreateRuleResponse, GetRuleResponse, ListRulesResponse, Rule, RuleAction,
    RuleAssetConfiguration, RuleCondition, RuleConditionExpression, UpdateRuleResponse,
    rule_service_server::RuleServiceServer,
};
use sift_test_util::{grpc::memory_sift_channel, mock::rules::v1::MockRuleServiceImpl};
use tokio::task::JoinHandle;
use tonic::{Response, Status, transport::Server};

use super::{RuleService, RuleUpdate};
use crate::service::common::PAGE_SIZE;

async fn service_with_mock(mock: MockRuleServiceImpl) -> (RuleService, JoinHandle<()>) {
    let (client, server) = tokio::io::duplex(1024);
    let channel = memory_sift_channel(client).await;

    let handle = tokio::spawn(async move {
        Server::builder()
            .add_service(RuleServiceServer::new(mock))
            .serve_with_incoming(tokio_stream::once(Ok::<_, std::io::Error>(server)))
            .await
            .unwrap();
    });

    (RuleService::new(channel), handle)
}

#[tokio::test]
async fn list_rules_returns_single_page() {
    let mut mock = MockRuleServiceImpl::new();
    mock.expect_list_rules()
        .withf(|req| req.get_ref().filter == "name == \"overtemp\"")
        .returning(|_| {
            Ok(Response::new(ListRulesResponse {
                rules: vec![Rule {
                    rule_id: "rule1".into(),
                    name: "overtemp".into(),
                    ..Default::default()
                }],
                next_page_token: String::new(),
            }))
        });

    let (service, _h) = service_with_mock(mock).await;

    let rules = service
        .list_rules("name == \"overtemp\"".to_string(), None, None)
        .await
        .expect("list_rules failed");

    assert_eq!(rules.len(), 1);
    assert_eq!(rules[0].rule_id, "rule1");
}

#[tokio::test]
async fn list_rules_forwards_order_by() {
    let mut mock = MockRuleServiceImpl::new();
    mock.expect_list_rules()
        .withf(|req| req.get_ref().order_by == "created_date desc")
        .returning(|_| {
            Ok(Response::new(ListRulesResponse {
                rules: vec![Rule {
                    rule_id: "rule1".into(),
                    ..Default::default()
                }],
                next_page_token: String::new(),
            }))
        });

    let (service, _h) = service_with_mock(mock).await;

    let rules = service
        .list_rules(String::new(), Some("created_date desc".to_string()), None)
        .await
        .expect("list_rules failed");

    assert_eq!(rules.len(), 1);
}

#[tokio::test]
async fn list_rules_paginates_until_token_empty() {
    let mut mock = MockRuleServiceImpl::new();
    mock.expect_list_rules().returning(|req| {
        let req = req.into_inner();
        assert_eq!(req.page_size, PAGE_SIZE);
        let (rules, next) = match req.page_token.as_str() {
            "" => (
                vec![Rule {
                    rule_id: "rule1".into(),
                    ..Default::default()
                }],
                "page-2".to_string(),
            ),
            "page-2" => (
                vec![Rule {
                    rule_id: "rule2".into(),
                    ..Default::default()
                }],
                String::new(),
            ),
            other => return Err(Status::invalid_argument(format!("bad token: {other}"))),
        };
        Ok(Response::new(ListRulesResponse {
            rules,
            next_page_token: next,
        }))
    });

    let (service, _h) = service_with_mock(mock).await;

    let rules = service
        .list_rules(String::new(), None, None)
        .await
        .expect("list_rules failed");

    let ids: Vec<&str> = rules.iter().map(|r| r.rule_id.as_str()).collect();
    assert_eq!(ids, vec!["rule1", "rule2"]);
}

#[tokio::test]
async fn list_rules_respects_limit() {
    let mut mock = MockRuleServiceImpl::new();
    mock.expect_list_rules().times(1).returning(|req| {
        let req = req.into_inner();
        assert_eq!(req.page_size, 2);
        Ok(Response::new(ListRulesResponse {
            rules: vec![
                Rule {
                    rule_id: "rule1".into(),
                    ..Default::default()
                },
                Rule {
                    rule_id: "rule2".into(),
                    ..Default::default()
                },
            ],
            next_page_token: "page-2".into(),
        }))
    });

    let (service, _h) = service_with_mock(mock).await;

    let rules = service
        .list_rules(String::new(), None, Some(2))
        .await
        .expect("list_rules failed");

    assert_eq!(rules.len(), 2);
}

#[tokio::test]
async fn list_rules_truncates_to_limit_across_pages() {
    let mut mock = MockRuleServiceImpl::new();
    mock.expect_list_rules().returning(|req| {
        let req = req.into_inner();
        assert_eq!(req.page_size, 3);
        let (rules, next) = match req.page_token.as_str() {
            "" => (
                vec![
                    Rule {
                        rule_id: "rule1".into(),
                        ..Default::default()
                    },
                    Rule {
                        rule_id: "rule2".into(),
                        ..Default::default()
                    },
                ],
                "page-2".to_string(),
            ),
            "page-2" => (
                vec![
                    Rule {
                        rule_id: "rule3".into(),
                        ..Default::default()
                    },
                    Rule {
                        rule_id: "rule4".into(),
                        ..Default::default()
                    },
                ],
                String::new(),
            ),
            other => return Err(Status::invalid_argument(format!("bad token: {other}"))),
        };
        Ok(Response::new(ListRulesResponse {
            rules,
            next_page_token: next,
        }))
    });

    let (service, _h) = service_with_mock(mock).await;

    let rules = service
        .list_rules(String::new(), None, Some(3))
        .await
        .expect("list_rules failed");

    let ids: Vec<&str> = rules.iter().map(|r| r.rule_id.as_str()).collect();
    assert_eq!(ids, vec!["rule1", "rule2", "rule3"]);
}

#[tokio::test]
async fn list_rules_breaks_on_empty_page() {
    let mut mock = MockRuleServiceImpl::new();
    mock.expect_list_rules().times(1).returning(|_| {
        Ok(Response::new(ListRulesResponse {
            rules: vec![],
            next_page_token: "ignored".into(),
        }))
    });

    let (service, _h) = service_with_mock(mock).await;

    let rules = service
        .list_rules(String::new(), None, None)
        .await
        .expect("list_rules failed");

    assert!(rules.is_empty());
}

#[tokio::test]
async fn list_rules_propagates_grpc_error() {
    let mut mock = MockRuleServiceImpl::new();
    mock.expect_list_rules()
        .returning(|_| Err(Status::not_found("no such rule")));

    let (service, _h) = service_with_mock(mock).await;

    let err = service
        .list_rules(String::new(), None, None)
        .await
        .expect_err("expected error");

    assert!(err.to_string().contains("failed to query rules"));
}

#[tokio::test]
async fn get_rule_by_id_returns_rule() {
    let mut mock = MockRuleServiceImpl::new();
    mock.expect_get_rule()
        .withf(|req| req.get_ref().rule_id == "r1" && req.get_ref().client_key.is_empty())
        .returning(|_| {
            Ok(Response::new(GetRuleResponse {
                rule: Some(Rule {
                    rule_id: "r1".into(),
                    name: "overtemp".into(),
                    ..Default::default()
                }),
            }))
        });

    let (service, _h) = service_with_mock(mock).await;

    let rule = service
        .get_rule("r1".into(), String::new())
        .await
        .expect("get failed");

    assert_eq!(rule.name, "overtemp");
}

#[tokio::test]
async fn get_missing_rule_is_not_found() {
    let mut mock = MockRuleServiceImpl::new();
    mock.expect_get_rule()
        .returning(|_| Ok(Response::new(GetRuleResponse { rule: None })));

    let (service, _h) = service_with_mock(mock).await;

    let err = service
        .get_rule("missing".into(), String::new())
        .await
        .expect_err("expected not found");

    let status = err.downcast::<Status>().expect("expected tonic Status");
    assert_eq!(status.code(), tonic::Code::NotFound);
}

#[tokio::test]
async fn create_rule_then_fetches_full_rule() {
    let mut mock = MockRuleServiceImpl::new();
    mock.expect_create_rule()
        .withf(|req| req.get_ref().update.as_ref().unwrap().name == "overtemp")
        .returning(|_| {
            Ok(Response::new(CreateRuleResponse {
                rule_id: "r9".into(),
            }))
        });
    mock.expect_get_rule()
        .withf(|req| req.get_ref().rule_id == "r9")
        .returning(|_| {
            Ok(Response::new(GetRuleResponse {
                rule: Some(Rule {
                    rule_id: "r9".into(),
                    name: "overtemp".into(),
                    ..Default::default()
                }),
            }))
        });

    let (service, _h) = service_with_mock(mock).await;

    let rule = service
        .create_rule(sift_rs::rules::v1::UpdateRuleRequest {
            name: "overtemp".into(),
            description: "too hot".into(),
            ..Default::default()
        })
        .await
        .expect("create failed");

    assert_eq!(rule.rule_id, "r9");
}

#[tokio::test]
async fn update_rule_preserves_existing_conditions_when_not_provided() {
    let mut mock = MockRuleServiceImpl::new();
    mock.expect_get_rule().returning(|_| {
        Ok(Response::new(GetRuleResponse {
            rule: Some(Rule {
                rule_id: "r1".into(),
                name: "old".into(),
                description: "keep".into(),
                organization_id: "org1".into(),
                client_key: "ck1".into(),
                is_external: true,
                is_live_evaluation_enabled: true,
                metadata: vec![MetadataValue::default()],
                contextual_channels: Some(ContextualChannels::default()),
                conditions: vec![RuleCondition {
                    rule_condition_id: "rc1".into(),
                    expression: Some(RuleConditionExpression::default()),
                    actions: vec![RuleAction {
                        rule_action_id: "ra1".into(),
                        action_type: 1,
                        ..Default::default()
                    }],
                    ..Default::default()
                }],
                ..Default::default()
            }),
        }))
    });
    mock.expect_update_rule()
        .withf(|req| {
            let req = req.get_ref();
            // name overlaid; all other fields preserved from the fetched rule and the
            // existing condition (including its expression) converted to write shape.
            req.name == "renamed"
                && req.description == "keep"
                && req.organization_id == "org1"
                && req.client_key.as_deref() == Some("ck1")
                && req.is_external
                && req.is_live_evaluation_enabled == Some(true)
                && req.metadata.len() == 1
                && req.contextual_channels.is_some()
                && req.conditions.len() == 1
                && req.conditions[0].rule_condition_id.as_deref() == Some("rc1")
                && req.conditions[0].expression.is_some()
                && req.conditions[0].actions.len() == 1
                && req.conditions[0].actions[0].rule_action_id.as_deref() == Some("ra1")
                && req.conditions[0].actions[0].action_type == 1
        })
        .returning(|_| {
            Ok(Response::new(UpdateRuleResponse {
                rule_id: "r1".into(),
            }))
        });

    let (service, _h) = service_with_mock(mock).await;

    let rule = service
        .update_rule(
            "r1".into(),
            String::new(),
            RuleUpdate {
                name: Some("renamed".into()),
                ..Default::default()
            },
        )
        .await
        .expect("update failed");

    assert_eq!(rule.rule_id, "r1");
}

#[tokio::test]
async fn update_rule_overlays_asset_config_and_live_eval() {
    let mut mock = MockRuleServiceImpl::new();
    mock.expect_get_rule().returning(|_| {
        Ok(Response::new(GetRuleResponse {
            rule: Some(Rule {
                rule_id: "r1".into(),
                name: "keep".into(),
                is_live_evaluation_enabled: true,
                asset_configuration: Some(RuleAssetConfiguration {
                    asset_ids: vec!["old".into()],
                    tag_ids: vec![],
                }),
                ..Default::default()
            }),
        }))
    });
    mock.expect_update_rule()
        .withf(|req| {
            let req = req.get_ref();
            // provided asset config overrides; provided live-eval overrides.
            req.asset_configuration
                .as_ref()
                .map(|a| a.asset_ids.clone())
                == Some(vec!["new".to_string()])
                && req.is_live_evaluation_enabled == Some(false)
        })
        .returning(|_| {
            Ok(Response::new(UpdateRuleResponse {
                rule_id: "r1".into(),
            }))
        });

    let (service, _h) = service_with_mock(mock).await;

    service
        .update_rule(
            "r1".into(),
            String::new(),
            RuleUpdate {
                asset_configuration: Some(RuleAssetConfiguration {
                    asset_ids: vec!["new".into()],
                    tag_ids: vec![],
                }),
                is_live_evaluation_enabled: Some(false),
                ..Default::default()
            },
        )
        .await
        .expect("update failed");
}

#[tokio::test]
async fn update_rule_maps_empty_condition_ids_to_none() {
    let mut mock = MockRuleServiceImpl::new();
    mock.expect_get_rule().returning(|_| {
        Ok(Response::new(GetRuleResponse {
            rule: Some(Rule {
                rule_id: "r1".into(),
                name: "keep".into(),
                // A condition/action with no server-assigned id (e.g. never persisted).
                conditions: vec![RuleCondition {
                    rule_condition_id: String::new(),
                    actions: vec![RuleAction {
                        rule_action_id: String::new(),
                        ..Default::default()
                    }],
                    ..Default::default()
                }],
                ..Default::default()
            }),
        }))
    });
    mock.expect_update_rule()
        .withf(|req| {
            let c = &req.get_ref().conditions[0];
            // empty ids must become None, not Some("").
            c.rule_condition_id.is_none() && c.actions[0].rule_action_id.is_none()
        })
        .returning(|_| {
            Ok(Response::new(UpdateRuleResponse {
                rule_id: "r1".into(),
            }))
        });

    let (service, _h) = service_with_mock(mock).await;

    service
        .update_rule(
            "r1".into(),
            String::new(),
            RuleUpdate {
                name: Some("renamed".into()),
                ..Default::default()
            },
        )
        .await
        .expect("update failed");
}
