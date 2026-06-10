// Integration tests for SiftStreamAutoRegister<T>.
//
// The test suite covers:
//   - Flows are auto-registered on first send when not in the local cache.
//   - A flow is registered exactly once regardless of how many messages share the flow name.
//   - Multiple distinct flows are each registered independently.
//   - Flows that were pre-registered during stream init are not re-registered.
//   - `into_inner()` returns the stream with an up-to-date local flow cache.
//   - `finish()` drains all queued messages before returning.

use std::collections::HashMap;
use std::io::Error as IoError;
use std::sync::{
    Arc, Mutex,
    atomic::{AtomicU32, Ordering},
};

use hyper_util::rt::TokioIo;
use pbjson_types::Timestamp;
use sift_connect::{SiftChannel, grpc::interceptor::AuthInterceptor};
use sift_rs::assets::v1::asset_service_server::{AssetService, AssetServiceServer};
use sift_rs::assets::v1::{
    ArchiveAssetRequest, ArchiveAssetResponse, Asset, CreateAssetRequest, CreateAssetResponse,
    DeleteAssetRequest, DeleteAssetResponse, GetAssetRequest, GetAssetResponse, ListAssetsRequest,
    ListAssetsResponse, UpdateAssetRequest, UpdateAssetResponse,
};
use sift_rs::ingest::v1::{
    IngestArbitraryProtobufDataStreamRequest, IngestArbitraryProtobufDataStreamResponse,
    IngestWithConfigDataStreamRequest, IngestWithConfigDataStreamResponse,
    ingest_service_server::{IngestService, IngestServiceServer},
};
use sift_rs::ingestion_configs::v2::ingestion_config_service_server::{
    IngestionConfigService, IngestionConfigServiceServer,
};
use sift_rs::ingestion_configs::v2::{
    CreateIngestionConfigFlowsRequest, CreateIngestionConfigFlowsResponse,
    CreateIngestionConfigRequest, CreateIngestionConfigResponse, FlowConfig,
    GetIngestionConfigRequest, GetIngestionConfigResponse, IngestionConfig,
    ListIngestionConfigFlowsRequest, ListIngestionConfigFlowsResponse, ListIngestionConfigsRequest,
    ListIngestionConfigsResponse,
};
use sift_rs::ping::v1::ping_service_server::{PingService, PingServiceServer};
use sift_rs::ping::v1::{PingRequest, PingResponse};
use sift_rs::runs::v2::run_service_server::{RunService, RunServiceServer};
use sift_rs::runs::v2::{
    CreateAdhocRunRequest, CreateAdhocRunResponse, CreateAutomaticRunAssociationForAssetsRequest,
    CreateAutomaticRunAssociationForAssetsResponse, CreateRunRequest, CreateRunResponse,
    DeleteRunRequest, DeleteRunResponse, GetFilterFieldsRequest, GetFilterFieldsResponse,
    GetRunRequest, GetRunResponse, ListRunsRequest, ListRunsResponse, Run, StopRunRequest,
    StopRunResponse, UpdateRunRequest, UpdateRunResponse, ValidateRunFilterRequest,
    ValidateRunFilterResponse,
};
use sift_stream::{
    AutoRegisterSendError, AutoRegisterStream, ChannelConfig, ChannelDataType, ChannelValue, Flow,
    IngestionConfigForm, SiftStreamAutoRegister, SiftStreamBuilder, TimeValue,
};
use tokio::task::JoinHandle;
use tokio_stream::StreamExt;
use tonic::transport::{Endpoint, Server, Uri};
use tonic::{Request, Response, Status};
use tower::{ServiceBuilder, service_fn};
use uuid::Uuid;

mod common;

// ============================================================
// Per-flow message counter (mirrors the pattern in
// test_ingestion_config_streaming_ingestion.rs)
// ============================================================

#[derive(Clone, Default)]
struct FlowMessageCounts(Arc<Mutex<HashMap<String, u32>>>);

impl FlowMessageCounts {
    fn record(&self, flow: &str) {
        *self.0.lock().unwrap().entry(flow.to_owned()).or_insert(0) += 1;
    }

    fn get(&self, flow: &str) -> u32 {
        self.0.lock().unwrap().get(flow).copied().unwrap_or(0)
    }
}

struct FlowCountingIngestService {
    counts: FlowMessageCounts,
}

impl FlowCountingIngestService {
    fn new() -> (Self, FlowMessageCounts) {
        let counts = FlowMessageCounts::default();
        (
            Self {
                counts: counts.clone(),
            },
            counts,
        )
    }
}

#[tonic::async_trait]
impl IngestService for FlowCountingIngestService {
    async fn ingest_with_config_data_stream(
        &self,
        request: Request<tonic::Streaming<IngestWithConfigDataStreamRequest>>,
    ) -> Result<Response<IngestWithConfigDataStreamResponse>, Status> {
        let mut stream = request.into_inner();
        while let Ok(Some(msg)) = stream.try_next().await {
            self.counts.record(&msg.flow);
        }
        Ok(Response::new(IngestWithConfigDataStreamResponse {}))
    }

