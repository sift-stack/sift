from google.api import annotations_pb2 as _annotations_pb2
from google.api import field_behavior_pb2 as _field_behavior_pb2
from google.protobuf import timestamp_pb2 as _timestamp_pb2
from protoc_gen_openapiv2.options import annotations_pb2 as _annotations_pb2_1
from google.protobuf.internal import containers as _containers
from google.protobuf.internal import enum_type_wrapper as _enum_type_wrapper
from google.protobuf import descriptor as _descriptor
from google.protobuf import message as _message
from typing import ClassVar as _ClassVar, Iterable as _Iterable, Mapping as _Mapping, Optional as _Optional, Union as _Union

DESCRIPTOR: _descriptor.FileDescriptor

class AnnotationLogKind(int, metaclass=_enum_type_wrapper.EnumTypeWrapper):
    __slots__ = ()
    ANNOTATION_LOG_KIND_UNSPECIFIED: _ClassVar[AnnotationLogKind]
    ANNOTATION_LOG_KIND_COMMENT: _ClassVar[AnnotationLogKind]
    ANNOTATION_LOG_KIND_STATE_UPDATE: _ClassVar[AnnotationLogKind]
    ANNOTATION_LOG_KIND_ASSIGNED: _ClassVar[AnnotationLogKind]

class AnnotationLogState(int, metaclass=_enum_type_wrapper.EnumTypeWrapper):
    __slots__ = ()
    ANNOTATION_LOG_STATE_UNSPECIFIED: _ClassVar[AnnotationLogState]
    ANNOTATION_LOG_STATE_CREATED: _ClassVar[AnnotationLogState]
    ANNOTATION_LOG_STATE_OPEN: _ClassVar[AnnotationLogState]
    ANNOTATION_LOG_STATE_FLAGGED: _ClassVar[AnnotationLogState]
    ANNOTATION_LOG_STATE_RESOLVED: _ClassVar[AnnotationLogState]

class AnnotationCommentBodyElementType(int, metaclass=_enum_type_wrapper.EnumTypeWrapper):
    __slots__ = ()
    ANNOTATION_COMMENT_BODY_ELEMENT_TYPE_UNSPECIFIED: _ClassVar[AnnotationCommentBodyElementType]
    ANNOTATION_COMMENT_BODY_ELEMENT_TYPE_TEXT: _ClassVar[AnnotationCommentBodyElementType]
    ANNOTATION_COMMENT_BODY_ELEMENT_TYPE_USER_MENTION: _ClassVar[AnnotationCommentBodyElementType]
ANNOTATION_LOG_KIND_UNSPECIFIED: AnnotationLogKind
ANNOTATION_LOG_KIND_COMMENT: AnnotationLogKind
ANNOTATION_LOG_KIND_STATE_UPDATE: AnnotationLogKind
ANNOTATION_LOG_KIND_ASSIGNED: AnnotationLogKind
ANNOTATION_LOG_STATE_UNSPECIFIED: AnnotationLogState
ANNOTATION_LOG_STATE_CREATED: AnnotationLogState
ANNOTATION_LOG_STATE_OPEN: AnnotationLogState
ANNOTATION_LOG_STATE_FLAGGED: AnnotationLogState
ANNOTATION_LOG_STATE_RESOLVED: AnnotationLogState
ANNOTATION_COMMENT_BODY_ELEMENT_TYPE_UNSPECIFIED: AnnotationCommentBodyElementType
ANNOTATION_COMMENT_BODY_ELEMENT_TYPE_TEXT: AnnotationCommentBodyElementType
ANNOTATION_COMMENT_BODY_ELEMENT_TYPE_USER_MENTION: AnnotationCommentBodyElementType

