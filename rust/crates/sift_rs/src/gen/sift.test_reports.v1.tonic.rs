// @generated
/// Generated client implementations.
pub mod test_report_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    ///
    #[derive(Debug, Clone)]
    pub struct TestReportServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl TestReportServiceClient<tonic::transport::Channel> {
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
    impl<T> TestReportServiceClient<T>
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
        ) -> TestReportServiceClient<InterceptedService<T, F>>
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
            TestReportServiceClient::new(InterceptedService::new(inner, interceptor))
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
        pub async fn import_test_report(
            &mut self,
            request: impl tonic::IntoRequest<super::ImportTestReportRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ImportTestReportResponse>,
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
                "/sift.test_reports.v1.TestReportService/ImportTestReport",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sift.test_reports.v1.TestReportService",
                        "ImportTestReport",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /** Creates a test report
*/
        pub async fn create_test_report(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateTestReportRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateTestReportResponse>,
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
                "/sift.test_reports.v1.TestReportService/CreateTestReport",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sift.test_reports.v1.TestReportService",
                        "CreateTestReport",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /** Gets a single test report
*/
        pub async fn get_test_report(
            &mut self,
            request: impl tonic::IntoRequest<super::GetTestReportRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetTestReportResponse>,
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
                "/sift.test_reports.v1.TestReportService/GetTestReport",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sift.test_reports.v1.TestReportService",
                        "GetTestReport",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /** Lists test reports with optional filtering
*/
        pub async fn list_test_reports(
            &mut self,
            request: impl tonic::IntoRequest<super::ListTestReportsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListTestReportsResponse>,
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
                "/sift.test_reports.v1.TestReportService/ListTestReports",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sift.test_reports.v1.TestReportService",
                        "ListTestReports",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /** Updates a test report
*/
        pub async fn update_test_report(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateTestReportRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateTestReportResponse>,
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
                "/sift.test_reports.v1.TestReportService/UpdateTestReport",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sift.test_reports.v1.TestReportService",
                        "UpdateTestReport",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /** Deletes a test report
*/
        pub async fn delete_test_report(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteTestReportRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DeleteTestReportResponse>,
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
                "/sift.test_reports.v1.TestReportService/DeleteTestReport",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sift.test_reports.v1.TestReportService",
                        "DeleteTestReport",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /** Creates a test step
*/
        pub async fn create_test_step(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateTestStepRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateTestStepResponse>,
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
                "/sift.test_reports.v1.TestReportService/CreateTestStep",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sift.test_reports.v1.TestReportService",
                        "CreateTestStep",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /** Lists test steps with optional filtering
*/
        pub async fn list_test_steps(
            &mut self,
            request: impl tonic::IntoRequest<super::ListTestStepsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListTestStepsResponse>,
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
                "/sift.test_reports.v1.TestReportService/ListTestSteps",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sift.test_reports.v1.TestReportService",
                        "ListTestSteps",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /** Updates a test step
*/
        pub async fn update_test_step(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateTestStepRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateTestStepResponse>,
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
                "/sift.test_reports.v1.TestReportService/UpdateTestStep",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sift.test_reports.v1.TestReportService",
                        "UpdateTestStep",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /** Deletes a test step
*/
        pub async fn delete_test_step(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteTestStepRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DeleteTestStepResponse>,
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
                "/sift.test_reports.v1.TestReportService/DeleteTestStep",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sift.test_reports.v1.TestReportService",
                        "DeleteTestStep",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /** Creates a test measurement
*/
        pub async fn create_test_measurement(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateTestMeasurementRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateTestMeasurementResponse>,
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
                "/sift.test_reports.v1.TestReportService/CreateTestMeasurement",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sift.test_reports.v1.TestReportService",
                        "CreateTestMeasurement",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /** Creates multiple test measurements in a single request
*/
        pub async fn create_test_measurements(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateTestMeasurementsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateTestMeasurementsResponse>,
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
                "/sift.test_reports.v1.TestReportService/CreateTestMeasurements",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sift.test_reports.v1.TestReportService",
                        "CreateTestMeasurements",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_test_measurements(
            &mut self,
            request: impl tonic::IntoRequest<super::ListTestMeasurementsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListTestMeasurementsResponse>,
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
                "/sift.test_reports.v1.TestReportService/ListTestMeasurements",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sift.test_reports.v1.TestReportService",
                        "ListTestMeasurements",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn count_test_steps(
            &mut self,
            request: impl tonic::IntoRequest<super::CountTestStepsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CountTestStepsResponse>,
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
                "/sift.test_reports.v1.TestReportService/CountTestSteps",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sift.test_reports.v1.TestReportService",
                        "CountTestSteps",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn count_test_measurements(
            &mut self,
            request: impl tonic::IntoRequest<super::CountTestMeasurementsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CountTestMeasurementsResponse>,
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
                "/sift.test_reports.v1.TestReportService/CountTestMeasurements",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sift.test_reports.v1.TestReportService",
                        "CountTestMeasurements",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn update_test_measurement(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateTestMeasurementRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateTestMeasurementResponse>,
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
                "/sift.test_reports.v1.TestReportService/UpdateTestMeasurement",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sift.test_reports.v1.TestReportService",
                        "UpdateTestMeasurement",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn delete_test_measurement(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteTestMeasurementRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DeleteTestMeasurementResponse>,
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
                "/sift.test_reports.v1.TestReportService/DeleteTestMeasurement",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sift.test_reports.v1.TestReportService",
                        "DeleteTestMeasurement",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod test_report_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with TestReportServiceServer.
    #[async_trait]
    pub trait TestReportService: Send + Sync + 'static {
        async fn import_test_report(
            &self,
            request: tonic::Request<super::ImportTestReportRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ImportTestReportResponse>,
            tonic::Status,
        >;
        /** Creates a test report
*/
        async fn create_test_report(
            &self,
            request: tonic::Request<super::CreateTestReportRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateTestReportResponse>,
            tonic::Status,
        >;
        /** Gets a single test report
*/
        async fn get_test_report(
            &self,
            request: tonic::Request<super::GetTestReportRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetTestReportResponse>,
            tonic::Status,
        >;
        /** Lists test reports with optional filtering
*/
        async fn list_test_reports(
            &self,
            request: tonic::Request<super::ListTestReportsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListTestReportsResponse>,
            tonic::Status,
        >;
        /** Updates a test report
*/
        async fn update_test_report(
            &self,
            request: tonic::Request<super::UpdateTestReportRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateTestReportResponse>,
            tonic::Status,
        >;
        /** Deletes a test report
*/
        async fn delete_test_report(
            &self,
            request: tonic::Request<super::DeleteTestReportRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DeleteTestReportResponse>,
            tonic::Status,
        >;
        /** Creates a test step
*/
        async fn create_test_step(
            &self,
            request: tonic::Request<super::CreateTestStepRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateTestStepResponse>,
            tonic::Status,
        >;
        /** Lists test steps with optional filtering
*/
        async fn list_test_steps(
            &self,
            request: tonic::Request<super::ListTestStepsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListTestStepsResponse>,
            tonic::Status,
        >;
        /** Updates a test step
*/
        async fn update_test_step(
            &self,
            request: tonic::Request<super::UpdateTestStepRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateTestStepResponse>,
            tonic::Status,
        >;
        /** Deletes a test step
*/
        async fn delete_test_step(
            &self,
            request: tonic::Request<super::DeleteTestStepRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DeleteTestStepResponse>,
            tonic::Status,
        >;
        /** Creates a test measurement
*/
        async fn create_test_measurement(
            &self,
            request: tonic::Request<super::CreateTestMeasurementRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateTestMeasurementResponse>,
            tonic::Status,
        >;
        /** Creates multiple test measurements in a single request
*/
        async fn create_test_measurements(
            &self,
            request: tonic::Request<super::CreateTestMeasurementsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateTestMeasurementsResponse>,
            tonic::Status,
        >;
        async fn list_test_measurements(
            &self,
            request: tonic::Request<super::ListTestMeasurementsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListTestMeasurementsResponse>,
            tonic::Status,
        >;
        async fn count_test_steps(
            &self,
            request: tonic::Request<super::CountTestStepsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CountTestStepsResponse>,
            tonic::Status,
        >;
        async fn count_test_measurements(
            &self,
            request: tonic::Request<super::CountTestMeasurementsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CountTestMeasurementsResponse>,
            tonic::Status,
        >;
        async fn update_test_measurement(
            &self,
            request: tonic::Request<super::UpdateTestMeasurementRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateTestMeasurementResponse>,
            tonic::Status,
        >;
        async fn delete_test_measurement(
            &self,
            request: tonic::Request<super::DeleteTestMeasurementRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DeleteTestMeasurementResponse>,
            tonic::Status,
        >;
    }
    ///
    #[derive(Debug)]
    pub struct TestReportServiceServer<T: TestReportService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: TestReportService> TestReportServiceServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for TestReportServiceServer<T>
    where
        T: TestReportService,
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
                "/sift.test_reports.v1.TestReportService/ImportTestReport" => {
                    #[allow(non_camel_case_types)]
                    struct ImportTestReportSvc<T: TestReportService>(pub Arc<T>);
                    impl<
                        T: TestReportService,
                    > tonic::server::UnaryService<super::ImportTestReportRequest>
                    for ImportTestReportSvc<T> {
                        type Response = super::ImportTestReportResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ImportTestReportRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as TestReportService>::import_test_report(
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
                        let method = ImportTestReportSvc(inner);
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
                "/sift.test_reports.v1.TestReportService/CreateTestReport" => {
                    #[allow(non_camel_case_types)]
                    struct CreateTestReportSvc<T: TestReportService>(pub Arc<T>);
                    impl<
                        T: TestReportService,
                    > tonic::server::UnaryService<super::CreateTestReportRequest>
                    for CreateTestReportSvc<T> {
                        type Response = super::CreateTestReportResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateTestReportRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as TestReportService>::create_test_report(
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
                        let method = CreateTestReportSvc(inner);
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
                "/sift.test_reports.v1.TestReportService/GetTestReport" => {
                    #[allow(non_camel_case_types)]
                    struct GetTestReportSvc<T: TestReportService>(pub Arc<T>);
                    impl<
                        T: TestReportService,
                    > tonic::server::UnaryService<super::GetTestReportRequest>
                    for GetTestReportSvc<T> {
                        type Response = super::GetTestReportResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetTestReportRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as TestReportService>::get_test_report(&inner, request)
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
                        let method = GetTestReportSvc(inner);
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
                "/sift.test_reports.v1.TestReportService/ListTestReports" => {
                    #[allow(non_camel_case_types)]
                    struct ListTestReportsSvc<T: TestReportService>(pub Arc<T>);
                    impl<
                        T: TestReportService,
                    > tonic::server::UnaryService<super::ListTestReportsRequest>
                    for ListTestReportsSvc<T> {
                        type Response = super::ListTestReportsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListTestReportsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as TestReportService>::list_test_reports(&inner, request)
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
                        let method = ListTestReportsSvc(inner);
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
                "/sift.test_reports.v1.TestReportService/UpdateTestReport" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateTestReportSvc<T: TestReportService>(pub Arc<T>);
                    impl<
                        T: TestReportService,
                    > tonic::server::UnaryService<super::UpdateTestReportRequest>
                    for UpdateTestReportSvc<T> {
                        type Response = super::UpdateTestReportResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdateTestReportRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as TestReportService>::update_test_report(
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
                        let method = UpdateTestReportSvc(inner);
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
                "/sift.test_reports.v1.TestReportService/DeleteTestReport" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteTestReportSvc<T: TestReportService>(pub Arc<T>);
                    impl<
                        T: TestReportService,
                    > tonic::server::UnaryService<super::DeleteTestReportRequest>
                    for DeleteTestReportSvc<T> {
                        type Response = super::DeleteTestReportResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteTestReportRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as TestReportService>::delete_test_report(
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
                        let method = DeleteTestReportSvc(inner);
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
                "/sift.test_reports.v1.TestReportService/CreateTestStep" => {
                    #[allow(non_camel_case_types)]
                    struct CreateTestStepSvc<T: TestReportService>(pub Arc<T>);
                    impl<
                        T: TestReportService,
                    > tonic::server::UnaryService<super::CreateTestStepRequest>
                    for CreateTestStepSvc<T> {
                        type Response = super::CreateTestStepResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateTestStepRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as TestReportService>::create_test_step(&inner, request)
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
                        let method = CreateTestStepSvc(inner);
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
                "/sift.test_reports.v1.TestReportService/ListTestSteps" => {
                    #[allow(non_camel_case_types)]
                    struct ListTestStepsSvc<T: TestReportService>(pub Arc<T>);
                    impl<
                        T: TestReportService,
                    > tonic::server::UnaryService<super::ListTestStepsRequest>
                    for ListTestStepsSvc<T> {
                        type Response = super::ListTestStepsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListTestStepsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as TestReportService>::list_test_steps(&inner, request)
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
                        let method = ListTestStepsSvc(inner);
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
                "/sift.test_reports.v1.TestReportService/UpdateTestStep" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateTestStepSvc<T: TestReportService>(pub Arc<T>);
                    impl<
                        T: TestReportService,
                    > tonic::server::UnaryService<super::UpdateTestStepRequest>
                    for UpdateTestStepSvc<T> {
                        type Response = super::UpdateTestStepResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdateTestStepRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as TestReportService>::update_test_step(&inner, request)
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
                        let method = UpdateTestStepSvc(inner);
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
                "/sift.test_reports.v1.TestReportService/DeleteTestStep" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteTestStepSvc<T: TestReportService>(pub Arc<T>);
                    impl<
                        T: TestReportService,
                    > tonic::server::UnaryService<super::DeleteTestStepRequest>
                    for DeleteTestStepSvc<T> {
                        type Response = super::DeleteTestStepResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteTestStepRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as TestReportService>::delete_test_step(&inner, request)
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
                        let method = DeleteTestStepSvc(inner);
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
                "/sift.test_reports.v1.TestReportService/CreateTestMeasurement" => {
                    #[allow(non_camel_case_types)]
                    struct CreateTestMeasurementSvc<T: TestReportService>(pub Arc<T>);
                    impl<
                        T: TestReportService,
                    > tonic::server::UnaryService<super::CreateTestMeasurementRequest>
                    for CreateTestMeasurementSvc<T> {
                        type Response = super::CreateTestMeasurementResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateTestMeasurementRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as TestReportService>::create_test_measurement(
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
                        let method = CreateTestMeasurementSvc(inner);
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
                "/sift.test_reports.v1.TestReportService/CreateTestMeasurements" => {
                    #[allow(non_camel_case_types)]
                    struct CreateTestMeasurementsSvc<T: TestReportService>(pub Arc<T>);
                    impl<
                        T: TestReportService,
                    > tonic::server::UnaryService<super::CreateTestMeasurementsRequest>
                    for CreateTestMeasurementsSvc<T> {
                        type Response = super::CreateTestMeasurementsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateTestMeasurementsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as TestReportService>::create_test_measurements(
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
                        let method = CreateTestMeasurementsSvc(inner);
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
                "/sift.test_reports.v1.TestReportService/ListTestMeasurements" => {
                    #[allow(non_camel_case_types)]
                    struct ListTestMeasurementsSvc<T: TestReportService>(pub Arc<T>);
                    impl<
                        T: TestReportService,
                    > tonic::server::UnaryService<super::ListTestMeasurementsRequest>
                    for ListTestMeasurementsSvc<T> {
                        type Response = super::ListTestMeasurementsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListTestMeasurementsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as TestReportService>::list_test_measurements(
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
                        let method = ListTestMeasurementsSvc(inner);
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
                "/sift.test_reports.v1.TestReportService/CountTestSteps" => {
                    #[allow(non_camel_case_types)]
                    struct CountTestStepsSvc<T: TestReportService>(pub Arc<T>);
                    impl<
                        T: TestReportService,
                    > tonic::server::UnaryService<super::CountTestStepsRequest>
                    for CountTestStepsSvc<T> {
                        type Response = super::CountTestStepsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CountTestStepsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as TestReportService>::count_test_steps(&inner, request)
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
                        let method = CountTestStepsSvc(inner);
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
                "/sift.test_reports.v1.TestReportService/CountTestMeasurements" => {
                    #[allow(non_camel_case_types)]
                    struct CountTestMeasurementsSvc<T: TestReportService>(pub Arc<T>);
                    impl<
                        T: TestReportService,
                    > tonic::server::UnaryService<super::CountTestMeasurementsRequest>
                    for CountTestMeasurementsSvc<T> {
                        type Response = super::CountTestMeasurementsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CountTestMeasurementsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as TestReportService>::count_test_measurements(
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
                        let method = CountTestMeasurementsSvc(inner);
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
                "/sift.test_reports.v1.TestReportService/UpdateTestMeasurement" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateTestMeasurementSvc<T: TestReportService>(pub Arc<T>);
                    impl<
                        T: TestReportService,
                    > tonic::server::UnaryService<super::UpdateTestMeasurementRequest>
                    for UpdateTestMeasurementSvc<T> {
                        type Response = super::UpdateTestMeasurementResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdateTestMeasurementRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as TestReportService>::update_test_measurement(
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
                        let method = UpdateTestMeasurementSvc(inner);
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
                "/sift.test_reports.v1.TestReportService/DeleteTestMeasurement" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteTestMeasurementSvc<T: TestReportService>(pub Arc<T>);
                    impl<
                        T: TestReportService,
                    > tonic::server::UnaryService<super::DeleteTestMeasurementRequest>
                    for DeleteTestMeasurementSvc<T> {
                        type Response = super::DeleteTestMeasurementResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteTestMeasurementRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as TestReportService>::delete_test_measurement(
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
                        let method = DeleteTestMeasurementSvc(inner);
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
    impl<T: TestReportService> Clone for TestReportServiceServer<T> {
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
    impl<T: TestReportService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: TestReportService> tonic::server::NamedService
    for TestReportServiceServer<T> {
        const NAME: &'static str = "sift.test_reports.v1.TestReportService";
    }
}
