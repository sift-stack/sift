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

from sift_py._internal.convert.json import to_json
from sift_py.grpc.transport import SiftChannel
from sift_py.ingestion.rule.config import RuleConfig


def get_asset_rules_json(
    transport_channel: SiftChannel,
    asset_id: str,
) -> List[Dict[str, Any]]:
    svc = RuleServiceStub(transport_channel)
    req = ViewJsonRulesRequest(asset_id=asset_id)
    res = cast(ViewJsonRulesResponse, svc.ViewJsonRules(req))
    rules_json: List[Dict[str, Any]] = cast(list, json.loads(res.rules_json))
    return rules_json


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
