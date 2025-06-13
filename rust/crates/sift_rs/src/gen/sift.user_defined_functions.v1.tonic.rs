// @generated
/// Generated client implementations.
pub mod user_defined_function_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct UserDefinedFunctionServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl UserDefinedFunctionServiceClient<tonic::transport::Channel> {
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
    impl<T> UserDefinedFunctionServiceClient<T>
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
        ) -> UserDefinedFunctionServiceClient<InterceptedService<T, F>>
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
            UserDefinedFunctionServiceClient::new(
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
        pub async fn get_user_defined_function(
            &mut self,
            request: impl tonic::IntoRequest<super::GetUserDefinedFunctionRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetUserDefinedFunctionResponse>,
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
                "/sift.user_defined_functions.v1.UserDefinedFunctionService/GetUserDefinedFunction",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sift.user_defined_functions.v1.UserDefinedFunctionService",
                        "GetUserDefinedFunction",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_user_defined_function_version(
            &mut self,
            request: impl tonic::IntoRequest<super::GetUserDefinedFunctionVersionRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetUserDefinedFunctionVersionResponse>,
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
                "/sift.user_defined_functions.v1.UserDefinedFunctionService/GetUserDefinedFunctionVersion",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sift.user_defined_functions.v1.UserDefinedFunctionService",
                        "GetUserDefinedFunctionVersion",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_user_defined_function_versions(
            &mut self,
            request: impl tonic::IntoRequest<
                super::GetUserDefinedFunctionVersionsRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::GetUserDefinedFunctionVersionsResponse>,
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
                "/sift.user_defined_functions.v1.UserDefinedFunctionService/GetUserDefinedFunctionVersions",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sift.user_defined_functions.v1.UserDefinedFunctionService",
                        "GetUserDefinedFunctionVersions",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_user_defined_function_dependents(
            &mut self,
            request: impl tonic::IntoRequest<
                super::GetUserDefinedFunctionDependentsRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::GetUserDefinedFunctionDependentsResponse>,
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
                "/sift.user_defined_functions.v1.UserDefinedFunctionService/GetUserDefinedFunctionDependents",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sift.user_defined_functions.v1.UserDefinedFunctionService",
                        "GetUserDefinedFunctionDependents",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn create_user_defined_function(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateUserDefinedFunctionRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateUserDefinedFunctionResponse>,
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
                "/sift.user_defined_functions.v1.UserDefinedFunctionService/CreateUserDefinedFunction",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sift.user_defined_functions.v1.UserDefinedFunctionService",
                        "CreateUserDefinedFunction",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn validate_user_defined_function(
            &mut self,
            request: impl tonic::IntoRequest<super::ValidateUserDefinedFunctionRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ValidateUserDefinedFunctionResponse>,
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
                "/sift.user_defined_functions.v1.UserDefinedFunctionService/ValidateUserDefinedFunction",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sift.user_defined_functions.v1.UserDefinedFunctionService",
                        "ValidateUserDefinedFunction",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn update_user_defined_function(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateUserDefinedFunctionRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateUserDefinedFunctionResponse>,
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
                "/sift.user_defined_functions.v1.UserDefinedFunctionService/UpdateUserDefinedFunction",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sift.user_defined_functions.v1.UserDefinedFunctionService",
                        "UpdateUserDefinedFunction",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn check_updatable_fields(
            &mut self,
            request: impl tonic::IntoRequest<super::CheckUpdatableFieldsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CheckUpdatableFieldsResponse>,
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
                "/sift.user_defined_functions.v1.UserDefinedFunctionService/CheckUpdatableFields",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sift.user_defined_functions.v1.UserDefinedFunctionService",
                        "CheckUpdatableFields",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_user_defined_functions(
            &mut self,
            request: impl tonic::IntoRequest<super::ListUserDefinedFunctionsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListUserDefinedFunctionsResponse>,
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
                "/sift.user_defined_functions.v1.UserDefinedFunctionService/ListUserDefinedFunctions",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sift.user_defined_functions.v1.UserDefinedFunctionService",
                        "ListUserDefinedFunctions",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_user_defined_function_versions(
            &mut self,
            request: impl tonic::IntoRequest<
                super::ListUserDefinedFunctionVersionsRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::ListUserDefinedFunctionVersionsResponse>,
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
                "/sift.user_defined_functions.v1.UserDefinedFunctionService/ListUserDefinedFunctionVersions",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sift.user_defined_functions.v1.UserDefinedFunctionService",
                        "ListUserDefinedFunctionVersions",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod user_defined_function_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with UserDefinedFunctionServiceServer.
    #[async_trait]
    pub trait UserDefinedFunctionService: Send + Sync + 'static {
        async fn get_user_defined_function(
            &self,
            request: tonic::Request<super::GetUserDefinedFunctionRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetUserDefinedFunctionResponse>,
            tonic::Status,
        >;
        async fn get_user_defined_function_version(
            &self,
            request: tonic::Request<super::GetUserDefinedFunctionVersionRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetUserDefinedFunctionVersionResponse>,
            tonic::Status,
        >;
        async fn get_user_defined_function_versions(
            &self,
            request: tonic::Request<super::GetUserDefinedFunctionVersionsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetUserDefinedFunctionVersionsResponse>,
            tonic::Status,
        >;
        async fn get_user_defined_function_dependents(
            &self,
            request: tonic::Request<super::GetUserDefinedFunctionDependentsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetUserDefinedFunctionDependentsResponse>,
            tonic::Status,
        >;
        async fn create_user_defined_function(
            &self,
            request: tonic::Request<super::CreateUserDefinedFunctionRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateUserDefinedFunctionResponse>,
            tonic::Status,
        >;
        async fn validate_user_defined_function(
            &self,
            request: tonic::Request<super::ValidateUserDefinedFunctionRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ValidateUserDefinedFunctionResponse>,
            tonic::Status,
        >;
        async fn update_user_defined_function(
            &self,
            request: tonic::Request<super::UpdateUserDefinedFunctionRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateUserDefinedFunctionResponse>,
            tonic::Status,
        >;
        async fn check_updatable_fields(
            &self,
            request: tonic::Request<super::CheckUpdatableFieldsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CheckUpdatableFieldsResponse>,
            tonic::Status,
        >;
        async fn list_user_defined_functions(
            &self,
            request: tonic::Request<super::ListUserDefinedFunctionsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListUserDefinedFunctionsResponse>,
            tonic::Status,
        >;
        async fn list_user_defined_function_versions(
            &self,
            request: tonic::Request<super::ListUserDefinedFunctionVersionsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListUserDefinedFunctionVersionsResponse>,
            tonic::Status,
        >;
    }
    #[derive(Debug)]
    pub struct UserDefinedFunctionServiceServer<T: UserDefinedFunctionService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: UserDefinedFunctionService> UserDefinedFunctionServiceServer<T> {
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
    for UserDefinedFunctionServiceServer<T>
    where
        T: UserDefinedFunctionService,
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
                "/sift.user_defined_functions.v1.UserDefinedFunctionService/GetUserDefinedFunction" => {
                    #[allow(non_camel_case_types)]
                    struct GetUserDefinedFunctionSvc<T: UserDefinedFunctionService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: UserDefinedFunctionService,
                    > tonic::server::UnaryService<super::GetUserDefinedFunctionRequest>
                    for GetUserDefinedFunctionSvc<T> {
                        type Response = super::GetUserDefinedFunctionResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetUserDefinedFunctionRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as UserDefinedFunctionService>::get_user_defined_function(
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
                        let method = GetUserDefinedFunctionSvc(inner);
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
                "/sift.user_defined_functions.v1.UserDefinedFunctionService/GetUserDefinedFunctionVersion" => {
                    #[allow(non_camel_case_types)]
                    struct GetUserDefinedFunctionVersionSvc<
                        T: UserDefinedFunctionService,
                    >(
                        pub Arc<T>,
                    );
                    impl<
                        T: UserDefinedFunctionService,
                    > tonic::server::UnaryService<
                        super::GetUserDefinedFunctionVersionRequest,
                    > for GetUserDefinedFunctionVersionSvc<T> {
                        type Response = super::GetUserDefinedFunctionVersionResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::GetUserDefinedFunctionVersionRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as UserDefinedFunctionService>::get_user_defined_function_version(
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
                        let method = GetUserDefinedFunctionVersionSvc(inner);
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
                "/sift.user_defined_functions.v1.UserDefinedFunctionService/GetUserDefinedFunctionVersions" => {
                    #[allow(non_camel_case_types)]
                    struct GetUserDefinedFunctionVersionsSvc<
                        T: UserDefinedFunctionService,
                    >(
                        pub Arc<T>,
                    );
                    impl<
                        T: UserDefinedFunctionService,
                    > tonic::server::UnaryService<
                        super::GetUserDefinedFunctionVersionsRequest,
                    > for GetUserDefinedFunctionVersionsSvc<T> {
                        type Response = super::GetUserDefinedFunctionVersionsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::GetUserDefinedFunctionVersionsRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as UserDefinedFunctionService>::get_user_defined_function_versions(
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
                        let method = GetUserDefinedFunctionVersionsSvc(inner);
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
                "/sift.user_defined_functions.v1.UserDefinedFunctionService/GetUserDefinedFunctionDependents" => {
                    #[allow(non_camel_case_types)]
                    struct GetUserDefinedFunctionDependentsSvc<
                        T: UserDefinedFunctionService,
                    >(
                        pub Arc<T>,
                    );
                    impl<
                        T: UserDefinedFunctionService,
                    > tonic::server::UnaryService<
                        super::GetUserDefinedFunctionDependentsRequest,
                    > for GetUserDefinedFunctionDependentsSvc<T> {
                        type Response = super::GetUserDefinedFunctionDependentsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::GetUserDefinedFunctionDependentsRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as UserDefinedFunctionService>::get_user_defined_function_dependents(
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
                        let method = GetUserDefinedFunctionDependentsSvc(inner);
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
                "/sift.user_defined_functions.v1.UserDefinedFunctionService/CreateUserDefinedFunction" => {
                    #[allow(non_camel_case_types)]
                    struct CreateUserDefinedFunctionSvc<T: UserDefinedFunctionService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: UserDefinedFunctionService,
                    > tonic::server::UnaryService<
                        super::CreateUserDefinedFunctionRequest,
                    > for CreateUserDefinedFunctionSvc<T> {
                        type Response = super::CreateUserDefinedFunctionResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::CreateUserDefinedFunctionRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as UserDefinedFunctionService>::create_user_defined_function(
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
                        let method = CreateUserDefinedFunctionSvc(inner);
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
                "/sift.user_defined_functions.v1.UserDefinedFunctionService/ValidateUserDefinedFunction" => {
                    #[allow(non_camel_case_types)]
                    struct ValidateUserDefinedFunctionSvc<T: UserDefinedFunctionService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: UserDefinedFunctionService,
                    > tonic::server::UnaryService<
                        super::ValidateUserDefinedFunctionRequest,
                    > for ValidateUserDefinedFunctionSvc<T> {
                        type Response = super::ValidateUserDefinedFunctionResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::ValidateUserDefinedFunctionRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as UserDefinedFunctionService>::validate_user_defined_function(
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
                        let method = ValidateUserDefinedFunctionSvc(inner);
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
                "/sift.user_defined_functions.v1.UserDefinedFunctionService/UpdateUserDefinedFunction" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateUserDefinedFunctionSvc<T: UserDefinedFunctionService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: UserDefinedFunctionService,
                    > tonic::server::UnaryService<
                        super::UpdateUserDefinedFunctionRequest,
                    > for UpdateUserDefinedFunctionSvc<T> {
                        type Response = super::UpdateUserDefinedFunctionResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::UpdateUserDefinedFunctionRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as UserDefinedFunctionService>::update_user_defined_function(
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
                        let method = UpdateUserDefinedFunctionSvc(inner);
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
                "/sift.user_defined_functions.v1.UserDefinedFunctionService/CheckUpdatableFields" => {
                    #[allow(non_camel_case_types)]
                    struct CheckUpdatableFieldsSvc<T: UserDefinedFunctionService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: UserDefinedFunctionService,
                    > tonic::server::UnaryService<super::CheckUpdatableFieldsRequest>
                    for CheckUpdatableFieldsSvc<T> {
                        type Response = super::CheckUpdatableFieldsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CheckUpdatableFieldsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as UserDefinedFunctionService>::check_updatable_fields(
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
                        let method = CheckUpdatableFieldsSvc(inner);
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
                "/sift.user_defined_functions.v1.UserDefinedFunctionService/ListUserDefinedFunctions" => {
                    #[allow(non_camel_case_types)]
                    struct ListUserDefinedFunctionsSvc<T: UserDefinedFunctionService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: UserDefinedFunctionService,
                    > tonic::server::UnaryService<super::ListUserDefinedFunctionsRequest>
                    for ListUserDefinedFunctionsSvc<T> {
                        type Response = super::ListUserDefinedFunctionsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::ListUserDefinedFunctionsRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as UserDefinedFunctionService>::list_user_defined_functions(
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
                        let method = ListUserDefinedFunctionsSvc(inner);
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
                "/sift.user_defined_functions.v1.UserDefinedFunctionService/ListUserDefinedFunctionVersions" => {
                    #[allow(non_camel_case_types)]
                    struct ListUserDefinedFunctionVersionsSvc<
                        T: UserDefinedFunctionService,
                    >(
                        pub Arc<T>,
                    );
                    impl<
                        T: UserDefinedFunctionService,
                    > tonic::server::UnaryService<
                        super::ListUserDefinedFunctionVersionsRequest,
                    > for ListUserDefinedFunctionVersionsSvc<T> {
                        type Response = super::ListUserDefinedFunctionVersionsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::ListUserDefinedFunctionVersionsRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as UserDefinedFunctionService>::list_user_defined_function_versions(
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
                        let method = ListUserDefinedFunctionVersionsSvc(inner);
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
    impl<T: UserDefinedFunctionService> Clone for UserDefinedFunctionServiceServer<T> {
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
    impl<T: UserDefinedFunctionService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: UserDefinedFunctionService> tonic::server::NamedService
    for UserDefinedFunctionServiceServer<T> {
        const NAME: &'static str = "sift.user_defined_functions.v1.UserDefinedFunctionService";
    }
}
