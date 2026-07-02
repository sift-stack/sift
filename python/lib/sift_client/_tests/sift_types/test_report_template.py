"""Tests for sift_types.ReportTemplate models."""

from datetime import datetime, timezone
from unittest.mock import MagicMock

import pytest
from sift.report_templates.v1.report_templates_pb2 import (
    ReportTemplate as ReportTemplateProto,
)
from sift.report_templates.v1.report_templates_pb2 import (
    ReportTemplateRule as ReportTemplateRuleProto,
)
from sift.report_templates.v1.report_templates_pb2 import (
    ReportTemplateTag as ReportTemplateTagProto,
)

from sift_client.sift_types.report_template import (
    ReportTemplate,
    ReportTemplateCreate,
    ReportTemplateRule,
    ReportTemplateUpdate,
)
from sift_client.sift_types.rule import Rule
from sift_client.sift_types.tag import Tag

NOW = datetime(2026, 1, 1, tzinfo=timezone.utc)


def _rule(rule_id: str = "rule-1") -> Rule:
    """Build a minimal Rule instance for object-or-ID coercion tests."""
    return Rule(
        id_=rule_id,
        name="test_rule",
        description="",
        created_date=NOW,
        modified_date=NOW,
        created_by_user_id="user-1",
        modified_by_user_id="user-1",
        organization_id="org-1",
        is_archived=False,
        is_external=False,
        evaluate_on_live_data=False,
        current_version_id="version-1",
        expression=None,
        channel_references=None,
        action=None,
        asset_ids=None,
        asset_tag_ids=None,
        contextual_channels=None,
        client_key=None,
        rule_version=None,
        archived_date=None,
    )


def _tag(name: str = "tag-1") -> Tag:
    return Tag(id_="tag-id-1", name=name, created_date=NOW, created_by_user_id="user-1")


class TestReportTemplateCreate:
    """Unit tests for ReportTemplateCreate proto conversion."""

    def test_to_proto_sets_all_fields(self):
        create = ReportTemplateCreate(
            name="template",
            client_key="template-key",
            description="a template",
            tags=["tag1", "tag2"],
            rule_ids=["rule-1", "rule-2"],
            metadata={"key1": "value1", "key2": 42.5},
            organization_id="org-1",
        )

        proto = create.to_proto()

        assert proto.name == "template"
        assert proto.client_key == "template-key"
        assert proto.description == "a template"
        assert list(proto.tag_names) == ["tag1", "tag2"]
        assert proto.organization_id == "org-1"
        assert proto.WhichOneof("rule_identifiers") == "rule_ids"
        assert list(proto.rule_ids.rule_ids) == ["rule-1", "rule-2"]
        assert len(proto.metadata) == 2

    def test_to_proto_minimal_leaves_optionals_unset(self):
        proto = ReportTemplateCreate(name="template").to_proto()

        assert proto.name == "template"
        assert not proto.HasField("client_key")
        assert not proto.HasField("description")
        assert proto.WhichOneof("rule_identifiers") is None
        assert list(proto.tag_names) == []
        assert len(proto.metadata) == 0

    def test_to_proto_with_rule_client_keys(self):
        proto = ReportTemplateCreate(name="template", rule_client_keys=["key-1"]).to_proto()

        assert proto.WhichOneof("rule_identifiers") == "rule_client_keys"
        assert list(proto.rule_client_keys.rule_client_keys) == ["key-1"]

    def test_rule_and_tag_objects_coerced_to_ids_and_names(self):
        proto = ReportTemplateCreate(
            name="template",
            rule_ids=[_rule("rule-9"), "rule-2"],
            tags=[_tag("tag-9"), "tag-2"],
        ).to_proto()

        assert list(proto.rule_ids.rule_ids) == ["rule-9", "rule-2"]
        assert list(proto.tag_names) == ["tag-9", "tag-2"]

    def test_raises_when_both_rule_ids_and_client_keys_provided(self):
        with pytest.raises(ValueError, match="Only one of rule_ids or rule_client_keys"):
            ReportTemplateCreate(name="template", rule_ids=["rule-1"], rule_client_keys=["key-1"])


