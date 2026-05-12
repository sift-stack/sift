// @generated
/// Generated client implementations.
pub mod external_sync_service_client {
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
    pub struct ExternalSyncServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl ExternalSyncServiceClient<tonic::transport::Channel> {
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
    impl<T> ExternalSyncServiceClient<T>
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
        ) -> ExternalSyncServiceClient<InterceptedService<T, F>>
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
            ExternalSyncServiceClient::new(InterceptedService::new(inner, interceptor))
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
        pub async fn sync_organization(
            &mut self,
            request: impl tonic::IntoRequest<super::SyncOrganizationRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SyncOrganizationResponse>,
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
                "/sift.external_sync.v1.ExternalSyncService/SyncOrganization",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sift.external_sync.v1.ExternalSyncService",
                        "SyncOrganization",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn generate_token(
            &mut self,
            request: impl tonic::IntoRequest<super::GenerateTokenRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GenerateTokenResponse>,
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
                "/sift.external_sync.v1.ExternalSyncService/GenerateToken",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sift.external_sync.v1.ExternalSyncService",
                        "GenerateToken",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_external_sync(
            &mut self,
            request: impl tonic::IntoRequest<super::GetExternalSyncRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetExternalSyncResponse>,
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
                "/sift.external_sync.v1.ExternalSyncService/GetExternalSync",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sift.external_sync.v1.ExternalSyncService",
                        "GetExternalSync",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_external_sync_tokens(
            &mut self,
            request: impl tonic::IntoRequest<super::ListExternalSyncTokensRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListExternalSyncTokensResponse>,
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
                "/sift.external_sync.v1.ExternalSyncService/ListExternalSyncTokens",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sift.external_sync.v1.ExternalSyncService",
                        "ListExternalSyncTokens",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_external_sync_runs(
            &mut self,
            request: impl tonic::IntoRequest<super::ListExternalSyncRunsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListExternalSyncRunsResponse>,
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
                "/sift.external_sync.v1.ExternalSyncService/ListExternalSyncRuns",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sift.external_sync.v1.ExternalSyncService",
                        "ListExternalSyncRuns",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_external_sync_run(
            &mut self,
            request: impl tonic::IntoRequest<super::GetExternalSyncRunRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetExternalSyncRunResponse>,
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
                "/sift.external_sync.v1.ExternalSyncService/GetExternalSyncRun",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sift.external_sync.v1.ExternalSyncService",
                        "GetExternalSyncRun",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_is_org_externally_provisioned(
            &mut self,
            request: impl tonic::IntoRequest<super::GetIsOrgExternallyProvisionedRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetIsOrgExternallyProvisionedResponse>,
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
                "/sift.external_sync.v1.ExternalSyncService/GetIsOrgExternallyProvisioned",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sift.external_sync.v1.ExternalSyncService",
                        "GetIsOrgExternallyProvisioned",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod external_sync_service_server {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::wildcard_imports,
        clippy::let_unit_value,
    )]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with ExternalSyncServiceServer.
    #[async_trait]
    pub trait ExternalSyncService: std::marker::Send + std::marker::Sync + 'static {
        async fn sync_organization(
            &self,
            request: tonic::Request<super::SyncOrganizationRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SyncOrganizationResponse>,
            tonic::Status,
        >;
        async fn generate_token(
            &self,
            request: tonic::Request<super::GenerateTokenRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GenerateTokenResponse>,
            tonic::Status,
        >;
        async fn get_external_sync(
            &self,
            request: tonic::Request<super::GetExternalSyncRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetExternalSyncResponse>,
            tonic::Status,
        >;
        async fn list_external_sync_tokens(
            &self,
            request: tonic::Request<super::ListExternalSyncTokensRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListExternalSyncTokensResponse>,
            tonic::Status,
        >;
        async fn list_external_sync_runs(
            &self,
            request: tonic::Request<super::ListExternalSyncRunsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListExternalSyncRunsResponse>,
            tonic::Status,
        >;
        async fn get_external_sync_run(
            &self,
            request: tonic::Request<super::GetExternalSyncRunRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetExternalSyncRunResponse>,
            tonic::Status,
        >;
        async fn get_is_org_externally_provisioned(
            &self,
            request: tonic::Request<super::GetIsOrgExternallyProvisionedRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetIsOrgExternallyProvisionedResponse>,
            tonic::Status,
        >;
    }
    #[derive(Debug)]
    pub struct ExternalSyncServiceServer<T> {
        inner: Arc<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    impl<T> ExternalSyncServiceServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for ExternalSyncServiceServer<T>
    where
        T: ExternalSyncService,
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
                "/sift.external_sync.v1.ExternalSyncService/SyncOrganization" => {
                    #[allow(non_camel_case_types)]
                    struct SyncOrganizationSvc<T: ExternalSyncService>(pub Arc<T>);
                    impl<
                        T: ExternalSyncService,
                    > tonic::server::UnaryService<super::SyncOrganizationRequest>
                    for SyncOrganizationSvc<T> {
                        type Response = super::SyncOrganizationResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SyncOrganizationRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ExternalSyncService>::sync_organization(
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
                        let method = SyncOrganizationSvc(inner);
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
                "/sift.external_sync.v1.ExternalSyncService/GenerateToken" => {
                    #[allow(non_camel_case_types)]
                    struct GenerateTokenSvc<T: ExternalSyncService>(pub Arc<T>);
                    impl<
                        T: ExternalSyncService,
                    > tonic::server::UnaryService<super::GenerateTokenRequest>
                    for GenerateTokenSvc<T> {
                        type Response = super::GenerateTokenResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GenerateTokenRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ExternalSyncService>::generate_token(&inner, request)
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
                        let method = GenerateTokenSvc(inner);
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
                "/sift.external_sync.v1.ExternalSyncService/GetExternalSync" => {
                    #[allow(non_camel_case_types)]
                    struct GetExternalSyncSvc<T: ExternalSyncService>(pub Arc<T>);
                    impl<
                        T: ExternalSyncService,
                    > tonic::server::UnaryService<super::GetExternalSyncRequest>
                    for GetExternalSyncSvc<T> {
                        type Response = super::GetExternalSyncResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetExternalSyncRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ExternalSyncService>::get_external_sync(
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
                        let method = GetExternalSyncSvc(inner);
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
                "/sift.external_sync.v1.ExternalSyncService/ListExternalSyncTokens" => {
                    #[allow(non_camel_case_types)]
                    struct ListExternalSyncTokensSvc<T: ExternalSyncService>(pub Arc<T>);
                    impl<
                        T: ExternalSyncService,
                    > tonic::server::UnaryService<super::ListExternalSyncTokensRequest>
                    for ListExternalSyncTokensSvc<T> {
                        type Response = super::ListExternalSyncTokensResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListExternalSyncTokensRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ExternalSyncService>::list_external_sync_tokens(
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
                        let method = ListExternalSyncTokensSvc(inner);
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
                "/sift.external_sync.v1.ExternalSyncService/ListExternalSyncRuns" => {
                    #[allow(non_camel_case_types)]
                    struct ListExternalSyncRunsSvc<T: ExternalSyncService>(pub Arc<T>);
                    impl<
                        T: ExternalSyncService,
                    > tonic::server::UnaryService<super::ListExternalSyncRunsRequest>
                    for ListExternalSyncRunsSvc<T> {
                        type Response = super::ListExternalSyncRunsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListExternalSyncRunsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ExternalSyncService>::list_external_sync_runs(
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
                        let method = ListExternalSyncRunsSvc(inner);
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
                "/sift.external_sync.v1.ExternalSyncService/GetExternalSyncRun" => {
                    #[allow(non_camel_case_types)]
                    struct GetExternalSyncRunSvc<T: ExternalSyncService>(pub Arc<T>);
                    impl<
                        T: ExternalSyncService,
                    > tonic::server::UnaryService<super::GetExternalSyncRunRequest>
                    for GetExternalSyncRunSvc<T> {
                        type Response = super::GetExternalSyncRunResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetExternalSyncRunRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ExternalSyncService>::get_external_sync_run(
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
                        let method = GetExternalSyncRunSvc(inner);
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
                "/sift.external_sync.v1.ExternalSyncService/GetIsOrgExternallyProvisioned" => {
                    #[allow(non_camel_case_types)]
                    struct GetIsOrgExternallyProvisionedSvc<T: ExternalSyncService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: ExternalSyncService,
                    > tonic::server::UnaryService<
                        super::GetIsOrgExternallyProvisionedRequest,
                    > for GetIsOrgExternallyProvisionedSvc<T> {
                        type Response = super::GetIsOrgExternallyProvisionedResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::GetIsOrgExternallyProvisionedRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ExternalSyncService>::get_is_org_externally_provisioned(
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
                        let method = GetIsOrgExternallyProvisionedSvc(inner);
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
    impl<T> Clone for ExternalSyncServiceServer<T> {
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
    pub const SERVICE_NAME: &str = "sift.external_sync.v1.ExternalSyncService";
    impl<T> tonic::server::NamedService for ExternalSyncServiceServer<T> {
        const NAME: &'static str = SERVICE_NAME;
    }
}
