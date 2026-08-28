use crate::policy::{RetryPolicy, with_retry};
use crate::service::common;
use anyhow::{Context, Error, Result, anyhow};
use futures::{StreamExt, stream};
use pbjson_types::{FieldMask, Timestamp};
use sift_rs::{
    SiftChannel,
    annotations::v1::{
        Annotation, AnnotationLinkedChannel, AnnotationState, AnnotationType,
        BatchArchiveAnnotationsRequest, CreateAnnotationRequest, ListAnnotationsRequest,
        ListAnnotationsResponse, UpdateAnnotationRequest, annotation_linked_channel,
        annotation_service_client::AnnotationServiceClient,
    },
    metadata::v1::MetadataValue,
};
use tonic::Code;

#[cfg(test)]
mod test;

const MAX_CONCURRENT_ANNOTATION_UPDATES: usize = 50;

#[derive(Debug)]
pub struct AnnotationUpdateFailure {
    pub annotation_id: String,
    pub error: Error,
}

#[derive(Debug)]
pub struct UpdateAnnotationsResult {
    pub annotations: Vec<Annotation>,
    pub failures: Vec<AnnotationUpdateFailure>,
    pub not_attempted: Vec<String>,
    pub batch_archive_error: Option<Error>,
    pub archive_skipped: bool,
}

async fn fan_out_bounded<T, R, F, Fut>(items: Vec<T>, concurrency: usize, mut op: F) -> Vec<R>
where
    F: FnMut(T) -> Fut,
    Fut: Future<Output = R>,
{
    let mut results = stream::iter(items.into_iter().enumerate())
        .map(|(index, item)| {
            let future = op(item);
            async move { (index, future.await) }
        })
        .buffer_unordered(concurrency)
        .collect::<Vec<_>>()
        .await;
    results.sort_unstable_by_key(|(index, _)| *index);
    results.into_iter().map(|(_, result)| result).collect()
}

