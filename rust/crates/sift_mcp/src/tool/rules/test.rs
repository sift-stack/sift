use rmcp::model::ErrorCode;

use super::parse_rule_definition;

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

mod bulk {
    use std::sync::{Arc, Mutex};

    use crate::test_util::{grpc::memory_grpc_channel, mock::rules::v1::MockRuleServiceImpl};
    use rmcp::{handler::server::wrapper::Parameters, model::ErrorCode};
    use serde_json::{Value, json};
    use sift_rs::rules::v1::{
        ArchiveRuleResponse, GetRuleResponse, Rule, UpdateRuleResponse,
        rule_service_server::RuleServiceServer,
    };
    use tonic::{Response, Status, transport::Server};

    use crate::{server::SiftMcpServer, tool::common::test_support::structured};

    async fn server(mock: MockRuleServiceImpl) -> (SiftMcpServer, tokio::task::JoinHandle<()>) {
        let (client, incoming) = tokio::io::duplex(1024);
        let channel = memory_grpc_channel(client).await;
        let handle = tokio::spawn(async move {
            Server::builder()
                .add_service(RuleServiceServer::new(mock))
                .serve_with_incoming(tokio_stream::once(Ok::<_, std::io::Error>(incoming)))
                .await
                .unwrap();
        });
        let server = SiftMcpServer::new(channel, "https://app.test.local".into(), false, true)
            .with_require_selection(true);
        (server, handle)
    }

    fn params<T: serde::de::DeserializeOwned>(value: Value) -> Parameters<T> {
        Parameters(serde_json::from_value(value).unwrap())
    }

    fn found(rule_id: &str) -> Result<Response<GetRuleResponse>, Status> {
        Ok(Response::new(GetRuleResponse {
            rule: Some(Rule {
                rule_id: rule_id.into(),
                name: format!("name-{rule_id}"),
                ..Default::default()
            }),
        }))
    }

    #[tokio::test]
    async fn archiving_ten_rules_reports_each_item_and_skips_rejected_ones() {
        let archived = Arc::new(Mutex::new(Vec::new()));
        let mut mock = MockRuleServiceImpl::new();
        mock.expect_get_rule().returning(|request| {
            let rule_id = request.get_ref().rule_id.clone();
            if rule_id == "rule-5" {
                Err(Status::not_found("rule not found"))
            } else {
                found(&rule_id)
            }
        });
        let seen = archived.clone();
        mock.expect_archive_rule().returning(move |request| {
            seen.lock().unwrap().push(request.get_ref().rule_id.clone());
            Ok(Response::new(ArchiveRuleResponse {}))
        });
        let (server, handle) = server(mock).await;
        let items = (0..10)
            .map(|i| json!({"rule_id": format!("rule-{i}")}))
            .collect::<Vec<_>>();
        let selected = (0..10).filter(|&i| i != 3).collect::<Vec<_>>();
        let result = server
            .archive_rule(params(
                json!({"items": items, "selected_indices": selected}),
            ))
            .await
            .unwrap();
        assert_eq!(result.is_error, Some(true));
        let body = structured(result);
        assert_eq!(body["archived_count"], 8);
        assert_eq!(body["not_selected"], json!([3]));
        assert_eq!(body["not_attempted"], json!([]));
        assert_eq!(body["failures"][0]["index"], 5);
        assert_eq!(
            body["failures"][0]["error"]["code"],
            ErrorCode::RESOURCE_NOT_FOUND.0
        );
        assert_eq!(body["successes"][0]["result"]["name"], "name-rule-0");
        assert_eq!(body["successes"][0]["result"]["archived"], true);
        let archived = archived.lock().unwrap().clone();
        assert_eq!(archived.len(), 8);
        assert!(!archived.contains(&"rule-3".to_string()));
        assert!(!archived.contains(&"rule-5".to_string()));
        handle.abort();
    }

