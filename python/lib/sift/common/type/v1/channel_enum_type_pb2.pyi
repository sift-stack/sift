"""
@generated by mypy-protobuf.  Do not edit manually!
isort:skip_file
"""

import builtins
import google.protobuf.descriptor
import google.protobuf.message
import typing

DESCRIPTOR: google.protobuf.descriptor.FileDescriptor

@typing.final
class ChannelEnumType(google.protobuf.message.Message):
    DESCRIPTOR: google.protobuf.descriptor.Descriptor

    NAME_FIELD_NUMBER: builtins.int
    KEY_FIELD_NUMBER: builtins.int
    name: builtins.str
    key: builtins.int
    def __init__(
        self,
        *,
        name: builtins.str = ...,
        key: builtins.int = ...,
    ) -> None: ...
    def ClearField(self, field_name: typing.Literal["key", b"key", "name", b"name"]) -> None: ...

global___ChannelEnumType = ChannelEnumType
