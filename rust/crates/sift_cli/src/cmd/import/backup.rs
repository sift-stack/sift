use std::sync::atomic::{AtomicUsize, Ordering};
use std::{
    fs::File,
    io::BufReader,
    path::{Path, PathBuf},
    process::ExitCode,
    sync::Arc,
};

use anyhow::{Context as AnyhowContext, Result, anyhow};
use crossterm::style::Stylize;
use sift_pbfs::BackupsDecoder;
use sift_rs::{
    SiftChannel,
    ingest::v1::{IngestWithConfigDataStreamRequest, ingest_service_client::IngestServiceClient},
};
use tokio_stream::StreamExt;

use crate::{
    cli::ImportBackupArgs,
    cmd::Context,
    util::{api::create_grpc_channel, progress::Spinner, tty::Output},
};

pub async fn run(ctx: Context, args: ImportBackupArgs) -> Result<ExitCode> {
    let grpc_channel = create_grpc_channel(&ctx)?;

    // Collect all backup files recursively
    let backup_files = collect_backup_files(&args.path)?;

    if backup_files.is_empty() {
        return Err(anyhow!("no backup files found in {}", args.path.display()));
    }

    // Process each backup file
    let total_files = backup_files.len();
    let mut failed_files = Vec::new();

    // Spinner for the overall progress.
    let spinner = Arc::new(Spinner::new());

    for (index, backup_file) in backup_files.iter().enumerate() {
        let file_num = index + 1;
        let file_name = backup_file
            .file_name()
            .and_then(|n| n.to_str())
            .map(|s| s.to_string())
            .unwrap_or_else(|| backup_file.display().to_string());

        // Create a spinner for this file with standardized prefix
        let spinner_prefix = format!("File {} of {}: {}", file_num, total_files, file_name);

        match process_backup_file(
            backup_file,
            grpc_channel.clone(),
            spinner.clone(),
            spinner_prefix.clone(),
        )
        .await
        {
            Ok(_) => {
                // Show complete state
                spinner.set_message(format!("{} - {}", spinner_prefix, "Complete".green()));

                if args.delete_after_upload
                    && let Err(e) = std::fs::remove_file(backup_file)
                {
                    failed_files.push((
                        backup_file.clone(),
                        anyhow!("failed to delete file after upload: {}", e),
                    ));
                }
            }
            Err(e) => {
                // Show error state
                spinner.set_message(format!("{} - {}", spinner_prefix, "Error".red()));
                failed_files.push((backup_file.clone(), e));
            }
        }
        tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;
    }

    spinner.finish_and_clear();

    // Display any errors encountered
    if !failed_files.is_empty() {
        let mut output = Output::new();
        output.line(format!("Failed to process {} file(s):", failed_files.len()));
        for (file, error) in &failed_files {
            output.line(format!("  {}: {}", file.display(), error));
        }
        output.eprint();
        return Ok(ExitCode::FAILURE);
    }

    Ok(ExitCode::SUCCESS)
}

fn collect_backup_files(dir: &Path) -> Result<Vec<PathBuf>> {
    let mut files = Vec::new();

    if !dir.is_dir() {
        return Err(anyhow!("{} is not a directory", dir.display()));
    }

    fn collect_recursive(dir: &Path, files: &mut Vec<PathBuf>) -> Result<()> {
        for entry in std::fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                collect_recursive(&path, files)?;
            } else if path.is_file() {
                // Accept any file as a potential backup file
                // The decoder will handle invalid files
                files.push(path);
            }
        }
        Ok(())
    }

    collect_recursive(dir, &mut files)?;
    Ok(files)
}

async fn process_backup_file(
    backup_file: &Path,
    grpc_channel: SiftChannel,
    spinner: Arc<Spinner>,
    spinner_prefix: String,
) -> Result<()> {
    // Update spinner to show reading phase
    spinner.set_message(format!("{} - {}", spinner_prefix, "Reading".green()));

    // Read all messages into memory
    let file = File::open(backup_file).context("failed to open backup file")?;
    let decoder: BackupsDecoder<IngestWithConfigDataStreamRequest, BufReader<File>> =
        BackupsDecoder::new(BufReader::new(file));

    let mut messages = Vec::new();
    for result in decoder {
        match result {
            Ok(msg) => messages.push(msg),
            Err(e) => {
                Output::new()
                    .line(format!("Warning: failed to decode message: {}", e))
                    .eprint();
            }
        }
    }

    let total_messages = messages.len();
    if total_messages == 0 {
        return Err(anyhow!("no messages found in backup file"));
    }

    let spinner_clone = spinner.clone();
    let spinner_prefix_clone = spinner_prefix.clone();

    // Create a stream from the collected messages with progress tracking
    let streamed_count = Arc::new(AtomicUsize::new(0));
    let streamed_count_clone = streamed_count.clone();

    // Initialize streaming message
    spinner_clone.set_message(format!(
        "{} - {} (0% - 0/{} messages)",
        spinner_prefix_clone,
        "Streaming".green(),
        total_messages
    ));

    let message_stream = tokio_stream::iter(messages.into_iter()).map(move |msg| {
        let current = streamed_count_clone.fetch_add(1, Ordering::Relaxed) + 1;

        let percentage = if total_messages > 0 {
            (current * 100) / total_messages
        } else {
            0
        };
        spinner_clone.set_message(format!(
            "{} - {} ({}% - {}/{} messages)",
            spinner_prefix_clone,
            "Streaming".green(),
            percentage,
            current,
            total_messages
        ));

        msg
    });

    // Stream the messages directly to the gRPC endpoint
    let mut client = IngestServiceClient::new(grpc_channel.clone());
    let _response = client
        .ingest_with_config_data_stream(message_stream)
        .await
        .map(|res| res.into_inner())
        .map_err(|e| anyhow!("failed to stream backup file to Sift: {e}"))?;

    // Show 100% completion before returning (caller will show Complete state)
    spinner.set_message(format!(
        "{} - {} (100% - {}/{} messages)",
        spinner_prefix,
        "Streaming".green(),
        total_messages,
        total_messages
    ));

    Ok(())
}
