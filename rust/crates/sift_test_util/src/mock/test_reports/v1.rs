use async_trait::async_trait;
use mockall::mock;
use sift_rs::test_reports::v1::{
    CountTestMeasurementsRequest, CountTestMeasurementsResponse, CountTestStepsRequest,
    CountTestStepsResponse, CreateTestMeasurementRequest, CreateTestMeasurementResponse,
    CreateTestMeasurementsRequest, CreateTestMeasurementsResponse, CreateTestReportRequest,
    CreateTestReportResponse, CreateTestStepRequest, CreateTestStepResponse,
    DeleteTestMeasurementRequest, DeleteTestMeasurementResponse, DeleteTestReportRequest,
    DeleteTestReportResponse, DeleteTestStepRequest, DeleteTestStepResponse, GetTestReportRequest,
    GetTestReportResponse, ImportTestReportRequest, ImportTestReportResponse,
    ListTestMeasurementsRequest, ListTestMeasurementsResponse, ListTestReportsRequest,
    ListTestReportsResponse, ListTestStepsRequest, ListTestStepsResponse,
    UpdateTestMeasurementRequest, UpdateTestMeasurementResponse, UpdateTestReportRequest,
    UpdateTestReportResponse, UpdateTestStepRequest, UpdateTestStepResponse,
    test_report_service_server::TestReportService,
};
use tonic::{Request, Response, Status};

mock! {
    pub TestReportServiceImpl {}

    #[async_trait]
    impl TestReportService for TestReportServiceImpl {
        async fn import_test_report(
            &self,
            request: Request<ImportTestReportRequest>,
        ) -> std::result::Result<Response<ImportTestReportResponse>, Status>;
        async fn create_test_report(
            &self,
            request: Request<CreateTestReportRequest>,
        ) -> std::result::Result<Response<CreateTestReportResponse>, Status>;
        async fn get_test_report(
            &self,
            request: Request<GetTestReportRequest>,
        ) -> std::result::Result<Response<GetTestReportResponse>, Status>;
        async fn list_test_reports(
            &self,
            request: Request<ListTestReportsRequest>,
        ) -> std::result::Result<Response<ListTestReportsResponse>, Status>;
        async fn update_test_report(
            &self,
            request: Request<UpdateTestReportRequest>,
        ) -> std::result::Result<Response<UpdateTestReportResponse>, Status>;
        async fn delete_test_report(
            &self,
            request: Request<DeleteTestReportRequest>,
        ) -> std::result::Result<Response<DeleteTestReportResponse>, Status>;
        async fn create_test_step(
            &self,
            request: Request<CreateTestStepRequest>,
        ) -> std::result::Result<Response<CreateTestStepResponse>, Status>;
        async fn list_test_steps(
            &self,
            request: Request<ListTestStepsRequest>,
        ) -> std::result::Result<Response<ListTestStepsResponse>, Status>;
        async fn update_test_step(
            &self,
            request: Request<UpdateTestStepRequest>,
        ) -> std::result::Result<Response<UpdateTestStepResponse>, Status>;
        async fn delete_test_step(
            &self,
            request: Request<DeleteTestStepRequest>,
        ) -> std::result::Result<Response<DeleteTestStepResponse>, Status>;
        async fn create_test_measurement(
            &self,
            request: Request<CreateTestMeasurementRequest>,
        ) -> std::result::Result<Response<CreateTestMeasurementResponse>, Status>;
        async fn create_test_measurements(
            &self,
            request: Request<CreateTestMeasurementsRequest>,
        ) -> std::result::Result<Response<CreateTestMeasurementsResponse>, Status>;
        async fn list_test_measurements(
            &self,
            request: Request<ListTestMeasurementsRequest>,
        ) -> std::result::Result<Response<ListTestMeasurementsResponse>, Status>;
        async fn count_test_steps(
            &self,
            request: Request<CountTestStepsRequest>,
        ) -> std::result::Result<Response<CountTestStepsResponse>, Status>;
        async fn count_test_measurements(
            &self,
            request: Request<CountTestMeasurementsRequest>,
        ) -> std::result::Result<Response<CountTestMeasurementsResponse>, Status>;
        async fn update_test_measurement(
            &self,
            request: Request<UpdateTestMeasurementRequest>,
        ) -> std::result::Result<Response<UpdateTestMeasurementResponse>, Status>;
        async fn delete_test_measurement(
            &self,
            request: Request<DeleteTestMeasurementRequest>,
        ) -> std::result::Result<Response<DeleteTestMeasurementResponse>, Status>;
    }
}
