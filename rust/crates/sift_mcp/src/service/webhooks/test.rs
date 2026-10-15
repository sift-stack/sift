use sift_rs::webhooks::v1::{
    ListWebhooksResponse, Webhook, WebhookHttpHeader, webhook_service_server::WebhookServiceServer,
};
use sift_test_util::{grpc::memory_sift_channel, mock::webhooks::v1::MockWebhookServiceImpl};
use tokio::task::JoinHandle;
use tonic::{Response, Status, transport::Server};

use super::WebhookService;
use crate::service::common::{DEFAULT_LIMIT, PAGE_SIZE};

async fn service_with_mock(mock: MockWebhookServiceImpl) -> (WebhookService, JoinHandle<()>) {
    let (client, server) = tokio::io::duplex(1024);
    let channel = memory_sift_channel(client).await;

    let handle = tokio::spawn(async move {
        Server::builder()
            .add_service(WebhookServiceServer::new(mock))
            .serve_with_incoming(tokio_stream::once(Ok::<_, std::io::Error>(server)))
            .await
            .unwrap();
    });

    (
        WebhookService::new(channel, crate::policy::RetryPolicy::default()),
        handle,
    )
}

fn webhook(id: &str) -> Webhook {
    Webhook {
        webhook_id: id.into(),
        name: format!("{id} name"),
        ..Default::default()
    }
}

#[tokio::test]
async fn list_webhooks_returns_single_page() {
    let mut mock = MockWebhookServiceImpl::new();
    mock.expect_list_webhooks()
        .withf(|req| {
            let req = req.get_ref();
            req.filter == "is_archived == false" && req.order_by == "created_date desc"
        })
        .returning(|_| {
            Ok(Response::new(ListWebhooksResponse {
                webhooks: vec![webhook("wh1")],
                next_page_token: String::new(),
            }))
        });

    let (service, _h) = service_with_mock(mock).await;

    let page = service
        .list_webhooks(
            "is_archived == false".to_string(),
            Some("created_date desc".to_string()),
            None,
        )
        .await
        .expect("list_webhooks failed");

    assert_eq!(page.items.len(), 1);
    assert_eq!(page.items[0].webhook_id, "wh1");
    assert!(!page.has_more);
}

#[tokio::test]
async fn list_webhooks_paginates_until_token_empty() {
    let mut mock = MockWebhookServiceImpl::new();
    mock.expect_list_webhooks().times(2).returning(|req| {
        let req = req.into_inner();
        assert_eq!(req.page_size, DEFAULT_LIMIT);
        let (webhooks, next) = match req.page_token.as_str() {
            "" => (vec![webhook("wh1")], "page-2".to_string()),
            "page-2" => (vec![webhook("wh2")], String::new()),
            other => return Err(Status::invalid_argument(format!("bad token: {other}"))),
        };
        Ok(Response::new(ListWebhooksResponse {
            webhooks,
            next_page_token: next,
        }))
    });

    let (service, _h) = service_with_mock(mock).await;

    let page = service
        .list_webhooks(String::new(), None, None)
        .await
        .expect("list_webhooks failed");

    let ids: Vec<&str> = page.items.iter().map(|w| w.webhook_id.as_str()).collect();
    assert_eq!(ids, vec!["wh1", "wh2"]);
    assert!(!page.has_more);
}

#[tokio::test]
async fn list_webhooks_truncates_to_limit_and_reports_more() {
    let mut mock = MockWebhookServiceImpl::new();
    mock.expect_list_webhooks().times(1).returning(|req| {
        assert_eq!(req.get_ref().page_size, 2);
        Ok(Response::new(ListWebhooksResponse {
            webhooks: vec![webhook("wh1"), webhook("wh2")],
            next_page_token: "page-2".into(),
        }))
    });

    let (service, _h) = service_with_mock(mock).await;

    let page = service
        .list_webhooks(String::new(), None, Some(2))
        .await
        .expect("list_webhooks failed");

    assert_eq!(page.items.len(), 2);
    assert!(page.has_more);
}

#[tokio::test]
async fn list_webhooks_breaks_on_empty_page() {
    let mut mock = MockWebhookServiceImpl::new();
    mock.expect_list_webhooks().times(1).returning(|_| {
        Ok(Response::new(ListWebhooksResponse {
            webhooks: vec![],
            next_page_token: "ignored".into(),
        }))
    });

    let (service, _h) = service_with_mock(mock).await;

    let page = service
        .list_webhooks(String::new(), None, None)
        .await
        .expect("list_webhooks failed");

    assert!(page.items.is_empty());
    assert!(!page.has_more);
}

