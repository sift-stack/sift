use async_trait::async_trait;
use mockall::mock;
use sift_rs::me::v2::{GetMeRequest, GetMeResponse, me_service_server::MeService};
use tonic::{Request, Response, Status};

mock! {
    pub MeServiceImpl {}

    #[async_trait]
    impl MeService for MeServiceImpl {
        async fn get_me(
            &self,
            request: Request<GetMeRequest>,
        ) -> std::result::Result<
            Response<GetMeResponse>,
            Status,
        >;
    }
}
