use std::{collections::HashSet, fs::File, io::Seek, process::ExitCode};

use anyhow::{Context as AnyhowContext, Result, anyhow};
use crossterm::style::Stylize;
use parquet::file::reader::{ChunkReader, FileReader, SerializedFileReader};
use parquet::record::Field;
use parquet::schema::types::Type as ParquetSchemaType;
use sift_rs::{
    common::r#type::v1::ChannelConfig,
    data_imports::v2::{
        CreateDataImportFromUploadRequest, CreateDataImportFromUploadResponse,
        ParquetComplexTypesImportMode, ParquetConfig,
        data_import_service_client::DataImportServiceClient, parquet_config::Config,
        parquet_single_channel_per_row_config::Config as ScprInnerConfig,
    },
};

use crate::cli::ScprArgs;
use crate::cmd::import::parquet::detect_parquet_schema::detect_scpr_config;
use crate::cmd::{
    Context,
    import::{
        parquet::FooterMetadata, preview_import_config, utils::upload_gzipped_file,
        wait_for_job_completion,
    },
};
use crate::util::{api::create_grpc_channel, tty::Output};

pub async fn run(ctx: Context, args: ScprArgs) -> Result<ExitCode> {
    let grpc_channel = create_grpc_channel(&ctx)?;
    let mut data_imports_client = DataImportServiceClient::new(grpc_channel.clone());
    let mut file = File::open(&args.common.path).context("failed to open parquet file")?;
    let footer_md = FooterMetadata::try_from(&mut file)?;

    let scpr_config =
        detect_scpr_config(&file, &args).context("failed to detect parquet schema")?;

    if args.common.preview {
        let run_label = args
            .common
            .run_id
            .as_deref()
            .filter(|s| !s.is_empty())
            .or(args.common.run.as_deref())
            .unwrap_or("");

        let multi_channels: Vec<ChannelConfig> = match scpr_config.config.as_ref() {
            Some(ScprInnerConfig::MultiChannel(multi)) => {
                let data_type = scpr_config
                    .columns
                    .iter()
                    .find(|c| c.path == multi.data_path)
                    .and_then(|c| c.column_config.as_ref())
                    .map(|c| c.data_type)
                    .unwrap_or_default();

                let discovery_file = File::open(&args.common.path)
                    .context("failed to open parquet file for channel discovery")?;
                discover_multi_channel_names_for_preview(discovery_file, &multi.name_path)?
                    .into_iter()
                    .map(|name| ChannelConfig {
                        name,
                        data_type,
                        ..Default::default()
                    })
                    .collect()
            }
            _ => Vec::new(),
        };

        let preview_channels: Vec<&ChannelConfig> = match scpr_config.config.as_ref() {
            Some(ScprInnerConfig::SingleChannel(single)) => single.channel.iter().collect(),
            Some(ScprInnerConfig::MultiChannel(_)) => multi_channels.iter().collect(),
            None => Vec::new(),
        };

        preview_import_config(&args.common.asset, run_label, &preview_channels);
        return Ok(ExitCode::SUCCESS);
    }

    let parquet_config = ParquetConfig {
        config: Some(Config::SingleChannelPerRow(scpr_config)),
        ..Default::default()
    };
    let create_data_import_req = create_data_import_request(&args, parquet_config, footer_md)?;

    let CreateDataImportFromUploadResponse { upload_url, .. } = data_imports_client
        .create_data_import_from_upload(create_data_import_req)
        .await
        .context("error creating data import")?
        .into_inner();

    file.rewind()?;
    let job_id = upload_gzipped_file(&ctx, &upload_url, file, "application/vnd.apache.parquet")
        .await
        .context("failed to upload Parquet file")?;

    let location = args.common.run.as_ref().map_or_else(
        || format!("asset '{}'", args.common.asset.cyan()),
        |r| format!("run '{}'", r.clone().cyan()),
    );

    if !args.common.wait {
        Output::new()
            .line(format!("{} file for processing", "Uploaded".green()))
            .tip(format!(
                "Once processing is complete the data will be available on the {location}."
            ))
            .print();
        return Ok(ExitCode::SUCCESS);
    }

    wait_for_job_completion(grpc_channel, job_id, location).await
}

fn create_data_import_request(
    args: &ScprArgs,
    config: ParquetConfig,
    footer_md: FooterMetadata,
) -> Result<CreateDataImportFromUploadRequest> {
    Ok(CreateDataImportFromUploadRequest {
        parquet_config: Some(ParquetConfig {
            asset_name: args.common.asset.clone(),
            run_name: args.common.run.clone().unwrap_or_default(),
            run_id: args.common.run_id.clone().unwrap_or_default(),
            footer_offset: footer_md.offset,
            footer_length: u32::try_from(footer_md.length)
                .context("parquet footer length too large")?,
            complex_types_import_mode: ParquetComplexTypesImportMode::from(
                args.complex_types_mode.clone(),
            )
            .into(),
            config: config.config,
        }),
        ..Default::default()
    })
}

/// Scan the parquet file's name column and return the distinct channel names
/// it contains (sorted, deduped). Used by multi-mode preview so the user can
/// see what channels the server will create at ingest.
pub(super) fn discover_multi_channel_names_for_preview<R: ChunkReader + 'static>(
    file: R,
    name_path: &str,
) -> Result<Vec<String>> {
    let reader =
        SerializedFileReader::new(file).context("failed to build parquet file reader")?;

    let file_schema = reader.metadata().file_metadata().schema();
    let root_name = file_schema.name().to_string();
    let name_field = file_schema
        .get_fields()
        .iter()
        .find(|t| t.name() == name_path)
        .with_context(|| format!("name column '{name_path}' not found in parquet schema"))?
        .clone();

    let projection = ParquetSchemaType::group_type_builder(&root_name)
        .with_fields(vec![name_field])
        .build()
        .context("failed to build parquet projection schema")?;

    let row_iter = reader
        .get_row_iter(Some(projection))
        .context("failed to build parquet row iterator")?;

    let mut seen: HashSet<String> = HashSet::new();
    for row_result in row_iter {
        let row = row_result.context("failed to read parquet row")?;
        let (_, field) = row
            .get_column_iter()
            .next()
            .ok_or_else(|| anyhow!("internal: projected row missing column"))?;
        match field {
            Field::Str(s) => {
                seen.insert(s.clone());
            }
            Field::Null => {}
            other => {
                return Err(anyhow!(
                    "name column '{name_path}' must be a string type; got {other:?}"
                ));
            }
        }
    }

    let mut names: Vec<String> = seen.into_iter().collect();
    names.sort();
    Ok(names)
}
