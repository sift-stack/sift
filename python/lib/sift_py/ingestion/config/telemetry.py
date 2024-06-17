from __future__ import annotations
from ..flow import FlowConfig
from ..rule.config import RuleConfig
from typing import Dict, List, Optional


class TelemetryConfig:
    """
    Configurations necessary to start ingestion.

    Attributes:
      `asset_name`: The name of the asset that you wish to telemeter data for.
      `ingestion_client_key`: An arbitrary string completely chosen by the user to uniquely identify
        this ingestion configuration. It should be unique with respect to your organization.

      `flows`: The list of `FlowConfig`. A single flow can specify a single channel value
        or a set of channel values, with each value belonging to a different channel. Channels
        that send data at the same frequency and time should be in the same flow.

      `organization_id`: ID of your organization in Sift. This field is only required if your user
        belongs to multiple organizations

      `rules`: Rules to evaluate during ingestion.
      `named_expressions`: Reusable expressions used to generate rules. The key is the name of the expression and the value
        of said key is the actual generic expression.
    """

    asset_name: str
    ingestion_client_key: str
    organization_id: Optional[str]
    flows: List[FlowConfig]
    rules: List[RuleConfig]

    named_expressions: Dict[str, str]

    def __init__(
        self,
        asset_name: str,
        ingestion_client_key: str,
        organization_id: Optional[str] = None,
        flows: List[FlowConfig] = [],
        rules: List[RuleConfig] = [],
        named_expressions: Dict[str, str] = {},
    ):
        # TODO: Add validation logic here as well.

        self.asset_name = asset_name
        self.ingestion_client_key = ingestion_client_key
        self.organization_id = organization_id
        self.flows = flows
        self.rules = rules
        self.named_expressions = named_expressions
