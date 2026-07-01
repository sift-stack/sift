"""Tests for the report templates low-level wrapper."""

from unittest.mock import AsyncMock, MagicMock

import pytest
from sift.report_templates.v1 import report_templates_pb2 as rt

from sift_client._internal.low_level_wrappers.report_templates import (
    ReportTemplatesLowLevelClient,
)
from sift_client.sift_types.report_template import ReportTemplateCreate, ReportTemplateUpdate


def _client_with_stub(stub: MagicMock) -> ReportTemplatesLowLevelClient:
    grpc = MagicMock()
    grpc.get_stub.return_value = stub
    return ReportTemplatesLowLevelClient(grpc)


def _template_proto(report_template_id: str = "template-1") -> rt.ReportTemplate:
    proto = rt.ReportTemplate(
        report_template_id=report_template_id, organization_id="org-1", name="template"
    )
    proto.created_date.GetCurrentTime()
    proto.modified_date.GetCurrentTime()
    return proto


class TestGetReportTemplate:
    @pytest.mark.asyncio
    async def test_get_by_id_or_client_key(self):
        stub = MagicMock()
        stub.GetReportTemplate = AsyncMock(
            return_value=rt.GetReportTemplateResponse(report_template=_template_proto())
        )
        client = _client_with_stub(stub)

        template = await client.get_report_template(report_template_id="template-1")

        request = stub.GetReportTemplate.call_args[0][0]
        assert request.report_template_id == "template-1"
        assert request.client_key == ""
        assert template.id_ == "template-1"

        await client.get_report_template(client_key="template-key", organization_id="org-1")

        request = stub.GetReportTemplate.call_args[0][0]
        assert request.report_template_id == ""
        assert request.client_key == "template-key"
        assert request.organization_id == "org-1"

    @pytest.mark.asyncio
    async def test_raises_when_no_identifier_provided(self):
        stub = MagicMock()
        stub.GetReportTemplate = AsyncMock()
        client = _client_with_stub(stub)

        with pytest.raises(ValueError, match="report_template_id or client_key"):
            await client.get_report_template()

        stub.GetReportTemplate.assert_not_awaited()


class TestCreateReportTemplate:
    @pytest.mark.asyncio
    async def test_sends_create_proto_and_unwraps_response(self):
        stub = MagicMock()
        stub.CreateReportTemplate = AsyncMock(
            return_value=rt.CreateReportTemplateResponse(report_template=_template_proto())
        )
        client = _client_with_stub(stub)

        template = await client.create_report_template(
            create=ReportTemplateCreate(name="template", rule_ids=["rule-1"])
        )

        request = stub.CreateReportTemplate.call_args[0][0]
        assert request.name == "template"
        assert list(request.rule_ids.rule_ids) == ["rule-1"]
        assert template.id_ == "template-1"


class TestUpdateReportTemplate:
    @pytest.mark.asyncio
    async def test_sends_update_mask_and_unwraps_response(self):
        stub = MagicMock()
        stub.UpdateReportTemplate = AsyncMock(
            return_value=rt.UpdateReportTemplateResponse(report_template=_template_proto())
        )
        client = _client_with_stub(stub)

        update = ReportTemplateUpdate(name="renamed", tags=["tag1"])
        update.resource_id = "template-1"
        template = await client.update_report_template(update=update)

        request = stub.UpdateReportTemplate.call_args[0][0]
        assert request.report_template.report_template_id == "template-1"
        assert request.report_template.name == "renamed"
        assert [tag.tag_name for tag in request.report_template.tags] == ["tag1"]
        assert sorted(request.update_mask.paths) == ["name", "tags"]
        assert template.id_ == "template-1"


class TestListAllReportTemplates:
    @pytest.mark.asyncio
    async def test_paginates_and_passes_request_fields(self):
        stub = MagicMock()
        stub.ListReportTemplates = AsyncMock(
            side_effect=[
                rt.ListReportTemplatesResponse(
                    report_templates=[_template_proto("template-1")], next_page_token="token"
                ),
                rt.ListReportTemplatesResponse(
                    report_templates=[_template_proto("template-2")], next_page_token=""
                ),
            ]
        )
        client = _client_with_stub(stub)

        templates = await client.list_all_report_templates(
            query_filter='name == "template"',
            organization_id="org-1",
            order_by="created_date desc",
        )

        assert [template.id_ for template in templates] == ["template-1", "template-2"]
        assert stub.ListReportTemplates.call_count == 2
        request = stub.ListReportTemplates.call_args_list[0][0][0]
        assert request.filter == 'name == "template"'
        assert request.organization_id == "org-1"
        assert request.order_by == "created_date desc"
