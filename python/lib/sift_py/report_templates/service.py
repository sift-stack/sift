from __future__ import annotations

from typing import Optional, cast

from google.protobuf.field_mask_pb2 import FieldMask
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


class ReportTemplateService:
    _report_template_service_stub: ReportTemplateServiceStub

    def __init__(self, channel: SiftChannel):
        self._report_template_service_stub = ReportTemplateServiceStub(channel)

    def create_or_update_report_template(self, config: ReportTemplateConfig):
        if not config.template_client_key:
            raise Exception(f"Report template {config.name} requires a template_client_key")
        report_template = self._get_report_template_by_client_key(config.template_client_key)
        if report_template:
            self._update_report_template(config, report_template)
            return
        self._create_report_template(config)

    def get_report_template(self, client_key: str = "", id: str = "") -> Optional[ReportTemplateConfig]:
        report_template = None
        if not client_key and not id:
            raise ValueError("Either client_key or id must be provided")

        if id:
            report_template = self._get_report_template_by_id(id)
        elif client_key:
            report_template = self._get_report_template_by_client_key(client_key)

        if not report_template:
            raise Exception(f"Report template with client key {client_key} or id {id} not found.")

        return ReportTemplateConfig(
            name=report_template.name,
            template_client_key=report_template.client_key,
            organization_id=report_template.organization_id,
            tags=[tag.tag_name for tag in report_template.tags],
            description=report_template.description,
            rule_client_keys=[rule.client_key for rule in report_template.rules],
        )


    def _get_report_template_by_id(self, report_template_id: str) -> Optional[ReportTemplate]:
        req = GetReportTemplateRequest(report_template_id=report_template_id)
        try:
            res = cast(
                GetReportTemplateResponse, self._report_template_service_stub.GetReportTemplate(req)
            )
            return cast(ReportTemplate, res.report_template) or None
        except:
            return None

    def _get_report_template_by_client_key(self, client_key: str) -> Optional[ReportTemplate]:
        req = GetReportTemplateRequest(client_key=client_key)
        try:
            res = cast(
                GetReportTemplateResponse, self._report_template_service_stub.GetReportTemplate(req)
            )
            return res.report_template or None
        except:
            return None

    def _create_report_template(self, config: ReportTemplateConfig):
        client_keys_req = CreateReportTemplateRequestClientKeys(rule_client_keys=config.rule_client_keys)
        req = CreateReportTemplateRequest(
            name=config.name,
            client_key=config.template_client_key,
            description=config.description,
            tag_names=config.tags,
            organization_id=config.organization_id,
            rule_client_keys=client_keys_req,
        )
        self._report_template_service_stub.CreateReportTemplate(req)

    def _update_report_template(
        self, config: ReportTemplateConfig, report_template: ReportTemplate
    ):
        tags = []
        if config.tags:
            tags = [ReportTemplateTag(tag_name=tag) for tag in config.tags]

        rules = [ReportTemplateRule(client_key=client_key) for client_key in config.rule_client_keys]

        updated_report_template = ReportTemplate(
            report_template_id=report_template.report_template_id,
            organization_id=report_template.organization_id,
            client_key=report_template.client_key,
            name=config.name,
            description=config.description,
            tags=tags,
            rules=rules,
        )

        field_mask = FieldMask(paths=["name", "description", "tags", "rules"])
        self._report_template_service_stub.UpdateReportTemplate(
            UpdateReportTemplateRequest(
                report_template=updated_report_template, update_mask=field_mask
            )
        )