use async_trait::async_trait;
use mockall::mock;
use sift_rs::reports::v1::{
    CancelReportRequest, CancelReportResponse, CreateReportRequest, CreateReportResponse,
    GetReportRequest, GetReportResponse, ListReportsRequest, ListReportsResponse,
    RerunReportRequest, RerunReportResponse, UpdateReportRequest, UpdateReportResponse,
    report_service_server::ReportService,
};
use tonic::{Request, Response, Status};

mock! {
    pub ReportServiceImpl {}

    #[async_trait]
    impl ReportService for ReportServiceImpl {
        async fn get_report(
            &self,
            request: Request<GetReportRequest>,
        ) -> std::result::Result<
            Response<GetReportResponse>,
            Status,
        >;
        async fn create_report(
            &self,
            request: Request<CreateReportRequest>,
        ) -> std::result::Result<
            Response<CreateReportResponse>,
            Status,
        >;
        async fn update_report(
            &self,
            request: Request<UpdateReportRequest>,
        ) -> std::result::Result<
            Response<UpdateReportResponse>,
            Status,
        >;
        async fn list_reports(
            &self,
            request: Request<ListReportsRequest>,
        ) -> std::result::Result<
            Response<ListReportsResponse>,
            Status,
        >;
        async fn rerun_report(
            &self,
            request: Request<RerunReportRequest>,
        ) -> std::result::Result<
            Response<RerunReportResponse>,
            Status,
        >;
        async fn cancel_report(
            &self,
            request: Request<CancelReportRequest>,
        ) -> std::result::Result<
            Response<CancelReportResponse>,
            Status,
        >;
    }
}
