use async_trait::async_trait;
use mockall::mock;
use sift_rs::runs::v2::{
    CreateAdhocRunRequest, CreateAdhocRunResponse, CreateAutomaticRunAssociationForAssetsRequest,
    CreateAutomaticRunAssociationForAssetsResponse, CreateRunRequest, CreateRunResponse,
    DeleteRunRequest, DeleteRunResponse, GetFilterFieldsRequest, GetFilterFieldsResponse,
    GetRunRequest, GetRunResponse, ListRunsRequest, ListRunsResponse, StopRunRequest,
    StopRunResponse, UpdateRunRequest, UpdateRunResponse, ValidateRunFilterRequest,
    ValidateRunFilterResponse, run_service_server::RunService,
};
use tonic::{Request, Response, Status};

mock! {
    pub RunServiceImpl {}

    #[async_trait]
    impl RunService for RunServiceImpl {
        async fn get_run(
            &self,
            request: Request<GetRunRequest>,
        ) -> std::result::Result<
            Response<GetRunResponse>,
            Status,
        >;
        async fn list_runs(
            &self,
            request: Request<ListRunsRequest>,
        ) -> std::result::Result<
            Response<ListRunsResponse>,
            Status,
        >;
        async fn create_run(
            &self,
            request: Request<CreateRunRequest>,
        ) -> std::result::Result<
            Response<CreateRunResponse>,
            Status,
        >;
        async fn create_adhoc_run(
            &self,
            request: Request<CreateAdhocRunRequest>,
        ) -> std::result::Result<
            Response<CreateAdhocRunResponse>,
            Status,
        >;
        async fn update_run(
            &self,
            request: Request<UpdateRunRequest>,
        ) -> std::result::Result<
            Response<UpdateRunResponse>,
            Status,
        >;
        async fn delete_run(
            &self,
            request: Request<DeleteRunRequest>,
        ) -> std::result::Result<
            Response<DeleteRunResponse>,
            Status,
        >;
        async fn stop_run(
            &self,
            request: Request<StopRunRequest>,
        ) -> std::result::Result<
            Response<StopRunResponse>,
            Status,
        >;
        async fn get_filter_fields(
            &self,
            request: Request<GetFilterFieldsRequest>,
        ) -> std::result::Result<
            Response<GetFilterFieldsResponse>,
            Status,
        >;
        async fn validate_run_filter(
            &self,
            request: Request<ValidateRunFilterRequest>,
        ) -> std::result::Result<
            Response<ValidateRunFilterResponse>,
            Status,
        >;
        async fn create_automatic_run_association_for_assets(
            &self,
            request: Request<CreateAutomaticRunAssociationForAssetsRequest>,
        ) -> std::result::Result<
            Response<CreateAutomaticRunAssociationForAssetsResponse>,
            Status,
        >;
    }
}
