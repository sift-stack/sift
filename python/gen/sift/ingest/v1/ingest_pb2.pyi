from google.protobuf import empty_pb2 as _empty_pb2
from google.protobuf import timestamp_pb2 as _timestamp_pb2
from google.protobuf.internal import containers as _containers
from google.protobuf import descriptor as _descriptor
from google.protobuf import message as _message
from typing import ClassVar as _ClassVar, Iterable as _Iterable, Mapping as _Mapping, Optional as _Optional, Union as _Union

DESCRIPTOR: _descriptor.FileDescriptor

class IngestWithConfigDataStreamRequest(_message.Message):
    __slots__ = ("ingestion_config_id", "flow", "timestamp", "channel_values", "run_id", "end_stream_on_validation_error", "organization_id")
    INGESTION_CONFIG_ID_FIELD_NUMBER: _ClassVar[int]
    FLOW_FIELD_NUMBER: _ClassVar[int]
    TIMESTAMP_FIELD_NUMBER: _ClassVar[int]
    CHANNEL_VALUES_FIELD_NUMBER: _ClassVar[int]
    RUN_ID_FIELD_NUMBER: _ClassVar[int]
    END_STREAM_ON_VALIDATION_ERROR_FIELD_NUMBER: _ClassVar[int]
    ORGANIZATION_ID_FIELD_NUMBER: _ClassVar[int]
    ingestion_config_id: str
    flow: str
    timestamp: _timestamp_pb2.Timestamp
    channel_values: _containers.RepeatedCompositeFieldContainer[IngestWithConfigDataChannelValue]
    run_id: str
    end_stream_on_validation_error: bool
    organization_id: str
    def __init__(self, ingestion_config_id: _Optional[str] = ..., flow: _Optional[str] = ..., timestamp: _Optional[_Union[_timestamp_pb2.Timestamp, _Mapping]] = ..., channel_values: _Optional[_Iterable[_Union[IngestWithConfigDataChannelValue, _Mapping]]] = ..., run_id: _Optional[str] = ..., end_stream_on_validation_error: bool = ..., organization_id: _Optional[str] = ...) -> None: ...

class IngestWithConfigDataStreamResponse(_message.Message):
    __slots__ = ()
    def __init__(self) -> None: ...

class IngestWithConfigDataChannelValue(_message.Message):
    __slots__ = ("string", "double", "float", "bool", "int32", "uint32", "int64", "uint64", "bit_field", "enum", "empty")
    STRING_FIELD_NUMBER: _ClassVar[int]
    DOUBLE_FIELD_NUMBER: _ClassVar[int]
    FLOAT_FIELD_NUMBER: _ClassVar[int]
    BOOL_FIELD_NUMBER: _ClassVar[int]
    INT32_FIELD_NUMBER: _ClassVar[int]
    UINT32_FIELD_NUMBER: _ClassVar[int]
    INT64_FIELD_NUMBER: _ClassVar[int]
    UINT64_FIELD_NUMBER: _ClassVar[int]
    BIT_FIELD_FIELD_NUMBER: _ClassVar[int]
    ENUM_FIELD_NUMBER: _ClassVar[int]
    EMPTY_FIELD_NUMBER: _ClassVar[int]
    string: str
    double: float
    float: float
    bool: bool
    int32: int
    uint32: int
    int64: int
    uint64: int
    bit_field: bytes
    enum: int
    empty: _empty_pb2.Empty
    def __init__(self, string: _Optional[str] = ..., double: _Optional[float] = ..., float: _Optional[float] = ..., bool: bool = ..., int32: _Optional[int] = ..., uint32: _Optional[int] = ..., int64: _Optional[int] = ..., uint64: _Optional[int] = ..., bit_field: _Optional[bytes] = ..., enum: _Optional[int] = ..., empty: _Optional[_Union[_empty_pb2.Empty, _Mapping]] = ...) -> None: ...
