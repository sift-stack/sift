from __future__ import annotations

from pathlib import Path
from typing import Any, Dict, List, Optional, Union, cast

from sift.assets.v1.assets_pb2 import Asset, ListAssetsRequest, ListAssetsResponse
from sift.assets.v1.assets_pb2_grpc import AssetServiceStub
from sift.channels.v2.channels_pb2_grpc import ChannelServiceStub
from sift.rules.v1.rules_pb2 import (
    ANNOTATION,
    AnnotationActionConfiguration,
    CalculatedChannelConfig,
    CreateRuleRequest,
    GetRuleRequest,
    GetRuleResponse,
    Rule,
    RuleActionConfiguration,
    RuleAssetConfiguration,
    RuleConditionExpression,
    UpdateActionRequest,
    UpdateConditionRequest,
    UpdateRuleRequest,
)
from sift.rules.v1.rules_pb2_grpc import RuleServiceStub

from sift_py.grpc.transport import SiftChannel
from sift_py.ingestion._internal.channel import channel_reference_from_fqn
from sift_py.ingestion.channel import channel_fqn
from sift_py.ingestion.config.yaml.load import load_rule_namespaces
from sift_py.ingestion.rule.config import (
    ExpressionChannelReference,
    ExpressionChannelReferenceChannelConfig,
    RuleActionCreateDataReviewAnnotation,
    RuleActionCreatePhaseAnnotation,
    RuleActionKind,
    RuleConfig,
)


