use crate::policy::{RetryPolicy, with_retry};
use crate::service::common;
use anyhow::{Context, Result, anyhow};
use pbjson_types::FieldMask;
use sift_rs::{
    SiftChannel,
    common::r#type::v1::{
        ClientKeys, Ids, ResourceIdentifier, ResourceIdentifiers, resource_identifier,
        resource_identifiers,
    },
    metadata::v1::MetadataValue,
    reports::v1::{
        GetReportRequest, ListReportRuleSummariesRequest, ListReportRuleSummariesResponse,
        ListReportsRequest, ListReportsResponse, Report, ReportRuleSummary, UpdateReportRequest,
        report_service_client::ReportServiceClient,
    },
    rule_evaluation::v1::{
        EvaluateRulesFromCurrentRuleVersions, EvaluateRulesFromReportTemplate,
        EvaluateRulesFromRuleVersions, EvaluateRulesRequest, evaluate_rules_request,
        rule_evaluation_service_client::RuleEvaluationServiceClient,
    },
};

#[cfg(test)]
mod test;

#[allow(clippy::enum_variant_names)]
pub enum RuleIdentifier {
    RuleIds(Vec<String>),
    RuleClientKeys(Vec<String>),
    RuleVersionIds(Vec<String>),
}

pub enum ReportSource {
    Template { report_template_id: String },
    Rules { rules: RuleIdentifier },
}

#[derive(Debug)]
pub struct CreateReportOutput {
    pub report: Report,
    pub job_id: Option<String>,
    pub created_annotation_count: i32,
}

#[derive(Clone)]
pub struct ReportService {
    channel: SiftChannel,
    policy: RetryPolicy,
}

impl ReportService {
    pub fn new(channel: SiftChannel, policy: RetryPolicy) -> Self {
        Self { channel, policy }
    }

