"""
Formal specification of the types that `sift_py` expects when loading a telemetry config from a YAML file.
"""

from __future__ import annotations

from typing import Dict, List, Literal, TypedDict, Union

from typing_extensions import NotRequired


class TelemetryConfigYamlSpec(TypedDict):
    """
    Formal spec that defines what the telemetry config should look like in YAML.

    `asset_name`: The name of the asset to telemeter.
    `ingestion_client_key`: User-defined string-key that uniquely identifies this telemetry config.
    `organization_id`: Optional ID of user's organization. Required if user belongs to multiple orgs.
    `channels`: Sensors that send the data.
    `rules`: Rules that, when evaluated to a true, will perform some sort of acction.
    `flows`: A list of named groups of channels that send data together.
    """

    asset_name: str
    ingestion_client_key: str
    organization_id: NotRequired[str]
    channels: Dict[str, ChannelConfigYamlSpec]
    rules: NotRequired[List[RuleYamlSpec]]
    flows: NotRequired[List[FlowYamlSpec]]


class ChannelConfigYamlSpec(TypedDict):
    """
    Formal spec that defines what a channel should look like in YAML.

    `name`: Name of channel.
    `description`: Optional channel description.
    `unit`: Unit of measurement.
    `component`: Name of component that channel belongs to.
    `data_type`: Type of the data associated with the channel.
    `enum_types`: Required if `data_type` is `enum.
    `bit_field_elements`: Required if `data_type` is `bit_field`.
    """

    name: str
    description: NotRequired[str]
    unit: NotRequired[str]
    component: NotRequired[str]
    data_type: Union[
        Literal["double"],
        Literal["string"],
        Literal["enum"],
        Literal["bit_field"],
        Literal["bool"],
        Literal["float"],
        Literal["int32"],
        Literal["int64"],
        Literal["uint32"],
        Literal["uint64"],
    ]
    enum_types: NotRequired[List[ChannelEnumTypeYamlSpec]]
    bit_field_elements: NotRequired[List[ChannelBitFieldElementYamlSpec]]


class ChannelEnumTypeYamlSpec(TypedDict):
    """
    Formal spec that defines what a channel enum type should look like in YAML.
    """

    name: str
    key: int


class ChannelBitFieldElementYamlSpec(TypedDict):
    """
    Formal spec that defines what a bit-field element should look like in YAML.
    """

    name: str
    index: int
    bit_count: int


class FlowYamlSpec(TypedDict):
    """
    Formal spec that defines what a flow should look like in YAML.
    """

    name: str
    channels: List[ChannelConfigYamlSpec]


class RuleYamlSpec(TypedDict):
    """
    The formal definition of what a single rule looks like in YAML.

    `name`: Name of the rule.
    `description`: Description of rule.
    `expression`:
        Either an expression-string or a `sift_py.ingestion.config.yaml.spec.NamedExpressionYamlSpec` referencing a named expression.
    `type`: Determines the action to perform if a rule gets evaluated to true.
    `assignee`: If `type` is `review`, determines who to notify. Expects an email.
    `tags`: Tags to associate with the rule.
    `channel_references`: A list of channel references that maps to an actual channel. More below.
    `sub_expressions`: A list of sub-expressions which is a mapping of place-holders to sub-expressions. Only used if using named expressions.

    Channel references:
    A channel reference is a string containing a numerical value prefixed with "$". Examples include "$1", "$2", "$11", and so on.
    The channel reference is mapped to an actual channel config. In YAML it would look something like this:

    ```yaml
    channel_references:
      - $1: *vehicle_state_channel
      - $2: *voltage_channel
    ```

    Sub-expressions:
    A sub-expression is made up of two components: A reference and the actual sub-expression. The sub-expression reference is
    a string with a "$" prepended to another string comprised of characters in the following character set: `[a-zA-Z0-9_]`.
    This reference should be mapped to the actual sub-expression. For example, say you have kinematic equations in `kinematics.yml`,
    and the equation you're interested in using looks like the following:

    ```yaml
    kinetic_energy_gt:
      0.5 * $mass * $1 * $1 > $threshold
    ```

    To properly use `kinetic_energy_gt` in your rule, it would look like the following:

    ```yaml
    rules:
      - name: kinetic_energy
        description: Tracks high energy output while in motion
        type: review
        assignee: bob@example.com
        expression:
          name: kinetic_energy_gt
        channel_references:
          - $1: *velocity_channel
        sub_expressions:
          - $mass: 10
          - $threshold: 470
        tags:
            - nostromo
    ```
    """

    name: str
    description: NotRequired[str]
    expression: Union[str, NamedExpressionYamlSpec]
    type: Union[Literal["phase"], Literal["review"]]
    assignee: NotRequired[str]
    tags: NotRequired[List[str]]
    channel_references: NotRequired[List[Dict[str, ChannelConfigYamlSpec]]]
    sub_expressions: NotRequired[List[Dict[str, str]]]


class NamedExpressionYamlSpec(TypedDict):
    """
    A named expression. This class is the formal definition of what a named expression
    should look like in YAML. The value of `name` may contain a mix of channel references
    and channel identifiers.

    For a formal definition of channel references and channel identifiers see the following:
    `sift_py.ingestion.config.yaml.spec.RuleYamlSpec`.
    """

    name: str


class YamlConfigError(Exception):
    """
    When the YAML config has missing or invalid properties.
    """

    message: str

    def __init__(self, message: str):
        super().__init__(message)
