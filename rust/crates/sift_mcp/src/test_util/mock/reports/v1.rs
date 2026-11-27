use async_trait::async_trait;
use mockall::mock;
use sift_rs::reports::v1::{
    CancelReportRequest, CancelReportResponse, CreateReportRequest, CreateReportResponse,
    GetReportRequest, GetReportResponse, ListReportMetadataValuesRequest,
    ListReportMetadataValuesResponse, ListReportRuleSummariesRequest,
    ListReportRuleSummariesResponse, ListReportsRequest, ListReportsResponse,
    ListReportsWithCumulativeSummaryRequest, ListReportsWithCumulativeSummaryResponse,
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
        async fn list_report_rule_summaries(
            &self,
            request: Request<ListReportRuleSummariesRequest>,
        ) -> std::result::Result<
            Response<ListReportRuleSummariesResponse>,
            Status,
        >;
        async fn list_report_metadata_values(
            &self,
            request: Request<ListReportMetadataValuesRequest>,
        ) -> std::result::Result<
            Response<ListReportMetadataValuesResponse>,
            Status,
        >;
        async fn list_reports_with_cumulative_summary(
            &self,
            request: Request<ListReportsWithCumulativeSummaryRequest>,
        ) -> std::result::Result<
            Response<ListReportsWithCumulativeSummaryResponse>,
            Status,
        >;
    }
}
