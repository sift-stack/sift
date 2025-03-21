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
import google.protobuf.timestamp_pb2
import sys
import typing

if sys.version_info >= (3, 10):
    import typing as typing_extensions
else:
    import typing_extensions

DESCRIPTOR: google.protobuf.descriptor.FileDescriptor

class _AnnotationLogKind:
    ValueType = typing.NewType("ValueType", builtins.int)
    V: typing_extensions.TypeAlias = ValueType

class _AnnotationLogKindEnumTypeWrapper(google.protobuf.internal.enum_type_wrapper._EnumTypeWrapper[_AnnotationLogKind.ValueType], builtins.type):
    DESCRIPTOR: google.protobuf.descriptor.EnumDescriptor
    ANNOTATION_LOG_KIND_UNSPECIFIED: _AnnotationLogKind.ValueType  # 0
    ANNOTATION_LOG_KIND_COMMENT: _AnnotationLogKind.ValueType  # 1
    ANNOTATION_LOG_KIND_STATE_UPDATE: _AnnotationLogKind.ValueType  # 2
    ANNOTATION_LOG_KIND_ASSIGNED: _AnnotationLogKind.ValueType  # 3

class AnnotationLogKind(_AnnotationLogKind, metaclass=_AnnotationLogKindEnumTypeWrapper): ...

ANNOTATION_LOG_KIND_UNSPECIFIED: AnnotationLogKind.ValueType  # 0
ANNOTATION_LOG_KIND_COMMENT: AnnotationLogKind.ValueType  # 1
ANNOTATION_LOG_KIND_STATE_UPDATE: AnnotationLogKind.ValueType  # 2
ANNOTATION_LOG_KIND_ASSIGNED: AnnotationLogKind.ValueType  # 3
global___AnnotationLogKind = AnnotationLogKind

class _AnnotationLogState:
    ValueType = typing.NewType("ValueType", builtins.int)
    V: typing_extensions.TypeAlias = ValueType

class _AnnotationLogStateEnumTypeWrapper(google.protobuf.internal.enum_type_wrapper._EnumTypeWrapper[_AnnotationLogState.ValueType], builtins.type):
    DESCRIPTOR: google.protobuf.descriptor.EnumDescriptor
    ANNOTATION_LOG_STATE_UNSPECIFIED: _AnnotationLogState.ValueType  # 0
    ANNOTATION_LOG_STATE_CREATED: _AnnotationLogState.ValueType  # 1
    ANNOTATION_LOG_STATE_OPEN: _AnnotationLogState.ValueType  # 2
    ANNOTATION_LOG_STATE_FLAGGED: _AnnotationLogState.ValueType  # 3
    ANNOTATION_LOG_STATE_RESOLVED: _AnnotationLogState.ValueType  # 4

class AnnotationLogState(_AnnotationLogState, metaclass=_AnnotationLogStateEnumTypeWrapper): ...

ANNOTATION_LOG_STATE_UNSPECIFIED: AnnotationLogState.ValueType  # 0
ANNOTATION_LOG_STATE_CREATED: AnnotationLogState.ValueType  # 1
ANNOTATION_LOG_STATE_OPEN: AnnotationLogState.ValueType  # 2
ANNOTATION_LOG_STATE_FLAGGED: AnnotationLogState.ValueType  # 3
ANNOTATION_LOG_STATE_RESOLVED: AnnotationLogState.ValueType  # 4
global___AnnotationLogState = AnnotationLogState

class _AnnotationCommentBodyElementType:
    ValueType = typing.NewType("ValueType", builtins.int)
    V: typing_extensions.TypeAlias = ValueType

