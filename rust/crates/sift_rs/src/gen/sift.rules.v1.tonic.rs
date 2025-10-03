// @generated
/// Generated client implementations.
pub mod rule_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct RuleServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl RuleServiceClient<tonic::transport::Channel> {
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
    impl<T> RuleServiceClient<T>
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
        ) -> RuleServiceClient<InterceptedService<T, F>>
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
            RuleServiceClient::new(InterceptedService::new(inner, interceptor))
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
        pub async fn search_rules(
            &mut self,
            request: impl tonic::IntoRequest<super::SearchRulesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SearchRulesResponse>,
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
                "/sift.rules.v1.RuleService/SearchRules",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("sift.rules.v1.RuleService", "SearchRules"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_rule(
            &mut self,
            request: impl tonic::IntoRequest<super::GetRuleRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetRuleResponse>,
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
                "/sift.rules.v1.RuleService/GetRule",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("sift.rules.v1.RuleService", "GetRule"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn batch_get_rules(
            &mut self,
            request: impl tonic::IntoRequest<super::BatchGetRulesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::BatchGetRulesResponse>,
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
                "/sift.rules.v1.RuleService/BatchGetRules",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("sift.rules.v1.RuleService", "BatchGetRules"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn create_rule(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateRuleRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateRuleResponse>,
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
                "/sift.rules.v1.RuleService/CreateRule",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("sift.rules.v1.RuleService", "CreateRule"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn update_rule(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateRuleRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateRuleResponse>,
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
                "/sift.rules.v1.RuleService/UpdateRule",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("sift.rules.v1.RuleService", "UpdateRule"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn batch_update_rules(
            &mut self,
            request: impl tonic::IntoRequest<super::BatchUpdateRulesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::BatchUpdateRulesResponse>,
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
                "/sift.rules.v1.RuleService/BatchUpdateRules",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("sift.rules.v1.RuleService", "BatchUpdateRules"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn delete_rule(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteRuleRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DeleteRuleResponse>,
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
                "/sift.rules.v1.RuleService/DeleteRule",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("sift.rules.v1.RuleService", "DeleteRule"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn archive_rule(
            &mut self,
            request: impl tonic::IntoRequest<super::ArchiveRuleRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ArchiveRuleResponse>,
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
                "/sift.rules.v1.RuleService/ArchiveRule",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("sift.rules.v1.RuleService", "ArchiveRule"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn batch_delete_rules(
            &mut self,
            request: impl tonic::IntoRequest<super::BatchDeleteRulesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::BatchDeleteRulesResponse>,
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
                "/sift.rules.v1.RuleService/BatchDeleteRules",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("sift.rules.v1.RuleService", "BatchDeleteRules"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn batch_archive_rules(
            &mut self,
            request: impl tonic::IntoRequest<super::BatchArchiveRulesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::BatchArchiveRulesResponse>,
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
                "/sift.rules.v1.RuleService/BatchArchiveRules",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("sift.rules.v1.RuleService", "BatchArchiveRules"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn unarchive_rule(
            &mut self,
            request: impl tonic::IntoRequest<super::UnarchiveRuleRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UnarchiveRuleResponse>,
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
                "/sift.rules.v1.RuleService/UnarchiveRule",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("sift.rules.v1.RuleService", "UnarchiveRule"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn batch_unarchive_rules(
            &mut self,
            request: impl tonic::IntoRequest<super::BatchUnarchiveRulesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::BatchUnarchiveRulesResponse>,
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
                "/sift.rules.v1.RuleService/BatchUnarchiveRules",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("sift.rules.v1.RuleService", "BatchUnarchiveRules"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn undelete_rule(
            &mut self,
            request: impl tonic::IntoRequest<super::UndeleteRuleRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UndeleteRuleResponse>,
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
                "/sift.rules.v1.RuleService/UndeleteRule",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("sift.rules.v1.RuleService", "UndeleteRule"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn batch_undelete_rules(
            &mut self,
            request: impl tonic::IntoRequest<super::BatchUndeleteRulesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::BatchUndeleteRulesResponse>,
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
                "/sift.rules.v1.RuleService/BatchUndeleteRules",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("sift.rules.v1.RuleService", "BatchUndeleteRules"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn evaluate_rules(
            &mut self,
            request: impl tonic::IntoRequest<super::EvaluateRulesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::EvaluateRulesResponse>,
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
                "/sift.rules.v1.RuleService/EvaluateRules",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("sift.rules.v1.RuleService", "EvaluateRules"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn view_human_friendly_rules(
            &mut self,
            request: impl tonic::IntoRequest<super::ViewHumanFriendlyRulesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ViewHumanFriendlyRulesResponse>,
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
                "/sift.rules.v1.RuleService/ViewHumanFriendlyRules",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sift.rules.v1.RuleService",
                        "ViewHumanFriendlyRules",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn view_json_rules(
            &mut self,
            request: impl tonic::IntoRequest<super::ViewJsonRulesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ViewJsonRulesResponse>,
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
                "/sift.rules.v1.RuleService/ViewJsonRules",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("sift.rules.v1.RuleService", "ViewJsonRules"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn update_human_friendly_rules(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateHumanFriendlyRulesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateHumanFriendlyRulesResponse>,
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
                "/sift.rules.v1.RuleService/UpdateHumanFriendlyRules",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sift.rules.v1.RuleService",
                        "UpdateHumanFriendlyRules",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn validate_json_rules(
            &mut self,
            request: impl tonic::IntoRequest<super::ValidateJsonRulesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ValidateJsonRulesResponse>,
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
                "/sift.rules.v1.RuleService/ValidateJsonRules",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("sift.rules.v1.RuleService", "ValidateJsonRules"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn update_json_rules(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateJsonRulesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateJsonRulesResponse>,
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
                "/sift.rules.v1.RuleService/UpdateJsonRules",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("sift.rules.v1.RuleService", "UpdateJsonRules"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_rules(
            &mut self,
            request: impl tonic::IntoRequest<super::ListRulesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListRulesResponse>,
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
                "/sift.rules.v1.RuleService/ListRules",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("sift.rules.v1.RuleService", "ListRules"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_rule_versions(
            &mut self,
            request: impl tonic::IntoRequest<super::ListRuleVersionsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListRuleVersionsResponse>,
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
                "/sift.rules.v1.RuleService/ListRuleVersions",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("sift.rules.v1.RuleService", "ListRuleVersions"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_rule_version(
            &mut self,
            request: impl tonic::IntoRequest<super::GetRuleVersionRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetRuleVersionResponse>,
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
                "/sift.rules.v1.RuleService/GetRuleVersion",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("sift.rules.v1.RuleService", "GetRuleVersion"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn batch_get_rule_versions(
            &mut self,
            request: impl tonic::IntoRequest<super::BatchGetRuleVersionsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::BatchGetRuleVersionsResponse>,
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
                "/sift.rules.v1.RuleService/BatchGetRuleVersions",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("sift.rules.v1.RuleService", "BatchGetRuleVersions"),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod rule_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with RuleServiceServer.
    #[async_trait]
    pub trait RuleService: Send + Sync + 'static {
        async fn search_rules(
            &self,
            request: tonic::Request<super::SearchRulesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SearchRulesResponse>,
            tonic::Status,
        >;
        async fn get_rule(
            &self,
            request: tonic::Request<super::GetRuleRequest>,
        ) -> std::result::Result<tonic::Response<super::GetRuleResponse>, tonic::Status>;
        async fn batch_get_rules(
            &self,
            request: tonic::Request<super::BatchGetRulesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::BatchGetRulesResponse>,
            tonic::Status,
        >;
        async fn create_rule(
            &self,
            request: tonic::Request<super::CreateRuleRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateRuleResponse>,
            tonic::Status,
        >;
        async fn update_rule(
            &self,
            request: tonic::Request<super::UpdateRuleRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateRuleResponse>,
            tonic::Status,
        >;
        async fn batch_update_rules(
            &self,
            request: tonic::Request<super::BatchUpdateRulesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::BatchUpdateRulesResponse>,
            tonic::Status,
        >;
        async fn delete_rule(
            &self,
            request: tonic::Request<super::DeleteRuleRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DeleteRuleResponse>,
            tonic::Status,
        >;
        async fn archive_rule(
            &self,
            request: tonic::Request<super::ArchiveRuleRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ArchiveRuleResponse>,
            tonic::Status,
        >;
        async fn batch_delete_rules(
            &self,
            request: tonic::Request<super::BatchDeleteRulesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::BatchDeleteRulesResponse>,
            tonic::Status,
        >;
        async fn batch_archive_rules(
            &self,
            request: tonic::Request<super::BatchArchiveRulesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::BatchArchiveRulesResponse>,
            tonic::Status,
        >;
        async fn unarchive_rule(
            &self,
            request: tonic::Request<super::UnarchiveRuleRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UnarchiveRuleResponse>,
            tonic::Status,
        >;
        async fn batch_unarchive_rules(
            &self,
            request: tonic::Request<super::BatchUnarchiveRulesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::BatchUnarchiveRulesResponse>,
            tonic::Status,
        >;
        async fn undelete_rule(
            &self,
            request: tonic::Request<super::UndeleteRuleRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UndeleteRuleResponse>,
            tonic::Status,
        >;
        async fn batch_undelete_rules(
            &self,
            request: tonic::Request<super::BatchUndeleteRulesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::BatchUndeleteRulesResponse>,
            tonic::Status,
        >;
        async fn evaluate_rules(
            &self,
            request: tonic::Request<super::EvaluateRulesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::EvaluateRulesResponse>,
            tonic::Status,
        >;
        async fn view_human_friendly_rules(
            &self,
            request: tonic::Request<super::ViewHumanFriendlyRulesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ViewHumanFriendlyRulesResponse>,
            tonic::Status,
        >;
        async fn view_json_rules(
            &self,
            request: tonic::Request<super::ViewJsonRulesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ViewJsonRulesResponse>,
            tonic::Status,
        >;
        async fn update_human_friendly_rules(
            &self,
            request: tonic::Request<super::UpdateHumanFriendlyRulesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateHumanFriendlyRulesResponse>,
            tonic::Status,
        >;
        async fn validate_json_rules(
            &self,
            request: tonic::Request<super::ValidateJsonRulesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ValidateJsonRulesResponse>,
            tonic::Status,
        >;
        async fn update_json_rules(
            &self,
            request: tonic::Request<super::UpdateJsonRulesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateJsonRulesResponse>,
            tonic::Status,
        >;
        async fn list_rules(
            &self,
            request: tonic::Request<super::ListRulesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListRulesResponse>,
            tonic::Status,
        >;
        async fn list_rule_versions(
            &self,
            request: tonic::Request<super::ListRuleVersionsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListRuleVersionsResponse>,
            tonic::Status,
        >;
        async fn get_rule_version(
            &self,
            request: tonic::Request<super::GetRuleVersionRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetRuleVersionResponse>,
            tonic::Status,
        >;
        async fn batch_get_rule_versions(
            &self,
            request: tonic::Request<super::BatchGetRuleVersionsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::BatchGetRuleVersionsResponse>,
            tonic::Status,
        >;
    }
    #[derive(Debug)]
    pub struct RuleServiceServer<T: RuleService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: RuleService> RuleServiceServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for RuleServiceServer<T>
    where
        T: RuleService,
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
                "/sift.rules.v1.RuleService/SearchRules" => {
                    #[allow(non_camel_case_types)]
                    struct SearchRulesSvc<T: RuleService>(pub Arc<T>);
                    impl<
                        T: RuleService,
                    > tonic::server::UnaryService<super::SearchRulesRequest>
                    for SearchRulesSvc<T> {
                        type Response = super::SearchRulesResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SearchRulesRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as RuleService>::search_rules(&inner, request).await
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
                        let method = SearchRulesSvc(inner);
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
                "/sift.rules.v1.RuleService/GetRule" => {
                    #[allow(non_camel_case_types)]
                    struct GetRuleSvc<T: RuleService>(pub Arc<T>);
                    impl<
                        T: RuleService,
                    > tonic::server::UnaryService<super::GetRuleRequest>
                    for GetRuleSvc<T> {
                        type Response = super::GetRuleResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetRuleRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as RuleService>::get_rule(&inner, request).await
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
                        let method = GetRuleSvc(inner);
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
                "/sift.rules.v1.RuleService/BatchGetRules" => {
                    #[allow(non_camel_case_types)]
                    struct BatchGetRulesSvc<T: RuleService>(pub Arc<T>);
                    impl<
                        T: RuleService,
                    > tonic::server::UnaryService<super::BatchGetRulesRequest>
                    for BatchGetRulesSvc<T> {
                        type Response = super::BatchGetRulesResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::BatchGetRulesRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as RuleService>::batch_get_rules(&inner, request).await
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
                        let method = BatchGetRulesSvc(inner);
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
                "/sift.rules.v1.RuleService/CreateRule" => {
                    #[allow(non_camel_case_types)]
                    struct CreateRuleSvc<T: RuleService>(pub Arc<T>);
                    impl<
                        T: RuleService,
                    > tonic::server::UnaryService<super::CreateRuleRequest>
                    for CreateRuleSvc<T> {
                        type Response = super::CreateRuleResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateRuleRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as RuleService>::create_rule(&inner, request).await
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
                        let method = CreateRuleSvc(inner);
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
                "/sift.rules.v1.RuleService/UpdateRule" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateRuleSvc<T: RuleService>(pub Arc<T>);
                    impl<
                        T: RuleService,
                    > tonic::server::UnaryService<super::UpdateRuleRequest>
                    for UpdateRuleSvc<T> {
                        type Response = super::UpdateRuleResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdateRuleRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as RuleService>::update_rule(&inner, request).await
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
                        let method = UpdateRuleSvc(inner);
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
                "/sift.rules.v1.RuleService/BatchUpdateRules" => {
                    #[allow(non_camel_case_types)]
                    struct BatchUpdateRulesSvc<T: RuleService>(pub Arc<T>);
                    impl<
                        T: RuleService,
                    > tonic::server::UnaryService<super::BatchUpdateRulesRequest>
                    for BatchUpdateRulesSvc<T> {
                        type Response = super::BatchUpdateRulesResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::BatchUpdateRulesRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as RuleService>::batch_update_rules(&inner, request)
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
                        let method = BatchUpdateRulesSvc(inner);
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
                "/sift.rules.v1.RuleService/DeleteRule" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteRuleSvc<T: RuleService>(pub Arc<T>);
                    impl<
                        T: RuleService,
                    > tonic::server::UnaryService<super::DeleteRuleRequest>
                    for DeleteRuleSvc<T> {
                        type Response = super::DeleteRuleResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteRuleRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as RuleService>::delete_rule(&inner, request).await
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
                        let method = DeleteRuleSvc(inner);
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
                "/sift.rules.v1.RuleService/ArchiveRule" => {
                    #[allow(non_camel_case_types)]
                    struct ArchiveRuleSvc<T: RuleService>(pub Arc<T>);
                    impl<
                        T: RuleService,
                    > tonic::server::UnaryService<super::ArchiveRuleRequest>
                    for ArchiveRuleSvc<T> {
                        type Response = super::ArchiveRuleResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ArchiveRuleRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as RuleService>::archive_rule(&inner, request).await
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
                        let method = ArchiveRuleSvc(inner);
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
                "/sift.rules.v1.RuleService/BatchDeleteRules" => {
                    #[allow(non_camel_case_types)]
                    struct BatchDeleteRulesSvc<T: RuleService>(pub Arc<T>);
                    impl<
                        T: RuleService,
                    > tonic::server::UnaryService<super::BatchDeleteRulesRequest>
                    for BatchDeleteRulesSvc<T> {
                        type Response = super::BatchDeleteRulesResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::BatchDeleteRulesRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as RuleService>::batch_delete_rules(&inner, request)
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
                        let method = BatchDeleteRulesSvc(inner);
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
                "/sift.rules.v1.RuleService/BatchArchiveRules" => {
                    #[allow(non_camel_case_types)]
                    struct BatchArchiveRulesSvc<T: RuleService>(pub Arc<T>);
                    impl<
                        T: RuleService,
                    > tonic::server::UnaryService<super::BatchArchiveRulesRequest>
                    for BatchArchiveRulesSvc<T> {
                        type Response = super::BatchArchiveRulesResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::BatchArchiveRulesRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as RuleService>::batch_archive_rules(&inner, request)
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
                        let method = BatchArchiveRulesSvc(inner);
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
                "/sift.rules.v1.RuleService/UnarchiveRule" => {
                    #[allow(non_camel_case_types)]
                    struct UnarchiveRuleSvc<T: RuleService>(pub Arc<T>);
                    impl<
                        T: RuleService,
                    > tonic::server::UnaryService<super::UnarchiveRuleRequest>
                    for UnarchiveRuleSvc<T> {
                        type Response = super::UnarchiveRuleResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UnarchiveRuleRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as RuleService>::unarchive_rule(&inner, request).await
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
                        let method = UnarchiveRuleSvc(inner);
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
                "/sift.rules.v1.RuleService/BatchUnarchiveRules" => {
                    #[allow(non_camel_case_types)]
                    struct BatchUnarchiveRulesSvc<T: RuleService>(pub Arc<T>);
                    impl<
                        T: RuleService,
                    > tonic::server::UnaryService<super::BatchUnarchiveRulesRequest>
                    for BatchUnarchiveRulesSvc<T> {
                        type Response = super::BatchUnarchiveRulesResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::BatchUnarchiveRulesRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as RuleService>::batch_unarchive_rules(&inner, request)
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
                        let method = BatchUnarchiveRulesSvc(inner);
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
                "/sift.rules.v1.RuleService/UndeleteRule" => {
                    #[allow(non_camel_case_types)]
                    struct UndeleteRuleSvc<T: RuleService>(pub Arc<T>);
                    impl<
                        T: RuleService,
                    > tonic::server::UnaryService<super::UndeleteRuleRequest>
                    for UndeleteRuleSvc<T> {
                        type Response = super::UndeleteRuleResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UndeleteRuleRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as RuleService>::undelete_rule(&inner, request).await
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
                        let method = UndeleteRuleSvc(inner);
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
                "/sift.rules.v1.RuleService/BatchUndeleteRules" => {
                    #[allow(non_camel_case_types)]
                    struct BatchUndeleteRulesSvc<T: RuleService>(pub Arc<T>);
                    impl<
                        T: RuleService,
                    > tonic::server::UnaryService<super::BatchUndeleteRulesRequest>
                    for BatchUndeleteRulesSvc<T> {
                        type Response = super::BatchUndeleteRulesResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::BatchUndeleteRulesRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as RuleService>::batch_undelete_rules(&inner, request)
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
                        let method = BatchUndeleteRulesSvc(inner);
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
                "/sift.rules.v1.RuleService/EvaluateRules" => {
                    #[allow(non_camel_case_types)]
                    struct EvaluateRulesSvc<T: RuleService>(pub Arc<T>);
                    impl<
                        T: RuleService,
                    > tonic::server::UnaryService<super::EvaluateRulesRequest>
                    for EvaluateRulesSvc<T> {
                        type Response = super::EvaluateRulesResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::EvaluateRulesRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as RuleService>::evaluate_rules(&inner, request).await
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
                        let method = EvaluateRulesSvc(inner);
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
                "/sift.rules.v1.RuleService/ViewHumanFriendlyRules" => {
                    #[allow(non_camel_case_types)]
                    struct ViewHumanFriendlyRulesSvc<T: RuleService>(pub Arc<T>);
                    impl<
                        T: RuleService,
                    > tonic::server::UnaryService<super::ViewHumanFriendlyRulesRequest>
                    for ViewHumanFriendlyRulesSvc<T> {
                        type Response = super::ViewHumanFriendlyRulesResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ViewHumanFriendlyRulesRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as RuleService>::view_human_friendly_rules(
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
                        let method = ViewHumanFriendlyRulesSvc(inner);
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
                "/sift.rules.v1.RuleService/ViewJsonRules" => {
                    #[allow(non_camel_case_types)]
                    struct ViewJsonRulesSvc<T: RuleService>(pub Arc<T>);
                    impl<
                        T: RuleService,
                    > tonic::server::UnaryService<super::ViewJsonRulesRequest>
                    for ViewJsonRulesSvc<T> {
                        type Response = super::ViewJsonRulesResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ViewJsonRulesRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as RuleService>::view_json_rules(&inner, request).await
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
                        let method = ViewJsonRulesSvc(inner);
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
                "/sift.rules.v1.RuleService/UpdateHumanFriendlyRules" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateHumanFriendlyRulesSvc<T: RuleService>(pub Arc<T>);
                    impl<
                        T: RuleService,
                    > tonic::server::UnaryService<super::UpdateHumanFriendlyRulesRequest>
                    for UpdateHumanFriendlyRulesSvc<T> {
                        type Response = super::UpdateHumanFriendlyRulesResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::UpdateHumanFriendlyRulesRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as RuleService>::update_human_friendly_rules(
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
                        let method = UpdateHumanFriendlyRulesSvc(inner);
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
                "/sift.rules.v1.RuleService/ValidateJsonRules" => {
                    #[allow(non_camel_case_types)]
                    struct ValidateJsonRulesSvc<T: RuleService>(pub Arc<T>);
                    impl<
                        T: RuleService,
                    > tonic::server::UnaryService<super::ValidateJsonRulesRequest>
                    for ValidateJsonRulesSvc<T> {
                        type Response = super::ValidateJsonRulesResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ValidateJsonRulesRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as RuleService>::validate_json_rules(&inner, request)
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
                        let method = ValidateJsonRulesSvc(inner);
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
                "/sift.rules.v1.RuleService/UpdateJsonRules" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateJsonRulesSvc<T: RuleService>(pub Arc<T>);
                    impl<
                        T: RuleService,
                    > tonic::server::UnaryService<super::UpdateJsonRulesRequest>
                    for UpdateJsonRulesSvc<T> {
                        type Response = super::UpdateJsonRulesResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdateJsonRulesRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as RuleService>::update_json_rules(&inner, request).await
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
                        let method = UpdateJsonRulesSvc(inner);
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
                "/sift.rules.v1.RuleService/ListRules" => {
                    #[allow(non_camel_case_types)]
                    struct ListRulesSvc<T: RuleService>(pub Arc<T>);
                    impl<
                        T: RuleService,
                    > tonic::server::UnaryService<super::ListRulesRequest>
                    for ListRulesSvc<T> {
                        type Response = super::ListRulesResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListRulesRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as RuleService>::list_rules(&inner, request).await
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
                        let method = ListRulesSvc(inner);
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
                "/sift.rules.v1.RuleService/ListRuleVersions" => {
                    #[allow(non_camel_case_types)]
                    struct ListRuleVersionsSvc<T: RuleService>(pub Arc<T>);
                    impl<
                        T: RuleService,
                    > tonic::server::UnaryService<super::ListRuleVersionsRequest>
                    for ListRuleVersionsSvc<T> {
                        type Response = super::ListRuleVersionsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListRuleVersionsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as RuleService>::list_rule_versions(&inner, request)
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
                        let method = ListRuleVersionsSvc(inner);
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
                "/sift.rules.v1.RuleService/GetRuleVersion" => {
                    #[allow(non_camel_case_types)]
                    struct GetRuleVersionSvc<T: RuleService>(pub Arc<T>);
                    impl<
                        T: RuleService,
                    > tonic::server::UnaryService<super::GetRuleVersionRequest>
                    for GetRuleVersionSvc<T> {
                        type Response = super::GetRuleVersionResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetRuleVersionRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as RuleService>::get_rule_version(&inner, request).await
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
                        let method = GetRuleVersionSvc(inner);
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
                "/sift.rules.v1.RuleService/BatchGetRuleVersions" => {
                    #[allow(non_camel_case_types)]
                    struct BatchGetRuleVersionsSvc<T: RuleService>(pub Arc<T>);
                    impl<
                        T: RuleService,
                    > tonic::server::UnaryService<super::BatchGetRuleVersionsRequest>
                    for BatchGetRuleVersionsSvc<T> {
                        type Response = super::BatchGetRuleVersionsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::BatchGetRuleVersionsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as RuleService>::batch_get_rule_versions(&inner, request)
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
                        let method = BatchGetRuleVersionsSvc(inner);
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
    impl<T: RuleService> Clone for RuleServiceServer<T> {
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
    impl<T: RuleService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: RuleService> tonic::server::NamedService for RuleServiceServer<T> {
        const NAME: &'static str = "sift.rules.v1.RuleService";
    }
}
