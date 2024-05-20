from google.api import annotations_pb2 as _annotations_pb2
from google.api import field_behavior_pb2 as _field_behavior_pb2
from protoc_gen_openapiv2.options import annotations_pb2 as _annotations_pb2_1
from sift.common.type.v1 import channel_bit_field_element_pb2 as _channel_bit_field_element_pb2
from sift.common.type.v1 import channel_data_type_pb2 as _channel_data_type_pb2
from sift.common.type.v1 import channel_enum_type_pb2 as _channel_enum_type_pb2
from google.protobuf.internal import containers as _containers
from google.protobuf import descriptor as _descriptor
from google.protobuf import message as _message
from typing import ClassVar as _ClassVar, Iterable as _Iterable, Mapping as _Mapping, Optional as _Optional, Union as _Union

DESCRIPTOR: _descriptor.FileDescriptor

class CreateChannelSchemaRequest(_message.Message):
    __slots__ = ("component", "channel", "unit", "data_type", "enum_types", "bit_field_elements", "asset_name", "organization_id")
    COMPONENT_FIELD_NUMBER: _ClassVar[int]
    CHANNEL_FIELD_NUMBER: _ClassVar[int]
    UNIT_FIELD_NUMBER: _ClassVar[int]
    DATA_TYPE_FIELD_NUMBER: _ClassVar[int]
    ENUM_TYPES_FIELD_NUMBER: _ClassVar[int]
    BIT_FIELD_ELEMENTS_FIELD_NUMBER: _ClassVar[int]
    ASSET_NAME_FIELD_NUMBER: _ClassVar[int]
    ORGANIZATION_ID_FIELD_NUMBER: _ClassVar[int]
    component: str
    channel: str
    unit: str
    data_type: _channel_data_type_pb2.ChannelDataType
    enum_types: _containers.RepeatedCompositeFieldContainer[_channel_enum_type_pb2.ChannelEnumType]
    bit_field_elements: _containers.RepeatedCompositeFieldContainer[_channel_bit_field_element_pb2.ChannelBitFieldElement]
    asset_name: str
    organization_id: str
    def __init__(self, component: _Optional[str] = ..., channel: _Optional[str] = ..., unit: _Optional[str] = ..., data_type: _Optional[_Union[_channel_data_type_pb2.ChannelDataType, str]] = ..., enum_types: _Optional[_Iterable[_Union[_channel_enum_type_pb2.ChannelEnumType, _Mapping]]] = ..., bit_field_elements: _Optional[_Iterable[_Union[_channel_bit_field_element_pb2.ChannelBitFieldElement, _Mapping]]] = ..., asset_name: _Optional[str] = ..., organization_id: _Optional[str] = ...) -> None: ...

class CreateChannelSchemaResponse(_message.Message):
    __slots__ = ()
    def __init__(self) -> None: ...

class BatchCreateChannelSchemasRequest(_message.Message):
    __slots__ = ("requests", "asset_name", "organization_id")
    REQUESTS_FIELD_NUMBER: _ClassVar[int]
    ASSET_NAME_FIELD_NUMBER: _ClassVar[int]
    ORGANIZATION_ID_FIELD_NUMBER: _ClassVar[int]
    requests: _containers.RepeatedCompositeFieldContainer[CreateChannelSchemaRequest]
    asset_name: str
    organization_id: str
    def __init__(self, requests: _Optional[_Iterable[_Union[CreateChannelSchemaRequest, _Mapping]]] = ..., asset_name: _Optional[str] = ..., organization_id: _Optional[str] = ...) -> None: ...

class BatchCreateChannelSchemasResponse(_message.Message):
    __slots__ = ()
    def __init__(self) -> None: ...
