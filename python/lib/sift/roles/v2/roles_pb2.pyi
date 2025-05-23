"""
@generated by mypy-protobuf.  Do not edit manually!
isort:skip_file
"""

import builtins
import collections.abc
import google.protobuf.descriptor
import google.protobuf.internal.containers
import google.protobuf.message
import typing

DESCRIPTOR: google.protobuf.descriptor.FileDescriptor

@typing.final
class Role(google.protobuf.message.Message):
    DESCRIPTOR: google.protobuf.descriptor.Descriptor

    ROLE_ID_FIELD_NUMBER: builtins.int
    NAME_FIELD_NUMBER: builtins.int
    DESCRIPTION_FIELD_NUMBER: builtins.int
    role_id: builtins.str
    name: builtins.str
    description: builtins.str
    def __init__(
        self,
        *,
        role_id: builtins.str = ...,
        name: builtins.str = ...,
        description: builtins.str = ...,
    ) -> None: ...
    def ClearField(self, field_name: typing.Literal["description", b"description", "name", b"name", "role_id", b"role_id"]) -> None: ...

global___Role = Role

@typing.final
class ListRolesRequest(google.protobuf.message.Message):
    DESCRIPTOR: google.protobuf.descriptor.Descriptor

    PAGE_SIZE_FIELD_NUMBER: builtins.int
    PAGE_TOKEN_FIELD_NUMBER: builtins.int
    FILTER_FIELD_NUMBER: builtins.int
    ORDER_BY_FIELD_NUMBER: builtins.int
    page_size: builtins.int
    """The maximum number of roles to return.
    The service may return fewer than this value.
    If unspecified, at most 50 roles will be returned.
    The maximum value is 1000; values above 1000 will be coerced to 1000.
    """
    page_token: builtins.str
    """A page token, received from a previous `ListRoles` call.
    Provide this to retrieve the subsequent page.

    When paginating, all other parameters provided to `ListRoles` must match
    the call that provided the page token.
    """
    filter: builtins.str
    """A [Common Expression Language (CEL)](https://github.com/google/cel-spec) filter string.
    Available fields to filter by are `role_id`, `name`, and `description`.
    For further information about how to use CELs, please refer to [this guide](https://github.com/google/cel-spec/blob/master/doc/langdef.md#standard-definitions).
    For more information about the fields used for filtering, please refer to [this definition](/docs/api/grpc/protocol-buffers/channels#channel). Optional.
    """
    order_by: builtins.str
    """How to order the retrieved channels. Formatted as a comma-separated string i.e. "FIELD_NAME[ desc],...".
    Available fields to order_by are `name` and `description`.
    If left empty, items are ordered by `name` in ascending order (oldest-first).
    For more information about the format of this field, read [this](https://google.aip.dev/132#ordering)
    Example: "name desc,description"
    """
    def __init__(
        self,
        *,
        page_size: builtins.int = ...,
        page_token: builtins.str = ...,
        filter: builtins.str = ...,
        order_by: builtins.str = ...,
    ) -> None: ...
    def ClearField(self, field_name: typing.Literal["filter", b"filter", "order_by", b"order_by", "page_size", b"page_size", "page_token", b"page_token"]) -> None: ...

global___ListRolesRequest = ListRolesRequest

@typing.final
class ListRolesResponse(google.protobuf.message.Message):
    DESCRIPTOR: google.protobuf.descriptor.Descriptor

    ROLES_FIELD_NUMBER: builtins.int
    NEXT_PAGE_TOKEN_FIELD_NUMBER: builtins.int
    next_page_token: builtins.str
    @property
    def roles(self) -> google.protobuf.internal.containers.RepeatedCompositeFieldContainer[global___Role]: ...
    def __init__(
        self,
        *,
        roles: collections.abc.Iterable[global___Role] | None = ...,
        next_page_token: builtins.str = ...,
    ) -> None: ...
    def ClearField(self, field_name: typing.Literal["next_page_token", b"next_page_token", "roles", b"roles"]) -> None: ...

global___ListRolesResponse = ListRolesResponse
