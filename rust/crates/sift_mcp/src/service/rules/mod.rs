use crate::policy::{RetryPolicy, with_retry};
use crate::service::common;
use anyhow::{Context, Result};
use sift_rs::{
    SiftChannel,
    rules::v1::{
        ActionKind, ArchiveRuleRequest, CreateRuleRequest, GetRuleRequest, ListRuleVersionsRequest,
        ListRuleVersionsResponse, ListRulesRequest, ListRulesResponse, Rule, RuleAction,
        RuleActionConfiguration, RuleCondition, RuleVersion, UnarchiveRuleRequest,
        UpdateActionRequest, UpdateConditionRequest, UpdateRuleRequest, WebhookActionConfiguration,
        rule_action_configuration::Configuration, rule_service_client::RuleServiceClient,
    },
};

/// A partial set of changes to apply to an existing rule. Every field is
/// optional; `None` means "leave unchanged". This is what makes `update_rule`
/// safe: only the fields set here are touched, everything else on the rule is
/// preserved across the full-replace `UpdateRule` RPC.
#[derive(Debug, Default)]
pub struct RuleUpdate {
    pub name: Option<String>,
    pub description: Option<String>,
    pub version_notes: Option<String>,
    pub is_live_evaluation_enabled: Option<bool>,
    pub asset_ids: Option<Vec<String>>,
    pub tag_ids: Option<Vec<String>>,
    pub conditions: Option<Vec<UpdateConditionRequest>>,
    /// Replaces the set of webhooks the rule calls. `None` keeps the current
    /// set, including when `conditions` replaces the actions they sat on.
    pub webhook_ids: Option<Vec<String>>,
}

/// The outcome of `update_rule`.
#[derive(Debug)]
pub struct UpdatedRule {
    pub rule_id: String,
    /// Webhooks the rule called before the update and no longer calls.
    pub detached_webhook_ids: Vec<String>,
}

#[cfg(test)]
mod test;

#[derive(Clone)]
pub struct RuleService {
    channel: SiftChannel,
    policy: RetryPolicy,
}

impl RuleService {
    pub fn new(channel: SiftChannel, policy: RetryPolicy) -> Self {
        Self { channel, policy }
    }

