"""
@generated by mypy-protobuf.  Do not edit manually!
isort:skip_file
"""

import builtins
import collections.abc
import google.protobuf.descriptor
import google.protobuf.field_mask_pb2
import google.protobuf.internal.containers
import google.protobuf.message
import google.protobuf.timestamp_pb2
import typing

DESCRIPTOR: google.protobuf.descriptor.FileDescriptor

@typing.final
class View(google.protobuf.message.Message):
    DESCRIPTOR: google.protobuf.descriptor.Descriptor

    @typing.final
    class AxisGroups(google.protobuf.message.Message):
        DESCRIPTOR: google.protobuf.descriptor.Descriptor

        LEFT_FIELD_NUMBER: builtins.int
        RIGHT_FIELD_NUMBER: builtins.int
        @property
        def left(self) -> google.protobuf.internal.containers.RepeatedScalarFieldContainer[builtins.str]: ...
        @property
        def right(self) -> google.protobuf.internal.containers.RepeatedScalarFieldContainer[builtins.str]: ...
        def __init__(
            self,
            *,
            left: collections.abc.Iterable[builtins.str] | None = ...,
            right: collections.abc.Iterable[builtins.str] | None = ...,
        ) -> None: ...
        def ClearField(self, field_name: typing.Literal["left", b"left", "right", b"right"]) -> None: ...

    @typing.final
    class Channel(google.protobuf.message.Message):
        DESCRIPTOR: google.protobuf.descriptor.Descriptor

        @typing.final
        class CalculatedChannelConfig(google.protobuf.message.Message):
            DESCRIPTOR: google.protobuf.descriptor.Descriptor

            @typing.final
            class ChannelReference(google.protobuf.message.Message):
                DESCRIPTOR: google.protobuf.descriptor.Descriptor

                NAME_FIELD_NUMBER: builtins.int
                COMPONENT_FIELD_NUMBER: builtins.int
                name: builtins.str
                component: builtins.str
                def __init__(
                    self,
                    *,
                    name: builtins.str = ...,
                    component: builtins.str = ...,
                ) -> None: ...
                def ClearField(self, field_name: typing.Literal["component", b"component", "name", b"name"]) -> None: ...

            @typing.final
            class ChannelReferencesEntry(google.protobuf.message.Message):
                DESCRIPTOR: google.protobuf.descriptor.Descriptor

                KEY_FIELD_NUMBER: builtins.int
                VALUE_FIELD_NUMBER: builtins.int
                key: builtins.str
                @property
                def value(self) -> global___View.Channel.CalculatedChannelConfig.ChannelReference: ...
                def __init__(
                    self,
                    *,
                    key: builtins.str = ...,
                    value: global___View.Channel.CalculatedChannelConfig.ChannelReference | None = ...,
                ) -> None: ...
                def HasField(self, field_name: typing.Literal["value", b"value"]) -> builtins.bool: ...
                def ClearField(self, field_name: typing.Literal["key", b"key", "value", b"value"]) -> None: ...

            CHANNEL_KEY_FIELD_NUMBER: builtins.int
            CHANNEL_REFERENCES_FIELD_NUMBER: builtins.int
            EXPRESSION_FIELD_NUMBER: builtins.int
            UNIT_FIELD_NUMBER: builtins.int
            channel_key: builtins.str
            expression: builtins.str
            unit: builtins.str
            @property
            def channel_references(self) -> google.protobuf.internal.containers.MessageMap[builtins.str, global___View.Channel.CalculatedChannelConfig.ChannelReference]: ...
            def __init__(
                self,
                *,
                channel_key: builtins.str = ...,
                channel_references: collections.abc.Mapping[builtins.str, global___View.Channel.CalculatedChannelConfig.ChannelReference] | None = ...,
                expression: builtins.str = ...,
                unit: builtins.str = ...,
            ) -> None: ...
            def ClearField(self, field_name: typing.Literal["channel_key", b"channel_key", "channel_references", b"channel_references", "expression", b"expression", "unit", b"unit"]) -> None: ...

        NAME_FIELD_NUMBER: builtins.int
        COMPONENT_FIELD_NUMBER: builtins.int
        DATA_TYPE_FIELD_NUMBER: builtins.int
        AXIS_GROUP_FIELD_NUMBER: builtins.int
        BIT_FIELD_NAMES_FIELD_NUMBER: builtins.int
        CALCULATED_CHANNEL_CONFIG_FIELD_NUMBER: builtins.int
        name: builtins.str
        component: builtins.str
        data_type: builtins.str
        axis_group: builtins.str
        @property
        def bit_field_names(self) -> google.protobuf.internal.containers.RepeatedScalarFieldContainer[builtins.str]: ...
        @property
        def calculated_channel_config(self) -> global___View.Channel.CalculatedChannelConfig: ...
        def __init__(
            self,
            *,
            name: builtins.str = ...,
            component: builtins.str = ...,
            data_type: builtins.str = ...,
            axis_group: builtins.str = ...,
            bit_field_names: collections.abc.Iterable[builtins.str] | None = ...,
            calculated_channel_config: global___View.Channel.CalculatedChannelConfig | None = ...,
        ) -> None: ...
        def HasField(self, field_name: typing.Literal["_calculated_channel_config", b"_calculated_channel_config", "calculated_channel_config", b"calculated_channel_config"]) -> builtins.bool: ...
        def ClearField(self, field_name: typing.Literal["_calculated_channel_config", b"_calculated_channel_config", "axis_group", b"axis_group", "bit_field_names", b"bit_field_names", "calculated_channel_config", b"calculated_channel_config", "component", b"component", "data_type", b"data_type", "name", b"name"]) -> None: ...
        def WhichOneof(self, oneof_group: typing.Literal["_calculated_channel_config", b"_calculated_channel_config"]) -> typing.Literal["calculated_channel_config"] | None: ...

    VIEW_ID_FIELD_NUMBER: builtins.int
    NAME_FIELD_NUMBER: builtins.int
    AXIS_GROUPS_FIELD_NUMBER: builtins.int
    CHANNELS_FIELD_NUMBER: builtins.int
    CREATED_DATE_FIELD_NUMBER: builtins.int
    MODIFIED_DATE_FIELD_NUMBER: builtins.int
    CREATED_BY_USER_ID_FIELD_NUMBER: builtins.int
    ORGANIZATION_ID_FIELD_NUMBER: builtins.int
    MODIFIED_BY_USER_ID_FIELD_NUMBER: builtins.int
    IS_PINNED_FIELD_NUMBER: builtins.int
    view_id: builtins.str
    name: builtins.str
    created_by_user_id: builtins.str
    organization_id: builtins.str
    modified_by_user_id: builtins.str
    is_pinned: builtins.bool
    @property
    def axis_groups(self) -> global___View.AxisGroups: ...
    @property
    def channels(self) -> google.protobuf.internal.containers.RepeatedCompositeFieldContainer[global___View.Channel]: ...
    @property
    def created_date(self) -> google.protobuf.timestamp_pb2.Timestamp: ...
    @property
    def modified_date(self) -> google.protobuf.timestamp_pb2.Timestamp: ...
    def __init__(
        self,
        *,
        view_id: builtins.str = ...,
        name: builtins.str = ...,
        axis_groups: global___View.AxisGroups | None = ...,
        channels: collections.abc.Iterable[global___View.Channel] | None = ...,
        created_date: google.protobuf.timestamp_pb2.Timestamp | None = ...,
        modified_date: google.protobuf.timestamp_pb2.Timestamp | None = ...,
        created_by_user_id: builtins.str = ...,
        organization_id: builtins.str = ...,
        modified_by_user_id: builtins.str = ...,
        is_pinned: builtins.bool = ...,
    ) -> None: ...
    def HasField(self, field_name: typing.Literal["axis_groups", b"axis_groups", "created_date", b"created_date", "modified_date", b"modified_date"]) -> builtins.bool: ...
    def ClearField(self, field_name: typing.Literal["axis_groups", b"axis_groups", "channels", b"channels", "created_by_user_id", b"created_by_user_id", "created_date", b"created_date", "is_pinned", b"is_pinned", "modified_by_user_id", b"modified_by_user_id", "modified_date", b"modified_date", "name", b"name", "organization_id", b"organization_id", "view_id", b"view_id"]) -> None: ...

