from __future__ import annotations

from pathlib import Path
from typing import Any, Dict, List, Optional, Tuple, Union, cast

from sift.assets.v1.assets_pb2 import Asset, ListAssetsRequest, ListAssetsResponse
from sift.assets.v1.assets_pb2_grpc import AssetServiceStub
from sift.channels.v2.channels_pb2 import Channel, ListChannelsRequest, ListChannelsResponse
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

import sift_py.yaml.rule as rule_yaml
from sift_py._internal.cel import cel_in
from sift_py.grpc.transport import SiftChannel
from sift_py.ingestion._internal.channel import channel_reference_from_fqn
from sift_py.ingestion.channel import channel_fqn
from sift_py.ingestion.rule.config import (
    ExpressionChannelReference,
    ExpressionChannelReferenceChannelConfig,
    RuleActionCreateDataReviewAnnotation,
    RuleActionCreatePhaseAnnotation,
    RuleActionKind,
    RuleConfig,
)
from sift_py.rule.config import RuleAction

load_rule_modules = rule_yaml.load_rule_modules
SubExpression = rule_yaml.SubExpression


class RuleService:
    """
    A service for managing rules. Allows for loading rules from YAML and creating or updating them in the Sift API.
    """

    _asset_service_stub: AssetServiceStub
    _channel_service_stub: ChannelServiceStub
    _rule_service_stub: RuleServiceStub

    def __init__(self, channel: SiftChannel):
        self._asset_service_stub = AssetServiceStub(channel)
        self._channel_service_stub = ChannelServiceStub(channel)
        self._rule_service_stub = RuleServiceStub(channel)

    def load_rules_from_yaml(
        self,
        paths: List[Path],
        sub_expressions: Optional[List[SubExpression]] = None,
        channel_references_map: Optional[
            Dict[
                str,
                List[Union[ExpressionChannelReference, ExpressionChannelReferenceChannelConfig]],
            ]
        ] = None,
    ) -> List[RuleConfig]:
        """
        Loads rules from a YAML spec, and creates or updates the rules in the Sift API.
        If the rule expression should be interpolated from sub-expressions, provide a list of `SubExpression` objects.
        If the rule does not contain channel references in its YAML definition, provide a dict of rule names mapped
        to a list of channel references. Otherwise if the YAML definition contains channel references, the `channel_references_map`
        should be omitted. If channel references are present in both the YAML definition and provided in the `channel_references_map`,
        or if neither are provided for a given rule, an exception will be thrown.
        For more on rule YAML definitions, see `sift_py.ingestion.config.yaml.spec.RuleYamlSpec`.
        """
        module_rules = load_rule_modules(paths)

        interpolation_map: Dict[str, Dict[str, Any]] = {}
        if sub_expressions:
            for sub_expression in sub_expressions:
                interpolation_map[sub_expression.fully_qualified_rule_name] = (
                    sub_expression.expressions
                )

        rule_configs = []
        for rule in module_rules:
            yaml_channel_references = rule.get("channel_references", [])
            arg_channel_references = (
                channel_references_map.get(rule["name"]) if channel_references_map else None
            )
            channel_references: List[
                Union[ExpressionChannelReference, ExpressionChannelReferenceChannelConfig]
            ] = []

            if yaml_channel_references:  # Parse channel references from YAML
                for channel_ref in yaml_channel_references:
                    for ref, config in channel_ref.items():
                        if isinstance(config, dict):  # Handle ExpressionChannelReference
                            channel_name = config.get("name", "")
                            component = config.get("component", "")
                        elif isinstance(config, str):  # Handle str
                            channel_reference = channel_reference_from_fqn(config)
                            channel_name = channel_reference.name
                            component = channel_reference.component
                        else:
                            raise ValueError(f"Channel malformed: {config}")

                        channel_references.append(
                            {
                                "channel_reference": ref,
                                "channel_identifier": channel_fqn(
                                    {
                                        "channel_name": channel_name,
                                        "component": component,
                                    }
                                ),
                            }
                        )
            elif arg_channel_references:  # Or parse channel references provided as an argument
                channel_references = cast(
                    List[
                        Union[ExpressionChannelReference, ExpressionChannelReferenceChannelConfig]
                    ],
                    arg_channel_references,
                )

            if not channel_references:
                raise ValueError(f"Rule of name '{rule['name']}' requires channel_references")

            rule_name = rule["name"]
            rule_fqn = rule_name
            rule_subexpr = interpolation_map.get(rule_fqn, {})

            expression = rule["expression"]
            if isinstance(expression, dict):  # Handle named expressions
                expression = expression.get("name", "")

            tags = rule.get("tags", [])
            annotation_type = rule_yaml.RuleActionAnnotationKind.from_str(rule["type"])
            action: RuleAction = RuleActionCreatePhaseAnnotation(tags)
            if annotation_type == rule_yaml.RuleActionAnnotationKind.REVIEW:
                action = RuleActionCreateDataReviewAnnotation(
                    assignee=rule.get("assignee"),
                    tags=tags,
                )

            rule_configs.append(
                RuleConfig(
                    name=rule["name"],
                    rule_client_key=rule.get("rule_client_key"),
                    description=rule.get("description", ""),
                    expression=expression,
                    action=action,
                    channel_references=channel_references,
                    asset_names=rule.get("asset_names", []),
                    sub_expressions=rule_subexpr,
                )
            )

        for rule_config in rule_configs:
            self.create_or_update_rule(rule_config)

        return rule_configs

    def create_or_update_rule(self, config: RuleConfig):
        """
        Create or update a rule via a RuleConfig. The config must contain a rule_client_key or an exception will be raised.
        If a rule with the given client key already exists it will be updated, otherwise it will be created.
        See `sift_py.rule.config.RuleConfig` for more information on configuation parameters for rules.
        """
        if not config.rule_client_key:
            raise ValueError(f"Rule of name '{config.name}' requires a rule_client_key")

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
            raise ValueError(
                "Cannot create a rule with an empty expression."
                "See `sift_py.rule.config.RuleConfig` for more information on rule configuration."
            )

        if not config.action:
            raise ValueError(
                "Cannot create a rule with no corresponding action."
                "See `sift_py.rule.config.RuleAction` for available actions."
            )

        # TODO:
        # - once we have TagService_ListTags we can do asset-agnostic rules via tags
        print(f"Creating rule {config.__dict__} with expression {config.expression}")  # TODO
        assets = self._get_assets_by_names(config.asset_names) if config.asset_names else None

        actions = []
        if config.action.kind() == RuleActionKind.NOTIFICATION:
            raise NotImplementedError(
                "Notification actions are not yet supported."
                "Please contact the Sift team for assistance."
            )
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

        def search_channels(
            filter: str = "", page_size: int = 1_000, page_token: str = ""
        ) -> Tuple[List[Channel], str]:
            req = ListChannelsRequest(
                filter=filter,
                page_size=page_size,
                page_token=page_token,
            )
            res = cast(ListChannelsResponse, self._channel_service_stub.ListChannels(req))
            return list(res.channels), res.next_page_token

        if assets and channel_references:
            identifiers = [ident.name for ident in channel_references.values()]
            components = [ident.component for ident in channel_references.values()]

            # Create CEL search filters
            name_in = cel_in("name", identifiers)
            component_in = cel_in("component", components)

            # Validate channels are present within each asset
            for asset in assets:
                found_channels = []
                filter = f"asset_id == '{asset.asset_id}' && {name_in} && {component_in}"
                channels, next_page_token = search_channels(  # Initialize next_page_token
                    filter,
                )
                found_channels.extend([channel.name for channel in channels])

                while len(next_page_token) > 0:
                    channels, next_page_token = search_channels(
                        filter=filter,
                        page_token=next_page_token,
                    )
                    found_channels.extend([channel.name for channel in channels])

                missing_channels = set(identifiers) ^ set(found_channels)
                if missing_channels:
                    raise RuntimeError(
                        f"Asset {asset.name} is missing channels required for rule {config.name}: {missing_channels}"
                    )

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
