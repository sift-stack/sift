from google.api import annotations_pb2 as _annotations_pb2
from google.api import field_behavior_pb2 as _field_behavior_pb2
from google.protobuf import field_mask_pb2 as _field_mask_pb2
from google.protobuf import timestamp_pb2 as _timestamp_pb2
from protoc_gen_openapiv2.options import annotations_pb2 as _annotations_pb2_1
from google.protobuf.internal import containers as _containers
from google.protobuf import descriptor as _descriptor
from google.protobuf import message as _message
from typing import ClassVar as _ClassVar, Iterable as _Iterable, Mapping as _Mapping, Optional as _Optional, Union as _Union

DESCRIPTOR: _descriptor.FileDescriptor

class Run(_message.Message):
    __slots__ = ("run_id", "created_date", "modified_date", "created_by_user_id", "modified_by_user_id", "organization_id", "start_time", "stop_time", "is_pinned", "name", "description", "tags")
    RUN_ID_FIELD_NUMBER: _ClassVar[int]
    CREATED_DATE_FIELD_NUMBER: _ClassVar[int]
    MODIFIED_DATE_FIELD_NUMBER: _ClassVar[int]
    CREATED_BY_USER_ID_FIELD_NUMBER: _ClassVar[int]
    MODIFIED_BY_USER_ID_FIELD_NUMBER: _ClassVar[int]
    ORGANIZATION_ID_FIELD_NUMBER: _ClassVar[int]
    START_TIME_FIELD_NUMBER: _ClassVar[int]
    STOP_TIME_FIELD_NUMBER: _ClassVar[int]
    IS_PINNED_FIELD_NUMBER: _ClassVar[int]
    NAME_FIELD_NUMBER: _ClassVar[int]
    DESCRIPTION_FIELD_NUMBER: _ClassVar[int]
    TAGS_FIELD_NUMBER: _ClassVar[int]
    run_id: str
    created_date: _timestamp_pb2.Timestamp
    modified_date: _timestamp_pb2.Timestamp
    created_by_user_id: str
    modified_by_user_id: str
    organization_id: str
    start_time: _timestamp_pb2.Timestamp
    stop_time: _timestamp_pb2.Timestamp
    is_pinned: bool
    name: str
    description: str
    tags: _containers.RepeatedScalarFieldContainer[str]
    def __init__(self, run_id: _Optional[str] = ..., created_date: _Optional[_Union[_timestamp_pb2.Timestamp, _Mapping]] = ..., modified_date: _Optional[_Union[_timestamp_pb2.Timestamp, _Mapping]] = ..., created_by_user_id: _Optional[str] = ..., modified_by_user_id: _Optional[str] = ..., organization_id: _Optional[str] = ..., start_time: _Optional[_Union[_timestamp_pb2.Timestamp, _Mapping]] = ..., stop_time: _Optional[_Union[_timestamp_pb2.Timestamp, _Mapping]] = ..., is_pinned: bool = ..., name: _Optional[str] = ..., description: _Optional[str] = ..., tags: _Optional[_Iterable[str]] = ...) -> None: ...

class GetRunRequest(_message.Message):
    __slots__ = ("run_id",)
    RUN_ID_FIELD_NUMBER: _ClassVar[int]
    run_id: str
    def __init__(self, run_id: _Optional[str] = ...) -> None: ...

class GetRunResponse(_message.Message):
    __slots__ = ("run",)
    RUN_FIELD_NUMBER: _ClassVar[int]
    run: Run
    def __init__(self, run: _Optional[_Union[Run, _Mapping]] = ...) -> None: ...

class ListRunsRequest(_message.Message):
    __slots__ = ("page_size", "page_token", "filter")
    PAGE_SIZE_FIELD_NUMBER: _ClassVar[int]
    PAGE_TOKEN_FIELD_NUMBER: _ClassVar[int]
    FILTER_FIELD_NUMBER: _ClassVar[int]
    page_size: int
    page_token: str
    filter: str
    def __init__(self, page_size: _Optional[int] = ..., page_token: _Optional[str] = ..., filter: _Optional[str] = ...) -> None: ...

