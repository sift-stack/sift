import asyncio
import logging
import threading
import time
import uuid
from queue import Queue
from typing import List, Optional

from sift.ingest.v1.ingest_pb2 import (
    IngestWithConfigDataChannelValue,
    IngestWithConfigDataStreamRequest,
)
from sift_stream_bindings import (
    ChannelBitFieldElementPy,
    ChannelConfigPy,
    ChannelDataTypePy,
    ChannelEnumTypePy,
    FlowConfigPy,
    IngestionConfigFormPy,
    IngestWithConfigDataChannelValuePy,
    IngestWithConfigDataStreamRequestPy,
    RecoveryStrategyPy,
    RetryPolicyPy,
    RunFormPy,
    SiftStreamBuilderPy,
    TimeValuePy,
)

from sift_py.grpc.transport import SiftChannel
from sift_py.ingestion.config.telemetry import TelemetryConfig

logger = logging.getLogger(__name__)


class IngestionThread(threading.Thread):
    """
    Manages ingestion for a single ingestion config.
    """

    IDLE_LOOP_PERIOD = 0.1  # Time of intervals loop will sleep while waiting for data.
    SIFT_STREAM_FINISH_TIMEOUT = 0.06  # Measured ~0.05s to finish stream.
    CLEANUP_TIMEOUT = IDLE_LOOP_PERIOD + SIFT_STREAM_FINISH_TIMEOUT

    def __init__(
        self,
        sift_stream_builder: SiftStreamBuilderPy,
        data_queue: Queue,
        metric_interval: float = 0.5,
    ):
        """
        Initialize the IngestionThread.

        Args:
            sift_stream_builder: The sift stream builder to build a new stream.
            data_queue: The queue to put IngestWithConfigDataStreamRequestPy requests into for ingestion.
            ingestion_config: The ingestion config to use for ingestion.
            metric_interval: Time (seconds) to wait between logging metrics.
        """
        super().__init__(daemon=True)
        self.data_queue = data_queue
        self._stop_event = threading.Event()
        self.sift_stream_builder = sift_stream_builder
        self.metric_interval = metric_interval

    def stop(self):
        self._stop_event.set()
        # Give a brief chance to finish the stream (should take < 50ms).
        time.sleep(self.CLEANUP_TIMEOUT)
        self.task.cancel()

    async def main(self):
        logger.debug("Ingestion thread started")
        sift_stream = await self.sift_stream_builder.build()
        time_since_last_metric = time.time() - 1
        count = 0
        try:
            while True:
                while not self.data_queue.empty():
                    if self._stop_event.is_set():
                        # Being forced to stop. Try to finish the stream.
                        logger.info(
                            f"Ingestion thread received stop signal. Exiting. Sent {count} requests. {self.data_queue.qsize()} requests remaining."
                        )
                        await sift_stream.finish()
                        return
                    item = self.data_queue.get()
                    if item is None:
                        self._stop_event.set()
                        continue
                    sift_stream = await sift_stream.send_requests(item)
                    count += 1
                    if time.time() - time_since_last_metric > self.metric_interval:
                        logger.debug(
                            f"Ingestion thread sent {count} requests, remaining: {self.data_queue.qsize()}"
                        )
                        time_since_last_metric = time.time()

                if self._stop_event.is_set():
                    logger.debug(
                        f"No more requests. Stopping. Sent {count} requests. {self.data_queue.qsize()} requests remaining."
                    )
                    await sift_stream.finish()
                    return
                else:
                    time.sleep(self.IDLE_LOOP_PERIOD)

        except asyncio.CancelledError:
            # It's possible the thread was joined while sleeping waiting for data. Only note error if we have data left.
            if self.data_queue.qsize() > 0:
                logger.error(
                    f"Ingestion thread cancelled without finishing stream. {self.data_queue.qsize()} requests were not sent."
                )

    async def _run(self):
        self.task = asyncio.create_task(self.main())
        await self.task

    def run(self):
        """This thread will handle sending data to Sift."""
        # Even thought this is a thread, we need to run this async task to await send_requests otherwise we get sift_stream consumed errors.
        asyncio.run(self._run())


