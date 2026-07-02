use crate::policy::{RetryPolicy, with_retry};
use crate::service::common;
use anyhow::{Context, Result, anyhow};
use pbjson_types::{FieldMask, Timestamp};
use sift_rs::{
    SiftChannel,
    metadata::v1::MetadataValue,
    runs::v2::{
        ListRunsRequest, ListRunsResponse, Run, UpdateRunRequest,
        run_service_client::RunServiceClient,
    },
};

/// Build a protobuf `Timestamp` from Unix nanoseconds via the shared helper.
fn timestamp_from_unix_nanos(nanos: i64) -> Timestamp {
    let (seconds, nanos) = common::unix_nanos_to_secs_and_subsec_nanos(nanos);
    Timestamp { seconds, nanos }
}

#[cfg(test)]
mod test;

#[derive(Clone)]
pub struct RunService {
    channel: SiftChannel,
    policy: RetryPolicy,
}

impl RunService {
    pub fn new(channel: SiftChannel, policy: RetryPolicy) -> Self {
        Self { channel, policy }
    }

    pub async fn list_runs(
        &self,
        filter: String,
        order_by: Option<String>,
        limit: Option<u32>,
    ) -> Result<Vec<Run>> {
        let (page_size, record_limit) = common::paging(limit);

        let mut page_token = String::new();
        let mut results = Vec::new();

        let order_by = order_by.unwrap_or_default();

        loop {
            let channel = self.channel.clone();
            let filter = filter.clone();
            let order_by = order_by.clone();
            let token = page_token.clone();

            let resp = with_retry(&self.policy, move || {
                let channel = channel.clone();
                let filter = filter.clone();
                let order_by = order_by.clone();
                let token = token.clone();
                async move {
                    let mut client = RunServiceClient::new(channel);
                    client
                        .list_runs(ListRunsRequest {
                            filter,
                            page_size,
                            page_token: token,
                            order_by,
                        })
                        .await
                        .map(|resp| resp.into_inner())
                }
            })
            .await
            .context("failed to query runs")?;

            let ListRunsResponse {
                runs,
                next_page_token,
            } = resp;
            if runs.is_empty() {
                break;
            }
            results.extend(runs);

            if results.len() >= record_limit || next_page_token.is_empty() {
                break;
            }
            page_token = next_page_token;
        }

        results.truncate(record_limit);

        Ok(results)
    }

    /// Update a subset of an existing run's fields. Per
    /// `protos/sift/runs/v2/runs.proto::UpdateRunRequest` the updatable fields are
    /// `name`, `description`, `start_time`, `stop_time`, `is_pinned`, `client_key`,
    /// `tags`, `is_archived`, and `metadata`. This service exposes all but
    /// `is_archived` (archive flow is out of scope).
    ///
    /// Caveats from the proto: `start_time` may be overwritten by a later
    /// ingestion, and `client_key` can be set only once — a second attempt errors.
    /// `tags` and `metadata` use REPLACE semantics.
    #[allow(clippy::too_many_arguments)]
    pub async fn update_run(
        &self,
        run_id: String,
        name: Option<String>,
        description: Option<String>,
        start_time_unix_nanos: Option<i64>,
        stop_time_unix_nanos: Option<i64>,
        is_pinned: Option<bool>,
        client_key: Option<String>,
        tags: Option<Vec<String>>,
        metadata: Option<Vec<MetadataValue>>,
    ) -> Result<Run> {
        let mut run = Run {
            run_id,
            ..Default::default()
        };
        let mut paths = Vec::new();

        if let Some(v) = name {
            run.name = v;
            paths.push("name".to_string());
        }
        if let Some(v) = description {
            run.description = v;
            paths.push("description".to_string());
        }
        if let Some(v) = start_time_unix_nanos {
            run.start_time = Some(timestamp_from_unix_nanos(v));
            paths.push("start_time".to_string());
        }
        if let Some(v) = stop_time_unix_nanos {
            run.stop_time = Some(timestamp_from_unix_nanos(v));
            paths.push("stop_time".to_string());
        }
        if let Some(v) = is_pinned {
            run.is_pinned = v;
            paths.push("is_pinned".to_string());
        }
        if let Some(v) = client_key {
            run.client_key = Some(v);
            paths.push("client_key".to_string());
        }
        if let Some(v) = tags {
            run.tags = v;
            paths.push("tags".to_string());
        }
        if let Some(v) = metadata {
            run.metadata = v;
            paths.push("metadata".to_string());
        }

        let channel = self.channel.clone();
        let resp = with_retry(&self.policy, move || {
            let channel = channel.clone();
            let run = run.clone();
            let paths = paths.clone();
            async move {
                let mut client = RunServiceClient::new(channel);
                client
                    .update_run(UpdateRunRequest {
                        run: Some(run),
                        update_mask: Some(FieldMask { paths }),
                    })
                    .await
                    .map(|resp| resp.into_inner())
            }
        })
        .await
        .context("failed to update run")?;

        resp.run
            .ok_or_else(|| anyhow!("update_run response missing run"))
    }
}
