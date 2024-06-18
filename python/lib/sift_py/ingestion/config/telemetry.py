from __future__ import annotations

from typing import List, Optional

from sift_py.ingestion.flow import FlowConfig
from sift_py.ingestion.rule.config import RuleConfig


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
        self.__class__.validate_flows(flows)

        self.asset_name = asset_name
        self.ingestion_client_key = ingestion_client_key
        self.organization_id = organization_id
        self.flows = flows
        self.rules = rules

    @staticmethod
    def validate_flows(flows: List[FlowConfig]):
        """
        Ensures no duplicate channels in a flow, otherwise raises a `TelemetryConfigValidationError` exception.
        """
        for flow in flows:
            seen_channels = set()

            for channel in flow.channels:
                fqn = channel.fqn()

                if fqn in seen_channels:
                    raise TelemetryConfigValidationError(f"Can't have two identical channels, '{fqn}', in flow '{flow.name}'.")
                else:
                    seen_channels.add(fqn)

class TelemetryConfigValidationError(Exception):
    """
    When the telemetry config has invalid properties
    """
    message: str

    def __init__(self, message: str):
        super().__init__(message)
