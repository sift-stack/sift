// @generated
/// Generated client implementations.
pub mod automation_service_client {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::wildcard_imports,
        clippy::let_unit_value,
    )]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct AutomationServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl AutomationServiceClient<tonic::transport::Channel> {
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
    impl<T> AutomationServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::Body>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + std::marker::Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + std::marker::Send,
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
        ) -> AutomationServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::Body>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::Body>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::Body>,
            >>::Error: Into<StdError> + std::marker::Send + std::marker::Sync,
        {
            AutomationServiceClient::new(InterceptedService::new(inner, interceptor))
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
        pub async fn get_automation_trigger(
            &mut self,
            request: impl tonic::IntoRequest<super::GetAutomationTriggerRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetAutomationTriggerResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/sift.automation.v1.AutomationService/GetAutomationTrigger",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sift.automation.v1.AutomationService",
                        "GetAutomationTrigger",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn create_automation_trigger(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateAutomationTriggerRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateAutomationTriggerResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/sift.automation.v1.AutomationService/CreateAutomationTrigger",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sift.automation.v1.AutomationService",
                        "CreateAutomationTrigger",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn update_automation_trigger(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateAutomationTriggerRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateAutomationTriggerResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/sift.automation.v1.AutomationService/UpdateAutomationTrigger",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sift.automation.v1.AutomationService",
                        "UpdateAutomationTrigger",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_automation_triggers(
            &mut self,
            request: impl tonic::IntoRequest<super::ListAutomationTriggersRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListAutomationTriggersResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/sift.automation.v1.AutomationService/ListAutomationTriggers",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sift.automation.v1.AutomationService",
                        "ListAutomationTriggers",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn delete_automation_trigger(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteAutomationTriggerRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DeleteAutomationTriggerResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/sift.automation.v1.AutomationService/DeleteAutomationTrigger",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sift.automation.v1.AutomationService",
                        "DeleteAutomationTrigger",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_automation_triggered_events(
            &mut self,
            request: impl tonic::IntoRequest<super::ListAutomationTriggeredEventsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListAutomationTriggeredEventsResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/sift.automation.v1.AutomationService/ListAutomationTriggeredEvents",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sift.automation.v1.AutomationService",
                        "ListAutomationTriggeredEvents",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod automation_service_server {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::wildcard_imports,
        clippy::let_unit_value,
    )]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with AutomationServiceServer.
    #[async_trait]
    pub trait AutomationService: std::marker::Send + std::marker::Sync + 'static {
        async fn get_automation_trigger(
            &self,
            request: tonic::Request<super::GetAutomationTriggerRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetAutomationTriggerResponse>,
            tonic::Status,
        >;
        async fn create_automation_trigger(
            &self,
            request: tonic::Request<super::CreateAutomationTriggerRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateAutomationTriggerResponse>,
            tonic::Status,
        >;
        async fn update_automation_trigger(
            &self,
            request: tonic::Request<super::UpdateAutomationTriggerRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateAutomationTriggerResponse>,
            tonic::Status,
        >;
        async fn list_automation_triggers(
            &self,
            request: tonic::Request<super::ListAutomationTriggersRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListAutomationTriggersResponse>,
            tonic::Status,
        >;
        async fn delete_automation_trigger(
            &self,
            request: tonic::Request<super::DeleteAutomationTriggerRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DeleteAutomationTriggerResponse>,
            tonic::Status,
        >;
        async fn list_automation_triggered_events(
            &self,
            request: tonic::Request<super::ListAutomationTriggeredEventsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListAutomationTriggeredEventsResponse>,
            tonic::Status,
        >;
    }
    #[derive(Debug)]
    pub struct AutomationServiceServer<T> {
        inner: Arc<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    impl<T> AutomationServiceServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for AutomationServiceServer<T>
    where
        T: AutomationService,
        B: Body + std::marker::Send + 'static,
        B::Error: Into<StdError> + std::marker::Send + 'static,
    {
        type Response = http::Response<tonic::body::Body>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<std::result::Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            match req.uri().path() {
                "/sift.automation.v1.AutomationService/GetAutomationTrigger" => {
                    #[allow(non_camel_case_types)]
                    struct GetAutomationTriggerSvc<T: AutomationService>(pub Arc<T>);
                    impl<
                        T: AutomationService,
                    > tonic::server::UnaryService<super::GetAutomationTriggerRequest>
                    for GetAutomationTriggerSvc<T> {
                        type Response = super::GetAutomationTriggerResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetAutomationTriggerRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as AutomationService>::get_automation_trigger(
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
                        let method = GetAutomationTriggerSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                "/sift.automation.v1.AutomationService/CreateAutomationTrigger" => {
                    #[allow(non_camel_case_types)]
                    struct CreateAutomationTriggerSvc<T: AutomationService>(pub Arc<T>);
                    impl<
                        T: AutomationService,
                    > tonic::server::UnaryService<super::CreateAutomationTriggerRequest>
                    for CreateAutomationTriggerSvc<T> {
                        type Response = super::CreateAutomationTriggerResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::CreateAutomationTriggerRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as AutomationService>::create_automation_trigger(
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
                        let method = CreateAutomationTriggerSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                "/sift.automation.v1.AutomationService/UpdateAutomationTrigger" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateAutomationTriggerSvc<T: AutomationService>(pub Arc<T>);
                    impl<
                        T: AutomationService,
                    > tonic::server::UnaryService<super::UpdateAutomationTriggerRequest>
                    for UpdateAutomationTriggerSvc<T> {
                        type Response = super::UpdateAutomationTriggerResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::UpdateAutomationTriggerRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as AutomationService>::update_automation_trigger(
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
                        let method = UpdateAutomationTriggerSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                "/sift.automation.v1.AutomationService/ListAutomationTriggers" => {
                    #[allow(non_camel_case_types)]
                    struct ListAutomationTriggersSvc<T: AutomationService>(pub Arc<T>);
                    impl<
                        T: AutomationService,
                    > tonic::server::UnaryService<super::ListAutomationTriggersRequest>
                    for ListAutomationTriggersSvc<T> {
                        type Response = super::ListAutomationTriggersResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListAutomationTriggersRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as AutomationService>::list_automation_triggers(
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
                        let method = ListAutomationTriggersSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                "/sift.automation.v1.AutomationService/DeleteAutomationTrigger" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteAutomationTriggerSvc<T: AutomationService>(pub Arc<T>);
                    impl<
                        T: AutomationService,
                    > tonic::server::UnaryService<super::DeleteAutomationTriggerRequest>
                    for DeleteAutomationTriggerSvc<T> {
                        type Response = super::DeleteAutomationTriggerResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::DeleteAutomationTriggerRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as AutomationService>::delete_automation_trigger(
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
                        let method = DeleteAutomationTriggerSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                "/sift.automation.v1.AutomationService/ListAutomationTriggeredEvents" => {
                    #[allow(non_camel_case_types)]
                    struct ListAutomationTriggeredEventsSvc<T: AutomationService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: AutomationService,
                    > tonic::server::UnaryService<
                        super::ListAutomationTriggeredEventsRequest,
                    > for ListAutomationTriggeredEventsSvc<T> {
                        type Response = super::ListAutomationTriggeredEventsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::ListAutomationTriggeredEventsRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as AutomationService>::list_automation_triggered_events(
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
                        let method = ListAutomationTriggeredEventsSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                        let mut response = http::Response::new(
                            tonic::body::Body::default(),
                        );
                        let headers = response.headers_mut();
                        headers
                            .insert(
                                tonic::Status::GRPC_STATUS,
                                (tonic::Code::Unimplemented as i32).into(),
                            );
                        headers
                            .insert(
                                http::header::CONTENT_TYPE,
                                tonic::metadata::GRPC_CONTENT_TYPE,
                            );
                        Ok(response)
                    })
                }
            }
        }
    }
    impl<T> Clone for AutomationServiceServer<T> {
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
    /// Generated gRPC service name
    pub const SERVICE_NAME: &str = "sift.automation.v1.AutomationService";
    impl<T> tonic::server::NamedService for AutomationServiceServer<T> {
        const NAME: &'static str = SERVICE_NAME;
    }
}
