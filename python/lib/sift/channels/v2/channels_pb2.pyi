"""
@generated by mypy-protobuf.  Do not edit manually!
isort:skip_file
"""

import builtins
import collections.abc
import google.protobuf.descriptor
import google.protobuf.internal.containers
import google.protobuf.message
import google.protobuf.timestamp_pb2
import sift.common.type.v1.channel_bit_field_element_pb2
import sift.common.type.v1.channel_data_type_pb2
import sift.common.type.v1.channel_enum_type_pb2
import typing

DESCRIPTOR: google.protobuf.descriptor.FileDescriptor

@typing.final
class Channel(google.protobuf.message.Message):
    DESCRIPTOR: google.protobuf.descriptor.Descriptor

    CHANNEL_ID_FIELD_NUMBER: builtins.int
    NAME_FIELD_NUMBER: builtins.int
    COMPONENT_FIELD_NUMBER: builtins.int
    ASSET_ID_FIELD_NUMBER: builtins.int
    DESCRIPTION_FIELD_NUMBER: builtins.int
    UNIT_ID_FIELD_NUMBER: builtins.int
    CREATED_DATE_FIELD_NUMBER: builtins.int
    MODIFIED_DATE_FIELD_NUMBER: builtins.int
    CREATED_BY_USER_ID_FIELD_NUMBER: builtins.int
    MODIFIED_BY_USER_ID_FIELD_NUMBER: builtins.int
    ORGANIZATION_ID_FIELD_NUMBER: builtins.int
    DATA_TYPE_FIELD_NUMBER: builtins.int
    ENUM_TYPES_FIELD_NUMBER: builtins.int
    BIT_FIELD_ELEMENTS_FIELD_NUMBER: builtins.int
    channel_id: builtins.str
    name: builtins.str
    component: builtins.str
    asset_id: builtins.str
    description: builtins.str
    unit_id: builtins.str
    created_by_user_id: builtins.str
    modified_by_user_id: builtins.str
    organization_id: builtins.str
    data_type: sift.common.type.v1.channel_data_type_pb2.ChannelDataType.ValueType
    @property
    def created_date(self) -> google.protobuf.timestamp_pb2.Timestamp: ...
    @property
    def modified_date(self) -> google.protobuf.timestamp_pb2.Timestamp: ...
    @property
    def enum_types(self) -> google.protobuf.internal.containers.RepeatedCompositeFieldContainer[sift.common.type.v1.channel_enum_type_pb2.ChannelEnumType]: ...
    @property
    def bit_field_elements(self) -> google.protobuf.internal.containers.RepeatedCompositeFieldContainer[sift.common.type.v1.channel_bit_field_element_pb2.ChannelBitFieldElement]: ...
    def __init__(
        self,
        *,
        channel_id: builtins.str = ...,
        name: builtins.str = ...,
        component: builtins.str = ...,
        asset_id: builtins.str = ...,
        description: builtins.str = ...,
        unit_id: builtins.str = ...,
        created_date: google.protobuf.timestamp_pb2.Timestamp | None = ...,
        modified_date: google.protobuf.timestamp_pb2.Timestamp | None = ...,
        created_by_user_id: builtins.str = ...,
        modified_by_user_id: builtins.str = ...,
        organization_id: builtins.str = ...,
        data_type: sift.common.type.v1.channel_data_type_pb2.ChannelDataType.ValueType = ...,
        enum_types: collections.abc.Iterable[sift.common.type.v1.channel_enum_type_pb2.ChannelEnumType] | None = ...,
        bit_field_elements: collections.abc.Iterable[sift.common.type.v1.channel_bit_field_element_pb2.ChannelBitFieldElement] | None = ...,
    ) -> None: ...
    def HasField(self, field_name: typing.Literal["created_date", b"created_date", "modified_date", b"modified_date"]) -> builtins.bool: ...
    def ClearField(self, field_name: typing.Literal["asset_id", b"asset_id", "bit_field_elements", b"bit_field_elements", "channel_id", b"channel_id", "component", b"component", "created_by_user_id", b"created_by_user_id", "created_date", b"created_date", "data_type", b"data_type", "description", b"description", "enum_types", b"enum_types", "modified_by_user_id", b"modified_by_user_id", "modified_date", b"modified_date", "name", b"name", "organization_id", b"organization_id", "unit_id", b"unit_id"]) -> None: ...