class _AnnotationCommentBodyElementTypeEnumTypeWrapper(google.protobuf.internal.enum_type_wrapper._EnumTypeWrapper[_AnnotationCommentBodyElementType.ValueType], builtins.type):
    DESCRIPTOR: google.protobuf.descriptor.EnumDescriptor
    ANNOTATION_COMMENT_BODY_ELEMENT_TYPE_UNSPECIFIED: _AnnotationCommentBodyElementType.ValueType  # 0
    ANNOTATION_COMMENT_BODY_ELEMENT_TYPE_TEXT: _AnnotationCommentBodyElementType.ValueType  # 1
    ANNOTATION_COMMENT_BODY_ELEMENT_TYPE_USER_MENTION: _AnnotationCommentBodyElementType.ValueType  # 2

class AnnotationCommentBodyElementType(_AnnotationCommentBodyElementType, metaclass=_AnnotationCommentBodyElementTypeEnumTypeWrapper): ...

ANNOTATION_COMMENT_BODY_ELEMENT_TYPE_UNSPECIFIED: AnnotationCommentBodyElementType.ValueType  # 0
ANNOTATION_COMMENT_BODY_ELEMENT_TYPE_TEXT: AnnotationCommentBodyElementType.ValueType  # 1
ANNOTATION_COMMENT_BODY_ELEMENT_TYPE_USER_MENTION: AnnotationCommentBodyElementType.ValueType  # 2
global___AnnotationCommentBodyElementType = AnnotationCommentBodyElementType

@typing.final
class CreateAnnotationLogRequest(google.protobuf.message.Message):
    """The request for a call to `AnnotationLogService_CreateAnnotationLog` to create an annotation log."""

    DESCRIPTOR: google.protobuf.descriptor.Descriptor

    ANNOTATION_ID_FIELD_NUMBER: builtins.int
    KIND_FIELD_NUMBER: builtins.int
    ASSIGNED_FIELD_NUMBER: builtins.int
    STATE_UPDATE_FIELD_NUMBER: builtins.int
    COMMENT_FIELD_NUMBER: builtins.int
    annotation_id: builtins.str
    """The ID of the parent annotation associated to associate with the new annotation log."""
    kind: global___AnnotationLogKind.ValueType
    """The kind of annotation log to create."""
    @property
    def assigned(self) -> global___AnnotationLogAssignedProperties: ...
    @property
    def state_update(self) -> global___AnnotationLogStateUpdateProperties: ...
    @property
    def comment(self) -> global___AnnotationLogCommentProperties: ...
    def __init__(
        self,
        *,
        annotation_id: builtins.str = ...,
        kind: global___AnnotationLogKind.ValueType = ...,
        assigned: global___AnnotationLogAssignedProperties | None = ...,
        state_update: global___AnnotationLogStateUpdateProperties | None = ...,
        comment: global___AnnotationLogCommentProperties | None = ...,
    ) -> None: ...
    def HasField(self, field_name: typing.Literal["assigned", b"assigned", "comment", b"comment", "properties", b"properties", "state_update", b"state_update"]) -> builtins.bool: ...
    def ClearField(self, field_name: typing.Literal["annotation_id", b"annotation_id", "assigned", b"assigned", "comment", b"comment", "kind", b"kind", "properties", b"properties", "state_update", b"state_update"]) -> None: ...
    def WhichOneof(self, oneof_group: typing.Literal["properties", b"properties"]) -> typing.Literal["assigned", "state_update", "comment"] | None: ...

global___CreateAnnotationLogRequest = CreateAnnotationLogRequest

@typing.final
class CreateAnnotationLogResponse(google.protobuf.message.Message):
    """The response of a call to `AnnotationLogService_CreateAnnotationLog`."""

    DESCRIPTOR: google.protobuf.descriptor.Descriptor

    ANNOTATION_LOG_FIELD_NUMBER: builtins.int
    @property
    def annotation_log(self) -> global___AnnotationLogSearchResult: ...
    def __init__(
        self,
        *,
        annotation_log: global___AnnotationLogSearchResult | None = ...,
    ) -> None: ...
    def HasField(self, field_name: typing.Literal["annotation_log", b"annotation_log"]) -> builtins.bool: ...
    def ClearField(self, field_name: typing.Literal["annotation_log", b"annotation_log"]) -> None: ...

