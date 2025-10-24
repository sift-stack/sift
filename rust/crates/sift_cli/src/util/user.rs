use anyhow::{Context, Result};
use sift_rs::{
    SiftChannel,
    me::v2::{GetMeRequest, me_service_client::MeServiceClient},
};

pub async fn get_user_id(grpc_channel: SiftChannel) -> Result<String> {
    let mut me_service = MeServiceClient::new(grpc_channel);
    let res = me_service
        .get_me(GetMeRequest::default())
        .await
        .context("failed to retrieve user info from provided --profile")?
        .into_inner();
    Ok(res.user_id)
}