    async fn ingest_arbitrary_protobuf_data_stream(
        &self,
        _: Request<tonic::Streaming<IngestArbitraryProtobufDataStreamRequest>>,
    ) -> Result<Response<IngestArbitraryProtobufDataStreamResponse>, Status> {
        unimplemented!()
    }
}

// ============================================================
// Registration-counting ingestion config service
//
// Maintains a single IngestionConfig that is created on demand.
// Counts every call to create_ingestion_config_flows so tests
// can assert that auto-registration happens exactly once per
// unique flow name.
// ============================================================

struct CountingIngestionConfigService {
    registration_call_count: Arc<AtomicU32>,
    config: Arc<Mutex<Option<IngestionConfig>>>,
}

impl CountingIngestionConfigService {
    fn new(counter: Arc<AtomicU32>) -> Self {
        Self {
            registration_call_count: counter,
            config: Arc::new(Mutex::new(None)),
        }
    }
}

#[tonic::async_trait]
impl IngestionConfigService for CountingIngestionConfigService {
    async fn list_ingestion_configs(
        &self,
        request: Request<ListIngestionConfigsRequest>,
    ) -> Result<Response<ListIngestionConfigsResponse>, Status> {
        let filter = request.into_inner().filter;
        let guard = self.config.lock().unwrap();
        let configs = guard
            .iter()
            .filter(|c| filter.is_empty() || filter.contains(&c.client_key))
            .cloned()
            .collect();
        Ok(Response::new(ListIngestionConfigsResponse {
            ingestion_configs: configs,
            next_page_token: String::new(),
        }))
    }

    async fn create_ingestion_config(
        &self,
        request: Request<CreateIngestionConfigRequest>,
    ) -> Result<Response<CreateIngestionConfigResponse>, Status> {
        let r = request.into_inner();
        let new_config = IngestionConfig {
            ingestion_config_id: Uuid::new_v4().to_string(),
            asset_id: r.asset_name,
            client_key: r.client_key,
        };
        *self.config.lock().unwrap() = Some(new_config.clone());
        Ok(Response::new(CreateIngestionConfigResponse {
            ingestion_config: Some(new_config),
        }))
    }

    async fn get_ingestion_config(
        &self,
        request: Request<GetIngestionConfigRequest>,
    ) -> Result<Response<GetIngestionConfigResponse>, Status> {
        let id = request.into_inner().ingestion_config_id;
        let guard = self.config.lock().unwrap();
        match guard.as_ref().filter(|c| c.ingestion_config_id == id) {
            Some(c) => Ok(Response::new(GetIngestionConfigResponse {
                ingestion_config: Some(c.clone()),
            })),
            None => Err(Status::not_found("ingestion config not found")),
        }
    }

    async fn list_ingestion_config_flows(
        &self,
        _: Request<ListIngestionConfigFlowsRequest>,
    ) -> Result<Response<ListIngestionConfigFlowsResponse>, Status> {
        Ok(Response::new(ListIngestionConfigFlowsResponse {
            flows: vec![],
            next_page_token: String::new(),
        }))
    }

    async fn create_ingestion_config_flows(
        &self,
        _: Request<CreateIngestionConfigFlowsRequest>,
    ) -> Result<Response<CreateIngestionConfigFlowsResponse>, Status> {
        self.registration_call_count.fetch_add(1, Ordering::SeqCst);
        Ok(Response::new(CreateIngestionConfigFlowsResponse {}))
    }
}

// ============================================================
// Minimal supporting services (ping, asset, run)
// ============================================================

struct MinimalPingService;

#[tonic::async_trait]
impl PingService for MinimalPingService {
    async fn ping(&self, _: Request<PingRequest>) -> Result<Response<PingResponse>, Status> {
        Ok(Response::new(PingResponse {
            response: "pong".to_string(),
        }))
    }
}

struct MinimalAssetService;