fn is_backend_wide_failure(error: &Error) -> bool {
    error.downcast_ref::<tonic::Status>().is_some_and(|status| {
        matches!(
            status.code(),
            Code::ResourceExhausted
                | Code::Unavailable
                | Code::DeadlineExceeded
                | Code::Internal
                | Code::Unauthenticated
                | Code::PermissionDenied
                | Code::Cancelled
        )
    })
}

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
    ) -> Result<common::Page<Annotation>> {
        let (page_size, record_limit) = common::paging(limit);

        let mut page_token = String::new();
        let mut results = Vec::new();
        let mut has_more = false;

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

            if results.len() >= record_limit {
                // The cap, not the end of the data: report that more exist so the
                // caller does not read this page's size as the match total.
                has_more = results.len() > record_limit || !next_page_token.is_empty();
                break;
            }
            if next_page_token.is_empty() {
                break;
            }
            page_token = next_page_token;
        }

        results.truncate(record_limit);

        Ok(common::Page {
            items: results,
            has_more,
        })
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

    pub async fn batch_archive_annotations(
        &self,
        annotation_ids: Vec<String>,
    ) -> Result<Vec<Annotation>> {
        let channel = self.channel.clone();
        let resp = with_retry(&self.policy, move || {
            let channel = channel.clone();
            let annotation_ids = annotation_ids.clone();
            async move {
                let mut client = AnnotationServiceClient::new(channel);
                client
                    .batch_archive_annotations(BatchArchiveAnnotationsRequest { annotation_ids })
                    .await
                    .map(|resp| resp.into_inner())
            }
        })
        .await
        .context("failed to batch archive annotations")?;

        Ok(resp.annotations)
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn update_annotations(
        &self,
        annotation_ids: Vec<String>,
        name: Option<String>,
        description: Option<String>,
        start_time_unix_nanos: Option<i64>,
        end_time_unix_nanos: Option<i64>,
        assigned_to_user_id: Option<String>,
        state: Option<AnnotationState>,
        tags: Option<Vec<String>>,
        linked_channel_ids: Option<Vec<String>>,
        metadata: Option<Vec<MetadataValue>>,
        is_archived: Option<bool>,
    ) -> Result<UpdateAnnotationsResult> {
        let has_field_update = name.is_some()
            || description.is_some()
            || start_time_unix_nanos.is_some()
            || end_time_unix_nanos.is_some()
            || assigned_to_user_id.is_some()
            || state.is_some()
            || tags.is_some()
            || linked_channel_ids.is_some()
            || metadata.is_some();
        let mut annotations = Vec::new();
        let mut failures = Vec::new();
        let mut not_attempted = Vec::new();

        if has_field_update {
            annotations.reserve(annotation_ids.len());
            for chunk_start in (0..annotation_ids.len()).step_by(MAX_CONCURRENT_ANNOTATION_UPDATES)
            {
                let chunk_end =
                    (chunk_start + MAX_CONCURRENT_ANNOTATION_UPDATES).min(annotation_ids.len());
                let service = self.clone();
                let name = name.clone();
                let description = description.clone();
                let assigned_to_user_id = assigned_to_user_id.clone();
                let tags = tags.clone();
                let linked_channel_ids = linked_channel_ids.clone();
                let metadata = metadata.clone();
                let updates = fan_out_bounded(
                    annotation_ids[chunk_start..chunk_end].to_vec(),
                    MAX_CONCURRENT_ANNOTATION_UPDATES,
                    move |annotation_id| {
                        let service = service.clone();
                        let name = name.clone();
                        let description = description.clone();
                        let assigned_to_user_id = assigned_to_user_id.clone();
                        let tags = tags.clone();
                        let linked_channel_ids = linked_channel_ids.clone();
                        let metadata = metadata.clone();
                        async move {
                            let result = service
                                .update_annotation(
                                    annotation_id.clone(),
                                    name,
                                    description,
                                    start_time_unix_nanos,
                                    end_time_unix_nanos,
                                    assigned_to_user_id,
                                    state,
                                    tags,
                                    linked_channel_ids,
                                    metadata,
                                )
                                .await;
                            (annotation_id, result)
                        }
                    },
                )
                .await;
                let stop_after_batch = updates
                    .iter()
                    .any(|(_, result)| result.as_ref().is_err_and(is_backend_wide_failure));

                for (annotation_id, result) in updates {
                    match result {
                        Ok(annotation) => annotations.push(annotation),
                        Err(error) => failures.push(AnnotationUpdateFailure {
                            annotation_id,
                            error,
                        }),
                    }
                }

                if stop_after_batch {
                    not_attempted.extend_from_slice(&annotation_ids[chunk_end..]);
                    break;
                }
            }
        }

        let mut batch_archive_error = None;
        let mut archive_skipped = false;
        if is_archived == Some(true) {
            if failures.is_empty() && not_attempted.is_empty() {
                let result = self.batch_archive_annotations(annotation_ids).await;
                match result {
                    Ok(archived) => annotations = archived,
                    Err(error) => batch_archive_error = Some(error),
                }
            } else {
                archive_skipped = true;
            }
        } else if is_archived == Some(false) && failures.is_empty() && not_attempted.is_empty() {
            annotations.clear();
            annotations.reserve(annotation_ids.len());
            for chunk_start in (0..annotation_ids.len()).step_by(MAX_CONCURRENT_ANNOTATION_UPDATES)
            {
                let chunk_end =
                    (chunk_start + MAX_CONCURRENT_ANNOTATION_UPDATES).min(annotation_ids.len());
                let service = self.clone();
                let unarchives = fan_out_bounded(
                    annotation_ids[chunk_start..chunk_end].to_vec(),
                    MAX_CONCURRENT_ANNOTATION_UPDATES,
                    move |annotation_id| {
                        let service = service.clone();
                        async move {
                            let result = service.unarchive_annotation(annotation_id.clone()).await;
                            (annotation_id, result)
                        }
                    },
                )
                .await;
                let stop_after_batch = unarchives
                    .iter()
                    .any(|(_, result)| result.as_ref().is_err_and(is_backend_wide_failure));

                for (annotation_id, result) in unarchives {
                    match result {
                        Ok(annotation) => annotations.push(annotation),
                        Err(error) => failures.push(AnnotationUpdateFailure {
                            annotation_id,
                            error,
                        }),
                    }
                }

                if stop_after_batch {
                    not_attempted.extend_from_slice(&annotation_ids[chunk_end..]);
                    break;
                }
            }
        } else if is_archived == Some(false) {
            archive_skipped = true;
        }

        Ok(UpdateAnnotationsResult {
            annotations,
            failures,
            not_attempted,
            batch_archive_error,
            archive_skipped,
        })
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

    #[allow(deprecated)] // The backend requires this deprecated field to unarchive annotations.
    pub async fn unarchive_annotation(&self, annotation_id: String) -> Result<Annotation> {
        let channel = self.channel.clone();
        let resp = with_retry(&self.policy, move || {
            let channel = channel.clone();
            let annotation_id = annotation_id.clone();
            async move {
                let mut client = AnnotationServiceClient::new(channel);
                client
                    .update_annotation(UpdateAnnotationRequest {
                        annotation: Some(Annotation {
                            annotation_id,
                            deleted_date: None,
                            ..Default::default()
                        }),
                        update_mask: Some(FieldMask {
                            paths: vec!["deleted_date".into()],
                        }),
                    })
                    .await
                    .map(|resp| resp.into_inner())
            }
        })
        .await
        .context("failed to unarchive annotation")?;

        resp.annotation
            .ok_or_else(|| anyhow!("unarchive_annotation response missing annotation"))
    }
}
