use crate::SiftChannel;
use hyper_util::rt::TokioIo;
use pbjson_types::Timestamp;
use sift_connect::grpc::interceptor::AuthInterceptor;
use sift_rs::assets::v1::asset_service_server::{AssetService, AssetServiceServer};
use sift_rs::assets::v1::{
    ArchiveAssetRequest, ArchiveAssetResponse, Asset, DeleteAssetRequest, DeleteAssetResponse,
    GetAssetRequest, GetAssetResponse, ListAssetsRequest, ListAssetsResponse, UpdateAssetRequest,
    UpdateAssetResponse,
};
use sift_rs::ingest::v1::IngestWithConfigDataStreamRequest;
use sift_rs::ingest::v1::{
    IngestWithConfigDataStreamResponse,
    ingest_service_server::{IngestService, IngestServiceServer},
};
use sift_rs::ingestion_configs::v2::ingestion_config_service_server::{
    IngestionConfigService, IngestionConfigServiceServer,
};
use sift_rs::ingestion_configs::v2::{
    CreateIngestionConfigFlowsRequest, CreateIngestionConfigFlowsResponse,
    CreateIngestionConfigRequest, CreateIngestionConfigResponse, GetIngestionConfigRequest,
    GetIngestionConfigResponse, IngestionConfig, ListIngestionConfigFlowsRequest,
    ListIngestionConfigFlowsResponse, ListIngestionConfigsRequest, ListIngestionConfigsResponse,
};
use sift_rs::ping::v1::ping_service_server::{PingService, PingServiceServer};
use sift_rs::ping::v1::{PingRequest, PingResponse};
use sift_rs::runs::v2::run_service_server::{RunService, RunServiceServer};
use sift_rs::runs::v2::{
    CreateAdhocRunRequest, CreateAdhocRunResponse, CreateAutomaticRunAssociationForAssetsRequest,
    CreateAutomaticRunAssociationForAssetsResponse, CreateRunRequest, CreateRunResponse,
    DeleteRunRequest, DeleteRunResponse, GetRunRequest, GetRunResponse, ListRunsRequest,
    ListRunsResponse, Run, StopRunRequest, StopRunResponse, UpdateRunRequest, UpdateRunResponse,
};
use std::sync::{Arc, Mutex};
use tonic::transport::{Endpoint, Server, Uri};
use tonic::{Request, Response, Status};
use tower::{ServiceBuilder, service_fn};

pub(crate) struct MockPingService;

#[tonic::async_trait]
impl PingService for MockPingService {
    async fn ping(&self, _: Request<PingRequest>) -> Result<Response<PingResponse>, Status> {
        Ok(Response::new(PingResponse {
            response: "Hello from a sift test!".to_string(),
        }))
    }
}

pub(crate) struct MockIngestionConfigService;

#[tonic::async_trait]
impl IngestionConfigService for MockIngestionConfigService {
    async fn get_ingestion_config(
        &self,
        _: Request<GetIngestionConfigRequest>,
    ) -> Result<Response<GetIngestionConfigResponse>, Status> {
        Ok(Response::new(GetIngestionConfigResponse {
            ingestion_config: Some(IngestionConfig {
                ingestion_config_id: "123".to_string(),
                asset_id: "123".to_string(),
                client_key: "test_client_key".to_string(),
            }),
        }))
    }
    async fn create_ingestion_config(
        &self,
        _: Request<CreateIngestionConfigRequest>,
    ) -> Result<Response<CreateIngestionConfigResponse>, Status> {
        Ok(Response::new(CreateIngestionConfigResponse {
            ingestion_config: Some(IngestionConfig {
                ingestion_config_id: "123".to_string(),
                asset_id: "123".to_string(),
                client_key: "test_client_key".to_string(),
            }),
        }))
    }
    async fn list_ingestion_configs(
        &self,
        _: Request<ListIngestionConfigsRequest>,
    ) -> Result<Response<ListIngestionConfigsResponse>, Status> {
        println!("list_ingestion_configs");
        Ok(Response::new(ListIngestionConfigsResponse {
            ingestion_configs: vec![IngestionConfig {
                ingestion_config_id: "123".to_string(),
                asset_id: "123".to_string(),
                client_key: "test_client_key".to_string(),
            }],
            next_page_token: "".to_string(),
        }))
    }
    async fn create_ingestion_config_flows(
        &self,
        _: Request<CreateIngestionConfigFlowsRequest>,
    ) -> Result<Response<CreateIngestionConfigFlowsResponse>, Status> {
        Ok(Response::new(CreateIngestionConfigFlowsResponse {}))
    }
    async fn list_ingestion_config_flows(
        &self,
        _: Request<ListIngestionConfigFlowsRequest>,
    ) -> Result<Response<ListIngestionConfigFlowsResponse>, Status> {
        Ok(Response::new(ListIngestionConfigFlowsResponse {
            flows: vec![],
            next_page_token: "".to_string(),
        }))
    }
}

