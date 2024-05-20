from google.api import annotations_pb2 as _annotations_pb2
from google.api import field_behavior_pb2 as _field_behavior_pb2
from google.protobuf import field_mask_pb2 as _field_mask_pb2
from google.protobuf import timestamp_pb2 as _timestamp_pb2
from protoc_gen_openapiv2.options import annotations_pb2 as _annotations_pb2_1
from google.protobuf.internal import containers as _containers
from google.protobuf.internal import enum_type_wrapper as _enum_type_wrapper
from google.protobuf import descriptor as _descriptor
from google.protobuf import message as _message
from typing import ClassVar as _ClassVar, Iterable as _Iterable, Mapping as _Mapping, Optional as _Optional, Union as _Union

DESCRIPTOR: _descriptor.FileDescriptor

class NotificationKind(int, metaclass=_enum_type_wrapper.EnumTypeWrapper):
    __slots__ = ()
    NOTIFICATION_KIND_UNSPECIFIED: _ClassVar[NotificationKind]
    NOTIFICATION_KIND_TEXT: _ClassVar[NotificationKind]
    NOTIFICATION_KIND_ANNOTATION_ASSIGNED: _ClassVar[NotificationKind]
    NOTIFICATION_KIND_MENTIONED_IN_ANNOTATION_COMMENT: _ClassVar[NotificationKind]
    NOTIFICATION_KIND_CONDITION_TRIGGERED: _ClassVar[NotificationKind]
    NOTIFICATION_KIND_ANNOTATION_STATE_CHANGED: _ClassVar[NotificationKind]
NOTIFICATION_KIND_UNSPECIFIED: NotificationKind
NOTIFICATION_KIND_TEXT: NotificationKind
NOTIFICATION_KIND_ANNOTATION_ASSIGNED: NotificationKind
NOTIFICATION_KIND_MENTIONED_IN_ANNOTATION_COMMENT: NotificationKind
NOTIFICATION_KIND_CONDITION_TRIGGERED: NotificationKind
NOTIFICATION_KIND_ANNOTATION_STATE_CHANGED: NotificationKind

class Notification(_message.Message):
    __slots__ = ("notification_id", "created_date", "modified_date", "created_by_user_id", "modified_by_user_id", "organization_id", "recipient_user_id", "is_read", "full_link", "notification_type", "contents", "entity_id")
    NOTIFICATION_ID_FIELD_NUMBER: _ClassVar[int]
    CREATED_DATE_FIELD_NUMBER: _ClassVar[int]
    MODIFIED_DATE_FIELD_NUMBER: _ClassVar[int]
    CREATED_BY_USER_ID_FIELD_NUMBER: _ClassVar[int]
    MODIFIED_BY_USER_ID_FIELD_NUMBER: _ClassVar[int]
    ORGANIZATION_ID_FIELD_NUMBER: _ClassVar[int]
    RECIPIENT_USER_ID_FIELD_NUMBER: _ClassVar[int]
    IS_READ_FIELD_NUMBER: _ClassVar[int]
    FULL_LINK_FIELD_NUMBER: _ClassVar[int]
    NOTIFICATION_TYPE_FIELD_NUMBER: _ClassVar[int]
    CONTENTS_FIELD_NUMBER: _ClassVar[int]
    ENTITY_ID_FIELD_NUMBER: _ClassVar[int]
    notification_id: str
    created_date: _timestamp_pb2.Timestamp
    modified_date: _timestamp_pb2.Timestamp
    created_by_user_id: str
    modified_by_user_id: str
    organization_id: str
    recipient_user_id: str
    is_read: bool
    full_link: str
    notification_type: NotificationKind
    contents: str
    entity_id: str
    def __init__(self, notification_id: _Optional[str] = ..., created_date: _Optional[_Union[_timestamp_pb2.Timestamp, _Mapping]] = ..., modified_date: _Optional[_Union[_timestamp_pb2.Timestamp, _Mapping]] = ..., created_by_user_id: _Optional[str] = ..., modified_by_user_id: _Optional[str] = ..., organization_id: _Optional[str] = ..., recipient_user_id: _Optional[str] = ..., is_read: bool = ..., full_link: _Optional[str] = ..., notification_type: _Optional[_Union[NotificationKind, str]] = ..., contents: _Optional[str] = ..., entity_id: _Optional[str] = ...) -> None: ...

class ListNotificationsRequest(_message.Message):
    __slots__ = ("page_size", "page_token", "filter")
    PAGE_SIZE_FIELD_NUMBER: _ClassVar[int]
    PAGE_TOKEN_FIELD_NUMBER: _ClassVar[int]
    FILTER_FIELD_NUMBER: _ClassVar[int]
    page_size: int
    page_token: str
    filter: str
    def __init__(self, page_size: _Optional[int] = ..., page_token: _Optional[str] = ..., filter: _Optional[str] = ...) -> None: ...

class ListNotificationsResponse(_message.Message):
    __slots__ = ("notifications", "next_page_token")
    NOTIFICATIONS_FIELD_NUMBER: _ClassVar[int]
    NEXT_PAGE_TOKEN_FIELD_NUMBER: _ClassVar[int]
    notifications: _containers.RepeatedCompositeFieldContainer[Notification]
    next_page_token: str
    def __init__(self, notifications: _Optional[_Iterable[_Union[Notification, _Mapping]]] = ..., next_page_token: _Optional[str] = ...) -> None: ...

class BatchUpdateNotificationsRequest(_message.Message):
    __slots__ = ("requests",)
    REQUESTS_FIELD_NUMBER: _ClassVar[int]
    requests: _containers.RepeatedCompositeFieldContainer[UpdateNotificationRequest]
    def __init__(self, requests: _Optional[_Iterable[_Union[UpdateNotificationRequest, _Mapping]]] = ...) -> None: ...

class UpdateNotificationRequest(_message.Message):
    __slots__ = ("notification", "update_mask")
    NOTIFICATION_FIELD_NUMBER: _ClassVar[int]
    UPDATE_MASK_FIELD_NUMBER: _ClassVar[int]
    notification: Notification
    update_mask: _field_mask_pb2.FieldMask
    def __init__(self, notification: _Optional[_Union[Notification, _Mapping]] = ..., update_mask: _Optional[_Union[_field_mask_pb2.FieldMask, _Mapping]] = ...) -> None: ...

class BatchUpdateNotificationsResponse(_message.Message):
    __slots__ = ("notifications",)
    NOTIFICATIONS_FIELD_NUMBER: _ClassVar[int]
    notifications: _containers.RepeatedCompositeFieldContainer[Notification]
    def __init__(self, notifications: _Optional[_Iterable[_Union[Notification, _Mapping]]] = ...) -> None: ...
