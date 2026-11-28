use async_trait::async_trait;
use mockall::mock;
use sift_rs::annotations::v1::{
    ArchiveAnnotationRequest, ArchiveAnnotationResponse, BatchArchiveAnnotationsRequest,
    BatchArchiveAnnotationsResponse, BatchDeleteAnnotationsRequest, BatchDeleteAnnotationsResponse,
    BatchUnarchiveAnnotationsRequest, BatchUnarchiveAnnotationsResponse, CreateAnnotationRequest,
    CreateAnnotationResponse, DeleteAnnotationRequest, DeleteAnnotationResponse,
    GetAnnotationRequest, GetAnnotationResponse, ListAnnotationsRequest, ListAnnotationsResponse,
    UnarchiveAnnotationRequest, UnarchiveAnnotationResponse, UpdateAnnotationRequest,
    UpdateAnnotationResponse, annotation_service_server::AnnotationService,
};
use tonic::{Request, Response, Status};

mock! {
    pub AnnotationServiceImpl {}

    #[async_trait]
    impl AnnotationService for AnnotationServiceImpl {
        async fn create_annotation(
            &self,
            request: Request<CreateAnnotationRequest>,
        ) -> std::result::Result<
            Response<CreateAnnotationResponse>,
            Status,
        >;
        async fn delete_annotation(
            &self,
            request: Request<DeleteAnnotationRequest>,
        ) -> std::result::Result<
            Response<DeleteAnnotationResponse>,
            Status,
        >;
        async fn archive_annotation(
            &self,
            request: Request<ArchiveAnnotationRequest>,
        ) -> std::result::Result<
            Response<ArchiveAnnotationResponse>,
            Status,
        >;
        async fn unarchive_annotation(
            &self,
            request: Request<UnarchiveAnnotationRequest>,
        ) -> std::result::Result<
            Response<UnarchiveAnnotationResponse>,
            Status,
        >;
        async fn batch_delete_annotations(
            &self,
            request: Request<BatchDeleteAnnotationsRequest>,
        ) -> std::result::Result<
            Response<BatchDeleteAnnotationsResponse>,
            Status,
        >;
        async fn batch_archive_annotations(
            &self,
            request: Request<BatchArchiveAnnotationsRequest>,
        ) -> std::result::Result<
            Response<BatchArchiveAnnotationsResponse>,
            Status,
        >;
        async fn batch_unarchive_annotations(
            &self,
            request: Request<BatchUnarchiveAnnotationsRequest>,
        ) -> std::result::Result<
            Response<BatchUnarchiveAnnotationsResponse>,
            Status,
        >;
        async fn list_annotations(
            &self,
            request: Request<ListAnnotationsRequest>,
        ) -> std::result::Result<
            Response<ListAnnotationsResponse>,
            Status,
        >;
        async fn get_annotation(
            &self,
            request: Request<GetAnnotationRequest>,
        ) -> std::result::Result<
            Response<GetAnnotationResponse>,
            Status,
        >;
        async fn update_annotation(
            &self,
            request: Request<UpdateAnnotationRequest>,
        ) -> std::result::Result<
            Response<UpdateAnnotationResponse>,
            Status,
        >;
    }
}
