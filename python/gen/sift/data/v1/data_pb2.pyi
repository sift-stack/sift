from google.api import annotations_pb2 as _annotations_pb2
from google.api import field_behavior_pb2 as _field_behavior_pb2
from google.protobuf import any_pb2 as _any_pb2
from google.protobuf import timestamp_pb2 as _timestamp_pb2
from protoc_gen_openapiv2.options import annotations_pb2 as _annotations_pb2_1
from sift.calculated_channels.v1 import calculated_channels_pb2 as _calculated_channels_pb2
from sift.common.type.v1 import channel_bit_field_element_pb2 as _channel_bit_field_element_pb2
from sift.common.type.v1 import channel_data_type_pb2 as _channel_data_type_pb2
from sift.common.type.v1 import channel_enum_type_pb2 as _channel_enum_type_pb2
from google.protobuf.internal import containers as _containers
from google.protobuf import descriptor as _descriptor
from google.protobuf import message as _message
from typing import ClassVar as _ClassVar, Iterable as _Iterable, Mapping as _Mapping, Optional as _Optional, Union as _Union

DESCRIPTOR: _descriptor.FileDescriptor

class GetDataRequest(_message.Message):
    __slots__ = ("queries", "start_time", "end_time", "sample_ms", "page_size", "page_token")
    QUERIES_FIELD_NUMBER: _ClassVar[int]
    START_TIME_FIELD_NUMBER: _ClassVar[int]
    END_TIME_FIELD_NUMBER: _ClassVar[int]
    SAMPLE_MS_FIELD_NUMBER: _ClassVar[int]
    PAGE_SIZE_FIELD_NUMBER: _ClassVar[int]
    PAGE_TOKEN_FIELD_NUMBER: _ClassVar[int]
    queries: _containers.RepeatedCompositeFieldContainer[Query]
    start_time: _timestamp_pb2.Timestamp
    end_time: _timestamp_pb2.Timestamp
    sample_ms: int
    page_size: int
    page_token: str
    def __init__(self, queries: _Optional[_Iterable[_Union[Query, _Mapping]]] = ..., start_time: _Optional[_Union[_timestamp_pb2.Timestamp, _Mapping]] = ..., end_time: _Optional[_Union[_timestamp_pb2.Timestamp, _Mapping]] = ..., sample_ms: _Optional[int] = ..., page_size: _Optional[int] = ..., page_token: _Optional[str] = ...) -> None: ...

class Query(_message.Message):
    __slots__ = ("channel", "calculated_channel")
    CHANNEL_FIELD_NUMBER: _ClassVar[int]
    CALCULATED_CHANNEL_FIELD_NUMBER: _ClassVar[int]
    channel: ChannelQuery
    calculated_channel: CalculatedChannelQuery
    def __init__(self, channel: _Optional[_Union[ChannelQuery, _Mapping]] = ..., calculated_channel: _Optional[_Union[CalculatedChannelQuery, _Mapping]] = ...) -> None: ...

class ChannelQuery(_message.Message):
    __slots__ = ("channel_id", "run_id")
    CHANNEL_ID_FIELD_NUMBER: _ClassVar[int]
    RUN_ID_FIELD_NUMBER: _ClassVar[int]
    channel_id: str
    run_id: str
    def __init__(self, channel_id: _Optional[str] = ..., run_id: _Optional[str] = ...) -> None: ...

class CalculatedChannelQuery(_message.Message):
    __slots__ = ("channel_key", "expression", "run_id")
    CHANNEL_KEY_FIELD_NUMBER: _ClassVar[int]
    EXPRESSION_FIELD_NUMBER: _ClassVar[int]
    RUN_ID_FIELD_NUMBER: _ClassVar[int]
    channel_key: str
    expression: _calculated_channels_pb2.ExpressionRequest
    run_id: str
    def __init__(self, channel_key: _Optional[str] = ..., expression: _Optional[_Union[_calculated_channels_pb2.ExpressionRequest, _Mapping]] = ..., run_id: _Optional[str] = ...) -> None: ...

class GetDataResponse(_message.Message):
    __slots__ = ("next_page_token", "data")
    NEXT_PAGE_TOKEN_FIELD_NUMBER: _ClassVar[int]
    DATA_FIELD_NUMBER: _ClassVar[int]
    next_page_token: str
    data: _containers.RepeatedCompositeFieldContainer[_any_pb2.Any]
    def __init__(self, next_page_token: _Optional[str] = ..., data: _Optional[_Iterable[_Union[_any_pb2.Any, _Mapping]]] = ...) -> None: ...

