from __future__ import annotations

from typing import Optional, cast

from sift.report_templates.v1.report_templates_pb2 import (
    GetReportTemplateRequest,
    GetReportTemplateResponse,
    ReportTemplate,
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

    def _get_report_template_by_client_key(self, client_key: str) -> Optional[ReportTemplate]:
        req = GetReportTemplateRequest(client_key=client_key)
        try:
            res = cast(GetReportTemplateResponse, self._report_template_service_stub.GetReportTemplate(req))
            return res.report_template or None
        except:
            return None

