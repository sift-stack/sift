// @generated
/// Generated client implementations.
pub mod run_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct RunServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl RunServiceClient<tonic::transport::Channel> {
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
    impl<T> RunServiceClient<T>
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
        ) -> RunServiceClient<InterceptedService<T, F>>
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
            RunServiceClient::new(InterceptedService::new(inner, interceptor))
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
        pub async fn get_run(
            &mut self,
            request: impl tonic::IntoRequest<super::GetRunRequest>,
        ) -> std::result::Result<tonic::Response<super::GetRunResponse>, tonic::Status> {
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
                "/sift.runs.v2.RunService/GetRun",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("sift.runs.v2.RunService", "GetRun"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_runs(
            &mut self,
            request: impl tonic::IntoRequest<super::ListRunsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListRunsResponse>,
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
                "/sift.runs.v2.RunService/ListRuns",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("sift.runs.v2.RunService", "ListRuns"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn create_run(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateRunRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateRunResponse>,
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
                "/sift.runs.v2.RunService/CreateRun",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("sift.runs.v2.RunService", "CreateRun"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn create_adhoc_run(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateAdhocRunRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateAdhocRunResponse>,
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
                "/sift.runs.v2.RunService/CreateAdhocRun",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("sift.runs.v2.RunService", "CreateAdhocRun"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn update_run(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateRunRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateRunResponse>,
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
                "/sift.runs.v2.RunService/UpdateRun",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("sift.runs.v2.RunService", "UpdateRun"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn delete_run(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteRunRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DeleteRunResponse>,
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
                "/sift.runs.v2.RunService/DeleteRun",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("sift.runs.v2.RunService", "DeleteRun"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn stop_run(
            &mut self,
            request: impl tonic::IntoRequest<super::StopRunRequest>,
        ) -> std::result::Result<
            tonic::Response<super::StopRunResponse>,
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
                "/sift.runs.v2.RunService/StopRun",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("sift.runs.v2.RunService", "StopRun"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn create_automatic_run_association_for_assets(
            &mut self,
            request: impl tonic::IntoRequest<
                super::CreateAutomaticRunAssociationForAssetsRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::CreateAutomaticRunAssociationForAssetsResponse>,
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
                "/sift.runs.v2.RunService/CreateAutomaticRunAssociationForAssets",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sift.runs.v2.RunService",
                        "CreateAutomaticRunAssociationForAssets",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod run_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with RunServiceServer.
    #[async_trait]
    pub trait RunService: Send + Sync + 'static {
        async fn get_run(
            &self,
            request: tonic::Request<super::GetRunRequest>,
        ) -> std::result::Result<tonic::Response<super::GetRunResponse>, tonic::Status>;
        async fn list_runs(
            &self,
            request: tonic::Request<super::ListRunsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListRunsResponse>,
            tonic::Status,
        >;
        async fn create_run(
            &self,
            request: tonic::Request<super::CreateRunRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateRunResponse>,
            tonic::Status,
        >;
        async fn create_adhoc_run(
            &self,
            request: tonic::Request<super::CreateAdhocRunRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateAdhocRunResponse>,
            tonic::Status,
        >;
        async fn update_run(
            &self,
            request: tonic::Request<super::UpdateRunRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateRunResponse>,
            tonic::Status,
        >;
        async fn delete_run(
            &self,
            request: tonic::Request<super::DeleteRunRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DeleteRunResponse>,
            tonic::Status,
        >;
        async fn stop_run(
            &self,
            request: tonic::Request<super::StopRunRequest>,
        ) -> std::result::Result<tonic::Response<super::StopRunResponse>, tonic::Status>;
        async fn create_automatic_run_association_for_assets(
            &self,
            request: tonic::Request<super::CreateAutomaticRunAssociationForAssetsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateAutomaticRunAssociationForAssetsResponse>,
            tonic::Status,
        >;
    }
    #[derive(Debug)]
    pub struct RunServiceServer<T: RunService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: RunService> RunServiceServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for RunServiceServer<T>
    where
        T: RunService,
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
                "/sift.runs.v2.RunService/GetRun" => {
                    #[allow(non_camel_case_types)]
                    struct GetRunSvc<T: RunService>(pub Arc<T>);
                    impl<T: RunService> tonic::server::UnaryService<super::GetRunRequest>
                    for GetRunSvc<T> {
                        type Response = super::GetRunResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetRunRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as RunService>::get_run(&inner, request).await
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
                        let method = GetRunSvc(inner);
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
                "/sift.runs.v2.RunService/ListRuns" => {
                    #[allow(non_camel_case_types)]
                    struct ListRunsSvc<T: RunService>(pub Arc<T>);
                    impl<
                        T: RunService,
                    > tonic::server::UnaryService<super::ListRunsRequest>
                    for ListRunsSvc<T> {
                        type Response = super::ListRunsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListRunsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as RunService>::list_runs(&inner, request).await
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
                        let method = ListRunsSvc(inner);
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
                "/sift.runs.v2.RunService/CreateRun" => {
                    #[allow(non_camel_case_types)]
                    struct CreateRunSvc<T: RunService>(pub Arc<T>);
                    impl<
                        T: RunService,
                    > tonic::server::UnaryService<super::CreateRunRequest>
                    for CreateRunSvc<T> {
                        type Response = super::CreateRunResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateRunRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as RunService>::create_run(&inner, request).await
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
                        let method = CreateRunSvc(inner);
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
                "/sift.runs.v2.RunService/CreateAdhocRun" => {
                    #[allow(non_camel_case_types)]
                    struct CreateAdhocRunSvc<T: RunService>(pub Arc<T>);
                    impl<
                        T: RunService,
                    > tonic::server::UnaryService<super::CreateAdhocRunRequest>
                    for CreateAdhocRunSvc<T> {
                        type Response = super::CreateAdhocRunResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateAdhocRunRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as RunService>::create_adhoc_run(&inner, request).await
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
                        let method = CreateAdhocRunSvc(inner);
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
                "/sift.runs.v2.RunService/UpdateRun" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateRunSvc<T: RunService>(pub Arc<T>);
                    impl<
                        T: RunService,
                    > tonic::server::UnaryService<super::UpdateRunRequest>
                    for UpdateRunSvc<T> {
                        type Response = super::UpdateRunResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdateRunRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as RunService>::update_run(&inner, request).await
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
                        let method = UpdateRunSvc(inner);
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
                "/sift.runs.v2.RunService/DeleteRun" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteRunSvc<T: RunService>(pub Arc<T>);
                    impl<
                        T: RunService,
                    > tonic::server::UnaryService<super::DeleteRunRequest>
                    for DeleteRunSvc<T> {
                        type Response = super::DeleteRunResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteRunRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as RunService>::delete_run(&inner, request).await
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
                        let method = DeleteRunSvc(inner);
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
                "/sift.runs.v2.RunService/StopRun" => {
                    #[allow(non_camel_case_types)]
                    struct StopRunSvc<T: RunService>(pub Arc<T>);
                    impl<
                        T: RunService,
                    > tonic::server::UnaryService<super::StopRunRequest>
                    for StopRunSvc<T> {
                        type Response = super::StopRunResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::StopRunRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as RunService>::stop_run(&inner, request).await
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
                        let method = StopRunSvc(inner);
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
                "/sift.runs.v2.RunService/CreateAutomaticRunAssociationForAssets" => {
                    #[allow(non_camel_case_types)]
                    struct CreateAutomaticRunAssociationForAssetsSvc<T: RunService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: RunService,
                    > tonic::server::UnaryService<
                        super::CreateAutomaticRunAssociationForAssetsRequest,
                    > for CreateAutomaticRunAssociationForAssetsSvc<T> {
                        type Response = super::CreateAutomaticRunAssociationForAssetsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::CreateAutomaticRunAssociationForAssetsRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as RunService>::create_automatic_run_association_for_assets(
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
                        let method = CreateAutomaticRunAssociationForAssetsSvc(inner);
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
    impl<T: RunService> Clone for RunServiceServer<T> {
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
    impl<T: RunService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: RunService> tonic::server::NamedService for RunServiceServer<T> {
        const NAME: &'static str = "sift.runs.v2.RunService";
    }
}
