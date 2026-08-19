use sift_rs::campaigns::v1::{
    Campaign, CampaignReport, GetCampaignReportSummariesResponse, ListCampaignsResponse,
    campaign_service_server::CampaignServiceServer,
};

use super::CampaignService;
use sift_test_util::{grpc::memory_sift_channel, mock::campaigns::v1::MockCampaignServiceImpl};
use tokio::task::JoinHandle;
use tonic::{Response, Status, transport::Server};

use crate::service::common::DEFAULT_LIMIT;

async fn service_with_mock(mock: MockCampaignServiceImpl) -> (CampaignService, JoinHandle<()>) {
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
        CampaignService::new(channel, crate::policy::RetryPolicy::default()),
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

    let (service, _h) = service_with_mock(mock).await;

    let campaigns = service
        .list_campaigns("name == \"launch\"".to_string(), None, None, None, None)
        .await
        .expect("list_campaigns failed");

    assert_eq!(campaigns.len(), 1);
    assert_eq!(campaigns[0].campaign_id, "campaign1");
}

/// Load-bearing test: `list_campaigns` MUST set `skip_report_summaries = true`
/// on the outgoing request so listing stays cheap. `review_campaigns` is the
/// dedicated path for the expensive summary data.
#[tokio::test]
async fn list_campaigns_sets_skip_report_summaries() {
    let mut mock = MockCampaignServiceImpl::new();
    mock.expect_list_campaigns()
        .withf(|req| req.get_ref().skip_report_summaries)
        .returning(|_| {
            Ok(Response::new(ListCampaignsResponse {
                campaigns: vec![Campaign {
                    campaign_id: "campaign1".into(),
                    ..Default::default()
                }],
                next_page_token: String::new(),
            }))
        });

    let (service, _h) = service_with_mock(mock).await;

    let campaigns = service
        .list_campaigns(String::new(), None, None, None, None)
        .await
        .expect("list_campaigns failed");

    assert_eq!(campaigns.len(), 1);
}

#[tokio::test]
async fn list_campaigns_forwards_order_by_and_include_archived() {
    let mut mock = MockCampaignServiceImpl::new();
    mock.expect_list_campaigns()
        .withf(|req| {
            let req = req.get_ref();
            req.order_by == "created_date desc" && req.include_archived
        })
        .returning(|_| {
            Ok(Response::new(ListCampaignsResponse {
                campaigns: vec![Campaign {
                    campaign_id: "campaign1".into(),
                    ..Default::default()
                }],
                next_page_token: String::new(),
            }))
        });

    let (service, _h) = service_with_mock(mock).await;

    let campaigns = service
        .list_campaigns(
            String::new(),
            Some("created_date desc".to_string()),
            None,
            Some(true),
            None,
        )
        .await
        .expect("list_campaigns failed");

    assert_eq!(campaigns.len(), 1);
}

#[tokio::test]
async fn list_campaigns_forwards_organization_id() {
    let mut mock = MockCampaignServiceImpl::new();
    mock.expect_list_campaigns()
        .withf(|req| req.get_ref().organization_id == "org-1")
        .returning(|_| {
            Ok(Response::new(ListCampaignsResponse {
                campaigns: vec![Campaign {
                    campaign_id: "campaign1".into(),
                    ..Default::default()
                }],
                next_page_token: String::new(),
            }))
        });

    let (service, _h) = service_with_mock(mock).await;

    let campaigns = service
        .list_campaigns(String::new(), None, None, None, Some("org-1".to_string()))
        .await
        .expect("list_campaigns failed");

    assert_eq!(campaigns.len(), 1);
}

#[tokio::test]
async fn list_campaigns_paginates_until_token_empty() {
    let mut mock = MockCampaignServiceImpl::new();
    mock.expect_list_campaigns().returning(|req| {
        let req = req.into_inner();
        assert_eq!(req.page_size, DEFAULT_LIMIT);
        let (campaigns, next) = match req.page_token.as_str() {
            "" => (
                vec![Campaign {
                    campaign_id: "campaign1".into(),
                    ..Default::default()
                }],
                "page-2".to_string(),
            ),
            "page-2" => (
                vec![Campaign {
                    campaign_id: "campaign2".into(),
                    ..Default::default()
                }],
                String::new(),
            ),
            other => return Err(Status::invalid_argument(format!("bad token: {other}"))),
        };
        Ok(Response::new(ListCampaignsResponse {
            campaigns,
            next_page_token: next,
        }))
    });

    let (service, _h) = service_with_mock(mock).await;

    let campaigns = service
        .list_campaigns(String::new(), None, None, None, None)
        .await
        .expect("list_campaigns failed");

    let ids: Vec<&str> = campaigns.iter().map(|c| c.campaign_id.as_str()).collect();
    assert_eq!(ids, vec!["campaign1", "campaign2"]);
}