global___CreateAnnotationLogResponse = CreateAnnotationLogResponse

@typing.final
class ListAnnotationLogsRequest(google.protobuf.message.Message):
    """The request for a call to `AnnotationLogService_ListAnnotationLogs` to retrieve annotation logs."""

    DESCRIPTOR: google.protobuf.descriptor.Descriptor

    ANNOTATION_ID_FIELD_NUMBER: builtins.int
    PAGE_SIZE_FIELD_NUMBER: builtins.int
    PAGE_TOKEN_FIELD_NUMBER: builtins.int
    FILTER_FIELD_NUMBER: builtins.int
    annotation_id: builtins.str
    """The ID of the parent annotation associated with the annotation logs."""
    page_size: builtins.int
    """The maximum number of annotation logs to return.
    The service may return fewer than this value.
    If unspecified, at most 50 annotation logs will be returned.
    The maximum value is 1000; values above 1000 will be coerced to 1000.
    """
    page_token: builtins.str
    """A page token, received from a previous `ListAnnotationLogs` call.
    Provide this to retrieve the subsequent page.
    When paginating, all other parameters provided to `ListAnnotationLogs` must match
    the call that provided the page token.
    """
    filter: builtins.str
    """A [Common Expression Language (CEL)](https://github.com/google/cel-spec) filter string.
    Available fields to filter by are `annotation_log_id`, `annotation_id`, `created_by_user_id`,
    `created_date`, `modified_date`, and `kind`.
    For further information about how to use CELs, please refer to [this guide](https://github.com/google/cel-spec/blob/master/doc/langdef.md#standard-definitions).
    For more information about the fields used for filtering, please refer to [this definition](/docs/api/grpc/protocol-buffers/annotation_logs#annotationlogsearchresult). Optional.
    """
    def __init__(
        self,
        *,
        annotation_id: builtins.str = ...,
        page_size: builtins.int = ...,
        page_token: builtins.str = ...,
        filter: builtins.str = ...,
    ) -> None: ...
    def ClearField(self, field_name: typing.Literal["annotation_id", b"annotation_id", "filter", b"filter", "page_size", b"page_size", "page_token", b"page_token"]) -> None: ...

global___ListAnnotationLogsRequest = ListAnnotationLogsRequest

@typing.final
class ListAnnotationLogsResponse(google.protobuf.message.Message):
    """The response of a call to `AnnotationLogService_ListAnnotationLogs`."""

    DESCRIPTOR: google.protobuf.descriptor.Descriptor

    ANNOTATION_LOGS_FIELD_NUMBER: builtins.int
    NEXT_PAGE_TOKEN_FIELD_NUMBER: builtins.int
    next_page_token: builtins.str
    @property
    def annotation_logs(self) -> google.protobuf.internal.containers.RepeatedCompositeFieldContainer[global___AnnotationLogSearchResult]: ...
    def __init__(
        self,
        *,
        annotation_logs: collections.abc.Iterable[global___AnnotationLogSearchResult] | None = ...,
        next_page_token: builtins.str = ...,
    ) -> None: ...
    def ClearField(self, field_name: typing.Literal["annotation_logs", b"annotation_logs", "next_page_token", b"next_page_token"]) -> None: ...

global___ListAnnotationLogsResponse = ListAnnotationLogsResponse

@typing.final
class DeleteAnnotationLogRequest(google.protobuf.message.Message):
    """The request for a call to `AnnotationLogService_DeleteAnnotationLog` to delete an annotation log."""

    DESCRIPTOR: google.protobuf.descriptor.Descriptor

    ANNOTATION_ID_FIELD_NUMBER: builtins.int
    ANNOTATION_LOG_ID_FIELD_NUMBER: builtins.int
    annotation_id: builtins.str
    """The ID of the parent annotation associated with the annotation log to be deleted."""
    annotation_log_id: builtins.str
    """ID of the annotation log to be deleted."""
    def __init__(
        self,
        *,
        annotation_id: builtins.str = ...,
        annotation_log_id: builtins.str = ...,
    ) -> None: ...
    def ClearField(self, field_name: typing.Literal["annotation_id", b"annotation_id", "annotation_log_id", b"annotation_log_id"]) -> None: ...