    pub async fn list_reports(
        &self,
        filter: String,
        order_by: Option<String>,
        limit: Option<u32>,
        organization_id: Option<String>,
    ) -> Result<Vec<Report>> {
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
                    let mut client = ReportServiceClient::new(channel);
                    client
                        .list_reports(ListReportsRequest {
                            filter,
                            page_size,
                            page_token: token,
                            order_by,
                            organization_id,
                        })
                        .await
                        .map(|resp| resp.into_inner())
                }
            })
            .await
            .context("failed to query reports")?;

            let ListReportsResponse {
                reports,
                next_page_token,
            } = resp;
            if reports.is_empty() {
                break;
            }
            results.extend(reports);

            if results.len() >= record_limit || next_page_token.is_empty() {
                break;
            }
            page_token = next_page_token;
        }

        results.truncate(record_limit);

        Ok(results)
    }

    pub async fn list_report_rule_summaries(
        &self,
        report_id: String,
        filter: String,
        order_by: Option<String>,
        limit: Option<u32>,
    ) -> Result<Vec<ReportRuleSummary>> {
        let (page_size, record_limit) = common::paging(limit);

        let mut page_token = String::new();
        let mut results = Vec::new();

        let order_by = order_by.unwrap_or_default();

        loop {
            let channel = self.channel.clone();
            let report_id = report_id.clone();
            let filter = filter.clone();
            let order_by = order_by.clone();
            let token = page_token.clone();

            let resp = with_retry(&self.policy, move || {
                let channel = channel.clone();
                let report_id = report_id.clone();
                let filter = filter.clone();
                let order_by = order_by.clone();
                let token = token.clone();
                async move {
                    let mut client = ReportServiceClient::new(channel);
                    client
                        .list_report_rule_summaries(ListReportRuleSummariesRequest {
                            report_id,
                            page_size,
                            page_token: token,
                            filter,
                            order_by,
                        })
                        .await
                        .map(|resp| resp.into_inner())
                }
            })
            .await
            .context("failed to query report rule summaries")?;

            let ListReportRuleSummariesResponse {
                report_rule_summaries,
                next_page_token,
            } = resp;
            if report_rule_summaries.is_empty() {
                break;
            }
            results.extend(report_rule_summaries);

            if results.len() >= record_limit || next_page_token.is_empty() {
                break;
            }
            page_token = next_page_token;
        }

        results.truncate(record_limit);

        Ok(results)
    }

    pub async fn create_report(
        &self,
        organization_id: Option<String>,
        run_id: String,
        name: String,
        description: Option<String>,
        metadata: Vec<MetadataValue>,
        source: ReportSource,
    ) -> Result<CreateReportOutput> {
        let mode = match source {
            ReportSource::Template { report_template_id } => {
                evaluate_rules_request::Mode::ReportTemplate(EvaluateRulesFromReportTemplate {
                    report_template: Some(ResourceIdentifier {
                        identifier: Some(resource_identifier::Identifier::Id(report_template_id)),
                    }),
                })
            }
            ReportSource::Rules { rules } => match rules {
                RuleIdentifier::RuleIds(rule_ids) => {
                    evaluate_rules_request::Mode::Rules(EvaluateRulesFromCurrentRuleVersions {
                        rules: Some(ResourceIdentifiers {
                            identifiers: Some(resource_identifiers::Identifiers::Ids(Ids {
                                ids: rule_ids,
                            })),
                        }),
                    })
                }
                RuleIdentifier::RuleClientKeys(rule_client_keys) => {
                    evaluate_rules_request::Mode::Rules(EvaluateRulesFromCurrentRuleVersions {
                        rules: Some(ResourceIdentifiers {
                            identifiers: Some(resource_identifiers::Identifiers::ClientKeys(
                                ClientKeys {
                                    client_keys: rule_client_keys,
                                },
                            )),
                        }),
                    })
                }
                RuleIdentifier::RuleVersionIds(rule_version_ids) => {
                    evaluate_rules_request::Mode::RuleVersions(EvaluateRulesFromRuleVersions {
                        rule_version_ids,
                    })
                }
            },
        };

        let evaluate_request = EvaluateRulesRequest {
            organization_id: organization_id.unwrap_or_default(),
            report_name: Some(name),
            time: Some(evaluate_rules_request::Time::Run(ResourceIdentifier {
                identifier: Some(resource_identifier::Identifier::Id(run_id)),
            })),
            mode: Some(mode),
            ..Default::default()
        };

        let channel = self.channel.clone();
        let resp = with_retry(&self.policy, move || {
            let channel = channel.clone();
            let request = evaluate_request.clone();
            async move {
                let mut client = RuleEvaluationServiceClient::new(channel);
                client
                    .evaluate_rules(request)
                    .await
                    .map(|resp| resp.into_inner())
            }
        })
        .await
        .context("failed to create report")?;

        let report_id = resp
            .report_id
            .clone()
            .ok_or_else(|| anyhow!("create_report response missing report_id"))?;
        let job_id = resp.job_id.clone();
        let created_annotation_count = resp.created_annotation_count;

        let metadata_update = (!metadata.is_empty()).then_some(metadata);
        if description.is_some() || metadata_update.is_some() {
            self.update_report_fields(report_id.clone(), description, metadata_update)
                .await?;
        }

        let report = self.get_report(report_id).await?;

        Ok(CreateReportOutput {
            report,
            job_id,
            created_annotation_count,
        })
    }

    pub async fn update_report(
        &self,
        report_id: String,
        metadata: Vec<MetadataValue>,
    ) -> Result<Report> {
        self.update_report_fields(report_id.clone(), None, Some(metadata))
            .await?;
        self.get_report(report_id).await
    }

    async fn update_report_fields(
        &self,
        report_id: String,
        description: Option<String>,
        metadata: Option<Vec<MetadataValue>>,
    ) -> Result<()> {
        let mut report = Report {
            report_id,
            ..Default::default()
        };
        let mut paths = Vec::new();

        if let Some(description) = description {
            report.description = Some(description);
            paths.push("description".to_string());
        }
        if let Some(metadata) = metadata {
            report.metadata = metadata;
            paths.push("metadata".to_string());
        }

        let channel = self.channel.clone();
        with_retry(&self.policy, move || {
            let channel = channel.clone();
            let report = report.clone();
            let paths = paths.clone();
            async move {
                let mut client = ReportServiceClient::new(channel);
                client
                    .update_report(UpdateReportRequest {
                        report: Some(report),
                        update_mask: Some(FieldMask { paths }),
                    })
                    .await
                    .map(|resp| resp.into_inner())
            }
        })
        .await
        .context("failed to update report")?;

        Ok(())
    }

    async fn get_report(&self, report_id: String) -> Result<Report> {
        let channel = self.channel.clone();
        let resp = with_retry(&self.policy, move || {
            let channel = channel.clone();
            let report_id = report_id.clone();
            async move {
                let mut client = ReportServiceClient::new(channel);
                client
                    .get_report(GetReportRequest { report_id })
                    .await
                    .map(|resp| resp.into_inner())
            }
        })
        .await
        .context("failed to fetch report")?;

        resp.report
            .ok_or_else(|| anyhow!("get_report response missing report"))
    }
}
