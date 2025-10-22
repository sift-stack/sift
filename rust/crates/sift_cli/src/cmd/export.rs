use std::{
    env::temp_dir,
    fs::{File, remove_file},
    io::{self, Write},
    path::PathBuf,
    process::ExitCode,
    time::Duration,
};

use anyhow::{Context as AnyhowContext, Result, anyhow};
use chrono::DateTime;
use crossterm::style::Stylize;
use parquet::data_type::AsBytes;
use pbjson_types::Timestamp;
use reqwest::Client;
use sift_rs::{
    SiftChannel,
    assets::v1::{ListAssetsRequest, ListAssetsResponse, asset_service_client::AssetServiceClient},
    exports::v1::{
        AssetsAndTimeRange, ExportDataRequest, ExportDataResponse, ExportOutputFormat,
        GetDownloadUrlRequest, GetDownloadUrlResponse, RunsAndTimeRange,
        export_data_request::TimeSelection, export_service_client::ExportServiceClient,
    },
    jobs::v1::JobStatus,
    runs::v2::{ListRunsRequest, ListRunsResponse, run_service_client::RunServiceClient},
};
use tokio::time::sleep;
use zip::ZipArchive;

use crate::{
    cli::{ExportAssetArgs, ExportRunArgs},
    util::{
        api::create_grpc_channel, channel::filter_channels, job::JobServiceWrapper,
        progress::Spinner, tty::Output,
    },
};

use super::Context;

pub async fn run(ctx: Context, args: ExportRunArgs) -> Result<ExitCode> {
    let grpc_channel = create_grpc_channel(&ctx)?;
    let mut run_service = RunServiceClient::new(grpc_channel.clone());

    let filter = {
        if let Some(run_name) = args.name.as_ref() {
            format!("name == \"{run_name}\"")
        } else if let Some(run_id) = args.run_id.as_ref() {
            format!("run_id == \"{run_id}\"")
        } else if let Some(client_key) = args.client_key.as_ref() {
            format!("client_key == \"{client_key}\"")
        } else {
            return Err(anyhow!(
                "at least one of the following arguments is required: `{}`, `{}`, `{}`",
                "--run".cyan(),
                "--id".cyan(),
                "--client-key".cyan(),
            ));
        }
    };

    let ListRunsResponse { runs, .. } = run_service
        .list_runs(ListRunsRequest {
            filter,
            ..Default::default()
        })
        .await
        .context("failed to query run")?
        .into_inner();

    if runs.is_empty() {
        return Err(anyhow!("no run found"));
    } else if runs.len() > 1 {
        return Err(anyhow!(
            "multiple runs found. Try providing a unique identifier with `{}` or `{}`",
            "--id".cyan(),
            "--client-key".cyan(),
        ));
    }

    let run = runs.first().unwrap();

    if run.asset_ids.is_empty() {
        return Err(anyhow!(
            "run '{}' isn't associated with any assets",
            run.name.clone().yellow()
        ));
    }
    let asset_ids_cel = run
        .asset_ids
        .iter()
        .map(|a| format!("'{a}'"))
        .collect::<Vec<String>>()
        .join(",");

    let mut channel_ids = args.common.channel_id;

    if !args.common.channel.is_empty() {
        let channel_names_cel = args
            .common
            .channel
            .iter()
            .map(|c| format!("'{c}'"))
            .collect::<Vec<String>>()
            .join(",");

        let filter = format!("asset_id in [{asset_ids_cel}] && name in [{channel_names_cel}]");
        let query_res = filter_channels(grpc_channel.clone(), &filter).await?;

        for channel in query_res {
            channel_ids.push(channel.channel_id);
        }
    }

    if let Some(re) = args.common.channel_regex {
        let filter = format!("asset_id in [{asset_ids_cel}] && name.matches(\"{re}\")");
        let query_res = filter_channels(grpc_channel.clone(), &filter).await?;

        for channel in query_res {
            channel_ids.push(channel.channel_id);
        }
    }

    let start_time = args
        .common
        .start
        .as_deref()
        .and_then(|t| {
            DateTime::parse_from_rfc3339(t)
                .map(|d| Timestamp::from(d.to_utc()))
                .ok()
        })
        .or(run.start_time);

    let stop_time = args
        .common
        .stop
        .as_deref()
        .and_then(|t| {
            DateTime::parse_from_rfc3339(t)
                .map(|d| Timestamp::from(d.to_utc()))
                .ok()
        })
        .or(run.stop_time);

    let export_req = ExportDataRequest {
        channel_ids,
        output_format: ExportOutputFormat::from(args.common.format).into(),
        time_selection: Some(TimeSelection::RunsAndTimeRange(RunsAndTimeRange {
            start_time,
            stop_time,
            run_ids: vec![run.run_id.clone()],
        })),
        ..Default::default()
    };

    export(grpc_channel, export_req, args.common.output).await
}

