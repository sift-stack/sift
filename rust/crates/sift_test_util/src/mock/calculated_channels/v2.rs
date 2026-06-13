use async_trait::async_trait;
use mockall::mock;
use sift_rs::calculated_channels::v2::{
    BatchResolveCalculatedChannelsRequest, BatchResolveCalculatedChannelsResponse,
    CreateCalculatedChannelRequest, CreateCalculatedChannelResponse,
    GetCalculatedChannelDependentsRequest, GetCalculatedChannelDependentsResponse,
    GetCalculatedChannelRequest, GetCalculatedChannelResponse, GetCalculatedChannelVersionsRequest,
    GetCalculatedChannelVersionsResponse, ListCalculatedChannelVersionsRequest,
    ListCalculatedChannelVersionsResponse, ListCalculatedChannelsRequest,
    ListCalculatedChannelsResponse, ListResolvedCalculatedChannelsRequest,
    ListResolvedCalculatedChannelsResponse, ResolveCalculatedChannelRequest,
    ResolveCalculatedChannelResponse, UpdateCalculatedChannelRequest,
    UpdateCalculatedChannelResponse, calculated_channel_service_server::CalculatedChannelService,
};
use tonic::{Request, Response, Status};

mock! {
    pub CalculatedChannelServiceImpl {}

    #[async_trait]
    impl CalculatedChannelService for CalculatedChannelServiceImpl {
        async fn get_calculated_channel(
            &self,
            request: Request<GetCalculatedChannelRequest>,
        ) -> std::result::Result<Response<GetCalculatedChannelResponse>, Status>;
        async fn create_calculated_channel(
            &self,
            request: Request<CreateCalculatedChannelRequest>,
        ) -> std::result::Result<Response<CreateCalculatedChannelResponse>, Status>;
        async fn list_calculated_channels(
            &self,
            request: Request<ListCalculatedChannelsRequest>,
        ) -> std::result::Result<Response<ListCalculatedChannelsResponse>, Status>;
        async fn update_calculated_channel(
            &self,
            request: Request<UpdateCalculatedChannelRequest>,
        ) -> std::result::Result<Response<UpdateCalculatedChannelResponse>, Status>;
        async fn list_calculated_channel_versions(
            &self,
            request: Request<ListCalculatedChannelVersionsRequest>,
        ) -> std::result::Result<Response<ListCalculatedChannelVersionsResponse>, Status>;
        async fn resolve_calculated_channel(
            &self,
            request: Request<ResolveCalculatedChannelRequest>,
        ) -> std::result::Result<Response<ResolveCalculatedChannelResponse>, Status>;
        async fn batch_resolve_calculated_channels(
            &self,
            request: Request<BatchResolveCalculatedChannelsRequest>,
        ) -> std::result::Result<Response<BatchResolveCalculatedChannelsResponse>, Status>;
        async fn list_resolved_calculated_channels(
            &self,
            request: Request<ListResolvedCalculatedChannelsRequest>,
        ) -> std::result::Result<Response<ListResolvedCalculatedChannelsResponse>, Status>;
        async fn get_calculated_channel_versions(
            &self,
            request: Request<GetCalculatedChannelVersionsRequest>,
        ) -> std::result::Result<Response<GetCalculatedChannelVersionsResponse>, Status>;
        async fn get_calculated_channel_dependents(
            &self,
            request: Request<GetCalculatedChannelDependentsRequest>,
        ) -> std::result::Result<Response<GetCalculatedChannelDependentsResponse>, Status>;
    }
}
