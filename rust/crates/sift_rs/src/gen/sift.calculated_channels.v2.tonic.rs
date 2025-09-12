// @generated
/// Generated client implementations.
pub mod calculated_channel_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct CalculatedChannelServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl CalculatedChannelServiceClient<tonic::transport::Channel> {
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
    impl<T> CalculatedChannelServiceClient<T>
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
        ) -> CalculatedChannelServiceClient<InterceptedService<T, F>>
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
            CalculatedChannelServiceClient::new(
                InterceptedService::new(inner, interceptor),
            )
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
        pub async fn get_calculated_channel(
            &mut self,
            request: impl tonic::IntoRequest<super::GetCalculatedChannelRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetCalculatedChannelResponse>,
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
                "/sift.calculated_channels.v2.CalculatedChannelService/GetCalculatedChannel",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sift.calculated_channels.v2.CalculatedChannelService",
                        "GetCalculatedChannel",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn create_calculated_channel(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateCalculatedChannelRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateCalculatedChannelResponse>,
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
                "/sift.calculated_channels.v2.CalculatedChannelService/CreateCalculatedChannel",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sift.calculated_channels.v2.CalculatedChannelService",
                        "CreateCalculatedChannel",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_calculated_channels(
            &mut self,
            request: impl tonic::IntoRequest<super::ListCalculatedChannelsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListCalculatedChannelsResponse>,
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
                "/sift.calculated_channels.v2.CalculatedChannelService/ListCalculatedChannels",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sift.calculated_channels.v2.CalculatedChannelService",
                        "ListCalculatedChannels",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn update_calculated_channel(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateCalculatedChannelRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateCalculatedChannelResponse>,
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
                "/sift.calculated_channels.v2.CalculatedChannelService/UpdateCalculatedChannel",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sift.calculated_channels.v2.CalculatedChannelService",
                        "UpdateCalculatedChannel",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_calculated_channel_versions(
            &mut self,
            request: impl tonic::IntoRequest<super::ListCalculatedChannelVersionsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListCalculatedChannelVersionsResponse>,
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
                "/sift.calculated_channels.v2.CalculatedChannelService/ListCalculatedChannelVersions",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sift.calculated_channels.v2.CalculatedChannelService",
                        "ListCalculatedChannelVersions",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn resolve_calculated_channel(
            &mut self,
            request: impl tonic::IntoRequest<super::ResolveCalculatedChannelRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ResolveCalculatedChannelResponse>,
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
                "/sift.calculated_channels.v2.CalculatedChannelService/ResolveCalculatedChannel",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sift.calculated_channels.v2.CalculatedChannelService",
                        "ResolveCalculatedChannel",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn batch_resolve_calculated_channels(
            &mut self,
            request: impl tonic::IntoRequest<
                super::BatchResolveCalculatedChannelsRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::BatchResolveCalculatedChannelsResponse>,
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
                "/sift.calculated_channels.v2.CalculatedChannelService/BatchResolveCalculatedChannels",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sift.calculated_channels.v2.CalculatedChannelService",
                        "BatchResolveCalculatedChannels",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_resolved_calculated_channels(
            &mut self,
            request: impl tonic::IntoRequest<
                super::ListResolvedCalculatedChannelsRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::ListResolvedCalculatedChannelsResponse>,
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
                "/sift.calculated_channels.v2.CalculatedChannelService/ListResolvedCalculatedChannels",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sift.calculated_channels.v2.CalculatedChannelService",
                        "ListResolvedCalculatedChannels",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod calculated_channel_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with CalculatedChannelServiceServer.
    #[async_trait]
    pub trait CalculatedChannelService: Send + Sync + 'static {
        async fn get_calculated_channel(
            &self,
            request: tonic::Request<super::GetCalculatedChannelRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetCalculatedChannelResponse>,
            tonic::Status,
        >;
        async fn create_calculated_channel(
            &self,
            request: tonic::Request<super::CreateCalculatedChannelRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateCalculatedChannelResponse>,
            tonic::Status,
        >;
        async fn list_calculated_channels(
            &self,
            request: tonic::Request<super::ListCalculatedChannelsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListCalculatedChannelsResponse>,
            tonic::Status,
        >;
        async fn update_calculated_channel(
            &self,
            request: tonic::Request<super::UpdateCalculatedChannelRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateCalculatedChannelResponse>,
            tonic::Status,
        >;
        async fn list_calculated_channel_versions(
            &self,
            request: tonic::Request<super::ListCalculatedChannelVersionsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListCalculatedChannelVersionsResponse>,
            tonic::Status,
        >;
        async fn resolve_calculated_channel(
            &self,
            request: tonic::Request<super::ResolveCalculatedChannelRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ResolveCalculatedChannelResponse>,
            tonic::Status,
        >;
        async fn batch_resolve_calculated_channels(
            &self,
            request: tonic::Request<super::BatchResolveCalculatedChannelsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::BatchResolveCalculatedChannelsResponse>,
            tonic::Status,
        >;
        async fn list_resolved_calculated_channels(
            &self,
            request: tonic::Request<super::ListResolvedCalculatedChannelsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListResolvedCalculatedChannelsResponse>,
            tonic::Status,
        >;
    }
    #[derive(Debug)]
    pub struct CalculatedChannelServiceServer<T: CalculatedChannelService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: CalculatedChannelService> CalculatedChannelServiceServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>>
    for CalculatedChannelServiceServer<T>
    where
        T: CalculatedChannelService,
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
                "/sift.calculated_channels.v2.CalculatedChannelService/GetCalculatedChannel" => {
                    #[allow(non_camel_case_types)]
                    struct GetCalculatedChannelSvc<T: CalculatedChannelService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: CalculatedChannelService,
                    > tonic::server::UnaryService<super::GetCalculatedChannelRequest>
                    for GetCalculatedChannelSvc<T> {
                        type Response = super::GetCalculatedChannelResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetCalculatedChannelRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as CalculatedChannelService>::get_calculated_channel(
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
                        let method = GetCalculatedChannelSvc(inner);
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
                "/sift.calculated_channels.v2.CalculatedChannelService/CreateCalculatedChannel" => {
                    #[allow(non_camel_case_types)]
                    struct CreateCalculatedChannelSvc<T: CalculatedChannelService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: CalculatedChannelService,
                    > tonic::server::UnaryService<super::CreateCalculatedChannelRequest>
                    for CreateCalculatedChannelSvc<T> {
                        type Response = super::CreateCalculatedChannelResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::CreateCalculatedChannelRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as CalculatedChannelService>::create_calculated_channel(
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
                        let method = CreateCalculatedChannelSvc(inner);
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
                "/sift.calculated_channels.v2.CalculatedChannelService/ListCalculatedChannels" => {
                    #[allow(non_camel_case_types)]
                    struct ListCalculatedChannelsSvc<T: CalculatedChannelService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: CalculatedChannelService,
                    > tonic::server::UnaryService<super::ListCalculatedChannelsRequest>
                    for ListCalculatedChannelsSvc<T> {
                        type Response = super::ListCalculatedChannelsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListCalculatedChannelsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as CalculatedChannelService>::list_calculated_channels(
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
                        let method = ListCalculatedChannelsSvc(inner);
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
                "/sift.calculated_channels.v2.CalculatedChannelService/UpdateCalculatedChannel" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateCalculatedChannelSvc<T: CalculatedChannelService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: CalculatedChannelService,
                    > tonic::server::UnaryService<super::UpdateCalculatedChannelRequest>
                    for UpdateCalculatedChannelSvc<T> {
                        type Response = super::UpdateCalculatedChannelResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::UpdateCalculatedChannelRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as CalculatedChannelService>::update_calculated_channel(
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
                        let method = UpdateCalculatedChannelSvc(inner);
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
                "/sift.calculated_channels.v2.CalculatedChannelService/ListCalculatedChannelVersions" => {
                    #[allow(non_camel_case_types)]
                    struct ListCalculatedChannelVersionsSvc<T: CalculatedChannelService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: CalculatedChannelService,
                    > tonic::server::UnaryService<
                        super::ListCalculatedChannelVersionsRequest,
                    > for ListCalculatedChannelVersionsSvc<T> {
                        type Response = super::ListCalculatedChannelVersionsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::ListCalculatedChannelVersionsRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as CalculatedChannelService>::list_calculated_channel_versions(
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
                        let method = ListCalculatedChannelVersionsSvc(inner);
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
                "/sift.calculated_channels.v2.CalculatedChannelService/ResolveCalculatedChannel" => {
                    #[allow(non_camel_case_types)]
                    struct ResolveCalculatedChannelSvc<T: CalculatedChannelService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: CalculatedChannelService,
                    > tonic::server::UnaryService<super::ResolveCalculatedChannelRequest>
                    for ResolveCalculatedChannelSvc<T> {
                        type Response = super::ResolveCalculatedChannelResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::ResolveCalculatedChannelRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as CalculatedChannelService>::resolve_calculated_channel(
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
                        let method = ResolveCalculatedChannelSvc(inner);
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
                "/sift.calculated_channels.v2.CalculatedChannelService/BatchResolveCalculatedChannels" => {
                    #[allow(non_camel_case_types)]
                    struct BatchResolveCalculatedChannelsSvc<
                        T: CalculatedChannelService,
                    >(
                        pub Arc<T>,
                    );
                    impl<
                        T: CalculatedChannelService,
                    > tonic::server::UnaryService<
                        super::BatchResolveCalculatedChannelsRequest,
                    > for BatchResolveCalculatedChannelsSvc<T> {
                        type Response = super::BatchResolveCalculatedChannelsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::BatchResolveCalculatedChannelsRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as CalculatedChannelService>::batch_resolve_calculated_channels(
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
                        let method = BatchResolveCalculatedChannelsSvc(inner);
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
                "/sift.calculated_channels.v2.CalculatedChannelService/ListResolvedCalculatedChannels" => {
                    #[allow(non_camel_case_types)]
                    struct ListResolvedCalculatedChannelsSvc<
                        T: CalculatedChannelService,
                    >(
                        pub Arc<T>,
                    );
                    impl<
                        T: CalculatedChannelService,
                    > tonic::server::UnaryService<
                        super::ListResolvedCalculatedChannelsRequest,
                    > for ListResolvedCalculatedChannelsSvc<T> {
                        type Response = super::ListResolvedCalculatedChannelsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::ListResolvedCalculatedChannelsRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as CalculatedChannelService>::list_resolved_calculated_channels(
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
                        let method = ListResolvedCalculatedChannelsSvc(inner);
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
    impl<T: CalculatedChannelService> Clone for CalculatedChannelServiceServer<T> {
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
    impl<T: CalculatedChannelService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: CalculatedChannelService> tonic::server::NamedService
    for CalculatedChannelServiceServer<T> {
        const NAME: &'static str = "sift.calculated_channels.v2.CalculatedChannelService";
    }
}