#[tokio::test]
async fn list_campaigns_respects_limit() {
    let mut mock = MockCampaignServiceImpl::new();
    mock.expect_list_campaigns().times(1).returning(|req| {
        let req = req.into_inner();
        assert_eq!(req.page_size, 2);
        Ok(Response::new(ListCampaignsResponse {
            campaigns: vec![
                Campaign {
                    campaign_id: "campaign1".into(),
                    ..Default::default()
                },
                Campaign {
                    campaign_id: "campaign2".into(),
                    ..Default::default()
                },
            ],
            next_page_token: "page-2".into(),
        }))
    });

    let (service, _h) = service_with_mock(mock).await;

    let campaigns = service
        .list_campaigns(String::new(), None, Some(2), None, None)
        .await
        .expect("list_campaigns failed");

    assert_eq!(campaigns.len(), 2);
}

#[tokio::test]
async fn list_campaigns_truncates_to_limit_across_pages() {
    let mut mock = MockCampaignServiceImpl::new();
    mock.expect_list_campaigns().returning(|req| {
        let req = req.into_inner();
        assert_eq!(req.page_size, 3);
        let (campaigns, next) = match req.page_token.as_str() {
            "" => (
                vec![
                    Campaign {
                        campaign_id: "campaign1".into(),
                        ..Default::default()
                    },
                    Campaign {
                        campaign_id: "campaign2".into(),
                        ..Default::default()
                    },
                ],
                "page-2".to_string(),
            ),
            "page-2" => (
                vec![
                    Campaign {
                        campaign_id: "campaign3".into(),
                        ..Default::default()
                    },
                    Campaign {
                        campaign_id: "campaign4".into(),
                        ..Default::default()
                    },
                ],
                String::new(),
            ),
            other => return Err(Status::invalid_argument(format!("bad token: {other}"))),
        };
        Ok(Response::new(ListCampaignsResponse {
            campaigns,
            next_page_token: next,
        }))
    });

    let (service, _h) = service_with_mock(mock).await;

    let campaigns = service
        .list_campaigns(String::new(), None, Some(3), None, None)
        .await
        .expect("list_campaigns failed");

    let ids: Vec<&str> = campaigns.iter().map(|c| c.campaign_id.as_str()).collect();
    assert_eq!(ids, vec!["campaign1", "campaign2", "campaign3"]);
}

#[tokio::test]
async fn list_campaigns_breaks_on_empty_page() {
    let mut mock = MockCampaignServiceImpl::new();
    mock.expect_list_campaigns().times(1).returning(|_| {
        Ok(Response::new(ListCampaignsResponse {
            campaigns: vec![],
            next_page_token: "ignored".into(),
        }))
    });

    let (service, _h) = service_with_mock(mock).await;

    let campaigns = service
        .list_campaigns(String::new(), None, None, None, None)
        .await
        .expect("list_campaigns failed");

    assert!(campaigns.is_empty());
}

#[tokio::test]
async fn list_campaigns_propagates_grpc_error() {
    let mut mock = MockCampaignServiceImpl::new();
    mock.expect_list_campaigns()
        .returning(|_| Err(Status::not_found("no such campaign")));

    let (service, _h) = service_with_mock(mock).await;

    let err = service
        .list_campaigns(String::new(), None, None, None, None)
        .await
        .expect_err("expected error");

    assert!(err.to_string().contains("failed to query campaigns"));
}

/// Review summaries are the expensive counterpart to `list_campaigns`: the
/// request must NOT skip report summaries (there is no such flag on this RPC;
/// summaries are the entire point of the call), and must forward the exact
/// campaign ids requested.
#[tokio::test]
async fn review_campaigns_forwards_campaign_ids() {
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
                        num_annotations: 3,
                        num_passed_rules: 1,
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

    let (service, _h) = service_with_mock(mock).await;

    let summaries = service
        .review_campaigns(vec!["campaign1".to_string()], None)
        .await
        .expect("review_campaigns failed");

    let reports = summaries.get("campaign1").expect("missing campaign1");
    assert_eq!(reports.len(), 1);
    assert_eq!(reports[0].num_annotations, 3);
}

#[tokio::test]
async fn review_campaigns_forwards_organization_id() {
    let mut mock = MockCampaignServiceImpl::new();
    mock.expect_get_campaign_report_summaries()
        .withf(|req| req.get_ref().organization_id == "org-1")
        .returning(|_| {
            Ok(Response::new(GetCampaignReportSummariesResponse {
                summaries_by_campaign_id: std::collections::HashMap::new(),
            }))
        });

    let (service, _h) = service_with_mock(mock).await;

    service
        .review_campaigns(vec!["campaign1".to_string()], Some("org-1".to_string()))
        .await
        .expect("review_campaigns failed");
}

#[tokio::test]
async fn review_campaigns_propagates_grpc_error() {
    let mut mock = MockCampaignServiceImpl::new();
    mock.expect_get_campaign_report_summaries()
        .returning(|_| Err(Status::not_found("no such campaign")));

    let (service, _h) = service_with_mock(mock).await;

    let err = service
        .review_campaigns(vec!["missing".to_string()], None)
        .await
        .expect_err("expected error");

    assert!(
        err.to_string()
            .contains("failed to fetch campaign report summaries")
    );
}
