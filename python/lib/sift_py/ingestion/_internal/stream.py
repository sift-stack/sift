import asyncio
from queue import Queue
from typing import List, Optional

from sift.ingest.v1.ingest_pb2 import IngestWithConfigDataStreamRequest
from sift_stream_bindings import (
    ChannelBitFieldElementPy,
    ChannelConfigPy,
    ChannelDataTypePy,
    ChannelEnumTypePy,
    ChannelValuePy,
    FlowConfigPy,
    IngestionConfigFormPy,
    IngestWithConfigDataStreamRequestPy,
    RunFormPy,
    SiftStreamBuilderPy,
    TimeValuePy,
)

from sift_py.grpc.transport import SiftChannel
from sift_py.ingestion.config.telemetry import TelemetryConfig


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

    if not uri.startswith("http"):
        if "localhost" in uri:
            uri = f"http://{uri}"
        else:
            uri = f"https://{uri}"

    builder = SiftStreamBuilderPy(uri, apikey)
    builder.ingestion_config = telemetry_config_to_ingestion_config_py(ingestion_config)
    builder.enable_tls = channel.config.get("use_ssl", True)
    return builder


async def stream_requests_async(
    builder: SiftStreamBuilderPy, run_id: str, *requests: IngestWithConfigDataStreamRequest
):
    async def ingestion_thread():
        # Create stream and send requests
        sift_stream = await builder.build()
        try:
            while not data_queue.empty():
                item = data_queue.get()
                sift_stream = await sift_stream.send_requests(item)
            await sift_stream.finish()
        except Exception as e:
            # Ensure stream is finished even if there's an error
            try:
                await sift_stream.finish()
            except:
                pass
            raise e

    # Create a dedicated queue for this batch of requests
    data_queue = Queue()

    # Put each request individually into the queue, filtering out None values
    processed_requests = []
    for request in requests:
        processed_request = ingest_request_to_ingest_request_py(request, run_id)
        if processed_request is not None:
            processed_requests.append(processed_request)
    data_queue.put(processed_requests)

    print(f"Processing {len(requests)} requests in queue")

    # Process this batch
    await ingestion_thread()


def stream_requests(
    builder: SiftStreamBuilderPy,
    *requests: IngestWithConfigDataStreamRequest,
    run_id: str = "",
) -> None:
    """
    Stream requests using the stream bindings synchronously.
    Each call to this function creates its own queue and stream, allowing multiple
    batches to be processed concurrently when called from different threads.

    Args:
        builder: The SiftStreamBuilderPy to use for streaming
        requests: List of IngestWithConfigDataStreamRequest protobuf objects
        run_id: Optional run ID to associate with the requests
    """
    print(f"Starting stream requests for {len(requests)} requests")
    asyncio.run(stream_requests_async(builder, run_id, *requests))


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
        client_key=client_key or "",
        tags=run_tags,
    )


def ingest_request_to_ingest_request_py(
    request,
    run_id: str = "",
) -> IngestWithConfigDataStreamRequestPy:
    """
    Convert an IngestWithConfigDataStreamRequest to IngestWithConfigDataStreamRequestPy.

    Args:
        request: The IngestWithConfigDataStreamRequest to convert
        run_id: The run ID to use

    Returns:
        IngestWithConfigDataStreamRequestPy: The converted request
    """
    if request is None:
        return None

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
        run_id=run_id or "",
        end_stream_on_validation_error=request.end_stream_on_validation_error,
        organization_id=request.organization_id,
    )


def convert_channel_value_to_channel_value_py(channel_value) -> ChannelValuePy:
    """
    Convert an IngestWithConfigDataChannelValue to ChannelValuePy.

    Args:
        channel_value: The IngestWithConfigDataChannelValue to convert

    Returns:
        ChannelValuePy: The converted channel value
    """
    # Import here to avoid circular imports
    from sift.ingest.v1.ingest_pb2 import IngestWithConfigDataChannelValue

    if not isinstance(channel_value, IngestWithConfigDataChannelValue):
        raise ValueError(f"Expected IngestWithConfigDataChannelValue, got {type(channel_value)}")

    if channel_value.HasField("string"):
        return ChannelValuePy.string("", channel_value.string)
    elif channel_value.HasField("double"):
        return ChannelValuePy.double("", channel_value.double)
    elif channel_value.HasField("float"):
        return ChannelValuePy.float("", channel_value.float)
    elif channel_value.HasField("bool"):
        return ChannelValuePy.bool("", channel_value.bool)
    elif channel_value.HasField("int32"):
        return ChannelValuePy.int32("", channel_value.int32)
    elif channel_value.HasField("uint32"):
        return ChannelValuePy.uint32("", channel_value.uint32)
    elif channel_value.HasField("int64"):
        return ChannelValuePy.int64("", channel_value.int64)
    elif channel_value.HasField("uint64"):
        return ChannelValuePy.uint64("", channel_value.uint64)
    elif channel_value.HasField("enum"):
        # For enum values, we need to create a ChannelEnumTypePy
        enum_type = ChannelEnumTypePy(name=f"enum_{channel_value.enum}", key=channel_value.enum)
        return ChannelValuePy.enum_value("", enum_type)
    elif channel_value.HasField("bit_field"):
        return ChannelValuePy.bitfield("", channel_value.bitfield)
    elif channel_value.HasField("bytes"):
        # For bytes values, we'll convert to a string representation
        return ChannelValuePy.string("", str(channel_value.bytes))
    elif channel_value.HasField("empty"):
        # For empty values, we'll return a default value
        return ChannelValuePy.string("", "")
    else:
        # No field set, return empty string
        return ChannelValuePy.string("", "")
