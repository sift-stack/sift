use sift_rs::rules::v1::{
    ArchiveRuleResponse, CreateRuleResponse, GetRuleResponse, ListRuleVersionsResponse,
    ListRulesResponse, Rule, RuleAssetConfiguration, RuleVersion, UnarchiveRuleResponse,
    UpdateRuleRequest, UpdateRuleResponse, rule_service_server::RuleServiceServer,
};

use super::RuleUpdate;
use sift_test_util::{grpc::memory_sift_channel, mock::rules::v1::MockRuleServiceImpl};
use tokio::task::JoinHandle;
use tonic::{Response, Status, transport::Server};

use super::RuleService;
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

    (
        RuleService::new(channel, crate::policy::RetryPolicy::default()),
        handle,
    )
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
        .create_rule(UpdateRuleRequest {
            name: "overtemp".into(),
            description: "engine over temperature".into(),
            ..Default::default()
        })
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

    let rule_id = service
        .update_rule(
            "rule-1".to_string(),
            RuleUpdate {
                name: Some("new name".to_string()),
                ..Default::default()
            },
        )
        .await
        .expect("update_rule failed");

    assert_eq!(rule_id, "rule-1");
}

#[tokio::test]
async fn update_rule_errors_when_rule_missing() {
    let mut mock = MockRuleServiceImpl::new();
    mock.expect_get_rule()
        .returning(|_| Ok(Response::new(GetRuleResponse { rule: None })));

    let (service, _h) = service_with_mock(mock).await;

    let err = service
        .update_rule(
            "missing".to_string(),
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
        .create_rule(UpdateRuleRequest::default())
        .await
        .expect_err("expected error");

    assert!(err.to_string().contains("failed to create rule"));
}
