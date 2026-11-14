use rmcp::{handler::server::wrapper::Parameters, model::ErrorCode};
use sift_rs::report_templates::v1::{
    CreateReportTemplateResponse, ListReportTemplatesResponse, ReportTemplate,
    UpdateReportTemplateResponse, report_template_service_server::ReportTemplateServiceServer,
};
use sift_test_util::{
    grpc::memory_sift_channel, mock::report_templates::v1::MockReportTemplateServiceImpl,
};
use tokio::task::JoinHandle;
use tonic::{Response, Status, transport::Server};

use super::{CreateReportTemplateItem, ReportTemplateListParams, UpdateReportTemplateItem};
use crate::{server::SiftMcpServer, tool::common::test_support::structured_field};

async fn server_with_mock(mock: MockReportTemplateServiceImpl) -> (SiftMcpServer, JoinHandle<()>) {
    server_with_mock_and_flags(mock, true, true).await
}

async fn server_with_mock_and_flags(
    mock: MockReportTemplateServiceImpl,
    allow_create: bool,
    allow_destructive: bool,
) -> (SiftMcpServer, JoinHandle<()>) {
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
        SiftMcpServer::new(
            channel,
            String::from("https://app.test.local"),
            allow_create,
            allow_destructive,
        ),
        handle,
    )
}

fn create_params() -> CreateReportTemplateItem {
    CreateReportTemplateItem {
        name: "safety".into(),
        description: None,
        client_key: None,
        tag_names: None,
        organization_id: None,
        metadata: None,
        rule_ids: None,
        rule_client_keys: None,
    }
}

mod bulk {
    use super::*;
    use serde_json::{Value, json};

    async fn call(server: &SiftMcpServer, value: Value) -> crate::error::McpResult {
        server
            .update_report_template(Parameters(serde_json::from_value(value).unwrap()))
            .await
    }
    fn item(id: &str) -> Value {
        json!({"report_template_id": id, "name": "renamed"})
    }
    fn body(result: rmcp::model::CallToolResult) -> Value {
        result.structured_content.unwrap()
    }

    #[tokio::test]
    async fn selected_subset_writes_only_selected_items() {
        let mut mock = MockReportTemplateServiceImpl::new();
        mock.expect_update_report_template()
            .times(1)
            .withf(|req| {
                req.get_ref()
                    .report_template
                    .as_ref()
                    .unwrap()
                    .report_template_id
                    == "selected"
            })
            .returning(|req| {
                Ok(Response::new(UpdateReportTemplateResponse {
                    report_template: req.into_inner().report_template,
                }))
            });
        let (server, handle) = server_with_mock(mock).await;
        let body = body(
            call(
                &server,
                json!({"items": [item("skip"), item("selected")], "selected_indices": [1]}),
            )
            .await
            .unwrap(),
        );
        assert_eq!(body["successes"][0]["index"], 1);
        assert_eq!(body["not_selected"], json!([0]));
        handle.abort();
    }

    #[tokio::test]
    async fn invalid_duplicate_and_missing_chat_mask_fail_before_rpc() {
        let (server, handle) = server_with_mock(MockReportTemplateServiceImpl::new()).await;
        let server = server.with_require_selection(true);
        for items in [
            json!([item("a"), item("a")]),
            json!([item("a"), item(" ")]),
            json!([{"report_template_id": "a", "unknown": true}]),
        ] {
            let error = call(&server, json!({"items": items, "selected_indices": [0]}))
                .await
                .unwrap_err();
            assert_eq!(error.code, ErrorCode::INVALID_PARAMS);
        }
        let error = call(&server, json!({"items": [item("a")]}))
            .await
            .unwrap_err();
        assert!(error.message.contains("nothing was written"));
        handle.abort();
    }
}

fn update_call(
    item: UpdateReportTemplateItem,
) -> Parameters<super::BulkItems<UpdateReportTemplateItem>> {
    assert!(item.metadata.is_none());
    Parameters(
        serde_json::from_value(serde_json::json!({"items": [{
            "report_template_id": item.report_template_id, "name": item.name,
            "description": item.description, "tag_names": item.tag_names,
            "rule_ids": item.rule_ids, "rule_client_keys": item.rule_client_keys,
            "is_archived": item.is_archived,
        }]}))
        .unwrap(),
    )
}

