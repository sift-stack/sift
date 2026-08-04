use crate::policy::{RetryPolicy, with_retry};
use crate::service::common;
use anyhow::{Context, Result, anyhow};
use pbjson_types::FieldMask;
use sift_rs::{
    SiftChannel,
    metadata::v1::MetadataValue,
    report_templates::v1::{
        CreateReportTemplateRequest, CreateReportTemplateRequestClientKeys,
        CreateReportTemplateRequestRuleIds, ListReportTemplatesRequest, ListReportTemplatesResponse,
        ReportTemplate, ReportTemplateRule, ReportTemplateTag, UpdateReportTemplateRequest,
        create_report_template_request,
        report_template_service_client::ReportTemplateServiceClient,
    },
};

#[cfg(test)]
mod test;

/// How the rules on a report template are identified. Exactly one variant is
/// constructed from the flat tool params. Variant names mirror the proto
/// `rule_identifiers` oneof fields on `CreateReportTemplateRequest`.
#[allow(clippy::enum_variant_names)]
pub enum TemplateRuleIdentifier {
    RuleIds(Vec<String>),
    RuleClientKeys(Vec<String>),
}

/// A partial set of changes to apply to an existing report template. Every
/// field is optional; `None` means "leave unchanged" (i.e. absent from the
/// `update_mask`). `Some(_)` names the field in the mask, which means server-
/// side REPLACE semantics on collections (`tags`, `rules`, `metadata`).
#[derive(Default)]
pub struct ReportTemplateUpdate {
    pub name: Option<String>,
    pub description: Option<String>,
    pub tag_names: Option<Vec<String>>,
    pub rules: Option<TemplateRuleIdentifier>,
    pub metadata: Option<Vec<MetadataValue>>,
    pub is_archived: Option<bool>,
}

#[derive(Clone)]
pub struct ReportTemplateService {
    channel: SiftChannel,
    policy: RetryPolicy,
}

impl ReportTemplateService {
    pub fn new(channel: SiftChannel, policy: RetryPolicy) -> Self {
        Self { channel, policy }
    }

