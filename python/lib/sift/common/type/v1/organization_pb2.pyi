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
class Organization(google.protobuf.message.Message):
    DESCRIPTOR: google.protobuf.descriptor.Descriptor

    ORGANIZATION_ID_FIELD_NUMBER: builtins.int
    ORGANIZATION_NAME_FIELD_NUMBER: builtins.int
    organization_id: builtins.str
    organization_name: builtins.str
    def __init__(
        self,
        *,
        organization_id: builtins.str = ...,
        organization_name: builtins.str = ...,
    ) -> None: ...
    def ClearField(self, field_name: typing.Literal["organization_id", b"organization_id", "organization_name", b"organization_name"]) -> None: ...

global___Organization = Organization
