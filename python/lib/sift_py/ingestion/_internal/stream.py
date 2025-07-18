import asyncio
from typing import List

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

"""
TODO:
    - helper to fetch ingestion config id via client key
    - stream helper: build, send, finish
    - helper to convert FlowOrderedChannelValues to IngestWithConfigDataStreamRequestPy
    - create_ingestion_request helper, IngestWithConfigDataStreamRequestPy?
    - helper to convert List of IngestWithConfigDataChannelValue to IngestWithConfigDataStreamRequestPy?
"""


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

    if not uri.startswith("https://"):
        uri = f"https://{uri}"
    print(f"Using URI: {uri}")

    builder = SiftStreamBuilderPy(uri, apikey)
    builder.ingestion_config = telemetry_config_to_ingestion_config_py(ingestion_config)
    builder.enable_tls = channel.config.get("use_ssl", True)
    return builder


async def stream_requests_async(
    builder: SiftStreamBuilderPy,
    requests: List,
    run_id: str = "",
) -> None:
    """
    Stream requests using the stream bindings asynchronously.

    Args:
        builder: The SiftStreamBuilderPy to use for streaming
        requests: List of IngestWithConfigDataStreamRequest protobuf objects
        run_id: Optional run ID to associate with the requests
    """
    # Convert protobuf requests to Python binding requests
    py_requests = [ingest_request_to_ingest_request_py(request, run_id) for request in requests]

    # Create stream and send requests
    sift_stream = await builder.build()
    sift_stream = await sift_stream.send_requests(py_requests)
    await sift_stream.finish()


def stream_requests(
    builder: SiftStreamBuilderPy,
    requests: List,
    run_id: str = "",
) -> None:
    """
    Stream requests using the stream bindings synchronously.

    Args:
        builder: The SiftStreamBuilderPy to use for streaming
        requests: List of IngestWithConfigDataStreamRequest protobuf objects
        run_id: Optional run ID to associate with the requests
    """
    # Run the async function in a new event loop
    asyncio.run(stream_requests_async(builder, requests, run_id))


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


def get_run_form(run_name: str, run_description: str, run_tags: List[str]) -> RunFormPy:
    """
    Get a run form.

    Args:
        run_name: The name of the run
        run_description: The description of the run
        run_tags: The tags of the run

    Returns:
        RunFormPy: The run form
    """
    return RunFormPy(
        name=run_name,
        description=run_description,
        client_key=run_name,
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

    # Extract the value from the oneof field
    # Note: We need a channel name, but the protobuf doesn't contain it
    # This is a limitation - we'll use a placeholder name
    channel_name = "unknown_channel"  # This is a limitation of the conversion

    if channel_value.HasField("string"):
        return ChannelValuePy.string(channel_name, channel_value.string)
    elif channel_value.HasField("double"):
        return ChannelValuePy.double(channel_name, channel_value.double)
    elif channel_value.HasField("float"):
        return ChannelValuePy.float(channel_name, channel_value.float)
    elif channel_value.HasField("bool"):
        return ChannelValuePy.bool(channel_name, channel_value.bool)
    elif channel_value.HasField("int32"):
        return ChannelValuePy.int32(channel_name, channel_value.int32)
    elif channel_value.HasField("uint32"):
        return ChannelValuePy.uint32(channel_name, channel_value.uint32)
    elif channel_value.HasField("int64"):
        return ChannelValuePy.int64(channel_name, channel_value.int64)
    elif channel_value.HasField("uint64"):
        return ChannelValuePy.uint64(channel_name, channel_value.uint64)
    elif channel_value.HasField("enum"):
        # For enum values, we need to create a ChannelEnumTypePy
        enum_type = ChannelEnumTypePy(name=f"enum_{channel_value.enum}", key=channel_value.enum)
        return ChannelValuePy.enum_value(channel_name, enum_type)
    elif channel_value.HasField("bit_field"):
        # For bit field values, we need to create ChannelBitFieldElementPy list
        # This is a simplified conversion - in practice you'd need the actual bit field definition
        bit_field_elements = []
        for i, byte in enumerate(channel_value.bit_field):
            if byte != 0:
                bit_field_elements.append(
                    ChannelBitFieldElementPy(name=f"bit_{i}", index=i, bit_count=1)
                )
        return ChannelValuePy.bitfield(channel_name, bit_field_elements)
    elif channel_value.HasField("bytes"):
        # For bytes values, we'll convert to a string representation
        return ChannelValuePy.string(channel_name, str(channel_value.bytes))
    elif channel_value.HasField("empty"):
        # For empty values, we'll return a default value
        return ChannelValuePy.string(channel_name, "")
    else:
        # No field set, return empty string
        return ChannelValuePy.string(channel_name, "")