class ListRunsResponse(_message.Message):
    __slots__ = ("runs", "next_page_token")
    RUNS_FIELD_NUMBER: _ClassVar[int]
    NEXT_PAGE_TOKEN_FIELD_NUMBER: _ClassVar[int]
    runs: _containers.RepeatedCompositeFieldContainer[Run]
    next_page_token: str
    def __init__(self, runs: _Optional[_Iterable[_Union[Run, _Mapping]]] = ..., next_page_token: _Optional[str] = ...) -> None: ...

class CreateRunRequest(_message.Message):
    __slots__ = ("name", "description", "tags", "start_time", "stop_time", "organization_id")
    NAME_FIELD_NUMBER: _ClassVar[int]
    DESCRIPTION_FIELD_NUMBER: _ClassVar[int]
    TAGS_FIELD_NUMBER: _ClassVar[int]
    START_TIME_FIELD_NUMBER: _ClassVar[int]
    STOP_TIME_FIELD_NUMBER: _ClassVar[int]
    ORGANIZATION_ID_FIELD_NUMBER: _ClassVar[int]
    name: str
    description: str
    tags: _containers.RepeatedScalarFieldContainer[str]
    start_time: _timestamp_pb2.Timestamp
    stop_time: _timestamp_pb2.Timestamp
    organization_id: str
    def __init__(self, name: _Optional[str] = ..., description: _Optional[str] = ..., tags: _Optional[_Iterable[str]] = ..., start_time: _Optional[_Union[_timestamp_pb2.Timestamp, _Mapping]] = ..., stop_time: _Optional[_Union[_timestamp_pb2.Timestamp, _Mapping]] = ..., organization_id: _Optional[str] = ...) -> None: ...

class CreateRunResponse(_message.Message):
    __slots__ = ("run",)
    RUN_FIELD_NUMBER: _ClassVar[int]
    run: Run
    def __init__(self, run: _Optional[_Union[Run, _Mapping]] = ...) -> None: ...

class UpdateRunRequest(_message.Message):
    __slots__ = ("run", "update_mask")
    RUN_FIELD_NUMBER: _ClassVar[int]
    UPDATE_MASK_FIELD_NUMBER: _ClassVar[int]
    run: Run
    update_mask: _field_mask_pb2.FieldMask
    def __init__(self, run: _Optional[_Union[Run, _Mapping]] = ..., update_mask: _Optional[_Union[_field_mask_pb2.FieldMask, _Mapping]] = ...) -> None: ...

class UpdateRunResponse(_message.Message):
    __slots__ = ("run",)
    RUN_FIELD_NUMBER: _ClassVar[int]
    run: Run
    def __init__(self, run: _Optional[_Union[Run, _Mapping]] = ...) -> None: ...

class CreateAutomaticRunAssociationForAssetsRequest(_message.Message):
    __slots__ = ("run_id", "asset_names")
    RUN_ID_FIELD_NUMBER: _ClassVar[int]
    ASSET_NAMES_FIELD_NUMBER: _ClassVar[int]
    run_id: str
    asset_names: _containers.RepeatedScalarFieldContainer[str]
    def __init__(self, run_id: _Optional[str] = ..., asset_names: _Optional[_Iterable[str]] = ...) -> None: ...

class CreateAutomaticRunAssociationForAssetsResponse(_message.Message):
    __slots__ = ()
    def __init__(self) -> None: ...

class DeleteRunRequest(_message.Message):
    __slots__ = ("run_id",)
    RUN_ID_FIELD_NUMBER: _ClassVar[int]
    run_id: str
    def __init__(self, run_id: _Optional[str] = ...) -> None: ...

class DeleteRunResponse(_message.Message):
    __slots__ = ()
    def __init__(self) -> None: ...

class StopRunRequest(_message.Message):
    __slots__ = ("run_id",)
    RUN_ID_FIELD_NUMBER: _ClassVar[int]
    run_id: str
    def __init__(self, run_id: _Optional[str] = ...) -> None: ...

class StopRunResponse(_message.Message):
    __slots__ = ()
    def __init__(self) -> None: ...
