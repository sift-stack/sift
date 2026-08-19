use std::time::{SystemTime, UNIX_EPOCH};

use crate::policy::{RetryPolicy, with_retry};
use crate::service::common;
use anyhow::{Context, Result, anyhow};
use pbjson_types::{FieldMask, Timestamp};
use sift_rs::{
    SiftChannel,
    calculated_channels::v2::{
        CalculatedChannel, CalculatedChannelAbstractChannelReference,
        CalculatedChannelAssetConfiguration, CalculatedChannelConfiguration,
        CalculatedChannelQueryConfiguration, CalculatedChannelValidationResult,
        CreateCalculatedChannelRequest, GetCalculatedChannelRequest,
        ListCalculatedChannelVersionsRequest, ListCalculatedChannelVersionsResponse,
        ListCalculatedChannelsRequest, ListCalculatedChannelsResponse,
        UpdateCalculatedChannelRequest,
        calculated_channel_asset_configuration::{AssetScope, AssetSelection},
        calculated_channel_query_configuration::{Query, Sel},
        calculated_channel_service_client::CalculatedChannelServiceClient,
    },
    metadata::v1::MetadataValue,
};

#[cfg(test)]
mod test;

/// A full calculated channel definition to create. The caller guarantees the
/// asset scope is unambiguous: either `all_assets` is set, or `asset_ids` /
/// `tag_ids` name a selection.
#[derive(Debug, Default)]
pub struct NewCalculatedChannel {
    pub name: String,
    pub description: Option<String>,
    pub user_notes: Option<String>,
    pub units: Option<String>,
    pub client_key: Option<String>,
    pub metadata: Vec<MetadataValue>,
    pub expression: String,
    pub expression_channel_references: Vec<CalculatedChannelAbstractChannelReference>,
    pub all_assets: bool,
    pub asset_ids: Vec<String>,
    pub tag_ids: Vec<String>,
}

/// A partial set of changes to apply to an existing calculated channel. Every
/// field is optional; `None` means "leave unchanged". Only the fields set here
/// land in the update mask, so everything else on the channel is preserved.
#[derive(Debug, Default)]
pub struct CalculatedChannelUpdate {
    pub name: Option<String>,
    pub description: Option<String>,
    pub units: Option<String>,
    pub metadata: Option<Vec<MetadataValue>>,
    pub expression: Option<String>,
    pub expression_channel_references: Option<Vec<CalculatedChannelAbstractChannelReference>>,
    pub all_assets: Option<bool>,
    pub asset_ids: Option<Vec<String>>,
    pub tag_ids: Option<Vec<String>>,
    pub user_notes: Option<String>,
}

impl CalculatedChannelUpdate {
    /// Whether any updatable field is set. An update with nothing set would
    /// produce an empty mask, which the API treats as a no-op.
    pub fn is_empty(&self) -> bool {
        self.name.is_none()
            && self.description.is_none()
            && self.units.is_none()
            && self.metadata.is_none()
            && self.expression.is_none()
            && self.expression_channel_references.is_none()
            && self.all_assets.is_none()
            && self.asset_ids.is_none()
            && self.tag_ids.is_none()
            && self.user_notes.is_none()
    }
}

/// The result of a create or update write: the stored calculated channel plus
/// the assets the API reported as inapplicable (they lack a channel the
/// expression references), which the caller should surface to the user.
#[derive(Debug)]
pub struct CalculatedChannelWrite {
    pub calculated_channel: CalculatedChannel,
    pub inapplicable_assets: Vec<CalculatedChannelValidationResult>,
}

#[derive(Clone)]
pub struct CalculatedChannelService {
    channel: SiftChannel,
    policy: RetryPolicy,
}

impl CalculatedChannelService {
    pub fn new(channel: SiftChannel, policy: RetryPolicy) -> Self {
        Self { channel, policy }
    }

