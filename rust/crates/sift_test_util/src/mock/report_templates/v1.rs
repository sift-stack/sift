use async_trait::async_trait;
use mockall::mock;
use sift_rs::report_templates::v1::{
    CreateReportTemplateRequest, CreateReportTemplateResponse, GetReportTemplateRequest,
    GetReportTemplateResponse, ListReportTemplatesRequest, ListReportTemplatesResponse,
    UpdateReportTemplateRequest, UpdateReportTemplateResponse,
    report_template_service_server::ReportTemplateService,
};
use tonic::{Request, Response, Status};

mock! {
    pub ReportTemplateServiceImpl {}

    #[async_trait]
    impl ReportTemplateService for ReportTemplateServiceImpl {
        async fn get_report_template(
            &self,
            request: Request<GetReportTemplateRequest>,
        ) -> std::result::Result<
            Response<GetReportTemplateResponse>,
            Status,
        >;
        async fn create_report_template(
            &self,
            request: Request<CreateReportTemplateRequest>,
        ) -> std::result::Result<
            Response<CreateReportTemplateResponse>,
            Status,
        >;
        async fn list_report_templates(
            &self,
            request: Request<ListReportTemplatesRequest>,
        ) -> std::result::Result<
            Response<ListReportTemplatesResponse>,
            Status,
        >;
        async fn update_report_template(
            &self,
            request: Request<UpdateReportTemplateRequest>,
        ) -> std::result::Result<
            Response<UpdateReportTemplateResponse>,
            Status,
        >;
    }
}