class CreateAnnotationLogRequest(_message.Message):
    __slots__ = ("annotation_id", "kind", "assigned", "state_update", "comment")
    ANNOTATION_ID_FIELD_NUMBER: _ClassVar[int]
    KIND_FIELD_NUMBER: _ClassVar[int]
    ASSIGNED_FIELD_NUMBER: _ClassVar[int]
    STATE_UPDATE_FIELD_NUMBER: _ClassVar[int]
    COMMENT_FIELD_NUMBER: _ClassVar[int]
    annotation_id: str
    kind: AnnotationLogKind
    assigned: AnnotationLogAssignedProperties
    state_update: AnnotationLogStateUpdateProperties
    comment: AnnotationLogCommentProperties
    def __init__(self, annotation_id: _Optional[str] = ..., kind: _Optional[_Union[AnnotationLogKind, str]] = ..., assigned: _Optional[_Union[AnnotationLogAssignedProperties, _Mapping]] = ..., state_update: _Optional[_Union[AnnotationLogStateUpdateProperties, _Mapping]] = ..., comment: _Optional[_Union[AnnotationLogCommentProperties, _Mapping]] = ...) -> None: ...

class CreateAnnotationLogResponse(_message.Message):
    __slots__ = ("annotation_log",)
    ANNOTATION_LOG_FIELD_NUMBER: _ClassVar[int]
    annotation_log: AnnotationLogSearchResult
    def __init__(self, annotation_log: _Optional[_Union[AnnotationLogSearchResult, _Mapping]] = ...) -> None: ...

class ListAnnotationLogsRequest(_message.Message):
    __slots__ = ("annotation_id", "page_size", "page_token", "filter")
    ANNOTATION_ID_FIELD_NUMBER: _ClassVar[int]
    PAGE_SIZE_FIELD_NUMBER: _ClassVar[int]
    PAGE_TOKEN_FIELD_NUMBER: _ClassVar[int]
    FILTER_FIELD_NUMBER: _ClassVar[int]
    annotation_id: str
    page_size: int
    page_token: str
    filter: str
    def __init__(self, annotation_id: _Optional[str] = ..., page_size: _Optional[int] = ..., page_token: _Optional[str] = ..., filter: _Optional[str] = ...) -> None: ...

class ListAnnotationLogsResponse(_message.Message):
    __slots__ = ("annotation_logs", "next_page_token")
    ANNOTATION_LOGS_FIELD_NUMBER: _ClassVar[int]
    NEXT_PAGE_TOKEN_FIELD_NUMBER: _ClassVar[int]
    annotation_logs: _containers.RepeatedCompositeFieldContainer[AnnotationLogSearchResult]
    next_page_token: str
    def __init__(self, annotation_logs: _Optional[_Iterable[_Union[AnnotationLogSearchResult, _Mapping]]] = ..., next_page_token: _Optional[str] = ...) -> None: ...

class DeleteAnnotationLogRequest(_message.Message):
    __slots__ = ("annotation_id", "annotation_log_id")
    ANNOTATION_ID_FIELD_NUMBER: _ClassVar[int]
    ANNOTATION_LOG_ID_FIELD_NUMBER: _ClassVar[int]
    annotation_id: str
    annotation_log_id: str
    def __init__(self, annotation_id: _Optional[str] = ..., annotation_log_id: _Optional[str] = ...) -> None: ...

class DeleteAnnotationLogResponse(_message.Message):
    __slots__ = ()
    def __init__(self) -> None: ...

