from __future__ import annotations

from datetime import datetime
from typing import Dict, List, Optional

from sift.ingest.v1.ingest_pb2 import (
    IngestWithConfigDataChannelValue,
    IngestWithConfigDataStreamRequest,
)
from sift.ingestion_configs.v1.ingestion_configs_pb2 import IngestionConfig

from sift_py.grpc.transport import SiftChannel
from sift_py.ingestion._internal.ingest import _IngestionServiceImpl
from sift_py.ingestion.channel import ChannelValue
from sift_py.ingestion.config.telemetry import TelemetryConfig
from sift_py.ingestion.flow import FlowConfig


class IngestionService(_IngestionServiceImpl):
    """
    A fully configured service that, when instantiated, is ready to start ingesting data.

    Attributes:
        `transport_channel`:
            A gRPC transport channel. Prefer to use `SiftChannel`.
        `ingestion_config`:
            The underlying strongly-typed ingestion config. Users of this service don't need to be concerned with this.
        `asset_name`:
            The name of the asset to telemeter.
        `flow_configs_by_name`:
            A mapping of flow config name to the actual flow config.
        `run_id`:
            The ID of the optional run to associated ingested data with.
        `organization_id`:
            ID of the organization of the user.
        `overwrite_rules`:
            If there are rules in Sift that aren't found in the local telemetry config, then initializing
            an `IngestionService` will raise an exception advising the user to update their telemetry config
            with the missing rule before proceeding. Setting this field to `True` replace all rules currently
            in Sift with the rules in the telemetry config.
        `end_stream_on_error`:
            By default any errors that may occur during ingestion API-side are produced asynchronously and ingestion
            won't be interrupted. The errors produced are surfaced on the user errors page. Setting this field to `True`
            will ensure that any errors that occur during ingestion is returned immediately, terminating the stream. This
            is useful for debugging purposes.
    """

    transport_channel: SiftChannel
    ingestion_config: IngestionConfig
    asset_name: str
    flow_configs_by_name: Dict[str, FlowConfig]
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
        super().__init__(channel, config, run_id, overwrite_rules, end_stream_on_error)

    def ingest(self, *requests: IngestWithConfigDataStreamRequest):
        """
        This method performs the actual data ingestion given a list of data ingestion requests.
        """
        super().ingest(*requests)

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
        super().attach_run(channel, run_name, description, organization_id, tags)

    def detach_run(self):
        """
        Detach run from this period of ingestion. Subsequent data ingested won't be associated with
        the run being detached.
        """
        super().detach_run()

    def try_create_ingestion_request(
        self,
        flow_name: str,
        timestamp: datetime,
        channel_values: List[ChannelValue],
    ) -> IngestWithConfigDataStreamRequest:
        """
        Creates an `IngestWithConfigDataStreamRequest`, i.e. a flow, given a `flow_name` and a
        list of `ChannelValue` objects. Channels that appear in the flow config but not in the
        `channel_values` will be assigned an empty value.

        This function will perform validation checks to ensure that the values provided in the dictionary; this
        includes:
          - Making sure the flow exists
          - Making sure that the there are no unexpected channels provided for the given flow
          - Making sure the channel value is the expected type
          - Making sure that the timestamp is in UTC
          - Making sure channels that belong to a component have the 'component' field for the channel value

        If any of the above validations fail then a `IngestionValidationError` will be raised.

        If for performance reasons you'd prefer to skip the validation checks, or perhaps you did the
        validations on your own, prefer to use `create_ingestion_request`. Any errors that occur during
        ingestion will be handled by the Sift API.
        """
        return super().try_create_ingestion_request(flow_name, timestamp, channel_values)

    def create_ingestion_request(
        self,
        flow_name: str,
        timestamp: datetime,
        channel_values: List[IngestWithConfigDataChannelValue],
    ) -> IngestWithConfigDataStreamRequest:
        """
        Unlike `try_create_ingestion_request`, this skips argument validations. Useful for when user has already done their own
        argument validation or if they require low-latency execution time client-side.

        If there are errors that occur during ingestion and the `end_stream_on_error` attribute is set to `False`,
        the data ingestion stream will skip over them and errors instead will be produced asynchronously and become
        available in the UI application in the errors page. If `end_stream_on_error` is set to `True`, then the
        data ingestion stream will be terminated if an error is encountered during ingestion.

        These are some things to look out for when using this method instead of `try_create_ingestion_request`:
        - Values in `channel_values` must appear in the same order its corresponding channel appears in the flow config
          associated with the `flow_name`.
        - The length of `channel_values` is expected to match the length of the channel configs list of the flow config
          associated with `flow_name`. `sift_py.ingestion.channel.empty_value()` may be used if you require empty values.
        - The `timestamp` must be in UTC.
        """
        return super().create_ingestion_request(flow_name, timestamp, channel_values)
