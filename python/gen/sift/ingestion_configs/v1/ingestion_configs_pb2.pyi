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

class IngestionConfig(_message.Message):
    __slots__ = ("ingestion_config_id", "asset_id", "client_key")
    INGESTION_CONFIG_ID_FIELD_NUMBER: _ClassVar[int]
    ASSET_ID_FIELD_NUMBER: _ClassVar[int]
    CLIENT_KEY_FIELD_NUMBER: _ClassVar[int]
    ingestion_config_id: str
    asset_id: str
    client_key: str
    def __init__(self, ingestion_config_id: _Optional[str] = ..., asset_id: _Optional[str] = ..., client_key: _Optional[str] = ...) -> None: ...

class FlowConfig(_message.Message):
    __slots__ = ("name", "channels")
    NAME_FIELD_NUMBER: _ClassVar[int]
    CHANNELS_FIELD_NUMBER: _ClassVar[int]
    name: str
    channels: _containers.RepeatedCompositeFieldContainer[ChannelConfig]
    def __init__(self, name: _Optional[str] = ..., channels: _Optional[_Iterable[_Union[ChannelConfig, _Mapping]]] = ...) -> None: ...

class ChannelConfig(_message.Message):
    __slots__ = ("name", "component", "unit", "description", "data_type", "enum_types", "bit_field_elements")
    NAME_FIELD_NUMBER: _ClassVar[int]
    COMPONENT_FIELD_NUMBER: _ClassVar[int]
    UNIT_FIELD_NUMBER: _ClassVar[int]
    DESCRIPTION_FIELD_NUMBER: _ClassVar[int]
    DATA_TYPE_FIELD_NUMBER: _ClassVar[int]
    ENUM_TYPES_FIELD_NUMBER: _ClassVar[int]
    BIT_FIELD_ELEMENTS_FIELD_NUMBER: _ClassVar[int]
    name: str
    component: str
    unit: str
    description: str
    data_type: _channel_data_type_pb2.ChannelDataType
    enum_types: _containers.RepeatedCompositeFieldContainer[_channel_enum_type_pb2.ChannelEnumType]
    bit_field_elements: _containers.RepeatedCompositeFieldContainer[_channel_bit_field_element_pb2.ChannelBitFieldElement]
    def __init__(self, name: _Optional[str] = ..., component: _Optional[str] = ..., unit: _Optional[str] = ..., description: _Optional[str] = ..., data_type: _Optional[_Union[_channel_data_type_pb2.ChannelDataType, str]] = ..., enum_types: _Optional[_Iterable[_Union[_channel_enum_type_pb2.ChannelEnumType, _Mapping]]] = ..., bit_field_elements: _Optional[_Iterable[_Union[_channel_bit_field_element_pb2.ChannelBitFieldElement, _Mapping]]] = ...) -> None: ...

class GetIngestionConfigRequest(_message.Message):
    __slots__ = ("ingestion_config_id",)
    INGESTION_CONFIG_ID_FIELD_NUMBER: _ClassVar[int]
    ingestion_config_id: str
    def __init__(self, ingestion_config_id: _Optional[str] = ...) -> None: ...

class GetIngestionConfigResponse(_message.Message):
    __slots__ = ("ingestion_config",)
    INGESTION_CONFIG_FIELD_NUMBER: _ClassVar[int]
    ingestion_config: IngestionConfig
    def __init__(self, ingestion_config: _Optional[_Union[IngestionConfig, _Mapping]] = ...) -> None: ...

class CreateIngestionConfigRequest(_message.Message):
    __slots__ = ("asset_name", "flows", "organization_id", "client_key")
    ASSET_NAME_FIELD_NUMBER: _ClassVar[int]
    FLOWS_FIELD_NUMBER: _ClassVar[int]
    ORGANIZATION_ID_FIELD_NUMBER: _ClassVar[int]
    CLIENT_KEY_FIELD_NUMBER: _ClassVar[int]
    asset_name: str
    flows: _containers.RepeatedCompositeFieldContainer[FlowConfig]
    organization_id: str
    client_key: str
    def __init__(self, asset_name: _Optional[str] = ..., flows: _Optional[_Iterable[_Union[FlowConfig, _Mapping]]] = ..., organization_id: _Optional[str] = ..., client_key: _Optional[str] = ...) -> None: ...

