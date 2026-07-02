use crate::policy::{RetryPolicy, with_retry};
use crate::service::common;
use anyhow::{Context, Result, anyhow};
use pbjson_types::{FieldMask, Timestamp};
use sift_rs::{
    SiftChannel,
    annotations::v1::{
        Annotation, AnnotationLinkedChannel, AnnotationState, AnnotationType,
        CreateAnnotationRequest, ListAnnotationsRequest, ListAnnotationsResponse,
        UpdateAnnotationRequest, annotation_linked_channel,
        annotation_service_client::AnnotationServiceClient,
    },
    metadata::v1::MetadataValue,
};

#[cfg(test)]
mod test;

/// Build a protobuf `Timestamp` from Unix nanoseconds via the shared helper.
fn timestamp_from_unix_nanos(nanos: i64) -> Timestamp {
    let (seconds, nanos) = common::unix_nanos_to_secs_and_subsec_nanos(nanos);
    Timestamp { seconds, nanos }
}

/// Map a list of channel ids to plain `AnnotationLinkedChannel` entries. Only the
/// raw-channel variant is supported here; bit-field and calculated-channel links
/// are out of scope for the MCP surface.
fn linked_channels(ids: Vec<String>) -> Vec<AnnotationLinkedChannel> {
    ids.into_iter()
        .map(|channel_id| AnnotationLinkedChannel {
            r#type: Some(annotation_linked_channel::Type::Channel(
                sift_rs::annotations::v1::AnnotationLinkedChannelsChannel { channel_id },
            )),
        })
        .collect()
}

#[derive(Clone)]
pub struct AnnotationService {
    channel: SiftChannel,
    policy: RetryPolicy,
}

impl AnnotationService {
    pub fn new(channel: SiftChannel, policy: RetryPolicy) -> Self {
        Self { channel, policy }
    }

