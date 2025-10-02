use hyper_util::rt::TokioIo;
use pbjson_types::Timestamp;
use sift_connect::{SiftChannel, grpc::interceptor::AuthInterceptor};
use sift_rs::assets::v1::asset_service_server::{AssetService, AssetServiceServer};
use sift_rs::assets::v1::{
    ArchiveAssetRequest, ArchiveAssetResponse, Asset, DeleteAssetRequest, DeleteAssetResponse,
    GetAssetRequest, GetAssetResponse, ListAssetsRequest, ListAssetsResponse, UpdateAssetRequest,
    UpdateAssetResponse,
};
use sift_rs::ingest::v1::ingest_service_server::{IngestService, IngestServiceServer};
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
use sift_stream::{ChannelConfig, ChannelDataType, FlowConfig};
use std::io::Error as IoError;
use tokio::task::JoinHandle;
use tonic::transport::{Endpoint, Server, Uri};
use tonic::{Request, Response, Status};
use tower::{ServiceBuilder, service_fn};

/// re-exports everything needed to implement an [IngestService].
pub mod prelude;

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
            flows: vec![FlowConfig {
                name: "123".to_string(),
                channels: vec![ChannelConfig {
                    name: "generator".to_string(),
                    data_type: ChannelDataType::Double.into(),
                    ..Default::default()
                }],
            }],
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

pub async fn start_test_ingest_server<I: IngestService>(
    ingest_service: I,
) -> (SiftChannel, JoinHandle<()>) {
    let (client, server) = tokio::io::duplex(1024);

    let server = tokio::spawn(async move {
        Server::builder()
            .add_service(IngestServiceServer::new(ingest_service))
            .add_service(PingServiceServer::new(MockPingService))
            .add_service(IngestionConfigServiceServer::new(
                MockIngestionConfigService,
            ))
            .add_service(AssetServiceServer::new(MockAssetService))
            .add_service(RunServiceServer::new(MockRunService))
            .serve_with_incoming(tokio_stream::once(Ok::<_, IoError>(server)))
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

    let sift_channel: SiftChannel = ServiceBuilder::new()
        .layer(tonic::service::interceptor(AuthInterceptor {
            apikey: "apikey".to_string(),
        }))
        .service(channel);

    (sift_channel, server)
}