def get_builder(channel: SiftChannel, ingestion_config: TelemetryConfig) -> SiftStreamBuilderPy:
    """
    Get a builder for a stream.

    Args:
        channel: The channel to get a builder for
        ingestion_config: The ingestion config to use for the builder

    Returns:
        SiftStreamBuilderPy: The builder for the channel
    """
    uri = channel.config.get("uri")
    apikey = channel.config.get("apikey")

    if not uri or not apikey:
        raise ValueError(f"Channel config is missing uri or apikey: {channel.config}")

    # SiftStreamBuilder needs URI to start with http or https
    if not uri.startswith("http"):
        if "localhost" in uri:
            uri = f"http://{uri}"
        else:
            uri = f"https://{uri}"

    builder = SiftStreamBuilderPy(uri, apikey)
    builder.ingestion_config = telemetry_config_to_ingestion_config_py(ingestion_config)
    builder.enable_tls = channel.config.get("use_ssl", True)
    # FD-177: Expose configuration for recovery strategy.
    builder.recovery_strategy = RecoveryStrategyPy.retry_only(RetryPolicyPy.default())

    return builder


async def stream_requests_async(data_queue: Queue, *requests: IngestWithConfigDataStreamRequest):
    """
    Non-blocking: Convert requests for rust bindings and put them into a queue.

    Args:
        data_queue: The queue to put IngestWithConfigDataStreamRequestPy requests into for ingestion.
        requests: List of IngestWithConfigDataStreamRequest protobuf objects
    """

    # Put each request individually into the queue, filtering out None values
    processed_requests = []
    for request in requests:
        if not isinstance(request, IngestWithConfigDataStreamRequest):
            raise ValueError(f"Received unexpected request: {request} of type {type(request)}")
        processed_requests.append(ingest_request_to_ingest_request_py(request))
    data_queue.put(processed_requests)


def stream_requests(
    data_queue: Queue,
    *requests: IngestWithConfigDataStreamRequest,
) -> None:
    """
    Blocking: Convert requests for rust bindings and put them into a queue.

    Args:
        data_queue: The queue to put IngestWithConfigDataStreamRequestPy requests into for ingestion.
        requests: List of IngestWithConfigDataStreamRequest protobuf objects
    """
    asyncio.run(stream_requests_async(data_queue, *requests))


def telemetry_config_to_ingestion_config_py(
    telemetry_config: TelemetryConfig,
) -> IngestionConfigFormPy:
    """
    Convert a TelemetryConfig to an IngestionConfigFormPy.

    Args:
        telemetry_config: The TelemetryConfig to convert

    Returns:
        IngestionConfigFormPy: The converted ingestion config
    """
    # Convert flows
    flow_configs_py = []

    for flow_config in telemetry_config.flows:
        # Convert channels in this flow
        channel_configs_py = []

        for channel_config in flow_config.channels:
            # Convert enum types
            enum_types_py = []
            for enum_type in channel_config.enum_types:
                enum_types_py.append(
                    ChannelEnumTypePy(
                        name=enum_type.name,
                        key=enum_type.key,
                    )
                )

            # Convert bit field elements
            bit_field_elements_py = []
            for bit_field_element in channel_config.bit_field_elements:
                bit_field_elements_py.append(
                    ChannelBitFieldElementPy(
                        name=bit_field_element.name,
                        index=bit_field_element.index,
                        bit_count=bit_field_element.bit_count,
                    )
                )

            # Convert data type
            data_type_py = convert_channel_data_type(channel_config.data_type)

            # Create channel config
            channel_config_py = ChannelConfigPy(
                name=channel_config.name,
                data_type=data_type_py,
                unit=channel_config.unit or "",
                description=channel_config.description or "",
                enum_types=enum_types_py,
                bit_field_elements=bit_field_elements_py,
            )

            channel_configs_py.append(channel_config_py)

        # Create flow config
        flow_config_py = FlowConfigPy(
            name=flow_config.name,
            channels=channel_configs_py,
        )

        flow_configs_py.append(flow_config_py)
    # Create ingestion config
    ingestion_config_py = IngestionConfigFormPy(
        asset_name=telemetry_config.asset_name,
        client_key=telemetry_config.ingestion_client_key,
        flows=flow_configs_py,
    )

    return ingestion_config_py


