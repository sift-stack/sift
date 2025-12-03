// @generated
/// Generated client implementations.
pub mod resource_attribute_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct ResourceAttributeServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl ResourceAttributeServiceClient<tonic::transport::Channel> {
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
    impl<T> ResourceAttributeServiceClient<T>
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
        ) -> ResourceAttributeServiceClient<InterceptedService<T, F>>
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
            ResourceAttributeServiceClient::new(
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
        pub async fn create_resource_attribute_key(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateResourceAttributeKeyRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateResourceAttributeKeyResponse>,
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
                "/sift.resource_attribute.v1.ResourceAttributeService/CreateResourceAttributeKey",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sift.resource_attribute.v1.ResourceAttributeService",
                        "CreateResourceAttributeKey",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_resource_attribute_key(
            &mut self,
            request: impl tonic::IntoRequest<super::GetResourceAttributeKeyRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetResourceAttributeKeyResponse>,
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
                "/sift.resource_attribute.v1.ResourceAttributeService/GetResourceAttributeKey",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sift.resource_attribute.v1.ResourceAttributeService",
                        "GetResourceAttributeKey",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_resource_attribute_keys(
            &mut self,
            request: impl tonic::IntoRequest<super::ListResourceAttributeKeysRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListResourceAttributeKeysResponse>,
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
                "/sift.resource_attribute.v1.ResourceAttributeService/ListResourceAttributeKeys",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sift.resource_attribute.v1.ResourceAttributeService",
                        "ListResourceAttributeKeys",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn update_resource_attribute_key(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateResourceAttributeKeyRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateResourceAttributeKeyResponse>,
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
                "/sift.resource_attribute.v1.ResourceAttributeService/UpdateResourceAttributeKey",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sift.resource_attribute.v1.ResourceAttributeService",
                        "UpdateResourceAttributeKey",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn archive_resource_attribute_key(
            &mut self,
            request: impl tonic::IntoRequest<super::ArchiveResourceAttributeKeyRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ArchiveResourceAttributeKeyResponse>,
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
                "/sift.resource_attribute.v1.ResourceAttributeService/ArchiveResourceAttributeKey",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sift.resource_attribute.v1.ResourceAttributeService",
                        "ArchiveResourceAttributeKey",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn unarchive_resource_attribute_key(
            &mut self,
            request: impl tonic::IntoRequest<super::UnarchiveResourceAttributeKeyRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UnarchiveResourceAttributeKeyResponse>,
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
                "/sift.resource_attribute.v1.ResourceAttributeService/UnarchiveResourceAttributeKey",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sift.resource_attribute.v1.ResourceAttributeService",
                        "UnarchiveResourceAttributeKey",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn batch_archive_resource_attribute_keys(
            &mut self,
            request: impl tonic::IntoRequest<
                super::BatchArchiveResourceAttributeKeysRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::BatchArchiveResourceAttributeKeysResponse>,
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
                "/sift.resource_attribute.v1.ResourceAttributeService/BatchArchiveResourceAttributeKeys",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sift.resource_attribute.v1.ResourceAttributeService",
                        "BatchArchiveResourceAttributeKeys",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn batch_unarchive_resource_attribute_keys(
            &mut self,
            request: impl tonic::IntoRequest<
                super::BatchUnarchiveResourceAttributeKeysRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::BatchUnarchiveResourceAttributeKeysResponse>,
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
                "/sift.resource_attribute.v1.ResourceAttributeService/BatchUnarchiveResourceAttributeKeys",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sift.resource_attribute.v1.ResourceAttributeService",
                        "BatchUnarchiveResourceAttributeKeys",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn create_resource_attribute_enum_value(
            &mut self,
            request: impl tonic::IntoRequest<
                super::CreateResourceAttributeEnumValueRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::CreateResourceAttributeEnumValueResponse>,
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
                "/sift.resource_attribute.v1.ResourceAttributeService/CreateResourceAttributeEnumValue",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sift.resource_attribute.v1.ResourceAttributeService",
                        "CreateResourceAttributeEnumValue",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_resource_attribute_enum_value(
            &mut self,
            request: impl tonic::IntoRequest<super::GetResourceAttributeEnumValueRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetResourceAttributeEnumValueResponse>,
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
                "/sift.resource_attribute.v1.ResourceAttributeService/GetResourceAttributeEnumValue",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sift.resource_attribute.v1.ResourceAttributeService",
                        "GetResourceAttributeEnumValue",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_resource_attribute_enum_values(
            &mut self,
            request: impl tonic::IntoRequest<
                super::ListResourceAttributeEnumValuesRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::ListResourceAttributeEnumValuesResponse>,
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
                "/sift.resource_attribute.v1.ResourceAttributeService/ListResourceAttributeEnumValues",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sift.resource_attribute.v1.ResourceAttributeService",
                        "ListResourceAttributeEnumValues",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn update_resource_attribute_enum_value(
            &mut self,
            request: impl tonic::IntoRequest<
                super::UpdateResourceAttributeEnumValueRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::UpdateResourceAttributeEnumValueResponse>,
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
                "/sift.resource_attribute.v1.ResourceAttributeService/UpdateResourceAttributeEnumValue",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sift.resource_attribute.v1.ResourceAttributeService",
                        "UpdateResourceAttributeEnumValue",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn archive_resource_attribute_enum_value(
            &mut self,
            request: impl tonic::IntoRequest<
                super::ArchiveResourceAttributeEnumValueRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::ArchiveResourceAttributeEnumValueResponse>,
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
                "/sift.resource_attribute.v1.ResourceAttributeService/ArchiveResourceAttributeEnumValue",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sift.resource_attribute.v1.ResourceAttributeService",
                        "ArchiveResourceAttributeEnumValue",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn unarchive_resource_attribute_enum_value(
            &mut self,
            request: impl tonic::IntoRequest<
                super::UnarchiveResourceAttributeEnumValueRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::UnarchiveResourceAttributeEnumValueResponse>,
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
                "/sift.resource_attribute.v1.ResourceAttributeService/UnarchiveResourceAttributeEnumValue",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sift.resource_attribute.v1.ResourceAttributeService",
                        "UnarchiveResourceAttributeEnumValue",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn batch_archive_resource_attribute_enum_values(
            &mut self,
            request: impl tonic::IntoRequest<
                super::BatchArchiveResourceAttributeEnumValuesRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::BatchArchiveResourceAttributeEnumValuesResponse>,
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
                "/sift.resource_attribute.v1.ResourceAttributeService/BatchArchiveResourceAttributeEnumValues",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sift.resource_attribute.v1.ResourceAttributeService",
                        "BatchArchiveResourceAttributeEnumValues",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn batch_unarchive_resource_attribute_enum_values(
            &mut self,
            request: impl tonic::IntoRequest<
                super::BatchUnarchiveResourceAttributeEnumValuesRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::BatchUnarchiveResourceAttributeEnumValuesResponse>,
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
                "/sift.resource_attribute.v1.ResourceAttributeService/BatchUnarchiveResourceAttributeEnumValues",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sift.resource_attribute.v1.ResourceAttributeService",
                        "BatchUnarchiveResourceAttributeEnumValues",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn create_resource_attribute(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateResourceAttributeRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateResourceAttributeResponse>,
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
                "/sift.resource_attribute.v1.ResourceAttributeService/CreateResourceAttribute",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sift.resource_attribute.v1.ResourceAttributeService",
                        "CreateResourceAttribute",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn batch_create_resource_attributes(
            &mut self,
            request: impl tonic::IntoRequest<super::BatchCreateResourceAttributesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::BatchCreateResourceAttributesResponse>,
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
                "/sift.resource_attribute.v1.ResourceAttributeService/BatchCreateResourceAttributes",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sift.resource_attribute.v1.ResourceAttributeService",
                        "BatchCreateResourceAttributes",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_resource_attribute(
            &mut self,
            request: impl tonic::IntoRequest<super::GetResourceAttributeRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetResourceAttributeResponse>,
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
                "/sift.resource_attribute.v1.ResourceAttributeService/GetResourceAttribute",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sift.resource_attribute.v1.ResourceAttributeService",
                        "GetResourceAttribute",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_resource_attributes(
            &mut self,
            request: impl tonic::IntoRequest<super::ListResourceAttributesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListResourceAttributesResponse>,
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
                "/sift.resource_attribute.v1.ResourceAttributeService/ListResourceAttributes",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sift.resource_attribute.v1.ResourceAttributeService",
                        "ListResourceAttributes",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_resource_attributes_by_entity(
            &mut self,
            request: impl tonic::IntoRequest<
                super::ListResourceAttributesByEntityRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::ListResourceAttributesByEntityResponse>,
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
                "/sift.resource_attribute.v1.ResourceAttributeService/ListResourceAttributesByEntity",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sift.resource_attribute.v1.ResourceAttributeService",
                        "ListResourceAttributesByEntity",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn archive_resource_attribute(
            &mut self,
            request: impl tonic::IntoRequest<super::ArchiveResourceAttributeRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ArchiveResourceAttributeResponse>,
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
                "/sift.resource_attribute.v1.ResourceAttributeService/ArchiveResourceAttribute",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sift.resource_attribute.v1.ResourceAttributeService",
                        "ArchiveResourceAttribute",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn unarchive_resource_attribute(
            &mut self,
            request: impl tonic::IntoRequest<super::UnarchiveResourceAttributeRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UnarchiveResourceAttributeResponse>,
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
                "/sift.resource_attribute.v1.ResourceAttributeService/UnarchiveResourceAttribute",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sift.resource_attribute.v1.ResourceAttributeService",
                        "UnarchiveResourceAttribute",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn batch_archive_resource_attributes(
            &mut self,
            request: impl tonic::IntoRequest<
                super::BatchArchiveResourceAttributesRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::BatchArchiveResourceAttributesResponse>,
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
                "/sift.resource_attribute.v1.ResourceAttributeService/BatchArchiveResourceAttributes",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sift.resource_attribute.v1.ResourceAttributeService",
                        "BatchArchiveResourceAttributes",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn batch_unarchive_resource_attributes(
            &mut self,
            request: impl tonic::IntoRequest<
                super::BatchUnarchiveResourceAttributesRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::BatchUnarchiveResourceAttributesResponse>,
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
                "/sift.resource_attribute.v1.ResourceAttributeService/BatchUnarchiveResourceAttributes",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sift.resource_attribute.v1.ResourceAttributeService",
                        "BatchUnarchiveResourceAttributes",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod resource_attribute_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with ResourceAttributeServiceServer.
    #[async_trait]
    pub trait ResourceAttributeService: Send + Sync + 'static {
        async fn create_resource_attribute_key(
            &self,
            request: tonic::Request<super::CreateResourceAttributeKeyRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateResourceAttributeKeyResponse>,
            tonic::Status,
        >;
        async fn get_resource_attribute_key(
            &self,
            request: tonic::Request<super::GetResourceAttributeKeyRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetResourceAttributeKeyResponse>,
            tonic::Status,
        >;
        async fn list_resource_attribute_keys(
            &self,
            request: tonic::Request<super::ListResourceAttributeKeysRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListResourceAttributeKeysResponse>,
            tonic::Status,
        >;
        async fn update_resource_attribute_key(
            &self,
            request: tonic::Request<super::UpdateResourceAttributeKeyRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateResourceAttributeKeyResponse>,
            tonic::Status,
        >;
        async fn archive_resource_attribute_key(
            &self,
            request: tonic::Request<super::ArchiveResourceAttributeKeyRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ArchiveResourceAttributeKeyResponse>,
            tonic::Status,
        >;
        async fn unarchive_resource_attribute_key(
            &self,
            request: tonic::Request<super::UnarchiveResourceAttributeKeyRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UnarchiveResourceAttributeKeyResponse>,
            tonic::Status,
        >;
        async fn batch_archive_resource_attribute_keys(
            &self,
            request: tonic::Request<super::BatchArchiveResourceAttributeKeysRequest>,
        ) -> std::result::Result<
            tonic::Response<super::BatchArchiveResourceAttributeKeysResponse>,
            tonic::Status,
        >;
        async fn batch_unarchive_resource_attribute_keys(
            &self,
            request: tonic::Request<super::BatchUnarchiveResourceAttributeKeysRequest>,
        ) -> std::result::Result<
            tonic::Response<super::BatchUnarchiveResourceAttributeKeysResponse>,
            tonic::Status,
        >;
        async fn create_resource_attribute_enum_value(
            &self,
            request: tonic::Request<super::CreateResourceAttributeEnumValueRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateResourceAttributeEnumValueResponse>,
            tonic::Status,
        >;
        async fn get_resource_attribute_enum_value(
            &self,
            request: tonic::Request<super::GetResourceAttributeEnumValueRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetResourceAttributeEnumValueResponse>,
            tonic::Status,
        >;
        async fn list_resource_attribute_enum_values(
            &self,
            request: tonic::Request<super::ListResourceAttributeEnumValuesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListResourceAttributeEnumValuesResponse>,
            tonic::Status,
        >;
        async fn update_resource_attribute_enum_value(
            &self,
            request: tonic::Request<super::UpdateResourceAttributeEnumValueRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateResourceAttributeEnumValueResponse>,
            tonic::Status,
        >;
        async fn archive_resource_attribute_enum_value(
            &self,
            request: tonic::Request<super::ArchiveResourceAttributeEnumValueRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ArchiveResourceAttributeEnumValueResponse>,
            tonic::Status,
        >;
        async fn unarchive_resource_attribute_enum_value(
            &self,
            request: tonic::Request<super::UnarchiveResourceAttributeEnumValueRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UnarchiveResourceAttributeEnumValueResponse>,
            tonic::Status,
        >;
        async fn batch_archive_resource_attribute_enum_values(
            &self,
            request: tonic::Request<
                super::BatchArchiveResourceAttributeEnumValuesRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::BatchArchiveResourceAttributeEnumValuesResponse>,
            tonic::Status,
        >;
        async fn batch_unarchive_resource_attribute_enum_values(
            &self,
            request: tonic::Request<
                super::BatchUnarchiveResourceAttributeEnumValuesRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::BatchUnarchiveResourceAttributeEnumValuesResponse>,
            tonic::Status,
        >;
        async fn create_resource_attribute(
            &self,
            request: tonic::Request<super::CreateResourceAttributeRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateResourceAttributeResponse>,
            tonic::Status,
        >;
        async fn batch_create_resource_attributes(
            &self,
            request: tonic::Request<super::BatchCreateResourceAttributesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::BatchCreateResourceAttributesResponse>,
            tonic::Status,
        >;
        async fn get_resource_attribute(
            &self,
            request: tonic::Request<super::GetResourceAttributeRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetResourceAttributeResponse>,
            tonic::Status,
        >;
        async fn list_resource_attributes(
            &self,
            request: tonic::Request<super::ListResourceAttributesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListResourceAttributesResponse>,
            tonic::Status,
        >;
        async fn list_resource_attributes_by_entity(
            &self,
            request: tonic::Request<super::ListResourceAttributesByEntityRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListResourceAttributesByEntityResponse>,
            tonic::Status,
        >;
        async fn archive_resource_attribute(
            &self,
            request: tonic::Request<super::ArchiveResourceAttributeRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ArchiveResourceAttributeResponse>,
            tonic::Status,
        >;
        async fn unarchive_resource_attribute(
            &self,
            request: tonic::Request<super::UnarchiveResourceAttributeRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UnarchiveResourceAttributeResponse>,
            tonic::Status,
        >;
        async fn batch_archive_resource_attributes(
            &self,
            request: tonic::Request<super::BatchArchiveResourceAttributesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::BatchArchiveResourceAttributesResponse>,
            tonic::Status,
        >;
        async fn batch_unarchive_resource_attributes(
            &self,
            request: tonic::Request<super::BatchUnarchiveResourceAttributesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::BatchUnarchiveResourceAttributesResponse>,
            tonic::Status,
        >;
    }
    #[derive(Debug)]
    pub struct ResourceAttributeServiceServer<T: ResourceAttributeService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: ResourceAttributeService> ResourceAttributeServiceServer<T> {
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
    for ResourceAttributeServiceServer<T>
    where
        T: ResourceAttributeService,
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
                "/sift.resource_attribute.v1.ResourceAttributeService/CreateResourceAttributeKey" => {
                    #[allow(non_camel_case_types)]
                    struct CreateResourceAttributeKeySvc<T: ResourceAttributeService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: ResourceAttributeService,
                    > tonic::server::UnaryService<
                        super::CreateResourceAttributeKeyRequest,
                    > for CreateResourceAttributeKeySvc<T> {
                        type Response = super::CreateResourceAttributeKeyResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::CreateResourceAttributeKeyRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ResourceAttributeService>::create_resource_attribute_key(
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
                        let method = CreateResourceAttributeKeySvc(inner);
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
                "/sift.resource_attribute.v1.ResourceAttributeService/GetResourceAttributeKey" => {
                    #[allow(non_camel_case_types)]
                    struct GetResourceAttributeKeySvc<T: ResourceAttributeService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: ResourceAttributeService,
                    > tonic::server::UnaryService<super::GetResourceAttributeKeyRequest>
                    for GetResourceAttributeKeySvc<T> {
                        type Response = super::GetResourceAttributeKeyResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::GetResourceAttributeKeyRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ResourceAttributeService>::get_resource_attribute_key(
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
                        let method = GetResourceAttributeKeySvc(inner);
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
                "/sift.resource_attribute.v1.ResourceAttributeService/ListResourceAttributeKeys" => {
                    #[allow(non_camel_case_types)]
                    struct ListResourceAttributeKeysSvc<T: ResourceAttributeService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: ResourceAttributeService,
                    > tonic::server::UnaryService<
                        super::ListResourceAttributeKeysRequest,
                    > for ListResourceAttributeKeysSvc<T> {
                        type Response = super::ListResourceAttributeKeysResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::ListResourceAttributeKeysRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ResourceAttributeService>::list_resource_attribute_keys(
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
                        let method = ListResourceAttributeKeysSvc(inner);
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
                "/sift.resource_attribute.v1.ResourceAttributeService/UpdateResourceAttributeKey" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateResourceAttributeKeySvc<T: ResourceAttributeService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: ResourceAttributeService,
                    > tonic::server::UnaryService<
                        super::UpdateResourceAttributeKeyRequest,
                    > for UpdateResourceAttributeKeySvc<T> {
                        type Response = super::UpdateResourceAttributeKeyResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::UpdateResourceAttributeKeyRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ResourceAttributeService>::update_resource_attribute_key(
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
                        let method = UpdateResourceAttributeKeySvc(inner);
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
                "/sift.resource_attribute.v1.ResourceAttributeService/ArchiveResourceAttributeKey" => {
                    #[allow(non_camel_case_types)]
                    struct ArchiveResourceAttributeKeySvc<T: ResourceAttributeService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: ResourceAttributeService,
                    > tonic::server::UnaryService<
                        super::ArchiveResourceAttributeKeyRequest,
                    > for ArchiveResourceAttributeKeySvc<T> {
                        type Response = super::ArchiveResourceAttributeKeyResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::ArchiveResourceAttributeKeyRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ResourceAttributeService>::archive_resource_attribute_key(
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
                        let method = ArchiveResourceAttributeKeySvc(inner);
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
                "/sift.resource_attribute.v1.ResourceAttributeService/UnarchiveResourceAttributeKey" => {
                    #[allow(non_camel_case_types)]
                    struct UnarchiveResourceAttributeKeySvc<T: ResourceAttributeService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: ResourceAttributeService,
                    > tonic::server::UnaryService<
                        super::UnarchiveResourceAttributeKeyRequest,
                    > for UnarchiveResourceAttributeKeySvc<T> {
                        type Response = super::UnarchiveResourceAttributeKeyResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::UnarchiveResourceAttributeKeyRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ResourceAttributeService>::unarchive_resource_attribute_key(
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
                        let method = UnarchiveResourceAttributeKeySvc(inner);
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
                "/sift.resource_attribute.v1.ResourceAttributeService/BatchArchiveResourceAttributeKeys" => {
                    #[allow(non_camel_case_types)]
                    struct BatchArchiveResourceAttributeKeysSvc<
                        T: ResourceAttributeService,
                    >(
                        pub Arc<T>,
                    );
                    impl<
                        T: ResourceAttributeService,
                    > tonic::server::UnaryService<
                        super::BatchArchiveResourceAttributeKeysRequest,
                    > for BatchArchiveResourceAttributeKeysSvc<T> {
                        type Response = super::BatchArchiveResourceAttributeKeysResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::BatchArchiveResourceAttributeKeysRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ResourceAttributeService>::batch_archive_resource_attribute_keys(
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
                        let method = BatchArchiveResourceAttributeKeysSvc(inner);
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
                "/sift.resource_attribute.v1.ResourceAttributeService/BatchUnarchiveResourceAttributeKeys" => {
                    #[allow(non_camel_case_types)]
                    struct BatchUnarchiveResourceAttributeKeysSvc<
                        T: ResourceAttributeService,
                    >(
                        pub Arc<T>,
                    );
                    impl<
                        T: ResourceAttributeService,
                    > tonic::server::UnaryService<
                        super::BatchUnarchiveResourceAttributeKeysRequest,
                    > for BatchUnarchiveResourceAttributeKeysSvc<T> {
                        type Response = super::BatchUnarchiveResourceAttributeKeysResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::BatchUnarchiveResourceAttributeKeysRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ResourceAttributeService>::batch_unarchive_resource_attribute_keys(
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
                        let method = BatchUnarchiveResourceAttributeKeysSvc(inner);
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
                "/sift.resource_attribute.v1.ResourceAttributeService/CreateResourceAttributeEnumValue" => {
                    #[allow(non_camel_case_types)]
                    struct CreateResourceAttributeEnumValueSvc<
                        T: ResourceAttributeService,
                    >(
                        pub Arc<T>,
                    );
                    impl<
                        T: ResourceAttributeService,
                    > tonic::server::UnaryService<
                        super::CreateResourceAttributeEnumValueRequest,
                    > for CreateResourceAttributeEnumValueSvc<T> {
                        type Response = super::CreateResourceAttributeEnumValueResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::CreateResourceAttributeEnumValueRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ResourceAttributeService>::create_resource_attribute_enum_value(
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
                        let method = CreateResourceAttributeEnumValueSvc(inner);
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
                "/sift.resource_attribute.v1.ResourceAttributeService/GetResourceAttributeEnumValue" => {
                    #[allow(non_camel_case_types)]
                    struct GetResourceAttributeEnumValueSvc<T: ResourceAttributeService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: ResourceAttributeService,
                    > tonic::server::UnaryService<
                        super::GetResourceAttributeEnumValueRequest,
                    > for GetResourceAttributeEnumValueSvc<T> {
                        type Response = super::GetResourceAttributeEnumValueResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::GetResourceAttributeEnumValueRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ResourceAttributeService>::get_resource_attribute_enum_value(
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
                        let method = GetResourceAttributeEnumValueSvc(inner);
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
                "/sift.resource_attribute.v1.ResourceAttributeService/ListResourceAttributeEnumValues" => {
                    #[allow(non_camel_case_types)]
                    struct ListResourceAttributeEnumValuesSvc<
                        T: ResourceAttributeService,
                    >(
                        pub Arc<T>,
                    );
                    impl<
                        T: ResourceAttributeService,
                    > tonic::server::UnaryService<
                        super::ListResourceAttributeEnumValuesRequest,
                    > for ListResourceAttributeEnumValuesSvc<T> {
                        type Response = super::ListResourceAttributeEnumValuesResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::ListResourceAttributeEnumValuesRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ResourceAttributeService>::list_resource_attribute_enum_values(
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
                        let method = ListResourceAttributeEnumValuesSvc(inner);
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
                "/sift.resource_attribute.v1.ResourceAttributeService/UpdateResourceAttributeEnumValue" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateResourceAttributeEnumValueSvc<
                        T: ResourceAttributeService,
                    >(
                        pub Arc<T>,
                    );
                    impl<
                        T: ResourceAttributeService,
                    > tonic::server::UnaryService<
                        super::UpdateResourceAttributeEnumValueRequest,
                    > for UpdateResourceAttributeEnumValueSvc<T> {
                        type Response = super::UpdateResourceAttributeEnumValueResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::UpdateResourceAttributeEnumValueRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ResourceAttributeService>::update_resource_attribute_enum_value(
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
                        let method = UpdateResourceAttributeEnumValueSvc(inner);
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
                "/sift.resource_attribute.v1.ResourceAttributeService/ArchiveResourceAttributeEnumValue" => {
                    #[allow(non_camel_case_types)]
                    struct ArchiveResourceAttributeEnumValueSvc<
                        T: ResourceAttributeService,
                    >(
                        pub Arc<T>,
                    );
                    impl<
                        T: ResourceAttributeService,
                    > tonic::server::UnaryService<
                        super::ArchiveResourceAttributeEnumValueRequest,
                    > for ArchiveResourceAttributeEnumValueSvc<T> {
                        type Response = super::ArchiveResourceAttributeEnumValueResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::ArchiveResourceAttributeEnumValueRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ResourceAttributeService>::archive_resource_attribute_enum_value(
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
                        let method = ArchiveResourceAttributeEnumValueSvc(inner);
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
                "/sift.resource_attribute.v1.ResourceAttributeService/UnarchiveResourceAttributeEnumValue" => {
                    #[allow(non_camel_case_types)]
                    struct UnarchiveResourceAttributeEnumValueSvc<
                        T: ResourceAttributeService,
                    >(
                        pub Arc<T>,
                    );
                    impl<
                        T: ResourceAttributeService,
                    > tonic::server::UnaryService<
                        super::UnarchiveResourceAttributeEnumValueRequest,
                    > for UnarchiveResourceAttributeEnumValueSvc<T> {
                        type Response = super::UnarchiveResourceAttributeEnumValueResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::UnarchiveResourceAttributeEnumValueRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ResourceAttributeService>::unarchive_resource_attribute_enum_value(
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
                        let method = UnarchiveResourceAttributeEnumValueSvc(inner);
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
                "/sift.resource_attribute.v1.ResourceAttributeService/BatchArchiveResourceAttributeEnumValues" => {
                    #[allow(non_camel_case_types)]
                    struct BatchArchiveResourceAttributeEnumValuesSvc<
                        T: ResourceAttributeService,
                    >(
                        pub Arc<T>,
                    );
                    impl<
                        T: ResourceAttributeService,
                    > tonic::server::UnaryService<
                        super::BatchArchiveResourceAttributeEnumValuesRequest,
                    > for BatchArchiveResourceAttributeEnumValuesSvc<T> {
                        type Response = super::BatchArchiveResourceAttributeEnumValuesResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::BatchArchiveResourceAttributeEnumValuesRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ResourceAttributeService>::batch_archive_resource_attribute_enum_values(
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
                        let method = BatchArchiveResourceAttributeEnumValuesSvc(inner);
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
                "/sift.resource_attribute.v1.ResourceAttributeService/BatchUnarchiveResourceAttributeEnumValues" => {
                    #[allow(non_camel_case_types)]
                    struct BatchUnarchiveResourceAttributeEnumValuesSvc<
                        T: ResourceAttributeService,
                    >(
                        pub Arc<T>,
                    );
                    impl<
                        T: ResourceAttributeService,
                    > tonic::server::UnaryService<
                        super::BatchUnarchiveResourceAttributeEnumValuesRequest,
                    > for BatchUnarchiveResourceAttributeEnumValuesSvc<T> {
                        type Response = super::BatchUnarchiveResourceAttributeEnumValuesResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::BatchUnarchiveResourceAttributeEnumValuesRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ResourceAttributeService>::batch_unarchive_resource_attribute_enum_values(
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
                        let method = BatchUnarchiveResourceAttributeEnumValuesSvc(inner);
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
                "/sift.resource_attribute.v1.ResourceAttributeService/CreateResourceAttribute" => {
                    #[allow(non_camel_case_types)]
                    struct CreateResourceAttributeSvc<T: ResourceAttributeService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: ResourceAttributeService,
                    > tonic::server::UnaryService<super::CreateResourceAttributeRequest>
                    for CreateResourceAttributeSvc<T> {
                        type Response = super::CreateResourceAttributeResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::CreateResourceAttributeRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ResourceAttributeService>::create_resource_attribute(
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
                        let method = CreateResourceAttributeSvc(inner);
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
                "/sift.resource_attribute.v1.ResourceAttributeService/BatchCreateResourceAttributes" => {
                    #[allow(non_camel_case_types)]
                    struct BatchCreateResourceAttributesSvc<T: ResourceAttributeService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: ResourceAttributeService,
                    > tonic::server::UnaryService<
                        super::BatchCreateResourceAttributesRequest,
                    > for BatchCreateResourceAttributesSvc<T> {
                        type Response = super::BatchCreateResourceAttributesResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::BatchCreateResourceAttributesRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ResourceAttributeService>::batch_create_resource_attributes(
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
                        let method = BatchCreateResourceAttributesSvc(inner);
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
                "/sift.resource_attribute.v1.ResourceAttributeService/GetResourceAttribute" => {
                    #[allow(non_camel_case_types)]
                    struct GetResourceAttributeSvc<T: ResourceAttributeService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: ResourceAttributeService,
                    > tonic::server::UnaryService<super::GetResourceAttributeRequest>
                    for GetResourceAttributeSvc<T> {
                        type Response = super::GetResourceAttributeResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetResourceAttributeRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ResourceAttributeService>::get_resource_attribute(
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
                        let method = GetResourceAttributeSvc(inner);
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
                "/sift.resource_attribute.v1.ResourceAttributeService/ListResourceAttributes" => {
                    #[allow(non_camel_case_types)]
                    struct ListResourceAttributesSvc<T: ResourceAttributeService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: ResourceAttributeService,
                    > tonic::server::UnaryService<super::ListResourceAttributesRequest>
                    for ListResourceAttributesSvc<T> {
                        type Response = super::ListResourceAttributesResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListResourceAttributesRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ResourceAttributeService>::list_resource_attributes(
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
                        let method = ListResourceAttributesSvc(inner);
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
                "/sift.resource_attribute.v1.ResourceAttributeService/ListResourceAttributesByEntity" => {
                    #[allow(non_camel_case_types)]
                    struct ListResourceAttributesByEntitySvc<
                        T: ResourceAttributeService,
                    >(
                        pub Arc<T>,
                    );
                    impl<
                        T: ResourceAttributeService,
                    > tonic::server::UnaryService<
                        super::ListResourceAttributesByEntityRequest,
                    > for ListResourceAttributesByEntitySvc<T> {
                        type Response = super::ListResourceAttributesByEntityResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::ListResourceAttributesByEntityRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ResourceAttributeService>::list_resource_attributes_by_entity(
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
                        let method = ListResourceAttributesByEntitySvc(inner);
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
                "/sift.resource_attribute.v1.ResourceAttributeService/ArchiveResourceAttribute" => {
                    #[allow(non_camel_case_types)]
                    struct ArchiveResourceAttributeSvc<T: ResourceAttributeService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: ResourceAttributeService,
                    > tonic::server::UnaryService<super::ArchiveResourceAttributeRequest>
                    for ArchiveResourceAttributeSvc<T> {
                        type Response = super::ArchiveResourceAttributeResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::ArchiveResourceAttributeRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ResourceAttributeService>::archive_resource_attribute(
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
                        let method = ArchiveResourceAttributeSvc(inner);
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
                "/sift.resource_attribute.v1.ResourceAttributeService/UnarchiveResourceAttribute" => {
                    #[allow(non_camel_case_types)]
                    struct UnarchiveResourceAttributeSvc<T: ResourceAttributeService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: ResourceAttributeService,
                    > tonic::server::UnaryService<
                        super::UnarchiveResourceAttributeRequest,
                    > for UnarchiveResourceAttributeSvc<T> {
                        type Response = super::UnarchiveResourceAttributeResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::UnarchiveResourceAttributeRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ResourceAttributeService>::unarchive_resource_attribute(
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
                        let method = UnarchiveResourceAttributeSvc(inner);
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
                "/sift.resource_attribute.v1.ResourceAttributeService/BatchArchiveResourceAttributes" => {
                    #[allow(non_camel_case_types)]
                    struct BatchArchiveResourceAttributesSvc<
                        T: ResourceAttributeService,
                    >(
                        pub Arc<T>,
                    );
                    impl<
                        T: ResourceAttributeService,
                    > tonic::server::UnaryService<
                        super::BatchArchiveResourceAttributesRequest,
                    > for BatchArchiveResourceAttributesSvc<T> {
                        type Response = super::BatchArchiveResourceAttributesResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::BatchArchiveResourceAttributesRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ResourceAttributeService>::batch_archive_resource_attributes(
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
                        let method = BatchArchiveResourceAttributesSvc(inner);
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
                "/sift.resource_attribute.v1.ResourceAttributeService/BatchUnarchiveResourceAttributes" => {
                    #[allow(non_camel_case_types)]
                    struct BatchUnarchiveResourceAttributesSvc<
                        T: ResourceAttributeService,
                    >(
                        pub Arc<T>,
                    );
                    impl<
                        T: ResourceAttributeService,
                    > tonic::server::UnaryService<
                        super::BatchUnarchiveResourceAttributesRequest,
                    > for BatchUnarchiveResourceAttributesSvc<T> {
                        type Response = super::BatchUnarchiveResourceAttributesResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::BatchUnarchiveResourceAttributesRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ResourceAttributeService>::batch_unarchive_resource_attributes(
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
                        let method = BatchUnarchiveResourceAttributesSvc(inner);
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
    impl<T: ResourceAttributeService> Clone for ResourceAttributeServiceServer<T> {
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
    impl<T: ResourceAttributeService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: ResourceAttributeService> tonic::server::NamedService
    for ResourceAttributeServiceServer<T> {
        const NAME: &'static str = "sift.resource_attribute.v1.ResourceAttributeService";
    }
}
