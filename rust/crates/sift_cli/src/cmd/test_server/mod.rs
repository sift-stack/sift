use super::Context;
use crate::cmd::test_server::metrics_streaming_client::MetricsStreamingClient;
use crate::{cli::TestServerArgs, util::tty::Output};
use anyhow::{Context as AnyhowContext, Result};
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

pub mod metrics_streaming_client;
pub mod server;
use crate::cmd::test_server::metrics_streaming_client::Metrics;

pub async fn run(ctx: Context, args: TestServerArgs) -> Result<ExitCode> {
    let local_address = args
        .local_address
        .unwrap_or_else(|| "0.0.0.0:50051".to_string());
    let addr = local_address
        .parse()
        .context(format!("failed to parse local_address: {}", local_address))?;

    // Initialize streaming client.
    let mut streaming_client =
        MetricsStreamingClient::build(ctx, &args.stream_metrics, &args.metrics_asset_name)
            .context("failed to create metrics streaming client")?;

    if let Some(client) = streaming_client.as_mut() {
        client
            .initialize()
            .await
            .context("failed to initialize streaming client")?;
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
                args.stream_metrics,
                args.plain_output,
            )
            .await
            .context("calculate metrics task failed")
            .unwrap();
    });

    // Start task to ingest metrics to Sift.
    let ingest_metrics_task = tokio::spawn(async move {
        if let Some(client) = streaming_client.as_mut() {
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
        }
    });

    let reflection_service = Builder::configure()
        .register_encoded_file_descriptor_set(sift_rs::assets::v1::FILE_DESCRIPTOR_SET)
        .register_encoded_file_descriptor_set(sift_rs::ingest::v1::FILE_DESCRIPTOR_SET)
        .register_encoded_file_descriptor_set(sift_rs::ingestion_configs::v2::FILE_DESCRIPTOR_SET)
        .register_encoded_file_descriptor_set(sift_rs::ping::v1::FILE_DESCRIPTOR_SET)
        .build_v1()
        .context("failed to create gRPC reflection service")?;

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

    calc_stats_task
        .await
        .context("failed to await calculation task")?;
    ingest_metrics_task
        .await
        .context("failed to await ingestion task")?;

    Output::new().line("Exiting.").print();

    Ok(ExitCode::SUCCESS)
}