    pub async fn list_rules(
        &self,
        filter: String,
        order_by: Option<String>,
        limit: Option<u32>,
    ) -> Result<common::Page<Rule>> {
        let (page_size, record_limit) = common::paging(limit);

        let mut page_token = String::new();
        let mut results = Vec::new();
        let mut has_more = false;

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
                    let mut client = RuleServiceClient::new(channel);
                    client
                        .list_rules(ListRulesRequest {
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
            .context("failed to query rules")?;

            let ListRulesResponse {
                rules,
                next_page_token,
            } = resp;
            if rules.is_empty() {
                break;
            }
            results.extend(rules);

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

    /// Creates a rule from a full rule definition that calls each of
    /// `webhook_ids` when it fires. Returns the new `rule_id`.
    pub async fn create_rule(
        &self,
        mut update: UpdateRuleRequest,
        webhook_ids: Vec<String>,
    ) -> Result<String> {
        set_webhooks(&mut update.conditions, &webhook_ids);

        let channel = self.channel.clone();

        let resp = with_retry(&self.policy, move || {
            let channel = channel.clone();
            let update = update.clone();
            async move {
                let mut client = RuleServiceClient::new(channel);
                client
                    .create_rule(CreateRuleRequest {
                        update: Some(update),
                    })
                    .await
                    .map(|resp| resp.into_inner())
            }
        })
        .await
        .context("failed to create rule")?;

        Ok(resp.rule_id)
    }

    /// Safely updates an existing rule by `rule_id`. The underlying `UpdateRule`
    /// RPC is a full replace, so this fetches the current rule, overlays only the
    /// fields set in `changes`, and sends the merged definition back. Fields not
    /// named in `changes` are preserved.
    pub async fn update_rule(&self, rule_id: String, changes: RuleUpdate) -> Result<UpdatedRule> {
        let current = self
            .get_rule(rule_id.clone())
            .await?
            .ok_or_else(|| anyhow::anyhow!("rule '{rule_id}' not found"))?;

        let mut update = rule_to_update_request(current);
        let current_webhook_ids = webhook_ids_in(&update.conditions);

        let RuleUpdate {
            name,
            description,
            version_notes,
            is_live_evaluation_enabled,
            asset_ids,
            tag_ids,
            conditions,
            webhook_ids,
        } = changes;

        if let Some(name) = name {
            update.name = name;
        }
        if let Some(description) = description {
            update.description = description;
        }
        if let Some(version_notes) = version_notes {
            update.version_notes = version_notes;
        }
        if let Some(live) = is_live_evaluation_enabled {
            update.is_live_evaluation_enabled = Some(live);
        }
        if asset_ids.is_some() || tag_ids.is_some() {
            let mut asset_configuration = update.asset_configuration.unwrap_or_default();
            if let Some(asset_ids) = asset_ids {
                asset_configuration.asset_ids = asset_ids;
            }
            if let Some(tag_ids) = tag_ids {
                asset_configuration.tag_ids = tag_ids;
            }
            update.asset_configuration = Some(asset_configuration);
        }
        if let Some(conditions) = conditions {
            update.conditions = conditions;
        }
        let webhook_ids = webhook_ids.unwrap_or_else(|| current_webhook_ids.clone());
        set_webhooks(&mut update.conditions, &webhook_ids);
        let detached_webhook_ids = current_webhook_ids
            .into_iter()
            .filter(|id| !webhook_ids.contains(id))
            .collect();

        let channel = self.channel.clone();
        let resp = with_retry(&self.policy, move || {
            let channel = channel.clone();
            let update = update.clone();
            async move {
                let mut client = RuleServiceClient::new(channel);
                client
                    .update_rule(update)
                    .await
                    .map(|resp| resp.into_inner())
            }
        })
        .await
        .context("failed to update rule")?;

        Ok(UpdatedRule {
            rule_id: resp.rule_id,
            detached_webhook_ids,
        })
    }

    /// Retrieves the latest version of a rule by id, or `None` if it does not exist.
    async fn get_rule(&self, rule_id: String) -> Result<Option<Rule>> {
        let channel = self.channel.clone();

        let resp = with_retry(&self.policy, move || {
            let channel = channel.clone();
            let rule_id = rule_id.clone();
            async move {
                let mut client = RuleServiceClient::new(channel);
                client
                    .get_rule(GetRuleRequest {
                        rule_id,
                        client_key: String::new(),
                    })
                    .await
                    .map(|resp| resp.into_inner())
            }
        })
        .await
        .context("failed to fetch rule")?;

        Ok(resp.rule)
    }

    /// Lists the version history for a single rule.
    pub async fn list_rule_versions(
        &self,
        rule_id: String,
        filter: String,
        limit: Option<u32>,
    ) -> Result<Vec<RuleVersion>> {
        let (page_size, record_limit) = common::paging(limit);

        let mut page_token = String::new();
        let mut results = Vec::new();

        loop {
            let channel = self.channel.clone();
            let rule_id = rule_id.clone();
            let filter = filter.clone();
            let token = page_token.clone();

            let resp = with_retry(&self.policy, move || {
                let channel = channel.clone();
                let rule_id = rule_id.clone();
                let filter = filter.clone();
                let token = token.clone();
                async move {
                    let mut client = RuleServiceClient::new(channel);
                    client
                        .list_rule_versions(ListRuleVersionsRequest {
                            rule_id,
                            page_size,
                            page_token: token,
                            filter,
                        })
                        .await
                        .map(|resp| resp.into_inner())
                }
            })
            .await
            .context("failed to query rule versions")?;

            let ListRuleVersionsResponse {
                rule_versions,
                next_page_token,
            } = resp;
            if rule_versions.is_empty() {
                break;
            }
            results.extend(rule_versions);

            if results.len() >= record_limit || next_page_token.is_empty() {
                break;
            }
            page_token = next_page_token;
        }

        results.truncate(record_limit);

        Ok(results)
    }

    /// Archives a rule, identified by either `rule_id` or `client_key` (the
    /// caller guarantees exactly one is non-empty).
    pub async fn archive_rule(&self, rule_id: String, client_key: String) -> Result<()> {
        let channel = self.channel.clone();

        with_retry(&self.policy, move || {
            let channel = channel.clone();
            let rule_id = rule_id.clone();
            let client_key = client_key.clone();
            async move {
                let mut client = RuleServiceClient::new(channel);
                client
                    .archive_rule(ArchiveRuleRequest {
                        rule_id,
                        client_key,
                    })
                    .await
                    .map(|resp| resp.into_inner())
            }
        })
        .await
        .context("failed to archive rule")?;

        Ok(())
    }

    /// Unarchives a rule, identified by either `rule_id` or `client_key` (the
    /// caller guarantees exactly one is non-empty).
    pub async fn unarchive_rule(&self, rule_id: String, client_key: String) -> Result<()> {
        let channel = self.channel.clone();

        with_retry(&self.policy, move || {
            let channel = channel.clone();
            let rule_id = rule_id.clone();
            let client_key = client_key.clone();
            async move {
                let mut client = RuleServiceClient::new(channel);
                client
                    .unarchive_rule(UnarchiveRuleRequest {
                        rule_id,
                        client_key,
                    })
                    .await
                    .map(|resp| resp.into_inner())
            }
        })
        .await
        .context("failed to unarchive rule")?;

        Ok(())
    }
}

/// Converts a fetched [`Rule`] into the [`UpdateRuleRequest`] shape so it can be
/// resent through the full-replace `UpdateRule` RPC. Every meaningful field is
/// carried over verbatim; this is the base that `update_rule` overlays changes
/// onto so unspecified fields survive the round-trip.
fn rule_to_update_request(rule: Rule) -> UpdateRuleRequest {
    UpdateRuleRequest {
        rule_id: Some(rule.rule_id),
        name: rule.name,
        description: rule.description,
        // `is_enabled` is deprecated in favor of archive / live-evaluation state,
        // both carried below; leave it at its default rather than touch the
        // deprecated field.
        conditions: rule
            .conditions
            .into_iter()
            .map(condition_to_update)
            .collect(),
        organization_id: rule.organization_id,
        client_key: (!rule.client_key.is_empty()).then_some(rule.client_key),
        asset_configuration: rule.asset_configuration,
        contextual_channels: rule.contextual_channels,
        is_external: rule.is_external,
        metadata: rule.metadata,
        is_archived: rule.is_archived,
        is_live_evaluation_enabled: Some(rule.is_live_evaluation_enabled),
        ..Default::default()
    }
}

fn condition_to_update(condition: RuleCondition) -> UpdateConditionRequest {
    UpdateConditionRequest {
        rule_condition_id: (!condition.rule_condition_id.is_empty())
            .then_some(condition.rule_condition_id),
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
        rule_action_id: (!action.rule_action_id.is_empty()).then_some(action.rule_action_id),
        action_type: action.action_type,
        configuration: action.configuration,
    }
}

/// Makes every condition call exactly `webhook_ids`. Sift fires a webhook from
/// the condition that holds it and allows one condition per rule, so every
/// condition is that one. An action for a kept webhook keeps its
/// `rule_action_id`, since Sift deletes any action an update leaves out, and a
/// repeated id is attached once because Sift rejects the rule otherwise.
/// Non-webhook actions are left alone.
fn set_webhooks(conditions: &mut [UpdateConditionRequest], webhook_ids: &[String]) {
    for condition in conditions {
        condition.actions.retain(|action| {
            webhook_id(action).is_none_or(|id| webhook_ids.iter().any(|w| w == id))
        });

        for id in webhook_ids {
            let attached = condition
                .actions
                .iter()
                .any(|action| webhook_id(action) == Some(id.as_str()));
            if !attached {
                condition.actions.push(webhook_action(id.clone()));
            }
        }
    }
}

/// Whether any condition carries a webhook action. The rule tools take
/// webhooks only through `webhook_ids`, so each one is checked before saving.
pub fn has_webhook_action(conditions: &[UpdateConditionRequest]) -> bool {
    conditions
        .iter()
        .flat_map(|condition| &condition.actions)
        .any(|action| webhook_id(action).is_some())
}

/// The webhooks the conditions call, in order, each once.
fn webhook_ids_in(conditions: &[UpdateConditionRequest]) -> Vec<String> {
    let mut ids: Vec<String> = Vec::new();
    for id in conditions
        .iter()
        .flat_map(|condition| &condition.actions)
        .filter_map(webhook_id)
    {
        if !ids.iter().any(|existing| existing == id) {
            ids.push(id.to_string());
        }
    }
    ids
}

/// The webhook an action calls, or `None` for any other kind of action.
fn webhook_id(action: &UpdateActionRequest) -> Option<&str> {
    match action.configuration.as_ref()?.configuration.as_ref()? {
        Configuration::Webhook(webhook) => Some(webhook.webhook_id.as_str()),
        _ => None,
    }
}

fn webhook_action(webhook_id: String) -> UpdateActionRequest {
    UpdateActionRequest {
        rule_action_id: None,
        action_type: ActionKind::Webhook as i32,
        configuration: Some(RuleActionConfiguration {
            configuration: Some(Configuration::Webhook(WebhookActionConfiguration {
                webhook_id,
            })),
        }),
    }
}
