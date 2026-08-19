use async_trait::async_trait;
use mockall::mock;
use sift_rs::campaigns::v1::{
    CreateCampaignRequest, CreateCampaignResponse, GetCampaignReportSummariesRequest,
    GetCampaignReportSummariesResponse, GetCampaignRequest, GetCampaignResponse,
    ListCampaignAnnotationsRequest, ListCampaignAnnotationsResponse, ListCampaignsRequest,
    ListCampaignsResponse, UpdateCampaignRequest, UpdateCampaignResponse,
    campaign_service_server::CampaignService,
};
use tonic::{Request, Response, Status};

mock! {
    pub CampaignServiceImpl {}

    #[async_trait]
    impl CampaignService for CampaignServiceImpl {
        async fn get_campaign(
            &self,
            request: Request<GetCampaignRequest>,
        ) -> std::result::Result<
            Response<GetCampaignResponse>,
            Status,
        >;
        async fn create_campaign(
            &self,
            request: Request<CreateCampaignRequest>,
        ) -> std::result::Result<
            Response<CreateCampaignResponse>,
            Status,
        >;
        async fn list_campaigns(
            &self,
            request: Request<ListCampaignsRequest>,
        ) -> std::result::Result<
            Response<ListCampaignsResponse>,
            Status,
        >;
        async fn update_campaign(
            &self,
            request: Request<UpdateCampaignRequest>,
        ) -> std::result::Result<
            Response<UpdateCampaignResponse>,
            Status,
        >;
        async fn list_campaign_annotations(
            &self,
            request: Request<ListCampaignAnnotationsRequest>,
        ) -> std::result::Result<
            Response<ListCampaignAnnotationsResponse>,
            Status,
        >;
        async fn get_campaign_report_summaries(
            &self,
            request: Request<GetCampaignReportSummariesRequest>,
        ) -> std::result::Result<
            Response<GetCampaignReportSummariesResponse>,
            Status,
        >;
    }
}