pub(crate) struct MockAssetService;

#[tonic::async_trait]
impl AssetService for MockAssetService {
    async fn get_asset(
        &self,
        _: Request<GetAssetRequest>,
    ) -> Result<Response<GetAssetResponse>, Status> {
        Ok(Response::new(GetAssetResponse {
            asset: Some(Asset {
                asset_id: "123".to_string(),
                name: "test_asset".to_string(),
                organization_id: "test".to_string(),
                created_by_user_id: "test".to_string(),
                modified_by_user_id: "test".to_string(),
                created_date: Some(Timestamp {
                    seconds: 1,
                    nanos: 0,
                }),
                modified_date: Some(Timestamp {
                    seconds: 1,
                    nanos: 0,
                }),
                tags: vec!["test".to_string()],
                metadata: vec![],
                archived_date: None,
                is_archived: false,
            }),
        }))
    }
    async fn update_asset(
        &self,
        _: Request<UpdateAssetRequest>,
    ) -> Result<Response<UpdateAssetResponse>, Status> {
        Err(Status::unimplemented("Not implemented for test"))
    }
    async fn delete_asset(
        &self,
        _: Request<DeleteAssetRequest>,
    ) -> Result<Response<DeleteAssetResponse>, Status> {
        Err(Status::unimplemented("Not implemented for test"))
    }
    async fn list_assets(
        &self,
        _: Request<ListAssetsRequest>,
    ) -> Result<Response<ListAssetsResponse>, Status> {
        Err(Status::unimplemented("Not implemented for test"))
    }
    async fn archive_asset(
        &self,
        _: Request<ArchiveAssetRequest>,
    ) -> Result<Response<ArchiveAssetResponse>, Status> {
        Err(Status::unimplemented("Not implemented for test"))
    }
}

pub(crate) struct MockRunService;

impl MockRunService {
    fn run() -> Run {
        Run {
            run_id: "123".to_string(),
            name: "test_run".to_string(),
            description: "test_run".to_string(),
            created_date: Some(Timestamp {
                seconds: 1,
                nanos: 0,
            }),
            modified_date: Some(Timestamp {
                seconds: 1,
                nanos: 0,
            }),
            created_by_user_id: "test".to_string(),
            modified_by_user_id: "test".to_string(),
            organization_id: "test".to_string(),
            start_time: None,
            stop_time: None,
            is_pinned: false,
            tags: vec!["test".to_string()],
            default_report_id: "".to_string(),
            client_key: None,
            metadata: vec![],
            asset_ids: vec!["123".to_string()],
            archived_date: None,
            is_adhoc: false,
        }
    }
}

#[tonic::async_trait]
impl RunService for MockRunService {
    async fn get_run(&self, _: Request<GetRunRequest>) -> Result<Response<GetRunResponse>, Status> {
        Ok(Response::new(GetRunResponse {
            run: Some(Self::run()),
        }))
    }
    async fn list_runs(
        &self,
        _: Request<ListRunsRequest>,
    ) -> Result<Response<ListRunsResponse>, Status> {
        Ok(Response::new(ListRunsResponse {
            runs: vec![Self::run()],
            next_page_token: "".to_string(),
        }))
    }
    async fn create_run(
        &self,
        _: Request<CreateRunRequest>,
    ) -> Result<Response<CreateRunResponse>, Status> {
        Ok(Response::new(CreateRunResponse {
            run: Some(Self::run()),
        }))
    }

