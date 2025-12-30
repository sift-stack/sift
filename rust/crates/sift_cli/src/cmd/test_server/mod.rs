use super::Context;
use crate::cmd::test_server::metrics_streaming_client::MetricsStreamingClient;
use crate::{cli::TestServerArgs, util::tty::Output};
use anyhow::Result;
use server::TestServer;
use sift_rs::assets::v1::asset_service_server::AssetServiceServer;
use sift_rs::ingest::v1::ingest_service_server::IngestServiceServer;
use sift_rs::ingestion_configs::v2::ingestion_config_service_server::IngestionConfigServiceServer;
use sift_rs::ping::v1::ping_service_server::PingServiceServer;
use std::process::ExitCode;
use std::sync::Arc;
use tokio::sync::mpsc;
use tokio::sync::watch;
use tonic::transport::Server;
use tonic_reflection::server::Builder;

pub const FILE_DESCRIPTOR_SET: &[u8] = include_bytes!("../../../descriptor.bin");
pub mod metrics_streaming_client;
pub mod server;
use crate::cmd::test_server::metrics_streaming_client::Metrics;

pub async fn run(ctx: Context, args: TestServerArgs) -> Result<ExitCode> {
    let local_address = args.local_address.unwrap_or("127.0.0.1:50051".into());
    let addr = local_address.parse()?;

    // Initialize streaming client.
    let mut streaming_client =
        MetricsStreamingClient::build(ctx, &args.stream_metrics, &args.metrics_asset_name)?;
    if streaming_client.is_some() {
        streaming_client.as_mut().unwrap().initialize().await?;
    }

    // Channel to signal program exit.
    let (shutdown_tx, mut shutdown_rx) = watch::channel(false);
    let mut shutdown_rx2 = shutdown_rx.clone();

    // Channel to send metrics.
    let (metrics_tx, mut metrics_rx) = mpsc::channel::<Metrics>(1024);

    // Initialize gRPC server.
    let server = Arc::new(TestServer::default());

    // Start task to calculate ingestion metrics.
    let server_arc = Arc::clone(&server);
    let calc_stats_task = tokio::spawn(async move {
        server_arc
            .calculate_metrics(
                &mut shutdown_rx,
                metrics_tx,
                args.stream_metrics.unwrap_or(false),
            )
            .await;
    });

    // Start task to ingest metrics to Sift.
    let ingest_metrics_task = tokio::spawn(async move {
        if streaming_client.is_none() {
            return;
        }

        let mut client = streaming_client.unwrap();
        loop {
            tokio::select! {
                _ = shutdown_rx2.changed() => {
                    Output::new().line("Ingest task shutting down").print();
                    break;
                }
                Some(metrics) = metrics_rx.recv() => {
                    client.ingest(metrics).await;
                }
            };
        }
    });

    let reflection_service = Builder::configure()
        .register_encoded_file_descriptor_set(FILE_DESCRIPTOR_SET)
        .build_v1()?;

    Output::new()
        .line(format!("Server listening on {addr}"))
        .print();

    Server::builder()
        .add_service(reflection_service)
        .add_service(PingServiceServer::from_arc(server.clone()))
        .add_service(IngestServiceServer::from_arc(server.clone()))
        .add_service(IngestionConfigServiceServer::from_arc(server.clone()))
        .add_service(AssetServiceServer::from_arc(server.clone()))
        .serve_with_shutdown(addr, async move {
            tokio::signal::ctrl_c().await.unwrap();
            let _ = shutdown_tx.send(true);
        })
        .await?;

    calc_stats_task.await?;
    ingest_metrics_task.await?;

    Output::new().line("Exiting.").print();

    Ok(ExitCode::SUCCESS)
}
