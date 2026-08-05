use sift_rs::{
    metadata::v1::{MetadataKey, MetadataKeyType, MetadataValue, metadata_value::Value},
    report_templates::v1::{
        CreateReportTemplateResponse, ListReportTemplatesResponse, ReportTemplate,
        ReportTemplateRule, UpdateReportTemplateResponse, create_report_template_request,
        report_template_service_server::ReportTemplateServiceServer,
    },
};
use sift_test_util::{
    grpc::memory_sift_channel, mock::report_templates::v1::MockReportTemplateServiceImpl,
};
use tokio::task::JoinHandle;
use tonic::{Response, Status, transport::Server};

use super::{ReportTemplateService, ReportTemplateUpdate, TemplateRuleIdentifier};
use crate::service::common::DEFAULT_LIMIT;

async fn service_with_mock(
    mock: MockReportTemplateServiceImpl,
) -> (ReportTemplateService, JoinHandle<()>) {
    let (client, server) = tokio::io::duplex(1024);
    let channel = memory_sift_channel(client).await;

    let handle = tokio::spawn(async move {
        Server::builder()
            .add_service(ReportTemplateServiceServer::new(mock))
            .serve_with_incoming(tokio_stream::once(Ok::<_, std::io::Error>(server)))
            .await
            .unwrap();
    });

    (
        ReportTemplateService::new(channel, crate::policy::RetryPolicy::default()),
        handle,
    )
}

#[tokio::test]
async fn list_report_templates_returns_single_page() {
    let mut mock = MockReportTemplateServiceImpl::new();
    mock.expect_list_report_templates()
        .withf(|req| req.get_ref().filter == "name == \"safety\"")
        .returning(|_| {
            Ok(Response::new(ListReportTemplatesResponse {
                report_templates: vec![ReportTemplate {
                    report_template_id: "tmpl-1".into(),
                    name: "safety".into(),
                    ..Default::default()
                }],
                next_page_token: String::new(),
            }))
        });

    let (service, _h) = service_with_mock(mock).await;

    let templates = service
        .list_report_templates("name == \"safety\"".to_string(), None, None, None)
        .await
        .expect("list_report_templates failed");

    assert_eq!(templates.len(), 1);
    assert_eq!(templates[0].report_template_id, "tmpl-1");
}

#[tokio::test]
async fn list_report_templates_forwards_organization_id() {
    let mut mock = MockReportTemplateServiceImpl::new();
    mock.expect_list_report_templates()
        .withf(|req| req.get_ref().organization_id == "org-123")
        .returning(|_| {
            Ok(Response::new(ListReportTemplatesResponse {
                report_templates: vec![ReportTemplate {
                    report_template_id: "tmpl-1".into(),
                    ..Default::default()
                }],
                next_page_token: String::new(),
            }))
        });

    let (service, _h) = service_with_mock(mock).await;

    let templates = service
        .list_report_templates(String::new(), None, None, Some("org-123".to_string()))
        .await
        .expect("list_report_templates failed");

    assert_eq!(templates.len(), 1);
}

#[tokio::test]
async fn list_report_templates_paginates_until_token_empty() {
    let mut mock = MockReportTemplateServiceImpl::new();
    mock.expect_list_report_templates().returning(|req| {
        let req = req.into_inner();
        assert_eq!(req.page_size, DEFAULT_LIMIT);
        let (report_templates, next) = match req.page_token.as_str() {
            "" => (
                vec![ReportTemplate {
                    report_template_id: "tmpl-1".into(),
                    ..Default::default()
                }],
                "page-2".to_string(),
            ),
            "page-2" => (
                vec![ReportTemplate {
                    report_template_id: "tmpl-2".into(),
                    ..Default::default()
                }],
                String::new(),
            ),
            other => return Err(Status::invalid_argument(format!("bad token: {other}"))),
        };
        Ok(Response::new(ListReportTemplatesResponse {
            report_templates,
            next_page_token: next,
        }))
    });

    let (service, _h) = service_with_mock(mock).await;

    let templates = service
        .list_report_templates(String::new(), None, None, None)
        .await
        .expect("list_report_templates failed");

    let ids: Vec<&str> = templates
        .iter()
        .map(|t| t.report_template_id.as_str())
        .collect();
    assert_eq!(ids, vec!["tmpl-1", "tmpl-2"]);
}