#[tonic::async_trait]
impl AssetService for MinimalAssetService {
    async fn get_asset(
        &self,
        request: Request<GetAssetRequest>,
    ) -> Result<Response<GetAssetResponse>, Status> {
        let asset_id = request.into_inner().asset_id;
        Ok(Response::new(GetAssetResponse {
            asset: Some(Asset {
                name: asset_id.clone(),
                asset_id,
                organization_id: "test".to_string(),
                created_by_user_id: "test".to_string(),
                modified_by_user_id: "test".to_string(),
                created_date: Some(Timestamp {
                    seconds: 0,
                    nanos: 0,
                }),
                modified_date: Some(Timestamp {
                    seconds: 0,
                    nanos: 0,
                }),
                tags: vec![],
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
        Err(Status::unimplemented(""))
    }
    async fn delete_asset(
        &self,
        _: Request<DeleteAssetRequest>,
    ) -> Result<Response<DeleteAssetResponse>, Status> {
        Err(Status::unimplemented(""))
    }
    async fn list_assets(
        &self,
        _: Request<ListAssetsRequest>,
    ) -> Result<Response<ListAssetsResponse>, Status> {
        Err(Status::unimplemented(""))
    }
    async fn archive_asset(
        &self,
        _: Request<ArchiveAssetRequest>,
    ) -> Result<Response<ArchiveAssetResponse>, Status> {
        Err(Status::unimplemented(""))
    }
    async fn create_asset(
        &self,
        _: Request<CreateAssetRequest>,
    ) -> Result<Response<CreateAssetResponse>, Status> {
        Err(Status::unimplemented(""))
    }
}

fn test_run() -> Run {
    Run {
        run_id: "test-run-id".to_string(),
        name: "test-run".to_string(),
        description: String::new(),
        created_date: Some(Timestamp {
            seconds: 0,
            nanos: 0,
        }),
        modified_date: Some(Timestamp {
            seconds: 0,
            nanos: 0,
        }),
        created_by_user_id: "test".to_string(),
        modified_by_user_id: "test".to_string(),
        organization_id: "test".to_string(),
        start_time: None,
        stop_time: None,
        is_pinned: false,
        tags: vec![],
        default_report_id: String::new(),
        client_key: None,
        metadata: vec![],
        asset_ids: vec![],
        archived_date: None,
        is_adhoc: false,
        duration: None,
        is_archived: false,
    }
}

struct MinimalRunService;

#[tonic::async_trait]
impl RunService for MinimalRunService {
    async fn get_run(&self, _: Request<GetRunRequest>) -> Result<Response<GetRunResponse>, Status> {
        Ok(Response::new(GetRunResponse {
            run: Some(test_run()),
        }))
    }
    async fn list_runs(
        &self,
        _: Request<ListRunsRequest>,
    ) -> Result<Response<ListRunsResponse>, Status> {
        Ok(Response::new(ListRunsResponse {
            runs: vec![test_run()],
            next_page_token: String::new(),
        }))
    }
    async fn create_run(
        &self,
        _: Request<CreateRunRequest>,
    ) -> Result<Response<CreateRunResponse>, Status> {
        Ok(Response::new(CreateRunResponse {
            run: Some(test_run()),
        }))
    }
    async fn create_adhoc_run(
        &self,
        _: Request<CreateAdhocRunRequest>,
    ) -> Result<Response<CreateAdhocRunResponse>, Status> {
        Err(Status::unimplemented(""))
    }
    async fn update_run(
        &self,
        _: Request<UpdateRunRequest>,
    ) -> Result<Response<UpdateRunResponse>, Status> {
        Ok(Response::new(UpdateRunResponse {
            run: Some(test_run()),
        }))
    }
    async fn delete_run(
        &self,
        _: Request<DeleteRunRequest>,
    ) -> Result<Response<DeleteRunResponse>, Status> {
        Err(Status::unimplemented(""))
    }
    async fn stop_run(
        &self,
        _: Request<StopRunRequest>,
    ) -> Result<Response<StopRunResponse>, Status> {
        Err(Status::unimplemented(""))
    }
    async fn create_automatic_run_association_for_assets(
        &self,
        _: Request<CreateAutomaticRunAssociationForAssetsRequest>,
    ) -> Result<Response<CreateAutomaticRunAssociationForAssetsResponse>, Status> {
        Err(Status::unimplemented(""))
    }
    async fn get_filter_fields(
        &self,
        _: Request<GetFilterFieldsRequest>,
    ) -> Result<Response<GetFilterFieldsResponse>, Status> {
        Err(Status::unimplemented(""))
    }
    async fn validate_run_filter(
        &self,
        _: Request<ValidateRunFilterRequest>,
    ) -> Result<Response<ValidateRunFilterResponse>, Status> {
        Err(Status::unimplemented(""))
    }
}

// ============================================================
// Server helpers
// ============================================================

/// Start a server with a custom ingest service and the standard common mock services.
/// Identical in structure to `common::start_test_ingest_server`.
async fn start_standard_server(
    ingest_service: impl IngestService,
) -> (SiftChannel, JoinHandle<()>) {
    start_full_server(ingest_service, None).await
}

/// Start a server where the IngestionConfigService counts registration calls.
async fn start_server_with_registration_counter(
    ingest_service: impl IngestService,
    registration_counter: Arc<AtomicU32>,
) -> (SiftChannel, JoinHandle<()>) {
    start_full_server(
        ingest_service,
        Some(CountingIngestionConfigService::new(registration_counter)),
    )
    .await
}

async fn start_full_server(
    ingest_service: impl IngestService,
    ingestion_config_svc: Option<CountingIngestionConfigService>,
) -> (SiftChannel, JoinHandle<()>) {
    let (client_io, server_io) = tokio::io::duplex(1024);

    let server = tokio::spawn(async move {
        let mut builder = Server::builder();
        let router = builder
            .add_service(IngestServiceServer::new(ingest_service))
            .add_service(PingServiceServer::new(MinimalPingService));

        if let Some(svc) = ingestion_config_svc {
            router
                .add_service(IngestionConfigServiceServer::new(svc))
                .add_service(AssetServiceServer::new(MinimalAssetService))
                .add_service(RunServiceServer::new(MinimalRunService))
                .serve_with_incoming(tokio_stream::once(Ok::<_, IoError>(server_io)))
                .await
                .unwrap();
        } else {
            router
                .add_service(IngestionConfigServiceServer::new(
                    common::MockIngestionConfigService::default(),
                ))
                .add_service(AssetServiceServer::new(common::MockAssetService))
                .add_service(RunServiceServer::new(common::MockRunService))
                .serve_with_incoming(tokio_stream::once(Ok::<_, IoError>(server_io)))
                .await
                .unwrap();
        }
    });

    let mut client_io_opt = Some(client_io);
    let channel = Endpoint::try_from("http://[::]:50051")
        .unwrap()
        .connect_with_connector(service_fn(move |_: Uri| {
            let io = client_io_opt.take();
            async move {
                io.map(TokioIo::new)
                    .ok_or_else(|| std::io::Error::other("connector already used"))
            }
        }))
        .await
        .unwrap();

    let sift_channel: SiftChannel = ServiceBuilder::new()
        .layer(tonic::service::interceptor::InterceptorLayer::new(
            AuthInterceptor {
                apikey: "test".to_string(),
            },
        ))
        .service(channel);

    (sift_channel, server)
}

// ============================================================
// Tests
// ============================================================

#[tokio::test]
async fn test_auto_register_sends_flow_not_in_initial_config() {
    let (service, counts) = FlowCountingIngestService::new();
    let (client, server) = start_standard_server(service).await;

    let stream = SiftStreamBuilder::from_channel(client)
        .ingestion_config(IngestionConfigForm {
            asset_name: "test_asset".to_string(),
            client_key: "test_client_key".to_string(),
            flows: vec![],
        })
        .live_with_backups()
        .build()
        .await
        .expect("failed to build stream");

    let mut auto = SiftStreamAutoRegister::new(stream, vec![]);

    let n = 20u32;
    for i in 0..n {
        auto.send(Flow::new(
            "dynamic-flow",
            TimeValue::default(),
            &[ChannelValue::new("value", i as f64)],
        ))
        .await
        .expect("send failed");
    }

    auto.finish().await.expect("finish failed");

    assert_eq!(counts.get("dynamic-flow"), n);
    assert!(server.await.is_ok());
}

#[tokio::test]
async fn test_auto_register_flow_in_local_cache_after_first_send() {
    let (service, _counts) = FlowCountingIngestService::new();
    let (client, server) = start_standard_server(service).await;

    let stream = SiftStreamBuilder::from_channel(client)
        .ingestion_config(IngestionConfigForm {
            asset_name: "test_asset".to_string(),
            client_key: "test_client_key".to_string(),
            flows: vec![],
        })
        .live_with_backups()
        .build()
        .await
        .expect("failed to build stream");

    let auto = SiftStreamAutoRegister::new(stream, vec![]);

    // Before first send: flow is not in the local cache.
    assert!(
        auto.into_inner().get_flow_descriptor("new-flow").is_err(),
        "flow should not be cached before any send"
    );

    // Rebuild (into_inner consumed the previous wrapper).
    let (service2, _) = FlowCountingIngestService::new();
    let (client2, server2) = start_standard_server(service2).await;

    let stream2 = SiftStreamBuilder::from_channel(client2)
        .ingestion_config(IngestionConfigForm {
            asset_name: "test_asset".to_string(),
            client_key: "test_client_key".to_string(),
            flows: vec![],
        })
        .live_with_backups()
        .build()
        .await
        .expect("failed to build stream");

    let mut auto2 = SiftStreamAutoRegister::new(stream2, vec![]);

    auto2
        .send(Flow::new(
            "new-flow",
            TimeValue::default(),
            &[ChannelValue::new("x", 1.0_f64)],
        ))
        .await
        .expect("send failed");

    // After first send: flow is in the local cache.
    let inner = auto2.into_inner();
    assert!(
        inner.get_flow_descriptor("new-flow").is_ok(),
        "flow should be cached after first send"
    );

    inner.finish().await.expect("finish failed");
    let _ = server.await;
    let _ = server2.await;
}

#[tokio::test]
async fn test_auto_register_multiple_distinct_flows_each_registered_independently() {
    let (service, counts) = FlowCountingIngestService::new();
    let (client, server) = start_standard_server(service).await;

    let stream = SiftStreamBuilder::from_channel(client)
        .ingestion_config(IngestionConfigForm {
            asset_name: "test_asset".to_string(),
            client_key: "test_client_key".to_string(),
            flows: vec![],
        })
        .live_with_backups()
        .build()
        .await
        .expect("failed to build stream");

    let mut auto = SiftStreamAutoRegister::new(stream, vec![]);

    for i in 0..10u32 {
        auto.send(Flow::new(
            "flow-alpha",
            TimeValue::default(),
            &[ChannelValue::new("a", i as f64)],
        ))
        .await
        .expect("send failed");

        auto.send(Flow::new(
            "flow-beta",
            TimeValue::default(),
            &[ChannelValue::new("b", i as f64)],
        ))
        .await
        .expect("send failed");
    }

    // Both flows should be in the cache after sending.
    let inner = auto.into_inner();
    assert!(inner.get_flow_descriptor("flow-alpha").is_ok());
    assert!(inner.get_flow_descriptor("flow-beta").is_ok());

    inner.finish().await.expect("finish failed");

    assert_eq!(counts.get("flow-alpha"), 10);
    assert_eq!(counts.get("flow-beta"), 10);
    assert!(server.await.is_ok());
}

#[tokio::test]
async fn test_auto_register_same_flow_registered_exactly_once() {
    let registration_counter = Arc::new(AtomicU32::new(0));
    let (service, counts) = FlowCountingIngestService::new();
    let (client, server) =
        start_server_with_registration_counter(service, registration_counter.clone()).await;

    let stream = SiftStreamBuilder::from_channel(client)
        .ingestion_config(IngestionConfigForm {
            asset_name: "test_asset".to_string(),
            client_key: "auto-register-test-key".to_string(),
            flows: vec![],
        })
        .live_with_backups()
        .build()
        .await
        .expect("failed to build stream");

    let mut auto = SiftStreamAutoRegister::new(stream, vec![]);

    let n = 100u32;
    for i in 0..n {
        auto.send(Flow::new(
            "repeated-flow",
            TimeValue::default(),
            &[ChannelValue::new("v", i as f64)],
        ))
        .await
        .expect("send failed");
    }

    auto.finish().await.expect("finish failed");

    // The registration endpoint should be hit exactly once — only on the first send.
    assert_eq!(
        registration_counter.load(Ordering::SeqCst),
        1,
        "flow should be registered exactly once regardless of message count"
    );

    assert_eq!(counts.get("repeated-flow"), n);
    assert!(server.await.is_ok());
}

#[tokio::test]
async fn test_auto_register_pre_registered_flow_skips_registration() {
    // "flow-0" is pre-registered in MockIngestionConfigService::default() and will
    // be present in the local cache after build(). SiftStreamAutoRegister should
    // skip registration entirely and send directly.
    let (service, counts) = FlowCountingIngestService::new();

    // Use the standard server (which starts with flow-0 pre-registered).
    let (client, server) = start_standard_server(service).await;

    let stream = SiftStreamBuilder::from_channel(client)
        .ingestion_config(IngestionConfigForm {
            asset_name: "test_asset".to_string(),
            client_key: "test_client_key".to_string(),
            flows: vec![FlowConfig {
                name: "flow-0".to_string(),
                channels: vec![ChannelConfig {
                    name: "generator".to_string(),
                    data_type: ChannelDataType::Double.into(),
                    ..Default::default()
                }],
            }],
        })
        .live_with_backups()
        .build()
        .await
        .expect("failed to build stream");

    // flow-0 is in the cache from build(); no registration call should happen.
    let mut auto = SiftStreamAutoRegister::new(stream, vec![]);

    let n = 30u32;
    for i in 0..n {
        auto.send(Flow::new(
            "flow-0",
            TimeValue::default(),
            &[ChannelValue::new("generator", i as f64)],
        ))
        .await
        .expect("send failed");
    }

    auto.finish().await.expect("finish failed");

    assert_eq!(counts.get("flow-0"), n);
    assert!(server.await.is_ok());
}

#[tokio::test]
async fn test_auto_register_into_inner_returns_stream_with_populated_cache() {
    let (service, _counts) = FlowCountingIngestService::new();
    let (client, server) = start_standard_server(service).await;

    let stream = SiftStreamBuilder::from_channel(client)
        .ingestion_config(IngestionConfigForm {
            asset_name: "test_asset".to_string(),
            client_key: "test_client_key".to_string(),
            flows: vec![],
        })
        .live_with_backups()
        .build()
        .await
        .expect("failed to build stream");

    let mut auto = SiftStreamAutoRegister::new(stream, vec![]);

    auto.send(Flow::new(
        "extracted-flow",
        TimeValue::default(),
        &[ChannelValue::new("reading", 42.0_f64)],
    ))
    .await
    .expect("send failed");

    // Consume the wrapper and verify the inner stream's cache is populated.
    let mut inner = auto.into_inner();

    assert!(
        inner.get_flow_descriptor("extracted-flow").is_ok(),
        "extracted-flow should be in the inner stream's cache"
    );

    // The inner stream should still be usable for direct sends.
    inner
        .send(Flow::new(
            "extracted-flow",
            TimeValue::default(),
            &[ChannelValue::new("reading", 99.0_f64)],
        ))
        .await
        .expect("direct send on inner stream failed");

    inner.finish().await.expect("finish failed");
    assert!(server.await.is_ok());
}

#[tokio::test]
async fn test_auto_register_finish_drains_all_queued_messages() {
    let (service, counts) = FlowCountingIngestService::new();
    let (client, server) = start_standard_server(service).await;

    let stream = SiftStreamBuilder::from_channel(client)
        .ingestion_config(IngestionConfigForm {
            asset_name: "test_asset".to_string(),
            client_key: "test_client_key".to_string(),
            flows: vec![],
        })
        .live_with_backups()
        .build()
        .await
        .expect("failed to build stream");

    let mut auto = SiftStreamAutoRegister::new(stream, vec![]);

    let n = 50u32;
    for i in 0..n {
        auto.send(Flow::new(
            "drain-test",
            TimeValue::default(),
            &[ChannelValue::new("v", i as f64)],
        ))
        .await
        .expect("send failed");
    }

    // finish() must drain all queued messages before returning.
    auto.finish().await.expect("finish failed");

    assert_eq!(
        counts.get("drain-test"),
        n,
        "all messages must be received after finish()"
    );
    assert!(server.await.is_ok());
}

#[tokio::test]
async fn test_auto_register_mixed_new_and_pre_registered_flows() {
    let (service, counts) = FlowCountingIngestService::new();
    let (client, server) = start_standard_server(service).await;

    let stream = SiftStreamBuilder::from_channel(client)
        .ingestion_config(IngestionConfigForm {
            asset_name: "test_asset".to_string(),
            client_key: "test_client_key".to_string(),
            flows: vec![FlowConfig {
                name: "flow-0".to_string(),
                channels: vec![ChannelConfig {
                    name: "generator".to_string(),
                    data_type: ChannelDataType::Double.into(),
                    ..Default::default()
                }],
            }],
        })
        .live_with_backups()
        .build()
        .await
        .expect("failed to build stream");

    let mut auto = SiftStreamAutoRegister::new(stream, vec![]);

    let n = 15u32;
    for i in 0..n {
        // Pre-registered flow — should never hit add_new_flows.
        auto.send(Flow::new(
            "flow-0",
            TimeValue::default(),
            &[ChannelValue::new("generator", i as f64)],
        ))
        .await
        .expect("send failed");

        // Unregistered flow — auto-registered on the first send.
        auto.send(Flow::new(
            "new-sensor",
            TimeValue::default(),
            &[ChannelValue::new("reading", i as f64 * 2.0)],
        ))
        .await
        .expect("send failed");
    }

    auto.finish().await.expect("finish failed");

    assert_eq!(counts.get("flow-0"), n);
    assert_eq!(counts.get("new-sensor"), n);
    assert!(server.await.is_ok());
}

// ============================================================
// Ingestion config service that captures registered FlowConfigs
// ============================================================

#[derive(Clone, Default)]
struct CapturedRegistrations(Arc<Mutex<Vec<FlowConfig>>>);

impl CapturedRegistrations {
    fn all(&self) -> Vec<FlowConfig> {
        self.0.lock().unwrap().clone()
    }
}

struct CapturingIngestionConfigService {
    captured: CapturedRegistrations,
    call_count: Arc<AtomicU32>,
    config: Arc<Mutex<Option<IngestionConfig>>>,
}

impl CapturingIngestionConfigService {
    fn new() -> (Self, CapturedRegistrations, Arc<AtomicU32>) {
        let captured = CapturedRegistrations::default();
        let call_count = Arc::new(AtomicU32::new(0));
        (
            Self {
                captured: captured.clone(),
                call_count: call_count.clone(),
                config: Arc::new(Mutex::new(None)),
            },
            captured,
            call_count,
        )
    }
}

#[tonic::async_trait]
impl IngestionConfigService for CapturingIngestionConfigService {
    async fn list_ingestion_configs(
        &self,
        request: Request<ListIngestionConfigsRequest>,
    ) -> Result<Response<ListIngestionConfigsResponse>, Status> {
        let filter = request.into_inner().filter;
        let guard = self.config.lock().unwrap();
        let configs = guard
            .iter()
            .filter(|c| filter.is_empty() || filter.contains(&c.client_key))
            .cloned()
            .collect();
        Ok(Response::new(ListIngestionConfigsResponse {
            ingestion_configs: configs,
            next_page_token: String::new(),
        }))
    }

    async fn create_ingestion_config(
        &self,
        request: Request<CreateIngestionConfigRequest>,
    ) -> Result<Response<CreateIngestionConfigResponse>, Status> {
        let r = request.into_inner();
        let new_config = IngestionConfig {
            ingestion_config_id: Uuid::new_v4().to_string(),
            asset_id: r.asset_name,
            client_key: r.client_key,
        };
        *self.config.lock().unwrap() = Some(new_config.clone());
        Ok(Response::new(CreateIngestionConfigResponse {
            ingestion_config: Some(new_config),
        }))
    }

    async fn get_ingestion_config(
        &self,
        request: Request<GetIngestionConfigRequest>,
    ) -> Result<Response<GetIngestionConfigResponse>, Status> {
        let id = request.into_inner().ingestion_config_id;
        let guard = self.config.lock().unwrap();
        match guard.as_ref().filter(|c| c.ingestion_config_id == id) {
            Some(c) => Ok(Response::new(GetIngestionConfigResponse {
                ingestion_config: Some(c.clone()),
            })),
            None => Err(Status::not_found("ingestion config not found")),
        }
    }

    async fn list_ingestion_config_flows(
        &self,
        _: Request<ListIngestionConfigFlowsRequest>,
    ) -> Result<Response<ListIngestionConfigFlowsResponse>, Status> {
        Ok(Response::new(ListIngestionConfigFlowsResponse {
            flows: vec![],
            next_page_token: String::new(),
        }))
    }

    async fn create_ingestion_config_flows(
        &self,
        request: Request<CreateIngestionConfigFlowsRequest>,
    ) -> Result<Response<CreateIngestionConfigFlowsResponse>, Status> {
        self.call_count.fetch_add(1, Ordering::SeqCst);
        self.captured
            .0
            .lock()
            .unwrap()
            .extend(request.into_inner().flows);
        Ok(Response::new(CreateIngestionConfigFlowsResponse {}))
    }
}

async fn start_server_with_capturing(
    ingest_service: impl IngestService,
    config_svc: CapturingIngestionConfigService,
) -> (SiftChannel, JoinHandle<()>) {
    let (client_io, server_io) = tokio::io::duplex(1024);

    let server = tokio::spawn(async move {
        Server::builder()
            .add_service(IngestServiceServer::new(ingest_service))
            .add_service(PingServiceServer::new(MinimalPingService))
            .add_service(IngestionConfigServiceServer::new(config_svc))
            .add_service(AssetServiceServer::new(MinimalAssetService))
            .add_service(RunServiceServer::new(MinimalRunService))
            .serve_with_incoming(tokio_stream::once(Ok::<_, IoError>(server_io)))
            .await
            .unwrap();
    });

    let mut client_io_opt = Some(client_io);
    let channel = Endpoint::try_from("http://[::]:50051")
        .unwrap()
        .connect_with_connector(service_fn(move |_: Uri| {
            let io = client_io_opt.take();
            async move {
                io.map(TokioIo::new)
                    .ok_or_else(|| std::io::Error::other("connector already used"))
            }
        }))
        .await
        .unwrap();

    let sift_channel: SiftChannel = ServiceBuilder::new()
        .layer(tonic::service::interceptor::InterceptorLayer::new(
            AuthInterceptor {
                apikey: "test".to_string(),
            },
        ))
        .service(channel);

    (sift_channel, server)
}

// ============================================================
// Staged config tests
// ============================================================

// Verifies that when a staged FlowConfig is provided, its full channel metadata
// (e.g. unit) is forwarded to the registration API rather than the minimal
// auto-derived config.
#[tokio::test]
async fn test_staged_config_channels_are_used_for_registration() {
    let (config_svc, registrations, call_count) = CapturingIngestionConfigService::new();
    let (ingest_svc, _counts) = FlowCountingIngestService::new();
    let (client, server) = start_server_with_capturing(ingest_svc, config_svc).await;

    let staged = FlowConfig {
        name: "sensor-flow".to_string(),
        channels: vec![ChannelConfig {
            name: "temperature".to_string(),
            data_type: ChannelDataType::Double.into(),
            unit: "celsius".to_string(),
            ..Default::default()
        }],
    };

    let stream = SiftStreamBuilder::from_channel(client)
        .ingestion_config(IngestionConfigForm {
            asset_name: "test_asset".to_string(),
            client_key: "staged-channels-test".to_string(),
            flows: vec![],
        })
        .live_with_backups()
        .build()
        .await
        .expect("failed to build stream");

    let mut auto = SiftStreamAutoRegister::new(stream, vec![staged]);

    auto.send(Flow::new(
        "sensor-flow",
        TimeValue::default(),
        &[ChannelValue::new("temperature", 22.5_f64)],
    ))
    .await
    .expect("send failed");

    auto.finish().await.expect("finish failed");

    assert_eq!(call_count.load(Ordering::SeqCst), 1);

    let registered = registrations.all();
    assert_eq!(registered.len(), 1);
    assert_eq!(registered[0].name, "sensor-flow");
    assert_eq!(registered[0].channels.len(), 1);
    assert_eq!(
        registered[0].channels[0].unit, "celsius",
        "staged config metadata should be forwarded, not overwritten by auto-derived config"
    );

    assert!(server.await.is_ok());
}

// Verifies that a channel name/type mismatch between the staged config and the
// flow being sent returns StagedConfigMismatch without attempting registration.
#[tokio::test]
async fn test_staged_config_mismatch_returns_error() {
    let (config_svc, _registrations, call_count) = CapturingIngestionConfigService::new();
    let (ingest_svc, _counts) = FlowCountingIngestService::new();
    let (client, server) = start_server_with_capturing(ingest_svc, config_svc).await;

    let staged = FlowConfig {
        name: "mismatch-flow".to_string(),
        channels: vec![ChannelConfig {
            name: "expected_channel".to_string(),
            data_type: ChannelDataType::Double.into(),
            ..Default::default()
        }],
    };

    let stream = SiftStreamBuilder::from_channel(client)
        .ingestion_config(IngestionConfigForm {
            asset_name: "test_asset".to_string(),
            client_key: "staged-mismatch-test".to_string(),
            flows: vec![],
        })
        .live_with_backups()
        .build()
        .await
        .expect("failed to build stream");

    let mut auto = SiftStreamAutoRegister::new(stream, vec![staged]);

    let err = auto
        .send(Flow::new(
            "mismatch-flow",
            TimeValue::default(),
            &[ChannelValue::new("wrong_channel", 1.0_f64)],
        ))
        .await
        .expect_err("expected StagedConfigMismatch error");

    assert!(
        matches!(err, AutoRegisterSendError::StagedConfigMismatch(_)),
        "expected StagedConfigMismatch, got: {err:?}"
    );
    assert_eq!(
        call_count.load(Ordering::SeqCst),
        0,
        "registration should not be attempted when staged config fails validation"
    );

    server.abort();
}

// Verifies that after a StagedConfigMismatch error the staged config is retained,
// so a corrected send can still use it for registration.
#[tokio::test]
async fn test_staged_config_retained_after_mismatch() {
    let registration_counter = Arc::new(AtomicU32::new(0));
    let (ingest_svc, _counts) = FlowCountingIngestService::new();
    let (client, server) =
        start_server_with_registration_counter(ingest_svc, registration_counter.clone()).await;

    let staged = FlowConfig {
        name: "retry-flow".to_string(),
        channels: vec![ChannelConfig {
            name: "correct_channel".to_string(),
            data_type: ChannelDataType::Double.into(),
            ..Default::default()
        }],
    };

    let stream = SiftStreamBuilder::from_channel(client)
        .ingestion_config(IngestionConfigForm {
            asset_name: "test_asset".to_string(),
            client_key: "staged-retry-test".to_string(),
            flows: vec![],
        })
        .live_with_backups()
        .build()
        .await
        .expect("failed to build stream");

    let mut auto = SiftStreamAutoRegister::new(stream, vec![staged]);

    // First attempt with the wrong channel — staged config is retained on error.
    let err = auto
        .send(Flow::new(
            "retry-flow",
            TimeValue::default(),
            &[ChannelValue::new("wrong_channel", 1.0_f64)],
        ))
        .await
        .expect_err("expected StagedConfigMismatch");
    assert!(matches!(
        err,
        AutoRegisterSendError::StagedConfigMismatch(_)
    ));

    // Second attempt with the correct channel — staged config validates and is used.
    auto.send(Flow::new(
        "retry-flow",
        TimeValue::default(),
        &[ChannelValue::new("correct_channel", 2.0_f64)],
    ))
    .await
    .expect("corrected send should succeed using retained staged config");

    auto.finish().await.expect("finish failed");

    assert_eq!(
        registration_counter.load(Ordering::SeqCst),
        1,
        "registration should happen exactly once on the successful retry"
    );
    assert!(server.await.is_ok());
}

// Verifies that the staged config is removed after successful registration so
// subsequent sends for the same flow hit the local cache and skip registration.
#[tokio::test]
async fn test_staged_config_consumed_after_successful_registration() {
    let registration_counter = Arc::new(AtomicU32::new(0));
    let (ingest_svc, counts) = FlowCountingIngestService::new();
    let (client, server) =
        start_server_with_registration_counter(ingest_svc, registration_counter.clone()).await;

    let staged = FlowConfig {
        name: "one-time-flow".to_string(),
        channels: vec![ChannelConfig {
            name: "reading".to_string(),
            data_type: ChannelDataType::Double.into(),
            ..Default::default()
        }],
    };

    let stream = SiftStreamBuilder::from_channel(client)
        .ingestion_config(IngestionConfigForm {
            asset_name: "test_asset".to_string(),
            client_key: "staged-consumed-test".to_string(),
            flows: vec![],
        })
        .live_with_backups()
        .build()
        .await
        .expect("failed to build stream");

    let mut auto = SiftStreamAutoRegister::new(stream, vec![staged]);

    let n = 10u32;
    for i in 0..n {
        auto.send(Flow::new(
            "one-time-flow",
            TimeValue::default(),
            &[ChannelValue::new("reading", i as f64)],
        ))
        .await
        .expect("send failed");
    }

    auto.finish().await.expect("finish failed");

    assert_eq!(
        registration_counter.load(Ordering::SeqCst),
        1,
        "staged config should trigger registration exactly once regardless of message count"
    );
    assert_eq!(counts.get("one-time-flow"), n);
    assert!(server.await.is_ok());
}