class AnnotationLogSearchResult(_message.Message):
    __slots__ = ("annotation_log_id", "created_date", "modified_date", "annotation_id", "kind", "created_by_user_id", "created_by_user_name", "assigned", "state_update", "comment")
    ANNOTATION_LOG_ID_FIELD_NUMBER: _ClassVar[int]
    CREATED_DATE_FIELD_NUMBER: _ClassVar[int]
    MODIFIED_DATE_FIELD_NUMBER: _ClassVar[int]
    ANNOTATION_ID_FIELD_NUMBER: _ClassVar[int]
    KIND_FIELD_NUMBER: _ClassVar[int]
    CREATED_BY_USER_ID_FIELD_NUMBER: _ClassVar[int]
    CREATED_BY_USER_NAME_FIELD_NUMBER: _ClassVar[int]
    ASSIGNED_FIELD_NUMBER: _ClassVar[int]
    STATE_UPDATE_FIELD_NUMBER: _ClassVar[int]
    COMMENT_FIELD_NUMBER: _ClassVar[int]
    annotation_log_id: str
    created_date: _timestamp_pb2.Timestamp
    modified_date: _timestamp_pb2.Timestamp
    annotation_id: str
    kind: AnnotationLogKind
    created_by_user_id: str
    created_by_user_name: str
    assigned: AnnotationLogAssignedProperties
    state_update: AnnotationLogStateUpdateProperties
    comment: AnnotationLogCommentProperties
    def __init__(self, annotation_log_id: _Optional[str] = ..., created_date: _Optional[_Union[_timestamp_pb2.Timestamp, _Mapping]] = ..., modified_date: _Optional[_Union[_timestamp_pb2.Timestamp, _Mapping]] = ..., annotation_id: _Optional[str] = ..., kind: _Optional[_Union[AnnotationLogKind, str]] = ..., created_by_user_id: _Optional[str] = ..., created_by_user_name: _Optional[str] = ..., assigned: _Optional[_Union[AnnotationLogAssignedProperties, _Mapping]] = ..., state_update: _Optional[_Union[AnnotationLogStateUpdateProperties, _Mapping]] = ..., comment: _Optional[_Union[AnnotationLogCommentProperties, _Mapping]] = ...) -> None: ...

class AnnotationLogAssignedProperties(_message.Message):
    __slots__ = ("assigned_to_user_id", "assigned_to_user_email")
    ASSIGNED_TO_USER_ID_FIELD_NUMBER: _ClassVar[int]
    ASSIGNED_TO_USER_EMAIL_FIELD_NUMBER: _ClassVar[int]
    assigned_to_user_id: str
    assigned_to_user_email: str
    def __init__(self, assigned_to_user_id: _Optional[str] = ..., assigned_to_user_email: _Optional[str] = ...) -> None: ...

class AnnotationLogStateUpdateProperties(_message.Message):
    __slots__ = ("state",)
    STATE_FIELD_NUMBER: _ClassVar[int]
    state: AnnotationLogState
    def __init__(self, state: _Optional[_Union[AnnotationLogState, str]] = ...) -> None: ...

class AnnotationLogCommentProperties(_message.Message):
    __slots__ = ("body",)
    BODY_FIELD_NUMBER: _ClassVar[int]
    body: _containers.RepeatedCompositeFieldContainer[AnnotationCommentBodyElement]
    def __init__(self, body: _Optional[_Iterable[_Union[AnnotationCommentBodyElement, _Mapping]]] = ...) -> None: ...

class AnnotationCommentBodyElement(_message.Message):
    __slots__ = ("type", "text", "user_mention")
    TYPE_FIELD_NUMBER: _ClassVar[int]
    TEXT_FIELD_NUMBER: _ClassVar[int]
    USER_MENTION_FIELD_NUMBER: _ClassVar[int]
    type: AnnotationCommentBodyElementType
    text: str
    user_mention: AnnotationCommentUserMention
    def __init__(self, type: _Optional[_Union[AnnotationCommentBodyElementType, str]] = ..., text: _Optional[str] = ..., user_mention: _Optional[_Union[AnnotationCommentUserMention, _Mapping]] = ...) -> None: ...

class AnnotationCommentUserMention(_message.Message):
    __slots__ = ("user_id", "user_email")
    USER_ID_FIELD_NUMBER: _ClassVar[int]
    USER_EMAIL_FIELD_NUMBER: _ClassVar[int]
    user_id: str
    user_email: str
    def __init__(self, user_id: _Optional[str] = ..., user_email: _Optional[str] = ...) -> None: ...
