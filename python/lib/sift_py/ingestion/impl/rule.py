import json
from typing import Any, Dict, List, Optional, cast

from sift.rules.v1.rules_pb2 import (
    JsonRulesRequest,
    UpdateJsonRulesRequest,
    UpdateJsonRulesResponse,
    ViewJsonRulesRequest,
    ViewJsonRulesResponse,
)
from sift.rules.v1.rules_pb2_grpc import RuleServiceStub
from sift_internal.convert.json import to_json
from sift_py.grpc.transport import SiftChannel
from sift_py.ingestion.rule.config import RuleConfig


def validate_rules_synchronized(
    transport_channel: SiftChannel, asset_id: str, rule_configs: List[RuleConfig]
):
    """
    Ensures that rules defined in the telemetry config and the rules in Sift are in sync, otherwise error.
    Namely, if a rule was added via a Sift UI and wasn't added immediately to the telemetry config, then
    this will raise an exception.
    """
    svc = RuleServiceStub(transport_channel)
    req = ViewJsonRulesRequest(asset_id=asset_id)
    res = cast(ViewJsonRulesResponse, svc.ViewJsonRules(req))

    rule_names_from_config = set()

    for rule_config in rule_configs:
        rule_names_from_config.add(rule_config.name)

    rules_json: List[Dict[str, Any]] = cast(list, json.loads(res.rules_json))

    for rule_json in rules_json:
        rule_name: str = rule_json.get("name", "")

        if len(rule_name) == 0:
            raise Exception("Encountered rule without a name from Sift API.")

        if rule_name not in rule_names_from_config:
            raise Exception(
                f"Encountered rule '{rule_name}' on asset '{asset_id}' not found in local telemetry config. Add it."
            )


def update_rules(
    transport_channel: SiftChannel,
    asset_id: str,
    rule_configs: List[RuleConfig],
    organization_id: Optional[str] = None,
):
    """
    Updates a set of rules. Raises an exception if failure.
    """
    svc = RuleServiceStub(transport_channel)
    json_rules = to_json(rule_configs)
    req = UpdateJsonRulesRequest(
        request=JsonRulesRequest(
            asset_id=asset_id,
            rules_json=json_rules,
            organization_id=organization_id or "",
        )
    )
    res = cast(UpdateJsonRulesResponse, svc.UpdateJsonRules(req))

    if not res.response.success:
        raise Exception(f"Failed to load rules: {res.response.error_messages}")
