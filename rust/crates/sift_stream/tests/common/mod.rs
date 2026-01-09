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
use std::sync::{Arc, Mutex};
use tokio::task::JoinHandle;
use tonic::transport::{Endpoint, Server, Uri};
use tonic::{Request, Response, Status};
use tower::{ServiceBuilder, service_fn};
use uuid::Uuid;

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

pub(crate) struct MockIngestionConfigService {
    existing_flows: Arc<Mutex<Vec<FlowConfig>>>,
    existing_ingestion_configs: Arc<Mutex<Vec<IngestionConfig>>>,
}

impl Default for MockIngestionConfigService {
    fn default() -> Self {
        let existing_flow = FlowConfig {
            name: "flow-0".to_string(),
            channels: vec![ChannelConfig {
                name: "generator".to_string(),
                data_type: ChannelDataType::Double.into(),
                ..Default::default()
            }],
        };
        let existing_ingestion_config = IngestionConfig {
            ingestion_config_id: Uuid::new_v4().to_string(),
            asset_id: "test_asset".to_string(),
            client_key: "test_client_key".to_string(),
        };
        Self {
            existing_flows: Arc::new(Mutex::new(vec![existing_flow])),
            existing_ingestion_configs: Arc::new(Mutex::new(vec![existing_ingestion_config])),
        }
    }
}

#[tonic::async_trait]
impl IngestionConfigService for MockIngestionConfigService {
    async fn get_ingestion_config(
        &self,
        request: Request<GetIngestionConfigRequest>,
    ) -> Result<Response<GetIngestionConfigResponse>, Status> {
        let get_ingestion_config = request.into_inner();
        let existing_ingestion_configs = self.existing_ingestion_configs.lock().unwrap();
        let ingestion_config = existing_ingestion_configs
            .iter()
            .find(|ic| ic.ingestion_config_id == get_ingestion_config.ingestion_config_id);
        if let Some(ingestion_config) = ingestion_config {
            return Ok(Response::new(GetIngestionConfigResponse {
                ingestion_config: Some(ingestion_config.clone()),
            }));
        }

        Err(Status::not_found("ingestion config not found"))
    }
    async fn create_ingestion_config(
        &self,
        request: Request<CreateIngestionConfigRequest>,
    ) -> Result<Response<CreateIngestionConfigResponse>, Status> {
        let create_ingestion_config = request.into_inner();

        let mut existing_ingestion_configs = self.existing_ingestion_configs.lock().unwrap();
        for config in existing_ingestion_configs.iter() {
            if config.client_key == create_ingestion_config.client_key {
                return Err(Status::already_exists("ingestion config already exists"));
            }
        }

        let new_config = IngestionConfig {
            ingestion_config_id: Uuid::new_v4().to_string(),
            asset_id: create_ingestion_config.asset_name.clone(),
            client_key: create_ingestion_config.client_key.clone(),
        };

        existing_ingestion_configs.push(new_config.clone());

        Ok(Response::new(CreateIngestionConfigResponse {
            ingestion_config: Some(new_config),
        }))
    }
    async fn list_ingestion_configs(
        &self,
        request: Request<ListIngestionConfigsRequest>,
    ) -> Result<Response<ListIngestionConfigsResponse>, Status> {
        let list_configs: ListIngestionConfigsRequest = request.into_inner();

        let existing_ingestion_configs = self.existing_ingestion_configs.lock().unwrap();

        let mut ingestion_configs = Vec::new();
        for config in existing_ingestion_configs.iter() {
            if list_configs.filter.is_empty() || list_configs.filter.contains(&config.client_key) {
                ingestion_configs.push(config.clone());
            }
        }

        Ok(Response::new(ListIngestionConfigsResponse {
            ingestion_configs: ingestion_configs,
            next_page_token: "".to_string(),
        }))
    }
    async fn create_ingestion_config_flows(
        &self,
        request: Request<CreateIngestionConfigFlowsRequest>,
    ) -> Result<Response<CreateIngestionConfigFlowsResponse>, Status> {
        let create_flows = request.into_inner();

        let mut existing_flows = self.existing_flows.lock().unwrap();
        for flow in create_flows.flows.iter() {
            if existing_flows.iter().any(|f| f.name == flow.name) {
                return Err(Status::already_exists("flow already exists"));
            }

            existing_flows.push(flow.clone());
        }

        Ok(Response::new(CreateIngestionConfigFlowsResponse {}))
    }
    async fn list_ingestion_config_flows(
        &self,
        request: Request<ListIngestionConfigFlowsRequest>,
    ) -> Result<Response<ListIngestionConfigFlowsResponse>, Status> {
        let list_flows: ListIngestionConfigFlowsRequest = request.into_inner();
        let existing_flows = self.existing_flows.lock().unwrap();

        // If the filter contains "already_exists_flow" or is empty, return the existing flow.
        let mut flows = Vec::new();
        for flow in existing_flows.iter() {
            if list_flows.filter.is_empty() || list_flows.filter.contains(&flow.name) {
                flows.push(flow.clone());
            }
        }

        Ok(Response::new(ListIngestionConfigFlowsResponse {
            flows,
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
                asset_id: "asset-0".to_string(),
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
            duration: None,
            is_archived: false,
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
                MockIngestionConfigService::default(),
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
