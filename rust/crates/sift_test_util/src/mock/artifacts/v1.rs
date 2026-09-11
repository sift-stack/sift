use async_trait::async_trait;
use mockall::mock;
use sift_rs::artifacts::v1::{
    ArchiveArtifactRequest, ArchiveArtifactResponse, CreateArtifactRequest, CreateArtifactResponse,
    GetArtifactRequest, GetArtifactResponse, LinkArtifactToConversationRequest,
    LinkArtifactToConversationResponse, ListArtifactVersionsRequest, ListArtifactVersionsResponse,
    ListArtifactsRequest, ListArtifactsResponse, UnarchiveArtifactRequest,
    UnarchiveArtifactResponse, UnlinkArtifactFromConversationRequest,
    UnlinkArtifactFromConversationResponse, artifact_service_server::ArtifactService,
};
use tonic::{Request, Response, Status};

mock! {
    pub ArtifactServiceImpl {}

    #[async_trait]
    impl ArtifactService for ArtifactServiceImpl {
        async fn create_artifact(
            &self,
            request: Request<CreateArtifactRequest>,
        ) -> std::result::Result<
            Response<CreateArtifactResponse>,
            Status,
        >;
        async fn get_artifact(
            &self,
            request: Request<GetArtifactRequest>,
        ) -> std::result::Result<
            Response<GetArtifactResponse>,
            Status,
        >;
        async fn list_artifacts(
            &self,
            request: Request<ListArtifactsRequest>,
        ) -> std::result::Result<
            Response<ListArtifactsResponse>,
            Status,
        >;
        async fn list_artifact_versions(
            &self,
            request: Request<ListArtifactVersionsRequest>,
        ) -> std::result::Result<
            Response<ListArtifactVersionsResponse>,
            Status,
        >;
        async fn link_artifact_to_conversation(
            &self,
            request: Request<LinkArtifactToConversationRequest>,
        ) -> std::result::Result<
            Response<LinkArtifactToConversationResponse>,
            Status,
        >;
        async fn unlink_artifact_from_conversation(
            &self,
            request: Request<UnlinkArtifactFromConversationRequest>,
        ) -> std::result::Result<
            Response<UnlinkArtifactFromConversationResponse>,
            Status,
        >;
        async fn archive_artifact(
            &self,
            request: Request<ArchiveArtifactRequest>,
        ) -> std::result::Result<
            Response<ArchiveArtifactResponse>,
            Status,
        >;
        async fn unarchive_artifact(
            &self,
            request: Request<UnarchiveArtifactRequest>,
        ) -> std::result::Result<
            Response<UnarchiveArtifactResponse>,
            Status,
        >;
    }
}
