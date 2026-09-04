// @generated
/// Generated client implementations.
pub mod artifact_service_client {
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
    pub struct ArtifactServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl ArtifactServiceClient<tonic::transport::Channel> {
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
    impl<T> ArtifactServiceClient<T>
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
        ) -> ArtifactServiceClient<InterceptedService<T, F>>
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
            ArtifactServiceClient::new(InterceptedService::new(inner, interceptor))
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
        pub async fn create_artifact(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateArtifactRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateArtifactResponse>,
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
                "/sift.artifacts.v1.ArtifactService/CreateArtifact",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sift.artifacts.v1.ArtifactService",
                        "CreateArtifact",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_artifact(
            &mut self,
            request: impl tonic::IntoRequest<super::GetArtifactRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetArtifactResponse>,
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
                "/sift.artifacts.v1.ArtifactService/GetArtifact",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("sift.artifacts.v1.ArtifactService", "GetArtifact"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_artifacts(
            &mut self,
            request: impl tonic::IntoRequest<super::ListArtifactsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListArtifactsResponse>,
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
                "/sift.artifacts.v1.ArtifactService/ListArtifacts",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("sift.artifacts.v1.ArtifactService", "ListArtifacts"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_artifact_versions(
            &mut self,
            request: impl tonic::IntoRequest<super::ListArtifactVersionsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListArtifactVersionsResponse>,
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
                "/sift.artifacts.v1.ArtifactService/ListArtifactVersions",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sift.artifacts.v1.ArtifactService",
                        "ListArtifactVersions",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn link_artifact(
            &mut self,
            request: impl tonic::IntoRequest<super::LinkArtifactRequest>,
        ) -> std::result::Result<
            tonic::Response<super::LinkArtifactResponse>,
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
                "/sift.artifacts.v1.ArtifactService/LinkArtifact",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("sift.artifacts.v1.ArtifactService", "LinkArtifact"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn unlink_artifact(
            &mut self,
            request: impl tonic::IntoRequest<super::UnlinkArtifactRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UnlinkArtifactResponse>,
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
                "/sift.artifacts.v1.ArtifactService/UnlinkArtifact",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sift.artifacts.v1.ArtifactService",
                        "UnlinkArtifact",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_artifact_links(
            &mut self,
            request: impl tonic::IntoRequest<super::ListArtifactLinksRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListArtifactLinksResponse>,
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
                "/sift.artifacts.v1.ArtifactService/ListArtifactLinks",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sift.artifacts.v1.ArtifactService",
                        "ListArtifactLinks",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn link_artifact_to_conversation(
            &mut self,
            request: impl tonic::IntoRequest<super::LinkArtifactToConversationRequest>,
        ) -> std::result::Result<
            tonic::Response<super::LinkArtifactToConversationResponse>,
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
                "/sift.artifacts.v1.ArtifactService/LinkArtifactToConversation",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sift.artifacts.v1.ArtifactService",
                        "LinkArtifactToConversation",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn unlink_artifact_from_conversation(
            &mut self,
            request: impl tonic::IntoRequest<
                super::UnlinkArtifactFromConversationRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::UnlinkArtifactFromConversationResponse>,
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
                "/sift.artifacts.v1.ArtifactService/UnlinkArtifactFromConversation",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sift.artifacts.v1.ArtifactService",
                        "UnlinkArtifactFromConversation",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn archive_artifact(
            &mut self,
            request: impl tonic::IntoRequest<super::ArchiveArtifactRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ArchiveArtifactResponse>,
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
                "/sift.artifacts.v1.ArtifactService/ArchiveArtifact",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sift.artifacts.v1.ArtifactService",
                        "ArchiveArtifact",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn unarchive_artifact(
            &mut self,
            request: impl tonic::IntoRequest<super::UnarchiveArtifactRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UnarchiveArtifactResponse>,
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
                "/sift.artifacts.v1.ArtifactService/UnarchiveArtifact",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sift.artifacts.v1.ArtifactService",
                        "UnarchiveArtifact",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod artifact_service_server {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::wildcard_imports,
        clippy::let_unit_value,
    )]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with ArtifactServiceServer.
    #[async_trait]
    pub trait ArtifactService: std::marker::Send + std::marker::Sync + 'static {
        async fn create_artifact(
            &self,
            request: tonic::Request<super::CreateArtifactRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateArtifactResponse>,
            tonic::Status,
        >;
        async fn get_artifact(
            &self,
            request: tonic::Request<super::GetArtifactRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetArtifactResponse>,
            tonic::Status,
        >;
        async fn list_artifacts(
            &self,
            request: tonic::Request<super::ListArtifactsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListArtifactsResponse>,
            tonic::Status,
        >;
        async fn list_artifact_versions(
            &self,
            request: tonic::Request<super::ListArtifactVersionsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListArtifactVersionsResponse>,
            tonic::Status,
        >;
        async fn link_artifact(
            &self,
            request: tonic::Request<super::LinkArtifactRequest>,
        ) -> std::result::Result<
            tonic::Response<super::LinkArtifactResponse>,
            tonic::Status,
        >;
        async fn unlink_artifact(
            &self,
            request: tonic::Request<super::UnlinkArtifactRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UnlinkArtifactResponse>,
            tonic::Status,
        >;
        async fn list_artifact_links(
            &self,
            request: tonic::Request<super::ListArtifactLinksRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListArtifactLinksResponse>,
            tonic::Status,
        >;
        async fn link_artifact_to_conversation(
            &self,
            request: tonic::Request<super::LinkArtifactToConversationRequest>,
        ) -> std::result::Result<
            tonic::Response<super::LinkArtifactToConversationResponse>,
            tonic::Status,
        >;
        async fn unlink_artifact_from_conversation(
            &self,
            request: tonic::Request<super::UnlinkArtifactFromConversationRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UnlinkArtifactFromConversationResponse>,
            tonic::Status,
        >;
        async fn archive_artifact(
            &self,
            request: tonic::Request<super::ArchiveArtifactRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ArchiveArtifactResponse>,
            tonic::Status,
        >;
        async fn unarchive_artifact(
            &self,
            request: tonic::Request<super::UnarchiveArtifactRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UnarchiveArtifactResponse>,
            tonic::Status,
        >;
    }
    #[derive(Debug)]
    pub struct ArtifactServiceServer<T> {
        inner: Arc<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    impl<T> ArtifactServiceServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for ArtifactServiceServer<T>
    where
        T: ArtifactService,
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
                "/sift.artifacts.v1.ArtifactService/CreateArtifact" => {
                    #[allow(non_camel_case_types)]
                    struct CreateArtifactSvc<T: ArtifactService>(pub Arc<T>);
                    impl<
                        T: ArtifactService,
                    > tonic::server::UnaryService<super::CreateArtifactRequest>
                    for CreateArtifactSvc<T> {
                        type Response = super::CreateArtifactResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateArtifactRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ArtifactService>::create_artifact(&inner, request)
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
                        let method = CreateArtifactSvc(inner);
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
                "/sift.artifacts.v1.ArtifactService/GetArtifact" => {
                    #[allow(non_camel_case_types)]
                    struct GetArtifactSvc<T: ArtifactService>(pub Arc<T>);
                    impl<
                        T: ArtifactService,
                    > tonic::server::UnaryService<super::GetArtifactRequest>
                    for GetArtifactSvc<T> {
                        type Response = super::GetArtifactResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetArtifactRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ArtifactService>::get_artifact(&inner, request).await
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
                        let method = GetArtifactSvc(inner);
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
                "/sift.artifacts.v1.ArtifactService/ListArtifacts" => {
                    #[allow(non_camel_case_types)]
                    struct ListArtifactsSvc<T: ArtifactService>(pub Arc<T>);
                    impl<
                        T: ArtifactService,
                    > tonic::server::UnaryService<super::ListArtifactsRequest>
                    for ListArtifactsSvc<T> {
                        type Response = super::ListArtifactsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListArtifactsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ArtifactService>::list_artifacts(&inner, request)
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
                        let method = ListArtifactsSvc(inner);
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
                "/sift.artifacts.v1.ArtifactService/ListArtifactVersions" => {
                    #[allow(non_camel_case_types)]
                    struct ListArtifactVersionsSvc<T: ArtifactService>(pub Arc<T>);
                    impl<
                        T: ArtifactService,
                    > tonic::server::UnaryService<super::ListArtifactVersionsRequest>
                    for ListArtifactVersionsSvc<T> {
                        type Response = super::ListArtifactVersionsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListArtifactVersionsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ArtifactService>::list_artifact_versions(
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
                        let method = ListArtifactVersionsSvc(inner);
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
                "/sift.artifacts.v1.ArtifactService/LinkArtifact" => {
                    #[allow(non_camel_case_types)]
                    struct LinkArtifactSvc<T: ArtifactService>(pub Arc<T>);
                    impl<
                        T: ArtifactService,
                    > tonic::server::UnaryService<super::LinkArtifactRequest>
                    for LinkArtifactSvc<T> {
                        type Response = super::LinkArtifactResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::LinkArtifactRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ArtifactService>::link_artifact(&inner, request).await
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
                        let method = LinkArtifactSvc(inner);
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
                "/sift.artifacts.v1.ArtifactService/UnlinkArtifact" => {
                    #[allow(non_camel_case_types)]
                    struct UnlinkArtifactSvc<T: ArtifactService>(pub Arc<T>);
                    impl<
                        T: ArtifactService,
                    > tonic::server::UnaryService<super::UnlinkArtifactRequest>
                    for UnlinkArtifactSvc<T> {
                        type Response = super::UnlinkArtifactResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UnlinkArtifactRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ArtifactService>::unlink_artifact(&inner, request)
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
                        let method = UnlinkArtifactSvc(inner);
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
                "/sift.artifacts.v1.ArtifactService/ListArtifactLinks" => {
                    #[allow(non_camel_case_types)]
                    struct ListArtifactLinksSvc<T: ArtifactService>(pub Arc<T>);
                    impl<
                        T: ArtifactService,
                    > tonic::server::UnaryService<super::ListArtifactLinksRequest>
                    for ListArtifactLinksSvc<T> {
                        type Response = super::ListArtifactLinksResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListArtifactLinksRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ArtifactService>::list_artifact_links(&inner, request)
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
                        let method = ListArtifactLinksSvc(inner);
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
                "/sift.artifacts.v1.ArtifactService/LinkArtifactToConversation" => {
                    #[allow(non_camel_case_types)]
                    struct LinkArtifactToConversationSvc<T: ArtifactService>(pub Arc<T>);
                    impl<
                        T: ArtifactService,
                    > tonic::server::UnaryService<
                        super::LinkArtifactToConversationRequest,
                    > for LinkArtifactToConversationSvc<T> {
                        type Response = super::LinkArtifactToConversationResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::LinkArtifactToConversationRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ArtifactService>::link_artifact_to_conversation(
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
                        let method = LinkArtifactToConversationSvc(inner);
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
                "/sift.artifacts.v1.ArtifactService/UnlinkArtifactFromConversation" => {
                    #[allow(non_camel_case_types)]
                    struct UnlinkArtifactFromConversationSvc<T: ArtifactService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: ArtifactService,
                    > tonic::server::UnaryService<
                        super::UnlinkArtifactFromConversationRequest,
                    > for UnlinkArtifactFromConversationSvc<T> {
                        type Response = super::UnlinkArtifactFromConversationResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::UnlinkArtifactFromConversationRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ArtifactService>::unlink_artifact_from_conversation(
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
                        let method = UnlinkArtifactFromConversationSvc(inner);
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
                "/sift.artifacts.v1.ArtifactService/ArchiveArtifact" => {
                    #[allow(non_camel_case_types)]
                    struct ArchiveArtifactSvc<T: ArtifactService>(pub Arc<T>);
                    impl<
                        T: ArtifactService,
                    > tonic::server::UnaryService<super::ArchiveArtifactRequest>
                    for ArchiveArtifactSvc<T> {
                        type Response = super::ArchiveArtifactResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ArchiveArtifactRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ArtifactService>::archive_artifact(&inner, request)
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
                        let method = ArchiveArtifactSvc(inner);
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
                "/sift.artifacts.v1.ArtifactService/UnarchiveArtifact" => {
                    #[allow(non_camel_case_types)]
                    struct UnarchiveArtifactSvc<T: ArtifactService>(pub Arc<T>);
                    impl<
                        T: ArtifactService,
                    > tonic::server::UnaryService<super::UnarchiveArtifactRequest>
                    for UnarchiveArtifactSvc<T> {
                        type Response = super::UnarchiveArtifactResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UnarchiveArtifactRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ArtifactService>::unarchive_artifact(&inner, request)
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
                        let method = UnarchiveArtifactSvc(inner);
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
    impl<T> Clone for ArtifactServiceServer<T> {
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
    pub const SERVICE_NAME: &str = "sift.artifacts.v1.ArtifactService";
    impl<T> tonic::server::NamedService for ArtifactServiceServer<T> {
        const NAME: &'static str = SERVICE_NAME;
    }
}