class TestReportTemplateUpdate:
    """Unit tests for ReportTemplateUpdate proto and field mask conversion."""

    def test_scalar_fields_and_mask(self):
        update = ReportTemplateUpdate(name="new name", description="new desc", is_archived=True)
        update.resource_id = "template-1"

        proto, mask = update.to_proto_with_mask()

        assert proto.report_template_id == "template-1"
        assert proto.name == "new name"
        assert proto.description == "new desc"
        assert proto.is_archived is True
        assert sorted(mask.paths) == ["description", "is_archived", "name"]

    def test_metadata_converter(self):
        update = ReportTemplateUpdate(metadata={"key1": "value1", "key2": True})
        update.resource_id = "template-1"

        proto, mask = update.to_proto_with_mask()

        metadata_dict = {md.key.name: md for md in proto.metadata}
        assert metadata_dict["key1"].string_value == "value1"
        assert metadata_dict["key2"].boolean_value is True
        assert list(mask.paths) == ["metadata"]

    def test_tags_converted_to_messages(self):
        update = ReportTemplateUpdate(tags=[_tag("tag-9"), "tag-2"])
        update.resource_id = "template-1"

        proto, mask = update.to_proto_with_mask()

        assert [tag.tag_name for tag in proto.tags] == ["tag-9", "tag-2"]
        assert list(mask.paths) == ["tags"]

    def test_empty_tags_clears_with_mask(self):
        update = ReportTemplateUpdate(tags=[])
        update.resource_id = "template-1"

        proto, mask = update.to_proto_with_mask()

        assert len(proto.tags) == 0
        assert list(mask.paths) == ["tags"]

    def test_rule_ids_converted_to_rules(self):
        update = ReportTemplateUpdate(rule_ids=[_rule("rule-9"), "rule-2"])
        update.resource_id = "template-1"

        proto, mask = update.to_proto_with_mask()

        assert [rule.rule_id for rule in proto.rules] == ["rule-9", "rule-2"]
        assert list(mask.paths) == ["rules"]

    def test_rule_client_keys_converted_to_rules(self):
        update = ReportTemplateUpdate(rule_client_keys=["key-1", "key-2"])
        update.resource_id = "template-1"

        proto, mask = update.to_proto_with_mask()

        assert [rule.client_key for rule in proto.rules] == ["key-1", "key-2"]
        assert all(rule.rule_id == "" for rule in proto.rules)
        assert list(mask.paths) == ["rules"]

    def test_raises_when_both_rule_ids_and_client_keys_provided(self):
        with pytest.raises(ValueError, match="Only one of rule_ids or rule_client_keys"):
            ReportTemplateUpdate(rule_ids=["rule-1"], rule_client_keys=["key-1"])

    def test_raises_on_empty_rule_lists(self):
        with pytest.raises(ValueError, match="at least one rule"):
            ReportTemplateUpdate(rule_ids=[])
        with pytest.raises(ValueError, match="at least one rule"):
            ReportTemplateUpdate(rule_client_keys=[])


@pytest.fixture
def mock_report_template_rule():
    """Create a ReportTemplateRule instance for testing."""
    return ReportTemplateRule(
        id_="rule-1",
        rule_id="rule-1",
        rule_version_id="version-1",
        rule_version_number=1,
        rule_client_key="rule-key",
        display_order=0,
    )


@pytest.fixture
def mock_report_template(mock_client, mock_report_template_rule):
    """Create a ReportTemplate instance for testing."""
    template = ReportTemplate(
        proto=MagicMock(),
        id_="template-1",
        organization_id="org-1",
        client_key="template-key",
        name="test_template",
        description="test description",
        created_by_user_id="user-1",
        modified_by_user_id="user-1",
        created_date=NOW,
        modified_date=NOW,
        rules=[mock_report_template_rule],
        tags=["tag1", "tag2"],
        metadata={"key": "value"},
        archived_date=None,
        is_archived=False,
    )
    template._apply_client_to_instance(mock_client)
    return template


