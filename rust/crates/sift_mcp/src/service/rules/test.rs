use sift_rs::rules::v1::{
    ActionKind, AnnotationActionConfiguration, ArchiveRuleResponse, CreateRuleResponse,
    GetRuleResponse, ListRuleVersionsResponse, ListRulesResponse, Rule, RuleAction,
    RuleActionConfiguration, RuleAssetConfiguration, RuleCondition, RuleVersion,
    UnarchiveRuleResponse, UpdateActionRequest, UpdateConditionRequest, UpdateRuleRequest,
    UpdateRuleResponse, WebhookActionConfiguration, rule_action_configuration::Configuration,
    rule_service_server::RuleServiceServer,
};

use super::RuleUpdate;
use sift_test_util::{grpc::memory_sift_channel, mock::rules::v1::MockRuleServiceImpl};
use tokio::task::JoinHandle;
use tonic::{Response, Status, transport::Server};

use super::RuleService;
use crate::service::common::DEFAULT_LIMIT;

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

    (
        RuleService::new(channel, crate::policy::RetryPolicy::default()),
        handle,
    )
}

/// Prepares and saves an update the way `update_rule` does, returning the
/// saved rule id and the webhooks the update detached.
async fn update_rule(
    service: &RuleService,
    rule_id: &str,
    changes: RuleUpdate,
) -> anyhow::Result<(String, Vec<String>)> {
    let pending = service.prepare_update(rule_id.to_string(), changes).await?;
    let detached = pending.detached_webhook_ids.clone();
    let rule_id = service.save_update(pending).await?;
    Ok((rule_id, detached))
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
        .expect("list_rules failed")
        .items;

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
        .expect("list_rules failed")
        .items;

    assert_eq!(rules.len(), 1);
}

