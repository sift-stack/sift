from __future__ import annotations

from pathlib import Path
from typing import Any, Dict, List, Optional, cast

from typing_extensions import Self

from sift_py.ingestion.channel import (
    ChannelBitFieldElement,
    ChannelConfig,
    ChannelDataType,
    ChannelEnumType,
    _channel_fqn,
)
from sift_py.ingestion.config.yaml.load import load_named_expression_modules, read_and_validate
from sift_py.ingestion.config.yaml.spec import TelemetryConfigYamlSpec
from sift_py.ingestion.flow import FlowConfig
from sift_py.ingestion.rule.config import (
    ExpressionChannelReference,
    ExpressionChannelReferenceChannelConfig,
    RuleAction,
    RuleActionAnnotationKind,
    RuleActionCreateDataReviewAnnotation,
    RuleActionCreatePhaseAnnotation,
    RuleConfig,
)


class TelemetryConfig:
    """
    Configurations necessary to start ingestion.

    Attributes:
        `asset_name`:
            The name of the asset that you wish to telemeter data for.
        `ingestion_client_key`:
            An arbitrary string chosen by the user to uniquely identify this ingestion configuration.
        `flows`:
            A single flow can specify a single channel value or a set of channel values that are ingested together.
        `organization_id`:
            ID of your organization in Sift. This field is only required if your user belongs to multiple organizations.
        `rules`:
            Rules to evaluate during ingestion.
    """

    asset_name: str
    ingestion_client_key: str
    organization_id: Optional[str]
    flows: List[FlowConfig]
    rules: List[RuleConfig]

    def __init__(
        self,
        asset_name: str,
        ingestion_client_key: str,
        organization_id: Optional[str] = None,
        flows: List[FlowConfig] = [],
        rules: List[RuleConfig] = [],
    ):
        """
        Will raise a `TelemetryConfigValidationError` under the following conditions:
            - Multiple flows with the same name
            - Multiple rules with the same name
            - Identical channels in the same flow
        """
        self.__class__.validate_flows(flows)
        self.__class__.validate_rules(rules)

        self.asset_name = asset_name
        self.ingestion_client_key = ingestion_client_key
        self.organization_id = organization_id
        self.flows = flows
        self.rules = rules

    @staticmethod
    def validate_rules(rules: List[RuleConfig]):
        """
        Ensure that there are no rules with identical names
        """
        seen_rule_names = set()

        for rule in rules:
            if rule.name in seen_rule_names:
                raise TelemetryConfigValidationError(
                    f"Can't have two rules with identical names, '{rule.name}'."
                )
            seen_rule_names.add(rule.name)

    @staticmethod
    def validate_flows(flows: List[FlowConfig]):
        """
        Ensures no duplicate channels and flows with the same name, otherwise raises a `TelemetryConfigValidationError` exception.
        """
        flow_names = set()

        for flow in flows:
            seen_channels = set()

            if flow.name in flow_names:
                raise TelemetryConfigValidationError(
                    f"Can't have two flows with the same name, '{flow.name}'."
                )

            flow_names.add(flow.name)

            for channel in flow.channels:
                fqn = channel.fqn()

                if fqn in seen_channels:
                    raise TelemetryConfigValidationError(
                        f"Can't have two identical channels, '{fqn}', in flow '{flow.name}'."
                    )
                else:
                    seen_channels.add(fqn)

    @classmethod
    def try_from_yaml(
        cls,
        path: Path,
        named_expression_modules: Optional[List[Path]] = None,
    ) -> Self:
        config_as_yaml = read_and_validate(path)

        if named_expression_modules is not None:
            named_expressions = load_named_expression_modules(named_expression_modules)
            return cls._from_yaml(config_as_yaml, named_expressions)
        else:
            return cls._from_yaml(config_as_yaml)

    @classmethod
    def _from_yaml(
        cls,
        config_as_yaml: TelemetryConfigYamlSpec,
        named_expressions: Dict[str, str] = {},
    ) -> Self:
        rules = []
        flows = []

        for flow in config_as_yaml.get("flows", []):
            channels = []

            for channel in flow["channels"]:
                data_type = cast(ChannelDataType, ChannelDataType.from_str(channel["data_type"]))

                bit_field_elements = []
                for bit_field_element in channel.get("bit_field_elements", []):
                    bit_field_elements.append(
                        ChannelBitFieldElement(
                            name=bit_field_element["name"],
                            index=bit_field_element["index"],
                            bit_count=bit_field_element["bit_count"],
                        )
                    )

                enum_types = []
                for enum_type in channel.get("enum_types", []):
                    enum_types.append(
                        ChannelEnumType(
                            name=enum_type["name"],
                            key=enum_type["key"],
                        )
                    )

                channels.append(
                    ChannelConfig(
                        name=channel["name"],
                        data_type=data_type,
                        description=channel.get("description"),
                        unit=channel.get("unit"),
                        component=channel.get("component"),
                        bit_field_elements=bit_field_elements,
                        enum_types=enum_types,
                    )
                )

            flows.append(
                FlowConfig(
                    name=flow["name"],
                    channels=channels,
                )
            )

        for rule in config_as_yaml.get("rules", []):
            annotation_type = RuleActionAnnotationKind.from_str(rule["type"])

            tags = rule.get("tags")

            action: RuleAction = RuleActionCreatePhaseAnnotation(tags)
            if annotation_type == RuleActionAnnotationKind.REVIEW:
                action = RuleActionCreateDataReviewAnnotation(
                    assignee=rule.get("assignee"),
                    tags=tags,
                )

            channel_references: List[
                ExpressionChannelReference | ExpressionChannelReferenceChannelConfig
            ] = []

            for channel_reference in rule.get("channel_references", []):
                for ref, val in channel_reference.items():
                    name = val["name"]
                    component = val.get("component")

                    channel_references.append(
                        {
                            "channel_reference": ref,
                            "channel_identifier": _channel_fqn(name, component),
                        }
                    )

            expression = rule["expression"]
            if isinstance(expression, str):
                rules.append(
                    RuleConfig(
                        name=rule["name"],
                        description=rule.get("description", ""),
                        expression=expression,
                        action=action,
                        channel_references=channel_references,
                    )
                )
            else:
                expression_name = cast(str, expression.get("name"))

                expr = named_expressions.get(expression_name)

                if expr is None:
                    raise TelemetryConfigValidationError(
                        f"Named expression '{expression_name}' could not be found. Make sure it was loaded in."
                    )

                sub_expressions = rule.get("sub_expressions", [])

                sub_exprs: Dict[str, Any] = {}
                for sub_expression in sub_expressions:
                    for iden, value in sub_expression.items():
                        sub_exprs[iden] = value

                rules.append(
                    RuleConfig(
                        name=rule["name"],
                        description=rule.get("description", ""),
                        expression=expr,
                        action=action,
                        channel_references=channel_references,
                        sub_expressions=sub_exprs,
                    )
                )

        return cls(
            asset_name=config_as_yaml["asset_name"],
            ingestion_client_key=config_as_yaml["ingestion_client_key"],
            organization_id=config_as_yaml.get("organization_id"),
            rules=rules,
            flows=flows,
        )


class TelemetryConfigValidationError(Exception):
    """
    When the telemetry config has invalid properties
    """

    message: str

    def __init__(self, message: str):
        super().__init__(message)
