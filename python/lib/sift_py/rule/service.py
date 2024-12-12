from __future__ import annotations

from dataclasses import dataclass
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
from sift.users.v2.users_pb2_grpc import UserServiceStub

from sift_py._internal.cel import cel_in
from sift_py._internal.channel import get_channels
from sift_py._internal.user import get_active_users
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
from sift_py.rule.config import RuleAction
from sift_py.yaml.rule import RuleActionAnnotationKind


class RuleService:
    """
    A service for managing rules. Allows for loading rules from YAML and creating or updating them in the Sift API.
    """

    _asset_service_stub: AssetServiceStub
    _channel_service_stub: ChannelServiceStub
    _rule_service_stub: RuleServiceStub
    _user_service_stub: UserServiceStub

    def __init__(self, channel: SiftChannel):
        self._asset_service_stub = AssetServiceStub(channel)
        self._channel_service_stub = ChannelServiceStub(channel)
        self._rule_service_stub = RuleServiceStub(channel)
        self._user_service_stub = UserServiceStub(channel)

    def load_rules_from_yaml(
        self,
        paths: List[Path],
        sub_expressions: Optional[List[SubExpression]] = None,
        channel_references: Optional[List[RuleChannelReference]] = None,
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
        namespaced_rules = load_rule_namespaces(paths)

        interpolation_map: Dict[str, Dict[str, Any]] = {}
        if sub_expressions:
            for sub_expression in sub_expressions:
                interpolation_map[sub_expression.fully_qualified_rule_name] = (
                    sub_expression.expressions
                )

        rule_configs = []
        for namespace, rule_yamls in namespaced_rules.items():
            for rule_yaml in rule_yamls:
                yaml_channel_references = rule_yaml.get("channel_references", [])
                arg_channel_references: Dict[str, Any] = {}

                if channel_references:
                    for rule_channel_refs in channel_references:
                        if rule_channel_refs.fully_qualified_rule_name == rule_yaml["name"]:
                            arg_channel_references = rule_channel_refs.channel_references

                if yaml_channel_references and arg_channel_references:
                    raise ValueError(
                        f"Rule of name '{rule_yaml['name']}' cannot have both YAML and channel_references argument provided. "
                        "Please provide only one or the other."
                    )

                rule_channel_references: List[
                    Union[ExpressionChannelReference, ExpressionChannelReferenceChannelConfig]
                ] = []

                def parse_channel_refs(channel_ref: Dict[str, Any]):
                    for ref, channel_config in channel_ref.items():
                        if isinstance(channel_config, dict):
                            name = channel_config.get("name", "")
                            component = channel_config.get("component", "")
                        elif isinstance(channel_config, str):
                            channel_reference = channel_reference_from_fqn(channel_config)
                            name = channel_reference.name
                            component = channel_reference.component
                        else:
                            raise ValueError(
                                f"Channel reference '{channel_config}' must be a string or a ChannelConfigYamlSpec"
                            )

                        rule_channel_references.append(
                            {
                                "channel_reference": ref,
                                "channel_identifier": channel_fqn(
                                    {
                                        "channel_name": name,
                                        "component": component,
                                    }
                                ),
                            }
                        )

                if yaml_channel_references:
                    for channel_ref in yaml_channel_references:
                        parse_channel_refs(channel_ref)
                elif arg_channel_references:
                    parse_channel_refs(arg_channel_references)

                if not rule_channel_references:
                    raise ValueError(
                        f"Rule of name '{rule_yaml['name']}' requires channel_references"
                    )

                rule_name = rule_yaml["name"]
                rule_fqn = f"{namespace}.{rule_name}"
                rule_subexpr = interpolation_map.get(rule_fqn, {})

                expression = rule_yaml["expression"]
                if isinstance(expression, dict):  # Handle named expressions
                    expression = expression.get("name", "")

                tags = rule_yaml.get("tags", [])
                annotation_type = RuleActionAnnotationKind.from_str(rule_yaml["type"])
                action: RuleAction = RuleActionCreatePhaseAnnotation(tags)
                if annotation_type == RuleActionAnnotationKind.REVIEW:
                    action = RuleActionCreateDataReviewAnnotation(
                        assignee=rule_yaml.get("assignee"),
                        tags=tags,
                    )

                rule_configs.append(
                    RuleConfig(
                        name=rule_yaml["name"],
                        namespace=namespace,
                        namespace_rules=namespaced_rules,
                        action=action,
                        rule_client_key=rule_yaml.get("rule_client_key"),
                        description=rule_yaml.get("description", ""),
                        expression=cast(str, rule_yaml["expression"]),
                        channel_references=rule_channel_references,
                        asset_names=rule_yaml.get("asset_names", []),
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
        assets = self._get_assets_by_names(config.asset_names) if config.asset_names else None

        actions = []
        if config.action.kind() == RuleActionKind.NOTIFICATION:
            raise NotImplementedError(
                "Notification actions are not yet supported."
                "Please contact the Sift team for assistance."
            )
        elif config.action.kind() == RuleActionKind.ANNOTATION:
            if isinstance(config.action, RuleActionCreateDataReviewAnnotation):
                assignee = config.action.assignee
                user_id = None
                if assignee:
                    users = get_active_users(
                        user_service=self._user_service_stub,
                        filter=f"name=='{assignee}'",
                    )
                    if not users:
                        raise ValueError(f"Cannot find user '{assignee}'.")
                    if len(users) > 1:
                        raise ValueError(f"Multiple users found with name '{assignee}'.")
                    user_id = users[0].user_id

                action_config = UpdateActionRequest(
                    action_type=ANNOTATION,
                    configuration=RuleActionConfiguration(
                        annotation=AnnotationActionConfiguration(
                            assigned_to_user_id=user_id,
                            # tag_ids=config.action.tags,  # TODO: Requires TagService
                        )
                    ),
                )
                actions.append(action_config)
            elif isinstance(config.action, RuleActionCreatePhaseAnnotation):
                action_config = UpdateActionRequest(
                    action_type=ANNOTATION,
                    configuration=RuleActionConfiguration(
                        annotation=AnnotationActionConfiguration(
                            # tag_ids=config.action.tags,  # TODO: Requires TagService
                        )
                    ),
                )

        channel_references = {}
        for channel_reference in config.channel_references:
            ref = channel_reference["channel_reference"]
            ident = channel_reference_from_fqn(channel_reference["channel_identifier"])
            channel_references[ref] = ident

        if assets and channel_references:
            identifiers = [ident.name for ident in channel_references.values()]
            components = [ident.component for ident in channel_references.values()]

            # Create CEL search filters
            name_in = cel_in("name", identifiers)
            component_in = cel_in("component", components)

            # Validate channels are present within each asset
            for asset in assets:
                found_channels = get_channels(
                    channel_service=self._channel_service_stub,
                    filter=f"asset_id == '{asset.asset_id}' && {name_in} && {component_in}",
                )
                found_channels_names = [channel.name for channel in found_channels]

                missing_channels = set(identifiers) ^ set(found_channels_names)
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


@dataclass
class SubExpression:
    fully_qualified_rule_name: str
    expressions: Dict[str, Any]

    def __init__(self, fully_qualified_rule_name: str, expressions: Dict[str, Any]):
        self.fully_qualified_rule_name = fully_qualified_rule_name
        self.expressions = expressions


@dataclass
class RuleChannelReference:
    """
    Convenient wrapper to map fully qualified rule names to relevant channel references
    when creating rules from yaml.
    """

    fully_qualified_rule_name: str
    channel_references: Dict[str, Any]

    def __init__(self, fully_qualified_rule_name: str, channel_references: Dict[str, Any]):
        self.fully_qualified_rule_name = fully_qualified_rule_name
        self.channel_references = channel_references
