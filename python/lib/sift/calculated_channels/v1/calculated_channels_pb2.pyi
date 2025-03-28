"""
@generated by mypy-protobuf.  Do not edit manually!
isort:skip_file
"""

import builtins
import collections.abc
import google.protobuf.descriptor
import google.protobuf.internal.containers
import google.protobuf.internal.enum_type_wrapper
import google.protobuf.message
import sift.common.type.v1.channel_data_type_pb2
import sys
import typing

if sys.version_info >= (3, 10):
    import typing as typing_extensions
else:
    import typing_extensions

DESCRIPTOR: google.protobuf.descriptor.FileDescriptor

class _ExpressionIdentifierType:
    ValueType = typing.NewType("ValueType", builtins.int)
    V: typing_extensions.TypeAlias = ValueType

class _ExpressionIdentifierTypeEnumTypeWrapper(google.protobuf.internal.enum_type_wrapper._EnumTypeWrapper[_ExpressionIdentifierType.ValueType], builtins.type):
    DESCRIPTOR: google.protobuf.descriptor.EnumDescriptor
    EXPRESSION_IDENTIFIER_TYPE_UNSPECIFIED: _ExpressionIdentifierType.ValueType  # 0
    EXPRESSION_IDENTIFIER_TYPE_FUNCTION: _ExpressionIdentifierType.ValueType  # 1
    EXPRESSION_IDENTIFIER_TYPE_CHANNEL: _ExpressionIdentifierType.ValueType  # 2

class ExpressionIdentifierType(_ExpressionIdentifierType, metaclass=_ExpressionIdentifierTypeEnumTypeWrapper): ...

EXPRESSION_IDENTIFIER_TYPE_UNSPECIFIED: ExpressionIdentifierType.ValueType  # 0
EXPRESSION_IDENTIFIER_TYPE_FUNCTION: ExpressionIdentifierType.ValueType  # 1
EXPRESSION_IDENTIFIER_TYPE_CHANNEL: ExpressionIdentifierType.ValueType  # 2
global___ExpressionIdentifierType = ExpressionIdentifierType

class _ExpressionIdentifierLibrary:
    ValueType = typing.NewType("ValueType", builtins.int)
    V: typing_extensions.TypeAlias = ValueType

class _ExpressionIdentifierLibraryEnumTypeWrapper(google.protobuf.internal.enum_type_wrapper._EnumTypeWrapper[_ExpressionIdentifierLibrary.ValueType], builtins.type):
    DESCRIPTOR: google.protobuf.descriptor.EnumDescriptor
    EXPRESSION_IDENTIFIER_LIBRARY_UNSPECIFIED: _ExpressionIdentifierLibrary.ValueType  # 0
    EXPRESSION_IDENTIFIER_LIBRARY_MATH: _ExpressionIdentifierLibrary.ValueType  # 1
    EXPRESSION_IDENTIFIER_LIBRARY_STRING: _ExpressionIdentifierLibrary.ValueType  # 2
    EXPRESSION_IDENTIFIER_LIBRARY_LIST: _ExpressionIdentifierLibrary.ValueType  # 3
    EXPRESSION_IDENTIFIER_LIBRARY_ITER: _ExpressionIdentifierLibrary.ValueType  # 4
    EXPRESSION_IDENTIFIER_LIBRARY_STATEFUL: _ExpressionIdentifierLibrary.ValueType  # 5
    EXPRESSION_IDENTIFIER_LIBRARY_SUMMARY: _ExpressionIdentifierLibrary.ValueType  # 6

class ExpressionIdentifierLibrary(_ExpressionIdentifierLibrary, metaclass=_ExpressionIdentifierLibraryEnumTypeWrapper): ...

EXPRESSION_IDENTIFIER_LIBRARY_UNSPECIFIED: ExpressionIdentifierLibrary.ValueType  # 0
EXPRESSION_IDENTIFIER_LIBRARY_MATH: ExpressionIdentifierLibrary.ValueType  # 1
EXPRESSION_IDENTIFIER_LIBRARY_STRING: ExpressionIdentifierLibrary.ValueType  # 2
EXPRESSION_IDENTIFIER_LIBRARY_LIST: ExpressionIdentifierLibrary.ValueType  # 3
EXPRESSION_IDENTIFIER_LIBRARY_ITER: ExpressionIdentifierLibrary.ValueType  # 4
EXPRESSION_IDENTIFIER_LIBRARY_STATEFUL: ExpressionIdentifierLibrary.ValueType  # 5
EXPRESSION_IDENTIFIER_LIBRARY_SUMMARY: ExpressionIdentifierLibrary.ValueType  # 6
global___ExpressionIdentifierLibrary = ExpressionIdentifierLibrary

class _ExpressionMode:
    ValueType = typing.NewType("ValueType", builtins.int)
    V: typing_extensions.TypeAlias = ValueType

