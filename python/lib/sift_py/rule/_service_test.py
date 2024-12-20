from unittest import mock

import pytest
from sift.assets.v1.assets_pb2 import Asset
from sift.rules.v1.rules_pb2 import Rule

from sift_py._internal.test_util.channel import MockChannel
from sift_py.rule.config import RuleActionCreateDataReviewAnnotation, RuleConfig
from sift_py.rule.service import RuleService


@pytest.fixture()
def rule_service():
    return RuleService(MockChannel())


def test_rule_service_get_rule_from_client_key(rule_service):
    rule_client_key = "rule-client-key"

    with mock.patch.object(
        RuleService,
        "_get_rule_from_client_key",
        return_value=Rule(name="abc"),
    ) as mock_get_rule_from_client_key:
        rule_service.get_rule(rule_client_key)
        mock_get_rule_from_client_key.assert_called_once_with(rule_client_key)


def test_rule_service_get_rule_from_rule_id(rule_service):
    rule_id = "rule-id"

    with mock.patch.object(
        RuleService, "_get_rule_from_rule_id", return_value=Rule(name="abc")
    ) as mock_get_rule_from_rule_id:
        rule_service.get_rule(rule_id)
        mock_get_rule_from_rule_id.assert_called_once_with(rule_id)


def test_rule_service_get_rule_missing_client_key_and_id(rule_service):
    with mock.patch.object(
        RuleService,
        "_get_rule_from_client_key",
        return_value=None,
    ) as mock_get_rule_from_client_key:
        with mock.patch.object(
            RuleService, "_get_rule_from_rule_id", return_value=None
        ) as mock_get_rule_from_rule_id:
            rule = rule_service.get_rule("")
            mock_get_rule_from_client_key.asset_called_once()
            mock_get_rule_from_rule_id.asset_called_once()
            assert rule is None


def test_rule_service_create_rule(rule_service):
    rule = RuleConfig(
        name="rule",
        rule_client_key="rule-client-key",
        channel_references=[],
    )

    with mock.patch.object(RuleService, "_create_rule") as mock_create_rule:
        rule_service.create_or_update_rule(rule)
        mock_create_rule.assert_called_once_with(rule)


def test_rule_service_update_rule(rule_service):
    rule = RuleConfig(
        name="rule",
        rule_client_key="rule-client-key",
        channel_references=[],
    )

    with mock.patch.object(RuleService, "_update_rule") as mock_update_rule:
        with mock.patch.object(
            RuleService, "_get_rule_from_client_key", return_value=Rule(name=rule.name)
        ) as mock_get_rule_from_client_key:
            rule_service.create_or_update_rule(rule)
            mock_get_rule_from_client_key.assert_called_once_with(rule.rule_client_key)
            mock_update_rule.assert_called_once()


def test_rule_service_load_rules_from_yaml(rule_service):
    rule_yaml = {
        "name": "rule",
        "rule_client_key": "rule-client-key",
        "channel_references": [{"$1": "channel"}],
        "description": "description",
        "expression": "$1 > 0",
        "assignee": "assignee@abc.com",
        "type": "review",
        "asset_names": ["asset"],
    }
    with mock.patch.object(RuleService, "create_or_update_rule"):
        with mock.patch(
            "sift_py.rule.service.load_rule_modules",
            return_value=[rule_yaml],
        ):
            rule_configs = rule_service.load_rules_from_yaml(["path/to/rules.yml"])
            assert len(rule_configs) == 1

            rule_config = rule_configs[0]
            assert rule_config.name == rule_yaml["name"]
            assert rule_config.rule_client_key == rule_yaml["rule_client_key"]
            assert rule_config.channel_references == [
                {
                    "channel_reference": "$1",
                    "channel_identifier": "channel",
                }
            ]
            assert rule_config.description == rule_yaml["description"]
            assert rule_config.expression == rule_yaml["expression"]
            assert rule_config.action.assignee == rule_yaml["assignee"]
            assert rule_config.asset_names == rule_yaml["asset_names"]
            assert isinstance(rule_config.action, RuleActionCreateDataReviewAnnotation)


def test_rule_service_attach_asset():
    rule_config = RuleConfig(
        name="rule",
        rule_client_key="rule-client-key",
        expression="1",
        channel_references=[],
        action=RuleActionCreateDataReviewAnnotation(),
        asset_names=["abc"],
    )
    asset = Asset(name="asset", asset_id="asset-id", organization_id="org-id")
    with mock.patch("sift_py.rule.service.RuleServiceStub", return_value=mock.MagicMock()):
        rule_service = RuleService(MockChannel())
        with mock.patch.object(RuleService, "_get_assets", return_value=[asset]):
            returned_config = rule_service.attach_asset(rule_config, ["asset"])
            assert "abc" in returned_config.asset_names
            assert "asset" in returned_config.asset_names


def test_rule_service_detach_asset():
    rule_config = RuleConfig(
        name="rule",
        rule_client_key="rule-client-key",
        expression="1",
        channel_references=[],
        action=RuleActionCreateDataReviewAnnotation(),
        asset_names=["abc", "def"],
    )
    asset_abc = Asset(name="abc", asset_id="abc-id", organization_id="org-id")
    asset_def = Asset(name="def", asset_id="def-id", organization_id="org-id")
    with mock.patch("sift_py.rule.service.RuleServiceStub", return_value=mock.MagicMock()):
        rule_service = RuleService(MockChannel())
        with mock.patch.object(RuleService, "_get_assets", return_value=[asset_abc, asset_def]):
            returned_config = rule_service.detach_asset(rule_config, ["abc"])
            assert "abc" not in returned_config.asset_names
            assert "def" in returned_config.asset_names


def test_rule_service_detach_asset_empty_list():
    rule_config = RuleConfig(
        name="rule",
        rule_client_key="rule-client-key",
        expression="1",
        channel_references=[],
        action=RuleActionCreateDataReviewAnnotation(),
        asset_names=["asset"],
    )
    asset = Asset(name="asset", asset_id="asset-id", organization_id="org-id")
    with mock.patch("sift_py.rule.service.RuleServiceStub", return_value=mock.MagicMock()):
        rule_service = RuleService(MockChannel())
        with mock.patch.object(RuleService, "_get_assets", return_value=[asset]):
            with pytest.raises(ValueError, match="must be associated with at least one asset"):
                rule_service.detach_asset(rule_config, ["asset"])
