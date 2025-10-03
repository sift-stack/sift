// @generated
/// Generated client implementations.
pub mod annotation_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct AnnotationServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl AnnotationServiceClient<tonic::transport::Channel> {
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
    impl<T> AnnotationServiceClient<T>
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
        ) -> AnnotationServiceClient<InterceptedService<T, F>>
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
            AnnotationServiceClient::new(InterceptedService::new(inner, interceptor))
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
        pub async fn create_annotation(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateAnnotationRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateAnnotationResponse>,
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
                "/sift.annotations.v1.AnnotationService/CreateAnnotation",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sift.annotations.v1.AnnotationService",
                        "CreateAnnotation",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn delete_annotation(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteAnnotationRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DeleteAnnotationResponse>,
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
                "/sift.annotations.v1.AnnotationService/DeleteAnnotation",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sift.annotations.v1.AnnotationService",
                        "DeleteAnnotation",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn archive_annotation(
            &mut self,
            request: impl tonic::IntoRequest<super::ArchiveAnnotationRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ArchiveAnnotationResponse>,
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
                "/sift.annotations.v1.AnnotationService/ArchiveAnnotation",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sift.annotations.v1.AnnotationService",
                        "ArchiveAnnotation",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn unarchive_annotation(
            &mut self,
            request: impl tonic::IntoRequest<super::UnarchiveAnnotationRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UnarchiveAnnotationResponse>,
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
                "/sift.annotations.v1.AnnotationService/UnarchiveAnnotation",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sift.annotations.v1.AnnotationService",
                        "UnarchiveAnnotation",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn batch_delete_annotations(
            &mut self,
            request: impl tonic::IntoRequest<super::BatchDeleteAnnotationsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::BatchDeleteAnnotationsResponse>,
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
                "/sift.annotations.v1.AnnotationService/BatchDeleteAnnotations",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sift.annotations.v1.AnnotationService",
                        "BatchDeleteAnnotations",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn batch_archive_annotations(
            &mut self,
            request: impl tonic::IntoRequest<super::BatchArchiveAnnotationsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::BatchArchiveAnnotationsResponse>,
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
                "/sift.annotations.v1.AnnotationService/BatchArchiveAnnotations",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sift.annotations.v1.AnnotationService",
                        "BatchArchiveAnnotations",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn batch_unarchive_annotations(
            &mut self,
            request: impl tonic::IntoRequest<super::BatchUnarchiveAnnotationsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::BatchUnarchiveAnnotationsResponse>,
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
                "/sift.annotations.v1.AnnotationService/BatchUnarchiveAnnotations",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sift.annotations.v1.AnnotationService",
                        "BatchUnarchiveAnnotations",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_annotations(
            &mut self,
            request: impl tonic::IntoRequest<super::ListAnnotationsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListAnnotationsResponse>,
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
                "/sift.annotations.v1.AnnotationService/ListAnnotations",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sift.annotations.v1.AnnotationService",
                        "ListAnnotations",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_annotation(
            &mut self,
            request: impl tonic::IntoRequest<super::GetAnnotationRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetAnnotationResponse>,
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
                "/sift.annotations.v1.AnnotationService/GetAnnotation",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sift.annotations.v1.AnnotationService",
                        "GetAnnotation",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn update_annotation(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateAnnotationRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateAnnotationResponse>,
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
                "/sift.annotations.v1.AnnotationService/UpdateAnnotation",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sift.annotations.v1.AnnotationService",
                        "UpdateAnnotation",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod annotation_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with AnnotationServiceServer.
    #[async_trait]
    pub trait AnnotationService: Send + Sync + 'static {
        async fn create_annotation(
            &self,
            request: tonic::Request<super::CreateAnnotationRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateAnnotationResponse>,
            tonic::Status,
        >;
        async fn delete_annotation(
            &self,
            request: tonic::Request<super::DeleteAnnotationRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DeleteAnnotationResponse>,
            tonic::Status,
        >;
        async fn archive_annotation(
            &self,
            request: tonic::Request<super::ArchiveAnnotationRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ArchiveAnnotationResponse>,
            tonic::Status,
        >;
        async fn unarchive_annotation(
            &self,
            request: tonic::Request<super::UnarchiveAnnotationRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UnarchiveAnnotationResponse>,
            tonic::Status,
        >;
        async fn batch_delete_annotations(
            &self,
            request: tonic::Request<super::BatchDeleteAnnotationsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::BatchDeleteAnnotationsResponse>,
            tonic::Status,
        >;
        async fn batch_archive_annotations(
            &self,
            request: tonic::Request<super::BatchArchiveAnnotationsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::BatchArchiveAnnotationsResponse>,
            tonic::Status,
        >;
        async fn batch_unarchive_annotations(
            &self,
            request: tonic::Request<super::BatchUnarchiveAnnotationsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::BatchUnarchiveAnnotationsResponse>,
            tonic::Status,
        >;
        async fn list_annotations(
            &self,
            request: tonic::Request<super::ListAnnotationsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListAnnotationsResponse>,
            tonic::Status,
        >;
        async fn get_annotation(
            &self,
            request: tonic::Request<super::GetAnnotationRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetAnnotationResponse>,
            tonic::Status,
        >;
        async fn update_annotation(
            &self,
            request: tonic::Request<super::UpdateAnnotationRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateAnnotationResponse>,
            tonic::Status,
        >;
    }
    #[derive(Debug)]
    pub struct AnnotationServiceServer<T: AnnotationService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: AnnotationService> AnnotationServiceServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for AnnotationServiceServer<T>
    where
        T: AnnotationService,
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
                "/sift.annotations.v1.AnnotationService/CreateAnnotation" => {
                    #[allow(non_camel_case_types)]
                    struct CreateAnnotationSvc<T: AnnotationService>(pub Arc<T>);
                    impl<
                        T: AnnotationService,
                    > tonic::server::UnaryService<super::CreateAnnotationRequest>
                    for CreateAnnotationSvc<T> {
                        type Response = super::CreateAnnotationResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateAnnotationRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as AnnotationService>::create_annotation(&inner, request)
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
                        let method = CreateAnnotationSvc(inner);
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
                "/sift.annotations.v1.AnnotationService/DeleteAnnotation" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteAnnotationSvc<T: AnnotationService>(pub Arc<T>);
                    impl<
                        T: AnnotationService,
                    > tonic::server::UnaryService<super::DeleteAnnotationRequest>
                    for DeleteAnnotationSvc<T> {
                        type Response = super::DeleteAnnotationResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteAnnotationRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as AnnotationService>::delete_annotation(&inner, request)
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
                        let method = DeleteAnnotationSvc(inner);
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
                "/sift.annotations.v1.AnnotationService/ArchiveAnnotation" => {
                    #[allow(non_camel_case_types)]
                    struct ArchiveAnnotationSvc<T: AnnotationService>(pub Arc<T>);
                    impl<
                        T: AnnotationService,
                    > tonic::server::UnaryService<super::ArchiveAnnotationRequest>
                    for ArchiveAnnotationSvc<T> {
                        type Response = super::ArchiveAnnotationResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ArchiveAnnotationRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as AnnotationService>::archive_annotation(
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
                        let method = ArchiveAnnotationSvc(inner);
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
                "/sift.annotations.v1.AnnotationService/UnarchiveAnnotation" => {
                    #[allow(non_camel_case_types)]
                    struct UnarchiveAnnotationSvc<T: AnnotationService>(pub Arc<T>);
                    impl<
                        T: AnnotationService,
                    > tonic::server::UnaryService<super::UnarchiveAnnotationRequest>
                    for UnarchiveAnnotationSvc<T> {
                        type Response = super::UnarchiveAnnotationResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UnarchiveAnnotationRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as AnnotationService>::unarchive_annotation(
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
                        let method = UnarchiveAnnotationSvc(inner);
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
                "/sift.annotations.v1.AnnotationService/BatchDeleteAnnotations" => {
                    #[allow(non_camel_case_types)]
                    struct BatchDeleteAnnotationsSvc<T: AnnotationService>(pub Arc<T>);
                    impl<
                        T: AnnotationService,
                    > tonic::server::UnaryService<super::BatchDeleteAnnotationsRequest>
                    for BatchDeleteAnnotationsSvc<T> {
                        type Response = super::BatchDeleteAnnotationsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::BatchDeleteAnnotationsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as AnnotationService>::batch_delete_annotations(
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
                        let method = BatchDeleteAnnotationsSvc(inner);
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
                "/sift.annotations.v1.AnnotationService/BatchArchiveAnnotations" => {
                    #[allow(non_camel_case_types)]
                    struct BatchArchiveAnnotationsSvc<T: AnnotationService>(pub Arc<T>);
                    impl<
                        T: AnnotationService,
                    > tonic::server::UnaryService<super::BatchArchiveAnnotationsRequest>
                    for BatchArchiveAnnotationsSvc<T> {
                        type Response = super::BatchArchiveAnnotationsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::BatchArchiveAnnotationsRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as AnnotationService>::batch_archive_annotations(
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
                        let method = BatchArchiveAnnotationsSvc(inner);
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
                "/sift.annotations.v1.AnnotationService/BatchUnarchiveAnnotations" => {
                    #[allow(non_camel_case_types)]
                    struct BatchUnarchiveAnnotationsSvc<T: AnnotationService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: AnnotationService,
                    > tonic::server::UnaryService<
                        super::BatchUnarchiveAnnotationsRequest,
                    > for BatchUnarchiveAnnotationsSvc<T> {
                        type Response = super::BatchUnarchiveAnnotationsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::BatchUnarchiveAnnotationsRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as AnnotationService>::batch_unarchive_annotations(
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
                        let method = BatchUnarchiveAnnotationsSvc(inner);
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
                "/sift.annotations.v1.AnnotationService/ListAnnotations" => {
                    #[allow(non_camel_case_types)]
                    struct ListAnnotationsSvc<T: AnnotationService>(pub Arc<T>);
                    impl<
                        T: AnnotationService,
                    > tonic::server::UnaryService<super::ListAnnotationsRequest>
                    for ListAnnotationsSvc<T> {
                        type Response = super::ListAnnotationsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListAnnotationsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as AnnotationService>::list_annotations(&inner, request)
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
                        let method = ListAnnotationsSvc(inner);
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
                "/sift.annotations.v1.AnnotationService/GetAnnotation" => {
                    #[allow(non_camel_case_types)]
                    struct GetAnnotationSvc<T: AnnotationService>(pub Arc<T>);
                    impl<
                        T: AnnotationService,
                    > tonic::server::UnaryService<super::GetAnnotationRequest>
                    for GetAnnotationSvc<T> {
                        type Response = super::GetAnnotationResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetAnnotationRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as AnnotationService>::get_annotation(&inner, request)
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
                        let method = GetAnnotationSvc(inner);
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
                "/sift.annotations.v1.AnnotationService/UpdateAnnotation" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateAnnotationSvc<T: AnnotationService>(pub Arc<T>);
                    impl<
                        T: AnnotationService,
                    > tonic::server::UnaryService<super::UpdateAnnotationRequest>
                    for UpdateAnnotationSvc<T> {
                        type Response = super::UpdateAnnotationResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdateAnnotationRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as AnnotationService>::update_annotation(&inner, request)
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
                        let method = UpdateAnnotationSvc(inner);
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
    impl<T: AnnotationService> Clone for AnnotationServiceServer<T> {
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
    impl<T: AnnotationService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: AnnotationService> tonic::server::NamedService
    for AnnotationServiceServer<T> {
        const NAME: &'static str = "sift.annotations.v1.AnnotationService";
    }
}