    pub async fn list_report_templates(
        &self,
        filter: String,
        order_by: Option<String>,
        limit: Option<u32>,
        organization_id: Option<String>,
    ) -> Result<Vec<ReportTemplate>> {
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
                    let mut client = ReportTemplateServiceClient::new(channel);
                    client
                        .list_report_templates(ListReportTemplatesRequest {
                            page_size,
                            page_token: token,
                            filter,
                            organization_id,
                            order_by,
                            ..Default::default()
                        })
                        .await
                        .map(|resp| resp.into_inner())
                }
            })
            .await
            .context("failed to query report templates")?;

            let ListReportTemplatesResponse {
                report_templates,
                next_page_token,
            } = resp;
            if report_templates.is_empty() {
                break;
            }
            results.extend(report_templates);

            if results.len() >= record_limit || next_page_token.is_empty() {
                break;
            }
            page_token = next_page_token;
        }

        results.truncate(record_limit);

        Ok(results)
    }

    pub async fn create_report_template(
        &self,
        organization_id: Option<String>,
        name: String,
        client_key: Option<String>,
        description: Option<String>,
        tag_names: Vec<String>,
        rules: TemplateRuleIdentifier,
        metadata: Vec<MetadataValue>,
    ) -> Result<ReportTemplate> {
        let rule_identifiers = match rules {
            TemplateRuleIdentifier::RuleIds(rule_ids) => {
                create_report_template_request::RuleIdentifiers::RuleIds(
                    CreateReportTemplateRequestRuleIds { rule_ids },
                )
            }
            TemplateRuleIdentifier::RuleClientKeys(rule_client_keys) => {
                create_report_template_request::RuleIdentifiers::RuleClientKeys(
                    CreateReportTemplateRequestClientKeys { rule_client_keys },
                )
            }
        };

        let request = CreateReportTemplateRequest {
            name,
            client_key,
            description,
            tag_names,
            organization_id: organization_id.unwrap_or_default(),
            rule_identifiers: Some(rule_identifiers),
            metadata,
        };

        let channel = self.channel.clone();
        let resp = with_retry(&self.policy, move || {
            let channel = channel.clone();
            let request = request.clone();
            async move {
                let mut client = ReportTemplateServiceClient::new(channel);
                client
                    .create_report_template(request)
                    .await
                    .map(|resp| resp.into_inner())
            }
        })
        .await
        .context("failed to create report template")?;

        resp.report_template
            .ok_or_else(|| anyhow!("create_report_template response missing report_template"))
    }

    /// Update selected fields on a report template. Per
    /// `protos/sift/report_templates/v1/report_templates.proto::UpdateReportTemplateRequest`
    /// the updatable fields are `name`, `description`, `tags`, `rules`,
    /// `metadata`, `is_archived`, and `archived_date`; this service exposes all
    /// but `archived_date` (server-managed via `is_archived`).
    ///
    /// Collections use REPLACE semantics when named in the mask: `tags`,
    /// `rules`, and `metadata` overwrite the current value on the template.
    pub async fn update_report_template(
        &self,
        report_template_id: String,
        changes: ReportTemplateUpdate,
    ) -> Result<ReportTemplate> {
        let ReportTemplateUpdate {
            name,
            description,
            tag_names,
            rules,
            metadata,
            is_archived,
        } = changes;

        let mut template = ReportTemplate {
            report_template_id,
            ..Default::default()
        };
        let mut paths = Vec::new();

        if let Some(name) = name {
            template.name = name;
            paths.push("name".to_string());
        }
        if let Some(description) = description {
            template.description = Some(description);
            paths.push("description".to_string());
        }
        if let Some(tag_names) = tag_names {
            template.tags = tag_names
                .into_iter()
                .map(|tag_name| ReportTemplateTag { tag_name })
                .collect();
            paths.push("tags".to_string());
        }
        if let Some(rules) = rules {
            template.rules = template_rules_from_identifier(rules);
            paths.push("rules".to_string());
        }
        if let Some(metadata) = metadata {
            template.metadata = metadata;
            paths.push("metadata".to_string());
        }
        if let Some(is_archived) = is_archived {
            template.is_archived = is_archived;
            paths.push("is_archived".to_string());
        }

        let channel = self.channel.clone();
        let resp = with_retry(&self.policy, move || {
            let channel = channel.clone();
            let template = template.clone();
            let paths = paths.clone();
            async move {
                let mut client = ReportTemplateServiceClient::new(channel);
                client
                    .update_report_template(UpdateReportTemplateRequest {
                        report_template: Some(template),
                        update_mask: Some(FieldMask { paths }),
                    })
                    .await
                    .map(|resp| resp.into_inner())
            }
        })
        .await
        .context("failed to update report template")?;

        resp.report_template
            .ok_or_else(|| anyhow!("update_report_template response missing report_template"))
    }
}

/// Build the write-side `rules` slice from a caller-supplied identifier list.
/// Only one field on each `ReportTemplateRule` is set (per the proto comment on
/// `UpdateReportTemplateRequest`: "only the rule ID or the rule client key is
/// required"). Slice order becomes `display_order` server-side when
/// `display_order` is omitted.
fn template_rules_from_identifier(rules: TemplateRuleIdentifier) -> Vec<ReportTemplateRule> {
    match rules {
        TemplateRuleIdentifier::RuleIds(rule_ids) => rule_ids
            .into_iter()
            .map(|rule_id| ReportTemplateRule {
                rule_id,
                ..Default::default()
            })
            .collect(),
        TemplateRuleIdentifier::RuleClientKeys(client_keys) => client_keys
            .into_iter()
            .map(|client_key| ReportTemplateRule {
                client_key,
                ..Default::default()
            })
            .collect(),
    }
}
