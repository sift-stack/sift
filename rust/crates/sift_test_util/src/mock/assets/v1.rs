use async_trait::async_trait;
use mockall::mock;
use sift_rs::assets::v1::{
    ArchiveAssetRequest, ArchiveAssetResponse, CreateAssetRequest, CreateAssetResponse,
    DeleteAssetRequest, DeleteAssetResponse, GetAssetRequest, GetAssetResponse, ListAssetsRequest,
    ListAssetsResponse, UpdateAssetRequest, UpdateAssetResponse,
    asset_service_server::AssetService,
};
use tonic::{Request, Response, Status};

mock! {
    pub AssetServiceImpl {}

    #[async_trait]
    impl AssetService for AssetServiceImpl {
        async fn delete_asset(
            &self,
            request: Request<DeleteAssetRequest>,
        ) -> std::result::Result<
            Response<DeleteAssetResponse>,
            Status,
        >;
        async fn create_asset(
            &self,
            request: Request<CreateAssetRequest>,
        ) -> std::result::Result<
            Response<CreateAssetResponse>,
            Status,
        >;
        async fn get_asset(
            &self,
            request: Request<GetAssetRequest>,
        ) -> std::result::Result<
            Response<GetAssetResponse>,
            Status,
        >;
        async fn list_assets(
            &self,
            request: Request<ListAssetsRequest>,
        ) -> std::result::Result<
            Response<ListAssetsResponse>,
            Status,
        >;
        async fn update_asset(
            &self,
            request: Request<UpdateAssetRequest>,
        ) -> std::result::Result<
            Response<UpdateAssetResponse>,
            Status,
        >;
        async fn archive_asset(
            &self,
            request: Request<ArchiveAssetRequest>,
        ) -> std::result::Result<
            Response<ArchiveAssetResponse>,
            Status,
        >;
    }
}
