from __future__ import annotations

from datetime import datetime
from typing import Dict, List, Optional

from google.protobuf.timestamp_pb2 import Timestamp
from sift.ingest.v1.ingest_pb2 import (
    IngestWithConfigDataChannelValue,
    IngestWithConfigDataStreamRequest,
)
from sift.ingest.v1.ingest_pb2_grpc import IngestServiceStub
from sift.ingestion_configs.v1.ingestion_configs_pb2 import IngestionConfig

from sift_py.grpc.transport import SiftChannel
from sift_py.ingestion._internal.error import IngestionValidationError
from sift_py.ingestion._internal.ingestion_config import (
    create_flow_configs,
    create_ingestion_config,
    get_ingestion_config_by_client_key,
    get_ingestion_config_flow_names,
)
from sift_py.ingestion._internal.rule import get_asset_rules_json, update_rules
from sift_py.ingestion._internal.run import create_run, get_run_id_by_name
from sift_py.ingestion.channel import (
    ChannelValue,
    channel_fqn,
    empty_value,
    is_data_type,
)
from sift_py.ingestion.config.telemetry import TelemetryConfig
from sift_py.ingestion.flow import FlowConfig
from sift_py.ingestion.rule.config import RuleConfig


class _IngestionServiceImpl:
    transport_channel: SiftChannel
    ingestion_config: IngestionConfig
    asset_name: str
    flow_configs_by_name: Dict[str, FlowConfig]
    rules: List[RuleConfig]
    run_id: Optional[str]
    organization_id: Optional[str]
    overwrite_rules: bool
    end_stream_on_error: bool

    def __init__(
        self,
        channel: SiftChannel,
        config: TelemetryConfig,
        run_id: Optional[str] = None,
        overwrite_rules: bool = False,
        end_stream_on_error: bool = False,
    ):
        ingestion_config = self.__class__._get_or_create_ingestion_config(channel, config)

        self.__class__._update_flow_configs(
            channel, ingestion_config.ingestion_config_id, config.flows
        )

        if not overwrite_rules:
            self.__class__._validate_rules_synchronized(
                channel, ingestion_config.asset_id, config.rules
            )

        if len(config.rules) > 0:
            update_rules(channel, ingestion_config.asset_id, config.rules)

        self.rules = config.rules
        self.ingestion_config = ingestion_config
        self.asset_name = config.asset_name
        self.transport_channel = channel
        self.run_id = run_id
        self.organization_id = config.organization_id
        self.end_stream_on_error = end_stream_on_error
        self.flow_configs_by_name = {flow.name: flow for flow in config.flows}

    def ingest(self, *requests: IngestWithConfigDataStreamRequest):
        """
        Perform data ingestion.
        """
        svc = IngestServiceStub(self.transport_channel)
        svc.IngestWithConfigDataStream(iter(requests))

    def attach_run(
        self,
        channel: SiftChannel,
        run_name: str,
        description: Optional[str] = None,
        organization_id: Optional[str] = None,
        tags: Optional[List[str]] = None,
    ):
        """
        Retrieve an existing run or create one to use during this period of ingestion.
        """
        run_id = get_run_id_by_name(channel, run_name)

        if run_id is not None:
            self.run_id = run_id
            return

        self.run_id = create_run(
            channel=channel,
            run_name=run_name,
            description=description or "",
            organization_id=organization_id or "",
            tags=tags or [],
        )

    def detach_run(self):
        """
        Detach run from this period of ingestion. Subsequent data ingested won't be associated with
        the run being detached.
        """
        self.run_id = None

    def try_create_ingestion_request(
        self,
        flow_name: str,
        timestamp: datetime,
        channel_values: List[ChannelValue],
    ) -> IngestWithConfigDataStreamRequest:
        """
        Creates an ingestion request for a flow that must exist in `flow_configs_by_name`. This method
        performs a series of client-side validations and will return a `IngestionValidationError` if any validations fail.
        """
        flow_config = self.flow_configs_by_name.get(flow_name)

        if flow_config is None:
            raise IngestionValidationError(
                f"A flow config of name '{flow_name}' could not be found."
            )

        channel_values_by_fqn: Dict[str, ChannelValue] = {}

        for channel_value in channel_values:
            fqn = channel_fqn(channel_value)

            if channel_values_by_fqn.get(fqn, None) is None:
                channel_values_by_fqn[fqn] = channel_value
            else:
                raise IngestionValidationError(f"Encountered multiple values for {fqn}")

        values: List[IngestWithConfigDataChannelValue] = []

        for channel in flow_config.channels:
            fqn = channel_fqn(channel)
            channel_val: Optional[ChannelValue] = channel_values_by_fqn.pop(fqn, None)

            if channel_val is None:
                values.append(empty_value())
                continue

            value = channel_val["value"]

            if is_data_type(value, channel.data_type):
                values.append(value)
            else:
                raise IngestionValidationError(
                    f"Expected value for `{channel.name}` to be a '{channel.data_type}'."
                )

        if len(channel_values_by_fqn) > 0:
            unexpected_channels = [name for name in channel_values_by_fqn.keys()]
            raise IngestionValidationError(
                f"Unexpected channel(s) for flow '{flow_name}': {unexpected_channels}"
            )

        if timestamp.tzname() != "UTC":
            raise IngestionValidationError(
                f"Expected 'timestamp' to be in UTC but it is in {timestamp.tzname()}."
            )

        timestamp_pb = Timestamp()
        timestamp_pb.FromDatetime(timestamp)

        return self.create_ingestion_request(flow_name, timestamp, values)

    def create_ingestion_request(
        self,
        flow_name: str,
        timestamp: datetime,
        channel_values: List[IngestWithConfigDataChannelValue],
    ) -> IngestWithConfigDataStreamRequest:
        """
        Creates an ingestion request for a flow that must exist in `flow_configs_by_name`. This method
        does not do any sort of client-side validation and is recommended to use if performance is required.
        """
        timestamp_pb = Timestamp()
        timestamp_pb.FromDatetime(timestamp)

        return IngestWithConfigDataStreamRequest(
            ingestion_config_id=self.ingestion_config.ingestion_config_id,
            flow=flow_name,
            timestamp=timestamp_pb,
            channel_values=channel_values,
            run_id=self.run_id or "",
            organization_id=self.organization_id or "",
            end_stream_on_validation_error=self.end_stream_on_error,
        )

    @staticmethod
    def _update_flow_configs(
        channel: SiftChannel, ingestion_config_id: str, flows: List[FlowConfig]
    ):
        """
        Queries flow configs from Sift and does a check to see if there are any new flow configs that need to be created.
        """
        if len(flows) == 0:
            return

        registered_flow_names = set(get_ingestion_config_flow_names(channel, ingestion_config_id))

        flows_to_create = []

        for flow in flows:
            if flow.name in registered_flow_names:
                continue

            flows_to_create.append(flow)

        if len(flows_to_create) > 0:
            create_flow_configs(channel, ingestion_config_id, flows_to_create)

    @staticmethod
    def _get_or_create_ingestion_config(
        channel: SiftChannel, config: TelemetryConfig
    ) -> IngestionConfig:
        """
        Retrieves an existing ingestion config or creates a new one.
        """

        ingestion_config = get_ingestion_config_by_client_key(channel, config.ingestion_client_key)

        if ingestion_config is not None:
            return ingestion_config

        return create_ingestion_config(
            channel,
            config.asset_name,
            config.flows,
            config.ingestion_client_key,
            config.organization_id,
        )

    @staticmethod
    def _validate_rules_synchronized(
        transport_channel: SiftChannel,
        asset_id: str,
        rule_configs: List[RuleConfig],
    ):
        """
        Ensures that rules defined in the telemetry config and the rules in Sift are in sync, otherwise error.
        Namely, if a rule was added via a Sift UI and wasn't added immediately to the telemetry config, then
        this will raise an exception.
        """
        if len(rule_configs) == 0:
            return

        rules_json = get_asset_rules_json(transport_channel, asset_id)

        rule_names_from_config = set()

        for rule_config in rule_configs:
            rule_names_from_config.add(rule_config.name)

        for rule_json in rules_json:
            rule_name: str = rule_json.get("name", "")

            if len(rule_name) == 0:
                raise IngestionValidationError("Encountered rule without a name from Sift API.")

            if rule_name not in rule_names_from_config:
                raise IngestionValidationError(
                    f"Encountered rule '{rule_name}' on asset '{asset_id}' not found in local telemetry config. Add it."
                )