global___DeleteAnnotationLogRequest = DeleteAnnotationLogRequest

@typing.final
class DeleteAnnotationLogResponse(google.protobuf.message.Message):
    """The Response of a call to `AnnotationLogService_DeleteAnnotationLog`."""

    DESCRIPTOR: google.protobuf.descriptor.Descriptor

    def __init__(
        self,
    ) -> None: ...

global___DeleteAnnotationLogResponse = DeleteAnnotationLogResponse

@typing.final
class AnnotationLogSearchResult(google.protobuf.message.Message):
    DESCRIPTOR: google.protobuf.descriptor.Descriptor

    ANNOTATION_LOG_ID_FIELD_NUMBER: builtins.int
    CREATED_DATE_FIELD_NUMBER: builtins.int
    MODIFIED_DATE_FIELD_NUMBER: builtins.int
    ANNOTATION_ID_FIELD_NUMBER: builtins.int
    KIND_FIELD_NUMBER: builtins.int
    CREATED_BY_USER_ID_FIELD_NUMBER: builtins.int
    CREATED_BY_USER_NAME_FIELD_NUMBER: builtins.int
    ASSIGNED_FIELD_NUMBER: builtins.int
    STATE_UPDATE_FIELD_NUMBER: builtins.int
    COMMENT_FIELD_NUMBER: builtins.int
    annotation_log_id: builtins.str
    annotation_id: builtins.str
    kind: global___AnnotationLogKind.ValueType
    created_by_user_id: builtins.str
    created_by_user_name: builtins.str
    @property
    def created_date(self) -> google.protobuf.timestamp_pb2.Timestamp: ...
    @property
    def modified_date(self) -> google.protobuf.timestamp_pb2.Timestamp: ...
    @property
    def assigned(self) -> global___AnnotationLogAssignedProperties: ...
    @property
    def state_update(self) -> global___AnnotationLogStateUpdateProperties: ...
    @property
    def comment(self) -> global___AnnotationLogCommentProperties: ...
    def __init__(
        self,
        *,
        annotation_log_id: builtins.str = ...,
        created_date: google.protobuf.timestamp_pb2.Timestamp | None = ...,
        modified_date: google.protobuf.timestamp_pb2.Timestamp | None = ...,
        annotation_id: builtins.str = ...,
        kind: global___AnnotationLogKind.ValueType = ...,
        created_by_user_id: builtins.str = ...,
        created_by_user_name: builtins.str = ...,
        assigned: global___AnnotationLogAssignedProperties | None = ...,
        state_update: global___AnnotationLogStateUpdateProperties | None = ...,
        comment: global___AnnotationLogCommentProperties | None = ...,
    ) -> None: ...
    def HasField(self, field_name: typing.Literal["assigned", b"assigned", "comment", b"comment", "created_date", b"created_date", "modified_date", b"modified_date", "properties", b"properties", "state_update", b"state_update"]) -> builtins.bool: ...
    def ClearField(self, field_name: typing.Literal["annotation_id", b"annotation_id", "annotation_log_id", b"annotation_log_id", "assigned", b"assigned", "comment", b"comment", "created_by_user_id", b"created_by_user_id", "created_by_user_name", b"created_by_user_name", "created_date", b"created_date", "kind", b"kind", "modified_date", b"modified_date", "properties", b"properties", "state_update", b"state_update"]) -> None: ...
    def WhichOneof(self, oneof_group: typing.Literal["properties", b"properties"]) -> typing.Literal["assigned", "state_update", "comment"] | None: ...

