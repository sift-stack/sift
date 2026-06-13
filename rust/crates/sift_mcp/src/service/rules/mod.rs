use crate::service::common;
use anyhow::{Context, Result};
use sift_rs::{
    SiftChannel,
    metadata::v1::MetadataValue,
    rules::v1::{
        CreateRuleRequest, CreateRuleResponse, GetRuleRequest, GetRuleResponse, ListRulesRequest,
        ListRulesResponse, Rule, RuleAction, RuleAssetConfiguration, RuleCondition,
        UpdateActionRequest, UpdateConditionRequest, UpdateRuleRequest, UpdateRuleResponse,
        rule_service_client::RuleServiceClient,
    },
};
use tonic::Status;

#[cfg(test)]
mod test;

/// Fields a caller may change via [`RuleService::update_rule`]. `None` leaves a
/// field at its current value (read-modify-write); the rule update RPC is a full
/// replace, so omitted fields are reconstructed from the fetched rule.
#[derive(Default)]
pub struct RuleUpdate {
    pub name: Option<String>,
    pub description: Option<String>,
    pub conditions: Option<Vec<UpdateConditionRequest>>,
    pub asset_configuration: Option<RuleAssetConfiguration>,
    pub metadata: Option<Vec<MetadataValue>>,
    pub version_notes: Option<String>,
    pub is_live_evaluation_enabled: Option<bool>,
}

/// Convert a read-shape [`RuleCondition`] back into the write-shape
/// [`UpdateConditionRequest`] so existing conditions survive a read-modify-write
/// update. `expression` and action `configuration` are the same types in both
/// shapes, so only the ids and the action wrapper need remapping.
fn condition_to_update(condition: RuleCondition) -> UpdateConditionRequest {
    UpdateConditionRequest {
        rule_condition_id: Some(condition.rule_condition_id).filter(|s| !s.is_empty()),
        expression: condition.expression,
        actions: condition
            .actions
            .into_iter()
            .map(action_to_update)
            .collect(),
    }
}

fn action_to_update(action: RuleAction) -> UpdateActionRequest {
    UpdateActionRequest {
        rule_action_id: Some(action.rule_action_id).filter(|s| !s.is_empty()),
        action_type: action.action_type,
        configuration: action.configuration,
    }
}

#[derive(Clone)]
pub struct RuleService {
    channel: SiftChannel,
}

impl RuleService {
    pub fn new(channel: SiftChannel) -> Self {
        Self { channel }
    }

    /// Fetch a single rule by id or client key. Exactly one of `id` /
    /// `client_key` should be non-empty; the tool layer enforces that.
    pub async fn get_rule(&self, id: String, client_key: String) -> Result<Rule> {
        let mut client = RuleServiceClient::new(self.channel.clone());

        let resp = client
            .get_rule(GetRuleRequest {
                rule_id: id,
                client_key,
            })
            .await
            .context("failed to get rule")?;

        let GetRuleResponse { rule } = resp.into_inner();
        rule.ok_or_else(|| Status::not_found("rule not found").into())
    }

    /// Create a rule from a fully-built update request, then fetch and return the
    /// created rule (the create RPC returns only the new id).
    pub async fn create_rule(&self, update: UpdateRuleRequest) -> Result<Rule> {
        let mut client = RuleServiceClient::new(self.channel.clone());

        let resp = client
            .create_rule(CreateRuleRequest {
                update: Some(update),
            })
            .await
            .context("failed to create rule")?;

        let CreateRuleResponse { rule_id } = resp.into_inner();
        self.get_rule(rule_id, String::new()).await
    }

    /// Read-modify-write: fetch the current rule, overlay the provided fields
    /// onto a full update request (the update RPC is a full replace), apply it,
    /// then fetch and return the updated rule.
    pub async fn update_rule(
        &self,
        id: String,
        client_key: String,
        update: RuleUpdate,
    ) -> Result<Rule> {
        let current = self.get_rule(id, client_key).await?;

        let conditions = update.conditions.unwrap_or_else(|| {
            current
                .conditions
                .into_iter()
                .map(condition_to_update)
                .collect()
        });

        let request = UpdateRuleRequest {
            rule_id: Some(current.rule_id),
            name: update.name.unwrap_or(current.name),
            description: update.description.unwrap_or(current.description),
            conditions,
            organization_id: current.organization_id,
            version_notes: update.version_notes.unwrap_or_default(),
            client_key: Some(current.client_key).filter(|s| !s.is_empty()),
            asset_configuration: update.asset_configuration.or(current.asset_configuration),
            contextual_channels: current.contextual_channels,
            is_external: current.is_external,
            metadata: update.metadata.unwrap_or(current.metadata),
            is_archived: current.is_archived,
            is_live_evaluation_enabled: Some(
                update
                    .is_live_evaluation_enabled
                    .unwrap_or(current.is_live_evaluation_enabled),
            ),
            ..Default::default()
        };

        let mut client = RuleServiceClient::new(self.channel.clone());
        let resp = client
            .update_rule(request)
            .await
            .context("failed to update rule")?;

        let UpdateRuleResponse { rule_id } = resp.into_inner();
        self.get_rule(rule_id, String::new()).await
    }

    pub async fn list_rules(
        &self,
        filter: String,
        order_by: Option<String>,
        limit: Option<u32>,
    ) -> Result<Vec<Rule>> {
        let (page_size, record_limit) = common::paging(limit);

        let mut client = RuleServiceClient::new(self.channel.clone());
        let mut page_token = String::new();
        let mut results = Vec::new();

        loop {
            let resp = client
                .list_rules(ListRulesRequest {
                    filter: filter.clone(),
                    page_size,
                    page_token,
                    order_by: order_by.clone().unwrap_or_default(),
                })
                .await
                .context("failed to query rules")?;

            let ListRulesResponse {
                rules,
                next_page_token,
            } = resp.into_inner();
            if rules.is_empty() {
                break;
            }
            results.extend(rules);

            if results.len() >= record_limit || next_page_token.is_empty() {
                break;
            }
            page_token = next_page_token;
        }

        results.truncate(record_limit);

        Ok(results)
    }
}