    async fn create_adhoc_run(
        &self,
        _: Request<CreateAdhocRunRequest>,
    ) -> Result<Response<CreateAdhocRunResponse>, Status> {
        Err(Status::unimplemented("Not implemented for test"))
    }
    async fn update_run(
        &self,
        _: Request<UpdateRunRequest>,
    ) -> Result<Response<UpdateRunResponse>, Status> {
        Err(Status::unimplemented("Not implemented for test"))
    }
    async fn delete_run(
        &self,
        _: Request<DeleteRunRequest>,
    ) -> Result<Response<DeleteRunResponse>, Status> {
        Err(Status::unimplemented("Not implemented for test"))
    }
    async fn stop_run(
        &self,
        _: Request<StopRunRequest>,
    ) -> Result<Response<StopRunResponse>, Status> {
        Err(Status::unimplemented("Not implemented for test"))
    }
    async fn create_automatic_run_association_for_assets(
        &self,
        _: Request<CreateAutomaticRunAssociationForAssetsRequest>,
    ) -> Result<Response<CreateAutomaticRunAssociationForAssetsResponse>, Status> {
        Err(Status::unimplemented("Not implemented for test"))
    }
}

#[derive(Debug, Clone)]
pub(crate) struct MockIngestService {
    captured_data: Arc<Mutex<Vec<IngestWithConfigDataStreamRequest>>>,
}

impl MockIngestService {
    pub(crate) fn new() -> Self {
        Self {
            captured_data: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub(crate) fn get_captured_data(&self) -> Vec<IngestWithConfigDataStreamRequest> {
        self.captured_data.lock().unwrap().clone()
    }
}

#[tonic::async_trait]
impl IngestService for MockIngestService {
    async fn ingest_with_config_data_stream(
        &self,
        request: Request<tonic::Streaming<IngestWithConfigDataStreamRequest>>,
    ) -> Result<Response<IngestWithConfigDataStreamResponse>, Status> {
        let mut stream = request.into_inner();

        while let Some(data) = stream.message().await? {
            self.captured_data.lock().unwrap().push(data);
        }

        Ok(Response::new(IngestWithConfigDataStreamResponse {}))
    }

    async fn ingest_arbitrary_protobuf_data_stream(
        &self,
        _request: Request<
            tonic::Streaming<sift_rs::ingest::v1::IngestArbitraryProtobufDataStreamRequest>,
        >,
    ) -> Result<Response<sift_rs::ingest::v1::IngestArbitraryProtobufDataStreamResponse>, Status>
    {
        Err(Status::unimplemented("Not implemented for test"))
    }
}

pub(crate) async fn create_mock_grpc_channel_with_service() -> (SiftChannel, MockIngestService) {
    // Create a mock ingest service.
    let mock_ingest_service = MockIngestService::new();
    let mock_ingest_service_clone = mock_ingest_service.clone();

    let (client, server) = tokio::io::duplex(1024);

    tokio::spawn(async move {
        Server::builder()
            .add_service(IngestServiceServer::new(mock_ingest_service_clone))
            .add_service(PingServiceServer::new(MockPingService))
            .add_service(IngestionConfigServiceServer::new(
                MockIngestionConfigService,
            ))
            .add_service(AssetServiceServer::new(MockAssetService))
            .add_service(RunServiceServer::new(MockRunService))
            .serve_with_incoming(tokio_stream::once(Ok::<_, std::io::Error>(server)))
            .await
            .unwrap();
    });

    let mut client = Some(client);
    let channel = Endpoint::try_from("http://[::]:50051")
        .unwrap()
        .connect_with_connector(service_fn(move |_: Uri| {
            let client = client.take();

            async move {
                if let Some(client) = client {
                    Ok(TokioIo::new(client))
                } else {
                    Err(std::io::Error::other("Client already taken"))
                }
            }
        }))
        .await
        .unwrap();

    let sift_channel = ServiceBuilder::new()
        .layer(tonic::service::interceptor(AuthInterceptor {
            apikey: "test-api-key".to_string(),
        }))
        .service(channel);

    (sift_channel, mock_ingest_service)
}
