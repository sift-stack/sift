from google.api import annotations_pb2 as _annotations_pb2
from google.api import field_behavior_pb2 as _field_behavior_pb2
from google.protobuf import timestamp_pb2 as _timestamp_pb2
from protoc_gen_openapiv2.options import annotations_pb2 as _annotations_pb2_1
from sift.common.type.v1 import channel_bit_field_element_pb2 as _channel_bit_field_element_pb2
from sift.common.type.v1 import channel_data_type_pb2 as _channel_data_type_pb2
from sift.common.type.v1 import channel_enum_type_pb2 as _channel_enum_type_pb2
from google.protobuf.internal import containers as _containers
from google.protobuf import descriptor as _descriptor
from google.protobuf import message as _message
from typing import ClassVar as _ClassVar, Iterable as _Iterable, Mapping as _Mapping, Optional as _Optional, Union as _Union

DESCRIPTOR: _descriptor.FileDescriptor

class Channel(_message.Message):
    __slots__ = ("channel_id", "name", "component", "asset_id", "description", "unit_id", "created_date", "modified_date", "created_by_user_id", "modified_by_user_id", "organization_id", "data_type", "enum_types", "bit_field_elements")
    CHANNEL_ID_FIELD_NUMBER: _ClassVar[int]
    NAME_FIELD_NUMBER: _ClassVar[int]
    COMPONENT_FIELD_NUMBER: _ClassVar[int]
    ASSET_ID_FIELD_NUMBER: _ClassVar[int]
    DESCRIPTION_FIELD_NUMBER: _ClassVar[int]
    UNIT_ID_FIELD_NUMBER: _ClassVar[int]
    CREATED_DATE_FIELD_NUMBER: _ClassVar[int]
    MODIFIED_DATE_FIELD_NUMBER: _ClassVar[int]
    CREATED_BY_USER_ID_FIELD_NUMBER: _ClassVar[int]
    MODIFIED_BY_USER_ID_FIELD_NUMBER: _ClassVar[int]
    ORGANIZATION_ID_FIELD_NUMBER: _ClassVar[int]
    DATA_TYPE_FIELD_NUMBER: _ClassVar[int]
    ENUM_TYPES_FIELD_NUMBER: _ClassVar[int]
    BIT_FIELD_ELEMENTS_FIELD_NUMBER: _ClassVar[int]
    channel_id: str
    name: str
    component: str
    asset_id: str
    description: str
    unit_id: str
    created_date: _timestamp_pb2.Timestamp
    modified_date: _timestamp_pb2.Timestamp
    created_by_user_id: str
    modified_by_user_id: str
    organization_id: str
    data_type: _channel_data_type_pb2.ChannelDataType
    enum_types: _containers.RepeatedCompositeFieldContainer[_channel_enum_type_pb2.ChannelEnumType]
    bit_field_elements: _containers.RepeatedCompositeFieldContainer[_channel_bit_field_element_pb2.ChannelBitFieldElement]
    def __init__(self, channel_id: _Optional[str] = ..., name: _Optional[str] = ..., component: _Optional[str] = ..., asset_id: _Optional[str] = ..., description: _Optional[str] = ..., unit_id: _Optional[str] = ..., created_date: _Optional[_Union[_timestamp_pb2.Timestamp, _Mapping]] = ..., modified_date: _Optional[_Union[_timestamp_pb2.Timestamp, _Mapping]] = ..., created_by_user_id: _Optional[str] = ..., modified_by_user_id: _Optional[str] = ..., organization_id: _Optional[str] = ..., data_type: _Optional[_Union[_channel_data_type_pb2.ChannelDataType, str]] = ..., enum_types: _Optional[_Iterable[_Union[_channel_enum_type_pb2.ChannelEnumType, _Mapping]]] = ..., bit_field_elements: _Optional[_Iterable[_Union[_channel_bit_field_element_pb2.ChannelBitFieldElement, _Mapping]]] = ...) -> None: ...

class GetChannelRequest(_message.Message):
    __slots__ = ("channel_id",)
    CHANNEL_ID_FIELD_NUMBER: _ClassVar[int]
    channel_id: str
    def __init__(self, channel_id: _Optional[str] = ...) -> None: ...

class GetChannelResponse(_message.Message):
    __slots__ = ("channel",)
    CHANNEL_FIELD_NUMBER: _ClassVar[int]
    channel: Channel
    def __init__(self, channel: _Optional[_Union[Channel, _Mapping]] = ...) -> None: ...

class ListChannelsRequest(_message.Message):
    __slots__ = ("page_size", "page_token", "filter")
    PAGE_SIZE_FIELD_NUMBER: _ClassVar[int]
    PAGE_TOKEN_FIELD_NUMBER: _ClassVar[int]
    FILTER_FIELD_NUMBER: _ClassVar[int]
    page_size: int
    page_token: str
    filter: str
    def __init__(self, page_size: _Optional[int] = ..., page_token: _Optional[str] = ..., filter: _Optional[str] = ...) -> None: ...

class ListChannelsResponse(_message.Message):
    __slots__ = ("channels", "next_page_token")
    CHANNELS_FIELD_NUMBER: _ClassVar[int]
    NEXT_PAGE_TOKEN_FIELD_NUMBER: _ClassVar[int]
    channels: _containers.RepeatedCompositeFieldContainer[Channel]
    next_page_token: str
    def __init__(self, channels: _Optional[_Iterable[_Union[Channel, _Mapping]]] = ..., next_page_token: _Optional[str] = ...) -> None: ...
