// @generated
/// Generated client implementations.
pub mod metadata_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct MetadataServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl MetadataServiceClient<tonic::transport::Channel> {
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
    impl<T> MetadataServiceClient<T>
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
        ) -> MetadataServiceClient<InterceptedService<T, F>>
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
            MetadataServiceClient::new(InterceptedService::new(inner, interceptor))
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
        pub async fn create_metadata_key(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateMetadataKeyRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateMetadataKeyResponse>,
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
                "/sift.metadata.v1.MetadataService/CreateMetadataKey",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sift.metadata.v1.MetadataService",
                        "CreateMetadataKey",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn create_metadata_value(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateMetadataValueRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateMetadataValueResponse>,
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
                "/sift.metadata.v1.MetadataService/CreateMetadataValue",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sift.metadata.v1.MetadataService",
                        "CreateMetadataValue",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_metadata_keys(
            &mut self,
            request: impl tonic::IntoRequest<super::ListMetadataKeysRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListMetadataKeysResponse>,
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
                "/sift.metadata.v1.MetadataService/ListMetadataKeys",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sift.metadata.v1.MetadataService",
                        "ListMetadataKeys",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_metadata_values(
            &mut self,
            request: impl tonic::IntoRequest<super::ListMetadataValuesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListMetadataValuesResponse>,
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
                "/sift.metadata.v1.MetadataService/ListMetadataValues",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sift.metadata.v1.MetadataService",
                        "ListMetadataValues",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn archive_metadata_keys(
            &mut self,
            request: impl tonic::IntoRequest<super::ArchiveMetadataKeysRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ArchiveMetadataKeysResponse>,
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
                "/sift.metadata.v1.MetadataService/ArchiveMetadataKeys",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sift.metadata.v1.MetadataService",
                        "ArchiveMetadataKeys",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn archive_metadata_values(
            &mut self,
            request: impl tonic::IntoRequest<super::ArchiveMetadataValuesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ArchiveMetadataValuesResponse>,
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
                "/sift.metadata.v1.MetadataService/ArchiveMetadataValues",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sift.metadata.v1.MetadataService",
                        "ArchiveMetadataValues",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn unarchive_metadata_keys(
            &mut self,
            request: impl tonic::IntoRequest<super::UnarchiveMetadataKeysRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UnarchiveMetadataKeysResponse>,
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
                "/sift.metadata.v1.MetadataService/UnarchiveMetadataKeys",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sift.metadata.v1.MetadataService",
                        "UnarchiveMetadataKeys",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn unarchive_metadata_values(
            &mut self,
            request: impl tonic::IntoRequest<super::UnarchiveMetadataValuesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UnarchiveMetadataValuesResponse>,
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
                "/sift.metadata.v1.MetadataService/UnarchiveMetadataValues",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sift.metadata.v1.MetadataService",
                        "UnarchiveMetadataValues",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn delete_metadata_keys(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteMetadataKeysRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DeleteMetadataKeysResponse>,
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
                "/sift.metadata.v1.MetadataService/DeleteMetadataKeys",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sift.metadata.v1.MetadataService",
                        "DeleteMetadataKeys",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn delete_metadata_values(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteMetadataValuesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DeleteMetadataValuesResponse>,
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
                "/sift.metadata.v1.MetadataService/DeleteMetadataValues",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sift.metadata.v1.MetadataService",
                        "DeleteMetadataValues",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_metadata_usage(
            &mut self,
            request: impl tonic::IntoRequest<super::ListMetadataUsageRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListMetadataUsageResponse>,
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
                "/sift.metadata.v1.MetadataService/ListMetadataUsage",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sift.metadata.v1.MetadataService",
                        "ListMetadataUsage",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod metadata_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with MetadataServiceServer.
    #[async_trait]
    pub trait MetadataService: Send + Sync + 'static {
        async fn create_metadata_key(
            &self,
            request: tonic::Request<super::CreateMetadataKeyRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateMetadataKeyResponse>,
            tonic::Status,
        >;
        async fn create_metadata_value(
            &self,
            request: tonic::Request<super::CreateMetadataValueRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateMetadataValueResponse>,
            tonic::Status,
        >;
        async fn list_metadata_keys(
            &self,
            request: tonic::Request<super::ListMetadataKeysRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListMetadataKeysResponse>,
            tonic::Status,
        >;
        async fn list_metadata_values(
            &self,
            request: tonic::Request<super::ListMetadataValuesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListMetadataValuesResponse>,
            tonic::Status,
        >;
        async fn archive_metadata_keys(
            &self,
            request: tonic::Request<super::ArchiveMetadataKeysRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ArchiveMetadataKeysResponse>,
            tonic::Status,
        >;
        async fn archive_metadata_values(
            &self,
            request: tonic::Request<super::ArchiveMetadataValuesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ArchiveMetadataValuesResponse>,
            tonic::Status,
        >;
        async fn unarchive_metadata_keys(
            &self,
            request: tonic::Request<super::UnarchiveMetadataKeysRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UnarchiveMetadataKeysResponse>,
            tonic::Status,
        >;
        async fn unarchive_metadata_values(
            &self,
            request: tonic::Request<super::UnarchiveMetadataValuesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UnarchiveMetadataValuesResponse>,
            tonic::Status,
        >;
        async fn delete_metadata_keys(
            &self,
            request: tonic::Request<super::DeleteMetadataKeysRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DeleteMetadataKeysResponse>,
            tonic::Status,
        >;
        async fn delete_metadata_values(
            &self,
            request: tonic::Request<super::DeleteMetadataValuesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DeleteMetadataValuesResponse>,
            tonic::Status,
        >;
        async fn list_metadata_usage(
            &self,
            request: tonic::Request<super::ListMetadataUsageRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListMetadataUsageResponse>,
            tonic::Status,
        >;
    }
    #[derive(Debug)]
    pub struct MetadataServiceServer<T: MetadataService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: MetadataService> MetadataServiceServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for MetadataServiceServer<T>
    where
        T: MetadataService,
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
                "/sift.metadata.v1.MetadataService/CreateMetadataKey" => {
                    #[allow(non_camel_case_types)]
                    struct CreateMetadataKeySvc<T: MetadataService>(pub Arc<T>);
                    impl<
                        T: MetadataService,
                    > tonic::server::UnaryService<super::CreateMetadataKeyRequest>
                    for CreateMetadataKeySvc<T> {
                        type Response = super::CreateMetadataKeyResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateMetadataKeyRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as MetadataService>::create_metadata_key(&inner, request)
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
                        let method = CreateMetadataKeySvc(inner);
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
                "/sift.metadata.v1.MetadataService/CreateMetadataValue" => {
                    #[allow(non_camel_case_types)]
                    struct CreateMetadataValueSvc<T: MetadataService>(pub Arc<T>);
                    impl<
                        T: MetadataService,
                    > tonic::server::UnaryService<super::CreateMetadataValueRequest>
                    for CreateMetadataValueSvc<T> {
                        type Response = super::CreateMetadataValueResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateMetadataValueRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as MetadataService>::create_metadata_value(
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
                        let method = CreateMetadataValueSvc(inner);
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
                "/sift.metadata.v1.MetadataService/ListMetadataKeys" => {
                    #[allow(non_camel_case_types)]
                    struct ListMetadataKeysSvc<T: MetadataService>(pub Arc<T>);
                    impl<
                        T: MetadataService,
                    > tonic::server::UnaryService<super::ListMetadataKeysRequest>
                    for ListMetadataKeysSvc<T> {
                        type Response = super::ListMetadataKeysResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListMetadataKeysRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as MetadataService>::list_metadata_keys(&inner, request)
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
                        let method = ListMetadataKeysSvc(inner);
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
                "/sift.metadata.v1.MetadataService/ListMetadataValues" => {
                    #[allow(non_camel_case_types)]
                    struct ListMetadataValuesSvc<T: MetadataService>(pub Arc<T>);
                    impl<
                        T: MetadataService,
                    > tonic::server::UnaryService<super::ListMetadataValuesRequest>
                    for ListMetadataValuesSvc<T> {
                        type Response = super::ListMetadataValuesResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListMetadataValuesRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as MetadataService>::list_metadata_values(
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
                        let method = ListMetadataValuesSvc(inner);
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
                "/sift.metadata.v1.MetadataService/ArchiveMetadataKeys" => {
                    #[allow(non_camel_case_types)]
                    struct ArchiveMetadataKeysSvc<T: MetadataService>(pub Arc<T>);
                    impl<
                        T: MetadataService,
                    > tonic::server::UnaryService<super::ArchiveMetadataKeysRequest>
                    for ArchiveMetadataKeysSvc<T> {
                        type Response = super::ArchiveMetadataKeysResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ArchiveMetadataKeysRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as MetadataService>::archive_metadata_keys(
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
                        let method = ArchiveMetadataKeysSvc(inner);
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
                "/sift.metadata.v1.MetadataService/ArchiveMetadataValues" => {
                    #[allow(non_camel_case_types)]
                    struct ArchiveMetadataValuesSvc<T: MetadataService>(pub Arc<T>);
                    impl<
                        T: MetadataService,
                    > tonic::server::UnaryService<super::ArchiveMetadataValuesRequest>
                    for ArchiveMetadataValuesSvc<T> {
                        type Response = super::ArchiveMetadataValuesResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ArchiveMetadataValuesRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as MetadataService>::archive_metadata_values(
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
                        let method = ArchiveMetadataValuesSvc(inner);
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
                "/sift.metadata.v1.MetadataService/UnarchiveMetadataKeys" => {
                    #[allow(non_camel_case_types)]
                    struct UnarchiveMetadataKeysSvc<T: MetadataService>(pub Arc<T>);
                    impl<
                        T: MetadataService,
                    > tonic::server::UnaryService<super::UnarchiveMetadataKeysRequest>
                    for UnarchiveMetadataKeysSvc<T> {
                        type Response = super::UnarchiveMetadataKeysResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UnarchiveMetadataKeysRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as MetadataService>::unarchive_metadata_keys(
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
                        let method = UnarchiveMetadataKeysSvc(inner);
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
                "/sift.metadata.v1.MetadataService/UnarchiveMetadataValues" => {
                    #[allow(non_camel_case_types)]
                    struct UnarchiveMetadataValuesSvc<T: MetadataService>(pub Arc<T>);
                    impl<
                        T: MetadataService,
                    > tonic::server::UnaryService<super::UnarchiveMetadataValuesRequest>
                    for UnarchiveMetadataValuesSvc<T> {
                        type Response = super::UnarchiveMetadataValuesResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::UnarchiveMetadataValuesRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as MetadataService>::unarchive_metadata_values(
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
                        let method = UnarchiveMetadataValuesSvc(inner);
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
                "/sift.metadata.v1.MetadataService/DeleteMetadataKeys" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteMetadataKeysSvc<T: MetadataService>(pub Arc<T>);
                    impl<
                        T: MetadataService,
                    > tonic::server::UnaryService<super::DeleteMetadataKeysRequest>
                    for DeleteMetadataKeysSvc<T> {
                        type Response = super::DeleteMetadataKeysResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteMetadataKeysRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as MetadataService>::delete_metadata_keys(
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
                        let method = DeleteMetadataKeysSvc(inner);
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
                "/sift.metadata.v1.MetadataService/DeleteMetadataValues" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteMetadataValuesSvc<T: MetadataService>(pub Arc<T>);
                    impl<
                        T: MetadataService,
                    > tonic::server::UnaryService<super::DeleteMetadataValuesRequest>
                    for DeleteMetadataValuesSvc<T> {
                        type Response = super::DeleteMetadataValuesResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteMetadataValuesRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as MetadataService>::delete_metadata_values(
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
                        let method = DeleteMetadataValuesSvc(inner);
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
                "/sift.metadata.v1.MetadataService/ListMetadataUsage" => {
                    #[allow(non_camel_case_types)]
                    struct ListMetadataUsageSvc<T: MetadataService>(pub Arc<T>);
                    impl<
                        T: MetadataService,
                    > tonic::server::UnaryService<super::ListMetadataUsageRequest>
                    for ListMetadataUsageSvc<T> {
                        type Response = super::ListMetadataUsageResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListMetadataUsageRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as MetadataService>::list_metadata_usage(&inner, request)
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
                        let method = ListMetadataUsageSvc(inner);
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
    impl<T: MetadataService> Clone for MetadataServiceServer<T> {
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
    impl<T: MetadataService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: MetadataService> tonic::server::NamedService for MetadataServiceServer<T> {
        const NAME: &'static str = "sift.metadata.v1.MetadataService";
    }
}
