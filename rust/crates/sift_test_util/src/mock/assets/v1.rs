use async_trait::async_trait;
use sift_rs::assets::v1::{
    asset_service_server::AssetService,
    DeleteAssetRequest,
    DeleteAssetResponse,
    GetAssetRequest,
    GetAssetResponse,
    ListAssetsRequest,
    ListAssetsResponse,
    UpdateAssetRequest,
    UpdateAssetResponse,
    ArchiveAssetRequest,
    ArchiveAssetResponse,
};
use mockall::mock;
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