#[tokio::test]
async fn list_report_templates_respects_limit() {
    let mut mock = MockReportTemplateServiceImpl::new();
    mock.expect_list_report_templates()
        .times(1)
        .returning(|req| {
            assert_eq!(req.get_ref().page_size, 2);
            Ok(Response::new(ListReportTemplatesResponse {
                report_templates: vec![
                    ReportTemplate {
                        report_template_id: "tmpl-1".into(),
                        ..Default::default()
                    },
                    ReportTemplate {
                        report_template_id: "tmpl-2".into(),
                        ..Default::default()
                    },
                ],
                next_page_token: "page-2".into(),
            }))
        });

    let (service, _h) = service_with_mock(mock).await;

    let templates = service
        .list_report_templates(String::new(), None, Some(2), None)
        .await
        .expect("list_report_templates failed");

    assert_eq!(templates.len(), 2);
}

#[tokio::test]
async fn list_report_templates_propagates_grpc_error() {
    let mut mock = MockReportTemplateServiceImpl::new();
    mock.expect_list_report_templates()
        .returning(|_| Err(Status::not_found("no such template")));

    let (service, _h) = service_with_mock(mock).await;

    let err = service
        .list_report_templates(String::new(), None, None, None)
        .await
        .expect_err("expected error");

    assert!(err.to_string().contains("failed to query report templates"));
}

#[tokio::test]
async fn create_report_template_forwards_rule_ids() {
    let mut mock = MockReportTemplateServiceImpl::new();
    mock.expect_create_report_template()
        .withf(|req| {
            let req = req.get_ref();
            req.name == "safety"
                && matches!(
                    &req.rule_identifiers,
                    Some(create_report_template_request::RuleIdentifiers::RuleIds(r))
                        if r.rule_ids == vec!["rule-1".to_string(), "rule-2".to_string()]
                )
        })
        .returning(|_| {
            Ok(Response::new(CreateReportTemplateResponse {
                report_template: Some(ReportTemplate {
                    report_template_id: "tmpl-new".into(),
                    name: "safety".into(),
                    ..Default::default()
                }),
            }))
        });

    let (service, _h) = service_with_mock(mock).await;

    let template = service
        .create_report_template(
            None,
            "safety".to_string(),
            None,
            None,
            Vec::new(),
            TemplateRuleIdentifier::RuleIds(vec!["rule-1".into(), "rule-2".into()]),
            Vec::new(),
        )
        .await
        .expect("create_report_template failed");

    assert_eq!(template.report_template_id, "tmpl-new");
}

#[tokio::test]
async fn create_report_template_forwards_client_keys() {
    let mut mock = MockReportTemplateServiceImpl::new();
    mock.expect_create_report_template()
        .withf(|req| {
            matches!(
                &req.get_ref().rule_identifiers,
                Some(create_report_template_request::RuleIdentifiers::RuleClientKeys(r))
                    if r.rule_client_keys == vec!["k1".to_string()]
            )
        })
        .returning(|_| {
            Ok(Response::new(CreateReportTemplateResponse {
                report_template: Some(ReportTemplate {
                    report_template_id: "tmpl-new".into(),
                    ..Default::default()
                }),
            }))
        });

    let (service, _h) = service_with_mock(mock).await;

    service
        .create_report_template(
            None,
            "safety".to_string(),
            None,
            None,
            Vec::new(),
            TemplateRuleIdentifier::RuleClientKeys(vec!["k1".into()]),
            Vec::new(),
        )
        .await
        .expect("create_report_template failed");
}

#[tokio::test]
async fn create_report_template_missing_body_errors() {
    let mut mock = MockReportTemplateServiceImpl::new();
    mock.expect_create_report_template()
        .returning(|_| Ok(Response::new(CreateReportTemplateResponse::default())));

    let (service, _h) = service_with_mock(mock).await;

    let err = service
        .create_report_template(
            None,
            "safety".to_string(),
            None,
            None,
            Vec::new(),
            TemplateRuleIdentifier::RuleIds(vec!["r1".into()]),
            Vec::new(),
        )
        .await
        .expect_err("expected missing report_template error");

    assert!(err.to_string().contains("missing report_template"));
}

