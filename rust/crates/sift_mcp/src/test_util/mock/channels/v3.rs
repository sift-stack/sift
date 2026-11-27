use async_trait::async_trait;
use mockall::mock;
use sift_rs::channels::v3::{
    BatchArchiveChannelsRequest, BatchArchiveChannelsResponse, BatchUnarchiveChannelsRequest,
    BatchUnarchiveChannelsResponse, GetChannelRequest, GetChannelResponse, ListChannelsRequest,
    ListChannelsResponse, UpdateChannelRequest, UpdateChannelResponse,
    channel_service_server::ChannelService,
};
use tonic::{Request, Response, Status};

mock! {
    pub ChannelServiceImpl {}

    #[async_trait]
    impl ChannelService for ChannelServiceImpl {
        async fn get_channel(
            &self,
            request: Request<GetChannelRequest>,
        ) -> std::result::Result<
            Response<GetChannelResponse>,
            Status,
        >;
        async fn list_channels(
            &self,
            request: Request<ListChannelsRequest>,
        ) -> std::result::Result<
            Response<ListChannelsResponse>,
            Status,
        >;
        async fn update_channel(
            &self,
            request: Request<UpdateChannelRequest>,
        ) -> std::result::Result<
            Response<UpdateChannelResponse>,
            Status,
        >;
        async fn batch_archive_channels(
            &self,
            request: Request<BatchArchiveChannelsRequest>,
        ) -> std::result::Result<
            Response<BatchArchiveChannelsResponse>,
            Status,
        >;
        async fn batch_unarchive_channels(
            &self,
            request: Request<BatchUnarchiveChannelsRequest>,
        ) -> std::result::Result<
            Response<BatchUnarchiveChannelsResponse>,
            Status,
        >;
    }
}
