// @generated
/// Generated client implementations.
pub mod protobuf_descriptor_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct ProtobufDescriptorServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl ProtobufDescriptorServiceClient<tonic::transport::Channel> {
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
    impl<T> ProtobufDescriptorServiceClient<T>
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
        ) -> ProtobufDescriptorServiceClient<InterceptedService<T, F>>
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
            ProtobufDescriptorServiceClient::new(
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
        pub async fn add_protobuf_descriptor(
            &mut self,
            request: impl tonic::IntoRequest<super::AddProtobufDescriptorRequest>,
        ) -> std::result::Result<
            tonic::Response<super::AddProtobufDescriptorResponse>,
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
                "/sift.protobuf_descriptors.v2.ProtobufDescriptorService/AddProtobufDescriptor",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sift.protobuf_descriptors.v2.ProtobufDescriptorService",
                        "AddProtobufDescriptor",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn check_protobuf_descriptor_compatibility(
            &mut self,
            request: impl tonic::IntoRequest<
                super::CheckProtobufDescriptorCompatibilityRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::CheckProtobufDescriptorCompatibilityResponse>,
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
                "/sift.protobuf_descriptors.v2.ProtobufDescriptorService/CheckProtobufDescriptorCompatibility",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sift.protobuf_descriptors.v2.ProtobufDescriptorService",
                        "CheckProtobufDescriptorCompatibility",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn delete_protobuf_descriptors(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteProtobufDescriptorsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DeleteProtobufDescriptorsResponse>,
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
                "/sift.protobuf_descriptors.v2.ProtobufDescriptorService/DeleteProtobufDescriptors",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sift.protobuf_descriptors.v2.ProtobufDescriptorService",
                        "DeleteProtobufDescriptors",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_protobuf_descriptors(
            &mut self,
            request: impl tonic::IntoRequest<super::ListProtobufDescriptorsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListProtobufDescriptorsResponse>,
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
                "/sift.protobuf_descriptors.v2.ProtobufDescriptorService/ListProtobufDescriptors",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sift.protobuf_descriptors.v2.ProtobufDescriptorService",
                        "ListProtobufDescriptors",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod protobuf_descriptor_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with ProtobufDescriptorServiceServer.
    #[async_trait]
    pub trait ProtobufDescriptorService: Send + Sync + 'static {
        async fn add_protobuf_descriptor(
            &self,
            request: tonic::Request<super::AddProtobufDescriptorRequest>,
        ) -> std::result::Result<
            tonic::Response<super::AddProtobufDescriptorResponse>,
            tonic::Status,
        >;
        async fn check_protobuf_descriptor_compatibility(
            &self,
            request: tonic::Request<super::CheckProtobufDescriptorCompatibilityRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CheckProtobufDescriptorCompatibilityResponse>,
            tonic::Status,
        >;
        async fn delete_protobuf_descriptors(
            &self,
            request: tonic::Request<super::DeleteProtobufDescriptorsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DeleteProtobufDescriptorsResponse>,
            tonic::Status,
        >;
        async fn list_protobuf_descriptors(
            &self,
            request: tonic::Request<super::ListProtobufDescriptorsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListProtobufDescriptorsResponse>,
            tonic::Status,
        >;
    }
    #[derive(Debug)]
    pub struct ProtobufDescriptorServiceServer<T: ProtobufDescriptorService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: ProtobufDescriptorService> ProtobufDescriptorServiceServer<T> {
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
    for ProtobufDescriptorServiceServer<T>
    where
        T: ProtobufDescriptorService,
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
                "/sift.protobuf_descriptors.v2.ProtobufDescriptorService/AddProtobufDescriptor" => {
                    #[allow(non_camel_case_types)]
                    struct AddProtobufDescriptorSvc<T: ProtobufDescriptorService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: ProtobufDescriptorService,
                    > tonic::server::UnaryService<super::AddProtobufDescriptorRequest>
                    for AddProtobufDescriptorSvc<T> {
                        type Response = super::AddProtobufDescriptorResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::AddProtobufDescriptorRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ProtobufDescriptorService>::add_protobuf_descriptor(
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
                        let method = AddProtobufDescriptorSvc(inner);
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
                "/sift.protobuf_descriptors.v2.ProtobufDescriptorService/CheckProtobufDescriptorCompatibility" => {
                    #[allow(non_camel_case_types)]
                    struct CheckProtobufDescriptorCompatibilitySvc<
                        T: ProtobufDescriptorService,
                    >(
                        pub Arc<T>,
                    );
                    impl<
                        T: ProtobufDescriptorService,
                    > tonic::server::UnaryService<
                        super::CheckProtobufDescriptorCompatibilityRequest,
                    > for CheckProtobufDescriptorCompatibilitySvc<T> {
                        type Response = super::CheckProtobufDescriptorCompatibilityResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::CheckProtobufDescriptorCompatibilityRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ProtobufDescriptorService>::check_protobuf_descriptor_compatibility(
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
                        let method = CheckProtobufDescriptorCompatibilitySvc(inner);
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
                "/sift.protobuf_descriptors.v2.ProtobufDescriptorService/DeleteProtobufDescriptors" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteProtobufDescriptorsSvc<T: ProtobufDescriptorService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: ProtobufDescriptorService,
                    > tonic::server::UnaryService<
                        super::DeleteProtobufDescriptorsRequest,
                    > for DeleteProtobufDescriptorsSvc<T> {
                        type Response = super::DeleteProtobufDescriptorsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::DeleteProtobufDescriptorsRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ProtobufDescriptorService>::delete_protobuf_descriptors(
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
                        let method = DeleteProtobufDescriptorsSvc(inner);
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
                "/sift.protobuf_descriptors.v2.ProtobufDescriptorService/ListProtobufDescriptors" => {
                    #[allow(non_camel_case_types)]
                    struct ListProtobufDescriptorsSvc<T: ProtobufDescriptorService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: ProtobufDescriptorService,
                    > tonic::server::UnaryService<super::ListProtobufDescriptorsRequest>
                    for ListProtobufDescriptorsSvc<T> {
                        type Response = super::ListProtobufDescriptorsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::ListProtobufDescriptorsRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ProtobufDescriptorService>::list_protobuf_descriptors(
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
                        let method = ListProtobufDescriptorsSvc(inner);
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
    impl<T: ProtobufDescriptorService> Clone for ProtobufDescriptorServiceServer<T> {
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
    impl<T: ProtobufDescriptorService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: ProtobufDescriptorService> tonic::server::NamedService
    for ProtobufDescriptorServiceServer<T> {
        const NAME: &'static str = "sift.protobuf_descriptors.v2.ProtobufDescriptorService";
    }
}
