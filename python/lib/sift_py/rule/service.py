from __future__ import annotations

from dataclasses import dataclass
from pathlib import Path
from typing import Any, Dict, List, Optional, Union, cast

from sift.annotations.v1.annotations_pb2 import AnnotationType
from sift.assets.v1.assets_pb2 import Asset, ListAssetsRequest, ListAssetsResponse
from sift.assets.v1.assets_pb2_grpc import AssetServiceStub
from sift.channels.v3.channels_pb2_grpc import ChannelServiceStub
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
from sift_py._internal.channel import channel_fqn as _channel_fqn
from sift_py._internal.channel import get_channels
from sift_py._internal.user import get_active_users
from sift_py.grpc.transport import SiftChannel
from sift_py.ingestion._internal.channel import channel_reference_from_fqn
from sift_py.ingestion.channel import channel_fqn
from sift_py.rule.config import (
    ExpressionChannelReference,
    ExpressionChannelReferenceChannelConfig,
    RuleAction,
    RuleActionAnnotationKind,
    RuleActionCreateDataReviewAnnotation,
    RuleActionCreatePhaseAnnotation,
    RuleActionKind,
    RuleConfig,
)
from sift_py.yaml.rule import load_rule_modules


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
        named_expressions: Optional[Dict[str, str]] = None,
    ) -> List[RuleConfig]:
        """
        Loads rules from a YAML spec, and creates or updates the rules in the Sift API.
        For more on rule YAML definitions, see `sift_py.ingestion.config.yaml.spec.RuleYamlSpec`.
        """
        module_rules = load_rule_modules(paths)

        rule_configs = []
        for rule_yaml in module_rules:
            rule_name = rule_yaml["name"]

            # First parse channel references
            yaml_channel_references = rule_yaml.get("channel_references", [])

            rule_channel_references: List[
                Union[ExpressionChannelReference, ExpressionChannelReferenceChannelConfig]
            ] = []

            for channel_ref in yaml_channel_references:
                for ref, channel_config in channel_ref.items():
                    if isinstance(channel_config, dict):
                        name = channel_config.get("name", "")
                        # NOTE: Component deprecated, but warning is thrown in the channel_fqn below
                        component = channel_config.get("component")
                    elif isinstance(channel_config, str):
                        channel_reference = channel_reference_from_fqn(channel_config)
                        name = _channel_fqn(
                            name=channel_reference.name, component=channel_reference.component
                        )
                        component = None
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

            if not rule_channel_references:
                raise ValueError(f"Rule of name '{rule_yaml['name']}' requires channel_references")

            # Parse expression for named expressions and sub expressions
            expression = rule_yaml["expression"]
            if isinstance(expression, dict):
                expression_name = expression.get("name", "")
                if not named_expressions:
                    raise ValueError(
                        f"Rule '{rule_name}' requires named expressions, but none were provided."
                    )
                expression = named_expressions.get(expression_name, "")
                if not expression:
                    raise ValueError(f"Named expression '{expression_name}' could not be found.")

            yaml_subexprs = rule_yaml.get("sub_expressions", [])
            subexpr: Dict[str, Any] = {}
            for sub in yaml_subexprs:
                for iden, value in sub.items():
                    subexpr[iden] = value

            # Create rule actions
            tags = rule_yaml.get("tags", [])
            annotation_type = RuleActionAnnotationKind.from_str(rule_yaml["type"])
            action: RuleAction = RuleActionCreatePhaseAnnotation(tags)
            if annotation_type == RuleActionAnnotationKind.REVIEW:
                action = RuleActionCreateDataReviewAnnotation(
                    assignee=rule_yaml.get("assignee"),
                    tags=tags,
                )

            # Append rule config to list
            rule_configs.append(
                RuleConfig(
                    name=rule_name,
                    rule_client_key=rule_yaml.get("rule_client_key"),
                    description=rule_yaml.get("description", ""),
                    expression=str(expression),
                    action=action,
                    channel_references=rule_channel_references,
                    asset_names=rule_yaml.get("asset_names", []),
                    sub_expressions=subexpr,
                )
            )

        # Create all the rules
        for rule_config in rule_configs:
            self.create_or_update_rule(rule_config)

        return rule_configs

    def create_or_update_rules(self, rule_configs: List[RuleConfig]):
        """
        Create or update a list of rules via a list of RuleConfigs.
        See `sift_py.rule.config.RuleConfig` for more information on configuation parameters for rules.
        """
        for config in rule_configs:
            self.create_or_update_rule(config)

    def attach_asset(self, rule: Union[str, RuleConfig], asset_names: List[str]) -> RuleConfig:
        """
        Associates a rule with an asset by name. The asset must already exist in the Sift API.
        The provided rule may either be a rule client key, rule id, or a RuleConfig.
        """
        return self._attach_or_detach_asset(rule, asset_names, attach=True)

    def detach_asset(self, rule: Union[str, RuleConfig], asset_names: List[str]) -> RuleConfig:
        """
        Disassociates a rule from an asset by name. The asset must already exist in the Sift API.
        The provided rule may either be a rule client key, rule id, or a RuleConfig.
        """
        return self._attach_or_detach_asset(rule, asset_names, attach=False)

    def _attach_or_detach_asset(
        self, rule: Union[str, RuleConfig], asset_names: List[str], attach: bool
    ) -> RuleConfig:
        assets = self._get_assets(names=asset_names)
        if not assets:
            raise ValueError(
                f"Cannot find all assets in list '{asset_names}'. One of these assets does not exist."
            )

        if isinstance(rule, str):
            rule = cast(RuleConfig, self.get_rule(rule))

        if attach:
            if not rule.asset_names:
                rule.asset_names = asset_names
            else:
                rule.asset_names.extend(asset_names)
        else:
            rule.asset_names = list(set(rule.asset_names) ^ set(asset_names))

        if not rule.asset_names:
            raise ValueError(f"Rule '{rule.name}' must be associated with at least one asset.")

        req = self._update_req_from_rule_config(rule)
        self._rule_service_stub.UpdateRule(req)

        return rule

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

    def _update_req_from_rule(self, rule: Rule) -> UpdateRuleRequest:
        return UpdateRuleRequest(
            organization_id=rule.organization_id,
            rule_id=rule.rule_id,
            client_key=rule.client_key,
            name=rule.name,
            description=rule.description,
            conditions=[
                UpdateConditionRequest(
                    rule_condition_id=condition.rule_condition_id,
                    actions=[
                        UpdateActionRequest(
                            rule_action_id=action.rule_action_id,
                            action_type=action.action_type,
                            configuration=action.configuration,
                        )
                        for action in condition.actions
                    ],
                    expression=condition.expression,
                )
                for condition in rule.conditions
            ],
            asset_configuration=RuleAssetConfiguration(
                asset_ids=rule.asset_configuration.asset_ids,
            ),
        )

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

        # TODO: once we have TagService_ListTags we can do asset-agnostic rules via tags
        assets = self._get_assets(names=config.asset_names) if config.asset_names else None

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
                            annotation_type=AnnotationType.ANNOTATION_TYPE_DATA_REVIEW,
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
                            annotation_type=AnnotationType.ANNOTATION_TYPE_PHASE,
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
            names = [
                _channel_fqn(name=ident.name, component=ident.component)
                for ident in channel_references.values()
            ]

            # Create CEL search filters
            name_in = cel_in("name", names)

            # Validate channels are present within each asset
            for asset in assets:
                found_channels = get_channels(
                    channel_service=self._channel_service_stub,
                    filter=f"asset_id == '{asset.asset_id}' && {name_in}",
                )
                found_channels_names = [channel.name for channel in found_channels]

                missing_channels = set(names) ^ set(found_channels_names)
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

    def get_rule(self, rule: str) -> Optional[RuleConfig]:
        """
        Get a rule by rule id or client key. Returns a RuleConfig if the rule exists, otherwise None.
        """
        rule_pb = self._get_rule_from_client_key(rule) or self._get_rule_from_rule_id(rule)
        if not rule_pb:
            return None

        channel_references: List[ExpressionChannelReference] = []
        expression = ""
        action: Optional[
            Union[RuleActionCreateDataReviewAnnotation, RuleActionCreatePhaseAnnotation]
        ] = None
        for condition in rule_pb.conditions:
            expression = condition.expression.calculated_channel.expression
            for ref, id in condition.expression.calculated_channel.channel_references.items():
                channel_references.append(
                    {
                        "channel_reference": ref,
                        "channel_identifier": id.name,
                    }
                )
            for action_config in condition.actions:
                annotation_type = action_config.configuration.annotation.annotation_type
                if annotation_type == AnnotationType.ANNOTATION_TYPE_PHASE:
                    action = RuleActionCreatePhaseAnnotation(
                        tags=[tag for tag in action_config.configuration.annotation.tag_ids],
                    )
                else:
                    assignee = action_config.configuration.annotation.assigned_to_user_id
                    action = RuleActionCreateDataReviewAnnotation(
                        assignee=assignee,
                        tags=[tag for tag in action_config.configuration.annotation.tag_ids],
                    )

        assets = self._get_assets(
            ids=[asset_id for asset_id in rule_pb.asset_configuration.asset_ids]
        )
        asset_names = [asset.name for asset in assets]

        rule_config = RuleConfig(
            name=rule_pb.name,
            description=rule_pb.description,
            rule_client_key=rule_pb.client_key,
            channel_references=channel_references,  # type: ignore
            asset_names=asset_names,
            action=action,
            expression=expression,
        )

        return rule_config

    def _get_rule_from_client_key(self, client_key: str) -> Optional[Rule]:
        req = GetRuleRequest(client_key=client_key)
        try:
            res = cast(GetRuleResponse, self._rule_service_stub.GetRule(req))
            return res.rule or None
        except:
            return None

    def _get_rule_from_rule_id(self, rule_id: str) -> Optional[Rule]:
        req = GetRuleRequest(rule_id=rule_id)
        try:
            res = cast(GetRuleResponse, self._rule_service_stub.GetRule(req))
            return res.rule or None
        except:
            return None

    def _get_assets(self, names: List[str] = [], ids: List[str] = []) -> List[Asset]:
        def get_assets_with_filter(cel_filter: str):
            assets: List[Asset] = []
            next_page_token = ""
            while True:
                req = ListAssetsRequest(
                    filter=cel_filter,
                    page_size=1_000,
                    page_token=next_page_token,
                )
                res = cast(ListAssetsResponse, self._asset_service_stub.ListAssets(req))
                assets.extend(res.assets)

                if not res.next_page_token:
                    break
                next_page_token = res.next_page_token

            return assets

        if names:
            names_cel = cel_in("name", names)
            return get_assets_with_filter(names_cel)
        elif ids:
            ids_cel = cel_in("asset_id", ids)
            return get_assets_with_filter(ids_cel)
        else:
            return []


@dataclass
class RuleChannelReference:
    """
    Convenient wrapper to map rule names to relevant channel references
    when creating rules from yaml.
    """

    rule_name: str
    channel_references: Dict[str, Any]
