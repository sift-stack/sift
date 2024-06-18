from __future__ import annotations
from ..channel import ChannelValue, channel_fqn, is_data_type, empty_value
from ..flow import FlowConfig
from ..rule.config import RuleConfig
from .ingestion_config import (
    get_ingestion_config_by_client_key,
    create_ingestion_config,
)
from ..config.telemetry import TelemetryConfig
from ...grpc.transport import SiftChannel
from sift.ingestion_configs.v1.ingestion_configs_pb2 import IngestionConfig
from sift.ingest.v1.ingest_pb2 import (
    IngestWithConfigDataChannelValue,
    IngestWithConfigDataStreamRequest,
)
from sift.ingest.v1.ingest_pb2_grpc import IngestServiceStub
from sift.runs.v2.runs_pb2 import CreateRunRequest, CreateRunResponse
from sift.runs.v2.runs_pb2_grpc import RunServiceStub
from sift.rules.v1.rules_pb2 import (
    UpdateJsonRulesRequest,
    UpdateJsonRulesResponse,
    JsonRulesRequest,
)
from sift.rules.v1.rules_pb2_grpc import RuleServiceStub
from google.protobuf.timestamp_pb2 import Timestamp
from typing import cast, Dict, List, Optional
from datetime import datetime

import json


class IngestionServiceImpl:
    transport_channel: SiftChannel
    ingestion_config: IngestionConfig
    asset_name: str

    # TODO: Multiple flows can have the same name if their channel configs differ...
    flow_configs_by_name: Dict[str, FlowConfig]
    rules: List[RuleConfig]

    run_id: Optional[str]
    organization_id: Optional[str]
    end_stream_on_error: bool

    def __init__(
        self,
        channel: SiftChannel,
        config: TelemetryConfig,
        run_id: Optional[str] = None,
        end_stream_on_error: bool = False,
    ):
        self.ingestion_config = self.__class__.get_or_create_ingestion_config(channel, config)
        self.asset_name = config.asset_name
        self.transport_channel = channel
        self.run_id = run_id
        self.organization_id = config.organization_id
        self.end_stream_on_error = end_stream_on_error

        # TODO... flows can have the same name...
        self.flow_configs_by_name = {flow.name: flow for flow in config.flows}

        self.__class__.update_rules(
            channel, self.ingestion_config.asset_id, config.rules, config.organization_id
        )
        self.rules = config.rules

    def ingest(self, *requests: IngestWithConfigDataStreamRequest):
        # TODO: Add logic to re-establish connection if channel has been closed due to idle timeout

        svc = IngestServiceStub(self.transport_channel)
        svc.IngestWithConfigDataStream(iter(requests))

    def start_run(
        self,
        channel: SiftChannel,
        run_name: str,
        description: Optional[str] = None,
        organization_id: Optional[str] = None,
        tags: Optional[List[str]] = None,
    ):
        svc = RunServiceStub(channel)
        req = CreateRunRequest(
            name=run_name,
            description=description or "",
            organization_id=organization_id or "",
            tags=tags,
        )
        res = cast(CreateRunResponse, svc.CreateRun(req))
        self.run_id = res.run.run_id

    def try_create_ingestion_request(
        self,
        flow_name: str,
        timestamp: datetime,
        channel_values: List[ChannelValue],
    ) -> IngestWithConfigDataStreamRequest:
        flow_config = self.flow_configs_by_name.get(flow_name)

        if flow_config is None:
            raise ValueError(f"A flow config of name '{flow_name}' could not be found.")

        channel_values_by_fqn: Dict[str, ChannelValue] = {}

        for channel_value in channel_values:
            fqn = channel_fqn(channel_value)

            if channel_values_by_fqn.get(fqn, None) is None:
                channel_values_by_fqn[fqn] = channel_value
            else:
                raise ValueError(f"Encountered multiple values for {fqn}")

        values: List[IngestWithConfigDataChannelValue] = []

        for channel in flow_config.channels:
            fqn = channel_fqn(channel)
            channel_value = channel_values_by_fqn.pop(fqn, None)

            if channel_value is None:
                values.append(empty_value())
                continue

            value = channel_value["value"]

            if is_data_type(value, channel.data_type):
                values.append(value)
            else:
                raise ValueError(
                    f"Expected value for `{channel.name}` to be a '{channel.data_type}'."
                )

        if len(channel_values_by_fqn) > 0:
            unexpected_channels = [name for name in channel_values_by_fqn.keys()]
            raise ValueError(
                f"Unexpected channels for flow '{flow_name}' or 'component' field missing for channel: {unexpected_channels}"
            )

        if timestamp.tzname() != "UTC":
            raise ValueError(
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
    def update_rules(
        channel: SiftChannel,
        asset_id: str,
        rule_configs: List[RuleConfig],
        organization_id: Optional[str] = None,
    ):
        svc = RuleServiceStub(channel)
        json_rules = json.dumps(rule_configs, default=lambda x: x.as_json())
        req = UpdateJsonRulesRequest(
            request=JsonRulesRequest(
                asset_id=asset_id,
                rules_json=json_rules,
                organization_id=organization_id or "",
            )
        )
        res = cast(UpdateJsonRulesResponse, svc.UpdateJsonRules(req))
        if not res.response.success:
            raise Exception(f"Failed to load rules: {res.response.error_messages}")

    @staticmethod
    def get_or_create_ingestion_config(channel: SiftChannel, config: TelemetryConfig):
        # TODO: Handle case where new Flows are added to an existing ingestion config
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
