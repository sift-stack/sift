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

class AnnotationState(int, metaclass=_enum_type_wrapper.EnumTypeWrapper):
    __slots__ = ()
    ANNOTATION_STATE_UNSPECIFIED: _ClassVar[AnnotationState]
    ANNOTATION_STATE_OPEN: _ClassVar[AnnotationState]
    ANNOTATION_STATE_FLAGGED: _ClassVar[AnnotationState]
    ANNOTATION_STATE_RESOLVED: _ClassVar[AnnotationState]

class AnnotationType(int, metaclass=_enum_type_wrapper.EnumTypeWrapper):
    __slots__ = ()
    ANNOTATION_TYPE_UNSPECIFIED: _ClassVar[AnnotationType]
    ANNOTATION_TYPE_DATA_REVIEW: _ClassVar[AnnotationType]
    ANNOTATION_TYPE_PHASE: _ClassVar[AnnotationType]
ANNOTATION_STATE_UNSPECIFIED: AnnotationState
ANNOTATION_STATE_OPEN: AnnotationState
ANNOTATION_STATE_FLAGGED: AnnotationState
ANNOTATION_STATE_RESOLVED: AnnotationState
ANNOTATION_TYPE_UNSPECIFIED: AnnotationType
ANNOTATION_TYPE_DATA_REVIEW: AnnotationType
ANNOTATION_TYPE_PHASE: AnnotationType

class Annotation(_message.Message):
    __slots__ = ("annotation_id", "name", "description", "start_time", "end_time", "created_by_user_id", "modified_by_user_id", "created_date", "modified_date", "run_id", "state", "organization_id", "assigned_to_user_id", "annotation_type", "tags", "legend_config", "created_by_condition_id")
    ANNOTATION_ID_FIELD_NUMBER: _ClassVar[int]
    NAME_FIELD_NUMBER: _ClassVar[int]
    DESCRIPTION_FIELD_NUMBER: _ClassVar[int]
    START_TIME_FIELD_NUMBER: _ClassVar[int]
    END_TIME_FIELD_NUMBER: _ClassVar[int]
    CREATED_BY_USER_ID_FIELD_NUMBER: _ClassVar[int]
    MODIFIED_BY_USER_ID_FIELD_NUMBER: _ClassVar[int]
    CREATED_DATE_FIELD_NUMBER: _ClassVar[int]
    MODIFIED_DATE_FIELD_NUMBER: _ClassVar[int]
    RUN_ID_FIELD_NUMBER: _ClassVar[int]
    STATE_FIELD_NUMBER: _ClassVar[int]
    ORGANIZATION_ID_FIELD_NUMBER: _ClassVar[int]
    ASSIGNED_TO_USER_ID_FIELD_NUMBER: _ClassVar[int]
    ANNOTATION_TYPE_FIELD_NUMBER: _ClassVar[int]
    TAGS_FIELD_NUMBER: _ClassVar[int]
    LEGEND_CONFIG_FIELD_NUMBER: _ClassVar[int]
    CREATED_BY_CONDITION_ID_FIELD_NUMBER: _ClassVar[int]
    annotation_id: str
    name: str
    description: str
    start_time: _timestamp_pb2.Timestamp
    end_time: _timestamp_pb2.Timestamp
    created_by_user_id: str
    modified_by_user_id: str
    created_date: _timestamp_pb2.Timestamp
    modified_date: _timestamp_pb2.Timestamp
    run_id: str
    state: AnnotationState
    organization_id: str
    assigned_to_user_id: str
    annotation_type: AnnotationType
    tags: _containers.RepeatedScalarFieldContainer[str]
    legend_config: str
    created_by_condition_id: str
    def __init__(self, annotation_id: _Optional[str] = ..., name: _Optional[str] = ..., description: _Optional[str] = ..., start_time: _Optional[_Union[_timestamp_pb2.Timestamp, _Mapping]] = ..., end_time: _Optional[_Union[_timestamp_pb2.Timestamp, _Mapping]] = ..., created_by_user_id: _Optional[str] = ..., modified_by_user_id: _Optional[str] = ..., created_date: _Optional[_Union[_timestamp_pb2.Timestamp, _Mapping]] = ..., modified_date: _Optional[_Union[_timestamp_pb2.Timestamp, _Mapping]] = ..., run_id: _Optional[str] = ..., state: _Optional[_Union[AnnotationState, str]] = ..., organization_id: _Optional[str] = ..., assigned_to_user_id: _Optional[str] = ..., annotation_type: _Optional[_Union[AnnotationType, str]] = ..., tags: _Optional[_Iterable[str]] = ..., legend_config: _Optional[str] = ..., created_by_condition_id: _Optional[str] = ...) -> None: ...