class Metadata(_message.Message):
    __slots__ = ("data_type", "sampled_ms", "asset", "run", "channel")
    class Asset(_message.Message):
        __slots__ = ("asset_id", "name")
        ASSET_ID_FIELD_NUMBER: _ClassVar[int]
        NAME_FIELD_NUMBER: _ClassVar[int]
        asset_id: str
        name: str
        def __init__(self, asset_id: _Optional[str] = ..., name: _Optional[str] = ...) -> None: ...
    class Run(_message.Message):
        __slots__ = ("run_id", "name")
        RUN_ID_FIELD_NUMBER: _ClassVar[int]
        NAME_FIELD_NUMBER: _ClassVar[int]
        run_id: str
        name: str
        def __init__(self, run_id: _Optional[str] = ..., name: _Optional[str] = ...) -> None: ...
    class Channel(_message.Message):
        __slots__ = ("channel_id", "component", "name", "unit", "enum_types", "bit_field_elements")
        class Unit(_message.Message):
            __slots__ = ("name", "abbreviated_name")
            NAME_FIELD_NUMBER: _ClassVar[int]
            ABBREVIATED_NAME_FIELD_NUMBER: _ClassVar[int]
            name: str
            abbreviated_name: str
            def __init__(self, name: _Optional[str] = ..., abbreviated_name: _Optional[str] = ...) -> None: ...
        CHANNEL_ID_FIELD_NUMBER: _ClassVar[int]
        COMPONENT_FIELD_NUMBER: _ClassVar[int]
        NAME_FIELD_NUMBER: _ClassVar[int]
        UNIT_FIELD_NUMBER: _ClassVar[int]
        ENUM_TYPES_FIELD_NUMBER: _ClassVar[int]
        BIT_FIELD_ELEMENTS_FIELD_NUMBER: _ClassVar[int]
        channel_id: str
        component: str
        name: str
        unit: Metadata.Channel.Unit
        enum_types: _containers.RepeatedCompositeFieldContainer[_channel_enum_type_pb2.ChannelEnumType]
        bit_field_elements: _containers.RepeatedCompositeFieldContainer[_channel_bit_field_element_pb2.ChannelBitFieldElement]
        def __init__(self, channel_id: _Optional[str] = ..., component: _Optional[str] = ..., name: _Optional[str] = ..., unit: _Optional[_Union[Metadata.Channel.Unit, _Mapping]] = ..., enum_types: _Optional[_Iterable[_Union[_channel_enum_type_pb2.ChannelEnumType, _Mapping]]] = ..., bit_field_elements: _Optional[_Iterable[_Union[_channel_bit_field_element_pb2.ChannelBitFieldElement, _Mapping]]] = ...) -> None: ...
    DATA_TYPE_FIELD_NUMBER: _ClassVar[int]
    SAMPLED_MS_FIELD_NUMBER: _ClassVar[int]
    ASSET_FIELD_NUMBER: _ClassVar[int]
    RUN_FIELD_NUMBER: _ClassVar[int]
    CHANNEL_FIELD_NUMBER: _ClassVar[int]
    data_type: _channel_data_type_pb2.ChannelDataType
    sampled_ms: int
    asset: Metadata.Asset
    run: Metadata.Run
    channel: Metadata.Channel
    def __init__(self, data_type: _Optional[_Union[_channel_data_type_pb2.ChannelDataType, str]] = ..., sampled_ms: _Optional[int] = ..., asset: _Optional[_Union[Metadata.Asset, _Mapping]] = ..., run: _Optional[_Union[Metadata.Run, _Mapping]] = ..., channel: _Optional[_Union[Metadata.Channel, _Mapping]] = ...) -> None: ...

class DoubleValue(_message.Message):
    __slots__ = ("timestamp", "value")
    TIMESTAMP_FIELD_NUMBER: _ClassVar[int]
    VALUE_FIELD_NUMBER: _ClassVar[int]
    timestamp: _timestamp_pb2.Timestamp
    value: float
    def __init__(self, timestamp: _Optional[_Union[_timestamp_pb2.Timestamp, _Mapping]] = ..., value: _Optional[float] = ...) -> None: ...

class DoubleValues(_message.Message):
    __slots__ = ("metadata", "values")
    METADATA_FIELD_NUMBER: _ClassVar[int]
    VALUES_FIELD_NUMBER: _ClassVar[int]
    metadata: Metadata
    values: _containers.RepeatedCompositeFieldContainer[DoubleValue]
    def __init__(self, metadata: _Optional[_Union[Metadata, _Mapping]] = ..., values: _Optional[_Iterable[_Union[DoubleValue, _Mapping]]] = ...) -> None: ...

