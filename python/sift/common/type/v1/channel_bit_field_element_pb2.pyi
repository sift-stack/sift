from google.api import field_behavior_pb2 as _field_behavior_pb2
from google.protobuf import descriptor as _descriptor
from google.protobuf import message as _message
from typing import ClassVar as _ClassVar, Optional as _Optional

DESCRIPTOR: _descriptor.FileDescriptor

class ChannelBitFieldElement(_message.Message):
    __slots__ = ("name", "index", "bit_count")
    NAME_FIELD_NUMBER: _ClassVar[int]
    INDEX_FIELD_NUMBER: _ClassVar[int]
    BIT_COUNT_FIELD_NUMBER: _ClassVar[int]
    name: str
    index: int
    bit_count: int
    def __init__(self, name: _Optional[str] = ..., index: _Optional[int] = ..., bit_count: _Optional[int] = ...) -> None: ...
