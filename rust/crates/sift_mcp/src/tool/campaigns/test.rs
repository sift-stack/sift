use rmcp::model::ErrorCode;
use sift_rs::campaigns::v1::{
    Campaign, CampaignReport, GetCampaignReportSummariesResponse, ListCampaignsResponse,
    campaign_service_server::CampaignServiceServer,
};
use sift_test_util::{grpc::memory_sift_channel, mock::campaigns::v1::MockCampaignServiceImpl};
use tokio::task::JoinHandle;
use tonic::{Response, Status, transport::Server};

use crate::{
    server::SiftMcpServer,
    tool::campaigns::{CampaignListParams, ReviewCampaignsParams},
    tool::common::test_support::{structured, structured_field},
};
use rmcp::handler::server::wrapper::Parameters;

fn list_params(filter: &str) -> Parameters<CampaignListParams> {
    Parameters(CampaignListParams {
        filter: filter.to_string(),
        order_by: None,
        limit: None,
        include_archived: None,
        organization_id: None,
    })
}

async fn server_with_mock(mock: MockCampaignServiceImpl) -> (SiftMcpServer, JoinHandle<()>) {
    let (client, server) = tokio::io::duplex(1024);
    let channel = memory_sift_channel(client).await;

    let handle = tokio::spawn(async move {
        Server::builder()
            .add_service(CampaignServiceServer::new(mock))
            .serve_with_incoming(tokio_stream::once(Ok::<_, std::io::Error>(server)))
            .await
            .unwrap();
    });

    (
        SiftMcpServer::new(
            channel,
            String::from("https://app.test.local"),
            false,
            false,
        ),
        handle,
    )
}

#[tokio::test]
async fn list_campaigns_returns_single_page() {
    let mut mock = MockCampaignServiceImpl::new();
    mock.expect_list_campaigns()
        .withf(|req| req.get_ref().filter == "name == \"launch\"")
        .returning(|_| {
            Ok(Response::new(ListCampaignsResponse {
                campaigns: vec![Campaign {
                    campaign_id: "campaign1".into(),
                    name: "launch".into(),
                    ..Default::default()
                }],
                next_page_token: String::new(),
            }))
        });

    let (server, _h) = server_with_mock(mock).await;

    let resp = server
        .list_campaigns(list_params("name == \"launch\""))
        .await
        .expect("list_campaigns failed");

    let campaigns = structured_field(resp, "campaigns");
    assert_eq!(campaigns.as_array().unwrap().len(), 1);
    assert_eq!(campaigns[0]["campaignId"], "campaign1");
}

#[tokio::test]
async fn list_campaigns_propagates_grpc_error() {
    let mut mock = MockCampaignServiceImpl::new();
    mock.expect_list_campaigns()
        .returning(|_| Err(Status::invalid_argument("bad filter")));

    let (server, _h) = server_with_mock(mock).await;

    let err = server
        .list_campaigns(list_params("nope"))
        .await
        .expect_err("expected error");

    assert_eq!(err.code, ErrorCode::INVALID_PARAMS);
    assert!(err.message.contains("bad filter"));
}

#[tokio::test]
async fn review_campaigns_returns_summaries() {
    let mut mock = MockCampaignServiceImpl::new();
    mock.expect_get_campaign_report_summaries()
        .withf(|req| req.get_ref().campaign_ids == vec!["campaign1".to_string()])
        .returning(|_| {
            let mut summaries = std::collections::HashMap::new();
            summaries.insert(
                "campaign1".to_string(),
                sift_rs::campaigns::v1::CampaignReports {
                    reports: vec![CampaignReport {
                        report_id: "report1".into(),
                        report_name: "report one".into(),
                        num_annotations: 5,
                        num_passed_rules: 2,
                        num_accepted_rules: 1,
                        num_failed_rules: 1,
                        num_open_rules: 0,
                    }],
                },
            );
            Ok(Response::new(GetCampaignReportSummariesResponse {
                summaries_by_campaign_id: summaries,
            }))
        });

    let (server, _h) = server_with_mock(mock).await;

    let resp = server
        .review_campaigns(Parameters(ReviewCampaignsParams {
            campaign_ids: vec!["campaign1".to_string()],
            organization_id: None,
        }))
        .await
        .expect("review_campaigns failed");

    let summaries = structured(resp);
    let reports = summaries["summaries"]["campaign1"].as_array().unwrap();
    assert_eq!(reports.len(), 1);
    assert_eq!(reports[0]["numAnnotations"], 5);
}

#[tokio::test]
async fn review_campaigns_rejects_empty_campaign_ids() {
    // No expectations on the mock: validation must fire before any RPC.
    let mock = MockCampaignServiceImpl::new();
    let (server, _h) = server_with_mock(mock).await;

    let err = server
        .review_campaigns(Parameters(ReviewCampaignsParams {
            campaign_ids: vec![],
            organization_id: None,
        }))
        .await
        .expect_err("expected validation error");

    assert_eq!(err.code, ErrorCode::INVALID_PARAMS);
    assert!(err.message.contains("campaign_ids"));
}

#[tokio::test]
async fn review_campaigns_propagates_grpc_error() {
    let mut mock = MockCampaignServiceImpl::new();
    mock.expect_get_campaign_report_summaries()
        .returning(|_| Err(Status::not_found("no such campaign")));

    let (server, _h) = server_with_mock(mock).await;

    let err = server
        .review_campaigns(Parameters(ReviewCampaignsParams {
            campaign_ids: vec!["missing".to_string()],
            organization_id: None,
        }))
        .await
        .expect_err("expected error");

    assert_eq!(err.code, ErrorCode::RESOURCE_NOT_FOUND);
}