class StringValue(_message.Message):
    __slots__ = ("timestamp", "value")
    TIMESTAMP_FIELD_NUMBER: _ClassVar[int]
    VALUE_FIELD_NUMBER: _ClassVar[int]
    timestamp: _timestamp_pb2.Timestamp
    value: str
    def __init__(self, timestamp: _Optional[_Union[_timestamp_pb2.Timestamp, _Mapping]] = ..., value: _Optional[str] = ...) -> None: ...

class StringValues(_message.Message):
    __slots__ = ("metadata", "values")
    METADATA_FIELD_NUMBER: _ClassVar[int]
    VALUES_FIELD_NUMBER: _ClassVar[int]
    metadata: Metadata
    values: _containers.RepeatedCompositeFieldContainer[StringValue]
    def __init__(self, metadata: _Optional[_Union[Metadata, _Mapping]] = ..., values: _Optional[_Iterable[_Union[StringValue, _Mapping]]] = ...) -> None: ...

class EnumValue(_message.Message):
    __slots__ = ("timestamp", "value")
    TIMESTAMP_FIELD_NUMBER: _ClassVar[int]
    VALUE_FIELD_NUMBER: _ClassVar[int]
    timestamp: _timestamp_pb2.Timestamp
    value: int
    def __init__(self, timestamp: _Optional[_Union[_timestamp_pb2.Timestamp, _Mapping]] = ..., value: _Optional[int] = ...) -> None: ...

class EnumValues(_message.Message):
    __slots__ = ("metadata", "values")
    METADATA_FIELD_NUMBER: _ClassVar[int]
    VALUES_FIELD_NUMBER: _ClassVar[int]
    metadata: Metadata
    values: _containers.RepeatedCompositeFieldContainer[EnumValue]
    def __init__(self, metadata: _Optional[_Union[Metadata, _Mapping]] = ..., values: _Optional[_Iterable[_Union[EnumValue, _Mapping]]] = ...) -> None: ...

class BitFieldValue(_message.Message):
    __slots__ = ("timestamp", "value")
    TIMESTAMP_FIELD_NUMBER: _ClassVar[int]
    VALUE_FIELD_NUMBER: _ClassVar[int]
    timestamp: _timestamp_pb2.Timestamp
    value: int
    def __init__(self, timestamp: _Optional[_Union[_timestamp_pb2.Timestamp, _Mapping]] = ..., value: _Optional[int] = ...) -> None: ...

class BitFieldElementValues(_message.Message):
    __slots__ = ("name", "values")
    NAME_FIELD_NUMBER: _ClassVar[int]
    VALUES_FIELD_NUMBER: _ClassVar[int]
    name: str
    values: _containers.RepeatedCompositeFieldContainer[BitFieldValue]
    def __init__(self, name: _Optional[str] = ..., values: _Optional[_Iterable[_Union[BitFieldValue, _Mapping]]] = ...) -> None: ...

class BitFieldValues(_message.Message):
    __slots__ = ("metadata", "values")
    METADATA_FIELD_NUMBER: _ClassVar[int]
    VALUES_FIELD_NUMBER: _ClassVar[int]
    metadata: Metadata
    values: _containers.RepeatedCompositeFieldContainer[BitFieldElementValues]
    def __init__(self, metadata: _Optional[_Union[Metadata, _Mapping]] = ..., values: _Optional[_Iterable[_Union[BitFieldElementValues, _Mapping]]] = ...) -> None: ...

class BoolValue(_message.Message):
    __slots__ = ("timestamp", "value")
    TIMESTAMP_FIELD_NUMBER: _ClassVar[int]
    VALUE_FIELD_NUMBER: _ClassVar[int]
    timestamp: _timestamp_pb2.Timestamp
    value: bool
    def __init__(self, timestamp: _Optional[_Union[_timestamp_pb2.Timestamp, _Mapping]] = ..., value: bool = ...) -> None: ...

class BoolValues(_message.Message):
    __slots__ = ("metadata", "values")
    METADATA_FIELD_NUMBER: _ClassVar[int]
    VALUES_FIELD_NUMBER: _ClassVar[int]
    metadata: Metadata
    values: _containers.RepeatedCompositeFieldContainer[BoolValue]
    def __init__(self, metadata: _Optional[_Union[Metadata, _Mapping]] = ..., values: _Optional[_Iterable[_Union[BoolValue, _Mapping]]] = ...) -> None: ...

class FloatValue(_message.Message):
    __slots__ = ("timestamp", "value")
    TIMESTAMP_FIELD_NUMBER: _ClassVar[int]
    VALUE_FIELD_NUMBER: _ClassVar[int]
    timestamp: _timestamp_pb2.Timestamp
    value: float
    def __init__(self, timestamp: _Optional[_Union[_timestamp_pb2.Timestamp, _Mapping]] = ..., value: _Optional[float] = ...) -> None: ...