global___View = View

@typing.final
class GetViewRequest(google.protobuf.message.Message):
    DESCRIPTOR: google.protobuf.descriptor.Descriptor

    VIEW_ID_FIELD_NUMBER: builtins.int
    view_id: builtins.str
    def __init__(
        self,
        *,
        view_id: builtins.str = ...,
    ) -> None: ...
    def ClearField(self, field_name: typing.Literal["view_id", b"view_id"]) -> None: ...

global___GetViewRequest = GetViewRequest

@typing.final
class GetViewResponse(google.protobuf.message.Message):
    DESCRIPTOR: google.protobuf.descriptor.Descriptor

    VIEW_FIELD_NUMBER: builtins.int
    @property
    def view(self) -> global___View: ...
    def __init__(
        self,
        *,
        view: global___View | None = ...,
    ) -> None: ...
    def HasField(self, field_name: typing.Literal["view", b"view"]) -> builtins.bool: ...
    def ClearField(self, field_name: typing.Literal["view", b"view"]) -> None: ...

global___GetViewResponse = GetViewResponse

@typing.final
class CreateViewRequest(google.protobuf.message.Message):
    DESCRIPTOR: google.protobuf.descriptor.Descriptor

    VIEW_FIELD_NUMBER: builtins.int
    @property
    def view(self) -> global___View: ...
    def __init__(
        self,
        *,
        view: global___View | None = ...,
    ) -> None: ...
    def HasField(self, field_name: typing.Literal["view", b"view"]) -> builtins.bool: ...
    def ClearField(self, field_name: typing.Literal["view", b"view"]) -> None: ...

