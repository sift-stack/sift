use rmcp::model::ErrorCode;

use super::{parse_rule_definition, rule_identifier};

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

mod bulk {
    use std::sync::{Arc, Mutex};

    use rmcp::{handler::server::wrapper::Parameters, model::ErrorCode};
    use serde_json::{Value, json};
    use sift_rs::rules::v1::{
        ArchiveRuleResponse, GetRuleResponse, Rule, UpdateRuleResponse,
        rule_service_server::RuleServiceServer,
    };
    use sift_test_util::{grpc::memory_sift_channel, mock::rules::v1::MockRuleServiceImpl};
    use tonic::{Response, Status, transport::Server};

    use crate::{server::SiftMcpServer, tool::common::test_support::structured};

    async fn server(mock: MockRuleServiceImpl) -> (SiftMcpServer, tokio::task::JoinHandle<()>) {
        let (client, incoming) = tokio::io::duplex(1024);
        let channel = memory_sift_channel(client).await;
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
    async fn permission_denied_fails_one_item_and_unavailable_stops_the_rest() {
        let mut mock = MockRuleServiceImpl::new();
        mock.expect_get_rule()
            .returning(|request| match request.get_ref().rule_id.as_str() {
                "denied" => Err(Status::permission_denied("no edit permission")),
                "down" => Err(Status::unavailable("backend down")),
                rule_id => found(rule_id),
            });
        mock.expect_archive_rule()
            .times(1)
            .returning(|_| Ok(Response::new(ArchiveRuleResponse {})));
        let (server, handle) = server(mock).await;
        let items = ["denied", "ok", "down", "never"].map(|id| json!({"rule_id": id}));
        let body = structured(
            server
                .archive_rule(params(
                    json!({"items": items, "selected_indices": [0, 1, 2, 3]}),
                ))
                .await
                .unwrap(),
        );
        assert_eq!(body["archived_count"], 1);
        assert_eq!(body["failures"][0]["index"], 0);
        assert_eq!(
            body["failures"][0]["error"]["data"]["reason"],
            "PermissionDenied"
        );
        assert_eq!(body["failures"][1]["index"], 2);
        assert_eq!(body["not_attempted"], json!([3]));
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
    async fn the_chat_host_must_send_a_mask() {
        let (server, handle) = server(MockRuleServiceImpl::new()).await;
        let error = server
            .archive_rule(params(json!({"items": [{"rule_id": "a"}]})))
            .await
            .unwrap_err();
        assert!(
            error.message.contains("nothing was written"),
            "{}",
            error.message
        );
        handle.abort();
    }

    #[test]
    fn the_legacy_single_rule_shape_is_rejected() {
        let error = serde_json::from_value::<
            crate::tool::common::bulk::BulkItems<super::super::RuleArchiveItem>,
        >(json!({"rule_id": "a"}))
        .unwrap_err();
        assert!(error.to_string().contains("`items`"), "{error}");
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
