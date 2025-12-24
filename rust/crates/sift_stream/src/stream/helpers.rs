use crate::stream::flow::{FlowBuilder, FlowDescriptor};
use crate::stream::mode::ingestion_config::Flow;
use sift_connect::SiftChannel;
use sift_error::prelude::*;
use sift_rs::runs::v2::Run;
use sift_rs::{
    assets::v1::Asset,
    ingest::v1::{IngestWithConfigDataChannelValue, IngestWithConfigDataStreamRequest},
    metadata::v1::MetadataValue,
    retry::{RetryConfig, RetryExt},
    wrappers::assets::{AssetServiceWrapper, new_asset_service},
};

/// Flows passed into this function should have names match `flow_name`. The only case
/// in which this returns `None` is if there is no [FlowConfig] for the given `message`.
pub(crate) fn message_to_ingest_req(
    message: &Flow,
    run: Option<&Run>,
    descriptor: &FlowDescriptor<String>,
) -> Option<IngestWithConfigDataStreamRequest> {
    // Create a vector of empty channel values. If the provided channel values
    // have a matching channel name and data type, the value will be updated.
    let mut builder = FlowBuilder::new(descriptor);

    // Update all provided channel values in the flow.
    for value in message.values.iter() {
        builder
            .set_with_key(&value.name, value.value.clone())
            .ok()?;
    }

    // Attach the run ID to the flow if it is provided.
    if let Some(run) = run {
        builder.attach_run_id(run.run_id.clone());
    }

    Some(builder.request(message.timestamp.clone()))
}

/// Creates an [IngestWithConfigDataStreamRequest] directly without consulting the flow cache.
pub(crate) fn message_to_ingest_req_direct(
    message: &Flow,
    ingestion_config_id: &str,
    run: Option<&Run>,
) -> IngestWithConfigDataStreamRequest {
    let channel_values = message
        .values
        .iter()
        .map(|val| IngestWithConfigDataChannelValue {
            r#type: Some(val.pb_value()),
        })
        .collect::<Vec<_>>();

    IngestWithConfigDataStreamRequest {
        channel_values,
        flow: message.flow_name.to_string(),
        ingestion_config_id: ingestion_config_id.to_string(),
        timestamp: Some(message.timestamp.0),
        run_id: run.map(|r| r.run_id.clone()).unwrap_or_default(),
        ..Default::default()
    }
}

/// Updates asset tags and metadata if provided. Returns early if neither is provided.
pub(crate) async fn update_asset_tags_and_metadata(
    mut asset: Asset,
    asset_tags: Option<Vec<String>>,
    asset_metadata: Option<Vec<MetadataValue>>,
    channel: SiftChannel,
) -> Result<()> {
    let mut update_mask = Vec::new();

    if let Some(asset_tags) = asset_tags {
        asset.tags = asset_tags;
        update_mask.push("tags".to_string());
    }

    if let Some(asset_metadata) = asset_metadata {
        asset.metadata = asset_metadata;
        update_mask.push("metadata".to_string());
    }

    if update_mask.is_empty() {
        return Ok(());
    }

    let asset_service_wrapper = new_asset_service(channel);
    let retrying_asset = asset_service_wrapper.retrying(RetryConfig::default());
    let asset_clone = asset.clone();
    let update_mask_clone = update_mask.clone();
    let _ = retrying_asset
        .call(|mut w| {
            let asset = asset_clone.clone();
            let update_mask = update_mask_clone.clone();
            async move { w.try_update_asset(asset, update_mask).await }
        })
        .await?;

    Ok(())
}