    pub async fn list_calculated_channels(
        &self,
        filter: String,
        order_by: Option<String>,
        limit: Option<u32>,
    ) -> Result<Vec<CalculatedChannel>> {
        let (page_size, record_limit) = common::paging(limit);

        let mut page_token = String::new();
        let mut results = Vec::new();

        let order_by = order_by.unwrap_or_default();

        loop {
            let grpc_channel = self.channel.clone();
            let filter = filter.clone();
            let order_by = order_by.clone();
            let token = page_token.clone();

            let resp = with_retry(&self.policy, move || {
                let grpc_channel = grpc_channel.clone();
                let filter = filter.clone();
                let order_by = order_by.clone();
                let token = token.clone();
                async move {
                    let mut client = CalculatedChannelServiceClient::new(grpc_channel);
                    client
                        .list_calculated_channels(ListCalculatedChannelsRequest {
                            page_size,
                            page_token: token,
                            filter,
                            organization_id: String::new(),
                            order_by,
                        })
                        .await
                        .map(|resp| resp.into_inner())
                }
            })
            .await
            .context("failed to query calculated channels")?;

            let ListCalculatedChannelsResponse {
                calculated_channels,
                next_page_token,
            } = resp;
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

    /// Lists the version history of a single calculated channel. Each version is
    /// a full [`CalculatedChannel`] snapshot, not a reduced version record.
    pub async fn list_calculated_channel_versions(
        &self,
        calculated_channel_id: String,
        filter: String,
        order_by: Option<String>,
        limit: Option<u32>,
    ) -> Result<Vec<CalculatedChannel>> {
        let (page_size, record_limit) = common::paging(limit);

        let mut page_token = String::new();
        let mut results = Vec::new();

        let order_by = order_by.unwrap_or_default();

        loop {
            let grpc_channel = self.channel.clone();
            let calculated_channel_id = calculated_channel_id.clone();
            let filter = filter.clone();
            let order_by = order_by.clone();
            let token = page_token.clone();

            let resp = with_retry(&self.policy, move || {
                let grpc_channel = grpc_channel.clone();
                let calculated_channel_id = calculated_channel_id.clone();
                let filter = filter.clone();
                let order_by = order_by.clone();
                let token = token.clone();
                async move {
                    let mut client = CalculatedChannelServiceClient::new(grpc_channel);
                    client
                        .list_calculated_channel_versions(ListCalculatedChannelVersionsRequest {
                            calculated_channel_id,
                            client_key: String::new(),
                            page_size,
                            page_token: token,
                            filter,
                            organization_id: String::new(),
                            order_by,
                        })
                        .await
                        .map(|resp| resp.into_inner())
                }
            })
            .await
            .context("failed to query calculated channel versions")?;

            let ListCalculatedChannelVersionsResponse {
                calculated_channel_versions,
                next_page_token,
            } = resp;
            if calculated_channel_versions.is_empty() {
                break;
            }
            results.extend(calculated_channel_versions);

            if results.len() >= record_limit || next_page_token.is_empty() {
                break;
            }
            page_token = next_page_token;
        }

        results.truncate(record_limit);

        Ok(results)
    }

    pub async fn create_calculated_channel(
        &self,
        new: NewCalculatedChannel,
    ) -> Result<CalculatedChannelWrite> {
        let NewCalculatedChannel {
            name,
            description,
            user_notes,
            units,
            client_key,
            metadata,
            expression,
            expression_channel_references,
            all_assets,
            asset_ids,
            tag_ids,
        } = new;

        let configuration = CalculatedChannelConfiguration {
            asset_configuration: Some(CalculatedChannelAssetConfiguration {
                asset_scope: Some(asset_scope(all_assets, asset_ids, tag_ids)),
            }),
            query_configuration: Some(CalculatedChannelQueryConfiguration {
                query: Some(Query::Sel(Sel {
                    expression,
                    expression_channel_references,
                })),
            }),
        };

        let request = CreateCalculatedChannelRequest {
            name,
            description: description.unwrap_or_default(),
            user_notes: user_notes.unwrap_or_default(),
            units,
            client_key,
            calculated_channel_configuration: Some(configuration),
            metadata,
        };

        let grpc_channel = self.channel.clone();
        let resp = with_retry(&self.policy, move || {
            let grpc_channel = grpc_channel.clone();
            let request = request.clone();
            async move {
                let mut client = CalculatedChannelServiceClient::new(grpc_channel);
                client
                    .create_calculated_channel(request)
                    .await
                    .map(|resp| resp.into_inner())
            }
        })
        .await
        .context("failed to create calculated channel")?;

        let calculated_channel = resp.calculated_channel.ok_or_else(|| {
            anyhow!("create_calculated_channel response missing calculated channel")
        })?;

        Ok(CalculatedChannelWrite {
            calculated_channel,
            inapplicable_assets: resp.inapplicable_assets,
        })
    }

    /// Updates an existing calculated channel, creating a new version. The
    /// current channel is fetched first so partially-specified nested
    /// configuration (an expression without new references, asset ids without
    /// tag ids) overlays the stored value instead of clearing it. Only the
    /// fields the caller set are named in the update mask.
    pub async fn update_calculated_channel(
        &self,
        calculated_channel_id: String,
        changes: CalculatedChannelUpdate,
    ) -> Result<CalculatedChannelWrite> {
        let mut channel = self
            .get_calculated_channel(calculated_channel_id.clone())
            .await?
            .ok_or_else(|| anyhow!("calculated channel '{calculated_channel_id}' not found"))?;
        channel.calculated_channel_id = calculated_channel_id;

        let CalculatedChannelUpdate {
            name,
            description,
            units,
            metadata,
            expression,
            expression_channel_references,
            all_assets,
            asset_ids,
            tag_ids,
            user_notes,
        } = changes;

        let mut paths = Vec::new();

        if let Some(v) = name {
            channel.name = v;
            paths.push("name".to_string());
        }
        if let Some(v) = description {
            channel.description = v;
            paths.push("description".to_string());
        }
        if let Some(v) = units {
            channel.units = Some(v);
            paths.push("units".to_string());
        }
        if let Some(v) = metadata {
            channel.metadata = v;
            paths.push("metadata".to_string());
        }

        let mut configuration = channel.calculated_channel_configuration.unwrap_or_default();

        if expression.is_some() || expression_channel_references.is_some() {
            let mut sel = current_sel(&configuration);
            if let Some(v) = expression {
                sel.expression = v;
            }
            if let Some(v) = expression_channel_references {
                sel.expression_channel_references = v;
            }
            configuration.query_configuration = Some(CalculatedChannelQueryConfiguration {
                query: Some(Query::Sel(sel)),
            });
            paths.push("query_configuration".to_string());
        }

        if all_assets.is_some() || asset_ids.is_some() || tag_ids.is_some() {
            let scope = match all_assets {
                Some(true) => AssetScope::AllAssets(true),
                _ => {
                    let mut selection = current_selection(&configuration);
                    if let Some(v) = asset_ids {
                        selection.asset_ids = v;
                    }
                    if let Some(v) = tag_ids {
                        selection.tag_ids = v;
                    }
                    AssetScope::Selection(selection)
                }
            };
            configuration.asset_configuration = Some(CalculatedChannelAssetConfiguration {
                asset_scope: Some(scope),
            });
            paths.push("asset_configuration".to_string());
        }

        channel.calculated_channel_configuration = Some(configuration);

        self.send_update(channel, paths, user_notes).await
    }

    /// Archives a calculated channel by stamping `archived_date`. There is no
    /// dedicated archive RPC; the API archives through the update mask.
    pub async fn archive_calculated_channel(
        &self,
        calculated_channel_id: String,
    ) -> Result<CalculatedChannelWrite> {
        let channel = CalculatedChannel {
            calculated_channel_id,
            archived_date: Some(now_timestamp()),
            ..Default::default()
        };

        self.send_update(channel, vec!["archived_date".to_string()], None)
            .await
            .context("failed to archive calculated channel")
    }

    /// Unarchives a calculated channel by clearing `archived_date` through the
    /// update mask. A masked field left at its default is cleared.
    pub async fn unarchive_calculated_channel(
        &self,
        calculated_channel_id: String,
    ) -> Result<CalculatedChannelWrite> {
        let channel = CalculatedChannel {
            calculated_channel_id,
            archived_date: None,
            ..Default::default()
        };

        self.send_update(channel, vec!["archived_date".to_string()], None)
            .await
            .context("failed to unarchive calculated channel")
    }

    /// Retrieves the latest version of a calculated channel by id, or `None` if
    /// it does not exist. Not exposed as a tool: `list_calculated_channels`
    /// filtered by id covers the read case.
    async fn get_calculated_channel(
        &self,
        calculated_channel_id: String,
    ) -> Result<Option<CalculatedChannel>> {
        let grpc_channel = self.channel.clone();

        let resp = with_retry(&self.policy, move || {
            let grpc_channel = grpc_channel.clone();
            let calculated_channel_id = calculated_channel_id.clone();
            async move {
                let mut client = CalculatedChannelServiceClient::new(grpc_channel);
                client
                    .get_calculated_channel(GetCalculatedChannelRequest {
                        calculated_channel_id,
                        client_key: String::new(),
                        organization_id: String::new(),
                        calculated_channel_version_id: String::new(),
                    })
                    .await
                    .map(|resp| resp.into_inner())
            }
        })
        .await
        .context("failed to fetch calculated channel")?;

        Ok(resp.calculated_channel)
    }

    async fn send_update(
        &self,
        channel: CalculatedChannel,
        paths: Vec<String>,
        user_notes: Option<String>,
    ) -> Result<CalculatedChannelWrite> {
        let grpc_channel = self.channel.clone();
        let resp = with_retry(&self.policy, move || {
            let grpc_channel = grpc_channel.clone();
            let channel = channel.clone();
            let paths = paths.clone();
            let user_notes = user_notes.clone();
            async move {
                let mut client = CalculatedChannelServiceClient::new(grpc_channel);
                client
                    .update_calculated_channel(UpdateCalculatedChannelRequest {
                        calculated_channel: Some(channel),
                        update_mask: Some(FieldMask { paths }),
                        user_notes,
                    })
                    .await
                    .map(|resp| resp.into_inner())
            }
        })
        .await
        .context("failed to update calculated channel")?;

        let calculated_channel = resp.calculated_channel.ok_or_else(|| {
            anyhow!("update_calculated_channel response missing calculated channel")
        })?;

        Ok(CalculatedChannelWrite {
            calculated_channel,
            inapplicable_assets: resp.inapplicable_assets,
        })
    }
}

/// Build the asset scope oneof from the flat inputs the tool validated.
fn asset_scope(all_assets: bool, asset_ids: Vec<String>, tag_ids: Vec<String>) -> AssetScope {
    if all_assets {
        AssetScope::AllAssets(true)
    } else {
        AssetScope::Selection(AssetSelection { asset_ids, tag_ids })
    }
}

/// The stored SEL query, or an empty one when the channel carries no query
/// configuration yet.
fn current_sel(configuration: &CalculatedChannelConfiguration) -> Sel {
    match configuration
        .query_configuration
        .as_ref()
        .and_then(|q| q.query.as_ref())
    {
        Some(Query::Sel(sel)) => sel.clone(),
        None => Sel::default(),
    }
}

/// The stored asset selection, or an empty one when the channel is scoped to
/// all assets or carries no asset configuration yet.
fn current_selection(configuration: &CalculatedChannelConfiguration) -> AssetSelection {
    match configuration
        .asset_configuration
        .as_ref()
        .and_then(|a| a.asset_scope.as_ref())
    {
        Some(AssetScope::Selection(selection)) => selection.clone(),
        _ => AssetSelection::default(),
    }
}

fn now_timestamp() -> Timestamp {
    let elapsed = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default();
    Timestamp {
        seconds: elapsed.as_secs() as i64,
        nanos: elapsed.subsec_nanos() as i32,
    }
}
