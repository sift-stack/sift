from __future__ import annotations

from datetime import datetime
from pathlib import Path
from typing import Any, Dict, List, Optional, cast

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

from sift_py._internal.time import to_timestamp_pb
from sift_py.grpc.transport import SiftChannel
from sift_py.report_templates.config import ReportTemplateConfig, ReportTemplateUpdate
from sift_py.yaml.report_templates import load_report_templates


class ReportTemplateService:
    """
    A service for managing report templates. Allows for creating, updating, and retrieving report
    templates.
    """

    _report_template_service_stub: ReportTemplateServiceStub

    def __init__(self, channel: SiftChannel):
        self._report_template_service_stub = ReportTemplateServiceStub(channel)

    def create_or_update_report_template(
        self, config: ReportTemplateConfig, updates: Optional[ReportTemplateUpdate] = None
    ):
        """
        Create or update a report template via a ReportTemplateConfig. The config must contain a
        template_client_key, otherwise an exception will be raised. If a report template with the
        same client key exists, it will be updated. Otherwise, a new report template will be created.
        See `sift_py.report_templates.config.ReportTemplateConfig` for more information on available
        fields to configure.
        """
        if updates:
            if not config.template_client_key and not config.template_id:
                raise Exception(
                    f"Report template {config.name} requires either a template_client_key or report_template_id to update."
                )
            self._update_report_template(config, updates)
        else:
            self._create_report_template(config)

    def get_report_template(
        self, client_key: Optional[str] = None, id: Optional[str] = None
    ) -> Optional[ReportTemplateConfig]:
        """
        Retrieve a report template by client key or id. Only one of client_key or id should be
        provided, otherwise an exception will be raised. If a report template is found, it will be
        returned as a ReportTemplateConfig object.
        """
        report_template = None
        if (not client_key and not id) or (client_key and id):
            raise ValueError("One of client_key or id must be provided")

        if id:
            report_template = self._get_report_template_by_id(id)
        elif client_key:
            report_template = self._get_report_template_by_client_key(client_key)

        return (
            ReportTemplateConfig(
                name=report_template.name,
                template_id=report_template.report_template_id,
                template_client_key=report_template.client_key,
                tags=[tag.tag_name for tag in report_template.tags],
                description=report_template.description,
                rule_client_keys=[rule.client_key for rule in report_template.rules],
                archived_date=report_template.archived_date.ToDatetime()
                if report_template.archived_date
                else None,
                archived=True if report_template.archived_date else False,
            )
            if report_template
            else None
        )

    def load_report_templates_from_yaml(self, paths: List[Path]) -> List[ReportTemplateConfig]:
        """
        Load report templates from YAML definitions. The YAML defined report template must have
        a client key. If the report template with the given client key exists, it will be updated,
        otherwise a new report template will be created.
        See `sift_py.yaml.report_templates.load_report_templates` for more information on the YAML
        spec for report templates.
        """
        report_templates = load_report_templates(paths)
        [
            self.create_or_update_report_template(report_template)
            for report_template in report_templates
        ]
        return report_templates

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
        client_keys_req = CreateReportTemplateRequestClientKeys(
            rule_client_keys=config.rule_client_keys
        )
        req = CreateReportTemplateRequest(
            name=config.name,
            client_key=config.template_client_key,
            description=config.description,
            tag_names=config.tags,
            rule_client_keys=client_keys_req,
        )
        self._report_template_service_stub.CreateReportTemplate(req)

    def _update_report_template(self, config: ReportTemplateConfig, updates: ReportTemplateUpdate):
        """
        Updates a report template using the provided updates. Only the fields specified in the updates
        will be modified.
        """
        update_map: Dict[str, Any] = {}
        if "name" in updates:
            update_map["name"] = updates["name"]
        if "description" in updates:
            update_map["description"] = updates["description"]
        if "tags" in updates:
            update_map["tags"] = [ReportTemplateTag(tag_name=tag) for tag in updates["tags"]]
        if "rule_client_keys" in updates:
            update_map["rules"] = [
                ReportTemplateRule(client_key=client_key)
                for client_key in updates["rule_client_keys"]
            ]
        if "archived" in updates:
            update_map["archived_date"] = (
                to_timestamp_pb(datetime.now()) if updates["archived"] else None
            )

        updated_report_template = ReportTemplate(
            report_template_id=config.template_id or "",
            client_key=config.template_client_key,
            name=update_map.get("name", config.name),
            description=update_map.get("description", config.description),
            tags=update_map.get("tags", config.tags),
            rules=update_map.get(
                "rules", [ReportTemplateRule(client_key=key) for key in config.rule_client_keys]
            ),
            archived_date=update_map.get("archived_date", config.archived_date),
        )

        field_mask = FieldMask(paths=list(update_map.keys()))
        self._report_template_service_stub.UpdateReportTemplate(
            UpdateReportTemplateRequest(
                report_template=updated_report_template, update_mask=field_mask
            )
        )
