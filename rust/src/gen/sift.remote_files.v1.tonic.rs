// @generated
/// Generated client implementations.
pub mod remote_file_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct RemoteFileServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl RemoteFileServiceClient<tonic::transport::Channel> {
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
    impl<T> RemoteFileServiceClient<T>
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
        ) -> RemoteFileServiceClient<InterceptedService<T, F>>
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
            RemoteFileServiceClient::new(InterceptedService::new(inner, interceptor))
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
        pub async fn get_remote_file(
            &mut self,
            request: impl tonic::IntoRequest<super::GetRemoteFileRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetRemoteFileResponse>,
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
                "/sift.remote_files.v1.RemoteFileService/GetRemoteFile",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sift.remote_files.v1.RemoteFileService",
                        "GetRemoteFile",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn create_remote_file(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateRemoteFileRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateRemoteFileResponse>,
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
                "/sift.remote_files.v1.RemoteFileService/CreateRemoteFile",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sift.remote_files.v1.RemoteFileService",
                        "CreateRemoteFile",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_remote_files(
            &mut self,
            request: impl tonic::IntoRequest<super::ListRemoteFilesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListRemoteFilesResponse>,
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
                "/sift.remote_files.v1.RemoteFileService/ListRemoteFiles",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sift.remote_files.v1.RemoteFileService",
                        "ListRemoteFiles",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn update_remote_file(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateRemoteFileRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateRemoteFileResponse>,
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
                "/sift.remote_files.v1.RemoteFileService/UpdateRemoteFile",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sift.remote_files.v1.RemoteFileService",
                        "UpdateRemoteFile",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn delete_remote_file(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteRemoteFileRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DeleteRemoteFileResponse>,
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
                "/sift.remote_files.v1.RemoteFileService/DeleteRemoteFile",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sift.remote_files.v1.RemoteFileService",
                        "DeleteRemoteFile",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn batch_delete_remote_files(
            &mut self,
            request: impl tonic::IntoRequest<super::BatchDeleteRemoteFilesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::BatchDeleteRemoteFilesResponse>,
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
                "/sift.remote_files.v1.RemoteFileService/BatchDeleteRemoteFiles",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sift.remote_files.v1.RemoteFileService",
                        "BatchDeleteRemoteFiles",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_remote_file_download_url(
            &mut self,
            request: impl tonic::IntoRequest<super::GetRemoteFileDownloadUrlRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetRemoteFileDownloadUrlResponse>,
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
                "/sift.remote_files.v1.RemoteFileService/GetRemoteFileDownloadUrl",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sift.remote_files.v1.RemoteFileService",
                        "GetRemoteFileDownloadUrl",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod remote_file_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with RemoteFileServiceServer.
    #[async_trait]
    pub trait RemoteFileService: Send + Sync + 'static {
        async fn get_remote_file(
            &self,
            request: tonic::Request<super::GetRemoteFileRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetRemoteFileResponse>,
            tonic::Status,
        >;
        async fn create_remote_file(
            &self,
            request: tonic::Request<super::CreateRemoteFileRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateRemoteFileResponse>,
            tonic::Status,
        >;
        async fn list_remote_files(
            &self,
            request: tonic::Request<super::ListRemoteFilesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListRemoteFilesResponse>,
            tonic::Status,
        >;
        async fn update_remote_file(
            &self,
            request: tonic::Request<super::UpdateRemoteFileRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateRemoteFileResponse>,
            tonic::Status,
        >;
        async fn delete_remote_file(
            &self,
            request: tonic::Request<super::DeleteRemoteFileRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DeleteRemoteFileResponse>,
            tonic::Status,
        >;
        async fn batch_delete_remote_files(
            &self,
            request: tonic::Request<super::BatchDeleteRemoteFilesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::BatchDeleteRemoteFilesResponse>,
            tonic::Status,
        >;
        async fn get_remote_file_download_url(
            &self,
            request: tonic::Request<super::GetRemoteFileDownloadUrlRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetRemoteFileDownloadUrlResponse>,
            tonic::Status,
        >;
    }
    #[derive(Debug)]
    pub struct RemoteFileServiceServer<T: RemoteFileService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: RemoteFileService> RemoteFileServiceServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for RemoteFileServiceServer<T>
    where
        T: RemoteFileService,
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
                "/sift.remote_files.v1.RemoteFileService/GetRemoteFile" => {
                    #[allow(non_camel_case_types)]
                    struct GetRemoteFileSvc<T: RemoteFileService>(pub Arc<T>);
                    impl<
                        T: RemoteFileService,
                    > tonic::server::UnaryService<super::GetRemoteFileRequest>
                    for GetRemoteFileSvc<T> {
                        type Response = super::GetRemoteFileResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetRemoteFileRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as RemoteFileService>::get_remote_file(&inner, request)
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
                        let method = GetRemoteFileSvc(inner);
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
                "/sift.remote_files.v1.RemoteFileService/CreateRemoteFile" => {
                    #[allow(non_camel_case_types)]
                    struct CreateRemoteFileSvc<T: RemoteFileService>(pub Arc<T>);
                    impl<
                        T: RemoteFileService,
                    > tonic::server::UnaryService<super::CreateRemoteFileRequest>
                    for CreateRemoteFileSvc<T> {
                        type Response = super::CreateRemoteFileResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateRemoteFileRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as RemoteFileService>::create_remote_file(
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
                        let method = CreateRemoteFileSvc(inner);
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
                "/sift.remote_files.v1.RemoteFileService/ListRemoteFiles" => {
                    #[allow(non_camel_case_types)]
                    struct ListRemoteFilesSvc<T: RemoteFileService>(pub Arc<T>);
                    impl<
                        T: RemoteFileService,
                    > tonic::server::UnaryService<super::ListRemoteFilesRequest>
                    for ListRemoteFilesSvc<T> {
                        type Response = super::ListRemoteFilesResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListRemoteFilesRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as RemoteFileService>::list_remote_files(&inner, request)
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
                        let method = ListRemoteFilesSvc(inner);
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
                "/sift.remote_files.v1.RemoteFileService/UpdateRemoteFile" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateRemoteFileSvc<T: RemoteFileService>(pub Arc<T>);
                    impl<
                        T: RemoteFileService,
                    > tonic::server::UnaryService<super::UpdateRemoteFileRequest>
                    for UpdateRemoteFileSvc<T> {
                        type Response = super::UpdateRemoteFileResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdateRemoteFileRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as RemoteFileService>::update_remote_file(
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
                        let method = UpdateRemoteFileSvc(inner);
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
                "/sift.remote_files.v1.RemoteFileService/DeleteRemoteFile" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteRemoteFileSvc<T: RemoteFileService>(pub Arc<T>);
                    impl<
                        T: RemoteFileService,
                    > tonic::server::UnaryService<super::DeleteRemoteFileRequest>
                    for DeleteRemoteFileSvc<T> {
                        type Response = super::DeleteRemoteFileResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteRemoteFileRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as RemoteFileService>::delete_remote_file(
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
                        let method = DeleteRemoteFileSvc(inner);
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
                "/sift.remote_files.v1.RemoteFileService/BatchDeleteRemoteFiles" => {
                    #[allow(non_camel_case_types)]
                    struct BatchDeleteRemoteFilesSvc<T: RemoteFileService>(pub Arc<T>);
                    impl<
                        T: RemoteFileService,
                    > tonic::server::UnaryService<super::BatchDeleteRemoteFilesRequest>
                    for BatchDeleteRemoteFilesSvc<T> {
                        type Response = super::BatchDeleteRemoteFilesResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::BatchDeleteRemoteFilesRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as RemoteFileService>::batch_delete_remote_files(
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
                        let method = BatchDeleteRemoteFilesSvc(inner);
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
                "/sift.remote_files.v1.RemoteFileService/GetRemoteFileDownloadUrl" => {
                    #[allow(non_camel_case_types)]
                    struct GetRemoteFileDownloadUrlSvc<T: RemoteFileService>(pub Arc<T>);
                    impl<
                        T: RemoteFileService,
                    > tonic::server::UnaryService<super::GetRemoteFileDownloadUrlRequest>
                    for GetRemoteFileDownloadUrlSvc<T> {
                        type Response = super::GetRemoteFileDownloadUrlResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::GetRemoteFileDownloadUrlRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as RemoteFileService>::get_remote_file_download_url(
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
                        let method = GetRemoteFileDownloadUrlSvc(inner);
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
    impl<T: RemoteFileService> Clone for RemoteFileServiceServer<T> {
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
    impl<T: RemoteFileService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: RemoteFileService> tonic::server::NamedService
    for RemoteFileServiceServer<T> {
        const NAME: &'static str = "sift.remote_files.v1.RemoteFileService";
    }
}
