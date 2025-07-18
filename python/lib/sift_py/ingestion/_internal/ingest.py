from __future__ import annotations

import logging
from collections.abc import Callable
from datetime import datetime
from typing import Any, Dict, List, Optional, Union, cast

from google.protobuf.timestamp_pb2 import Timestamp
from sift.ingest.v1.ingest_pb2 import (
    IngestWithConfigDataChannelValue,
    IngestWithConfigDataStreamRequest,
)
from sift.ingest.v1.ingest_pb2_grpc import IngestServiceStub
from sift.ingestion_configs.v2.ingestion_configs_pb2 import ChannelConfig as ChannelConfigPb
from sift.ingestion_configs.v2.ingestion_configs_pb2 import IngestionConfig

from sift_py.grpc.transport import SiftChannel
from sift_py.ingestion._internal.error import IngestionValidationError
from sift_py.ingestion._internal.ingestion_config import (
    create_flow_configs,
    create_ingestion_config,
    get_ingestion_config_by_client_key,
    get_ingestion_config_flows,
)
from sift_py.ingestion._internal.run import create_run, get_run_id_by_name
from sift_py.ingestion.channel import (
    ChannelConfig,
    ChannelValue,
    channel_fqn,
    empty_value,
    is_data_type,
)
from sift_py.ingestion.config.telemetry import TelemetryConfig
from sift_py.ingestion.flow import Flow, FlowConfig, FlowOrderedChannelValues
from sift_py.ingestion.rule.config import RuleConfig
from sift_py.rule.service import RuleService

logger = logging.getLogger(__name__)