pub async fn asset(ctx: Context, args: ExportAssetArgs) -> Result<ExitCode> {
    let start_time = args
        .common
        .start
        .as_deref()
        .and_then(|t| {
            DateTime::parse_from_rfc3339(t)
                .map(|d| Timestamp::from(d.to_utc()))
                .ok()
        })
        .ok_or_else(|| anyhow!("missing required argument `{}`", "--start".yellow()))?;

    let stop_time = args
        .common
        .stop
        .as_deref()
        .and_then(|t| {
            DateTime::parse_from_rfc3339(t)
                .map(|d| Timestamp::from(d.to_utc()))
                .ok()
        })
        .ok_or_else(|| anyhow!("missing required argument `{}`", "--stop".yellow()))?;

    let grpc_channel = create_grpc_channel(&ctx)?;
    let mut asset_service = AssetServiceClient::new(grpc_channel.clone());

    let filter = format!("name == '{}'", args.asset);

    let ListAssetsResponse { assets, .. } = asset_service
        .list_assets(ListAssetsRequest {
            filter,
            ..Default::default()
        })
        .await
        .context("failed to query asset")?
        .into_inner();

    if assets.is_empty() {
        return Err(anyhow!("no run found"));
    }
    let asset = assets.first().unwrap();
    let asset_id = &asset.asset_id;

    let mut channel_ids = args.common.channel_id;

    if !args.common.channel.is_empty() {
        let channel_names_cel = args
            .common
            .channel
            .iter()
            .map(|c| format!("'{c}'"))
            .collect::<Vec<String>>()
            .join(",");

        let filter = format!("asset_id == '{asset_id}' && name in [{channel_names_cel}]");
        let query_res = filter_channels(grpc_channel.clone(), &filter).await?;

        for channel in query_res {
            channel_ids.push(channel.channel_id);
        }
    }

    if let Some(re) = args.common.channel_regex {
        let filter = format!("asset_id == '{asset_id}' && name.matches(\"{re}\")");
        let query_res = filter_channels(grpc_channel.clone(), &filter).await?;

        for channel in query_res {
            channel_ids.push(channel.channel_id);
        }
    }

    let export_req = ExportDataRequest {
        channel_ids,
        output_format: ExportOutputFormat::from(args.common.format).into(),
        time_selection: Some(TimeSelection::AssetsAndTimeRange(AssetsAndTimeRange {
            asset_ids: vec![asset_id.to_string()],
            start_time: Some(start_time),
            stop_time: Some(stop_time),
        })),
        ..Default::default()
    };

    export(grpc_channel, export_req, args.common.output).await
}

async fn export(
    grpc_channel: SiftChannel,
    req: ExportDataRequest,
    output: PathBuf,
) -> Result<ExitCode> {
    let mut export_service = ExportServiceClient::new(grpc_channel.clone());
    let ExportDataResponse { job_id, .. } = export_service
        .export_data(req)
        .await
        .context("failed to initiate export")?
        .into_inner();

    let mut job_service = JobServiceWrapper::new(grpc_channel);
    let mut job = job_service
        .get_job(&job_id)
        .await?
        .ok_or_else(|| anyhow!("failed to find job {job_id}"))?;

    let spinner = Spinner::new();
    spinner.set_message(format!("{} export", "Processing".green()));

    loop {
        sleep(Duration::from_secs(3)).await;

        match job.job_status() {
            JobStatus::Created => (),
            JobStatus::Running => {
                spinner.set_message(format!("{} export", "Processing".green()));
            }
            JobStatus::CancelRequested => {
                spinner.set_message(format!(
                    "{} was requested but the job may still finish",
                    "Cancellation".green()
                ));
            }
            JobStatus::Cancelled => {
                spinner.finish_and_clear();
                Output::new()
                    .line(format!("{} data export job", "Cancelled".green()))
                    .print();
                break;
            }
            JobStatus::Failed => {
                spinner.finish_and_clear();
                Output::new()
                    .line("Processing failed")
                    .tip("Please check the Sift jobs manage page for further details")
                    .eprint();
                return Ok(ExitCode::FAILURE);
            }
            JobStatus::Finished => {
                spinner.finish_and_clear();
                Output::new()
                    .line(format!("{} export", "Downloading".green()))
                    .print();
                break;
            }
            _ => (),
        }

        job = job_service
            .get_job(&job_id)
            .await?
            .ok_or_else(|| anyhow!("failed to find job {job_id}"))?;
    }

    let GetDownloadUrlResponse { presigned_url } = export_service
        .get_download_url(GetDownloadUrlRequest { job_id })
        .await
        .context("failed to get download URL for export")?
        .into_inner();

    let http_client = Client::new();

    let mut output_file = match File::create_new(&output) {
        Ok(fd) => fd,
        Err(err) => {
            spinner.finish_and_clear();
            Output::new()
                .line(format!("Failed to create output file: {err}"))
                .tip("the export is available to download in the jobs manage page")
                .eprint();
            return Ok(ExitCode::FAILURE);
        }
    };

    let zip_file_path = temp_dir().join(format!("{}.zip", output.display()));
    let mut zip_file = match File::create_new(&zip_file_path) {
        Ok(fd) => fd,
        Err(err) => {
            spinner.finish_and_clear();
            Output::new()
                .line(format!("Failed to create output zip file: {err}"))
                .tip("the export is available to download in the jobs manage page")
                .eprint();
            return Ok(ExitCode::FAILURE);
        }
    };

    let mut resp = match http_client.get(presigned_url).send().await {
        Ok(res) => res,
        Err(err) => {
            spinner.finish_and_clear();
            Output::new()
                .line(format!(
                    "Something went wrong while downloading export: {err}"
                ))
                .tip("the export is available to download in the jobs manage page")
                .eprint();
            return Ok(ExitCode::FAILURE);
        }
    };

    while let Some(chunk) = resp
        .chunk()
        .await
        .context("error while streaming response body")?
    {
        zip_file
            .write_all(chunk.as_bytes())
            .context("failed to write to output file")?;
    }
    zip_file.sync_all()?;

    let mut zip_reader = ZipArchive::new(zip_file).context("failed to read zip")?;
    let mut zip_item = zip_reader
        .by_index(0)
        .context("unexpected empty zip file")?;
    io::copy(&mut zip_item, &mut output_file)?;
    output_file.sync_all()?;

    let _ = remove_file(zip_file_path);

    spinner.finish_and_clear();

    Output::new()
        .line(format!("{} download", "Completed".green()))
        .tip(format!(
            "download can be located at '{}'",
            output.display().to_string().cyan()
        ))
        .print();

    Ok(ExitCode::SUCCESS)
}
