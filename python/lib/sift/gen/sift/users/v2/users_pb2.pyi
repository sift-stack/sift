from google.api import annotations_pb2 as _annotations_pb2
from google.api import field_behavior_pb2 as _field_behavior_pb2
from protoc_gen_openapiv2.options import annotations_pb2 as _annotations_pb2_1
from sift.common.type.v1 import user_pb2 as _user_pb2
from google.protobuf import descriptor as _descriptor
from google.protobuf import message as _message
from typing import ClassVar as _ClassVar, Mapping as _Mapping, Optional as _Optional, Union as _Union

DESCRIPTOR: _descriptor.FileDescriptor

class UpdateUserOrganizationActiveRequest(_message.Message):
    __slots__ = ("user_id", "organization_id", "active")
    USER_ID_FIELD_NUMBER: _ClassVar[int]
    ORGANIZATION_ID_FIELD_NUMBER: _ClassVar[int]
    ACTIVE_FIELD_NUMBER: _ClassVar[int]
    user_id: str
    organization_id: str
    active: bool
    def __init__(self, user_id: _Optional[str] = ..., organization_id: _Optional[str] = ..., active: bool = ...) -> None: ...

class UpdateUserOrganizationActiveResponse(_message.Message):
    __slots__ = ()
    def __init__(self) -> None: ...

class GetUserRequest(_message.Message):
    __slots__ = ("user_id",)
    USER_ID_FIELD_NUMBER: _ClassVar[int]
    user_id: str
    def __init__(self, user_id: _Optional[str] = ...) -> None: ...

class GetUserResponse(_message.Message):
    __slots__ = ("user",)
    USER_FIELD_NUMBER: _ClassVar[int]
    user: _user_pb2.User
    def __init__(self, user: _Optional[_Union[_user_pb2.User, _Mapping]] = ...) -> None: ...
