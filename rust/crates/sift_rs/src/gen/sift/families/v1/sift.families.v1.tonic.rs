// @generated
/// Generated client implementations.
pub mod family_service_client {
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
    pub struct FamilyServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl FamilyServiceClient<tonic::transport::Channel> {
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
    impl<T> FamilyServiceClient<T>
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
        ) -> FamilyServiceClient<InterceptedService<T, F>>
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
            FamilyServiceClient::new(InterceptedService::new(inner, interceptor))
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
        pub async fn get_family(
            &mut self,
            request: impl tonic::IntoRequest<super::GetFamilyRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetFamilyResponse>,
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
                "/sift.families.v1.FamilyService/GetFamily",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("sift.families.v1.FamilyService", "GetFamily"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_families(
            &mut self,
            request: impl tonic::IntoRequest<super::GetFamiliesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetFamiliesResponse>,
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
                "/sift.families.v1.FamilyService/GetFamilies",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("sift.families.v1.FamilyService", "GetFamilies"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_family_version(
            &mut self,
            request: impl tonic::IntoRequest<super::GetFamilyVersionRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetFamilyVersionResponse>,
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
                "/sift.families.v1.FamilyService/GetFamilyVersion",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("sift.families.v1.FamilyService", "GetFamilyVersion"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn create_family(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateFamilyRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateFamilyResponse>,
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
                "/sift.families.v1.FamilyService/CreateFamily",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("sift.families.v1.FamilyService", "CreateFamily"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn update_family(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateFamilyRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateFamilyResponse>,
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
                "/sift.families.v1.FamilyService/UpdateFamily",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("sift.families.v1.FamilyService", "UpdateFamily"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_family_candidate_runs(
            &mut self,
            request: impl tonic::IntoRequest<super::GetFamilyCandidateRunsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetFamilyCandidateRunsResponse>,
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
                "/sift.families.v1.FamilyService/GetFamilyCandidateRuns",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sift.families.v1.FamilyService",
                        "GetFamilyCandidateRuns",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_family_candidate_run_count(
            &mut self,
            request: impl tonic::IntoRequest<super::GetFamilyCandidateRunCountRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetFamilyCandidateRunCountResponse>,
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
                "/sift.families.v1.FamilyService/GetFamilyCandidateRunCount",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sift.families.v1.FamilyService",
                        "GetFamilyCandidateRunCount",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_family_candidate_runs(
            &mut self,
            request: impl tonic::IntoRequest<super::ListFamilyCandidateRunsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListFamilyCandidateRunsResponse>,
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
                "/sift.families.v1.FamilyService/ListFamilyCandidateRuns",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sift.families.v1.FamilyService",
                        "ListFamilyCandidateRuns",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_family_candidate_run_filter_fields(
            &mut self,
            request: impl tonic::IntoRequest<
                super::GetFamilyCandidateRunFilterFieldsRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::GetFamilyCandidateRunFilterFieldsResponse>,
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
                "/sift.families.v1.FamilyService/GetFamilyCandidateRunFilterFields",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sift.families.v1.FamilyService",
                        "GetFamilyCandidateRunFilterFields",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_families(
            &mut self,
            request: impl tonic::IntoRequest<super::ListFamiliesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListFamiliesResponse>,
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
                "/sift.families.v1.FamilyService/ListFamilies",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("sift.families.v1.FamilyService", "ListFamilies"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_family_versions(
            &mut self,
            request: impl tonic::IntoRequest<super::ListFamilyVersionsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListFamilyVersionsResponse>,
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
                "/sift.families.v1.FamilyService/ListFamilyVersions",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sift.families.v1.FamilyService",
                        "ListFamilyVersions",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_family_members(
            &mut self,
            request: impl tonic::IntoRequest<super::ListFamilyMembersRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListFamilyMembersResponse>,
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
                "/sift.families.v1.FamilyService/ListFamilyMembers",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sift.families.v1.FamilyService",
                        "ListFamilyMembers",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_family_alignments(
            &mut self,
            request: impl tonic::IntoRequest<super::ListFamilyAlignmentsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListFamilyAlignmentsResponse>,
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
                "/sift.families.v1.FamilyService/ListFamilyAlignments",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sift.families.v1.FamilyService",
                        "ListFamilyAlignments",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_family_alignment_points(
            &mut self,
            request: impl tonic::IntoRequest<super::ListFamilyAlignmentPointsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListFamilyAlignmentPointsResponse>,
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
                "/sift.families.v1.FamilyService/ListFamilyAlignmentPoints",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sift.families.v1.FamilyService",
                        "ListFamilyAlignmentPoints",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn import_family(
            &mut self,
            request: impl tonic::IntoRequest<super::ImportFamilyRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ImportFamilyResponse>,
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
                "/sift.families.v1.FamilyService/ImportFamily",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("sift.families.v1.FamilyService", "ImportFamily"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn import_update_family(
            &mut self,
            request: impl tonic::IntoRequest<super::ImportUpdateFamilyRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ImportUpdateFamilyResponse>,
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
                "/sift.families.v1.FamilyService/ImportUpdateFamily",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sift.families.v1.FamilyService",
                        "ImportUpdateFamily",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn export_family(
            &mut self,
            request: impl tonic::IntoRequest<super::ExportFamilyRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ExportFamilyResponse>,
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
                "/sift.families.v1.FamilyService/ExportFamily",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("sift.families.v1.FamilyService", "ExportFamily"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn validate_family_name(
            &mut self,
            request: impl tonic::IntoRequest<super::ValidateFamilyNameRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ValidateFamilyNameResponse>,
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
                "/sift.families.v1.FamilyService/ValidateFamilyName",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sift.families.v1.FamilyService",
                        "ValidateFamilyName",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn validate_family_client_key(
            &mut self,
            request: impl tonic::IntoRequest<super::ValidateFamilyClientKeyRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ValidateFamilyClientKeyResponse>,
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
                "/sift.families.v1.FamilyService/ValidateFamilyClientKey",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sift.families.v1.FamilyService",
                        "ValidateFamilyClientKey",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_family_stat(
            &mut self,
            request: impl tonic::IntoRequest<super::GetFamilyStatRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetFamilyStatResponse>,
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
                "/sift.families.v1.FamilyService/GetFamilyStat",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("sift.families.v1.FamilyService", "GetFamilyStat"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_family_stats(
            &mut self,
            request: impl tonic::IntoRequest<super::GetFamilyStatsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetFamilyStatsResponse>,
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
                "/sift.families.v1.FamilyService/GetFamilyStats",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("sift.families.v1.FamilyService", "GetFamilyStats"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn create_family_stat_ranges(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateFamilyStatRangesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateFamilyStatRangesResponse>,
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
                "/sift.families.v1.FamilyService/CreateFamilyStatRanges",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sift.families.v1.FamilyService",
                        "CreateFamilyStatRanges",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_family_rule_dependencies(
            &mut self,
            request: impl tonic::IntoRequest<super::GetFamilyRuleDependenciesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetFamilyRuleDependenciesResponse>,
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
                "/sift.families.v1.FamilyService/GetFamilyRuleDependencies",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sift.families.v1.FamilyService",
                        "GetFamilyRuleDependencies",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn generate_family_stat_range_name(
            &mut self,
            request: impl tonic::IntoRequest<super::GenerateFamilyStatRangeNameRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GenerateFamilyStatRangeNameResponse>,
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
                "/sift.families.v1.FamilyService/GenerateFamilyStatRangeName",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sift.families.v1.FamilyService",
                        "GenerateFamilyStatRangeName",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod family_service_server {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::wildcard_imports,
        clippy::let_unit_value,
    )]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with FamilyServiceServer.
    #[async_trait]
    pub trait FamilyService: std::marker::Send + std::marker::Sync + 'static {
        async fn get_family(
            &self,
            request: tonic::Request<super::GetFamilyRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetFamilyResponse>,
            tonic::Status,
        >;
        async fn get_families(
            &self,
            request: tonic::Request<super::GetFamiliesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetFamiliesResponse>,
            tonic::Status,
        >;
        async fn get_family_version(
            &self,
            request: tonic::Request<super::GetFamilyVersionRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetFamilyVersionResponse>,
            tonic::Status,
        >;
        async fn create_family(
            &self,
            request: tonic::Request<super::CreateFamilyRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateFamilyResponse>,
            tonic::Status,
        >;
        async fn update_family(
            &self,
            request: tonic::Request<super::UpdateFamilyRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateFamilyResponse>,
            tonic::Status,
        >;
        async fn get_family_candidate_runs(
            &self,
            request: tonic::Request<super::GetFamilyCandidateRunsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetFamilyCandidateRunsResponse>,
            tonic::Status,
        >;
        async fn get_family_candidate_run_count(
            &self,
            request: tonic::Request<super::GetFamilyCandidateRunCountRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetFamilyCandidateRunCountResponse>,
            tonic::Status,
        >;
        async fn list_family_candidate_runs(
            &self,
            request: tonic::Request<super::ListFamilyCandidateRunsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListFamilyCandidateRunsResponse>,
            tonic::Status,
        >;
        async fn get_family_candidate_run_filter_fields(
            &self,
            request: tonic::Request<super::GetFamilyCandidateRunFilterFieldsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetFamilyCandidateRunFilterFieldsResponse>,
            tonic::Status,
        >;
        async fn list_families(
            &self,
            request: tonic::Request<super::ListFamiliesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListFamiliesResponse>,
            tonic::Status,
        >;
        async fn list_family_versions(
            &self,
            request: tonic::Request<super::ListFamilyVersionsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListFamilyVersionsResponse>,
            tonic::Status,
        >;
        async fn list_family_members(
            &self,
            request: tonic::Request<super::ListFamilyMembersRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListFamilyMembersResponse>,
            tonic::Status,
        >;
        async fn list_family_alignments(
            &self,
            request: tonic::Request<super::ListFamilyAlignmentsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListFamilyAlignmentsResponse>,
            tonic::Status,
        >;
        async fn list_family_alignment_points(
            &self,
            request: tonic::Request<super::ListFamilyAlignmentPointsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListFamilyAlignmentPointsResponse>,
            tonic::Status,
        >;
        async fn import_family(
            &self,
            request: tonic::Request<super::ImportFamilyRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ImportFamilyResponse>,
            tonic::Status,
        >;
        async fn import_update_family(
            &self,
            request: tonic::Request<super::ImportUpdateFamilyRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ImportUpdateFamilyResponse>,
            tonic::Status,
        >;
        async fn export_family(
            &self,
            request: tonic::Request<super::ExportFamilyRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ExportFamilyResponse>,
            tonic::Status,
        >;
        async fn validate_family_name(
            &self,
            request: tonic::Request<super::ValidateFamilyNameRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ValidateFamilyNameResponse>,
            tonic::Status,
        >;
        async fn validate_family_client_key(
            &self,
            request: tonic::Request<super::ValidateFamilyClientKeyRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ValidateFamilyClientKeyResponse>,
            tonic::Status,
        >;
        async fn get_family_stat(
            &self,
            request: tonic::Request<super::GetFamilyStatRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetFamilyStatResponse>,
            tonic::Status,
        >;
        async fn get_family_stats(
            &self,
            request: tonic::Request<super::GetFamilyStatsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetFamilyStatsResponse>,
            tonic::Status,
        >;
        async fn create_family_stat_ranges(
            &self,
            request: tonic::Request<super::CreateFamilyStatRangesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateFamilyStatRangesResponse>,
            tonic::Status,
        >;
        async fn get_family_rule_dependencies(
            &self,
            request: tonic::Request<super::GetFamilyRuleDependenciesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetFamilyRuleDependenciesResponse>,
            tonic::Status,
        >;
        async fn generate_family_stat_range_name(
            &self,
            request: tonic::Request<super::GenerateFamilyStatRangeNameRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GenerateFamilyStatRangeNameResponse>,
            tonic::Status,
        >;
    }
    #[derive(Debug)]
    pub struct FamilyServiceServer<T> {
        inner: Arc<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    impl<T> FamilyServiceServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for FamilyServiceServer<T>
    where
        T: FamilyService,
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
                "/sift.families.v1.FamilyService/GetFamily" => {
                    #[allow(non_camel_case_types)]
                    struct GetFamilySvc<T: FamilyService>(pub Arc<T>);
                    impl<
                        T: FamilyService,
                    > tonic::server::UnaryService<super::GetFamilyRequest>
                    for GetFamilySvc<T> {
                        type Response = super::GetFamilyResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetFamilyRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as FamilyService>::get_family(&inner, request).await
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
                        let method = GetFamilySvc(inner);
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
                "/sift.families.v1.FamilyService/GetFamilies" => {
                    #[allow(non_camel_case_types)]
                    struct GetFamiliesSvc<T: FamilyService>(pub Arc<T>);
                    impl<
                        T: FamilyService,
                    > tonic::server::UnaryService<super::GetFamiliesRequest>
                    for GetFamiliesSvc<T> {
                        type Response = super::GetFamiliesResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetFamiliesRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as FamilyService>::get_families(&inner, request).await
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
                        let method = GetFamiliesSvc(inner);
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
                "/sift.families.v1.FamilyService/GetFamilyVersion" => {
                    #[allow(non_camel_case_types)]
                    struct GetFamilyVersionSvc<T: FamilyService>(pub Arc<T>);
                    impl<
                        T: FamilyService,
                    > tonic::server::UnaryService<super::GetFamilyVersionRequest>
                    for GetFamilyVersionSvc<T> {
                        type Response = super::GetFamilyVersionResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetFamilyVersionRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as FamilyService>::get_family_version(&inner, request)
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
                        let method = GetFamilyVersionSvc(inner);
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
                "/sift.families.v1.FamilyService/CreateFamily" => {
                    #[allow(non_camel_case_types)]
                    struct CreateFamilySvc<T: FamilyService>(pub Arc<T>);
                    impl<
                        T: FamilyService,
                    > tonic::server::UnaryService<super::CreateFamilyRequest>
                    for CreateFamilySvc<T> {
                        type Response = super::CreateFamilyResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateFamilyRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as FamilyService>::create_family(&inner, request).await
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
                        let method = CreateFamilySvc(inner);
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
                "/sift.families.v1.FamilyService/UpdateFamily" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateFamilySvc<T: FamilyService>(pub Arc<T>);
                    impl<
                        T: FamilyService,
                    > tonic::server::UnaryService<super::UpdateFamilyRequest>
                    for UpdateFamilySvc<T> {
                        type Response = super::UpdateFamilyResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdateFamilyRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as FamilyService>::update_family(&inner, request).await
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
                        let method = UpdateFamilySvc(inner);
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
                "/sift.families.v1.FamilyService/GetFamilyCandidateRuns" => {
                    #[allow(non_camel_case_types)]
                    struct GetFamilyCandidateRunsSvc<T: FamilyService>(pub Arc<T>);
                    impl<
                        T: FamilyService,
                    > tonic::server::UnaryService<super::GetFamilyCandidateRunsRequest>
                    for GetFamilyCandidateRunsSvc<T> {
                        type Response = super::GetFamilyCandidateRunsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetFamilyCandidateRunsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as FamilyService>::get_family_candidate_runs(
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
                        let method = GetFamilyCandidateRunsSvc(inner);
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
                "/sift.families.v1.FamilyService/GetFamilyCandidateRunCount" => {
                    #[allow(non_camel_case_types)]
                    struct GetFamilyCandidateRunCountSvc<T: FamilyService>(pub Arc<T>);
                    impl<
                        T: FamilyService,
                    > tonic::server::UnaryService<
                        super::GetFamilyCandidateRunCountRequest,
                    > for GetFamilyCandidateRunCountSvc<T> {
                        type Response = super::GetFamilyCandidateRunCountResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::GetFamilyCandidateRunCountRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as FamilyService>::get_family_candidate_run_count(
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
                        let method = GetFamilyCandidateRunCountSvc(inner);
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
                "/sift.families.v1.FamilyService/ListFamilyCandidateRuns" => {
                    #[allow(non_camel_case_types)]
                    struct ListFamilyCandidateRunsSvc<T: FamilyService>(pub Arc<T>);
                    impl<
                        T: FamilyService,
                    > tonic::server::UnaryService<super::ListFamilyCandidateRunsRequest>
                    for ListFamilyCandidateRunsSvc<T> {
                        type Response = super::ListFamilyCandidateRunsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::ListFamilyCandidateRunsRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as FamilyService>::list_family_candidate_runs(
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
                        let method = ListFamilyCandidateRunsSvc(inner);
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
                "/sift.families.v1.FamilyService/GetFamilyCandidateRunFilterFields" => {
                    #[allow(non_camel_case_types)]
                    struct GetFamilyCandidateRunFilterFieldsSvc<T: FamilyService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: FamilyService,
                    > tonic::server::UnaryService<
                        super::GetFamilyCandidateRunFilterFieldsRequest,
                    > for GetFamilyCandidateRunFilterFieldsSvc<T> {
                        type Response = super::GetFamilyCandidateRunFilterFieldsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::GetFamilyCandidateRunFilterFieldsRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as FamilyService>::get_family_candidate_run_filter_fields(
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
                        let method = GetFamilyCandidateRunFilterFieldsSvc(inner);
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
                "/sift.families.v1.FamilyService/ListFamilies" => {
                    #[allow(non_camel_case_types)]
                    struct ListFamiliesSvc<T: FamilyService>(pub Arc<T>);
                    impl<
                        T: FamilyService,
                    > tonic::server::UnaryService<super::ListFamiliesRequest>
                    for ListFamiliesSvc<T> {
                        type Response = super::ListFamiliesResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListFamiliesRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as FamilyService>::list_families(&inner, request).await
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
                        let method = ListFamiliesSvc(inner);
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
                "/sift.families.v1.FamilyService/ListFamilyVersions" => {
                    #[allow(non_camel_case_types)]
                    struct ListFamilyVersionsSvc<T: FamilyService>(pub Arc<T>);
                    impl<
                        T: FamilyService,
                    > tonic::server::UnaryService<super::ListFamilyVersionsRequest>
                    for ListFamilyVersionsSvc<T> {
                        type Response = super::ListFamilyVersionsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListFamilyVersionsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as FamilyService>::list_family_versions(&inner, request)
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
                        let method = ListFamilyVersionsSvc(inner);
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
                "/sift.families.v1.FamilyService/ListFamilyMembers" => {
                    #[allow(non_camel_case_types)]
                    struct ListFamilyMembersSvc<T: FamilyService>(pub Arc<T>);
                    impl<
                        T: FamilyService,
                    > tonic::server::UnaryService<super::ListFamilyMembersRequest>
                    for ListFamilyMembersSvc<T> {
                        type Response = super::ListFamilyMembersResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListFamilyMembersRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as FamilyService>::list_family_members(&inner, request)
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
                        let method = ListFamilyMembersSvc(inner);
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
                "/sift.families.v1.FamilyService/ListFamilyAlignments" => {
                    #[allow(non_camel_case_types)]
                    struct ListFamilyAlignmentsSvc<T: FamilyService>(pub Arc<T>);
                    impl<
                        T: FamilyService,
                    > tonic::server::UnaryService<super::ListFamilyAlignmentsRequest>
                    for ListFamilyAlignmentsSvc<T> {
                        type Response = super::ListFamilyAlignmentsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListFamilyAlignmentsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as FamilyService>::list_family_alignments(
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
                        let method = ListFamilyAlignmentsSvc(inner);
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
                "/sift.families.v1.FamilyService/ListFamilyAlignmentPoints" => {
                    #[allow(non_camel_case_types)]
                    struct ListFamilyAlignmentPointsSvc<T: FamilyService>(pub Arc<T>);
                    impl<
                        T: FamilyService,
                    > tonic::server::UnaryService<
                        super::ListFamilyAlignmentPointsRequest,
                    > for ListFamilyAlignmentPointsSvc<T> {
                        type Response = super::ListFamilyAlignmentPointsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::ListFamilyAlignmentPointsRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as FamilyService>::list_family_alignment_points(
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
                        let method = ListFamilyAlignmentPointsSvc(inner);
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
                "/sift.families.v1.FamilyService/ImportFamily" => {
                    #[allow(non_camel_case_types)]
                    struct ImportFamilySvc<T: FamilyService>(pub Arc<T>);
                    impl<
                        T: FamilyService,
                    > tonic::server::UnaryService<super::ImportFamilyRequest>
                    for ImportFamilySvc<T> {
                        type Response = super::ImportFamilyResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ImportFamilyRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as FamilyService>::import_family(&inner, request).await
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
                        let method = ImportFamilySvc(inner);
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
                "/sift.families.v1.FamilyService/ImportUpdateFamily" => {
                    #[allow(non_camel_case_types)]
                    struct ImportUpdateFamilySvc<T: FamilyService>(pub Arc<T>);
                    impl<
                        T: FamilyService,
                    > tonic::server::UnaryService<super::ImportUpdateFamilyRequest>
                    for ImportUpdateFamilySvc<T> {
                        type Response = super::ImportUpdateFamilyResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ImportUpdateFamilyRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as FamilyService>::import_update_family(&inner, request)
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
                        let method = ImportUpdateFamilySvc(inner);
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
                "/sift.families.v1.FamilyService/ExportFamily" => {
                    #[allow(non_camel_case_types)]
                    struct ExportFamilySvc<T: FamilyService>(pub Arc<T>);
                    impl<
                        T: FamilyService,
                    > tonic::server::UnaryService<super::ExportFamilyRequest>
                    for ExportFamilySvc<T> {
                        type Response = super::ExportFamilyResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ExportFamilyRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as FamilyService>::export_family(&inner, request).await
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
                        let method = ExportFamilySvc(inner);
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
                "/sift.families.v1.FamilyService/ValidateFamilyName" => {
                    #[allow(non_camel_case_types)]
                    struct ValidateFamilyNameSvc<T: FamilyService>(pub Arc<T>);
                    impl<
                        T: FamilyService,
                    > tonic::server::UnaryService<super::ValidateFamilyNameRequest>
                    for ValidateFamilyNameSvc<T> {
                        type Response = super::ValidateFamilyNameResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ValidateFamilyNameRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as FamilyService>::validate_family_name(&inner, request)
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
                        let method = ValidateFamilyNameSvc(inner);
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
                "/sift.families.v1.FamilyService/ValidateFamilyClientKey" => {
                    #[allow(non_camel_case_types)]
                    struct ValidateFamilyClientKeySvc<T: FamilyService>(pub Arc<T>);
                    impl<
                        T: FamilyService,
                    > tonic::server::UnaryService<super::ValidateFamilyClientKeyRequest>
                    for ValidateFamilyClientKeySvc<T> {
                        type Response = super::ValidateFamilyClientKeyResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::ValidateFamilyClientKeyRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as FamilyService>::validate_family_client_key(
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
                        let method = ValidateFamilyClientKeySvc(inner);
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
                "/sift.families.v1.FamilyService/GetFamilyStat" => {
                    #[allow(non_camel_case_types)]
                    struct GetFamilyStatSvc<T: FamilyService>(pub Arc<T>);
                    impl<
                        T: FamilyService,
                    > tonic::server::UnaryService<super::GetFamilyStatRequest>
                    for GetFamilyStatSvc<T> {
                        type Response = super::GetFamilyStatResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetFamilyStatRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as FamilyService>::get_family_stat(&inner, request).await
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
                        let method = GetFamilyStatSvc(inner);
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
                "/sift.families.v1.FamilyService/GetFamilyStats" => {
                    #[allow(non_camel_case_types)]
                    struct GetFamilyStatsSvc<T: FamilyService>(pub Arc<T>);
                    impl<
                        T: FamilyService,
                    > tonic::server::UnaryService<super::GetFamilyStatsRequest>
                    for GetFamilyStatsSvc<T> {
                        type Response = super::GetFamilyStatsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetFamilyStatsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as FamilyService>::get_family_stats(&inner, request)
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
                        let method = GetFamilyStatsSvc(inner);
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
                "/sift.families.v1.FamilyService/CreateFamilyStatRanges" => {
                    #[allow(non_camel_case_types)]
                    struct CreateFamilyStatRangesSvc<T: FamilyService>(pub Arc<T>);
                    impl<
                        T: FamilyService,
                    > tonic::server::UnaryService<super::CreateFamilyStatRangesRequest>
                    for CreateFamilyStatRangesSvc<T> {
                        type Response = super::CreateFamilyStatRangesResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateFamilyStatRangesRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as FamilyService>::create_family_stat_ranges(
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
                        let method = CreateFamilyStatRangesSvc(inner);
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
                "/sift.families.v1.FamilyService/GetFamilyRuleDependencies" => {
                    #[allow(non_camel_case_types)]
                    struct GetFamilyRuleDependenciesSvc<T: FamilyService>(pub Arc<T>);
                    impl<
                        T: FamilyService,
                    > tonic::server::UnaryService<
                        super::GetFamilyRuleDependenciesRequest,
                    > for GetFamilyRuleDependenciesSvc<T> {
                        type Response = super::GetFamilyRuleDependenciesResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::GetFamilyRuleDependenciesRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as FamilyService>::get_family_rule_dependencies(
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
                        let method = GetFamilyRuleDependenciesSvc(inner);
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
                "/sift.families.v1.FamilyService/GenerateFamilyStatRangeName" => {
                    #[allow(non_camel_case_types)]
                    struct GenerateFamilyStatRangeNameSvc<T: FamilyService>(pub Arc<T>);
                    impl<
                        T: FamilyService,
                    > tonic::server::UnaryService<
                        super::GenerateFamilyStatRangeNameRequest,
                    > for GenerateFamilyStatRangeNameSvc<T> {
                        type Response = super::GenerateFamilyStatRangeNameResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::GenerateFamilyStatRangeNameRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as FamilyService>::generate_family_stat_range_name(
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
                        let method = GenerateFamilyStatRangeNameSvc(inner);
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
    impl<T> Clone for FamilyServiceServer<T> {
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
    pub const SERVICE_NAME: &str = "sift.families.v1.FamilyService";
    impl<T> tonic::server::NamedService for FamilyServiceServer<T> {
        const NAME: &'static str = SERVICE_NAME;
    }
}