#[tokio::test]
async fn list_rules_paginates_until_token_empty() {
    let mut mock = MockRuleServiceImpl::new();
    mock.expect_list_rules().returning(|req| {
        let req = req.into_inner();
        assert_eq!(req.page_size, DEFAULT_LIMIT);
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
        .expect("list_rules failed")
        .items;

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
        .expect("list_rules failed")
        .items;

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
        .expect("list_rules failed")
        .items;

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
        .expect("list_rules failed")
        .items;

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
async fn create_rule_forwards_definition_and_returns_id() {
    let mut mock = MockRuleServiceImpl::new();
    mock.expect_create_rule()
        .withf(|req| {
            req.get_ref()
                .update
                .as_ref()
                .is_some_and(|u| u.name == "overtemp")
        })
        .returning(|_| {
            Ok(Response::new(CreateRuleResponse {
                rule_id: "new-rule".into(),
            }))
        });

    let (service, _h) = service_with_mock(mock).await;

    let rule_id = service
        .create_rule(
            UpdateRuleRequest {
                name: "overtemp".into(),
                description: "engine over temperature".into(),
                ..Default::default()
            },
            Vec::new(),
        )
        .await
        .expect("create_rule failed");

    assert_eq!(rule_id, "new-rule");
}

#[tokio::test]
async fn update_rule_merges_changes_over_current_definition() {
    let mut mock = MockRuleServiceImpl::new();

    // The current rule has conditions and metadata we expect to be preserved.
    mock.expect_get_rule()
        .withf(|req| req.get_ref().rule_id == "rule-1")
        .returning(|_| {
            Ok(Response::new(GetRuleResponse {
                rule: Some(Rule {
                    rule_id: "rule-1".into(),
                    name: "old name".into(),
                    description: "old description".into(),
                    asset_configuration: Some(RuleAssetConfiguration {
                        asset_ids: vec!["asset-1".into()],
                        tag_ids: vec![],
                    }),
                    is_live_evaluation_enabled: true,
                    ..Default::default()
                }),
            }))
        });

    // Only `name` changes; everything else must be carried over from the fetched rule.
    mock.expect_update_rule()
        .withf(|req| {
            let u = req.get_ref();
            u.rule_id.as_deref() == Some("rule-1")
                && u.name == "new name"
                && u.description == "old description"
                && u.is_live_evaluation_enabled == Some(true)
                && u.asset_configuration
                    .as_ref()
                    .is_some_and(|ac| ac.asset_ids == vec!["asset-1".to_string()])
        })
        .returning(|_| {
            Ok(Response::new(UpdateRuleResponse {
                rule_id: "rule-1".into(),
            }))
        });

    let (service, _h) = service_with_mock(mock).await;

    let updated = update_rule(
        &service,
        "rule-1",
        RuleUpdate {
            name: Some("new name".to_string()),
            ..Default::default()
        },
    )
    .await
    .expect("update_rule failed");

    assert_eq!(updated.0, "rule-1");
    assert!(updated.1.is_empty());
}

#[tokio::test]
async fn update_rule_errors_when_rule_missing() {
    let mut mock = MockRuleServiceImpl::new();
    mock.expect_get_rule()
        .returning(|_| Ok(Response::new(GetRuleResponse { rule: None })));

    let (service, _h) = service_with_mock(mock).await;

    let err = update_rule(
        &service,
        "missing",
        RuleUpdate {
            name: Some("x".to_string()),
            ..Default::default()
        },
    )
    .await
    .expect_err("expected error");

    assert!(err.to_string().contains("not found"));
}

#[tokio::test]
async fn list_rule_versions_returns_single_page() {
    let mut mock = MockRuleServiceImpl::new();
    mock.expect_list_rule_versions()
        .withf(|req| req.get_ref().rule_id == "rule-1")
        .returning(|_| {
            Ok(Response::new(ListRuleVersionsResponse {
                rule_versions: vec![RuleVersion {
                    rule_id: "rule-1".into(),
                    rule_version_id: "v1".into(),
                    ..Default::default()
                }],
                next_page_token: String::new(),
            }))
        });

    let (service, _h) = service_with_mock(mock).await;

    let versions = service
        .list_rule_versions("rule-1".to_string(), String::new(), None)
        .await
        .expect("list_rule_versions failed");

    assert_eq!(versions.len(), 1);
    assert_eq!(versions[0].rule_version_id, "v1");
}

#[tokio::test]
async fn archive_rule_forwards_rule_id() {
    let mut mock = MockRuleServiceImpl::new();
    mock.expect_archive_rule()
        .withf(|req| req.get_ref().rule_id == "rule-1" && req.get_ref().client_key.is_empty())
        .returning(|_| Ok(Response::new(ArchiveRuleResponse {})));

    let (service, _h) = service_with_mock(mock).await;

    service
        .archive_rule("rule-1".to_string(), String::new())
        .await
        .expect("archive_rule failed");
}

#[tokio::test]
async fn unarchive_rule_forwards_client_key() {
    let mut mock = MockRuleServiceImpl::new();
    mock.expect_unarchive_rule()
        .withf(|req| req.get_ref().client_key == "ck-1" && req.get_ref().rule_id.is_empty())
        .returning(|_| Ok(Response::new(UnarchiveRuleResponse {})));

    let (service, _h) = service_with_mock(mock).await;

    service
        .unarchive_rule(String::new(), "ck-1".to_string())
        .await
        .expect("unarchive_rule failed");
}

#[tokio::test]
async fn create_rule_propagates_grpc_error() {
    let mut mock = MockRuleServiceImpl::new();
    mock.expect_create_rule()
        .returning(|_| Err(Status::invalid_argument("bad rule")));

    let (service, _h) = service_with_mock(mock).await;

    let err = service
        .create_rule(UpdateRuleRequest::default(), Vec::new())
        .await
        .expect_err("expected error");

    assert!(err.to_string().contains("failed to create rule"));
}

fn annotation_config() -> RuleActionConfiguration {
    RuleActionConfiguration {
        configuration: Some(Configuration::Annotation(
            AnnotationActionConfiguration::default(),
        )),
    }
}

fn webhook_config(webhook_id: &str) -> RuleActionConfiguration {
    RuleActionConfiguration {
        configuration: Some(Configuration::Webhook(WebhookActionConfiguration {
            webhook_id: webhook_id.into(),
        })),
    }
}

/// The webhook ids an action list calls, in order.
fn webhook_ids(actions: &[UpdateActionRequest]) -> Vec<&str> {
    actions
        .iter()
        .filter_map(
            |a| match a.configuration.as_ref()?.configuration.as_ref()? {
                Configuration::Webhook(w) => Some(w.webhook_id.as_str()),
                _ => None,
            },
        )
        .collect()
}

/// A rule with one condition holding an annotation action and a webhook
/// action for `wh-old`, each with its server-assigned id.
fn rule_with_webhook() -> Rule {
    Rule {
        rule_id: "rule-1".into(),
        name: "overtemp".into(),
        conditions: vec![RuleCondition {
            rule_condition_id: "cond-1".into(),
            actions: vec![
                RuleAction {
                    rule_action_id: "act-annotation".into(),
                    action_type: ActionKind::Annotation as i32,
                    configuration: Some(annotation_config()),
                    ..Default::default()
                },
                RuleAction {
                    rule_action_id: "act-wh-old".into(),
                    action_type: ActionKind::Webhook as i32,
                    configuration: Some(webhook_config("wh-old")),
                    ..Default::default()
                },
            ],
            ..Default::default()
        }],
        ..Default::default()
    }
}

#[tokio::test]
async fn create_rule_attaches_webhooks_beside_the_annotation() {
    let mut mock = MockRuleServiceImpl::new();
    mock.expect_create_rule()
        .withf(|req| {
            let update = req.get_ref().update.as_ref().unwrap();
            let actions = &update.conditions[0].actions;
            actions.len() == 3
                && actions[0].action_type == ActionKind::Annotation as i32
                && webhook_ids(actions) == vec!["wh-1", "wh-2"]
                && actions[1..].iter().all(|a| {
                    a.action_type == ActionKind::Webhook as i32 && a.rule_action_id.is_none()
                })
        })
        .returning(|_| {
            Ok(Response::new(CreateRuleResponse {
                rule_id: "new-rule".into(),
            }))
        });

    let (service, _h) = service_with_mock(mock).await;

    let rule_id = service
        .create_rule(
            UpdateRuleRequest {
                name: "overtemp".into(),
                conditions: vec![UpdateConditionRequest {
                    actions: vec![UpdateActionRequest {
                        action_type: ActionKind::Annotation as i32,
                        configuration: Some(annotation_config()),
                        ..Default::default()
                    }],
                    ..Default::default()
                }],
                ..Default::default()
            },
            // A repeated id must not attach twice; Sift rejects the rule if it does.
            vec!["wh-1".into(), "wh-2".into(), "wh-1".into()],
        )
        .await
        .expect("create_rule failed");

    assert_eq!(rule_id, "new-rule");
}

#[tokio::test]
async fn update_rule_keeps_existing_actions_when_webhooks_are_replaced() {
    let mut mock = MockRuleServiceImpl::new();
    mock.expect_get_rule().returning(|_| {
        Ok(Response::new(GetRuleResponse {
            rule: Some(rule_with_webhook()),
        }))
    });

    // Sift deletes any action left out of an update, so the existing annotation
    // and the kept webhook must be resent with their ids, and `wh-old` must not repeat.
    mock.expect_update_rule()
        .withf(|req| {
            let condition = &req.get_ref().conditions[0];
            let actions = &condition.actions;
            condition.rule_condition_id.as_deref() == Some("cond-1")
                && actions.len() == 3
                && actions[0].rule_action_id.as_deref() == Some("act-annotation")
                && actions[1].rule_action_id.as_deref() == Some("act-wh-old")
                && actions[2].rule_action_id.is_none()
                && webhook_ids(actions) == vec!["wh-old", "wh-new"]
        })
        .returning(|_| {
            Ok(Response::new(UpdateRuleResponse {
                rule_id: "rule-1".into(),
            }))
        });

    let (service, _h) = service_with_mock(mock).await;

    let updated = update_rule(
        &service,
        "rule-1",
        RuleUpdate {
            webhook_ids: Some(vec!["wh-old".into(), "wh-new".into(), "wh-new".into()]),
            ..Default::default()
        },
    )
    .await
    .expect("update_rule failed");

    assert!(updated.1.is_empty());
}

#[tokio::test]
async fn update_rule_reports_the_webhooks_it_detaches() {
    let mut mock = MockRuleServiceImpl::new();
    mock.expect_get_rule().returning(|_| {
        Ok(Response::new(GetRuleResponse {
            rule: Some(rule_with_webhook()),
        }))
    });

    mock.expect_update_rule()
        .withf(|req| {
            let actions = &req.get_ref().conditions[0].actions;
            actions.len() == 1
                && actions[0].rule_action_id.as_deref() == Some("act-annotation")
                && webhook_ids(actions).is_empty()
        })
        .returning(|_| {
            Ok(Response::new(UpdateRuleResponse {
                rule_id: "rule-1".into(),
            }))
        });

    let (service, _h) = service_with_mock(mock).await;

    let updated = update_rule(
        &service,
        "rule-1",
        RuleUpdate {
            webhook_ids: Some(Vec::new()),
            ..Default::default()
        },
    )
    .await
    .expect("update_rule failed");

    assert_eq!(updated.1, vec!["wh-old".to_string()]);
}

#[tokio::test]
async fn update_rule_carries_webhooks_onto_replaced_conditions() {
    let mut mock = MockRuleServiceImpl::new();
    mock.expect_get_rule().returning(|_| {
        Ok(Response::new(GetRuleResponse {
            rule: Some(rule_with_webhook()),
        }))
    });

    // The new conditions carry only an annotation. Webhooks are set through
    // `webhook_ids`, so rewriting the logic must not silently drop `wh-old`.
    mock.expect_update_rule()
        .withf(|req| {
            let actions = &req.get_ref().conditions[0].actions;
            actions.len() == 2
                && actions[0].action_type == ActionKind::Annotation as i32
                && webhook_ids(actions) == vec!["wh-old"]
        })
        .returning(|_| {
            Ok(Response::new(UpdateRuleResponse {
                rule_id: "rule-1".into(),
            }))
        });

    let (service, _h) = service_with_mock(mock).await;

    let updated = update_rule(
        &service,
        "rule-1",
        RuleUpdate {
            conditions: Some(vec![UpdateConditionRequest {
                actions: vec![UpdateActionRequest {
                    action_type: ActionKind::Annotation as i32,
                    configuration: Some(annotation_config()),
                    ..Default::default()
                }],
                ..Default::default()
            }]),
            ..Default::default()
        },
    )
    .await
    .expect("update_rule failed");

    assert!(updated.1.is_empty());
}

#[test]
fn has_webhook_action_finds_only_webhooks() {
    let annotation = UpdateConditionRequest {
        actions: vec![UpdateActionRequest {
            action_type: ActionKind::Annotation as i32,
            configuration: Some(annotation_config()),
            ..Default::default()
        }],
        ..Default::default()
    };
    let webhook = UpdateConditionRequest {
        actions: vec![UpdateActionRequest {
            action_type: ActionKind::Webhook as i32,
            configuration: Some(webhook_config("wh-1")),
            ..Default::default()
        }],
        ..Default::default()
    };

    assert!(!super::has_webhook_action(std::slice::from_ref(
        &annotation
    )));
    assert!(super::has_webhook_action(&[annotation, webhook]));
}

/// `rule_with_webhook` plus a webhook action that has a type but no
/// configuration, which Sift saves as a webhook action with no webhook.
fn rule_with_unconfigured_webhook() -> Rule {
    let mut rule = rule_with_webhook();
    rule.conditions[0].actions.push(RuleAction {
        rule_action_id: "act-wh-empty".into(),
        action_type: ActionKind::Webhook as i32,
        ..Default::default()
    });
    rule
}

#[tokio::test]
async fn update_rule_replacing_webhooks_drops_one_with_no_webhook() {
    let mut mock = MockRuleServiceImpl::new();
    mock.expect_get_rule().returning(|_| {
        Ok(Response::new(GetRuleResponse {
            rule: Some(rule_with_unconfigured_webhook()),
        }))
    });
    mock.expect_update_rule()
        .withf(|req| {
            let actions = &req.get_ref().conditions[0].actions;
            actions.len() == 2
                && actions
                    .iter()
                    .all(|a| a.rule_action_id.as_deref() != Some("act-wh-empty"))
                && webhook_ids(actions) == vec!["wh-old"]
        })
        .returning(|_| {
            Ok(Response::new(UpdateRuleResponse {
                rule_id: "rule-1".into(),
            }))
        });

    let (service, _h) = service_with_mock(mock).await;

    update_rule(
        &service,
        "rule-1",
        RuleUpdate {
            webhook_ids: Some(vec!["wh-old".into()]),
            ..Default::default()
        },
    )
    .await
    .expect("update_rule failed");
}

#[tokio::test]
async fn update_rule_without_webhook_ids_leaves_actions_alone() {
    let mut mock = MockRuleServiceImpl::new();
    mock.expect_get_rule().returning(|_| {
        Ok(Response::new(GetRuleResponse {
            rule: Some(rule_with_unconfigured_webhook()),
        }))
    });
    mock.expect_update_rule()
        .withf(|req| {
            let u = req.get_ref();
            u.name == "renamed" && u.conditions[0].actions.len() == 3
        })
        .returning(|_| {
            Ok(Response::new(UpdateRuleResponse {
                rule_id: "rule-1".into(),
            }))
        });

    let (service, _h) = service_with_mock(mock).await;

    update_rule(
        &service,
        "rule-1",
        RuleUpdate {
            name: Some("renamed".into()),
            ..Default::default()
        },
    )
    .await
    .expect("update_rule failed");
}

#[tokio::test]
async fn prepare_update_reports_current_webhooks_without_writing() {
    let mut mock = MockRuleServiceImpl::new();
    mock.expect_get_rule().returning(|_| {
        Ok(Response::new(GetRuleResponse {
            rule: Some(rule_with_webhook()),
        }))
    });
    mock.expect_update_rule().times(0);

    let (service, _h) = service_with_mock(mock).await;

    let pending = service
        .prepare_update(
            "rule-1".to_string(),
            RuleUpdate {
                webhook_ids: Some(vec!["wh-new".into()]),
                ..Default::default()
            },
        )
        .await
        .expect("prepare_update failed");

    assert_eq!(pending.current_webhook_ids, vec!["wh-old".to_string()]);
    assert_eq!(pending.detached_webhook_ids, vec!["wh-old".to_string()]);
}

#[test]
fn has_webhook_action_counts_a_type_with_no_configuration() {
    let typed_only = UpdateConditionRequest {
        actions: vec![UpdateActionRequest {
            action_type: ActionKind::Webhook as i32,
            ..Default::default()
        }],
        ..Default::default()
    };

    assert!(super::has_webhook_action(&[typed_only]));
}