class RuleService:
    _asset_service_stub: AssetServiceStub
    _channel_service_stub: ChannelServiceStub
    _rule_service_stub: RuleServiceStub

    def __init__(self, channel: SiftChannel):
        self._asset_service_stub = AssetServiceStub(channel)
        self._channel_service_stub = ChannelServiceStub(channel)
        self._rule_service_stub = RuleServiceStub(channel)

    def load_rules_from_yaml(
        self, paths: List[Path], sub_expressions: List[SubExpression]
    ) -> List[RuleConfig]:
        namespaced_rules = load_rule_namespaces(paths)

        interpolation_map: Dict[str, Dict[str, Any]] = {}
        for sub_expression in sub_expressions:
            interpolation_map[sub_expression.fully_qualified_rule_name] = sub_expression.expressions

        rule_configs = []
        for namespace, rule_yamls in namespaced_rules.items():
            for rule_yaml in rule_yamls:
                channel_references: List[
                    Union[ExpressionChannelReference, ExpressionChannelReferenceChannelConfig]
                ] = []

                for channel_ref in rule_yaml.get("channel_references", []):
                    for ref, config in channel_ref.items():
                        channel_references.append(
                            {
                                "channel_reference": ref,
                                "channel_identifier": channel_fqn(
                                    {
                                        "channel_name": config.get("name", ""),
                                        "component": config.get("component", ""),
                                    }
                                ),
                            }
                        )

                rule_name = rule_yaml["name"]
                rule_fqn = f"{namespace}.{rule_name}"
                rule_subexpr = interpolation_map.get(rule_fqn, {})

                rule_configs.append(
                    RuleConfig(
                        name=rule_yaml["name"],
                        namespace=namespace,
                        namespace_rules=namespaced_rules,
                        rule_client_key=rule_yaml.get("rule_client_key"),
                        description=rule_yaml.get("description", ""),
                        expression=cast(str, rule_yaml["expression"]),
                        channel_references=channel_references,
                        asset_names=rule_yaml.get("asset_names", []),
                        sub_expressions=rule_subexpr,
                    )
                )

        for rule_config in rule_configs:
            self.create_or_update_rule(rule_config)

        return rule_configs

    def create_or_update_rule(self, config: RuleConfig):
        if not config.rule_client_key:
            raise Exception(f"rule of name '{config.name}' requires a rule_client_key")

        rule = self._get_rule_from_client_key(config.rule_client_key)
        if rule:
            self._update_rule(config, rule)
        else:
            self._create_rule(config)

    def _update_rule(self, updated_config: RuleConfig, rule: Rule):
        req = self._update_req_from_rule_config(updated_config, rule)
        self._rule_service_stub.UpdateRule(req)

    def _create_rule(self, config: RuleConfig):
        req = self._update_req_from_rule_config(config)
        self._rule_service_stub.CreateRule(CreateRuleRequest(update=req))

    def _update_req_from_rule_config(
        self, config: RuleConfig, rule: Optional[Rule] = None
    ) -> UpdateRuleRequest:
        if not config.expression:
            raise Exception("cannot create a rule with an empty expression")

        if not config.action:
            raise Exception("cannot create a rule with no corresponding action")

        # TODO:
        # - once we have TagService_ListTags we can do asset-agnostic rules via tags
        # - we need to validate that that channel references refer to an actual channel for an asset
        assets = self._get_assets_by_names(config.asset_names) if config.asset_names else None

        actions = []
        if config.action.kind() == RuleActionKind.NOTIFICATION:
            raise Exception("notification actions are not yet supported")
        elif config.action.kind() == RuleActionKind.ANNOTATION:
            if isinstance(config.action, RuleActionCreateDataReviewAnnotation):
                action_config = UpdateActionRequest(
                    action_type=ANNOTATION,
                    configuration=RuleActionConfiguration(
                        annotation=AnnotationActionConfiguration(
                            assigned_to_user_id=config.action.assignee,
                            tag_ids=config.action.tags,
                        )
                    ),
                )
                actions.append(action_config)
            elif isinstance(config.action, RuleActionCreatePhaseAnnotation):
                action_config = UpdateActionRequest(
                    action_type=ANNOTATION,
                    configuration=RuleActionConfiguration(
                        annotation=AnnotationActionConfiguration(
                            tag_ids=config.action.tags,
                        )
                    ),
                )

        channel_references = {}
        for channel_reference in config.channel_references:
            ref = channel_reference["channel_reference"]
            ident = channel_reference_from_fqn(channel_reference["channel_identifier"])
            channel_references[ref] = ident

        rule_id = None
        organization_id = ""
        if rule:
            rule_id = rule.rule_id
            organization_id = rule.organization_id

        return UpdateRuleRequest(
            organization_id=organization_id,
            rule_id=rule_id,
            client_key=config.rule_client_key,
            name=config.name,
            description=config.description,
            conditions=[
                UpdateConditionRequest(
                    actions=actions,
                    expression=RuleConditionExpression(
                        calculated_channel=CalculatedChannelConfig(
                            expression=config.expression,
                            channel_references=channel_references,
                        )
                    ),
                )
            ],
            asset_configuration=RuleAssetConfiguration(
                asset_ids=[asset.asset_id for asset in assets] if assets else None,
            ),
        )

    def _get_rule_from_client_key(self, client_key: str) -> Optional[Rule]:
        req = GetRuleRequest(client_key=client_key)
        try:
            res = cast(GetRuleResponse, self._rule_service_stub.GetRule(req))
            return res.rule or None
        except:
            return None

    def _get_assets_by_names(self, names: List[str]) -> List[Asset]:
        quoted_names = [f"'{name}'" for name in names]

        assets: List[Asset] = []
        next_page_token = ""
        while True:
            req = ListAssetsRequest(
                filter="name in [{}]".format(",".join(quoted_names)),
                page_size=1_000,
                page_token=next_page_token,
            )
            res = cast(ListAssetsResponse, self._asset_service_stub.ListAssets(req))
            assets.extend(res.assets)

            if not res.next_page_token:
                break
            next_page_token = res.next_page_token

        return assets


class SubExpression:
    fully_qualified_rule_name: str
    expressions: Dict[str, Any]

    def __init__(self, fully_qualified_rule_name: str, expressions: Dict[str, Any]):
        self.fully_qualified_rule_name = fully_qualified_rule_name
        self.expressions = expressions
