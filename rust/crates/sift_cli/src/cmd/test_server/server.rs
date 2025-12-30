use crate::cmd::test_server::metrics_streaming_client::Metrics;
use crate::util::tty::Output;
use anyhow::Result;
use crossterm::{ExecutableCommand, cursor, terminal};
use prost::Message;
use sift_rs::assets::v1::{
    Asset, GetAssetRequest, GetAssetResponse, asset_service_server::AssetService,
};
use sift_rs::ingest::v1::{
    IngestArbitraryProtobufDataStreamRequest, IngestArbitraryProtobufDataStreamResponse,
    IngestWithConfigDataStreamRequest, IngestWithConfigDataStreamResponse,
    ingest_service_server::IngestService,
};
use sift_rs::ingestion_configs::v2::{ingestion_config_service_server::IngestionConfigService, *};
use sift_rs::ping::v1::{PingRequest, PingResponse, ping_service_server::PingService};
use std::io::stdout;
use std::time::Duration;
use std::{
    collections::HashMap,
    sync::{
        Mutex,
        atomic::{AtomicU32, AtomicU64, Ordering::Relaxed},
    },
};
use tokio::sync::mpsc::Sender;
use tokio::sync::watch;
use tokio_stream::StreamExt;
use tonic::{Request, Response, Status, Streaming};
use uuid::Uuid;

#[derive(Default)]
pub struct TestServer {
    /// Total number of streams created.
    total_num_streams: AtomicU32,

    /// Total number of messages received.
    total_num_messages: AtomicU64,

    /// Total number of bytes received.
    total_num_bytes_read: AtomicU64,

    // Total number of ingestion configs created.
    total_num_ingestion_configs: AtomicU32,

    /// Ingestion configs by Ingestion Config ID.
    ingestion_configs_by_id: Mutex<HashMap<String, IngestionConfig>>,

    /// Assets by Asset ID.
    asset_ids_by_name: Mutex<HashMap<String, String>>,
}

/// Ingested data and drops it. Calculates ingestion stats and optionally streams them to Sift.
impl TestServer {
    /// Calculate ingestion metrics and optionally stream them to Sift.
    pub async fn calculate_metrics(
        &self,
        shutdown: &mut watch::Receiver<bool>,
        metrics_tx: Sender<Metrics>,
        streaming_enabled: bool,
    ) {
        let mut last_total_num_bytes_read: u64 = 0;
        let mut last_total_num_messages: u64 = 0;

        loop {
            tokio::select! {
                _ = shutdown.changed() => {
                    Output::new().line("Metrics task shutting down").print();
                    break;
                }

                _ = tokio::time::sleep(Duration::from_millis(100)) => {
                    let current_total_num_bytes_read = self.total_num_bytes_read.load(Relaxed);
                    let current_total_num_messages = self.total_num_messages.load(Relaxed);
                    let current_total_num_streams = self.total_num_streams.load(Relaxed);
                    let bytes_per_s = current_total_num_bytes_read - last_total_num_bytes_read;
                    let messages_per_s = current_total_num_messages - last_total_num_messages;

                    last_total_num_bytes_read = current_total_num_bytes_read;
                    last_total_num_messages = current_total_num_messages;

                    // Clear terminal and print metrics.
                    stdout()
                        .execute(terminal::Clear(terminal::ClearType::All))
                        .expect("");
                    stdout().execute(cursor::MoveTo(0, 0)).expect("msg");
                    stdout().execute(cursor::MoveUp(5)).expect("terminal error");
                    stdout().execute(terminal::Clear(terminal::ClearType::FromCursorDown)).expect("msg");

                    Output::new().line(format!("Total num streams:  {current_total_num_streams}")).print();
                    Output::new().line(format!("Total num bytes:    {current_total_num_bytes_read}")).print();
                    Output::new().line(format!("Total num messages: {current_total_num_messages}")).print();
                    Output::new().line(format!("bytes/s:            {bytes_per_s}")).print();
                    Output::new().line(format!("messages/s:         {messages_per_s}")).print();

                    // Stream to Sift.
                    if streaming_enabled {
                        let e = metrics_tx.try_send(Metrics{
                            total_num_streams: current_total_num_streams,
                            total_num_bytes_read: current_total_num_bytes_read,
                            total_num_messages: current_total_num_messages,
                            bytes_per_s: (10 * bytes_per_s )as f64,
                            messages_per_s: (10 * messages_per_s) as f64,
                        });

                        if e.is_err() {
                            Output::new().line(format!("{e:?}"));
                        }
                    }
                }
            }
        }
    }
}

#[tonic::async_trait]
impl PingService for TestServer {
    async fn ping(&self, _request: Request<PingRequest>) -> Result<Response<PingResponse>, Status> {
        let resp = PingResponse {
            response: "".into(),
        };

        Ok(Response::new(resp))
    }
}

#[tonic::async_trait]
impl AssetService for TestServer {
    /// Returns an asset ID.
    async fn get_asset(
        &self,
        request: Request<GetAssetRequest>,
    ) -> Result<Response<GetAssetResponse>, Status> {
        let asset_ids_by_name = self.asset_ids_by_name.lock().unwrap();
        let inner = request.into_inner();

        for (asset_id, asset_name) in asset_ids_by_name.iter() {
            if inner.asset_id == *asset_id {
                return Ok(Response::new(GetAssetResponse {
                    asset: Some(Asset {
                        asset_id: asset_id.into(),
                        name: asset_name.into(),
                        ..Default::default()
                    }),
                }));
            }
        }

        Err(Status::not_found("asset not found"))
    }