#[tokio::test]
async fn update_report_template_builds_mask_for_provided_fields() {
    let mut mock = MockReportTemplateServiceImpl::new();
    mock.expect_update_report_template()
        .withf(|req| {
            let req = req.get_ref();
            let mask_paths = req
                .update_mask
                .as_ref()
                .map(|m| m.paths.clone())
                .unwrap_or_default();
            let template = req.report_template.as_ref().expect("template");

            template.report_template_id == "tmpl-1"
                && template.name == "renamed"
                && mask_paths == vec!["name".to_string(), "tags".to_string()]
                && template.tags.iter().map(|t| t.tag_name.as_str()).eq(["qa"])
        })
        .returning(|_| {
            Ok(Response::new(UpdateReportTemplateResponse {
                report_template: Some(ReportTemplate {
                    report_template_id: "tmpl-1".into(),
                    name: "renamed".into(),
                    ..Default::default()
                }),
            }))
        });

    let (service, _h) = service_with_mock(mock).await;

    let updated = service
        .update_report_template(
            "tmpl-1".to_string(),
            ReportTemplateUpdate {
                name: Some("renamed".to_string()),
                tag_names: Some(vec!["qa".to_string()]),
                ..Default::default()
            },
        )
        .await
        .expect("update_report_template failed");

    assert_eq!(updated.name, "renamed");
}

#[tokio::test]
async fn update_report_template_replaces_rules() {
    let mut mock = MockReportTemplateServiceImpl::new();
    mock.expect_update_report_template()
        .withf(|req| {
            let req = req.get_ref();
            let paths = req
                .update_mask
                .as_ref()
                .map(|m| m.paths.clone())
                .unwrap_or_default();
            let template = req.report_template.as_ref().expect("template");
            let rule_ids: Vec<&str> = template.rules.iter().map(|r| r.rule_id.as_str()).collect();

            paths == vec!["rules".to_string()]
                && rule_ids == vec!["r1", "r2"]
                && template.rules.iter().all(|r| r.client_key.is_empty())
        })
        .returning(|_| {
            Ok(Response::new(UpdateReportTemplateResponse {
                report_template: Some(ReportTemplate {
                    report_template_id: "tmpl-1".into(),
                    rules: vec![
                        ReportTemplateRule {
                            rule_id: "r1".into(),
                            ..Default::default()
                        },
                        ReportTemplateRule {
                            rule_id: "r2".into(),
                            ..Default::default()
                        },
                    ],
                    ..Default::default()
                }),
            }))
        });

    let (service, _h) = service_with_mock(mock).await;

    service
        .update_report_template(
            "tmpl-1".to_string(),
            ReportTemplateUpdate {
                rules: Some(TemplateRuleIdentifier::RuleIds(vec![
                    "r1".to_string(),
                    "r2".to_string(),
                ])),
                ..Default::default()
            },
        )
        .await
        .expect("update_report_template failed");
}

#[tokio::test]
async fn update_report_template_metadata_and_archive() {
    let metadata_entry = MetadataValue {
        key: Some(MetadataKey {
            name: "owner".into(),
            r#type: MetadataKeyType::String.into(),
            ..Default::default()
        }),
        value: Some(Value::StringValue("qa-team".into())),
        ..Default::default()
    };

    let mut mock = MockReportTemplateServiceImpl::new();
    mock.expect_update_report_template()
        .withf(|req| {
            let req = req.get_ref();
            let paths = req
                .update_mask
                .as_ref()
                .map(|m| m.paths.clone())
                .unwrap_or_default();
            let template = req.report_template.as_ref().expect("template");

            paths == vec!["metadata".to_string(), "is_archived".to_string()]
                && template.metadata.len() == 1
                && template.is_archived
        })
        .returning(|_| {
            Ok(Response::new(UpdateReportTemplateResponse {
                report_template: Some(ReportTemplate {
                    report_template_id: "tmpl-1".into(),
                    is_archived: true,
                    ..Default::default()
                }),
            }))
        });

    let (service, _h) = service_with_mock(mock).await;

    service
        .update_report_template(
            "tmpl-1".to_string(),
            ReportTemplateUpdate {
                metadata: Some(vec![metadata_entry]),
                is_archived: Some(true),
                ..Default::default()
            },
        )
        .await
        .expect("update_report_template failed");
}