global___CreateViewRequest = CreateViewRequest

@typing.final
class CreateViewResponse(google.protobuf.message.Message):
    DESCRIPTOR: google.protobuf.descriptor.Descriptor

    VIEW_FIELD_NUMBER: builtins.int
    @property
    def view(self) -> global___View: ...
    def __init__(
        self,
        *,
        view: global___View | None = ...,
    ) -> None: ...
    def HasField(self, field_name: typing.Literal["view", b"view"]) -> builtins.bool: ...
    def ClearField(self, field_name: typing.Literal["view", b"view"]) -> None: ...

global___CreateViewResponse = CreateViewResponse

@typing.final
class UpdateViewRequest(google.protobuf.message.Message):
    DESCRIPTOR: google.protobuf.descriptor.Descriptor

    VIEW_FIELD_NUMBER: builtins.int
    UPDATE_MASK_FIELD_NUMBER: builtins.int
    @property
    def view(self) -> global___View: ...
    @property
    def update_mask(self) -> google.protobuf.field_mask_pb2.FieldMask:
        """The list of fields to update."""

    def __init__(
        self,
        *,
        view: global___View | None = ...,
        update_mask: google.protobuf.field_mask_pb2.FieldMask | None = ...,
    ) -> None: ...
    def HasField(self, field_name: typing.Literal["update_mask", b"update_mask", "view", b"view"]) -> builtins.bool: ...
    def ClearField(self, field_name: typing.Literal["update_mask", b"update_mask", "view", b"view"]) -> None: ...

global___UpdateViewRequest = UpdateViewRequest