global___Channel = Channel

@typing.final
class GetChannelRequest(google.protobuf.message.Message):
    """The request for a call to `ChannelService_GetChannel`."""

    DESCRIPTOR: google.protobuf.descriptor.Descriptor

    CHANNEL_ID_FIELD_NUMBER: builtins.int
    channel_id: builtins.str
    def __init__(
        self,
        *,
        channel_id: builtins.str = ...,
    ) -> None: ...
    def ClearField(self, field_name: typing.Literal["channel_id", b"channel_id"]) -> None: ...

global___GetChannelRequest = GetChannelRequest

@typing.final
class GetChannelResponse(google.protobuf.message.Message):
    """The response of a call to `ChannelService_GetChannel`."""

    DESCRIPTOR: google.protobuf.descriptor.Descriptor

    CHANNEL_FIELD_NUMBER: builtins.int
    @property
    def channel(self) -> global___Channel: ...
    def __init__(
        self,
        *,
        channel: global___Channel | None = ...,
    ) -> None: ...
    def HasField(self, field_name: typing.Literal["channel", b"channel"]) -> builtins.bool: ...
    def ClearField(self, field_name: typing.Literal["channel", b"channel"]) -> None: ...

global___GetChannelResponse = GetChannelResponse

@typing.final
class ListChannelsRequest(google.protobuf.message.Message):
    """The request for a call to `ChannelService_ListChannels` to retrieve channels."""

    DESCRIPTOR: google.protobuf.descriptor.Descriptor

    PAGE_SIZE_FIELD_NUMBER: builtins.int
    PAGE_TOKEN_FIELD_NUMBER: builtins.int
    FILTER_FIELD_NUMBER: builtins.int
    ORGANIZATION_ID_FIELD_NUMBER: builtins.int
    page_size: builtins.int
    """The maximum number of channels to return. The service may return fewer than this value.
    If unspecified, at most 50 channels will be returned. The maximum value is 10,000; values above
    10,000 will be coerced to 10,000. Optional.
    """
    page_token: builtins.str
    """A page token, received from a previous `ListChannels` call.
    Provide this to retrieve the subsequent page.
    When paginating, all other parameters provided to `ListChannels` must match
    the call that provided the page token. Optional.
    """
    filter: builtins.str
    """A [Common Expression Language (CEL)](https://github.com/google/cel-spec) filter string.
    Available fields to filter by are `channel_id`, `asset_id`, `name`, `component`, `description`, `active`, `created_date`, and `modified_date`.
    For further information about how to use CELs, please refer to [this guide](https://github.com/google/cel-spec/blob/master/doc/langdef.md#standard-definitions).
    For more information about the fields used for filtering, please refer to [this definition](/protocol-buffers/documentation#channel). Optional.
    """
    organization_id: builtins.str
    """This field is only required if your user belongs to multiple organizations."""
    def __init__(
        self,
        *,
        page_size: builtins.int = ...,
        page_token: builtins.str = ...,
        filter: builtins.str = ...,
        organization_id: builtins.str = ...,
    ) -> None: ...
    def ClearField(self, field_name: typing.Literal["filter", b"filter", "organization_id", b"organization_id", "page_size", b"page_size", "page_token", b"page_token"]) -> None: ...

global___ListChannelsRequest = ListChannelsRequest

@typing.final
class ListChannelsResponse(google.protobuf.message.Message):
    """The result of a call to `ChannelService_ListChannels`."""

    DESCRIPTOR: google.protobuf.descriptor.Descriptor

    CHANNELS_FIELD_NUMBER: builtins.int
    NEXT_PAGE_TOKEN_FIELD_NUMBER: builtins.int
    next_page_token: builtins.str
    @property
    def channels(self) -> google.protobuf.internal.containers.RepeatedCompositeFieldContainer[global___Channel]: ...
    def __init__(
        self,
        *,
        channels: collections.abc.Iterable[global___Channel] | None = ...,
        next_page_token: builtins.str = ...,
    ) -> None: ...
    def ClearField(self, field_name: typing.Literal["channels", b"channels", "next_page_token", b"next_page_token"]) -> None: ...

global___ListChannelsResponse = ListChannelsResponse
