use crate::service::common;
use anyhow::{Context, Result};
use pbjson_types::FieldMask;
use sift_rs::{
    SiftChannel,
    calculated_channels::v2::{
        CalculatedChannel, CalculatedChannelAbstractChannelReference,
        CalculatedChannelAssetConfiguration, CalculatedChannelConfiguration,
        CalculatedChannelQueryConfiguration, CreateCalculatedChannelRequest,
        CreateCalculatedChannelResponse, GetCalculatedChannelRequest, GetCalculatedChannelResponse,
        ListCalculatedChannelsRequest, ListCalculatedChannelsResponse,
        UpdateCalculatedChannelRequest, UpdateCalculatedChannelResponse,
        calculated_channel_query_configuration::{Query, Sel},
        calculated_channel_service_client::CalculatedChannelServiceClient,
    },
    metadata::v1::MetadataValue,
};
use tonic::Status;

#[cfg(test)]
mod test;

/// Fields a caller may change via [`CalculatedChannelService::update_calculated_channel`].
/// `None` leaves a field unchanged. `expression` / `channel_references` overlay the
/// current query configuration so either can be changed independently.
#[derive(Default)]
pub struct CalculatedChannelUpdate {
    pub name: Option<String>,
    pub description: Option<String>,
    pub units: Option<String>,
    pub expression: Option<String>,
    pub channel_references: Option<Vec<CalculatedChannelAbstractChannelReference>>,
    pub asset_configuration: Option<CalculatedChannelAssetConfiguration>,
    pub metadata: Option<Vec<MetadataValue>>,
    pub user_notes: Option<String>,
}

#[derive(Clone)]
pub struct CalculatedChannelService {
    channel: SiftChannel,
}

impl CalculatedChannelService {
    pub fn new(channel: SiftChannel) -> Self {
        Self { channel }
    }

    pub async fn list_calculated_channels(
        &self,
        filter: String,
        order_by: Option<String>,
        limit: Option<u32>,
    ) -> Result<Vec<CalculatedChannel>> {
        let (page_size, record_limit) = common::paging(limit);

        let mut client = CalculatedChannelServiceClient::new(self.channel.clone());
        let mut page_token = String::new();
        let mut results = Vec::new();

        loop {
            let resp = client
                .list_calculated_channels(ListCalculatedChannelsRequest {
                    filter: filter.clone(),
                    page_size,
                    page_token,
                    order_by: order_by.clone().unwrap_or_default(),
                    organization_id: String::new(),
                })
                .await
                .context("failed to query calculated channels")?;

            let ListCalculatedChannelsResponse {
                calculated_channels,
                next_page_token,
            } = resp.into_inner();
            if calculated_channels.is_empty() {
                break;
            }
            results.extend(calculated_channels);

            if results.len() >= record_limit || next_page_token.is_empty() {
                break;
            }
            page_token = next_page_token;
        }

        results.truncate(record_limit);

        Ok(results)
    }

    /// Fetch a single calculated channel by id or client key. Exactly one of
    /// `id` / `client_key` should be non-empty; the tool layer enforces that.
    pub async fn get_calculated_channel(
        &self,
        id: String,
        client_key: String,
    ) -> Result<CalculatedChannel> {
        let mut client = CalculatedChannelServiceClient::new(self.channel.clone());

        let resp = client
            .get_calculated_channel(GetCalculatedChannelRequest {
                calculated_channel_id: id,
                client_key,
                organization_id: String::new(),
                calculated_channel_version_id: String::new(),
            })
            .await
            .context("failed to get calculated channel")?;

        let GetCalculatedChannelResponse { calculated_channel } = resp.into_inner();

        calculated_channel.ok_or_else(|| Status::not_found("calculated channel not found").into())
    }

    /// Returns the full create response so the caller can surface
    /// `inapplicable_assets` (assets the channel could not be applied to).
    #[allow(clippy::too_many_arguments)]
    pub async fn create_calculated_channel(
        &self,
        name: String,
        description: String,
        units: Option<String>,
        client_key: Option<String>,
        user_notes: String,
        configuration: CalculatedChannelConfiguration,
        metadata: Vec<MetadataValue>,
    ) -> Result<CreateCalculatedChannelResponse> {
        let mut client = CalculatedChannelServiceClient::new(self.channel.clone());

        let resp = client
            .create_calculated_channel(CreateCalculatedChannelRequest {
                name,
                description,
                user_notes,
                units,
                client_key,
                calculated_channel_configuration: Some(configuration),
                metadata,
            })
            .await
            .context("failed to create calculated channel")?;

        Ok(resp.into_inner())
    }

    /// Read-modify-write: fetch the current channel, overlay the provided
    /// fields, and update with a mask covering exactly those fields.
    pub async fn update_calculated_channel(
        &self,
        id: String,
        client_key: String,
        update: CalculatedChannelUpdate,
    ) -> Result<UpdateCalculatedChannelResponse> {
        let mut current = self.get_calculated_channel(id, client_key).await?;

        let mut paths = Vec::new();
        if let Some(name) = update.name {
            current.name = name;
            paths.push("name".to_string());
        }
        if let Some(description) = update.description {
            current.description = description;
            paths.push("description".to_string());
        }
        if let Some(units) = update.units {
            current.units = Some(units);
            paths.push("units".to_string());
        }
        if let Some(metadata) = update.metadata {
            current.metadata = metadata;
            paths.push("metadata".to_string());
        }
        if let Some(asset_configuration) = update.asset_configuration {
            let config = current
                .calculated_channel_configuration
                .get_or_insert_with(Default::default);
            config.asset_configuration = Some(asset_configuration);
            paths.push("asset_configuration".to_string());
        }
        if update.expression.is_some() || update.channel_references.is_some() {
            let config = current
                .calculated_channel_configuration
                .get_or_insert_with(Default::default);
            // Start from the current Sel so an expression-only or references-only
            // change preserves the other half.
            let mut sel = match config.query_configuration.take().and_then(|q| q.query) {
                Some(Query::Sel(sel)) => sel,
                None => Sel::default(),
            };
            if let Some(expression) = update.expression {
                sel.expression = expression;
            }
            if let Some(channel_references) = update.channel_references {
                sel.expression_channel_references = channel_references;
            }
            config.query_configuration = Some(CalculatedChannelQueryConfiguration {
                query: Some(Query::Sel(sel)),
            });
            paths.push("query_configuration".to_string());
        }

        if paths.is_empty() {
            return Err(Status::invalid_argument(
                "no updatable fields provided; set at least one of name, description, units, expression, channel_references, asset scope, metadata",
            )
            .into());
        }

        let mut client = CalculatedChannelServiceClient::new(self.channel.clone());
        let resp = client
            .update_calculated_channel(UpdateCalculatedChannelRequest {
                calculated_channel: Some(current),
                update_mask: Some(FieldMask { paths }),
                user_notes: update.user_notes,
            })
            .await
            .context("failed to update calculated channel")?;

        Ok(resp.into_inner())
    }
}
