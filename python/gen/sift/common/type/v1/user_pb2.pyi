from google.api import field_behavior_pb2 as _field_behavior_pb2
from sift.common.type.v1 import organization_pb2 as _organization_pb2
from google.protobuf.internal import containers as _containers
from google.protobuf import descriptor as _descriptor
from google.protobuf import message as _message
from typing import ClassVar as _ClassVar, Iterable as _Iterable, Mapping as _Mapping, Optional as _Optional, Union as _Union

DESCRIPTOR: _descriptor.FileDescriptor

class User(_message.Message):
    __slots__ = ("user_id", "user_name", "organizations")
    USER_ID_FIELD_NUMBER: _ClassVar[int]
    USER_NAME_FIELD_NUMBER: _ClassVar[int]
    ORGANIZATIONS_FIELD_NUMBER: _ClassVar[int]
    user_id: str
    user_name: str
    organizations: _containers.RepeatedCompositeFieldContainer[_organization_pb2.Organization]
    def __init__(self, user_id: _Optional[str] = ..., user_name: _Optional[str] = ..., organizations: _Optional[_Iterable[_Union[_organization_pb2.Organization, _Mapping]]] = ...) -> None: ...
