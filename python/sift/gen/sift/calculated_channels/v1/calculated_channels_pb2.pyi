from google.api import annotations_pb2 as _annotations_pb2
from google.api import field_behavior_pb2 as _field_behavior_pb2
from protoc_gen_openapiv2.options import annotations_pb2 as _annotations_pb2_1
from sift.common.type.v1 import channel_data_type_pb2 as _channel_data_type_pb2
from google.protobuf.internal import containers as _containers
from google.protobuf.internal import enum_type_wrapper as _enum_type_wrapper
from google.protobuf import descriptor as _descriptor
from google.protobuf import message as _message
from typing import ClassVar as _ClassVar, Iterable as _Iterable, Mapping as _Mapping, Optional as _Optional, Union as _Union

DESCRIPTOR: _descriptor.FileDescriptor

class ExpressionIdentifierType(int, metaclass=_enum_type_wrapper.EnumTypeWrapper):
    __slots__ = ()
    EXPRESSION_IDENTIFIER_TYPE_UNSPECIFIED: _ClassVar[ExpressionIdentifierType]
    EXPRESSION_IDENTIFIER_TYPE_FUNCTION: _ClassVar[ExpressionIdentifierType]
    EXPRESSION_IDENTIFIER_TYPE_CHANNEL: _ClassVar[ExpressionIdentifierType]

class ExpressionMode(int, metaclass=_enum_type_wrapper.EnumTypeWrapper):
    __slots__ = ()
    EXPRESSION_MODE_UNSPECIFIED: _ClassVar[ExpressionMode]
    EXPRESSION_MODE_RULES: _ClassVar[ExpressionMode]
    EXPRESSION_MODE_CALCULATED_CHANNELS: _ClassVar[ExpressionMode]
EXPRESSION_IDENTIFIER_TYPE_UNSPECIFIED: ExpressionIdentifierType
EXPRESSION_IDENTIFIER_TYPE_FUNCTION: ExpressionIdentifierType
EXPRESSION_IDENTIFIER_TYPE_CHANNEL: ExpressionIdentifierType
EXPRESSION_MODE_UNSPECIFIED: ExpressionMode
EXPRESSION_MODE_RULES: ExpressionMode
EXPRESSION_MODE_CALCULATED_CHANNELS: ExpressionMode

class ExpressionRequest(_message.Message):
    __slots__ = ("channel_references", "expression")
    class ChannelReferencesEntry(_message.Message):
        __slots__ = ("key", "value")
        KEY_FIELD_NUMBER: _ClassVar[int]
        VALUE_FIELD_NUMBER: _ClassVar[int]
        key: str
        value: str
        def __init__(self, key: _Optional[str] = ..., value: _Optional[str] = ...) -> None: ...
    CHANNEL_REFERENCES_FIELD_NUMBER: _ClassVar[int]
    EXPRESSION_FIELD_NUMBER: _ClassVar[int]
    channel_references: _containers.ScalarMap[str, str]
    expression: str
    def __init__(self, channel_references: _Optional[_Mapping[str, str]] = ..., expression: _Optional[str] = ...) -> None: ...

class ListExpressionIdentifiersRequest(_message.Message):
    __slots__ = ("page_size", "page_token", "mode")
    PAGE_SIZE_FIELD_NUMBER: _ClassVar[int]
    PAGE_TOKEN_FIELD_NUMBER: _ClassVar[int]
    MODE_FIELD_NUMBER: _ClassVar[int]
    page_size: int
    page_token: str
    mode: ExpressionMode
    def __init__(self, page_size: _Optional[int] = ..., page_token: _Optional[str] = ..., mode: _Optional[_Union[ExpressionMode, str]] = ...) -> None: ...

class ListExpressionIdentifiersResponse(_message.Message):
    __slots__ = ("identifiers",)
    IDENTIFIERS_FIELD_NUMBER: _ClassVar[int]
    identifiers: _containers.RepeatedCompositeFieldContainer[ExpressionIdentifier]
    def __init__(self, identifiers: _Optional[_Iterable[_Union[ExpressionIdentifier, _Mapping]]] = ...) -> None: ...

class ExpressionIdentifier(_message.Message):
    __slots__ = ("name", "description", "type", "display_name")
    NAME_FIELD_NUMBER: _ClassVar[int]
    DESCRIPTION_FIELD_NUMBER: _ClassVar[int]
    TYPE_FIELD_NUMBER: _ClassVar[int]
    DISPLAY_NAME_FIELD_NUMBER: _ClassVar[int]
    name: str
    description: str
    type: ExpressionIdentifierType
    display_name: str
    def __init__(self, name: _Optional[str] = ..., description: _Optional[str] = ..., type: _Optional[_Union[ExpressionIdentifierType, str]] = ..., display_name: _Optional[str] = ...) -> None: ...

class ValidateExpressionRequest(_message.Message):
    __slots__ = ("expression", "mode")
    EXPRESSION_FIELD_NUMBER: _ClassVar[int]
    MODE_FIELD_NUMBER: _ClassVar[int]
    expression: ExpressionRequest
    mode: ExpressionMode
    def __init__(self, expression: _Optional[_Union[ExpressionRequest, _Mapping]] = ..., mode: _Optional[_Union[ExpressionMode, str]] = ...) -> None: ...

class ValidateExpressionResponse(_message.Message):
    __slots__ = ("error", "success")
    ERROR_FIELD_NUMBER: _ClassVar[int]
    SUCCESS_FIELD_NUMBER: _ClassVar[int]
    error: ErrorValidatingExpressionResult
    success: SuccessValidatingExpressionResult
    def __init__(self, error: _Optional[_Union[ErrorValidatingExpressionResult, _Mapping]] = ..., success: _Optional[_Union[SuccessValidatingExpressionResult, _Mapping]] = ...) -> None: ...

class ErrorValidatingExpressionResult(_message.Message):
    __slots__ = ("error_message",)
    ERROR_MESSAGE_FIELD_NUMBER: _ClassVar[int]
    error_message: str
    def __init__(self, error_message: _Optional[str] = ...) -> None: ...

class SuccessValidatingExpressionResult(_message.Message):
    __slots__ = ("data_type",)
    DATA_TYPE_FIELD_NUMBER: _ClassVar[int]
    data_type: _channel_data_type_pb2.ChannelDataType
    def __init__(self, data_type: _Optional[_Union[_channel_data_type_pb2.ChannelDataType, str]] = ...) -> None: ...
