use sift_rs::webhooks::v1::{
    ListWebhooksResponse, Webhook, WebhookHttpHeader, webhook_service_server::WebhookServiceServer,
};
use sift_test_util::{grpc::memory_sift_channel, mock::webhooks::v1::MockWebhookServiceImpl};
use tokio::task::JoinHandle;
use tonic::{Response, transport::Server};

use crate::{
    server::SiftMcpServer,
    tool::common::test_support::{list_params, structured},
};

async fn server_with_mock(mock: MockWebhookServiceImpl) -> (SiftMcpServer, JoinHandle<()>) {
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
async fn list_webhooks_returns_redacted_webhooks() {
    let mut mock = MockWebhookServiceImpl::new();
    mock.expect_list_webhooks()
        .withf(|req| req.get_ref().filter == "is_archived == false")
        .returning(|_| {
            Ok(Response::new(ListWebhooksResponse {
                webhooks: vec![Webhook {
                    webhook_id: "wh1".into(),
                    name: "slack alerts".into(),
                    target_url: "https://hooks.slack.com/services/T000/B000/secret".into(),
                    http_headers: vec![WebhookHttpHeader {
                        name: "Authorization".into(),
                        value: "Bearer secret".into(),
                    }],
                    payload: Some(r#"{"token": "secret"}"#.into()),
                    ..Default::default()
                }],
                next_page_token: String::new(),
            }))
        });

    let (server, _h) = server_with_mock(mock).await;

    let result = server
        .list_webhooks(list_params("is_archived == false", None))
        .await
        .expect("list_webhooks failed");
    let body = structured(result);

    assert_eq!(body["count"], 1);
    assert_eq!(body["has_more"], false);
    let webhook = &body["webhooks"][0];
    assert_eq!(webhook["webhookId"], "wh1");
    assert_eq!(webhook["targetUrl"], "https://slack.com");
    assert_eq!(
        webhook["httpHeaders"],
        serde_json::json!([{ "name": "Authorization" }])
    );
    assert!(webhook.get("payload").is_none());
    assert!(!body.to_string().contains("secret"));
}

#[tokio::test]
async fn list_webhooks_is_available_read_only() {
    let mut mock = MockWebhookServiceImpl::new();
    mock.expect_list_webhooks().times(1).returning(|_| {
        Ok(Response::new(ListWebhooksResponse {
            webhooks: vec![],
            next_page_token: String::new(),
        }))
    });

    let (server, _h) = server_with_mock(mock).await;

    let result = server
        .list_webhooks(list_params("", None))
        .await
        .expect("list_webhooks should not need a write flag");

    assert_eq!(structured(result)["count"], 0);
}