    #[tokio::test]
    async fn a_client_key_resolves_to_its_rule_id_before_unarchiving() {
        let mut mock = MockRuleServiceImpl::new();
        mock.expect_get_rule()
            .withf(|request| request.get_ref().client_key == "ck-1")
            .returning(|_| found("rule-1"));
        mock.expect_unarchive_rule()
            .withf(|request| {
                request.get_ref().rule_id == "rule-1" && request.get_ref().client_key.is_empty()
            })
            .times(1)
            .returning(|_| Ok(Response::new(Default::default())));
        let (server, handle) = server(mock).await;
        let body = structured(
            server
                .unarchive_rule(params(
                    json!({"items": [{"client_key": "ck-1"}], "selected_indices": [0]}),
                ))
                .await
                .unwrap(),
        );
        assert_eq!(body["unarchived_count"], 1);
        assert_eq!(
            body["successes"][0]["result"]["rule_url"],
            "https://app.test.local/rules/rule-1"
        );
        handle.abort();
    }

    #[tokio::test]
    async fn invalid_or_duplicate_items_fail_before_any_rpc() {
        let (server, handle) = server(MockRuleServiceImpl::new()).await;
        for (items, needle) in [
            (json!([{"rule_id": "a"}, {"rule_id": "a"}]), "item 1"),
            (
                json!([{"rule_id": "a"}, {"client_key": "k", "rule_id": "b"}]),
                "item 1",
            ),
            (json!([{}]), "item 0"),
            (
                json!([{"rule_id": "a"}, {"rule_id": " "}]),
                "item 1: `rule_id` must not be empty",
            ),
            (
                json!([{"rule_id": "a"}, {"client_key": ""}]),
                "item 1: `client_key` must not be empty",
            ),
            (
                json!([{"rule_id": "a", "typo": true}]),
                "unknown field `typo`",
            ),
        ] {
            let error = server
                .archive_rule(params(json!({"items": items, "selected_indices": [0]})))
                .await
                .unwrap_err();
            assert_eq!(error.code, ErrorCode::INVALID_PARAMS);
            assert!(error.message.contains(needle), "{}", error.message);
        }
        for (items, needle) in [
            (
                json!([{"rule_id": "a", "name": "x"}, {"rule_id": "a", "name": "y"}]),
                "item 1",
            ),
            (
                json!([{"rule_id": "a", "name": "x"}, {"rule_id": "b"}]),
                "item 1",
            ),
            (json!([{"rule_id": " ", "name": "x"}]), "item 0"),
            (
                json!([{"rule_id": "a", "conditions_json": "nope"}]),
                "item 0",
            ),
        ] {
            let error = server
                .update_rule(params(json!({"items": items, "selected_indices": [0]})))
                .await
                .unwrap_err();
            assert_eq!(error.code, ErrorCode::INVALID_PARAMS);
            assert!(error.message.contains(needle), "{}", error.message);
        }
        handle.abort();
    }

    #[tokio::test]
    async fn update_rule_runs_one_get_and_update_per_selected_item() {
        let mut mock = MockRuleServiceImpl::new();
        mock.expect_get_rule()
            .times(2)
            .returning(|request| found(&request.get_ref().rule_id));
        mock.expect_update_rule()
            .times(2)
            .withf(|request| request.get_ref().name == "renamed")
            .returning(|request| {
                Ok(Response::new(UpdateRuleResponse {
                    rule_id: request.get_ref().rule_id.clone().unwrap_or_default(),
                }))
            });
        let (server, handle) = server(mock).await;
        let items = ["a", "b", "c"].map(|id| json!({"rule_id": id, "name": "renamed"}));
        let body = structured(
            server
                .update_rule(params(json!({"items": items, "selected_indices": [0, 2]})))
                .await
                .unwrap(),
        );
        assert_eq!(body["updated_count"], 2);
        assert_eq!(body["successes"][1]["index"], 2);
        assert_eq!(body["successes"][1]["result"]["rule_id"], "c");
        assert_eq!(body["not_selected"], json!([1]));
        handle.abort();
    }
}

mod webhooks {
    use crate::test_util::{
        grpc::memory_grpc_channel,
        mock::{rules::v1::MockRuleServiceImpl, webhooks::v1::MockWebhookServiceImpl},
    };
    use rmcp::{handler::server::wrapper::Parameters, model::ErrorCode};
    use serde_json::{Value, json};
    use sift_rs::{
        rules::v1::{
            ActionKind, CreateRuleResponse, GetRuleResponse, Rule, RuleAction,
            RuleActionConfiguration, RuleCondition, UpdateRuleResponse, WebhookActionConfiguration,
            rule_action_configuration::Configuration, rule_service_server::RuleServiceServer,
        },
        webhooks::v1::{
            ListWebhooksResponse, Webhook, webhook_service_server::WebhookServiceServer,
        },
    };
    use tokio::task::JoinHandle;
    use tonic::{Response, Status, transport::Server};