class FloatValues(_message.Message):
    __slots__ = ("metadata", "values")
    METADATA_FIELD_NUMBER: _ClassVar[int]
    VALUES_FIELD_NUMBER: _ClassVar[int]
    metadata: Metadata
    values: _containers.RepeatedCompositeFieldContainer[FloatValue]
    def __init__(self, metadata: _Optional[_Union[Metadata, _Mapping]] = ..., values: _Optional[_Iterable[_Union[FloatValue, _Mapping]]] = ...) -> None: ...

class Int32Value(_message.Message):
    __slots__ = ("timestamp", "value")
    TIMESTAMP_FIELD_NUMBER: _ClassVar[int]
    VALUE_FIELD_NUMBER: _ClassVar[int]
    timestamp: _timestamp_pb2.Timestamp
    value: int
    def __init__(self, timestamp: _Optional[_Union[_timestamp_pb2.Timestamp, _Mapping]] = ..., value: _Optional[int] = ...) -> None: ...

class Int32Values(_message.Message):
    __slots__ = ("metadata", "values")
    METADATA_FIELD_NUMBER: _ClassVar[int]
    VALUES_FIELD_NUMBER: _ClassVar[int]
    metadata: Metadata
    values: _containers.RepeatedCompositeFieldContainer[Int32Value]
    def __init__(self, metadata: _Optional[_Union[Metadata, _Mapping]] = ..., values: _Optional[_Iterable[_Union[Int32Value, _Mapping]]] = ...) -> None: ...

class Uint32Value(_message.Message):
    __slots__ = ("timestamp", "value")
    TIMESTAMP_FIELD_NUMBER: _ClassVar[int]
    VALUE_FIELD_NUMBER: _ClassVar[int]
    timestamp: _timestamp_pb2.Timestamp
    value: int
    def __init__(self, timestamp: _Optional[_Union[_timestamp_pb2.Timestamp, _Mapping]] = ..., value: _Optional[int] = ...) -> None: ...

class Uint32Values(_message.Message):
    __slots__ = ("metadata", "values")
    METADATA_FIELD_NUMBER: _ClassVar[int]
    VALUES_FIELD_NUMBER: _ClassVar[int]
    metadata: Metadata
    values: _containers.RepeatedCompositeFieldContainer[Uint32Value]
    def __init__(self, metadata: _Optional[_Union[Metadata, _Mapping]] = ..., values: _Optional[_Iterable[_Union[Uint32Value, _Mapping]]] = ...) -> None: ...

class Int64Value(_message.Message):
    __slots__ = ("timestamp", "value")
    TIMESTAMP_FIELD_NUMBER: _ClassVar[int]
    VALUE_FIELD_NUMBER: _ClassVar[int]
    timestamp: _timestamp_pb2.Timestamp
    value: int
    def __init__(self, timestamp: _Optional[_Union[_timestamp_pb2.Timestamp, _Mapping]] = ..., value: _Optional[int] = ...) -> None: ...

class Int64Values(_message.Message):
    __slots__ = ("metadata", "values")
    METADATA_FIELD_NUMBER: _ClassVar[int]
    VALUES_FIELD_NUMBER: _ClassVar[int]
    metadata: Metadata
    values: _containers.RepeatedCompositeFieldContainer[Int64Value]
    def __init__(self, metadata: _Optional[_Union[Metadata, _Mapping]] = ..., values: _Optional[_Iterable[_Union[Int64Value, _Mapping]]] = ...) -> None: ...

class Uint64Value(_message.Message):
    __slots__ = ("timestamp", "value")
    TIMESTAMP_FIELD_NUMBER: _ClassVar[int]
    VALUE_FIELD_NUMBER: _ClassVar[int]
    timestamp: _timestamp_pb2.Timestamp
    value: int
    def __init__(self, timestamp: _Optional[_Union[_timestamp_pb2.Timestamp, _Mapping]] = ..., value: _Optional[int] = ...) -> None: ...

class Uint64Values(_message.Message):
    __slots__ = ("metadata", "values")
    METADATA_FIELD_NUMBER: _ClassVar[int]
    VALUES_FIELD_NUMBER: _ClassVar[int]
    metadata: Metadata
    values: _containers.RepeatedCompositeFieldContainer[Uint64Value]
    def __init__(self, metadata: _Optional[_Union[Metadata, _Mapping]] = ..., values: _Optional[_Iterable[_Union[Uint64Value, _Mapping]]] = ...) -> None: ...
