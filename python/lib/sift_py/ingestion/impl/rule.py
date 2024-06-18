from typing import List, Optional, cast

from sift.rules.v1.rules_pb2 import (
    JsonRulesRequest,
    UpdateJsonRulesRequest,
    UpdateJsonRulesResponse,
)
from sift.rules.v1.rules_pb2_grpc import RuleServiceStub
from sift_internal.convert.json import to_json
from sift_py.grpc.transport import SiftChannel
from sift_py.ingestion.rule.config import RuleConfig


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
