use sift_rs::rules::v1::{
    ListRulesResponse, Rule, rule_service_server::RuleServiceServer,
};
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