#[tokio::test]
async fn list_webhooks_propagates_grpc_error() {
    let mut mock = MockWebhookServiceImpl::new();
    mock.expect_list_webhooks()
        .returning(|_| Err(Status::permission_denied("no access")));

    let (service, _h) = service_with_mock(mock).await;

    let err = service
        .list_webhooks(String::new(), None, None)
        .await
        .expect_err("expected error");

    assert!(err.to_string().contains("failed to query webhooks"));
}

#[tokio::test]
async fn list_webhooks_withholds_credentials() {
    let mut mock = MockWebhookServiceImpl::new();
    mock.expect_list_webhooks().returning(|_| {
        Ok(Response::new(ListWebhooksResponse {
            webhooks: vec![
                Webhook {
                    webhook_id: "slack".into(),
                    target_url: "https://hooks.slack.com/services/T000/B000/secret".into(),
                    http_headers: vec![WebhookHttpHeader {
                        name: "Authorization".into(),
                        value: "Bearer secret".into(),
                    }],
                    ..Default::default()
                },
                Webhook {
                    webhook_id: "pagerduty".into(),
                    target_url: "https://events.pagerduty.com/v2/enqueue".into(),
                    payload: Some(r#"{"routing_key": "secret"}"#.into()),
                    ..Default::default()
                },
            ],
            next_page_token: String::new(),
        }))
    });

    let (service, _h) = service_with_mock(mock).await;

    let page = service
        .list_webhooks(String::new(), None, None)
        .await
        .expect("list_webhooks failed");

    assert_eq!(page.items[0].target_url, "https://slack.com");
    assert_eq!(page.items[0].http_headers[0].name, "Authorization");
    assert_eq!(page.items[0].http_headers[0].value, "");
    assert_eq!(page.items[1].target_url, "https://pagerduty.com");
    assert_eq!(page.items[1].payload, None);

    let body = serde_json::to_string(&page.items).unwrap();
    assert!(!body.contains("secret"), "credentials leaked: {body}");
}

#[test]
fn redact_target_keeps_only_scheme_and_domain() {
    let cases = [
        (
            "https://eo3k9x2secret.m.pipedream.net/",
            "https://pipedream.net",
        ),
        (
            "https://secretid.lambda-url.us-east-1.on.aws/",
            "https://on.aws",
        ),
        (
            "https://user:secret@example.com:8443/hook?token=secret",
            "https://example.com",
        ),
        ("http://10.0.0.5:8080/hook/secret", "http://10.0.0.5"),
        ("http://[::1]:8080/hook/secret", "http://[::1]"),
        ("https://localhost/secret", "https://localhost"),
        ("not a url secret", ""),
        ("mailto:secret@example.com", ""),
    ];
    for (input, expected) in cases {
        assert_eq!(super::redact_target(input), expected, "{input}");
    }
}

#[tokio::test]
async fn get_webhooks_filters_by_id_and_includes_archived() {
    let mut mock = MockWebhookServiceImpl::new();
    mock.expect_list_webhooks()
        .withf(|req| {
            let req = req.get_ref();
            req.filter == r#"webhook_id in ["wh1", "wh\"2"]"# && req.page_size == PAGE_SIZE
        })
        .returning(|_| {
            Ok(Response::new(ListWebhooksResponse {
                webhooks: vec![webhook("wh1")],
                next_page_token: String::new(),
            }))
        });

    let (service, _h) = service_with_mock(mock).await;

    let webhooks = service
        .get_webhooks(&["wh1".to_string(), "wh\"2".to_string()])
        .await
        .expect("get_webhooks failed");

    assert_eq!(webhooks.len(), 1);
    assert_eq!(webhooks[0].webhook_id, "wh1");
}

#[tokio::test]
async fn get_webhooks_skips_the_call_for_no_ids() {
    let mut mock = MockWebhookServiceImpl::new();
    mock.expect_list_webhooks().times(0);

    let (service, _h) = service_with_mock(mock).await;

    let webhooks = service
        .get_webhooks(&[])
        .await
        .expect("get_webhooks failed");

    assert!(webhooks.is_empty());
}

#[tokio::test]
async fn get_webhooks_fails_when_the_lookup_is_capped() {
    let mut mock = MockWebhookServiceImpl::new();
    mock.expect_list_webhooks().times(1).returning(|req| {
        let page_size = req.get_ref().page_size as usize;
        Ok(Response::new(ListWebhooksResponse {
            webhooks: (0..page_size).map(|i| webhook(&format!("wh{i}"))).collect(),
            next_page_token: "more".into(),
        }))
    });

    let (service, _h) = service_with_mock(mock).await;

    let err = service
        .get_webhooks(&["wh0".to_string()])
        .await
        .expect_err("expected error");

    assert!(err.to_string().contains("incomplete"));
}