class _ExpressionModeEnumTypeWrapper(google.protobuf.internal.enum_type_wrapper._EnumTypeWrapper[_ExpressionMode.ValueType], builtins.type):
    DESCRIPTOR: google.protobuf.descriptor.EnumDescriptor
    EXPRESSION_MODE_UNSPECIFIED: _ExpressionMode.ValueType  # 0
    EXPRESSION_MODE_RULES: _ExpressionMode.ValueType  # 1
    EXPRESSION_MODE_CALCULATED_CHANNELS: _ExpressionMode.ValueType  # 2
    EXPRESSION_MODE_RULER: _ExpressionMode.ValueType  # 3
    EXPRESSION_MODE_STRUCTURED_DATA: _ExpressionMode.ValueType  # 4

class ExpressionMode(_ExpressionMode, metaclass=_ExpressionModeEnumTypeWrapper): ...

EXPRESSION_MODE_UNSPECIFIED: ExpressionMode.ValueType  # 0
EXPRESSION_MODE_RULES: ExpressionMode.ValueType  # 1
EXPRESSION_MODE_CALCULATED_CHANNELS: ExpressionMode.ValueType  # 2
EXPRESSION_MODE_RULER: ExpressionMode.ValueType  # 3
EXPRESSION_MODE_STRUCTURED_DATA: ExpressionMode.ValueType  # 4
global___ExpressionMode = ExpressionMode

@typing.final
class ExpressionChannelReference(google.protobuf.message.Message):
    DESCRIPTOR: google.protobuf.descriptor.Descriptor

    CHANNEL_REFERENCE_FIELD_NUMBER: builtins.int
    CHANNEL_ID_FIELD_NUMBER: builtins.int
    channel_reference: builtins.str
    channel_id: builtins.str
    def __init__(
        self,
        *,
        channel_reference: builtins.str = ...,
        channel_id: builtins.str = ...,
    ) -> None: ...
    def ClearField(self, field_name: typing.Literal["channel_id", b"channel_id", "channel_reference", b"channel_reference"]) -> None: ...

global___ExpressionChannelReference = ExpressionChannelReference

@typing.final
class ExpressionRequest(google.protobuf.message.Message):
    DESCRIPTOR: google.protobuf.descriptor.Descriptor

    @typing.final
    class ChannelReferencesEntry(google.protobuf.message.Message):
        DESCRIPTOR: google.protobuf.descriptor.Descriptor

        KEY_FIELD_NUMBER: builtins.int
        VALUE_FIELD_NUMBER: builtins.int
        key: builtins.str
        value: builtins.str
        def __init__(
            self,
            *,
            key: builtins.str = ...,
            value: builtins.str = ...,
        ) -> None: ...
        def ClearField(self, field_name: typing.Literal["key", b"key", "value", b"value"]) -> None: ...

    CHANNEL_REFERENCES_FIELD_NUMBER: builtins.int
    EXPRESSION_FIELD_NUMBER: builtins.int
    EXPRESSION_CHANNEL_REFERENCES_FIELD_NUMBER: builtins.int
    expression: builtins.str
    @property
    def channel_references(self) -> google.protobuf.internal.containers.ScalarMap[builtins.str, builtins.str]:
        """A map from the channel reference in the expression string (e.g. $1) to the channel id (uuid).
        This is deprecated and should be passed in expression_channel_references instead.
        """

    @property
    def expression_channel_references(self) -> google.protobuf.internal.containers.RepeatedCompositeFieldContainer[global___ExpressionChannelReference]: ...
    def __init__(
        self,
        *,
        channel_references: collections.abc.Mapping[builtins.str, builtins.str] | None = ...,
        expression: builtins.str = ...,
        expression_channel_references: collections.abc.Iterable[global___ExpressionChannelReference] | None = ...,
    ) -> None: ...
    def ClearField(self, field_name: typing.Literal["channel_references", b"channel_references", "expression", b"expression", "expression_channel_references", b"expression_channel_references"]) -> None: ...

global___ExpressionRequest = ExpressionRequest

@typing.final
class ListExpressionIdentifiersRequest(google.protobuf.message.Message):
    DESCRIPTOR: google.protobuf.descriptor.Descriptor

    PAGE_SIZE_FIELD_NUMBER: builtins.int
    PAGE_TOKEN_FIELD_NUMBER: builtins.int
    MODE_FIELD_NUMBER: builtins.int
    page_size: builtins.int
    """Defaults to 1000. Max of 10,000."""
    page_token: builtins.str
    mode: global___ExpressionMode.ValueType
    def __init__(
        self,
        *,
        page_size: builtins.int = ...,
        page_token: builtins.str = ...,
        mode: global___ExpressionMode.ValueType = ...,
    ) -> None: ...
    def ClearField(self, field_name: typing.Literal["mode", b"mode", "page_size", b"page_size", "page_token", b"page_token"]) -> None: ...

global___ListExpressionIdentifiersRequest = ListExpressionIdentifiersRequest