@typing.final
class UpdateViewResponse(google.protobuf.message.Message):
    DESCRIPTOR: google.protobuf.descriptor.Descriptor

    VIEW_FIELD_NUMBER: builtins.int
    @property
    def view(self) -> global___View: ...
    def __init__(
        self,
        *,
        view: global___View | None = ...,
    ) -> None: ...
    def HasField(self, field_name: typing.Literal["view", b"view"]) -> builtins.bool: ...
    def ClearField(self, field_name: typing.Literal["view", b"view"]) -> None: ...

global___UpdateViewResponse = UpdateViewResponse

@typing.final
class ListViewsRequest(google.protobuf.message.Message):
    DESCRIPTOR: google.protobuf.descriptor.Descriptor

    PAGE_SIZE_FIELD_NUMBER: builtins.int
    PAGE_TOKEN_FIELD_NUMBER: builtins.int
    FILTER_FIELD_NUMBER: builtins.int
    page_size: builtins.int
    """The maximum number of views to return.
    The service may return fewer than this value.
    If unspecified, at most 50 views will be returned.
    The maximum value is 1000; values above 1000 will be coerced to 1000.
    """
    page_token: builtins.str
    """A page token, received from a previous `ListViews` call.
    Provide this to retrieve the subsequent page.
    When paginating, all other parameters provided to `ListViews` must match
    the call that provided the page token.
    """
    filter: builtins.str
    """A [Common Expression Language (CEL)](https://github.com/google/cel-spec) filter string
    Available fields to filter by are 'name', 'createdDate', and 'modifiedDate'.
    For further information about how to use CELs, please refer to [this guide](https://github.com/google/cel-spec/blob/master/doc/langdef.md#standard-definitions).
    For more information about the fields used for filtering, please refer to [this definition](/api/grpc/protocol_buffers/views#view). Optional.
    """
    def __init__(
        self,
        *,
        page_size: builtins.int = ...,
        page_token: builtins.str = ...,
        filter: builtins.str = ...,
    ) -> None: ...
    def ClearField(self, field_name: typing.Literal["filter", b"filter", "page_size", b"page_size", "page_token", b"page_token"]) -> None: ...

global___ListViewsRequest = ListViewsRequest

@typing.final
class ListViewsResponse(google.protobuf.message.Message):
    DESCRIPTOR: google.protobuf.descriptor.Descriptor

    VIEWS_FIELD_NUMBER: builtins.int
    NEXT_PAGE_TOKEN_FIELD_NUMBER: builtins.int
    next_page_token: builtins.str
    """A token, which can be sent as `page_token` to retrieve the next page.
    If this field is omitted, there are no subsequent pages.
    """
    @property
    def views(self) -> google.protobuf.internal.containers.RepeatedCompositeFieldContainer[global___View]: ...
    def __init__(
        self,
        *,
        views: collections.abc.Iterable[global___View] | None = ...,
        next_page_token: builtins.str = ...,
    ) -> None: ...
    def ClearField(self, field_name: typing.Literal["next_page_token", b"next_page_token", "views", b"views"]) -> None: ...

global___ListViewsResponse = ListViewsResponse

@typing.final
class ListApplicableViewsRequest(google.protobuf.message.Message):
    DESCRIPTOR: google.protobuf.descriptor.Descriptor

    PAGE_SIZE_FIELD_NUMBER: builtins.int
    PAGE_TOKEN_FIELD_NUMBER: builtins.int
    ASSET_IDS_FIELD_NUMBER: builtins.int
    RUN_IDS_FIELD_NUMBER: builtins.int
    page_size: builtins.int
    """The maximum number of views to return.
    The service may return fewer than this value.
    If unspecified, at most 50 views will be returned.
    The maximum value is 1000; values above 1000 will be coerced to 1000.
    """
    page_token: builtins.str
    """A page token, received from a previous `ListApplicableViews` call.
    Provide this to retrieve the subsequent page.
    When paginating, all other parameters provided to `ListApplicableViews` must match
    the call that provided the page token.
    """
    @property
    def asset_ids(self) -> google.protobuf.internal.containers.RepeatedScalarFieldContainer[builtins.str]: ...
    @property
    def run_ids(self) -> google.protobuf.internal.containers.RepeatedScalarFieldContainer[builtins.str]: ...
    def __init__(
        self,
        *,
        page_size: builtins.int = ...,
        page_token: builtins.str = ...,
        asset_ids: collections.abc.Iterable[builtins.str] | None = ...,
        run_ids: collections.abc.Iterable[builtins.str] | None = ...,
    ) -> None: ...
    def ClearField(self, field_name: typing.Literal["asset_ids", b"asset_ids", "page_size", b"page_size", "page_token", b"page_token", "run_ids", b"run_ids"]) -> None: ...

