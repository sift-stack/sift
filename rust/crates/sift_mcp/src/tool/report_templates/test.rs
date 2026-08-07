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

use super::{CreateReportTemplateParams, ReportTemplateListParams, UpdateReportTemplateParams};
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

fn create_params() -> CreateReportTemplateParams {
    CreateReportTemplateParams {
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

fn update_params() -> UpdateReportTemplateParams {
    UpdateReportTemplateParams {
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
        .create_report_template(Parameters(params))
        .await
        .expect("create_report_template failed");

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
        .create_report_template(Parameters(params))
        .await
        .expect_err("expected error");

    assert_eq!(err.code, ErrorCode::INVALID_PARAMS);
}

#[tokio::test]
async fn create_report_template_rejects_no_rules() {
    let (server, _h) = server_with_mock(MockReportTemplateServiceImpl::new()).await;

    let err = server
        .create_report_template(Parameters(create_params()))
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
        .create_report_template(Parameters(params))
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
        .update_report_template(Parameters(params))
        .await
        .expect("update_report_template failed");

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
        .update_report_template(Parameters(params))
        .await
        .expect_err("expected error");

    assert_eq!(err.code, ErrorCode::INVALID_PARAMS);
}

#[tokio::test]
async fn update_report_template_rejects_no_fields() {
    let (server, _h) = server_with_mock(MockReportTemplateServiceImpl::new()).await;

    let err = server
        .update_report_template(Parameters(update_params()))
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
        .update_report_template(Parameters(params))
        .await
        .expect_err("expected error");

    assert_eq!(err.code, ErrorCode::INVALID_PARAMS);
}

#[tokio::test]
async fn create_report_template_blocked_without_allow_create() {
    let mock = MockReportTemplateServiceImpl::new();
    let (server, _h) = server_with_mock_and_flags(mock, false, false).await;

    let err = server
        .create_report_template(Parameters(create_params()))
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
        .update_report_template(Parameters(params))
        .await
        .expect_err("expected destructive gate to reject the call");

    assert_eq!(err.code, ErrorCode::INVALID_REQUEST);
    assert!(err.message.contains("--allow-destructive"));
}
