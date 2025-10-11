use anyhow::Result;
use crossterm::style::Stylize;
use std::{process::ExitCode, time::Duration};
use tokio::time::sleep;

use sift_rs::{
    SiftChannel,
    common::r#type::v1::ChannelConfig,
    jobs::v1::{JobStatus, JobType},
};

use crate::util::{job::JobServiceWrapper, progress::Spinner, tty::Output, user::get_user_id};

pub mod csv;
pub mod parquet;
mod utils;

const INDENT_1: &str = "  ";
const INDENT_2: &str = "    ";
const INDENT_3: &str = "      ";
const INDENT_4: &str = "        ";

pub async fn wait_for_job_completion(
    grpc_channel: SiftChannel,
    import_output_location: String,
) -> Result<ExitCode> {
    let spinner = Spinner::new();
    spinner.set_message(format!("{} file for processing", "Uploaded".green()));

    let user_id = get_user_id(grpc_channel.clone()).await?;
    let mut job_service = JobServiceWrapper::new(grpc_channel.clone());

    let Some(mut job) = job_service
        .get_latest_job_for_user(&user_id, JobType::DataImport)
        .await?
    else {
        spinner.finish_and_clear();

        Output::new()
            .line("The file was successfully uploaded but the job was unexpectedly not found")
            .tip("Please notify Sift about this bug")
            .eprint();
        return Ok(ExitCode::FAILURE);
    };

    loop {
        sleep(Duration::from_secs(3)).await;

        let Some(updated_job) = job_service.get_job(&job.job_id).await? else {
            spinner.finish_and_clear();
            Output::new()
                .line("The file was successfully uploaded but the job was unexpectedly not found")
                .tip("Please notify Sift about this bug")
                .eprint();
            return Ok(ExitCode::FAILURE);
        };
        job = updated_job;

        match job.job_status() {
            JobStatus::Created => (),
            JobStatus::Running => {
                spinner.set_message(format!("{} imported file", "Processing".green()));
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
                    .line(format!("{} data import job", "Cancelled".green()))
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
                    .line(format!("{} data import job", "Completed".green()))
                    .tip(format!(
                        "The data should be available on the {import_output_location}"
                    ))
                    .print();
                break;
            }
            _ => (),
        }
    }
    Ok(ExitCode::SUCCESS)
}

fn preview_import_config(asset: &str, run: &str, channel_configs: &[&ChannelConfig]) {
    let mut asset_run = Output::new();
    asset_run.line(format!("{}: {asset}", "Asset".green()));

    if !run.is_empty() {
        asset_run.line(format!("{}: {run}", "Run".green()));
    }
    asset_run.print();

    let mut configs = Output::new();

    if channel_configs.is_empty() {
        configs.line(format!("{}: {{}}", "Channels".green()));
        configs.print();
        return;
    }

    configs.line(format!("{}: {{", "Channels".green()));

    for conf in channel_configs {
        configs.line(format!(
            "{INDENT_1}{:<27} {}",
            conf.data_type().as_str_name(),
            conf.name.clone().cyan()
        ));

        if !conf.units.is_empty() {
            configs.line(format!("{INDENT_2}{} {}", "units".yellow(), conf.units));
        }

        if !conf.description.is_empty() {
            configs.line(format!(
                "{INDENT_2}{} {}",
                "description".yellow(),
                conf.description
            ));
        }

        if !conf.enum_types.is_empty() {
            configs.line(format!("{INDENT_2}{} {{", "enum-config".yellow()));

            let mut enum_confs = Vec::new();

            for enum_conf in &conf.enum_types {
                enum_confs.push((enum_conf.key, &enum_conf.name));
            }
            enum_confs.sort_by(|(k_a, _), (k_b, _)| k_a.cmp(k_b));

            for (k, v) in enum_confs {
                configs.line(format!("{INDENT_3}{k} => {v}"));
            }
            configs.line(format!("{INDENT_2}}}"));
        }

        if !conf.bit_field_elements.is_empty() {
            configs.line(format!("{INDENT_2}{} {{", "bit-field-elements".yellow()));

            let mut bit_field_elements = Vec::new();

            for el in &conf.bit_field_elements {
                bit_field_elements.push((el.index, el.bit_count, &el.name));
            }
            bit_field_elements.sort_by(|(k_a, _, _), (k_b, _, _)| k_a.cmp(k_b));

            for (idx, length, name) in bit_field_elements {
                configs
                    .line(format!("{INDENT_3}{{"))
                    .line(format!("{INDENT_4}element: {name}"))
                    .line(format!("{INDENT_4}index: {idx}"))
                    .line(format!("{INDENT_4}length: {length}"))
                    .line(format!("{INDENT_3}}}"));
            }
            configs.line(format!("{INDENT_2}}}"));
        }
    }
    configs.line("}");
    configs.print();
}
