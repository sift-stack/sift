use std::process::ExitCode;

use anyhow::{Context as AnyhowContext, Result};
use sift_rs::ping::v1::{PingRequest, ping_service_client::PingServiceClient};

use crate::util::{api::create_grpc_channel, tty::Output};

use super::Context;

pub async fn run(ctx: Context) -> Result<ExitCode> {
    let grpc_channel = create_grpc_channel(&ctx)?;
    let mut ping_service = PingServiceClient::new(grpc_channel);

    let response = ping_service
        .ping(PingRequest::default())
        .await
        .context("failed to ping Sift")?
        .into_inner()
        .response;

    Output::new().line(response).print();

    Ok(ExitCode::SUCCESS)
}