@typing.final
class ListExpressionIdentifiersResponse(google.protobuf.message.Message):
    DESCRIPTOR: google.protobuf.descriptor.Descriptor

    IDENTIFIERS_FIELD_NUMBER: builtins.int
    @property
    def identifiers(self) -> google.protobuf.internal.containers.RepeatedCompositeFieldContainer[global___ExpressionIdentifier]: ...
    def __init__(
        self,
        *,
        identifiers: collections.abc.Iterable[global___ExpressionIdentifier] | None = ...,
    ) -> None: ...
    def ClearField(self, field_name: typing.Literal["identifiers", b"identifiers"]) -> None: ...

global___ListExpressionIdentifiersResponse = ListExpressionIdentifiersResponse

@typing.final
class ExpressionIdentifier(google.protobuf.message.Message):
    DESCRIPTOR: google.protobuf.descriptor.Descriptor

    NAME_FIELD_NUMBER: builtins.int
    DESCRIPTION_FIELD_NUMBER: builtins.int
    TYPE_FIELD_NUMBER: builtins.int
    DISPLAY_NAME_FIELD_NUMBER: builtins.int
    LIBRARY_FIELD_NUMBER: builtins.int
    name: builtins.str
    description: builtins.str
    type: global___ExpressionIdentifierType.ValueType
    display_name: builtins.str
    library: global___ExpressionIdentifierLibrary.ValueType
    def __init__(
        self,
        *,
        name: builtins.str = ...,
        description: builtins.str = ...,
        type: global___ExpressionIdentifierType.ValueType = ...,
        display_name: builtins.str = ...,
        library: global___ExpressionIdentifierLibrary.ValueType = ...,
    ) -> None: ...
    def ClearField(self, field_name: typing.Literal["description", b"description", "display_name", b"display_name", "library", b"library", "name", b"name", "type", b"type"]) -> None: ...

global___ExpressionIdentifier = ExpressionIdentifier

@typing.final
class ValidateExpressionRequest(google.protobuf.message.Message):
    DESCRIPTOR: google.protobuf.descriptor.Descriptor

    EXPRESSION_FIELD_NUMBER: builtins.int
    MODE_FIELD_NUMBER: builtins.int
    mode: global___ExpressionMode.ValueType
    @property
    def expression(self) -> global___ExpressionRequest: ...
    def __init__(
        self,
        *,
        expression: global___ExpressionRequest | None = ...,
        mode: global___ExpressionMode.ValueType = ...,
    ) -> None: ...
    def HasField(self, field_name: typing.Literal["expression", b"expression"]) -> builtins.bool: ...
    def ClearField(self, field_name: typing.Literal["expression", b"expression", "mode", b"mode"]) -> None: ...

global___ValidateExpressionRequest = ValidateExpressionRequest

@typing.final
class ValidateExpressionResponse(google.protobuf.message.Message):
    DESCRIPTOR: google.protobuf.descriptor.Descriptor

    ERROR_FIELD_NUMBER: builtins.int
    SUCCESS_FIELD_NUMBER: builtins.int
    @property
    def error(self) -> global___ErrorValidatingExpressionResult: ...
    @property
    def success(self) -> global___SuccessValidatingExpressionResult: ...
    def __init__(
        self,
        *,
        error: global___ErrorValidatingExpressionResult | None = ...,
        success: global___SuccessValidatingExpressionResult | None = ...,
    ) -> None: ...
    def HasField(self, field_name: typing.Literal["error", b"error", "result", b"result", "success", b"success"]) -> builtins.bool: ...
    def ClearField(self, field_name: typing.Literal["error", b"error", "result", b"result", "success", b"success"]) -> None: ...
    def WhichOneof(self, oneof_group: typing.Literal["result", b"result"]) -> typing.Literal["error", "success"] | None: ...

global___ValidateExpressionResponse = ValidateExpressionResponse

@typing.final
class ErrorValidatingExpressionResult(google.protobuf.message.Message):
    DESCRIPTOR: google.protobuf.descriptor.Descriptor

    ERROR_MESSAGE_FIELD_NUMBER: builtins.int
    error_message: builtins.str
    def __init__(
        self,
        *,
        error_message: builtins.str = ...,
    ) -> None: ...
    def ClearField(self, field_name: typing.Literal["error_message", b"error_message"]) -> None: ...

global___ErrorValidatingExpressionResult = ErrorValidatingExpressionResult

@typing.final
class SuccessValidatingExpressionResult(google.protobuf.message.Message):
    DESCRIPTOR: google.protobuf.descriptor.Descriptor

    DATA_TYPE_FIELD_NUMBER: builtins.int
    data_type: sift.common.type.v1.channel_data_type_pb2.ChannelDataType.ValueType
    def __init__(
        self,
        *,
        data_type: sift.common.type.v1.channel_data_type_pb2.ChannelDataType.ValueType = ...,
    ) -> None: ...
    def ClearField(self, field_name: typing.Literal["data_type", b"data_type"]) -> None: ...

global___SuccessValidatingExpressionResult = SuccessValidatingExpressionResult
