// @generated
/// Generated client implementations.
pub mod webhook_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct WebhookServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl WebhookServiceClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> WebhookServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> WebhookServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            WebhookServiceClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        pub async fn get_webhook(
            &mut self,
            request: impl tonic::IntoRequest<super::GetWebhookRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetWebhookResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/sift.webhooks.v1.WebhookService/GetWebhook",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("sift.webhooks.v1.WebhookService", "GetWebhook"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn create_webhook(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateWebhookRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateWebhookResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/sift.webhooks.v1.WebhookService/CreateWebhook",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("sift.webhooks.v1.WebhookService", "CreateWebhook"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn update_webhook(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateWebhookRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateWebhookResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/sift.webhooks.v1.WebhookService/UpdateWebhook",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("sift.webhooks.v1.WebhookService", "UpdateWebhook"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_webhooks(
            &mut self,
            request: impl tonic::IntoRequest<super::ListWebhooksRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListWebhooksResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/sift.webhooks.v1.WebhookService/ListWebhooks",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("sift.webhooks.v1.WebhookService", "ListWebhooks"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn test_webhook(
            &mut self,
            request: impl tonic::IntoRequest<super::TestWebhookRequest>,
        ) -> std::result::Result<
            tonic::Response<super::TestWebhookResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/sift.webhooks.v1.WebhookService/TestWebhook",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("sift.webhooks.v1.WebhookService", "TestWebhook"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn create_webhook_signature_key(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateWebhookSignatureKeyRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateWebhookSignatureKeyResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/sift.webhooks.v1.WebhookService/CreateWebhookSignatureKey",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sift.webhooks.v1.WebhookService",
                        "CreateWebhookSignatureKey",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_webhook_signature_key(
            &mut self,
            request: impl tonic::IntoRequest<super::GetWebhookSignatureKeyRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetWebhookSignatureKeyResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/sift.webhooks.v1.WebhookService/GetWebhookSignatureKey",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sift.webhooks.v1.WebhookService",
                        "GetWebhookSignatureKey",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn toggle_webhook_signature_key_activation(
            &mut self,
            request: impl tonic::IntoRequest<
                super::ToggleWebhookSignatureKeyActivationRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::ToggleWebhookSignatureKeyActivationResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/sift.webhooks.v1.WebhookService/ToggleWebhookSignatureKeyActivation",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sift.webhooks.v1.WebhookService",
                        "ToggleWebhookSignatureKeyActivation",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn rotate_webhook_signature_key(
            &mut self,
            request: impl tonic::IntoRequest<super::RotateWebhookSignatureKeyRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RotateWebhookSignatureKeyResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/sift.webhooks.v1.WebhookService/RotateWebhookSignatureKey",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sift.webhooks.v1.WebhookService",
                        "RotateWebhookSignatureKey",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn batch_create_webhook_logs(
            &mut self,
            request: impl tonic::IntoRequest<super::BatchCreateWebhookLogsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::BatchCreateWebhookLogsResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/sift.webhooks.v1.WebhookService/BatchCreateWebhookLogs",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sift.webhooks.v1.WebhookService",
                        "BatchCreateWebhookLogs",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_webhook_logs(
            &mut self,
            request: impl tonic::IntoRequest<super::ListWebhookLogsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListWebhookLogsResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/sift.webhooks.v1.WebhookService/ListWebhookLogs",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("sift.webhooks.v1.WebhookService", "ListWebhookLogs"),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod webhook_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with WebhookServiceServer.
    #[async_trait]
    pub trait WebhookService: Send + Sync + 'static {
        async fn get_webhook(
            &self,
            request: tonic::Request<super::GetWebhookRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetWebhookResponse>,
            tonic::Status,
        >;
        async fn create_webhook(
            &self,
            request: tonic::Request<super::CreateWebhookRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateWebhookResponse>,
            tonic::Status,
        >;
        async fn update_webhook(
            &self,
            request: tonic::Request<super::UpdateWebhookRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateWebhookResponse>,
            tonic::Status,
        >;
        async fn list_webhooks(
            &self,
            request: tonic::Request<super::ListWebhooksRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListWebhooksResponse>,
            tonic::Status,
        >;
        async fn test_webhook(
            &self,
            request: tonic::Request<super::TestWebhookRequest>,
        ) -> std::result::Result<
            tonic::Response<super::TestWebhookResponse>,
            tonic::Status,
        >;
        async fn create_webhook_signature_key(
            &self,
            request: tonic::Request<super::CreateWebhookSignatureKeyRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateWebhookSignatureKeyResponse>,
            tonic::Status,
        >;
        async fn get_webhook_signature_key(
            &self,
            request: tonic::Request<super::GetWebhookSignatureKeyRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetWebhookSignatureKeyResponse>,
            tonic::Status,
        >;
        async fn toggle_webhook_signature_key_activation(
            &self,
            request: tonic::Request<super::ToggleWebhookSignatureKeyActivationRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ToggleWebhookSignatureKeyActivationResponse>,
            tonic::Status,
        >;
        async fn rotate_webhook_signature_key(
            &self,
            request: tonic::Request<super::RotateWebhookSignatureKeyRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RotateWebhookSignatureKeyResponse>,
            tonic::Status,
        >;
        async fn batch_create_webhook_logs(
            &self,
            request: tonic::Request<super::BatchCreateWebhookLogsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::BatchCreateWebhookLogsResponse>,
            tonic::Status,
        >;
        async fn list_webhook_logs(
            &self,
            request: tonic::Request<super::ListWebhookLogsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListWebhookLogsResponse>,
            tonic::Status,
        >;
    }
    #[derive(Debug)]
    pub struct WebhookServiceServer<T: WebhookService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: WebhookService> WebhookServiceServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
                max_decoding_message_size: None,
                max_encoding_message_size: None,
            }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
        /// Enable decompressing requests with the given encoding.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.accept_compression_encodings.enable(encoding);
            self
        }
        /// Compress responses with the given encoding, if the client supports it.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.send_compression_encodings.enable(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.max_decoding_message_size = Some(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.max_encoding_message_size = Some(limit);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for WebhookServiceServer<T>
    where
        T: WebhookService,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<std::result::Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/sift.webhooks.v1.WebhookService/GetWebhook" => {
                    #[allow(non_camel_case_types)]
                    struct GetWebhookSvc<T: WebhookService>(pub Arc<T>);
                    impl<
                        T: WebhookService,
                    > tonic::server::UnaryService<super::GetWebhookRequest>
                    for GetWebhookSvc<T> {
                        type Response = super::GetWebhookResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetWebhookRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as WebhookService>::get_webhook(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetWebhookSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/sift.webhooks.v1.WebhookService/CreateWebhook" => {
                    #[allow(non_camel_case_types)]
                    struct CreateWebhookSvc<T: WebhookService>(pub Arc<T>);
                    impl<
                        T: WebhookService,
                    > tonic::server::UnaryService<super::CreateWebhookRequest>
                    for CreateWebhookSvc<T> {
                        type Response = super::CreateWebhookResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateWebhookRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as WebhookService>::create_webhook(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = CreateWebhookSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/sift.webhooks.v1.WebhookService/UpdateWebhook" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateWebhookSvc<T: WebhookService>(pub Arc<T>);
                    impl<
                        T: WebhookService,
                    > tonic::server::UnaryService<super::UpdateWebhookRequest>
                    for UpdateWebhookSvc<T> {
                        type Response = super::UpdateWebhookResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdateWebhookRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as WebhookService>::update_webhook(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = UpdateWebhookSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/sift.webhooks.v1.WebhookService/ListWebhooks" => {
                    #[allow(non_camel_case_types)]
                    struct ListWebhooksSvc<T: WebhookService>(pub Arc<T>);
                    impl<
                        T: WebhookService,
                    > tonic::server::UnaryService<super::ListWebhooksRequest>
                    for ListWebhooksSvc<T> {
                        type Response = super::ListWebhooksResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListWebhooksRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as WebhookService>::list_webhooks(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ListWebhooksSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/sift.webhooks.v1.WebhookService/TestWebhook" => {
                    #[allow(non_camel_case_types)]
                    struct TestWebhookSvc<T: WebhookService>(pub Arc<T>);
                    impl<
                        T: WebhookService,
                    > tonic::server::UnaryService<super::TestWebhookRequest>
                    for TestWebhookSvc<T> {
                        type Response = super::TestWebhookResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::TestWebhookRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as WebhookService>::test_webhook(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = TestWebhookSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/sift.webhooks.v1.WebhookService/CreateWebhookSignatureKey" => {
                    #[allow(non_camel_case_types)]
                    struct CreateWebhookSignatureKeySvc<T: WebhookService>(pub Arc<T>);
                    impl<
                        T: WebhookService,
                    > tonic::server::UnaryService<
                        super::CreateWebhookSignatureKeyRequest,
                    > for CreateWebhookSignatureKeySvc<T> {
                        type Response = super::CreateWebhookSignatureKeyResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::CreateWebhookSignatureKeyRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as WebhookService>::create_webhook_signature_key(
                                        &inner,
                                        request,
                                    )
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = CreateWebhookSignatureKeySvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/sift.webhooks.v1.WebhookService/GetWebhookSignatureKey" => {
                    #[allow(non_camel_case_types)]
                    struct GetWebhookSignatureKeySvc<T: WebhookService>(pub Arc<T>);
                    impl<
                        T: WebhookService,
                    > tonic::server::UnaryService<super::GetWebhookSignatureKeyRequest>
                    for GetWebhookSignatureKeySvc<T> {
                        type Response = super::GetWebhookSignatureKeyResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetWebhookSignatureKeyRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as WebhookService>::get_webhook_signature_key(
                                        &inner,
                                        request,
                                    )
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetWebhookSignatureKeySvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/sift.webhooks.v1.WebhookService/ToggleWebhookSignatureKeyActivation" => {
                    #[allow(non_camel_case_types)]
                    struct ToggleWebhookSignatureKeyActivationSvc<T: WebhookService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: WebhookService,
                    > tonic::server::UnaryService<
                        super::ToggleWebhookSignatureKeyActivationRequest,
                    > for ToggleWebhookSignatureKeyActivationSvc<T> {
                        type Response = super::ToggleWebhookSignatureKeyActivationResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::ToggleWebhookSignatureKeyActivationRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as WebhookService>::toggle_webhook_signature_key_activation(
                                        &inner,
                                        request,
                                    )
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ToggleWebhookSignatureKeyActivationSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/sift.webhooks.v1.WebhookService/RotateWebhookSignatureKey" => {
                    #[allow(non_camel_case_types)]
                    struct RotateWebhookSignatureKeySvc<T: WebhookService>(pub Arc<T>);
                    impl<
                        T: WebhookService,
                    > tonic::server::UnaryService<
                        super::RotateWebhookSignatureKeyRequest,
                    > for RotateWebhookSignatureKeySvc<T> {
                        type Response = super::RotateWebhookSignatureKeyResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::RotateWebhookSignatureKeyRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as WebhookService>::rotate_webhook_signature_key(
                                        &inner,
                                        request,
                                    )
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = RotateWebhookSignatureKeySvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/sift.webhooks.v1.WebhookService/BatchCreateWebhookLogs" => {
                    #[allow(non_camel_case_types)]
                    struct BatchCreateWebhookLogsSvc<T: WebhookService>(pub Arc<T>);
                    impl<
                        T: WebhookService,
                    > tonic::server::UnaryService<super::BatchCreateWebhookLogsRequest>
                    for BatchCreateWebhookLogsSvc<T> {
                        type Response = super::BatchCreateWebhookLogsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::BatchCreateWebhookLogsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as WebhookService>::batch_create_webhook_logs(
                                        &inner,
                                        request,
                                    )
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = BatchCreateWebhookLogsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/sift.webhooks.v1.WebhookService/ListWebhookLogs" => {
                    #[allow(non_camel_case_types)]
                    struct ListWebhookLogsSvc<T: WebhookService>(pub Arc<T>);
                    impl<
                        T: WebhookService,
                    > tonic::server::UnaryService<super::ListWebhookLogsRequest>
                    for ListWebhookLogsSvc<T> {
                        type Response = super::ListWebhookLogsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListWebhookLogsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as WebhookService>::list_webhook_logs(&inner, request)
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ListWebhookLogsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => {
                    Box::pin(async move {
                        Ok(
                            http::Response::builder()
                                .status(200)
                                .header("grpc-status", "12")
                                .header("content-type", "application/grpc")
                                .body(empty_body())
                                .unwrap(),
                        )
                    })
                }
            }
        }
    }
    impl<T: WebhookService> Clone for WebhookServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
                max_decoding_message_size: self.max_decoding_message_size,
                max_encoding_message_size: self.max_encoding_message_size,
            }
        }
    }
    impl<T: WebhookService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: WebhookService> tonic::server::NamedService for WebhookServiceServer<T> {
        const NAME: &'static str = "sift.webhooks.v1.WebhookService";
    }
}