class _IngestionServiceImpl:
    transport_channel: SiftChannel
    ingestion_config: IngestionConfig
    asset_name: str
    flow_configs_by_name: Dict[str, FlowConfig]
    rules: List[RuleConfig]
    run_id: Optional[str]
    organization_id: Optional[str]
    end_stream_on_error: bool
    config: TelemetryConfig

    ingest_service_stub: IngestServiceStub
    rule_service: RuleService

    def __init__(
        self,
        channel: SiftChannel,
        config: TelemetryConfig,
        run_id: Optional[str] = None,
        end_stream_on_error: bool = False,
    ):
        ingestion_config = self.__class__._get_or_create_ingestion_config(channel, config)
        self.ingestion_config = ingestion_config

        if config._ingestion_client_key_is_generated:
            # If this is a generated key, use the local telemetry config since it is static.
            self.flow_configs_by_name = {flow.name: flow for flow in config.flows}
        else:
            # If the user specified a client key, use the configuration from Sift since it
            # may have been updated.
            flows = [
                FlowConfig.from_pb(f)
                for f in get_ingestion_config_flows(channel, ingestion_config.ingestion_config_id)
            ]
            self.flow_configs_by_name = {flow.name: flow for flow in flows}

        self.rule_service = RuleService(channel)
        if config.rules:
            for rule in config.rules:
                if config.asset_name not in rule.asset_names:
                    rule.asset_names.append(config.asset_name)
            self.rule_service.create_or_update_rules(config.rules)

        self.rules = config.rules
        self.asset_name = config.asset_name
        self.transport_channel = channel
        self.run_id = run_id
        self.organization_id = config.organization_id
        self.end_stream_on_error = end_stream_on_error
        self.ingest_service_stub = IngestServiceStub(channel)
        self.config = config

    def ingest(self, *requests: IngestWithConfigDataStreamRequest):
        """
        Perform data ingestion.
        """
        self.ingest_service_stub.IngestWithConfigDataStream(iter(requests))

    def ingest_flows(self, *flows: FlowOrderedChannelValues):
        """
        Combines the requests creation step and ingestion into a single call.
        See `create_ingestion_request` for information about how client-side validations are handled.
        """

        requests = []
        for flow in flows:
            flow_name = flow["flow_name"]
            timestamp = flow["timestamp"]
            channel_values = flow["channel_values"]
            req = self.create_ingestion_request(flow_name, timestamp, channel_values)
            requests.append(req)

        self.ingest_service_stub.IngestWithConfigDataStream(iter(requests))

    def try_ingest_flows(self, *flows: Flow):
        """
        Combines the requests creation step and ingestion into a single call.
        See `try_create_ingestion_request` for information about how client-side validations are handled.
        """

        requests = []
        for flow in flows:
            flow_name = flow["flow_name"]
            timestamp = flow["timestamp"]
            channel_values = flow["channel_values"]
            req = self.try_create_ingestion_request(flow_name, timestamp, channel_values)
            requests.append(req)

        self.ingest_service_stub.IngestWithConfigDataStream(iter(requests))

    def attach_run(
        self,
        channel: SiftChannel,
        run_name: str,
        description: Optional[str] = None,
        organization_id: Optional[str] = None,
        tags: Optional[List[str]] = None,
        metadata: Optional[Dict[str, Union[str, float, bool]]] = None,
        force_new: bool = False,
    ):
        """
        Retrieve an existing run or create one to use during this period of ingestion.

        Include `force_new=True` to force the creation of a new run, which will allow creation of a new run using an existing name.
        """
        if not force_new:
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
            metadata=metadata,
        )

    def detach_run(self):
        """
        Detach run from this period of ingestion. Subsequent data ingested won't be associated with
        the run being detached.
        """
        self.run_id = None

    def try_create_ingestion_request_ordered_values(
        self,
        flow_name: str,
        flow_config: FlowConfig,
        timestamp: datetime,
        channel_values: List[IngestWithConfigDataChannelValue],
    ) -> List[IngestWithConfigDataChannelValue]:
        values: List[IngestWithConfigDataChannelValue] = []

        if len(channel_values) != len(flow_config.channels):
            raise IngestionValidationError(
                f"Expected {len(flow_config.channels)} channel values, got {len(channel_values)}."
            )
        for i in range(len(channel_values)):
            channel_dict = channel_values[i]

            channel_type = list(channel_dict.keys())  # type: ignore
            if len(channel_type) != 1:
                raise ValueError(
                    f"Expected exactly one key in flow value, got keys: {channel_type}"
                )
            channel_type_key = channel_type[0]

            channel_config = flow_config.channels[i]
            try:
                chan_value = channel_config.try_value_from(channel_dict[channel_type_key])  # type: ignore
                if is_data_type(chan_value, channel_config.data_type):
                    values.append(chan_value)
            except ValueError:
                raise IngestionValidationError(
                    f"Expected value for `{flow_config.channels[i].name}` to be a '{flow_config.channels[i].data_type}'. Instead found {channel_type} in flow {flow_name}."
                )

        return values

    def try_create_ingestion_request_channel_values(
        self,
        flow_name: str,
        flow_config: FlowConfig,
        timestamp: datetime,
        channel_values: List[ChannelValue],
    ) -> List[IngestWithConfigDataChannelValue]:
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

        return values

    def _is_channel_value(self, value: Any) -> bool:
        """
        Check if a value is a ChannelValue.
        ChannelValue has a "value" field and either a "name" or "channel_name" field.
        """
        return (
            isinstance(value, dict)
            and "value" in value
            and ("name" in value or "channel_name" in value)
        )

    def _is_ingest_channel_value(self, value: Any) -> bool:
        """
        Check if a value is an IngestWithConfigDataChannelValue.
        This is a protobuf message with specific fields.
        """
        return isinstance(value, IngestWithConfigDataChannelValue) or (
            isinstance(value, dict)
            and any(
                field in value
                for field in [
                    "double",
                    "string",
                    "int32",
                    "int64",
                    "bool",
                ]
            )
        )

    def try_create_ingestion_request(
        self,
        flow_name: str,
        timestamp: datetime,
        channel_values: Union[List[ChannelValue], List[IngestWithConfigDataChannelValue]],
    ) -> IngestWithConfigDataStreamRequest:
        """
        Creates an ingestion request for a flow that must exist in `flow_configs_by_name`. This method
        performs a series of client-side validations and will return a `IngestionValidationError` if any validations fail.
        Channel values can be provided as a list of `sift_py.ingestion.channel.ChannelValue` or a list of values from a
        `sift_py.ingestion.flow.FlowOrderedChannelValues`.
        """
        flow_config = self.flow_configs_by_name.get(flow_name)
        if flow_config is None:
            raise IngestionValidationError(
                f"A flow config of name '{flow_name}' could not be found."
            )

        if not channel_values:
            raise IngestionValidationError("Channel values list cannot be empty")

        first_value = channel_values[0]
        values: List[IngestWithConfigDataChannelValue] = []

        if self._is_channel_value(first_value):
            # Handle ChannelValue list
            values = self.try_create_ingestion_request_channel_values(
                flow_name, flow_config, timestamp, cast(List[ChannelValue], channel_values)
            )
        elif self._is_ingest_channel_value(first_value):
            # Handle IngestWithConfigDataChannelValue list
            values = self.try_create_ingestion_request_ordered_values(
                flow_name,
                flow_config,
                timestamp,
                cast(List[IngestWithConfigDataChannelValue], channel_values),
            )
        else:
            raise ValueError(
                f"Unknown channel values format: {type(first_value)}. "
                "Expected either ChannelValue or IngestWithConfigDataChannelValue"
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

    def try_create_flow(self, *flow_config: FlowConfig):
        """
        Tries to create a new flow at runtime. Will raise an `IngestionValidationError` if there already exists
        a flow with the name of the `flow_config` argument.
        """
        if self.config._ingestion_client_key_is_generated:
            raise IngestionValidationError(
                "Telemetry configs with generated ingestion client keys can not be updated at runtime."
                "Use a custom ingestion client key if you want to update flows at runtime."
            )

        for fc in flow_config:
            if fc.name in self.flow_configs_by_name:
                raise IngestionValidationError(f"There is already a flow with name '{fc.name}'.")

        create_flow_configs(
            self.transport_channel,
            self.ingestion_config.ingestion_config_id,
            flow_config,
        )

        for fc in flow_config:
            self.flow_configs_by_name[fc.name] = fc

    def try_create_flows(self, *flow_configs: FlowConfig):
        """
        See `try_create_flows`.
        """
        return self.try_create_flow(*flow_configs)

    def create_flow(self, *flow_config: FlowConfig):
        """
        Like `try_create_flow` but will not do any client side validation and
        raise `IngestionValidationError`.
        """
        if self.config._ingestion_client_key_is_generated:
            raise IngestionValidationError(
                "Telemetry configs with generated ingestion client keys can not be updated at runtime."
                "Use a custom ingestion client key if you want to update flows at runtime."
            )

        create_flow_configs(
            self.transport_channel,
            self.ingestion_config.ingestion_config_id,
            flow_config,
        )
        for fc in flow_config:
            self.flow_configs_by_name[fc.name] = fc

    def create_flows(self, *flow_configs: FlowConfig):
        """
        See `create_flow`.
        """
        return self.create_flow(*flow_configs)

    @staticmethod
    def _update_flow_configs(
        channel: SiftChannel, ingestion_config_id: str, telemetry_config: TelemetryConfig
    ):
        """
        Compares local flows from a telemetry config with the flows registered in Sift. If a local flow
        contains channels that isn't in Sift then this will fail.
        """
        config_flows = telemetry_config.flows

        if len(config_flows) == 0:
            return

        sift_flows = get_ingestion_config_flows(channel, ingestion_config_id)

        sift_flow_indices_by_name = {flow.name: i for i, flow in enumerate(sift_flows)}

        flows_to_create: List[FlowConfig] = []

        # We can have multiple channels of the same name but different data-type. This will create a completely unique channel
        # identifier by creating a composite key of the fully qualified channel name with the channel's data-type.
        sift_channel_identifier: Callable[[ChannelConfigPb], str] = (
            lambda x: f"{channel_fqn(x)}.{x.data_type}"
        )
        config_channel_identifier: Callable[[ChannelConfig], str] = (
            lambda x: f"{channel_fqn(x)}.{x.data_type.value}"
        )

        for config_flow in config_flows:
            sift_flow_index = sift_flow_indices_by_name.get(config_flow.name)

            # There isn't a flow in Sift for this config flow so we'll create it.
            if sift_flow_index is None:
                flows_to_create.append(config_flow)
                continue

            # There is a flow in Sift with the name of the config flow. We'll
            # compare the channels in the config flow with the sift flow and
            # see if there's a difference. If there is we'll create a new flow.
            sift_flow = sift_flows[sift_flow_index]

            sift_channel_identifiers = {
                sift_channel_identifier(channel) for channel in sift_flow.channels
            }

            for config_channel in config_flow.channels:
                # Found a channel for this flow that doesn't exist in Sift based on channel
                # fully-qualified name and data-type. Create a new flow.
                if not config_channel_identifier(config_channel) in sift_channel_identifiers:
                    raise IngestionValidationError(
                        "Encountered duplicate flow with mismatched channels"
                    )

        if len(flows_to_create) > 0:
            create_flow_configs(channel, ingestion_config_id, flows_to_create)

    @classmethod
    def _get_or_create_ingestion_config(
        cls, channel: SiftChannel, config: TelemetryConfig
    ) -> IngestionConfig:
        """
        Retrieves an existing ingestion config or creates a new one. If an existing ingestion config is fetched,
        then flows may be updated to reflect any changes that may have occured in the telemetry config.
        """

        ingestion_config = get_ingestion_config_by_client_key(channel, config.ingestion_client_key)

        # Exiting ingestion config.. update flows if necessary
        if ingestion_config is not None:
            if config._ingestion_client_key_is_generated:
                return ingestion_config
            else:
                cls._update_flow_configs(channel, ingestion_config.ingestion_config_id, config)
                return ingestion_config

        ingestion_config = create_ingestion_config(
            channel,
            config.asset_name,
            config.flows,
            config.ingestion_client_key,
            config.organization_id,
        )

        return ingestion_config
