from __future__ import annotations
from ..grpc.transport import SiftChannel
from .config import TelemetryConfig
from ..ingestion.flow import FlowConfig
from .channel import ChannelValue
from sift.ingest.v1.ingest_pb2 import (
    IngestWithConfigDataChannelValue,
    IngestWithConfigDataStreamRequest,
)
from sift.ingestion_configs.v1.ingestion_configs_pb2 import IngestionConfig
from typing import Dict, List, Optional
from .impl.ingest import IngestionServiceImpl
from datetime import datetime


class IngestionService(IngestionServiceImpl):
    """
    A fully configured service that, when instantiated, is ready to start ingesting data.
    """

    transport_channel: SiftChannel
    ingestion_config: IngestionConfig
    asset_name: str
    flow_configs_by_name: Dict[str, FlowConfig]
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
        super().__init__(channel, config, run_id, end_stream_on_error)

    def ingest(self, *requests: IngestWithConfigDataStreamRequest):
        """
        This method performs the actual data ingestion given a list of data ingestion requests.
        """
        super().ingest(*requests)

    def start_run(
        self,
        channel: SiftChannel,
        run_name: str,
        description: Optional[str] = None,
        organization_id: Optional[str] = None,
        tags: Optional[List[str]] = None,
    ):
        """
        Create a run to use as part of the call to `ingest`.
        """
        super().start_run(channel, run_name, description, organization_id, tags)

    def end_run(self):
        """
        End the current run if any and don't include it in subsequent calls to `ingest`.
        """
        super().end_run()

    def try_create_ingestion_request(
        self,
        flow_name: str,
        timestamp: datetime,
        channel_values: List[ChannelValue],
    ) -> IngestWithConfigDataStreamRequest:
        """
        Creates an `IngestWithConfigDataStreamRequest`, i.e. a flow, given a `flow_name` and a
        list of `ChannelValue` objects. Channels that appear in the flow config but not in the
        `channel_value_by_channel_name` will be assigned an empty value.

        This function will perform validation checks to ensure that the values provided in the dictionary; this
        includes:
          - Making sure the flow exists
          - Making sure that the there are no unexpected channels provided for the given flow
          - Making sure the channel value is the expected type
          - Making sure that the timestamp is in UTC
          - Making sure channels that belong to a component have the 'component' field for the channel value

        If any of the above validations fail then a `ValueError` will be raised.

        If for performance reasons you'd prefer to skip the validation checks, or perhaps you did the
        validations on your own, prefer to use `create_ingestion_request`. Any errors that occur during
        ingestion will be handled by the Sift API.
        """
        return super().try_create_ingestion_request(
            flow_name, timestamp, channel_values
        )

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
          associated with `flow_name`. `google.protobuf.empty_pb2.Empty` may be used if you require empty values.
        - The `timestamp` must be in UTC.
        """
        return super().create_ingestion_request(flow_name, timestamp, channel_values)
