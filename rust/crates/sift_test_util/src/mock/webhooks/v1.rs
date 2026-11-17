use async_trait::async_trait;
use mockall::mock;
use sift_rs::webhooks::v1::{
    BatchCreateWebhookLogsRequest, BatchCreateWebhookLogsResponse, CreateWebhookRequest,
    CreateWebhookResponse, CreateWebhookSignatureKeyRequest, CreateWebhookSignatureKeyResponse,
    GetWebhookRequest, GetWebhookResponse, GetWebhookSignatureKeyRequest,
    GetWebhookSignatureKeyResponse, ListWebhookLogsRequest, ListWebhookLogsResponse,
    ListWebhooksRequest, ListWebhooksResponse, RotateWebhookSignatureKeyRequest,
    RotateWebhookSignatureKeyResponse, TestWebhookRequest, TestWebhookResponse,
    ToggleWebhookSignatureKeyActivationRequest, ToggleWebhookSignatureKeyActivationResponse,
    UpdateWebhookRequest, UpdateWebhookResponse, webhook_service_server::WebhookService,
};
use tonic::{Request, Response, Status};

mock! {
    pub WebhookServiceImpl {}

    #[async_trait]
    impl WebhookService for WebhookServiceImpl {
        async fn get_webhook(
            &self,
            request: Request<GetWebhookRequest>,
        ) -> std::result::Result<
            Response<GetWebhookResponse>,
            Status,
        >;
        async fn create_webhook(
            &self,
            request: Request<CreateWebhookRequest>,
        ) -> std::result::Result<
            Response<CreateWebhookResponse>,
            Status,
        >;
        async fn update_webhook(
            &self,
            request: Request<UpdateWebhookRequest>,
        ) -> std::result::Result<
            Response<UpdateWebhookResponse>,
            Status,
        >;
        async fn list_webhooks(
            &self,
            request: Request<ListWebhooksRequest>,
        ) -> std::result::Result<
            Response<ListWebhooksResponse>,
            Status,
        >;
        async fn test_webhook(
            &self,
            request: Request<TestWebhookRequest>,
        ) -> std::result::Result<
            Response<TestWebhookResponse>,
            Status,
        >;
        async fn create_webhook_signature_key(
            &self,
            request: Request<CreateWebhookSignatureKeyRequest>,
        ) -> std::result::Result<
            Response<CreateWebhookSignatureKeyResponse>,
            Status,
        >;
        async fn get_webhook_signature_key(
            &self,
            request: Request<GetWebhookSignatureKeyRequest>,
        ) -> std::result::Result<
            Response<GetWebhookSignatureKeyResponse>,
            Status,
        >;
        async fn toggle_webhook_signature_key_activation(
            &self,
            request: Request<ToggleWebhookSignatureKeyActivationRequest>,
        ) -> std::result::Result<
            Response<ToggleWebhookSignatureKeyActivationResponse>,
            Status,
        >;
        async fn rotate_webhook_signature_key(
            &self,
            request: Request<RotateWebhookSignatureKeyRequest>,
        ) -> std::result::Result<
            Response<RotateWebhookSignatureKeyResponse>,
            Status,
        >;
        async fn batch_create_webhook_logs(
            &self,
            request: Request<BatchCreateWebhookLogsRequest>,
        ) -> std::result::Result<
            Response<BatchCreateWebhookLogsResponse>,
            Status,
        >;
        async fn list_webhook_logs(
            &self,
            request: Request<ListWebhookLogsRequest>,
        ) -> std::result::Result<
            Response<ListWebhookLogsResponse>,
            Status,
        >;
    }
}