class AnnotationLinkedChannelsChannel(_message.Message):
    __slots__ = ("channel_id",)
    CHANNEL_ID_FIELD_NUMBER: _ClassVar[int]
    channel_id: str
    def __init__(self, channel_id: _Optional[str] = ...) -> None: ...

class AnnotationLinkedChannelsBitFieldElement(_message.Message):
    __slots__ = ("channel_id", "bit_field_name")
    CHANNEL_ID_FIELD_NUMBER: _ClassVar[int]
    BIT_FIELD_NAME_FIELD_NUMBER: _ClassVar[int]
    channel_id: str
    bit_field_name: str
    def __init__(self, channel_id: _Optional[str] = ..., bit_field_name: _Optional[str] = ...) -> None: ...

class AnnotationLinkedChannel(_message.Message):
    __slots__ = ("channel", "bit_field_element")
    CHANNEL_FIELD_NUMBER: _ClassVar[int]
    BIT_FIELD_ELEMENT_FIELD_NUMBER: _ClassVar[int]
    channel: AnnotationLinkedChannelsChannel
    bit_field_element: AnnotationLinkedChannelsBitFieldElement
    def __init__(self, channel: _Optional[_Union[AnnotationLinkedChannelsChannel, _Mapping]] = ..., bit_field_element: _Optional[_Union[AnnotationLinkedChannelsBitFieldElement, _Mapping]] = ...) -> None: ...

class CreateAnnotationRequest(_message.Message):
    __slots__ = ("name", "description", "start_time", "end_time", "assets", "linked_channels", "tags", "run_id", "assign_to_user_id", "organization_id", "state", "annotation_type", "created_by_condition_id", "legend_config")
    NAME_FIELD_NUMBER: _ClassVar[int]
    DESCRIPTION_FIELD_NUMBER: _ClassVar[int]
    START_TIME_FIELD_NUMBER: _ClassVar[int]
    END_TIME_FIELD_NUMBER: _ClassVar[int]
    ASSETS_FIELD_NUMBER: _ClassVar[int]
    LINKED_CHANNELS_FIELD_NUMBER: _ClassVar[int]
    TAGS_FIELD_NUMBER: _ClassVar[int]
    RUN_ID_FIELD_NUMBER: _ClassVar[int]
    ASSIGN_TO_USER_ID_FIELD_NUMBER: _ClassVar[int]
    ORGANIZATION_ID_FIELD_NUMBER: _ClassVar[int]
    STATE_FIELD_NUMBER: _ClassVar[int]
    ANNOTATION_TYPE_FIELD_NUMBER: _ClassVar[int]
    CREATED_BY_CONDITION_ID_FIELD_NUMBER: _ClassVar[int]
    LEGEND_CONFIG_FIELD_NUMBER: _ClassVar[int]
    name: str
    description: str
    start_time: _timestamp_pb2.Timestamp
    end_time: _timestamp_pb2.Timestamp
    assets: _containers.RepeatedScalarFieldContainer[str]
    linked_channels: _containers.RepeatedCompositeFieldContainer[AnnotationLinkedChannel]
    tags: _containers.RepeatedScalarFieldContainer[str]
    run_id: str
    assign_to_user_id: str
    organization_id: str
    state: AnnotationState
    annotation_type: AnnotationType
    created_by_condition_id: str
    legend_config: str
    def __init__(self, name: _Optional[str] = ..., description: _Optional[str] = ..., start_time: _Optional[_Union[_timestamp_pb2.Timestamp, _Mapping]] = ..., end_time: _Optional[_Union[_timestamp_pb2.Timestamp, _Mapping]] = ..., assets: _Optional[_Iterable[str]] = ..., linked_channels: _Optional[_Iterable[_Union[AnnotationLinkedChannel, _Mapping]]] = ..., tags: _Optional[_Iterable[str]] = ..., run_id: _Optional[str] = ..., assign_to_user_id: _Optional[str] = ..., organization_id: _Optional[str] = ..., state: _Optional[_Union[AnnotationState, str]] = ..., annotation_type: _Optional[_Union[AnnotationType, str]] = ..., created_by_condition_id: _Optional[str] = ..., legend_config: _Optional[str] = ...) -> None: ...

