// @generated
/// Generated client implementations.
pub mod user_group_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct UserGroupServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl UserGroupServiceClient<tonic::transport::Channel> {
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
    impl<T> UserGroupServiceClient<T>
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
        ) -> UserGroupServiceClient<InterceptedService<T, F>>
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
            UserGroupServiceClient::new(InterceptedService::new(inner, interceptor))
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
        pub async fn list_user_groups(
            &mut self,
            request: impl tonic::IntoRequest<super::ListUserGroupsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListUserGroupsResponse>,
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
                "/sift.user_groups.v2.UserGroupService/ListUserGroups",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sift.user_groups.v2.UserGroupService",
                        "ListUserGroups",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_user_group(
            &mut self,
            request: impl tonic::IntoRequest<super::GetUserGroupRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetUserGroupResponse>,
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
                "/sift.user_groups.v2.UserGroupService/GetUserGroup",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sift.user_groups.v2.UserGroupService",
                        "GetUserGroup",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn create_user_group(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateUserGroupRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateUserGroupResponse>,
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
                "/sift.user_groups.v2.UserGroupService/CreateUserGroup",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sift.user_groups.v2.UserGroupService",
                        "CreateUserGroup",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn update_user_group(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateUserGroupRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateUserGroupResponse>,
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
                "/sift.user_groups.v2.UserGroupService/UpdateUserGroup",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sift.user_groups.v2.UserGroupService",
                        "UpdateUserGroup",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn delete_user_group(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteUserGroupRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DeleteUserGroupResponse>,
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
                "/sift.user_groups.v2.UserGroupService/DeleteUserGroup",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sift.user_groups.v2.UserGroupService",
                        "DeleteUserGroup",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn add_user_to_user_group(
            &mut self,
            request: impl tonic::IntoRequest<super::AddUserToUserGroupRequest>,
        ) -> std::result::Result<
            tonic::Response<super::AddUserToUserGroupResponse>,
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
                "/sift.user_groups.v2.UserGroupService/AddUserToUserGroup",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sift.user_groups.v2.UserGroupService",
                        "AddUserToUserGroup",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn remove_user_from_user_group(
            &mut self,
            request: impl tonic::IntoRequest<super::RemoveUserFromUserGroupRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RemoveUserFromUserGroupResponse>,
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
                "/sift.user_groups.v2.UserGroupService/RemoveUserFromUserGroup",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sift.user_groups.v2.UserGroupService",
                        "RemoveUserFromUserGroup",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn update_user_user_groups(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateUserUserGroupsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateUserUserGroupsResponse>,
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
                "/sift.user_groups.v2.UserGroupService/UpdateUserUserGroups",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sift.user_groups.v2.UserGroupService",
                        "UpdateUserUserGroups",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_user_groups_for_assets(
            &mut self,
            request: impl tonic::IntoRequest<super::GetUserGroupsForAssetsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetUserGroupsForAssetsResponse>,
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
                "/sift.user_groups.v2.UserGroupService/GetUserGroupsForAssets",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sift.user_groups.v2.UserGroupService",
                        "GetUserGroupsForAssets",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod user_group_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with UserGroupServiceServer.
    #[async_trait]
    pub trait UserGroupService: Send + Sync + 'static {
        async fn list_user_groups(
            &self,
            request: tonic::Request<super::ListUserGroupsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListUserGroupsResponse>,
            tonic::Status,
        >;
        async fn get_user_group(
            &self,
            request: tonic::Request<super::GetUserGroupRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetUserGroupResponse>,
            tonic::Status,
        >;
        async fn create_user_group(
            &self,
            request: tonic::Request<super::CreateUserGroupRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateUserGroupResponse>,
            tonic::Status,
        >;
        async fn update_user_group(
            &self,
            request: tonic::Request<super::UpdateUserGroupRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateUserGroupResponse>,
            tonic::Status,
        >;
        async fn delete_user_group(
            &self,
            request: tonic::Request<super::DeleteUserGroupRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DeleteUserGroupResponse>,
            tonic::Status,
        >;
        async fn add_user_to_user_group(
            &self,
            request: tonic::Request<super::AddUserToUserGroupRequest>,
        ) -> std::result::Result<
            tonic::Response<super::AddUserToUserGroupResponse>,
            tonic::Status,
        >;
        async fn remove_user_from_user_group(
            &self,
            request: tonic::Request<super::RemoveUserFromUserGroupRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RemoveUserFromUserGroupResponse>,
            tonic::Status,
        >;
        async fn update_user_user_groups(
            &self,
            request: tonic::Request<super::UpdateUserUserGroupsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateUserUserGroupsResponse>,
            tonic::Status,
        >;
        async fn get_user_groups_for_assets(
            &self,
            request: tonic::Request<super::GetUserGroupsForAssetsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetUserGroupsForAssetsResponse>,
            tonic::Status,
        >;
    }
    #[derive(Debug)]
    pub struct UserGroupServiceServer<T: UserGroupService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: UserGroupService> UserGroupServiceServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for UserGroupServiceServer<T>
    where
        T: UserGroupService,
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
                "/sift.user_groups.v2.UserGroupService/ListUserGroups" => {
                    #[allow(non_camel_case_types)]
                    struct ListUserGroupsSvc<T: UserGroupService>(pub Arc<T>);
                    impl<
                        T: UserGroupService,
                    > tonic::server::UnaryService<super::ListUserGroupsRequest>
                    for ListUserGroupsSvc<T> {
                        type Response = super::ListUserGroupsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListUserGroupsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as UserGroupService>::list_user_groups(&inner, request)
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
                        let method = ListUserGroupsSvc(inner);
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
                "/sift.user_groups.v2.UserGroupService/GetUserGroup" => {
                    #[allow(non_camel_case_types)]
                    struct GetUserGroupSvc<T: UserGroupService>(pub Arc<T>);
                    impl<
                        T: UserGroupService,
                    > tonic::server::UnaryService<super::GetUserGroupRequest>
                    for GetUserGroupSvc<T> {
                        type Response = super::GetUserGroupResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetUserGroupRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as UserGroupService>::get_user_group(&inner, request)
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
                        let method = GetUserGroupSvc(inner);
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
                "/sift.user_groups.v2.UserGroupService/CreateUserGroup" => {
                    #[allow(non_camel_case_types)]
                    struct CreateUserGroupSvc<T: UserGroupService>(pub Arc<T>);
                    impl<
                        T: UserGroupService,
                    > tonic::server::UnaryService<super::CreateUserGroupRequest>
                    for CreateUserGroupSvc<T> {
                        type Response = super::CreateUserGroupResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateUserGroupRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as UserGroupService>::create_user_group(&inner, request)
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
                        let method = CreateUserGroupSvc(inner);
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
                "/sift.user_groups.v2.UserGroupService/UpdateUserGroup" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateUserGroupSvc<T: UserGroupService>(pub Arc<T>);
                    impl<
                        T: UserGroupService,
                    > tonic::server::UnaryService<super::UpdateUserGroupRequest>
                    for UpdateUserGroupSvc<T> {
                        type Response = super::UpdateUserGroupResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdateUserGroupRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as UserGroupService>::update_user_group(&inner, request)
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
                        let method = UpdateUserGroupSvc(inner);
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
                "/sift.user_groups.v2.UserGroupService/DeleteUserGroup" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteUserGroupSvc<T: UserGroupService>(pub Arc<T>);
                    impl<
                        T: UserGroupService,
                    > tonic::server::UnaryService<super::DeleteUserGroupRequest>
                    for DeleteUserGroupSvc<T> {
                        type Response = super::DeleteUserGroupResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteUserGroupRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as UserGroupService>::delete_user_group(&inner, request)
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
                        let method = DeleteUserGroupSvc(inner);
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
                "/sift.user_groups.v2.UserGroupService/AddUserToUserGroup" => {
                    #[allow(non_camel_case_types)]
                    struct AddUserToUserGroupSvc<T: UserGroupService>(pub Arc<T>);
                    impl<
                        T: UserGroupService,
                    > tonic::server::UnaryService<super::AddUserToUserGroupRequest>
                    for AddUserToUserGroupSvc<T> {
                        type Response = super::AddUserToUserGroupResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::AddUserToUserGroupRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as UserGroupService>::add_user_to_user_group(
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
                        let method = AddUserToUserGroupSvc(inner);
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
                "/sift.user_groups.v2.UserGroupService/RemoveUserFromUserGroup" => {
                    #[allow(non_camel_case_types)]
                    struct RemoveUserFromUserGroupSvc<T: UserGroupService>(pub Arc<T>);
                    impl<
                        T: UserGroupService,
                    > tonic::server::UnaryService<super::RemoveUserFromUserGroupRequest>
                    for RemoveUserFromUserGroupSvc<T> {
                        type Response = super::RemoveUserFromUserGroupResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::RemoveUserFromUserGroupRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as UserGroupService>::remove_user_from_user_group(
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
                        let method = RemoveUserFromUserGroupSvc(inner);
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
                "/sift.user_groups.v2.UserGroupService/UpdateUserUserGroups" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateUserUserGroupsSvc<T: UserGroupService>(pub Arc<T>);
                    impl<
                        T: UserGroupService,
                    > tonic::server::UnaryService<super::UpdateUserUserGroupsRequest>
                    for UpdateUserUserGroupsSvc<T> {
                        type Response = super::UpdateUserUserGroupsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdateUserUserGroupsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as UserGroupService>::update_user_user_groups(
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
                        let method = UpdateUserUserGroupsSvc(inner);
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
                "/sift.user_groups.v2.UserGroupService/GetUserGroupsForAssets" => {
                    #[allow(non_camel_case_types)]
                    struct GetUserGroupsForAssetsSvc<T: UserGroupService>(pub Arc<T>);
                    impl<
                        T: UserGroupService,
                    > tonic::server::UnaryService<super::GetUserGroupsForAssetsRequest>
                    for GetUserGroupsForAssetsSvc<T> {
                        type Response = super::GetUserGroupsForAssetsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetUserGroupsForAssetsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as UserGroupService>::get_user_groups_for_assets(
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
                        let method = GetUserGroupsForAssetsSvc(inner);
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
    impl<T: UserGroupService> Clone for UserGroupServiceServer<T> {
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
    impl<T: UserGroupService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: UserGroupService> tonic::server::NamedService for UserGroupServiceServer<T> {
        const NAME: &'static str = "sift.user_groups.v2.UserGroupService";
    }
}
