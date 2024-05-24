"""
@generated by mypy-protobuf.  Do not edit manually!
isort:skip_file
"""

import builtins
import collections.abc
import google.protobuf.descriptor
import google.protobuf.empty_pb2
import google.protobuf.internal.containers
import google.protobuf.message
import google.protobuf.timestamp_pb2
import typing

DESCRIPTOR: google.protobuf.descriptor.FileDescriptor

@typing.final
class IngestWithConfigDataStreamRequest(google.protobuf.message.Message):
    DESCRIPTOR: google.protobuf.descriptor.Descriptor

    INGESTION_CONFIG_ID_FIELD_NUMBER: builtins.int
    FLOW_FIELD_NUMBER: builtins.int
    TIMESTAMP_FIELD_NUMBER: builtins.int
    CHANNEL_VALUES_FIELD_NUMBER: builtins.int
    RUN_ID_FIELD_NUMBER: builtins.int
    END_STREAM_ON_VALIDATION_ERROR_FIELD_NUMBER: builtins.int
    ORGANIZATION_ID_FIELD_NUMBER: builtins.int
    ingestion_config_id: builtins.str
    flow: builtins.str
    run_id: builtins.str
    """The run_id MUST be included if this data is part of a run."""
    end_stream_on_validation_error: builtins.bool
    """By default, if this request contains any channel values that do not match
    the supplied ingestion config, the request is stored in an error queue and
    the stream continues to accept data. This ensures all data is saved, but
    only valid data is fully ingested. If this is set to `true`, any validation
    errors end the stream and return the error to the client.
    """
    organization_id: builtins.str
    @property
    def timestamp(self) -> google.protobuf.timestamp_pb2.Timestamp: ...
    @property
    def channel_values(self) -> google.protobuf.internal.containers.RepeatedCompositeFieldContainer[global___IngestWithConfigDataChannelValue]: ...
    def __init__(
        self,
        *,
        ingestion_config_id: builtins.str = ...,
        flow: builtins.str = ...,
        timestamp: google.protobuf.timestamp_pb2.Timestamp | None = ...,
        channel_values: collections.abc.Iterable[global___IngestWithConfigDataChannelValue] | None = ...,
        run_id: builtins.str = ...,
        end_stream_on_validation_error: builtins.bool = ...,
        organization_id: builtins.str = ...,
    ) -> None: ...
    def HasField(self, field_name: typing.Literal["timestamp", b"timestamp"]) -> builtins.bool: ...
    def ClearField(self, field_name: typing.Literal["channel_values", b"channel_values", "end_stream_on_validation_error", b"end_stream_on_validation_error", "flow", b"flow", "ingestion_config_id", b"ingestion_config_id", "organization_id", b"organization_id", "run_id", b"run_id", "timestamp", b"timestamp"]) -> None: ...

global___IngestWithConfigDataStreamRequest = IngestWithConfigDataStreamRequest

@typing.final
class IngestWithConfigDataStreamResponse(google.protobuf.message.Message):
    DESCRIPTOR: google.protobuf.descriptor.Descriptor

    def __init__(
        self,
    ) -> None: ...

global___IngestWithConfigDataStreamResponse = IngestWithConfigDataStreamResponse

@typing.final
class IngestWithConfigDataChannelValue(google.protobuf.message.Message):
    DESCRIPTOR: google.protobuf.descriptor.Descriptor

    STRING_FIELD_NUMBER: builtins.int
    DOUBLE_FIELD_NUMBER: builtins.int
    FLOAT_FIELD_NUMBER: builtins.int
    BOOL_FIELD_NUMBER: builtins.int
    INT32_FIELD_NUMBER: builtins.int
    UINT32_FIELD_NUMBER: builtins.int
    INT64_FIELD_NUMBER: builtins.int
    UINT64_FIELD_NUMBER: builtins.int
    BIT_FIELD_FIELD_NUMBER: builtins.int
    ENUM_FIELD_NUMBER: builtins.int
    EMPTY_FIELD_NUMBER: builtins.int
    string: builtins.str
    double: builtins.float
    float: builtins.float
    bool: builtins.bool
    int32: builtins.int
    uint32: builtins.int
    int64: builtins.int
    uint64: builtins.int
    bit_field: builtins.bytes
    enum: builtins.int
    @property
    def empty(self) -> google.protobuf.empty_pb2.Empty:
        """If there's not a new data point for a channel at the given timestamp, pass empty to skip it"""

    def __init__(
        self,
        *,
        string: builtins.str = ...,
        double: builtins.float = ...,
        float: builtins.float = ...,
        bool: builtins.bool = ...,
        int32: builtins.int = ...,
        uint32: builtins.int = ...,
        int64: builtins.int = ...,
        uint64: builtins.int = ...,
        bit_field: builtins.bytes = ...,
        enum: builtins.int = ...,
        empty: google.protobuf.empty_pb2.Empty | None = ...,
    ) -> None: ...
    def HasField(self, field_name: typing.Literal["bit_field", b"bit_field", "bool", b"bool", "double", b"double", "empty", b"empty", "enum", b"enum", "float", b"float", "int32", b"int32", "int64", b"int64", "string", b"string", "type", b"type", "uint32", b"uint32", "uint64", b"uint64"]) -> builtins.bool: ...
    def ClearField(self, field_name: typing.Literal["bit_field", b"bit_field", "bool", b"bool", "double", b"double", "empty", b"empty", "enum", b"enum", "float", b"float", "int32", b"int32", "int64", b"int64", "string", b"string", "type", b"type", "uint32", b"uint32", "uint64", b"uint64"]) -> None: ...
    def WhichOneof(self, oneof_group: typing.Literal["type", b"type"]) -> typing.Literal["string", "double", "float", "bool", "int32", "uint32", "int64", "uint64", "bit_field", "enum", "empty"] | None: ...

global___IngestWithConfigDataChannelValue = IngestWithConfigDataChannelValue
