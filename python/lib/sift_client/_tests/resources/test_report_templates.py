"""Tests for the ReportTemplates resource nested under Reports."""

from datetime import datetime, timezone
from unittest.mock import AsyncMock, MagicMock

import pytest

from sift_client.resources import (
    ReportsAPIAsync,
    ReportTemplatesAPI,
    ReportTemplatesAPIAsync,
)
from sift_client.sift_types import ReportTemplate, ReportTemplateCreate, ReportTemplateUpdate

NOW = datetime(2026, 1, 1, tzinfo=timezone.utc)


def _template(template_id: str = "template-1") -> ReportTemplate:
    return ReportTemplate(
        id_=template_id,
        organization_id="org-1",
        client_key=None,
        name="template",
        description=None,
        created_by_user_id="user-1",
        modified_by_user_id="user-1",
        created_date=NOW,
        modified_date=NOW,
        rules=[],
        tags=[],
        metadata={},
        archived_date=None,
        is_archived=False,
    )


def _api(mock_client) -> ReportTemplatesAPIAsync:
    api = ReportTemplatesAPIAsync(mock_client)
    api._low_level_client = MagicMock()
    return api


def test_client_binding(sift_client):
    assert sift_client.reports.templates
    assert isinstance(sift_client.reports.templates, ReportTemplatesAPI)
    assert sift_client.reports.templates is sift_client.reports.templates
    assert sift_client.async_.reports.templates
    assert isinstance(sift_client.async_.reports.templates, ReportTemplatesAPIAsync)


class TestReportTemplatesGet:
    @pytest.mark.asyncio
    async def test_forwards_identifiers_to_low_level(self, mock_client):
        api = _api(mock_client)
        api._low_level_client.get_report_template = AsyncMock(return_value=_template())

        template = await api.get(client_key="template-key", organization_id="org-1")

        api._low_level_client.get_report_template.assert_awaited_once_with(
            report_template_id=None, client_key="template-key", organization_id="org-1"
        )
        assert template.id_ == "template-1"


class TestReportTemplatesList:
    @pytest.mark.asyncio
    async def test_builds_cel_filter_and_forwards_args(self, mock_client):
        api = _api(mock_client)
        api._low_level_client.list_all_report_templates = AsyncMock(return_value=[_template()])

        await api.list_(
            name="template",
            report_template_ids=["template-1"],
            client_keys=["template-key"],
            organization_id="org-1",
            order_by="created_date desc",
            limit=10,
        )

        kwargs = api._low_level_client.list_all_report_templates.await_args.kwargs
        query_filter = kwargs["query_filter"]
        assert "name == 'template'" in query_filter
        assert "report_template_id in ['template-1']" in query_filter
        assert "client_key in ['template-key']" in query_filter
        assert "is_archived == false" in query_filter
        assert kwargs["organization_id"] == "org-1"
        assert kwargs["order_by"] == "created_date desc"
        assert kwargs["max_results"] == 10

    @pytest.mark.asyncio
    async def test_include_archived_drops_archived_filter(self, mock_client):
        api = _api(mock_client)
        api._low_level_client.list_all_report_templates = AsyncMock(return_value=[])

        await api.list_(include_archived=True)

        kwargs = api._low_level_client.list_all_report_templates.await_args.kwargs
        assert kwargs["query_filter"] is None

    @pytest.mark.asyncio
    async def test_find_raises_on_multiple_matches(self, mock_client):
        api = _api(mock_client)
        api._low_level_client.list_all_report_templates = AsyncMock(
            return_value=[_template("template-1"), _template("template-2")]
        )

        with pytest.raises(ValueError, match="Multiple report templates found for query"):
            await api.find(name="template")


class TestReportTemplatesCreate:
    @pytest.mark.asyncio
    async def test_accepts_dict_and_validates(self, mock_client):
        api = _api(mock_client)
        api._low_level_client.create_report_template = AsyncMock(return_value=_template())

        template = await api.create({"name": "template", "rule_ids": ["rule-1"]})

        create = api._low_level_client.create_report_template.await_args.kwargs["create"]
        assert isinstance(create, ReportTemplateCreate)
        assert create.name == "template"
        assert create.rule_ids == ["rule-1"]
        assert template.id_ == "template-1"

    @pytest.mark.asyncio
    async def test_accepts_create_model(self, mock_client):
        api = _api(mock_client)
        api._low_level_client.create_report_template = AsyncMock(return_value=_template())

        create = ReportTemplateCreate(name="template")
        await api.create(create)

        assert api._low_level_client.create_report_template.await_args.kwargs["create"] is create


class TestReportTemplatesUpdate:
    @pytest.mark.asyncio
    async def test_accepts_object_and_dict_update(self, mock_client):
        api = _api(mock_client)
        api._low_level_client.update_report_template = AsyncMock(return_value=_template())

        await api.update(_template("template-9"), {"name": "renamed"})

        update = api._low_level_client.update_report_template.await_args.kwargs["update"]
        assert isinstance(update, ReportTemplateUpdate)
        assert update.resource_id == "template-9"
        assert update.name == "renamed"

    @pytest.mark.asyncio
    async def test_archive_sets_is_archived(self, mock_client):
        api = _api(mock_client)
        api._low_level_client.update_report_template = AsyncMock(return_value=_template())

        await api.archive(report_template="template-1")

        update = api._low_level_client.update_report_template.await_args.kwargs["update"]
        assert update.resource_id == "template-1"
        assert update.is_archived is True

    @pytest.mark.asyncio
    async def test_unarchive_clears_is_archived(self, mock_client):
        api = _api(mock_client)
        api._low_level_client.update_report_template = AsyncMock(return_value=_template())

        await api.unarchive(report_template="template-1")

        update = api._low_level_client.update_report_template.await_args.kwargs["update"]
        assert update.resource_id == "template-1"
        assert update.is_archived is False


class TestReportsCreateFromTemplate:
    """Unit tests for ReportsAPIAsync.create_from_template argument coercion."""

    @pytest.mark.asyncio
    async def test_accepts_template_and_run_objects(self, mock_client):
        api = ReportsAPIAsync(mock_client)
        evaluate_rules = AsyncMock(return_value=(0, None, None))
        api._rules_low_level_client.evaluate_rules = evaluate_rules

        await api.create_from_template(report_template=_template("template-9"), run="run-1")

        kwargs = evaluate_rules.await_args.kwargs
        assert kwargs["report_template_id"] == "template-9"
        assert kwargs["run_id"] == "run-1"

    @pytest.mark.asyncio
    async def test_accepts_template_id_string(self, mock_client):
        api = ReportsAPIAsync(mock_client)
        evaluate_rules = AsyncMock(return_value=(0, None, None))
        api._rules_low_level_client.evaluate_rules = evaluate_rules

        await api.create_from_template(report_template="template-1", run="run-1", name="my report")

        kwargs = evaluate_rules.await_args.kwargs
        assert kwargs["report_template_id"] == "template-1"
        assert kwargs["report_name"] == "my report"