    /// No-op.
    async fn delete_asset(
        &self,
        _request: Request<sift_rs::assets::v1::DeleteAssetRequest>,
    ) -> Result<Response<sift_rs::assets::v1::DeleteAssetResponse>, Status> {
        Ok(Response::new(
            sift_rs::assets::v1::DeleteAssetResponse::default(),
        ))
    }

    /// No-op.
    async fn list_assets(
        &self,
        _request: Request<sift_rs::assets::v1::ListAssetsRequest>,
    ) -> Result<Response<sift_rs::assets::v1::ListAssetsResponse>, Status> {
        Ok(Response::new(
            sift_rs::assets::v1::ListAssetsResponse::default(),
        ))
    }

    /// No-op.
    async fn update_asset(
        &self,
        _request: Request<sift_rs::assets::v1::UpdateAssetRequest>,
    ) -> Result<Response<sift_rs::assets::v1::UpdateAssetResponse>, Status> {
        Ok(Response::new(
            sift_rs::assets::v1::UpdateAssetResponse::default(),
        ))
    }

    /// No-op.
    async fn archive_asset(
        &self,
        _request: Request<sift_rs::assets::v1::ArchiveAssetRequest>,
    ) -> Result<Response<sift_rs::assets::v1::ArchiveAssetResponse>, Status> {
        Ok(Response::new(
            sift_rs::assets::v1::ArchiveAssetResponse::default(),
        ))
    }
}

#[tonic::async_trait]
impl IngestionConfigService for TestServer {
    /// Returns an arbitrary Ingestion Config with a new UUID.
    async fn get_ingestion_config(
        &self,
        request: Request<GetIngestionConfigRequest>,
    ) -> Result<Response<GetIngestionConfigResponse>, Status> {
        let inner = request.into_inner();
        let ingestion_configs = self.ingestion_configs_by_id.lock().unwrap();
        let ingestion_config = ingestion_configs
            .get(&inner.ingestion_config_id)
            .ok_or(Status::not_found("ingestion config not found"))?;

        Ok(Response::new(GetIngestionConfigResponse {
            ingestion_config: Some(IngestionConfig {
                ingestion_config_id: ingestion_config.ingestion_config_id.clone(),
                asset_id: ingestion_config.asset_id.clone(),
                client_key: ingestion_config.client_key.clone(),
            }),
        }))
    }

    /// Returns an empty list of ingestion configs.
    async fn list_ingestion_configs(
        &self,
        _request: Request<ListIngestionConfigsRequest>,
    ) -> Result<Response<ListIngestionConfigsResponse>, Status> {
        let ingestion_configs = self.ingestion_configs_by_id.lock().unwrap();

        let mut all_ingestion_configs: Vec<IngestionConfig> =
            Vec::with_capacity(ingestion_configs.len());

        for ingestion_config in ingestion_configs.values() {
            all_ingestion_configs.push(ingestion_config.clone());
        }

        Ok(Response::new(ListIngestionConfigsResponse {
            ingestion_configs: all_ingestion_configs,
            next_page_token: "".into(),
        }))
    }

    /// Returns an arbitrary Ingestion Config with a new UUID.
    async fn create_ingestion_config(
        &self,
        request: Request<CreateIngestionConfigRequest>,
    ) -> Result<Response<CreateIngestionConfigResponse>, Status> {
        self.total_num_ingestion_configs.fetch_add(1, Relaxed);
        let inner = request.into_inner();

        let mut assets = self.asset_ids_by_name.lock().unwrap();
        let default_asset_id = Uuid::new_v4().to_string();
        let asset_id = assets
            .get(&inner.asset_name)
            .unwrap_or(&default_asset_id)
            .to_string();

        let new_ingestion_config = CreateIngestionConfigResponse {
            ingestion_config: Some(IngestionConfig {
                ingestion_config_id: Uuid::new_v4().to_string(),
                asset_id: asset_id.clone(),
                client_key: inner.client_key,
            }),
        };

        assets.insert(asset_id.clone(), inner.asset_name);

        Ok(Response::new(new_ingestion_config))
    }

    /// No-op.
    async fn create_ingestion_config_flows(
        &self,
        _request: Request<CreateIngestionConfigFlowsRequest>,
    ) -> Result<Response<CreateIngestionConfigFlowsResponse>, Status> {
        Ok(Response::new(CreateIngestionConfigFlowsResponse::default()))
    }

    /// No-op.
    async fn list_ingestion_config_flows(
        &self,
        _request: Request<ListIngestionConfigFlowsRequest>,
    ) -> Result<Response<ListIngestionConfigFlowsResponse>, Status> {
        Ok(Response::new(ListIngestionConfigFlowsResponse::default()))
    }
}

#[tonic::async_trait]
impl IngestService for TestServer {
    /// Store ingestion stats.
    async fn ingest_with_config_data_stream(
        &self,
        request: Request<Streaming<IngestWithConfigDataStreamRequest>>,
    ) -> Result<Response<IngestWithConfigDataStreamResponse>, Status> {
        self.total_num_streams.fetch_add(1, Relaxed);

        let mut stream = request.into_inner();
        while let Some(msg) = stream.next().await {
            self.total_num_messages.fetch_add(1, Relaxed);
            let inner = msg?;
            self.total_num_bytes_read
                .fetch_add(inner.encoded_len() as u64, Relaxed);
        }

        Ok(Response::new(IngestWithConfigDataStreamResponse::default()))
    }

    /// No-op.
    async fn ingest_arbitrary_protobuf_data_stream(
        &self,
        _: Request<Streaming<IngestArbitraryProtobufDataStreamRequest>>,
    ) -> Result<Response<IngestArbitraryProtobufDataStreamResponse>, Status> {
        unimplemented!()
    }
}