global___ListApplicableViewsRequest = ListApplicableViewsRequest

@typing.final
class ListApplicableViewsResponse(google.protobuf.message.Message):
    DESCRIPTOR: google.protobuf.descriptor.Descriptor

    VIEWS_FIELD_NUMBER: builtins.int
    NEXT_PAGE_TOKEN_FIELD_NUMBER: builtins.int
    next_page_token: builtins.str
    """A token, which can be sent as `page_token` to retrieve the next page.
    If this field is omitted, there are no subsequent pages.
    """
    @property
    def views(self) -> google.protobuf.internal.containers.RepeatedCompositeFieldContainer[global___View]: ...
    def __init__(
        self,
        *,
        views: collections.abc.Iterable[global___View] | None = ...,
        next_page_token: builtins.str = ...,
    ) -> None: ...
    def ClearField(self, field_name: typing.Literal["next_page_token", b"next_page_token", "views", b"views"]) -> None: ...

global___ListApplicableViewsResponse = ListApplicableViewsResponse

@typing.final
class DeleteViewRequest(google.protobuf.message.Message):
    DESCRIPTOR: google.protobuf.descriptor.Descriptor

    VIEW_ID_FIELD_NUMBER: builtins.int
    view_id: builtins.str
    def __init__(
        self,
        *,
        view_id: builtins.str = ...,
    ) -> None: ...
    def ClearField(self, field_name: typing.Literal["view_id", b"view_id"]) -> None: ...

global___DeleteViewRequest = DeleteViewRequest

@typing.final
class DeleteViewResponse(google.protobuf.message.Message):
    DESCRIPTOR: google.protobuf.descriptor.Descriptor

    def __init__(
        self,
    ) -> None: ...

global___DeleteViewResponse = DeleteViewResponse

@typing.final
class PinViewRequest(google.protobuf.message.Message):
    DESCRIPTOR: google.protobuf.descriptor.Descriptor

    VIEW_ID_FIELD_NUMBER: builtins.int
    view_id: builtins.str
    def __init__(
        self,
        *,
        view_id: builtins.str = ...,
    ) -> None: ...
    def ClearField(self, field_name: typing.Literal["view_id", b"view_id"]) -> None: ...

global___PinViewRequest = PinViewRequest

@typing.final
class PinViewResponse(google.protobuf.message.Message):
    DESCRIPTOR: google.protobuf.descriptor.Descriptor

    def __init__(
        self,
    ) -> None: ...

global___PinViewResponse = PinViewResponse

@typing.final
class UnpinViewRequest(google.protobuf.message.Message):
    DESCRIPTOR: google.protobuf.descriptor.Descriptor

    VIEW_ID_FIELD_NUMBER: builtins.int
    view_id: builtins.str
    def __init__(
        self,
        *,
        view_id: builtins.str = ...,
    ) -> None: ...
    def ClearField(self, field_name: typing.Literal["view_id", b"view_id"]) -> None: ...

global___UnpinViewRequest = UnpinViewRequest

@typing.final
class UnpinViewResponse(google.protobuf.message.Message):
    DESCRIPTOR: google.protobuf.descriptor.Descriptor

    def __init__(
        self,
    ) -> None: ...

global___UnpinViewResponse = UnpinViewResponse
