// @generated
/// Generated client implementations.
pub mod principal_attribute_service_client {
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
    pub struct PrincipalAttributeServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl PrincipalAttributeServiceClient<tonic::transport::Channel> {
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
    impl<T> PrincipalAttributeServiceClient<T>
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
        ) -> PrincipalAttributeServiceClient<InterceptedService<T, F>>
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
            PrincipalAttributeServiceClient::new(
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
        pub async fn create_principal_attribute_key(
            &mut self,
            request: impl tonic::IntoRequest<super::CreatePrincipalAttributeKeyRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreatePrincipalAttributeKeyResponse>,
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
                "/sift.principal_attributes.v1.PrincipalAttributeService/CreatePrincipalAttributeKey",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sift.principal_attributes.v1.PrincipalAttributeService",
                        "CreatePrincipalAttributeKey",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_principal_attribute_key(
            &mut self,
            request: impl tonic::IntoRequest<super::GetPrincipalAttributeKeyRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetPrincipalAttributeKeyResponse>,
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
                "/sift.principal_attributes.v1.PrincipalAttributeService/GetPrincipalAttributeKey",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sift.principal_attributes.v1.PrincipalAttributeService",
                        "GetPrincipalAttributeKey",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_principal_attribute_keys(
            &mut self,
            request: impl tonic::IntoRequest<super::ListPrincipalAttributeKeysRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListPrincipalAttributeKeysResponse>,
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
                "/sift.principal_attributes.v1.PrincipalAttributeService/ListPrincipalAttributeKeys",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sift.principal_attributes.v1.PrincipalAttributeService",
                        "ListPrincipalAttributeKeys",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn update_principal_attribute_key(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdatePrincipalAttributeKeyRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdatePrincipalAttributeKeyResponse>,
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
                "/sift.principal_attributes.v1.PrincipalAttributeService/UpdatePrincipalAttributeKey",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sift.principal_attributes.v1.PrincipalAttributeService",
                        "UpdatePrincipalAttributeKey",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn check_principal_attribute_key_archive_impact(
            &mut self,
            request: impl tonic::IntoRequest<
                super::CheckPrincipalAttributeKeyArchiveImpactRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::CheckPrincipalAttributeKeyArchiveImpactResponse>,
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
                "/sift.principal_attributes.v1.PrincipalAttributeService/CheckPrincipalAttributeKeyArchiveImpact",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sift.principal_attributes.v1.PrincipalAttributeService",
                        "CheckPrincipalAttributeKeyArchiveImpact",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn archive_principal_attribute_keys(
            &mut self,
            request: impl tonic::IntoRequest<super::ArchivePrincipalAttributeKeysRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ArchivePrincipalAttributeKeysResponse>,
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
                "/sift.principal_attributes.v1.PrincipalAttributeService/ArchivePrincipalAttributeKeys",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sift.principal_attributes.v1.PrincipalAttributeService",
                        "ArchivePrincipalAttributeKeys",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn unarchive_principal_attribute_keys(
            &mut self,
            request: impl tonic::IntoRequest<
                super::UnarchivePrincipalAttributeKeysRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::UnarchivePrincipalAttributeKeysResponse>,
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
                "/sift.principal_attributes.v1.PrincipalAttributeService/UnarchivePrincipalAttributeKeys",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sift.principal_attributes.v1.PrincipalAttributeService",
                        "UnarchivePrincipalAttributeKeys",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn batch_create_principal_attribute_value(
            &mut self,
            request: impl tonic::IntoRequest<
                super::BatchCreatePrincipalAttributeValueRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::BatchCreatePrincipalAttributeValueResponse>,
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
                "/sift.principal_attributes.v1.PrincipalAttributeService/BatchCreatePrincipalAttributeValue",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sift.principal_attributes.v1.PrincipalAttributeService",
                        "BatchCreatePrincipalAttributeValue",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_principal_attribute_value(
            &mut self,
            request: impl tonic::IntoRequest<super::GetPrincipalAttributeValueRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetPrincipalAttributeValueResponse>,
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
                "/sift.principal_attributes.v1.PrincipalAttributeService/GetPrincipalAttributeValue",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sift.principal_attributes.v1.PrincipalAttributeService",
                        "GetPrincipalAttributeValue",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_principal_attribute_key_values(
            &mut self,
            request: impl tonic::IntoRequest<
                super::ListPrincipalAttributeKeyValuesRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::ListPrincipalAttributeKeyValuesResponse>,
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
                "/sift.principal_attributes.v1.PrincipalAttributeService/ListPrincipalAttributeKeyValues",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sift.principal_attributes.v1.PrincipalAttributeService",
                        "ListPrincipalAttributeKeyValues",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_principal_attribute_values(
            &mut self,
            request: impl tonic::IntoRequest<super::ListPrincipalAttributeValuesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListPrincipalAttributeValuesResponse>,
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
                "/sift.principal_attributes.v1.PrincipalAttributeService/ListPrincipalAttributeValues",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sift.principal_attributes.v1.PrincipalAttributeService",
                        "ListPrincipalAttributeValues",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn archive_principal_attribute_values(
            &mut self,
            request: impl tonic::IntoRequest<
                super::ArchivePrincipalAttributeValuesRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::ArchivePrincipalAttributeValuesResponse>,
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
                "/sift.principal_attributes.v1.PrincipalAttributeService/ArchivePrincipalAttributeValues",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sift.principal_attributes.v1.PrincipalAttributeService",
                        "ArchivePrincipalAttributeValues",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn unarchive_principal_attribute_values(
            &mut self,
            request: impl tonic::IntoRequest<
                super::UnarchivePrincipalAttributeValuesRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::UnarchivePrincipalAttributeValuesResponse>,
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
                "/sift.principal_attributes.v1.PrincipalAttributeService/UnarchivePrincipalAttributeValues",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sift.principal_attributes.v1.PrincipalAttributeService",
                        "UnarchivePrincipalAttributeValues",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn create_principal_attribute_enum_value(
            &mut self,
            request: impl tonic::IntoRequest<
                super::CreatePrincipalAttributeEnumValueRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::CreatePrincipalAttributeEnumValueResponse>,
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
                "/sift.principal_attributes.v1.PrincipalAttributeService/CreatePrincipalAttributeEnumValue",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sift.principal_attributes.v1.PrincipalAttributeService",
                        "CreatePrincipalAttributeEnumValue",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_principal_attribute_enum_value(
            &mut self,
            request: impl tonic::IntoRequest<
                super::GetPrincipalAttributeEnumValueRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::GetPrincipalAttributeEnumValueResponse>,
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
                "/sift.principal_attributes.v1.PrincipalAttributeService/GetPrincipalAttributeEnumValue",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sift.principal_attributes.v1.PrincipalAttributeService",
                        "GetPrincipalAttributeEnumValue",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_principal_attribute_enum_values(
            &mut self,
            request: impl tonic::IntoRequest<
                super::ListPrincipalAttributeEnumValuesRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::ListPrincipalAttributeEnumValuesResponse>,
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
                "/sift.principal_attributes.v1.PrincipalAttributeService/ListPrincipalAttributeEnumValues",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sift.principal_attributes.v1.PrincipalAttributeService",
                        "ListPrincipalAttributeEnumValues",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn update_principal_attribute_enum_value(
            &mut self,
            request: impl tonic::IntoRequest<
                super::UpdatePrincipalAttributeEnumValueRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::UpdatePrincipalAttributeEnumValueResponse>,
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
                "/sift.principal_attributes.v1.PrincipalAttributeService/UpdatePrincipalAttributeEnumValue",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sift.principal_attributes.v1.PrincipalAttributeService",
                        "UpdatePrincipalAttributeEnumValue",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn archive_principal_attribute_enum_value(
            &mut self,
            request: impl tonic::IntoRequest<
                super::ArchivePrincipalAttributeEnumValueRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::ArchivePrincipalAttributeEnumValueResponse>,
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
                "/sift.principal_attributes.v1.PrincipalAttributeService/ArchivePrincipalAttributeEnumValue",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sift.principal_attributes.v1.PrincipalAttributeService",
                        "ArchivePrincipalAttributeEnumValue",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn unarchive_principal_attribute_enum_value(
            &mut self,
            request: impl tonic::IntoRequest<
                super::UnarchivePrincipalAttributeEnumValueRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::UnarchivePrincipalAttributeEnumValueResponse>,
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
                "/sift.principal_attributes.v1.PrincipalAttributeService/UnarchivePrincipalAttributeEnumValue",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sift.principal_attributes.v1.PrincipalAttributeService",
                        "UnarchivePrincipalAttributeEnumValue",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn batch_archive_principal_attribute_enum_values(
            &mut self,
            request: impl tonic::IntoRequest<
                super::BatchArchivePrincipalAttributeEnumValuesRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::BatchArchivePrincipalAttributeEnumValuesResponse>,
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
                "/sift.principal_attributes.v1.PrincipalAttributeService/BatchArchivePrincipalAttributeEnumValues",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sift.principal_attributes.v1.PrincipalAttributeService",
                        "BatchArchivePrincipalAttributeEnumValues",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn batch_unarchive_principal_attribute_enum_values(
            &mut self,
            request: impl tonic::IntoRequest<
                super::BatchUnarchivePrincipalAttributeEnumValuesRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::BatchUnarchivePrincipalAttributeEnumValuesResponse>,
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
                "/sift.principal_attributes.v1.PrincipalAttributeService/BatchUnarchivePrincipalAttributeEnumValues",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sift.principal_attributes.v1.PrincipalAttributeService",
                        "BatchUnarchivePrincipalAttributeEnumValues",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod principal_attribute_service_server {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::wildcard_imports,
        clippy::let_unit_value,
    )]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with PrincipalAttributeServiceServer.
    #[async_trait]
    pub trait PrincipalAttributeService: std::marker::Send + std::marker::Sync + 'static {
        async fn create_principal_attribute_key(
            &self,
            request: tonic::Request<super::CreatePrincipalAttributeKeyRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreatePrincipalAttributeKeyResponse>,
            tonic::Status,
        >;
        async fn get_principal_attribute_key(
            &self,
            request: tonic::Request<super::GetPrincipalAttributeKeyRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetPrincipalAttributeKeyResponse>,
            tonic::Status,
        >;
        async fn list_principal_attribute_keys(
            &self,
            request: tonic::Request<super::ListPrincipalAttributeKeysRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListPrincipalAttributeKeysResponse>,
            tonic::Status,
        >;
        async fn update_principal_attribute_key(
            &self,
            request: tonic::Request<super::UpdatePrincipalAttributeKeyRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdatePrincipalAttributeKeyResponse>,
            tonic::Status,
        >;
        async fn check_principal_attribute_key_archive_impact(
            &self,
            request: tonic::Request<
                super::CheckPrincipalAttributeKeyArchiveImpactRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::CheckPrincipalAttributeKeyArchiveImpactResponse>,
            tonic::Status,
        >;
        async fn archive_principal_attribute_keys(
            &self,
            request: tonic::Request<super::ArchivePrincipalAttributeKeysRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ArchivePrincipalAttributeKeysResponse>,
            tonic::Status,
        >;
        async fn unarchive_principal_attribute_keys(
            &self,
            request: tonic::Request<super::UnarchivePrincipalAttributeKeysRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UnarchivePrincipalAttributeKeysResponse>,
            tonic::Status,
        >;
        async fn batch_create_principal_attribute_value(
            &self,
            request: tonic::Request<super::BatchCreatePrincipalAttributeValueRequest>,
        ) -> std::result::Result<
            tonic::Response<super::BatchCreatePrincipalAttributeValueResponse>,
            tonic::Status,
        >;
        async fn get_principal_attribute_value(
            &self,
            request: tonic::Request<super::GetPrincipalAttributeValueRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetPrincipalAttributeValueResponse>,
            tonic::Status,
        >;
        async fn list_principal_attribute_key_values(
            &self,
            request: tonic::Request<super::ListPrincipalAttributeKeyValuesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListPrincipalAttributeKeyValuesResponse>,
            tonic::Status,
        >;
        async fn list_principal_attribute_values(
            &self,
            request: tonic::Request<super::ListPrincipalAttributeValuesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListPrincipalAttributeValuesResponse>,
            tonic::Status,
        >;
        async fn archive_principal_attribute_values(
            &self,
            request: tonic::Request<super::ArchivePrincipalAttributeValuesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ArchivePrincipalAttributeValuesResponse>,
            tonic::Status,
        >;
        async fn unarchive_principal_attribute_values(
            &self,
            request: tonic::Request<super::UnarchivePrincipalAttributeValuesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UnarchivePrincipalAttributeValuesResponse>,
            tonic::Status,
        >;
        async fn create_principal_attribute_enum_value(
            &self,
            request: tonic::Request<super::CreatePrincipalAttributeEnumValueRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreatePrincipalAttributeEnumValueResponse>,
            tonic::Status,
        >;
        async fn get_principal_attribute_enum_value(
            &self,
            request: tonic::Request<super::GetPrincipalAttributeEnumValueRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetPrincipalAttributeEnumValueResponse>,
            tonic::Status,
        >;
        async fn list_principal_attribute_enum_values(
            &self,
            request: tonic::Request<super::ListPrincipalAttributeEnumValuesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListPrincipalAttributeEnumValuesResponse>,
            tonic::Status,
        >;
        async fn update_principal_attribute_enum_value(
            &self,
            request: tonic::Request<super::UpdatePrincipalAttributeEnumValueRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdatePrincipalAttributeEnumValueResponse>,
            tonic::Status,
        >;
        async fn archive_principal_attribute_enum_value(
            &self,
            request: tonic::Request<super::ArchivePrincipalAttributeEnumValueRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ArchivePrincipalAttributeEnumValueResponse>,
            tonic::Status,
        >;
        async fn unarchive_principal_attribute_enum_value(
            &self,
            request: tonic::Request<super::UnarchivePrincipalAttributeEnumValueRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UnarchivePrincipalAttributeEnumValueResponse>,
            tonic::Status,
        >;
        async fn batch_archive_principal_attribute_enum_values(
            &self,
            request: tonic::Request<
                super::BatchArchivePrincipalAttributeEnumValuesRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::BatchArchivePrincipalAttributeEnumValuesResponse>,
            tonic::Status,
        >;
        async fn batch_unarchive_principal_attribute_enum_values(
            &self,
            request: tonic::Request<
                super::BatchUnarchivePrincipalAttributeEnumValuesRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::BatchUnarchivePrincipalAttributeEnumValuesResponse>,
            tonic::Status,
        >;
    }
    #[derive(Debug)]
    pub struct PrincipalAttributeServiceServer<T> {
        inner: Arc<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    impl<T> PrincipalAttributeServiceServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>>
    for PrincipalAttributeServiceServer<T>
    where
        T: PrincipalAttributeService,
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
                "/sift.principal_attributes.v1.PrincipalAttributeService/CreatePrincipalAttributeKey" => {
                    #[allow(non_camel_case_types)]
                    struct CreatePrincipalAttributeKeySvc<T: PrincipalAttributeService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: PrincipalAttributeService,
                    > tonic::server::UnaryService<
                        super::CreatePrincipalAttributeKeyRequest,
                    > for CreatePrincipalAttributeKeySvc<T> {
                        type Response = super::CreatePrincipalAttributeKeyResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::CreatePrincipalAttributeKeyRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as PrincipalAttributeService>::create_principal_attribute_key(
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
                        let method = CreatePrincipalAttributeKeySvc(inner);
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
                "/sift.principal_attributes.v1.PrincipalAttributeService/GetPrincipalAttributeKey" => {
                    #[allow(non_camel_case_types)]
                    struct GetPrincipalAttributeKeySvc<T: PrincipalAttributeService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: PrincipalAttributeService,
                    > tonic::server::UnaryService<super::GetPrincipalAttributeKeyRequest>
                    for GetPrincipalAttributeKeySvc<T> {
                        type Response = super::GetPrincipalAttributeKeyResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::GetPrincipalAttributeKeyRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as PrincipalAttributeService>::get_principal_attribute_key(
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
                        let method = GetPrincipalAttributeKeySvc(inner);
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
                "/sift.principal_attributes.v1.PrincipalAttributeService/ListPrincipalAttributeKeys" => {
                    #[allow(non_camel_case_types)]
                    struct ListPrincipalAttributeKeysSvc<T: PrincipalAttributeService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: PrincipalAttributeService,
                    > tonic::server::UnaryService<
                        super::ListPrincipalAttributeKeysRequest,
                    > for ListPrincipalAttributeKeysSvc<T> {
                        type Response = super::ListPrincipalAttributeKeysResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::ListPrincipalAttributeKeysRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as PrincipalAttributeService>::list_principal_attribute_keys(
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
                        let method = ListPrincipalAttributeKeysSvc(inner);
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
                "/sift.principal_attributes.v1.PrincipalAttributeService/UpdatePrincipalAttributeKey" => {
                    #[allow(non_camel_case_types)]
                    struct UpdatePrincipalAttributeKeySvc<T: PrincipalAttributeService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: PrincipalAttributeService,
                    > tonic::server::UnaryService<
                        super::UpdatePrincipalAttributeKeyRequest,
                    > for UpdatePrincipalAttributeKeySvc<T> {
                        type Response = super::UpdatePrincipalAttributeKeyResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::UpdatePrincipalAttributeKeyRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as PrincipalAttributeService>::update_principal_attribute_key(
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
                        let method = UpdatePrincipalAttributeKeySvc(inner);
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
                "/sift.principal_attributes.v1.PrincipalAttributeService/CheckPrincipalAttributeKeyArchiveImpact" => {
                    #[allow(non_camel_case_types)]
                    struct CheckPrincipalAttributeKeyArchiveImpactSvc<
                        T: PrincipalAttributeService,
                    >(
                        pub Arc<T>,
                    );
                    impl<
                        T: PrincipalAttributeService,
                    > tonic::server::UnaryService<
                        super::CheckPrincipalAttributeKeyArchiveImpactRequest,
                    > for CheckPrincipalAttributeKeyArchiveImpactSvc<T> {
                        type Response = super::CheckPrincipalAttributeKeyArchiveImpactResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::CheckPrincipalAttributeKeyArchiveImpactRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as PrincipalAttributeService>::check_principal_attribute_key_archive_impact(
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
                        let method = CheckPrincipalAttributeKeyArchiveImpactSvc(inner);
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
                "/sift.principal_attributes.v1.PrincipalAttributeService/ArchivePrincipalAttributeKeys" => {
                    #[allow(non_camel_case_types)]
                    struct ArchivePrincipalAttributeKeysSvc<
                        T: PrincipalAttributeService,
                    >(
                        pub Arc<T>,
                    );
                    impl<
                        T: PrincipalAttributeService,
                    > tonic::server::UnaryService<
                        super::ArchivePrincipalAttributeKeysRequest,
                    > for ArchivePrincipalAttributeKeysSvc<T> {
                        type Response = super::ArchivePrincipalAttributeKeysResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::ArchivePrincipalAttributeKeysRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as PrincipalAttributeService>::archive_principal_attribute_keys(
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
                        let method = ArchivePrincipalAttributeKeysSvc(inner);
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
                "/sift.principal_attributes.v1.PrincipalAttributeService/UnarchivePrincipalAttributeKeys" => {
                    #[allow(non_camel_case_types)]
                    struct UnarchivePrincipalAttributeKeysSvc<
                        T: PrincipalAttributeService,
                    >(
                        pub Arc<T>,
                    );
                    impl<
                        T: PrincipalAttributeService,
                    > tonic::server::UnaryService<
                        super::UnarchivePrincipalAttributeKeysRequest,
                    > for UnarchivePrincipalAttributeKeysSvc<T> {
                        type Response = super::UnarchivePrincipalAttributeKeysResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::UnarchivePrincipalAttributeKeysRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as PrincipalAttributeService>::unarchive_principal_attribute_keys(
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
                        let method = UnarchivePrincipalAttributeKeysSvc(inner);
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
                "/sift.principal_attributes.v1.PrincipalAttributeService/BatchCreatePrincipalAttributeValue" => {
                    #[allow(non_camel_case_types)]
                    struct BatchCreatePrincipalAttributeValueSvc<
                        T: PrincipalAttributeService,
                    >(
                        pub Arc<T>,
                    );
                    impl<
                        T: PrincipalAttributeService,
                    > tonic::server::UnaryService<
                        super::BatchCreatePrincipalAttributeValueRequest,
                    > for BatchCreatePrincipalAttributeValueSvc<T> {
                        type Response = super::BatchCreatePrincipalAttributeValueResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::BatchCreatePrincipalAttributeValueRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as PrincipalAttributeService>::batch_create_principal_attribute_value(
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
                        let method = BatchCreatePrincipalAttributeValueSvc(inner);
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
                "/sift.principal_attributes.v1.PrincipalAttributeService/GetPrincipalAttributeValue" => {
                    #[allow(non_camel_case_types)]
                    struct GetPrincipalAttributeValueSvc<T: PrincipalAttributeService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: PrincipalAttributeService,
                    > tonic::server::UnaryService<
                        super::GetPrincipalAttributeValueRequest,
                    > for GetPrincipalAttributeValueSvc<T> {
                        type Response = super::GetPrincipalAttributeValueResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::GetPrincipalAttributeValueRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as PrincipalAttributeService>::get_principal_attribute_value(
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
                        let method = GetPrincipalAttributeValueSvc(inner);
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
                "/sift.principal_attributes.v1.PrincipalAttributeService/ListPrincipalAttributeKeyValues" => {
                    #[allow(non_camel_case_types)]
                    struct ListPrincipalAttributeKeyValuesSvc<
                        T: PrincipalAttributeService,
                    >(
                        pub Arc<T>,
                    );
                    impl<
                        T: PrincipalAttributeService,
                    > tonic::server::UnaryService<
                        super::ListPrincipalAttributeKeyValuesRequest,
                    > for ListPrincipalAttributeKeyValuesSvc<T> {
                        type Response = super::ListPrincipalAttributeKeyValuesResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::ListPrincipalAttributeKeyValuesRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as PrincipalAttributeService>::list_principal_attribute_key_values(
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
                        let method = ListPrincipalAttributeKeyValuesSvc(inner);
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
                "/sift.principal_attributes.v1.PrincipalAttributeService/ListPrincipalAttributeValues" => {
                    #[allow(non_camel_case_types)]
                    struct ListPrincipalAttributeValuesSvc<T: PrincipalAttributeService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: PrincipalAttributeService,
                    > tonic::server::UnaryService<
                        super::ListPrincipalAttributeValuesRequest,
                    > for ListPrincipalAttributeValuesSvc<T> {
                        type Response = super::ListPrincipalAttributeValuesResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::ListPrincipalAttributeValuesRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as PrincipalAttributeService>::list_principal_attribute_values(
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
                        let method = ListPrincipalAttributeValuesSvc(inner);
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
                "/sift.principal_attributes.v1.PrincipalAttributeService/ArchivePrincipalAttributeValues" => {
                    #[allow(non_camel_case_types)]
                    struct ArchivePrincipalAttributeValuesSvc<
                        T: PrincipalAttributeService,
                    >(
                        pub Arc<T>,
                    );
                    impl<
                        T: PrincipalAttributeService,
                    > tonic::server::UnaryService<
                        super::ArchivePrincipalAttributeValuesRequest,
                    > for ArchivePrincipalAttributeValuesSvc<T> {
                        type Response = super::ArchivePrincipalAttributeValuesResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::ArchivePrincipalAttributeValuesRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as PrincipalAttributeService>::archive_principal_attribute_values(
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
                        let method = ArchivePrincipalAttributeValuesSvc(inner);
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
                "/sift.principal_attributes.v1.PrincipalAttributeService/UnarchivePrincipalAttributeValues" => {
                    #[allow(non_camel_case_types)]
                    struct UnarchivePrincipalAttributeValuesSvc<
                        T: PrincipalAttributeService,
                    >(
                        pub Arc<T>,
                    );
                    impl<
                        T: PrincipalAttributeService,
                    > tonic::server::UnaryService<
                        super::UnarchivePrincipalAttributeValuesRequest,
                    > for UnarchivePrincipalAttributeValuesSvc<T> {
                        type Response = super::UnarchivePrincipalAttributeValuesResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::UnarchivePrincipalAttributeValuesRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as PrincipalAttributeService>::unarchive_principal_attribute_values(
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
                        let method = UnarchivePrincipalAttributeValuesSvc(inner);
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
                "/sift.principal_attributes.v1.PrincipalAttributeService/CreatePrincipalAttributeEnumValue" => {
                    #[allow(non_camel_case_types)]
                    struct CreatePrincipalAttributeEnumValueSvc<
                        T: PrincipalAttributeService,
                    >(
                        pub Arc<T>,
                    );
                    impl<
                        T: PrincipalAttributeService,
                    > tonic::server::UnaryService<
                        super::CreatePrincipalAttributeEnumValueRequest,
                    > for CreatePrincipalAttributeEnumValueSvc<T> {
                        type Response = super::CreatePrincipalAttributeEnumValueResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::CreatePrincipalAttributeEnumValueRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as PrincipalAttributeService>::create_principal_attribute_enum_value(
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
                        let method = CreatePrincipalAttributeEnumValueSvc(inner);
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
                "/sift.principal_attributes.v1.PrincipalAttributeService/GetPrincipalAttributeEnumValue" => {
                    #[allow(non_camel_case_types)]
                    struct GetPrincipalAttributeEnumValueSvc<
                        T: PrincipalAttributeService,
                    >(
                        pub Arc<T>,
                    );
                    impl<
                        T: PrincipalAttributeService,
                    > tonic::server::UnaryService<
                        super::GetPrincipalAttributeEnumValueRequest,
                    > for GetPrincipalAttributeEnumValueSvc<T> {
                        type Response = super::GetPrincipalAttributeEnumValueResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::GetPrincipalAttributeEnumValueRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as PrincipalAttributeService>::get_principal_attribute_enum_value(
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
                        let method = GetPrincipalAttributeEnumValueSvc(inner);
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
                "/sift.principal_attributes.v1.PrincipalAttributeService/ListPrincipalAttributeEnumValues" => {
                    #[allow(non_camel_case_types)]
                    struct ListPrincipalAttributeEnumValuesSvc<
                        T: PrincipalAttributeService,
                    >(
                        pub Arc<T>,
                    );
                    impl<
                        T: PrincipalAttributeService,
                    > tonic::server::UnaryService<
                        super::ListPrincipalAttributeEnumValuesRequest,
                    > for ListPrincipalAttributeEnumValuesSvc<T> {
                        type Response = super::ListPrincipalAttributeEnumValuesResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::ListPrincipalAttributeEnumValuesRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as PrincipalAttributeService>::list_principal_attribute_enum_values(
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
                        let method = ListPrincipalAttributeEnumValuesSvc(inner);
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
                "/sift.principal_attributes.v1.PrincipalAttributeService/UpdatePrincipalAttributeEnumValue" => {
                    #[allow(non_camel_case_types)]
                    struct UpdatePrincipalAttributeEnumValueSvc<
                        T: PrincipalAttributeService,
                    >(
                        pub Arc<T>,
                    );
                    impl<
                        T: PrincipalAttributeService,
                    > tonic::server::UnaryService<
                        super::UpdatePrincipalAttributeEnumValueRequest,
                    > for UpdatePrincipalAttributeEnumValueSvc<T> {
                        type Response = super::UpdatePrincipalAttributeEnumValueResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::UpdatePrincipalAttributeEnumValueRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as PrincipalAttributeService>::update_principal_attribute_enum_value(
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
                        let method = UpdatePrincipalAttributeEnumValueSvc(inner);
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
                "/sift.principal_attributes.v1.PrincipalAttributeService/ArchivePrincipalAttributeEnumValue" => {
                    #[allow(non_camel_case_types)]
                    struct ArchivePrincipalAttributeEnumValueSvc<
                        T: PrincipalAttributeService,
                    >(
                        pub Arc<T>,
                    );
                    impl<
                        T: PrincipalAttributeService,
                    > tonic::server::UnaryService<
                        super::ArchivePrincipalAttributeEnumValueRequest,
                    > for ArchivePrincipalAttributeEnumValueSvc<T> {
                        type Response = super::ArchivePrincipalAttributeEnumValueResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::ArchivePrincipalAttributeEnumValueRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as PrincipalAttributeService>::archive_principal_attribute_enum_value(
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
                        let method = ArchivePrincipalAttributeEnumValueSvc(inner);
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
                "/sift.principal_attributes.v1.PrincipalAttributeService/UnarchivePrincipalAttributeEnumValue" => {
                    #[allow(non_camel_case_types)]
                    struct UnarchivePrincipalAttributeEnumValueSvc<
                        T: PrincipalAttributeService,
                    >(
                        pub Arc<T>,
                    );
                    impl<
                        T: PrincipalAttributeService,
                    > tonic::server::UnaryService<
                        super::UnarchivePrincipalAttributeEnumValueRequest,
                    > for UnarchivePrincipalAttributeEnumValueSvc<T> {
                        type Response = super::UnarchivePrincipalAttributeEnumValueResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::UnarchivePrincipalAttributeEnumValueRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as PrincipalAttributeService>::unarchive_principal_attribute_enum_value(
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
                        let method = UnarchivePrincipalAttributeEnumValueSvc(inner);
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
                "/sift.principal_attributes.v1.PrincipalAttributeService/BatchArchivePrincipalAttributeEnumValues" => {
                    #[allow(non_camel_case_types)]
                    struct BatchArchivePrincipalAttributeEnumValuesSvc<
                        T: PrincipalAttributeService,
                    >(
                        pub Arc<T>,
                    );
                    impl<
                        T: PrincipalAttributeService,
                    > tonic::server::UnaryService<
                        super::BatchArchivePrincipalAttributeEnumValuesRequest,
                    > for BatchArchivePrincipalAttributeEnumValuesSvc<T> {
                        type Response = super::BatchArchivePrincipalAttributeEnumValuesResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::BatchArchivePrincipalAttributeEnumValuesRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as PrincipalAttributeService>::batch_archive_principal_attribute_enum_values(
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
                        let method = BatchArchivePrincipalAttributeEnumValuesSvc(inner);
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
                "/sift.principal_attributes.v1.PrincipalAttributeService/BatchUnarchivePrincipalAttributeEnumValues" => {
                    #[allow(non_camel_case_types)]
                    struct BatchUnarchivePrincipalAttributeEnumValuesSvc<
                        T: PrincipalAttributeService,
                    >(
                        pub Arc<T>,
                    );
                    impl<
                        T: PrincipalAttributeService,
                    > tonic::server::UnaryService<
                        super::BatchUnarchivePrincipalAttributeEnumValuesRequest,
                    > for BatchUnarchivePrincipalAttributeEnumValuesSvc<T> {
                        type Response = super::BatchUnarchivePrincipalAttributeEnumValuesResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::BatchUnarchivePrincipalAttributeEnumValuesRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as PrincipalAttributeService>::batch_unarchive_principal_attribute_enum_values(
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
                        let method = BatchUnarchivePrincipalAttributeEnumValuesSvc(
                            inner,
                        );
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
    impl<T> Clone for PrincipalAttributeServiceServer<T> {
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
    pub const SERVICE_NAME: &str = "sift.principal_attributes.v1.PrincipalAttributeService";
    impl<T> tonic::server::NamedService for PrincipalAttributeServiceServer<T> {
        const NAME: &'static str = SERVICE_NAME;
    }
}
