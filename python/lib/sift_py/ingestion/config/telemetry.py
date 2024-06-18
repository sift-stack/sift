from __future__ import annotations

from typing import List, Optional

from ..flow import FlowConfig
from ..rule.config import RuleConfig


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
        # TODO: Add validation logic here as well.

        self.asset_name = asset_name
        self.ingestion_client_key = ingestion_client_key
        self.organization_id = organization_id
        self.flows = flows
        self.rules = rules
