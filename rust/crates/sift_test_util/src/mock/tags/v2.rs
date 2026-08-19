use async_trait::async_trait;
use mockall::mock;
use sift_rs::tags::v2::{
    CreateTagRequest, CreateTagResponse, ListTagsRequest, ListTagsResponse,
    tag_service_server::TagService,
};
use tonic::{Request, Response, Status};

mock! {
    pub TagServiceImpl {}

    #[async_trait]
    impl TagService for TagServiceImpl {
        async fn create_tag(
            &self,
            request: Request<CreateTagRequest>,
        ) -> std::result::Result<
            Response<CreateTagResponse>,
            Status,
        >;
        async fn list_tags(
            &self,
            request: Request<ListTagsRequest>,
        ) -> std::result::Result<
            Response<ListTagsResponse>,
            Status,
        >;
    }
}
