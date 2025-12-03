// @generated
/// Generated client implementations.
pub mod user_attributes_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct UserAttributesServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl UserAttributesServiceClient<tonic::transport::Channel> {
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
    impl<T> UserAttributesServiceClient<T>
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
        ) -> UserAttributesServiceClient<InterceptedService<T, F>>
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
            UserAttributesServiceClient::new(InterceptedService::new(inner, interceptor))
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
        pub async fn create_user_attribute_key(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateUserAttributeKeyRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateUserAttributeKeyResponse>,
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
                "/sift.user_attributes.v1.UserAttributesService/CreateUserAttributeKey",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sift.user_attributes.v1.UserAttributesService",
                        "CreateUserAttributeKey",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_user_attribute_key(
            &mut self,
            request: impl tonic::IntoRequest<super::GetUserAttributeKeyRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetUserAttributeKeyResponse>,
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
                "/sift.user_attributes.v1.UserAttributesService/GetUserAttributeKey",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sift.user_attributes.v1.UserAttributesService",
                        "GetUserAttributeKey",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_user_attribute_keys(
            &mut self,
            request: impl tonic::IntoRequest<super::ListUserAttributeKeysRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListUserAttributeKeysResponse>,
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
                "/sift.user_attributes.v1.UserAttributesService/ListUserAttributeKeys",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sift.user_attributes.v1.UserAttributesService",
                        "ListUserAttributeKeys",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn update_user_attribute_key(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateUserAttributeKeyRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateUserAttributeKeyResponse>,
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
                "/sift.user_attributes.v1.UserAttributesService/UpdateUserAttributeKey",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sift.user_attributes.v1.UserAttributesService",
                        "UpdateUserAttributeKey",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn archive_user_attribute_keys(
            &mut self,
            request: impl tonic::IntoRequest<super::ArchiveUserAttributeKeysRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ArchiveUserAttributeKeysResponse>,
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
                "/sift.user_attributes.v1.UserAttributesService/ArchiveUserAttributeKeys",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sift.user_attributes.v1.UserAttributesService",
                        "ArchiveUserAttributeKeys",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn unarchive_user_attribute_keys(
            &mut self,
            request: impl tonic::IntoRequest<super::UnarchiveUserAttributeKeysRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UnarchiveUserAttributeKeysResponse>,
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
                "/sift.user_attributes.v1.UserAttributesService/UnarchiveUserAttributeKeys",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sift.user_attributes.v1.UserAttributesService",
                        "UnarchiveUserAttributeKeys",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn create_user_attribute_value(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateUserAttributeValueRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateUserAttributeValueResponse>,
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
                "/sift.user_attributes.v1.UserAttributesService/CreateUserAttributeValue",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sift.user_attributes.v1.UserAttributesService",
                        "CreateUserAttributeValue",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn batch_create_user_attribute_value(
            &mut self,
            request: impl tonic::IntoRequest<super::BatchCreateUserAttributeValueRequest>,
        ) -> std::result::Result<
            tonic::Response<super::BatchCreateUserAttributeValueResponse>,
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
                "/sift.user_attributes.v1.UserAttributesService/BatchCreateUserAttributeValue",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sift.user_attributes.v1.UserAttributesService",
                        "BatchCreateUserAttributeValue",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_user_attribute_value(
            &mut self,
            request: impl tonic::IntoRequest<super::GetUserAttributeValueRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetUserAttributeValueResponse>,
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
                "/sift.user_attributes.v1.UserAttributesService/GetUserAttributeValue",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sift.user_attributes.v1.UserAttributesService",
                        "GetUserAttributeValue",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_user_attribute_key_values(
            &mut self,
            request: impl tonic::IntoRequest<super::ListUserAttributeKeyValuesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListUserAttributeKeyValuesResponse>,
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
                "/sift.user_attributes.v1.UserAttributesService/ListUserAttributeKeyValues",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sift.user_attributes.v1.UserAttributesService",
                        "ListUserAttributeKeyValues",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_user_attribute_values(
            &mut self,
            request: impl tonic::IntoRequest<super::ListUserAttributeValuesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListUserAttributeValuesResponse>,
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
                "/sift.user_attributes.v1.UserAttributesService/ListUserAttributeValues",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sift.user_attributes.v1.UserAttributesService",
                        "ListUserAttributeValues",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn archive_user_attribute_values(
            &mut self,
            request: impl tonic::IntoRequest<super::ArchiveUserAttributeValuesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ArchiveUserAttributeValuesResponse>,
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
                "/sift.user_attributes.v1.UserAttributesService/ArchiveUserAttributeValues",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sift.user_attributes.v1.UserAttributesService",
                        "ArchiveUserAttributeValues",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn unarchive_user_attribute_values(
            &mut self,
            request: impl tonic::IntoRequest<super::UnarchiveUserAttributeValuesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UnarchiveUserAttributeValuesResponse>,
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
                "/sift.user_attributes.v1.UserAttributesService/UnarchiveUserAttributeValues",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sift.user_attributes.v1.UserAttributesService",
                        "UnarchiveUserAttributeValues",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod user_attributes_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with UserAttributesServiceServer.
    #[async_trait]
    pub trait UserAttributesService: Send + Sync + 'static {
        async fn create_user_attribute_key(
            &self,
            request: tonic::Request<super::CreateUserAttributeKeyRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateUserAttributeKeyResponse>,
            tonic::Status,
        >;
        async fn get_user_attribute_key(
            &self,
            request: tonic::Request<super::GetUserAttributeKeyRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetUserAttributeKeyResponse>,
            tonic::Status,
        >;
        async fn list_user_attribute_keys(
            &self,
            request: tonic::Request<super::ListUserAttributeKeysRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListUserAttributeKeysResponse>,
            tonic::Status,
        >;
        async fn update_user_attribute_key(
            &self,
            request: tonic::Request<super::UpdateUserAttributeKeyRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateUserAttributeKeyResponse>,
            tonic::Status,
        >;
        async fn archive_user_attribute_keys(
            &self,
            request: tonic::Request<super::ArchiveUserAttributeKeysRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ArchiveUserAttributeKeysResponse>,
            tonic::Status,
        >;
        async fn unarchive_user_attribute_keys(
            &self,
            request: tonic::Request<super::UnarchiveUserAttributeKeysRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UnarchiveUserAttributeKeysResponse>,
            tonic::Status,
        >;
        async fn create_user_attribute_value(
            &self,
            request: tonic::Request<super::CreateUserAttributeValueRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateUserAttributeValueResponse>,
            tonic::Status,
        >;
        async fn batch_create_user_attribute_value(
            &self,
            request: tonic::Request<super::BatchCreateUserAttributeValueRequest>,
        ) -> std::result::Result<
            tonic::Response<super::BatchCreateUserAttributeValueResponse>,
            tonic::Status,
        >;
        async fn get_user_attribute_value(
            &self,
            request: tonic::Request<super::GetUserAttributeValueRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetUserAttributeValueResponse>,
            tonic::Status,
        >;
        async fn list_user_attribute_key_values(
            &self,
            request: tonic::Request<super::ListUserAttributeKeyValuesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListUserAttributeKeyValuesResponse>,
            tonic::Status,
        >;
        async fn list_user_attribute_values(
            &self,
            request: tonic::Request<super::ListUserAttributeValuesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListUserAttributeValuesResponse>,
            tonic::Status,
        >;
        async fn archive_user_attribute_values(
            &self,
            request: tonic::Request<super::ArchiveUserAttributeValuesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ArchiveUserAttributeValuesResponse>,
            tonic::Status,
        >;
        async fn unarchive_user_attribute_values(
            &self,
            request: tonic::Request<super::UnarchiveUserAttributeValuesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UnarchiveUserAttributeValuesResponse>,
            tonic::Status,
        >;
    }
    #[derive(Debug)]
    pub struct UserAttributesServiceServer<T: UserAttributesService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: UserAttributesService> UserAttributesServiceServer<T> {
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
    for UserAttributesServiceServer<T>
    where
        T: UserAttributesService,
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
                "/sift.user_attributes.v1.UserAttributesService/CreateUserAttributeKey" => {
                    #[allow(non_camel_case_types)]
                    struct CreateUserAttributeKeySvc<T: UserAttributesService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: UserAttributesService,
                    > tonic::server::UnaryService<super::CreateUserAttributeKeyRequest>
                    for CreateUserAttributeKeySvc<T> {
                        type Response = super::CreateUserAttributeKeyResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateUserAttributeKeyRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as UserAttributesService>::create_user_attribute_key(
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
                        let method = CreateUserAttributeKeySvc(inner);
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
                "/sift.user_attributes.v1.UserAttributesService/GetUserAttributeKey" => {
                    #[allow(non_camel_case_types)]
                    struct GetUserAttributeKeySvc<T: UserAttributesService>(pub Arc<T>);
                    impl<
                        T: UserAttributesService,
                    > tonic::server::UnaryService<super::GetUserAttributeKeyRequest>
                    for GetUserAttributeKeySvc<T> {
                        type Response = super::GetUserAttributeKeyResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetUserAttributeKeyRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as UserAttributesService>::get_user_attribute_key(
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
                        let method = GetUserAttributeKeySvc(inner);
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
                "/sift.user_attributes.v1.UserAttributesService/ListUserAttributeKeys" => {
                    #[allow(non_camel_case_types)]
                    struct ListUserAttributeKeysSvc<T: UserAttributesService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: UserAttributesService,
                    > tonic::server::UnaryService<super::ListUserAttributeKeysRequest>
                    for ListUserAttributeKeysSvc<T> {
                        type Response = super::ListUserAttributeKeysResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListUserAttributeKeysRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as UserAttributesService>::list_user_attribute_keys(
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
                        let method = ListUserAttributeKeysSvc(inner);
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
                "/sift.user_attributes.v1.UserAttributesService/UpdateUserAttributeKey" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateUserAttributeKeySvc<T: UserAttributesService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: UserAttributesService,
                    > tonic::server::UnaryService<super::UpdateUserAttributeKeyRequest>
                    for UpdateUserAttributeKeySvc<T> {
                        type Response = super::UpdateUserAttributeKeyResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdateUserAttributeKeyRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as UserAttributesService>::update_user_attribute_key(
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
                        let method = UpdateUserAttributeKeySvc(inner);
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
                "/sift.user_attributes.v1.UserAttributesService/ArchiveUserAttributeKeys" => {
                    #[allow(non_camel_case_types)]
                    struct ArchiveUserAttributeKeysSvc<T: UserAttributesService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: UserAttributesService,
                    > tonic::server::UnaryService<super::ArchiveUserAttributeKeysRequest>
                    for ArchiveUserAttributeKeysSvc<T> {
                        type Response = super::ArchiveUserAttributeKeysResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::ArchiveUserAttributeKeysRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as UserAttributesService>::archive_user_attribute_keys(
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
                        let method = ArchiveUserAttributeKeysSvc(inner);
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
                "/sift.user_attributes.v1.UserAttributesService/UnarchiveUserAttributeKeys" => {
                    #[allow(non_camel_case_types)]
                    struct UnarchiveUserAttributeKeysSvc<T: UserAttributesService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: UserAttributesService,
                    > tonic::server::UnaryService<
                        super::UnarchiveUserAttributeKeysRequest,
                    > for UnarchiveUserAttributeKeysSvc<T> {
                        type Response = super::UnarchiveUserAttributeKeysResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::UnarchiveUserAttributeKeysRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as UserAttributesService>::unarchive_user_attribute_keys(
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
                        let method = UnarchiveUserAttributeKeysSvc(inner);
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
                "/sift.user_attributes.v1.UserAttributesService/CreateUserAttributeValue" => {
                    #[allow(non_camel_case_types)]
                    struct CreateUserAttributeValueSvc<T: UserAttributesService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: UserAttributesService,
                    > tonic::server::UnaryService<super::CreateUserAttributeValueRequest>
                    for CreateUserAttributeValueSvc<T> {
                        type Response = super::CreateUserAttributeValueResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::CreateUserAttributeValueRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as UserAttributesService>::create_user_attribute_value(
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
                        let method = CreateUserAttributeValueSvc(inner);
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
                "/sift.user_attributes.v1.UserAttributesService/BatchCreateUserAttributeValue" => {
                    #[allow(non_camel_case_types)]
                    struct BatchCreateUserAttributeValueSvc<T: UserAttributesService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: UserAttributesService,
                    > tonic::server::UnaryService<
                        super::BatchCreateUserAttributeValueRequest,
                    > for BatchCreateUserAttributeValueSvc<T> {
                        type Response = super::BatchCreateUserAttributeValueResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::BatchCreateUserAttributeValueRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as UserAttributesService>::batch_create_user_attribute_value(
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
                        let method = BatchCreateUserAttributeValueSvc(inner);
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
                "/sift.user_attributes.v1.UserAttributesService/GetUserAttributeValue" => {
                    #[allow(non_camel_case_types)]
                    struct GetUserAttributeValueSvc<T: UserAttributesService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: UserAttributesService,
                    > tonic::server::UnaryService<super::GetUserAttributeValueRequest>
                    for GetUserAttributeValueSvc<T> {
                        type Response = super::GetUserAttributeValueResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetUserAttributeValueRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as UserAttributesService>::get_user_attribute_value(
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
                        let method = GetUserAttributeValueSvc(inner);
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
                "/sift.user_attributes.v1.UserAttributesService/ListUserAttributeKeyValues" => {
                    #[allow(non_camel_case_types)]
                    struct ListUserAttributeKeyValuesSvc<T: UserAttributesService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: UserAttributesService,
                    > tonic::server::UnaryService<
                        super::ListUserAttributeKeyValuesRequest,
                    > for ListUserAttributeKeyValuesSvc<T> {
                        type Response = super::ListUserAttributeKeyValuesResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::ListUserAttributeKeyValuesRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as UserAttributesService>::list_user_attribute_key_values(
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
                        let method = ListUserAttributeKeyValuesSvc(inner);
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
                "/sift.user_attributes.v1.UserAttributesService/ListUserAttributeValues" => {
                    #[allow(non_camel_case_types)]
                    struct ListUserAttributeValuesSvc<T: UserAttributesService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: UserAttributesService,
                    > tonic::server::UnaryService<super::ListUserAttributeValuesRequest>
                    for ListUserAttributeValuesSvc<T> {
                        type Response = super::ListUserAttributeValuesResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::ListUserAttributeValuesRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as UserAttributesService>::list_user_attribute_values(
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
                        let method = ListUserAttributeValuesSvc(inner);
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
                "/sift.user_attributes.v1.UserAttributesService/ArchiveUserAttributeValues" => {
                    #[allow(non_camel_case_types)]
                    struct ArchiveUserAttributeValuesSvc<T: UserAttributesService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: UserAttributesService,
                    > tonic::server::UnaryService<
                        super::ArchiveUserAttributeValuesRequest,
                    > for ArchiveUserAttributeValuesSvc<T> {
                        type Response = super::ArchiveUserAttributeValuesResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::ArchiveUserAttributeValuesRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as UserAttributesService>::archive_user_attribute_values(
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
                        let method = ArchiveUserAttributeValuesSvc(inner);
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
                "/sift.user_attributes.v1.UserAttributesService/UnarchiveUserAttributeValues" => {
                    #[allow(non_camel_case_types)]
                    struct UnarchiveUserAttributeValuesSvc<T: UserAttributesService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: UserAttributesService,
                    > tonic::server::UnaryService<
                        super::UnarchiveUserAttributeValuesRequest,
                    > for UnarchiveUserAttributeValuesSvc<T> {
                        type Response = super::UnarchiveUserAttributeValuesResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::UnarchiveUserAttributeValuesRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as UserAttributesService>::unarchive_user_attribute_values(
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
                        let method = UnarchiveUserAttributeValuesSvc(inner);
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
    impl<T: UserAttributesService> Clone for UserAttributesServiceServer<T> {
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
    impl<T: UserAttributesService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: UserAttributesService> tonic::server::NamedService
    for UserAttributesServiceServer<T> {
        const NAME: &'static str = "sift.user_attributes.v1.UserAttributesService";
    }
}