global___AnnotationLogSearchResult = AnnotationLogSearchResult

@typing.final
class AnnotationLogAssignedProperties(google.protobuf.message.Message):
    DESCRIPTOR: google.protobuf.descriptor.Descriptor

    ASSIGNED_TO_USER_ID_FIELD_NUMBER: builtins.int
    ASSIGNED_TO_USER_EMAIL_FIELD_NUMBER: builtins.int
    assigned_to_user_id: builtins.str
    assigned_to_user_email: builtins.str
    def __init__(
        self,
        *,
        assigned_to_user_id: builtins.str = ...,
        assigned_to_user_email: builtins.str = ...,
    ) -> None: ...
    def ClearField(self, field_name: typing.Literal["assigned_to_user_email", b"assigned_to_user_email", "assigned_to_user_id", b"assigned_to_user_id"]) -> None: ...

global___AnnotationLogAssignedProperties = AnnotationLogAssignedProperties

@typing.final
class AnnotationLogStateUpdateProperties(google.protobuf.message.Message):
    DESCRIPTOR: google.protobuf.descriptor.Descriptor

    STATE_FIELD_NUMBER: builtins.int
    state: global___AnnotationLogState.ValueType
    def __init__(
        self,
        *,
        state: global___AnnotationLogState.ValueType = ...,
    ) -> None: ...
    def ClearField(self, field_name: typing.Literal["state", b"state"]) -> None: ...

global___AnnotationLogStateUpdateProperties = AnnotationLogStateUpdateProperties

@typing.final
class AnnotationLogCommentProperties(google.protobuf.message.Message):
    DESCRIPTOR: google.protobuf.descriptor.Descriptor

    BODY_FIELD_NUMBER: builtins.int
    @property
    def body(self) -> google.protobuf.internal.containers.RepeatedCompositeFieldContainer[global___AnnotationCommentBodyElement]: ...
    def __init__(
        self,
        *,
        body: collections.abc.Iterable[global___AnnotationCommentBodyElement] | None = ...,
    ) -> None: ...
    def ClearField(self, field_name: typing.Literal["body", b"body"]) -> None: ...

global___AnnotationLogCommentProperties = AnnotationLogCommentProperties

@typing.final
class AnnotationCommentBodyElement(google.protobuf.message.Message):
    DESCRIPTOR: google.protobuf.descriptor.Descriptor

    TYPE_FIELD_NUMBER: builtins.int
    TEXT_FIELD_NUMBER: builtins.int
    USER_MENTION_FIELD_NUMBER: builtins.int
    type: global___AnnotationCommentBodyElementType.ValueType
    text: builtins.str
    @property
    def user_mention(self) -> global___AnnotationCommentUserMention: ...
    def __init__(
        self,
        *,
        type: global___AnnotationCommentBodyElementType.ValueType = ...,
        text: builtins.str = ...,
        user_mention: global___AnnotationCommentUserMention | None = ...,
    ) -> None: ...
    def HasField(self, field_name: typing.Literal["user_mention", b"user_mention"]) -> builtins.bool: ...
    def ClearField(self, field_name: typing.Literal["text", b"text", "type", b"type", "user_mention", b"user_mention"]) -> None: ...

global___AnnotationCommentBodyElement = AnnotationCommentBodyElement

@typing.final
class AnnotationCommentUserMention(google.protobuf.message.Message):
    DESCRIPTOR: google.protobuf.descriptor.Descriptor

    USER_ID_FIELD_NUMBER: builtins.int
    USER_EMAIL_FIELD_NUMBER: builtins.int
    user_id: builtins.str
    user_email: builtins.str
    def __init__(
        self,
        *,
        user_id: builtins.str = ...,
        user_email: builtins.str = ...,
    ) -> None: ...
    def ClearField(self, field_name: typing.Literal["user_email", b"user_email", "user_id", b"user_id"]) -> None: ...

global___AnnotationCommentUserMention = AnnotationCommentUserMention
