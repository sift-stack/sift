use async_trait::async_trait;
use mockall::mock;
use sift_rs::remote_files::v1::{
    BatchDeleteRemoteFilesRequest, BatchDeleteRemoteFilesResponse, CreateRemoteFileRequest,
    CreateRemoteFileResponse, DeleteRemoteFileRequest, DeleteRemoteFileResponse,
    GetRemoteFileDownloadUrlRequest, GetRemoteFileDownloadUrlResponse, GetRemoteFileRequest,
    GetRemoteFileResponse, ListRemoteFilesRequest, ListRemoteFilesResponse,
    UpdateRemoteFileRequest, UpdateRemoteFileResponse,
    remote_file_service_server::RemoteFileService,
};
use tonic::{Request, Response, Status};

mock! {
    pub RemoteFileServiceImpl {}

    #[async_trait]
    impl RemoteFileService for RemoteFileServiceImpl {
        async fn get_remote_file(
            &self,
            request: Request<GetRemoteFileRequest>,
        ) -> std::result::Result<
            Response<GetRemoteFileResponse>,
            Status,
        >;
        async fn create_remote_file(
            &self,
            request: Request<CreateRemoteFileRequest>,
        ) -> std::result::Result<
            Response<CreateRemoteFileResponse>,
            Status,
        >;
        async fn list_remote_files(
            &self,
            request: Request<ListRemoteFilesRequest>,
        ) -> std::result::Result<
            Response<ListRemoteFilesResponse>,
            Status,
        >;
        async fn update_remote_file(
            &self,
            request: Request<UpdateRemoteFileRequest>,
        ) -> std::result::Result<
            Response<UpdateRemoteFileResponse>,
            Status,
        >;
        async fn delete_remote_file(
            &self,
            request: Request<DeleteRemoteFileRequest>,
        ) -> std::result::Result<
            Response<DeleteRemoteFileResponse>,
            Status,
        >;
        async fn batch_delete_remote_files(
            &self,
            request: Request<BatchDeleteRemoteFilesRequest>,
        ) -> std::result::Result<
            Response<BatchDeleteRemoteFilesResponse>,
            Status,
        >;
        async fn get_remote_file_download_url(
            &self,
            request: Request<GetRemoteFileDownloadUrlRequest>,
        ) -> std::result::Result<
            Response<GetRemoteFileDownloadUrlResponse>,
            Status,
        >;
    }
}
