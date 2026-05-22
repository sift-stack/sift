use async_trait::async_trait;
use mockall::mock;
use sift_rs::data::v2::{GetDataRequest, GetDataResponse, data_service_server::DataService};
use tonic::{Request, Response, Status};

mock! {
    pub DataServiceImpl {}

    #[async_trait]
    impl DataService for DataServiceImpl {
        async fn get_data(
            &self,
            request: Request<GetDataRequest>,
        ) -> std::result::Result<
            Response<GetDataResponse>,
            Status,
        >;
    }
}
