use crate::policy::{RetryPolicy, with_retry};
use crate::service::common;
use anyhow::{Context, Result, anyhow};
use pbjson_types::FieldMask;
use sift_rs::{
    SiftChannel,
    metadata::v1::MetadataValue,
    reports::v1::{
        CreateReportFromReportTemplateRequest, CreateReportFromRulesRequest, CreateReportRequest,
        CreateReportRequestClientKeys, CreateReportRequestRuleIds,
        CreateReportRequestRuleVersionIds, GetReportRequest, ListReportRuleSummariesRequest,
        ListReportRuleSummariesResponse, ListReportsRequest, ListReportsResponse, Report,
        ReportRuleSummary, UpdateReportRequest, create_report_from_rules_request,
        create_report_request, report_service_client::ReportServiceClient,
    },
};

#[cfg(test)]
mod test;

/// How the set of rules a report is built from is identified. Exactly one variant
/// is constructed from the flat tool params. Variant names mirror the proto
/// `rule_identifiers` oneof fields.
#[allow(clippy::enum_variant_names)]
pub enum RuleIdentifier {
    RuleIds(Vec<String>),
    RuleClientKeys(Vec<String>),
    RuleVersionIds(Vec<String>),
}

/// The source a report is created from. Flattens the `CreateReportRequest`
/// oneof into a typed choice built in the tool handler.
pub enum ReportSource {
    Template {
        report_template_id: String,
    },
    Rules {
        description: Option<String>,
        tag_names: Vec<String>,
        rules: RuleIdentifier,
    },
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
        metadata: Option<Vec<MetadataValue>>,
        source: ReportSource,
    ) -> Result<Report> {
        let request = match source {
            ReportSource::Template { report_template_id } => {
                create_report_request::Request::ReportFromReportTemplateRequest(
                    CreateReportFromReportTemplateRequest { report_template_id },
                )
            }
            ReportSource::Rules {
                description,
                tag_names,
                rules,
            } => {
                let rule_identifiers = match rules {
                    RuleIdentifier::RuleIds(rule_ids) => {
                        create_report_from_rules_request::RuleIdentifiers::RuleIds(
                            CreateReportRequestRuleIds { rule_ids },
                        )
                    }
                    RuleIdentifier::RuleClientKeys(rule_client_keys) => {
                        create_report_from_rules_request::RuleIdentifiers::RuleClientKeys(
                            CreateReportRequestClientKeys { rule_client_keys },
                        )
                    }
                    RuleIdentifier::RuleVersionIds(rule_version_ids) => {
                        create_report_from_rules_request::RuleIdentifiers::RuleVersionIds(
                            CreateReportRequestRuleVersionIds { rule_version_ids },
                        )
                    }
                };
                create_report_request::Request::ReportFromRulesRequest(
                    CreateReportFromRulesRequest {
                        name: name.clone(),
                        description,
                        tag_names,
                        rule_identifiers: Some(rule_identifiers),
                    },
                )
            }
        };

        let create_request = CreateReportRequest {
            organization_id: organization_id.unwrap_or_default(),
            run_id,
            name: Some(name),
            metadata: metadata.unwrap_or_default(),
            request: Some(request),
        };

        let channel = self.channel.clone();
        let resp = with_retry(&self.policy, move || {
            let channel = channel.clone();
            let create_request = create_request.clone();
            async move {
                let mut client = ReportServiceClient::new(channel);
                client
                    .create_report(create_request)
                    .await
                    .map(|resp| resp.into_inner())
            }
        })
        .await
        .context("failed to create report")?;

        resp.report
            .ok_or_else(|| anyhow!("create_report response missing report"))
    }

    /// Update an existing report's metadata. Per
    /// `protos/sift/reports/v1/reports.proto::UpdateReportRequest` the updatable
    /// fields are `archived_date`, `is_archived`, and `metadata`; this service
    /// exposes `metadata` only (archive flow is out of scope). `metadata` uses
    /// REPLACE semantics.
    ///
    /// `UpdateReportResponse` is empty, so the updated `Report` is re-fetched via
    /// `GetReport` and returned.
    pub async fn update_report(
        &self,
        report_id: String,
        metadata: Vec<MetadataValue>,
    ) -> Result<Report> {
        let report = Report {
            report_id: report_id.clone(),
            metadata,
            ..Default::default()
        };
        let paths = vec!["metadata".to_string()];

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
        .context("failed to fetch report after update")?;

        resp.report
            .ok_or_else(|| anyhow!("get_report response missing report"))
    }
}