def convert_channel_data_type(data_type) -> ChannelDataTypePy:
    """
    Convert a ChannelDataType to ChannelDataTypePy.

    Args:
        data_type: The ChannelDataType to convert

    Returns:
        ChannelDataTypePy: The converted data type
    """
    # Import here to avoid circular imports
    from sift_py.ingestion.channel import ChannelDataType

    if data_type == ChannelDataType.DOUBLE:
        return ChannelDataTypePy.Double
    elif data_type == ChannelDataType.STRING:
        return ChannelDataTypePy.String
    elif data_type == ChannelDataType.ENUM:
        return ChannelDataTypePy.Enum
    elif data_type == ChannelDataType.BIT_FIELD:
        return ChannelDataTypePy.BitField
    elif data_type == ChannelDataType.BOOL:
        return ChannelDataTypePy.Bool
    elif data_type == ChannelDataType.FLOAT:
        return ChannelDataTypePy.Float
    elif data_type == ChannelDataType.INT_32:
        return ChannelDataTypePy.Int32
    elif data_type == ChannelDataType.UINT_32:
        return ChannelDataTypePy.Uint32
    elif data_type == ChannelDataType.INT_64:
        return ChannelDataTypePy.Int64
    elif data_type == ChannelDataType.UINT_64:
        return ChannelDataTypePy.Uint64
    elif data_type == ChannelDataType.BYTES:
        return ChannelDataTypePy.Bytes
    else:
        return ChannelDataTypePy.Unspecified


def get_run_form(
    run_name: str, run_description: str, client_key: Optional[str] = None, run_tags: List[str] = []
) -> RunFormPy:
    """
    Get a run form.

    Args:
        run_name: The name of the run
        run_description: The description of the run
        client_key: The client key to use (if empty, run_name will be used and validated)
        run_tags: The tags of the run

    Returns:
        RunFormPy: The run form
    """
    return RunFormPy(
        name=run_name,
        description=run_description,
        client_key=client_key or str(uuid.uuid4()),
        tags=run_tags,
    )


def ingest_request_to_ingest_request_py(
    request: IngestWithConfigDataStreamRequest,
) -> IngestWithConfigDataStreamRequestPy:
    """
    Convert an IngestWithConfigDataStreamRequest to IngestWithConfigDataStreamRequestPy.

    Args:
        request: The IngestWithConfigDataStreamRequest to convert
        run_id: The run ID to use

    Returns:
        IngestWithConfigDataStreamRequestPy: The converted request
    """
    timestamp_py = None
    if request.HasField("timestamp"):
        timestamp_py = TimeValuePy.from_timestamp(
            request.timestamp.seconds, request.timestamp.nanos
        )

    channel_values_py = [
        convert_channel_value_to_channel_value_py(channel_value)
        for channel_value in request.channel_values
    ]

    return IngestWithConfigDataStreamRequestPy(
        ingestion_config_id=request.ingestion_config_id,
        flow=request.flow,
        timestamp=timestamp_py,
        channel_values=channel_values_py,
        run_id=request.run_id or "",
        end_stream_on_validation_error=request.end_stream_on_validation_error,
        organization_id=request.organization_id,
    )


def convert_channel_value_to_channel_value_py(
    channel_value: IngestWithConfigDataChannelValue,
) -> IngestWithConfigDataChannelValuePy:
    """
    Convert an IngestWithConfigDataChannelValue to IngestWithConfigDataChannelValuePy.

    Args:
        channel_value: The IngestWithConfigDataChannelValue to convert

    Returns:
        IngestWithConfigDataChannelValuePy: The converted channel value
    """
    if channel_value.HasField("string"):
        return IngestWithConfigDataChannelValuePy.string(channel_value.string)
    elif channel_value.HasField("double"):
        return IngestWithConfigDataChannelValuePy.double(channel_value.double)
    elif channel_value.HasField("float"):
        return IngestWithConfigDataChannelValuePy.float(channel_value.float)
    elif channel_value.HasField("bool"):
        return IngestWithConfigDataChannelValuePy.bool(channel_value.bool)
    elif channel_value.HasField("int32"):
        return IngestWithConfigDataChannelValuePy.int32(channel_value.int32)
    elif channel_value.HasField("uint32"):
        return IngestWithConfigDataChannelValuePy.uint32(channel_value.uint32)
    elif channel_value.HasField("int64"):
        return IngestWithConfigDataChannelValuePy.int64(channel_value.int64)
    elif channel_value.HasField("uint64"):
        return IngestWithConfigDataChannelValuePy.uint64(channel_value.uint64)
    elif channel_value.HasField("enum"):
        return IngestWithConfigDataChannelValuePy.enum_value(channel_value.enum)
    elif channel_value.HasField("bit_field"):
        return IngestWithConfigDataChannelValuePy.bitfield(channel_value.bit_field)
    elif channel_value.HasField("bytes"):
        # For bytes values, we'll convert to a string representation
        return IngestWithConfigDataChannelValuePy.string(str(channel_value.bytes))
    elif channel_value.HasField("empty"):
        # For empty values, we'll return a default value
        return IngestWithConfigDataChannelValuePy.empty()
    else:
        raise ValueError(f"{channel_value} missing type field.")