    pub async fn list_annotations(
        &self,
        filter: String,
        order_by: Option<String>,
        limit: Option<u32>,
        organization_id: Option<String>,
    ) -> Result<Vec<Annotation>> {
        let (page_size, record_limit) = common::paging(limit);

        let mut page_token = String::new();
        let mut results = Vec::new();

        let order_by = order_by.unwrap_or_default();
        let organization_id = organization_id.unwrap_or_default();

        loop {
            let channel = self.channel.clone();
            let filter = filter.clone();
            let order_by = order_by.clone();
            let organization_id = organization_id.clone();
            let token = page_token.clone();

            let resp = with_retry(&self.policy, move || {
                let channel = channel.clone();
                let filter = filter.clone();
                let order_by = order_by.clone();
                let organization_id = organization_id.clone();
                let token = token.clone();
                async move {
                    let mut client = AnnotationServiceClient::new(channel);
                    client
                        .list_annotations(ListAnnotationsRequest {
                            page_size,
                            page_token: token,
                            filter,
                            organization_id,
                            order_by,
                        })
                        .await
                        .map(|resp| resp.into_inner())
                }
            })
            .await
            .context("failed to query annotations")?;

            let ListAnnotationsResponse {
                annotations,
                next_page_token,
            } = resp;
            if annotations.is_empty() {
                break;
            }
            results.extend(annotations);

            if results.len() >= record_limit || next_page_token.is_empty() {
                break;
            }
            page_token = next_page_token;
        }

        results.truncate(record_limit);

        Ok(results)
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn create_annotation(
        &self,
        name: String,
        description: Option<String>,
        start_time_unix_nanos: i64,
        end_time_unix_nanos: i64,
        annotation_type: AnnotationType,
        state: Option<AnnotationState>,
        assets: Option<Vec<String>>,
        tags: Option<Vec<String>>,
        linked_channel_ids: Option<Vec<String>>,
        run_id: Option<String>,
        assign_to_user_id: Option<String>,
        metadata: Option<Vec<MetadataValue>>,
        organization_id: Option<String>,
    ) -> Result<Annotation> {
        let request = CreateAnnotationRequest {
            name,
            description: description.unwrap_or_default(),
            start_time: Some(timestamp_from_unix_nanos(start_time_unix_nanos)),
            end_time: Some(timestamp_from_unix_nanos(end_time_unix_nanos)),
            assets: assets.unwrap_or_default(),
            linked_channels: linked_channels(linked_channel_ids.unwrap_or_default()),
            tags: tags.unwrap_or_default(),
            run_id,
            assign_to_user_id,
            organization_id: organization_id.unwrap_or_default(),
            state: state.map(|s| s as i32),
            annotation_type: annotation_type as i32,
            created_by_condition_id: None,
            legend_config: None,
            created_by_rule_condition_version_id: None,
            metadata: metadata.unwrap_or_default(),
        };

        let channel = self.channel.clone();
        let resp = with_retry(&self.policy, move || {
            let channel = channel.clone();
            let request = request.clone();
            async move {
                let mut client = AnnotationServiceClient::new(channel);
                client
                    .create_annotation(request)
                    .await
                    .map(|resp| resp.into_inner())
            }
        })
        .await
        .context("failed to create annotation")?;

        resp.annotation
            .ok_or_else(|| anyhow!("create_annotation response missing annotation"))
    }

    /// Update a subset of an existing annotation's fields. Per
    /// `protos/sift/annotations/v1/annotations.proto::UpdateAnnotationRequest` the
    /// updatable fields are `name`, `description`, `start_time`, `end_time`,
    /// `assigned_to_user_id`, `state`, `tags`, `legend_config`, `linked_channels`,
    /// and `metadata`. This service exposes all but `legend_config`.
    ///
    /// `tags`, `linked_channels`, and `metadata` use REPLACE semantics — passing
    /// `Some(vec![])` clears the field.
    #[allow(clippy::too_many_arguments)]
    pub async fn update_annotation(
        &self,
        annotation_id: String,
        name: Option<String>,
        description: Option<String>,
        start_time_unix_nanos: Option<i64>,
        end_time_unix_nanos: Option<i64>,
        assigned_to_user_id: Option<String>,
        state: Option<AnnotationState>,
        tags: Option<Vec<String>>,
        linked_channel_ids: Option<Vec<String>>,
        metadata: Option<Vec<MetadataValue>>,
    ) -> Result<Annotation> {
        let mut annotation = Annotation {
            annotation_id,
            ..Default::default()
        };
        let mut paths = Vec::new();

        if let Some(v) = name {
            annotation.name = v;
            paths.push("name".to_string());
        }
        if let Some(v) = description {
            annotation.description = v;
            paths.push("description".to_string());
        }
        if let Some(v) = start_time_unix_nanos {
            annotation.start_time = Some(timestamp_from_unix_nanos(v));
            paths.push("start_time".to_string());
        }
        if let Some(v) = end_time_unix_nanos {
            annotation.end_time = Some(timestamp_from_unix_nanos(v));
            paths.push("end_time".to_string());
        }
        if let Some(v) = assigned_to_user_id {
            annotation.assigned_to_user_id = v;
            paths.push("assigned_to_user_id".to_string());
        }
        if let Some(v) = state {
            annotation.state = Some(v as i32);
            paths.push("state".to_string());
        }
        if let Some(v) = tags {
            annotation.tags = v;
            paths.push("tags".to_string());
        }
        if let Some(v) = linked_channel_ids {
            annotation.linked_channels = linked_channels(v);
            paths.push("linked_channels".to_string());
        }
        if let Some(v) = metadata {
            annotation.metadata = v;
            paths.push("metadata".to_string());
        }

        let channel = self.channel.clone();
        let resp = with_retry(&self.policy, move || {
            let channel = channel.clone();
            let annotation = annotation.clone();
            let paths = paths.clone();
            async move {
                let mut client = AnnotationServiceClient::new(channel);
                client
                    .update_annotation(UpdateAnnotationRequest {
                        annotation: Some(annotation),
                        update_mask: Some(FieldMask { paths }),
                    })
                    .await
                    .map(|resp| resp.into_inner())
            }
        })
        .await
        .context("failed to update annotation")?;

        resp.annotation
            .ok_or_else(|| anyhow!("update_annotation response missing annotation"))
    }
}
