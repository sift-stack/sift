use async_trait::async_trait;
use mockall::mock;
use sift_rs::docs::v1::{
    ReadDocRequest, ReadDocResponse, SearchDocsRequest, SearchDocsResponse,
    docs_service_server::DocsService,
};
use tonic::{Request, Response, Status};

mock! {
    pub DocsServiceImpl {}

    #[async_trait]
    impl DocsService for DocsServiceImpl {
        async fn search_docs(
            &self,
            request: Request<SearchDocsRequest>,
        ) -> std::result::Result<
            Response<SearchDocsResponse>,
            Status,
        >;
        async fn read_doc(
            &self,
            request: Request<ReadDocRequest>,
        ) -> std::result::Result<
            Response<ReadDocResponse>,
            Status,
        >;
    }
}