class TestReportTemplate:
    """Unit tests for the ReportTemplate model."""

    def test_to_proto(self, mock_report_template):
        proto = mock_report_template.to_proto()

        assert proto.report_template_id == "template-1"
        assert proto.name == "test_template"
        assert proto.client_key == "template-key"
        assert proto.description == "test description"
        assert proto.organization_id == "org-1"
        assert [tag.tag_name for tag in proto.tags] == ["tag1", "tag2"]
        assert len(proto.rules) == 1
        assert proto.rules[0].rule_id == "rule-1"
        assert proto.rules[0].client_key == "rule-key"
        assert proto.is_archived is False

    def test_from_proto_sorts_rules_by_display_order(self):
        proto = ReportTemplateProto(
            report_template_id="template-1",
            organization_id="org-1",
            name="test_template",
            rules=[
                ReportTemplateRuleProto(rule_id="rule-2", display_order=1),
                ReportTemplateRuleProto(rule_id="rule-1", display_order=0),
            ],
            tags=[ReportTemplateTagProto(tag_name="tag1")],
        )
        proto.created_date.FromDatetime(NOW)
        proto.modified_date.FromDatetime(NOW)

        template = ReportTemplate._from_proto(proto)

        assert [rule.rule_id for rule in template.rules] == ["rule-1", "rule-2"]

    def test_from_proto_unset_optionals_are_none(self):
        proto = ReportTemplateProto(
            report_template_id="template-1", organization_id="org-1", name="test_template"
        )
        proto.created_date.FromDatetime(NOW)
        proto.modified_date.FromDatetime(NOW)

        template = ReportTemplate._from_proto(proto)

        assert template.client_key is None
        assert template.description is None
        assert template.archived_date is None
        assert template.rules == []
        assert template.tags == []

    def test_update(self, mock_report_template, mock_client):
        updated_template = MagicMock()
        mock_client.reports.templates.update.return_value = updated_template
        with MagicMock() as mock_update:
            mock_report_template._update = mock_update
            result = mock_report_template.update({"name": "renamed"})
            mock_client.reports.templates.update.assert_called_once_with(
                report_template=mock_report_template, update={"name": "renamed"}
            )
            mock_update.assert_called_once_with(updated_template)
            assert result is mock_report_template

    def test_create_report_delegates_to_client(self, mock_report_template, mock_client):
        job = MagicMock()
        mock_client.reports.create_from_template.return_value = job

        result = mock_report_template.create_report(run="run-1", name="my report")

        mock_client.reports.create_from_template.assert_called_once_with(
            report_template=mock_report_template,
            run="run-1",
            organization_id=None,
            name="my report",
        )
        assert result is job

    def test_archive_and_unarchive_delegate_to_client(self, mock_report_template, mock_client):
        archived_template = MagicMock()
        mock_client.reports.templates.archive.return_value = archived_template
        unarchived_template = MagicMock()
        mock_client.reports.templates.unarchive.return_value = unarchived_template
        with MagicMock() as mock_update:
            mock_report_template._update = mock_update

            result = mock_report_template.archive()
            mock_client.reports.templates.archive.assert_called_once_with(
                report_template=mock_report_template
            )
            mock_update.assert_called_once_with(archived_template)
            assert result is mock_report_template

            result = mock_report_template.unarchive()
            mock_client.reports.templates.unarchive.assert_called_once_with(
                report_template=mock_report_template
            )
            mock_update.assert_called_with(unarchived_template)
            assert result is mock_report_template


class TestReportTemplateRule:
    """Unit tests for the ReportTemplateRule model."""

    def test_proto_round_trip(self):
        proto = ReportTemplateRuleProto(
            rule_id="rule-1", rule_version_id="version-1", rule_version_number=1, display_order=2
        )

        rule = ReportTemplateRule._from_proto(proto)

        # Empty proto string coerces to None at the type boundary.
        assert rule.rule_client_key is None
        assert rule.display_order == 2

        round_tripped = rule.to_proto()
        assert round_tripped.rule_id == "rule-1"
        assert round_tripped.rule_version_id == "version-1"
        assert round_tripped.rule_version_number == 1
        assert round_tripped.client_key == ""
        assert round_tripped.display_order == 2
