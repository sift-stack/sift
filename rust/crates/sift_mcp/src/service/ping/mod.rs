use crate::policy::{RetryPolicy, with_retry};
use anyhow::{Context, Result};
use sift_rs::{
    SiftChannel,
    ping::v1::{PingRequest, ping_service_client::PingServiceClient},
};

#[derive(Clone)]
pub struct PingService {
    channel: SiftChannel,
    policy: RetryPolicy,
}

impl PingService {
    pub fn new(channel: SiftChannel, policy: RetryPolicy) -> Self {
        Self { channel, policy }
    }

    pub async fn ping(&self) -> Result<String> {
        let channel = self.channel.clone();

        with_retry(&self.policy, move || {
            let channel = channel.clone();
            async move {
                let mut client = PingServiceClient::new(channel);
                client
                    .ping(PingRequest::default())
                    .await
                    .map(|resp| resp.into_inner().response)
            }
        })
        .await
        .context("failed to ping Sift")
    }
}