class CreateIngestionConfigResponse(_message.Message):
    __slots__ = ("ingestion_config",)
    INGESTION_CONFIG_FIELD_NUMBER: _ClassVar[int]
    ingestion_config: IngestionConfig
    def __init__(self, ingestion_config: _Optional[_Union[IngestionConfig, _Mapping]] = ...) -> None: ...

class ListIngestionConfigsRequest(_message.Message):
    __slots__ = ("page_size", "page_token", "filter")
    PAGE_SIZE_FIELD_NUMBER: _ClassVar[int]
    PAGE_TOKEN_FIELD_NUMBER: _ClassVar[int]
    FILTER_FIELD_NUMBER: _ClassVar[int]
    page_size: int
    page_token: str
    filter: str
    def __init__(self, page_size: _Optional[int] = ..., page_token: _Optional[str] = ..., filter: _Optional[str] = ...) -> None: ...

class ListIngestionConfigsResponse(_message.Message):
    __slots__ = ("ingestion_configs", "next_page_token")
    INGESTION_CONFIGS_FIELD_NUMBER: _ClassVar[int]
    NEXT_PAGE_TOKEN_FIELD_NUMBER: _ClassVar[int]
    ingestion_configs: _containers.RepeatedCompositeFieldContainer[IngestionConfig]
    next_page_token: str
    def __init__(self, ingestion_configs: _Optional[_Iterable[_Union[IngestionConfig, _Mapping]]] = ..., next_page_token: _Optional[str] = ...) -> None: ...

class CreateIngestionConfigFlowsRequest(_message.Message):
    __slots__ = ("ingestion_config_id", "flows")
    INGESTION_CONFIG_ID_FIELD_NUMBER: _ClassVar[int]
    FLOWS_FIELD_NUMBER: _ClassVar[int]
    ingestion_config_id: str
    flows: _containers.RepeatedCompositeFieldContainer[FlowConfig]
    def __init__(self, ingestion_config_id: _Optional[str] = ..., flows: _Optional[_Iterable[_Union[FlowConfig, _Mapping]]] = ...) -> None: ...

class CreateIngestionConfigFlowsResponse(_message.Message):
    __slots__ = ()
    def __init__(self) -> None: ...

class ListIngestionConfigFlowsRequest(_message.Message):
    __slots__ = ("ingestion_config_id", "page_size", "page_token", "filter")
    INGESTION_CONFIG_ID_FIELD_NUMBER: _ClassVar[int]
    PAGE_SIZE_FIELD_NUMBER: _ClassVar[int]
    PAGE_TOKEN_FIELD_NUMBER: _ClassVar[int]
    FILTER_FIELD_NUMBER: _ClassVar[int]
    ingestion_config_id: str
    page_size: int
    page_token: str
    filter: str
    def __init__(self, ingestion_config_id: _Optional[str] = ..., page_size: _Optional[int] = ..., page_token: _Optional[str] = ..., filter: _Optional[str] = ...) -> None: ...

class ListIngestionConfigFlowsResponse(_message.Message):
    __slots__ = ("flows", "next_page_token")
    FLOWS_FIELD_NUMBER: _ClassVar[int]
    NEXT_PAGE_TOKEN_FIELD_NUMBER: _ClassVar[int]
    flows: _containers.RepeatedCompositeFieldContainer[FlowConfig]
    next_page_token: str
    def __init__(self, flows: _Optional[_Iterable[_Union[FlowConfig, _Mapping]]] = ..., next_page_token: _Optional[str] = ...) -> None: ...