fn update_params() -> UpdateReportTemplateItem {
    UpdateReportTemplateItem {
        report_template_id: "tmpl-1".into(),
        name: None,
        description: None,
        tag_names: None,
        rule_ids: None,
        rule_client_keys: None,
        metadata: None,
        is_archived: None,
    }
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

    let (server, _h) = server_with_mock(mock).await;

    let resp = server
        .list_report_templates(Parameters(ReportTemplateListParams {
            filter: "name == \"safety\"".into(),
            order_by: None,
            limit: None,
            organization_id: None,
        }))
        .await
        .expect("list_report_templates failed");

    let templates = structured_field(resp, "report_templates");
    let arr = templates.as_array().unwrap();
    assert_eq!(arr.len(), 1);
    assert_eq!(arr[0]["reportTemplateId"], "tmpl-1");
    assert_eq!(
        arr[0]["url"],
        "https://app.test.local/report-templates/tmpl-1"
    );
}

#[tokio::test]
async fn list_report_templates_forwards_organization_id() {
    let mut mock = MockReportTemplateServiceImpl::new();
    mock.expect_list_report_templates()
        .withf(|req| req.get_ref().organization_id == "org-7")
        .returning(|_| {
            Ok(Response::new(ListReportTemplatesResponse {
                report_templates: vec![ReportTemplate {
                    report_template_id: "tmpl-1".into(),
                    ..Default::default()
                }],
                next_page_token: String::new(),
            }))
        });

    let (server, _h) = server_with_mock(mock).await;

    server
        .list_report_templates(Parameters(ReportTemplateListParams {
            filter: String::new(),
            order_by: None,
            limit: None,
            organization_id: Some("org-7".into()),
        }))
        .await
        .expect("list_report_templates failed");
}

#[tokio::test]
async fn list_report_templates_propagates_grpc_error() {
    let mut mock = MockReportTemplateServiceImpl::new();
    mock.expect_list_report_templates()
        .returning(|_| Err(Status::invalid_argument("bad filter")));

    let (server, _h) = server_with_mock(mock).await;

    let err = server
        .list_report_templates(Parameters(ReportTemplateListParams {
            filter: "nope".into(),
            order_by: None,
            limit: None,
            organization_id: None,
        }))
        .await
        .expect_err("expected error");

    assert_eq!(err.code, ErrorCode::INVALID_PARAMS);
    assert!(err.message.contains("bad filter"));
}

#[tokio::test]
async fn create_report_template_happy_path() {
    let mut mock = MockReportTemplateServiceImpl::new();
    mock.expect_create_report_template().returning(|_| {
        Ok(Response::new(CreateReportTemplateResponse {
            report_template: Some(ReportTemplate {
                report_template_id: "tmpl-new".into(),
                name: "safety".into(),
                ..Default::default()
            }),
        }))
    });

    let (server, _h) = server_with_mock(mock).await;

    let mut params = create_params();
    params.rule_ids = Some(vec!["rule-1".into(), "rule-2".into()]);

    let resp = server
        .create_report_template_item(Parameters(params))
        .await
        .expect("create_report_template failed");

    let url = structured_field(resp.clone(), "report_template_url");
    assert_eq!(url, "https://app.test.local/report-templates/tmpl-new");
    let template = structured_field(resp, "report_template");
    assert_eq!(template["reportTemplateId"], "tmpl-new");
}

#[tokio::test]
async fn create_report_template_rejects_empty_name() {
    let (server, _h) = server_with_mock(MockReportTemplateServiceImpl::new()).await;

    let mut params = create_params();
    params.name = String::new();
    params.rule_ids = Some(vec!["rule-1".into()]);

    let err = server
        .create_report_template_item(Parameters(params))
        .await
        .expect_err("expected error");

    assert_eq!(err.code, ErrorCode::INVALID_PARAMS);
}