class CreateAnnotationResponse(_message.Message):
    __slots__ = ("annotation",)
    ANNOTATION_FIELD_NUMBER: _ClassVar[int]
    annotation: Annotation
    def __init__(self, annotation: _Optional[_Union[Annotation, _Mapping]] = ...) -> None: ...

class DeleteAnnotationRequest(_message.Message):
    __slots__ = ("annotation_id",)
    ANNOTATION_ID_FIELD_NUMBER: _ClassVar[int]
    annotation_id: str
    def __init__(self, annotation_id: _Optional[str] = ...) -> None: ...

class DeleteAnnotationResponse(_message.Message):
    __slots__ = ()
    def __init__(self) -> None: ...

class BatchDeleteAnnotationsRequest(_message.Message):
    __slots__ = ("annotation_ids",)
    ANNOTATION_IDS_FIELD_NUMBER: _ClassVar[int]
    annotation_ids: _containers.RepeatedScalarFieldContainer[str]
    def __init__(self, annotation_ids: _Optional[_Iterable[str]] = ...) -> None: ...

class BatchDeleteAnnotationsResponse(_message.Message):
    __slots__ = ()
    def __init__(self) -> None: ...

class GetAnnotationRequest(_message.Message):
    __slots__ = ("annotation_id",)
    ANNOTATION_ID_FIELD_NUMBER: _ClassVar[int]
    annotation_id: str
    def __init__(self, annotation_id: _Optional[str] = ...) -> None: ...

class GetAnnotationResponse(_message.Message):
    __slots__ = ("annotation",)
    ANNOTATION_FIELD_NUMBER: _ClassVar[int]
    annotation: Annotation
    def __init__(self, annotation: _Optional[_Union[Annotation, _Mapping]] = ...) -> None: ...

class ListAnnotationsRequest(_message.Message):
    __slots__ = ("page_size", "page_token", "filter")
    PAGE_SIZE_FIELD_NUMBER: _ClassVar[int]
    PAGE_TOKEN_FIELD_NUMBER: _ClassVar[int]
    FILTER_FIELD_NUMBER: _ClassVar[int]
    page_size: int
    page_token: str
    filter: str
    def __init__(self, page_size: _Optional[int] = ..., page_token: _Optional[str] = ..., filter: _Optional[str] = ...) -> None: ...

class ListAnnotationsResponse(_message.Message):
    __slots__ = ("annotations", "next_page_token")
    ANNOTATIONS_FIELD_NUMBER: _ClassVar[int]
    NEXT_PAGE_TOKEN_FIELD_NUMBER: _ClassVar[int]
    annotations: _containers.RepeatedCompositeFieldContainer[Annotation]
    next_page_token: str
    def __init__(self, annotations: _Optional[_Iterable[_Union[Annotation, _Mapping]]] = ..., next_page_token: _Optional[str] = ...) -> None: ...

class UpdateAnnotationRequest(_message.Message):
    __slots__ = ("annotation", "update_mask")
    ANNOTATION_FIELD_NUMBER: _ClassVar[int]
    UPDATE_MASK_FIELD_NUMBER: _ClassVar[int]
    annotation: Annotation
    update_mask: _field_mask_pb2.FieldMask
    def __init__(self, annotation: _Optional[_Union[Annotation, _Mapping]] = ..., update_mask: _Optional[_Union[_field_mask_pb2.FieldMask, _Mapping]] = ...) -> None: ...

class UpdateAnnotationResponse(_message.Message):
    __slots__ = ("annotation",)
    ANNOTATION_FIELD_NUMBER: _ClassVar[int]
    annotation: Annotation
    def __init__(self, annotation: _Optional[_Union[Annotation, _Mapping]] = ...) -> None: ...
