use async_trait::async_trait;
use mockall::mock;
use sift_rs::metadata::v1::{
    ArchiveMetadataKeysRequest, ArchiveMetadataKeysResponse, ArchiveMetadataValuesRequest,
    ArchiveMetadataValuesResponse, CreateMetadataKeyRequest, CreateMetadataKeyResponse,
    CreateMetadataValueRequest, CreateMetadataValueResponse, DeleteMetadataKeysRequest,
    DeleteMetadataKeysResponse, DeleteMetadataValuesRequest, DeleteMetadataValuesResponse,
    ListMetadataKeysRequest, ListMetadataKeysResponse, ListMetadataUsageRequest,
    ListMetadataUsageResponse, ListMetadataValuesRequest, ListMetadataValuesResponse,
    UnarchiveMetadataKeysRequest, UnarchiveMetadataKeysResponse, UnarchiveMetadataValuesRequest,
    UnarchiveMetadataValuesResponse, metadata_service_server::MetadataService,
};
use tonic::{Request, Response, Status};

mock! {
    pub MetadataServiceImpl {}

    #[async_trait]
    impl MetadataService for MetadataServiceImpl {
        async fn create_metadata_key(
            &self,
            request: Request<CreateMetadataKeyRequest>,
        ) -> std::result::Result<
            Response<CreateMetadataKeyResponse>,
            Status,
        >;
        async fn create_metadata_value(
            &self,
            request: Request<CreateMetadataValueRequest>,
        ) -> std::result::Result<
            Response<CreateMetadataValueResponse>,
            Status,
        >;
        async fn list_metadata_keys(
            &self,
            request: Request<ListMetadataKeysRequest>,
        ) -> std::result::Result<
            Response<ListMetadataKeysResponse>,
            Status,
        >;
        async fn list_metadata_values(
            &self,
            request: Request<ListMetadataValuesRequest>,
        ) -> std::result::Result<
            Response<ListMetadataValuesResponse>,
            Status,
        >;
        async fn archive_metadata_keys(
            &self,
            request: Request<ArchiveMetadataKeysRequest>,
        ) -> std::result::Result<
            Response<ArchiveMetadataKeysResponse>,
            Status,
        >;
        async fn archive_metadata_values(
            &self,
            request: Request<ArchiveMetadataValuesRequest>,
        ) -> std::result::Result<
            Response<ArchiveMetadataValuesResponse>,
            Status,
        >;
        async fn unarchive_metadata_keys(
            &self,
            request: Request<UnarchiveMetadataKeysRequest>,
        ) -> std::result::Result<
            Response<UnarchiveMetadataKeysResponse>,
            Status,
        >;
        async fn unarchive_metadata_values(
            &self,
            request: Request<UnarchiveMetadataValuesRequest>,
        ) -> std::result::Result<
            Response<UnarchiveMetadataValuesResponse>,
            Status,
        >;
        async fn delete_metadata_keys(
            &self,
            request: Request<DeleteMetadataKeysRequest>,
        ) -> std::result::Result<
            Response<DeleteMetadataKeysResponse>,
            Status,
        >;
        async fn delete_metadata_values(
            &self,
            request: Request<DeleteMetadataValuesRequest>,
        ) -> std::result::Result<
            Response<DeleteMetadataValuesResponse>,
            Status,
        >;
        async fn list_metadata_usage(
            &self,
            request: Request<ListMetadataUsageRequest>,
        ) -> std::result::Result<
            Response<ListMetadataUsageResponse>,
            Status,
        >;
    }
}