#[tokio::test]
async fn create_report_template_rejects_no_rules() {
    let (server, _h) = server_with_mock(MockReportTemplateServiceImpl::new()).await;

    let err = server
        .create_report_template_item(Parameters(create_params()))
        .await
        .expect_err("expected error");

    assert_eq!(err.code, ErrorCode::INVALID_PARAMS);
}

#[tokio::test]
async fn create_report_template_rejects_both_rule_shapes() {
    let (server, _h) = server_with_mock(MockReportTemplateServiceImpl::new()).await;

    let mut params = create_params();
    params.rule_ids = Some(vec!["rule-1".into()]);
    params.rule_client_keys = Some(vec!["ck-1".into()]);

    let err = server
        .create_report_template_item(Parameters(params))
        .await
        .expect_err("expected error");

    assert_eq!(err.code, ErrorCode::INVALID_PARAMS);
}

#[tokio::test]
async fn update_report_template_happy_path() {
    let mut mock = MockReportTemplateServiceImpl::new();
    mock.expect_update_report_template().returning(|_| {
        Ok(Response::new(UpdateReportTemplateResponse {
            report_template: Some(ReportTemplate {
                report_template_id: "tmpl-1".into(),
                name: "renamed".into(),
                ..Default::default()
            }),
        }))
    });

    let (server, _h) = server_with_mock(mock).await;

    let mut params = update_params();
    params.name = Some("renamed".into());

    let resp = server
        .update_report_template(update_call(params))
        .await
        .expect("update_report_template failed");
    let resp = rmcp::model::CallToolResult::structured(
        structured_field(resp, "successes")[0]["result"].clone(),
    );

    let url = structured_field(resp.clone(), "report_template_url");
    assert_eq!(url, "https://app.test.local/report-templates/tmpl-1");
    let template = structured_field(resp, "report_template");
    assert_eq!(template["name"], "renamed");
}

#[tokio::test]
async fn update_report_template_rejects_empty_id() {
    let (server, _h) = server_with_mock(MockReportTemplateServiceImpl::new()).await;

    let mut params = update_params();
    params.report_template_id = String::new();
    params.name = Some("renamed".into());

    let err = server
        .update_report_template(update_call(params))
        .await
        .expect_err("expected error");

    assert_eq!(err.code, ErrorCode::INVALID_PARAMS);
}

#[tokio::test]
async fn update_report_template_rejects_no_fields() {
    let (server, _h) = server_with_mock(MockReportTemplateServiceImpl::new()).await;

    let err = server
        .update_report_template(update_call(update_params()))
        .await
        .expect_err("expected error");

    assert_eq!(err.code, ErrorCode::INVALID_PARAMS);
}

#[tokio::test]
async fn update_report_template_rejects_both_rule_shapes() {
    let (server, _h) = server_with_mock(MockReportTemplateServiceImpl::new()).await;

    let mut params = update_params();
    params.rule_ids = Some(vec!["rule-1".into()]);
    params.rule_client_keys = Some(vec!["ck-1".into()]);

    let err = server
        .update_report_template(update_call(params))
        .await
        .expect_err("expected error");

    assert_eq!(err.code, ErrorCode::INVALID_PARAMS);
}

#[tokio::test]
async fn create_report_template_blocked_without_allow_create() {
    let mock = MockReportTemplateServiceImpl::new();
    let (server, _h) = server_with_mock_and_flags(mock, false, false).await;

    let err = server
        .create_report_template_item(Parameters(create_params()))
        .await
        .expect_err("expected create gate to reject the call");

    assert_eq!(err.code, ErrorCode::INVALID_REQUEST);
    assert!(err.message.contains("--allow-create"));
}

#[tokio::test]
async fn update_report_template_blocked_without_allow_destructive() {
    let mock = MockReportTemplateServiceImpl::new();
    let (server, _h) = server_with_mock_and_flags(mock, false, false).await;

    let mut params = update_params();
    params.name = Some("renamed".into());

    let err = server
        .update_report_template(update_call(params))
        .await
        .expect_err("expected destructive gate to reject the call");

    assert_eq!(err.code, ErrorCode::INVALID_REQUEST);
    assert!(err.message.contains("--allow-destructive"));
}
