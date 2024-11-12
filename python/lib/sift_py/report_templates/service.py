from __future__ import annotations

from typing import Optional, cast

from sift.report_templates.v1.report_templates_pb2 import (
    CreateReportTemplateRequest,
    CreateReportTemplateRequestClientKeys,
    GetReportTemplateRequest,
    GetReportTemplateResponse,
    ReportTemplate,
    ReportTemplateRule,
    ReportTemplateTag,
    UpdateReportTemplateRequest,
)
from sift.report_templates.v1.report_templates_pb2_grpc import ReportTemplateServiceStub

from sift_py.grpc.transport import SiftChannel
from sift_py.report_templates.config import ReportTemplateConfig


class ReportTemplateService():
    _report_template_service_stub: ReportTemplateServiceStub

    def __init__(self, channel: SiftChannel):
        self._report_template_service_stub = ReportTemplateServiceStub(channel)

    def create_or_update_report_template(self, config: ReportTemplateConfig):
        if not config.template_client_key:
            raise Exception(f"Report template {config.name} requires a template_client_key")
        if self._get_report_template_by_client_key(config.template_client_key):
            self._update_report_template(config)
            return
        self._create_report_template(config)

    def get_report_template(self, client_key: str = "", report_template_id: str = "") -> Optional[ReportTemplate]:
        if client_key:
            return self._get_report_template_by_client_key(client_key)
        if report_template_id:
            return self._get_report_template_by_id(report_template_id)
        raise ValueError("Either client_key or report_template_id must be provided")

    def _get_report_template_by_id(self, report_template_id: str) -> Optional[ReportTemplate]:
        req = GetReportTemplateRequest(report_template_id=report_template_id)
        try:
            res = cast(GetReportTemplateResponse, self._report_template_service_stub.GetReportTemplate(req))
            return res.report_template or None
        except:
            return None

    def _get_report_template_by_client_key(self, client_key: str) -> Optional[ReportTemplate]:
        req = GetReportTemplateRequest(client_key=client_key)
        try:
            res = cast(GetReportTemplateResponse, self._report_template_service_stub.GetReportTemplate(req))
            return res.report_template or None
        except:
            return None

    def _create_report_template(self, config: ReportTemplateConfig):
        rule_client_keys = self._get_rule_client_keys(config)
        client_keys_req = CreateReportTemplateRequestClientKeys(rule_client_keys=rule_client_keys)
        req = CreateReportTemplateRequest(
            name=config.name,
            client_key=config.template_client_key,
            description=config.description,
            tag_names=config.tags,
            organization_id=config.organization_id,
            rule_client_keys=client_keys_req,
        )
        self._report_template_service_stub.CreateReportTemplate(req)

    def _update_report_template(self, config: ReportTemplateConfig):
        tags = []
        if config.tags:
            tags = [ReportTemplateTag(tag_name=tag) for tag in config.tags]

        rule_client_keys = self._get_rule_client_keys(config)
        rules = [ReportTemplateRule(client_key=client_key) for client_key in rule_client_keys]

        report_template = ReportTemplate(
            name=config.name,
            client_key=config.template_client_key,
            description=config.description,
            tags=tags,
            organization_id=config.organization_id,
            rules=rules,
        )
        self._report_template_service_stub.UpdateReportTemplate(UpdateReportTemplateRequest(report_template=report_template))

    def _get_rule_client_keys(self, config: ReportTemplateConfig) -> list[str]:
        client_keys = []
        for rule in config.rules:
            client_key = rule.rule_client_key
            if not client_key:
                raise Exception(f"Rule {rule.name} requires a rule_client_key")
            client_keys.append(client_key)

        return client_keys
