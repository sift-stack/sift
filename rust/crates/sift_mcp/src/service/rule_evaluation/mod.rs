use crate::policy::{RetryPolicy, with_retry};
use anyhow::{Context, Result};
use sift_rs::{
    SiftChannel,
    common::r#type::v1::{
        Ids, ResourceIdentifier, ResourceIdentifiers, resource_identifier, resource_identifiers,
    },
    rule_evaluation::v1::{
        EvaluateRulesFromCurrentRuleVersions, EvaluateRulesFromRuleConfigs,
        EvaluateRulesPreviewRequest, EvaluateRulesPreviewResponse, evaluate_rules_preview_request,
        rule_evaluation_service_client::RuleEvaluationServiceClient,
    },
    rules::v1::UpdateRuleRequest,
};

#[cfg(test)]
mod test;

/// Which rule to dry-run: a saved rule identified by id (already resolved from
/// a name upstream, if that's how the caller identified it), or an ad-hoc
/// draft definition that is never persisted anywhere.
pub enum PreviewRuleSource {
    SavedRuleId(String),
    DraftRuleConfig(Box<UpdateRuleRequest>),
}

#[derive(Clone)]
pub struct RuleEvaluationService {
    channel: SiftChannel,
    policy: RetryPolicy,
}

impl RuleEvaluationService {
    pub fn new(channel: SiftChannel, policy: RetryPolicy) -> Self {
        Self { channel, policy }
    }

    /// Dry-runs rule evaluation against a run via `EvaluateRulesPreview`. Nothing
    /// is persisted by this RPC: no report, annotation, or rule version is
    /// created. Returns the count of annotations that would be created and their
    /// would-be details.
    pub async fn preview_rule(
        &self,
        run_id: String,
        source: PreviewRuleSource,
        organization_id: Option<String>,
    ) -> Result<EvaluateRulesPreviewResponse> {
        let mode = match source {
            PreviewRuleSource::SavedRuleId(rule_id) => {
                evaluate_rules_preview_request::Mode::Rules(EvaluateRulesFromCurrentRuleVersions {
                    rules: Some(ResourceIdentifiers {
                        identifiers: Some(resource_identifiers::Identifiers::Ids(Ids {
                            ids: vec![rule_id],
                        })),
                    }),
                })
            }
            PreviewRuleSource::DraftRuleConfig(config) => {
                evaluate_rules_preview_request::Mode::RuleConfigs(EvaluateRulesFromRuleConfigs {
                    configs: vec![*config],
                })
            }
        };

        let request = EvaluateRulesPreviewRequest {
            organization_id: organization_id.unwrap_or_default(),
            time: Some(evaluate_rules_preview_request::Time::Run(
                ResourceIdentifier {
                    identifier: Some(resource_identifier::Identifier::Id(run_id)),
                },
            )),
            mode: Some(mode),
            ..Default::default()
        };

        let channel = self.channel.clone();
        with_retry(&self.policy, move || {
            let channel = channel.clone();
            let request = request.clone();
            async move {
                let mut client = RuleEvaluationServiceClient::new(channel);
                client
                    .evaluate_rules_preview(request)
                    .await
                    .map(|resp| resp.into_inner())
            }
        })
        .await
        .context("failed to preview rule evaluation")
    }
}