    use super::super::{check_webhooks, parse_rule_definition};
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

    /// A webhook action as `create_rule` would refuse to find it in `rule_json`.
    const WEBHOOK_ACTION_JSON: &str = r#"{
        "action_type": "WEBHOOK",
        "configuration": { "webhook": { "webhook_id": "wh1" } }
    }"#;

    async fn server_with_mocks(
        rule_mock: MockRuleServiceImpl,
        webhook_mock: MockWebhookServiceImpl,
    ) -> (SiftMcpServer, JoinHandle<()>) {
        let (client, server) = tokio::io::duplex(1024);
        let channel = memory_grpc_channel(client).await;

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

    fn params<T: serde::de::DeserializeOwned>(items: Value) -> Parameters<T> {
        Parameters(serde_json::from_value(json!({ "items": items })).unwrap())
    }

    fn webhook(id: &str, name: &str) -> Webhook {
        Webhook {
            webhook_id: id.into(),
            name: name.into(),
            ..Default::default()
        }
    }

    fn archived(id: &str, name: &str) -> Webhook {
        Webhook {
            is_archived: true,
            ..webhook(id, name)
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

    /// `rule-1`, with one condition that calls `webhook_id`.
    fn rule_calling(webhook_id: &str) -> Result<Response<GetRuleResponse>, Status> {
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
                                    webhook_id: webhook_id.into(),
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
    }

    fn updated(
        _: tonic::Request<sift_rs::rules::v1::UpdateRuleRequest>,
    ) -> Result<Response<UpdateRuleResponse>, Status> {
        Ok(Response::new(UpdateRuleResponse {
            rule_id: "rule-1".into(),
        }))
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
        let webhooks = check_webhooks(&["wh1".to_string()], vec![webhook("wh1", "alerts")], &[])
            .expect("should accept");
        assert_eq!(webhooks[0].name, "alerts");
    }

    #[test]
    fn check_webhooks_rejects_an_unknown_id() {
        let err = check_webhooks(
            &["wh1".to_string(), "wh-missing".to_string()],
            vec![webhook("wh1", "alerts")],
            &[],
        )
        .expect_err("should reject");
        assert_eq!(err.code, ErrorCode::RESOURCE_NOT_FOUND);
        assert!(err.message.contains("wh-missing"));
    }

    #[test]
    fn check_webhooks_rejects_an_archived_webhook() {
        let err = check_webhooks(
            &["wh1".to_string()],
            vec![archived("wh1", "old alerts")],
            &[],
        )
        .expect_err("should reject");
        assert_eq!(err.code, ErrorCode::INVALID_PARAMS);
        assert!(err.message.contains("old alerts"));
    }

    #[test]
    fn check_webhooks_keeps_an_archived_webhook_already_attached() {
        let webhooks = check_webhooks(
            &["wh1".to_string()],
            vec![archived("wh1", "old alerts")],
            &["wh1".to_string()],
        )
        .expect("an archived webhook the rule already calls may stay");
        assert_eq!(webhooks.len(), 1);
    }

    #[tokio::test]
    async fn create_rule_attaches_webhooks_and_lists_them() {
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

        let (server, handle) = server_with_mocks(
            rule_mock,
            webhook_mock_returning(vec![webhook("wh1", "slack alerts")]),
        )
        .await;

        let body = structured(
            server
                .create_rule(params(
                    json!([{ "rule_json": RULE_JSON, "webhook_ids": ["wh1"] }]),
                ))
                .await
                .expect("create_rule failed"),
        );

        let result = &body["successes"][0]["result"];
        assert_eq!(
            result["webhooks"],
            json!([{ "webhook_id": "wh1", "name": "slack alerts" }])
        );
        assert!(result.get("warning").is_none(), "{result}");
        handle.abort();
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

        let (server, handle) = server_with_mocks(rule_mock, webhook_mock).await;

        let body = structured(
            server
                .create_rule(params(json!([{ "rule_json": RULE_JSON }])))
                .await
                .expect("create_rule failed"),
        );

        assert!(body["successes"][0]["result"].get("webhooks").is_none());
        handle.abort();
    }

    #[tokio::test]
    async fn create_rule_fails_only_the_item_whose_webhook_is_missing_or_archived() {
        let mut rule_mock = MockRuleServiceImpl::new();
        rule_mock.expect_create_rule().times(1).returning(|_| {
            Ok(Response::new(CreateRuleResponse {
                rule_id: "new-rule".into(),
            }))
        });
        let mut webhook_mock = MockWebhookServiceImpl::new();
        webhook_mock.expect_list_webhooks().returning(|req| {
            let filter = &req.get_ref().filter;
            let webhooks = if filter.contains("wh-old") {
                vec![archived("wh-old", "old alerts")]
            } else if filter.contains("wh-live") {
                vec![webhook("wh-live", "alerts")]
            } else {
                Vec::new()
            };
            Ok(Response::new(ListWebhooksResponse {
                webhooks,
                next_page_token: String::new(),
            }))
        });

        let (server, handle) = server_with_mocks(rule_mock, webhook_mock).await;

        let result = server
            .create_rule(params(json!([
                { "rule_json": RULE_JSON, "webhook_ids": ["wh-missing"] },
                { "rule_json": RULE_JSON, "webhook_ids": ["wh-old"] },
                { "rule_json": RULE_JSON, "webhook_ids": ["wh-live"] },
            ])))
            .await
            .expect("create_rule failed");

        assert_eq!(result.is_error, Some(true));
        let body = structured(result);
        assert_eq!(body["created_count"], 1);
        assert_eq!(body["successes"][0]["index"], 2);
        assert_eq!(body["not_attempted"], json!([]));
        let failures = &body["failures"];
        assert_eq!(failures[0]["index"], 0);
        assert_eq!(
            failures[0]["error"]["code"],
            ErrorCode::RESOURCE_NOT_FOUND.0
        );
        assert_eq!(failures[1]["index"], 1);
        assert_eq!(failures[1]["error"]["code"], ErrorCode::INVALID_PARAMS.0);
        assert_eq!(failures[1]["outcome_unknown"], false);
        handle.abort();
    }

    #[tokio::test]
    async fn create_rule_refuses_bad_conditions_before_any_rpc() {
        let (server, handle) =
            server_with_mocks(MockRuleServiceImpl::new(), MockWebhookServiceImpl::new()).await;

        let with_action = |action: &str| {
            RULE_JSON.replacen(r#""actions": ["#, &format!(r#""actions": [{action}, "#), 1)
        };
        for (rule_json, needle) in [
            (with_action(WEBHOOK_ACTION_JSON), "webhook_ids"),
            (
                with_action(r#"{ "action_type": "WEBHOOK" }"#),
                "webhook_ids",
            ),
            (
                r#"{ "name": "overtemp", "description": "d", "metadata": [] }"#.to_string(),
                "exactly one",
            ),
        ] {
            let err = server
                .create_rule(params(json!([{ "rule_json": rule_json }])))
                .await
                .expect_err("should refuse");
            assert_eq!(err.code, ErrorCode::INVALID_PARAMS);
            assert!(err.message.contains("item 0"), "{}", err.message);
            assert!(err.message.contains(needle), "{}", err.message);
        }
        handle.abort();
    }

    #[tokio::test]
    async fn update_rule_refuses_bad_conditions_before_any_rpc() {
        let (server, handle) =
            server_with_mocks(MockRuleServiceImpl::new(), MockWebhookServiceImpl::new()).await;

        for (conditions_json, needle) in [
            (
                format!(r#"[{{ "expression": {{}}, "actions": [{WEBHOOK_ACTION_JSON}] }}]"#),
                "webhook_ids",
            ),
            ("[]".to_string(), "exactly one"),
        ] {
            let err = server
                .update_rule(params(
                    json!([{ "rule_id": "rule-1", "conditions_json": conditions_json }]),
                ))
                .await
                .expect_err("should refuse");
            assert_eq!(err.code, ErrorCode::INVALID_PARAMS);
            assert!(err.message.contains(needle), "{}", err.message);
        }
        handle.abort();
    }

    #[tokio::test]
    async fn update_rule_replaces_webhooks_and_warns_about_the_detached_one() {
        let mut rule_mock = MockRuleServiceImpl::new();
        rule_mock
            .expect_get_rule()
            .returning(|_| rule_calling("wh-old"));
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
            .returning(updated);

        // One lookup validates the new set; a second names the webhook detached.
        let mut webhook_mock = MockWebhookServiceImpl::new();
        webhook_mock
            .expect_list_webhooks()
            .times(2)
            .returning(|req| {
                let webhooks = if req.get_ref().filter.contains("wh-new") {
                    vec![webhook("wh-new", "new alerts")]
                } else {
                    vec![webhook("wh-old", "old alerts")]
                };
                Ok(Response::new(ListWebhooksResponse {
                    webhooks,
                    next_page_token: String::new(),
                }))
            });

        let (server, handle) = server_with_mocks(rule_mock, webhook_mock).await;

        let body = structured(
            server
                .update_rule(params(
                    json!([{ "rule_id": "rule-1", "webhook_ids": ["wh-new"] }]),
                ))
                .await
                .expect("update_rule failed"),
        );

        let result = &body["successes"][0]["result"];
        assert_eq!(
            result["webhooks"],
            json!([{ "webhook_id": "wh-new", "name": "new alerts" }])
        );
        let warning = result["warning"].as_str().expect("warning");
        assert!(
            warning.contains("no longer calls webhook `old alerts`"),
            "{warning}"
        );
        handle.abort();
    }

    #[tokio::test]
    async fn update_rule_fails_an_item_whose_new_webhook_is_archived() {
        let mut rule_mock = MockRuleServiceImpl::new();
        rule_mock.expect_get_rule().returning(|_| {
            Ok(Response::new(GetRuleResponse {
                rule: Some(Rule {
                    rule_id: "rule-1".into(),
                    conditions: vec![RuleCondition::default()],
                    ..Default::default()
                }),
            }))
        });
        rule_mock.expect_update_rule().times(0);

        let (server, handle) = server_with_mocks(
            rule_mock,
            webhook_mock_returning(vec![archived("wh1", "old alerts")]),
        )
        .await;

        let result = server
            .update_rule(params(
                json!([{ "rule_id": "rule-1", "webhook_ids": ["wh1"] }]),
            ))
            .await
            .expect("update_rule failed");

        assert_eq!(result.is_error, Some(true));
        let body = structured(result);
        assert_eq!(
            body["failures"][0]["error"]["code"],
            ErrorCode::INVALID_PARAMS.0
        );
        handle.abort();
    }

    #[tokio::test]
    async fn update_rule_keeps_an_archived_webhook_the_rule_already_calls() {
        let mut rule_mock = MockRuleServiceImpl::new();
        rule_mock
            .expect_get_rule()
            .returning(|_| rule_calling("wh-old"));
        rule_mock
            .expect_update_rule()
            .withf(|req| {
                let actions = &req.get_ref().conditions[0].actions;
                actions.len() == 2 && actions[0].rule_action_id.as_deref() == Some("act-old")
            })
            .returning(updated);

        let (server, handle) = server_with_mocks(
            rule_mock,
            webhook_mock_returning(vec![
                archived("wh-old", "old alerts"),
                webhook("wh-new", "new alerts"),
            ]),
        )
        .await;

        let body = structured(
            server
                .update_rule(params(
                    json!([{ "rule_id": "rule-1", "webhook_ids": ["wh-old", "wh-new"] }]),
                ))
                .await
                .expect("update_rule failed"),
        );

        let result = &body["successes"][0]["result"];
        assert_eq!(result["webhooks"].as_array().unwrap().len(), 2);
        let warning = result["warning"].as_str().expect("warning");
        assert!(
            warning.contains("still lists archived webhook `old alerts`"),
            "{warning}"
        );
        assert!(!warning.contains("no longer calls"), "{warning}");
        handle.abort();
    }
}
