use std::process::ExitCode;

use anyhow::Result;
use sift_rs::data_imports::v2::{
    CreateDataImportFromUploadRequest, ParquetConfig, ParquetSingleChannelPerRowConfig,
};

use crate::{
    cli::ScprArgs,
    cmd::{Context, import::parquet::FooterMetadata},
};

pub async fn run(_ctx: Context, _args: ScprArgs) -> Result<ExitCode> {
    todo!("SCPR run flow — see flat_dataset::run for pattern")
}

pub fn build_scpr_config(_args: &ScprArgs) -> Result<ParquetSingleChannelPerRowConfig> {
    todo!("validate per-mode args, then call detect_scpr_config")
}

#[allow(dead_code)]
fn create_data_import_request(
    _args: &ScprArgs,
    _config: ParquetConfig,
    _footer_md: FooterMetadata,
) -> Result<CreateDataImportFromUploadRequest> {
    todo!(
        "mirror flat_dataset::create_data_import_request, set config: Some(Config::SingleChannelPerRow(...))"
    )
}
