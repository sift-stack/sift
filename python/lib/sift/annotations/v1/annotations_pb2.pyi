"""
@generated by mypy-protobuf.  Do not edit manually!
isort:skip_file
"""

import builtins
import collections.abc
import google.protobuf.descriptor
import google.protobuf.field_mask_pb2
import google.protobuf.internal.containers
import google.protobuf.internal.enum_type_wrapper
import google.protobuf.message
import google.protobuf.timestamp_pb2
import sys
import typing

if sys.version_info >= (3, 10):
    import typing as typing_extensions
else:
    import typing_extensions

DESCRIPTOR: google.protobuf.descriptor.FileDescriptor

class _AnnotationState:
    ValueType = typing.NewType("ValueType", builtins.int)
    V: typing_extensions.TypeAlias = ValueType

class _AnnotationStateEnumTypeWrapper(google.protobuf.internal.enum_type_wrapper._EnumTypeWrapper[_AnnotationState.ValueType], builtins.type):
    DESCRIPTOR: google.protobuf.descriptor.EnumDescriptor
    ANNOTATION_STATE_UNSPECIFIED: _AnnotationState.ValueType  # 0
    ANNOTATION_STATE_OPEN: _AnnotationState.ValueType  # 1
    ANNOTATION_STATE_FLAGGED: _AnnotationState.ValueType  # 2
    ANNOTATION_STATE_RESOLVED: _AnnotationState.ValueType  # 3

class AnnotationState(_AnnotationState, metaclass=_AnnotationStateEnumTypeWrapper): ...

ANNOTATION_STATE_UNSPECIFIED: AnnotationState.ValueType  # 0
ANNOTATION_STATE_OPEN: AnnotationState.ValueType  # 1
ANNOTATION_STATE_FLAGGED: AnnotationState.ValueType  # 2
ANNOTATION_STATE_RESOLVED: AnnotationState.ValueType  # 3
global___AnnotationState = AnnotationState

class _AnnotationType:
    ValueType = typing.NewType("ValueType", builtins.int)
    V: typing_extensions.TypeAlias = ValueType

class _AnnotationTypeEnumTypeWrapper(google.protobuf.internal.enum_type_wrapper._EnumTypeWrapper[_AnnotationType.ValueType], builtins.type):
    DESCRIPTOR: google.protobuf.descriptor.EnumDescriptor
    ANNOTATION_TYPE_UNSPECIFIED: _AnnotationType.ValueType  # 0
    ANNOTATION_TYPE_DATA_REVIEW: _AnnotationType.ValueType  # 1
    ANNOTATION_TYPE_PHASE: _AnnotationType.ValueType  # 2

class AnnotationType(_AnnotationType, metaclass=_AnnotationTypeEnumTypeWrapper): ...

ANNOTATION_TYPE_UNSPECIFIED: AnnotationType.ValueType  # 0
ANNOTATION_TYPE_DATA_REVIEW: AnnotationType.ValueType  # 1
ANNOTATION_TYPE_PHASE: AnnotationType.ValueType  # 2
global___AnnotationType = AnnotationType

@typing.final
class Annotation(google.protobuf.message.Message):
    DESCRIPTOR: google.protobuf.descriptor.Descriptor

    ANNOTATION_ID_FIELD_NUMBER: builtins.int
    NAME_FIELD_NUMBER: builtins.int
    DESCRIPTION_FIELD_NUMBER: builtins.int
    START_TIME_FIELD_NUMBER: builtins.int
    END_TIME_FIELD_NUMBER: builtins.int
    CREATED_BY_USER_ID_FIELD_NUMBER: builtins.int
    MODIFIED_BY_USER_ID_FIELD_NUMBER: builtins.int
    CREATED_DATE_FIELD_NUMBER: builtins.int
    MODIFIED_DATE_FIELD_NUMBER: builtins.int
    RUN_ID_FIELD_NUMBER: builtins.int
    STATE_FIELD_NUMBER: builtins.int
    ORGANIZATION_ID_FIELD_NUMBER: builtins.int
    ASSIGNED_TO_USER_ID_FIELD_NUMBER: builtins.int
    ANNOTATION_TYPE_FIELD_NUMBER: builtins.int
    TAGS_FIELD_NUMBER: builtins.int
    LEGEND_CONFIG_FIELD_NUMBER: builtins.int
    CREATED_BY_CONDITION_ID_FIELD_NUMBER: builtins.int
    CREATED_BY_RULE_CONDITION_VERSION_ID_FIELD_NUMBER: builtins.int
    REPORT_RULE_VERSION_ID_FIELD_NUMBER: builtins.int
    PENDING_FIELD_NUMBER: builtins.int
    annotation_id: builtins.str
    name: builtins.str
    description: builtins.str
    created_by_user_id: builtins.str
    modified_by_user_id: builtins.str
    run_id: builtins.str
    state: global___AnnotationState.ValueType
    organization_id: builtins.str
    assigned_to_user_id: builtins.str
    annotation_type: global___AnnotationType.ValueType
    legend_config: builtins.str
    created_by_condition_id: builtins.str
    created_by_rule_condition_version_id: builtins.str
    report_rule_version_id: builtins.str
    pending: builtins.bool
    """An annotation is pending if it is part of an ongoing violation of a rule condition.
    The `end_time` of a pending annotation might be set, but is not yet finalized.
    """
    @property
    def start_time(self) -> google.protobuf.timestamp_pb2.Timestamp: ...
    @property
    def end_time(self) -> google.protobuf.timestamp_pb2.Timestamp: ...
    @property
    def created_date(self) -> google.protobuf.timestamp_pb2.Timestamp: ...
    @property
    def modified_date(self) -> google.protobuf.timestamp_pb2.Timestamp: ...
    @property
    def tags(self) -> google.protobuf.internal.containers.RepeatedScalarFieldContainer[builtins.str]: ...
    def __init__(
        self,
        *,
        annotation_id: builtins.str = ...,
        name: builtins.str = ...,
        description: builtins.str = ...,
        start_time: google.protobuf.timestamp_pb2.Timestamp | None = ...,
        end_time: google.protobuf.timestamp_pb2.Timestamp | None = ...,
        created_by_user_id: builtins.str = ...,
        modified_by_user_id: builtins.str = ...,
        created_date: google.protobuf.timestamp_pb2.Timestamp | None = ...,
        modified_date: google.protobuf.timestamp_pb2.Timestamp | None = ...,
        run_id: builtins.str | None = ...,
        state: global___AnnotationState.ValueType | None = ...,
        organization_id: builtins.str = ...,
        assigned_to_user_id: builtins.str = ...,
        annotation_type: global___AnnotationType.ValueType = ...,
        tags: collections.abc.Iterable[builtins.str] | None = ...,
        legend_config: builtins.str | None = ...,
        created_by_condition_id: builtins.str | None = ...,
        created_by_rule_condition_version_id: builtins.str | None = ...,
        report_rule_version_id: builtins.str | None = ...,
        pending: builtins.bool = ...,
    ) -> None: ...
    def HasField(self, field_name: typing.Literal["_created_by_condition_id", b"_created_by_condition_id", "_created_by_rule_condition_version_id", b"_created_by_rule_condition_version_id", "_legend_config", b"_legend_config", "_report_rule_version_id", b"_report_rule_version_id", "_run_id", b"_run_id", "_state", b"_state", "created_by_condition_id", b"created_by_condition_id", "created_by_rule_condition_version_id", b"created_by_rule_condition_version_id", "created_date", b"created_date", "end_time", b"end_time", "legend_config", b"legend_config", "modified_date", b"modified_date", "report_rule_version_id", b"report_rule_version_id", "run_id", b"run_id", "start_time", b"start_time", "state", b"state"]) -> builtins.bool: ...
    def ClearField(self, field_name: typing.Literal["_created_by_condition_id", b"_created_by_condition_id", "_created_by_rule_condition_version_id", b"_created_by_rule_condition_version_id", "_legend_config", b"_legend_config", "_report_rule_version_id", b"_report_rule_version_id", "_run_id", b"_run_id", "_state", b"_state", "annotation_id", b"annotation_id", "annotation_type", b"annotation_type", "assigned_to_user_id", b"assigned_to_user_id", "created_by_condition_id", b"created_by_condition_id", "created_by_rule_condition_version_id", b"created_by_rule_condition_version_id", "created_by_user_id", b"created_by_user_id", "created_date", b"created_date", "description", b"description", "end_time", b"end_time", "legend_config", b"legend_config", "modified_by_user_id", b"modified_by_user_id", "modified_date", b"modified_date", "name", b"name", "organization_id", b"organization_id", "pending", b"pending", "report_rule_version_id", b"report_rule_version_id", "run_id", b"run_id", "start_time", b"start_time", "state", b"state", "tags", b"tags"]) -> None: ...
    @typing.overload
    def WhichOneof(self, oneof_group: typing.Literal["_created_by_condition_id", b"_created_by_condition_id"]) -> typing.Literal["created_by_condition_id"] | None: ...
    @typing.overload
    def WhichOneof(self, oneof_group: typing.Literal["_created_by_rule_condition_version_id", b"_created_by_rule_condition_version_id"]) -> typing.Literal["created_by_rule_condition_version_id"] | None: ...
    @typing.overload
    def WhichOneof(self, oneof_group: typing.Literal["_legend_config", b"_legend_config"]) -> typing.Literal["legend_config"] | None: ...
    @typing.overload
    def WhichOneof(self, oneof_group: typing.Literal["_report_rule_version_id", b"_report_rule_version_id"]) -> typing.Literal["report_rule_version_id"] | None: ...
    @typing.overload
    def WhichOneof(self, oneof_group: typing.Literal["_run_id", b"_run_id"]) -> typing.Literal["run_id"] | None: ...
    @typing.overload
    def WhichOneof(self, oneof_group: typing.Literal["_state", b"_state"]) -> typing.Literal["state"] | None: ...

global___Annotation = Annotation

@typing.final
class AnnotationLinkedChannelsChannel(google.protobuf.message.Message):
    DESCRIPTOR: google.protobuf.descriptor.Descriptor

    CHANNEL_ID_FIELD_NUMBER: builtins.int
    channel_id: builtins.str
    def __init__(
        self,
        *,
        channel_id: builtins.str = ...,
    ) -> None: ...
    def ClearField(self, field_name: typing.Literal["channel_id", b"channel_id"]) -> None: ...

global___AnnotationLinkedChannelsChannel = AnnotationLinkedChannelsChannel

@typing.final
class AnnotationLinkedChannelsBitFieldElement(google.protobuf.message.Message):
    DESCRIPTOR: google.protobuf.descriptor.Descriptor

    CHANNEL_ID_FIELD_NUMBER: builtins.int
    BIT_FIELD_NAME_FIELD_NUMBER: builtins.int
    channel_id: builtins.str
    bit_field_name: builtins.str
    def __init__(
        self,
        *,
        channel_id: builtins.str = ...,
        bit_field_name: builtins.str = ...,
    ) -> None: ...
    def ClearField(self, field_name: typing.Literal["bit_field_name", b"bit_field_name", "channel_id", b"channel_id"]) -> None: ...

global___AnnotationLinkedChannelsBitFieldElement = AnnotationLinkedChannelsBitFieldElement

@typing.final
class AnnotationLinkedChannel(google.protobuf.message.Message):
    DESCRIPTOR: google.protobuf.descriptor.Descriptor

    CHANNEL_FIELD_NUMBER: builtins.int
    BIT_FIELD_ELEMENT_FIELD_NUMBER: builtins.int
    @property
    def channel(self) -> global___AnnotationLinkedChannelsChannel: ...
    @property
    def bit_field_element(self) -> global___AnnotationLinkedChannelsBitFieldElement: ...
    def __init__(
        self,
        *,
        channel: global___AnnotationLinkedChannelsChannel | None = ...,
        bit_field_element: global___AnnotationLinkedChannelsBitFieldElement | None = ...,
    ) -> None: ...
    def HasField(self, field_name: typing.Literal["bit_field_element", b"bit_field_element", "channel", b"channel", "type", b"type"]) -> builtins.bool: ...
    def ClearField(self, field_name: typing.Literal["bit_field_element", b"bit_field_element", "channel", b"channel", "type", b"type"]) -> None: ...
    def WhichOneof(self, oneof_group: typing.Literal["type", b"type"]) -> typing.Literal["channel", "bit_field_element"] | None: ...

global___AnnotationLinkedChannel = AnnotationLinkedChannel

@typing.final
class CreateAnnotationRequest(google.protobuf.message.Message):
    """The request for a call to `AnnotationService_CreateAnnotation` to create a new annotation.
    At least 1 asset, tag, or channel must be specified.
    """

    DESCRIPTOR: google.protobuf.descriptor.Descriptor

    NAME_FIELD_NUMBER: builtins.int
    DESCRIPTION_FIELD_NUMBER: builtins.int
    START_TIME_FIELD_NUMBER: builtins.int
    END_TIME_FIELD_NUMBER: builtins.int
    ASSETS_FIELD_NUMBER: builtins.int
    LINKED_CHANNELS_FIELD_NUMBER: builtins.int
    TAGS_FIELD_NUMBER: builtins.int
    RUN_ID_FIELD_NUMBER: builtins.int
    ASSIGN_TO_USER_ID_FIELD_NUMBER: builtins.int
    ORGANIZATION_ID_FIELD_NUMBER: builtins.int
    STATE_FIELD_NUMBER: builtins.int
    ANNOTATION_TYPE_FIELD_NUMBER: builtins.int
    CREATED_BY_CONDITION_ID_FIELD_NUMBER: builtins.int
    LEGEND_CONFIG_FIELD_NUMBER: builtins.int
    CREATED_BY_RULE_CONDITION_VERSION_ID_FIELD_NUMBER: builtins.int
    name: builtins.str
    """The name assigned to the new annotation."""
    description: builtins.str
    """A short description about the new annotation."""
    run_id: builtins.str
    """The ID of the run that this annotation is associated with."""
    assign_to_user_id: builtins.str
    """The ID of the user that this annotation is assigned to."""
    organization_id: builtins.str
    """The organization associated with this annotation. An organization ID is only required
    if the user belongs to multiple organizations.
    """
    state: global___AnnotationState.ValueType
    """The state of the annotation. If an annotation has an `annotation_type` of `ANNOTATION_TYPE_PHASE`, then state must be
    unset, otherwise an error will be returned.
    """
    annotation_type: global___AnnotationType.ValueType
    """The type of the annotation."""
    created_by_condition_id: builtins.str
    """The ID of the rule condition that created this annotation."""
    legend_config: builtins.str
    """A JSON string containing the axes configuration of the annotation's linked channels."""
    created_by_rule_condition_version_id: builtins.str
    """The ID of the rule condition version that created this annotation."""
    @property
    def start_time(self) -> google.protobuf.timestamp_pb2.Timestamp:
        """When the annotation starts."""

    @property
    def end_time(self) -> google.protobuf.timestamp_pb2.Timestamp:
        """When the annotation ends."""

    @property
    def assets(self) -> google.protobuf.internal.containers.RepeatedScalarFieldContainer[builtins.str]:
        """The names of the assets to associate with this annotation."""

    @property
    def linked_channels(self) -> google.protobuf.internal.containers.RepeatedCompositeFieldContainer[global___AnnotationLinkedChannel]:
        """The channels to associate with this annotation."""

    @property
    def tags(self) -> google.protobuf.internal.containers.RepeatedScalarFieldContainer[builtins.str]:
        """The names of the tags to associate with this annotation."""

    def __init__(
        self,
        *,
        name: builtins.str = ...,
        description: builtins.str = ...,
        start_time: google.protobuf.timestamp_pb2.Timestamp | None = ...,
        end_time: google.protobuf.timestamp_pb2.Timestamp | None = ...,
        assets: collections.abc.Iterable[builtins.str] | None = ...,
        linked_channels: collections.abc.Iterable[global___AnnotationLinkedChannel] | None = ...,
        tags: collections.abc.Iterable[builtins.str] | None = ...,
        run_id: builtins.str | None = ...,
        assign_to_user_id: builtins.str | None = ...,
        organization_id: builtins.str = ...,
        state: global___AnnotationState.ValueType | None = ...,
        annotation_type: global___AnnotationType.ValueType = ...,
        created_by_condition_id: builtins.str | None = ...,
        legend_config: builtins.str | None = ...,
        created_by_rule_condition_version_id: builtins.str | None = ...,
    ) -> None: ...
    def HasField(self, field_name: typing.Literal["_assign_to_user_id", b"_assign_to_user_id", "_created_by_condition_id", b"_created_by_condition_id", "_created_by_rule_condition_version_id", b"_created_by_rule_condition_version_id", "_legend_config", b"_legend_config", "_run_id", b"_run_id", "_state", b"_state", "assign_to_user_id", b"assign_to_user_id", "created_by_condition_id", b"created_by_condition_id", "created_by_rule_condition_version_id", b"created_by_rule_condition_version_id", "end_time", b"end_time", "legend_config", b"legend_config", "run_id", b"run_id", "start_time", b"start_time", "state", b"state"]) -> builtins.bool: ...
    def ClearField(self, field_name: typing.Literal["_assign_to_user_id", b"_assign_to_user_id", "_created_by_condition_id", b"_created_by_condition_id", "_created_by_rule_condition_version_id", b"_created_by_rule_condition_version_id", "_legend_config", b"_legend_config", "_run_id", b"_run_id", "_state", b"_state", "annotation_type", b"annotation_type", "assets", b"assets", "assign_to_user_id", b"assign_to_user_id", "created_by_condition_id", b"created_by_condition_id", "created_by_rule_condition_version_id", b"created_by_rule_condition_version_id", "description", b"description", "end_time", b"end_time", "legend_config", b"legend_config", "linked_channels", b"linked_channels", "name", b"name", "organization_id", b"organization_id", "run_id", b"run_id", "start_time", b"start_time", "state", b"state", "tags", b"tags"]) -> None: ...
    @typing.overload
    def WhichOneof(self, oneof_group: typing.Literal["_assign_to_user_id", b"_assign_to_user_id"]) -> typing.Literal["assign_to_user_id"] | None: ...
    @typing.overload
    def WhichOneof(self, oneof_group: typing.Literal["_created_by_condition_id", b"_created_by_condition_id"]) -> typing.Literal["created_by_condition_id"] | None: ...
    @typing.overload
    def WhichOneof(self, oneof_group: typing.Literal["_created_by_rule_condition_version_id", b"_created_by_rule_condition_version_id"]) -> typing.Literal["created_by_rule_condition_version_id"] | None: ...
    @typing.overload
    def WhichOneof(self, oneof_group: typing.Literal["_legend_config", b"_legend_config"]) -> typing.Literal["legend_config"] | None: ...
    @typing.overload
    def WhichOneof(self, oneof_group: typing.Literal["_run_id", b"_run_id"]) -> typing.Literal["run_id"] | None: ...
    @typing.overload
    def WhichOneof(self, oneof_group: typing.Literal["_state", b"_state"]) -> typing.Literal["state"] | None: ...

global___CreateAnnotationRequest = CreateAnnotationRequest

@typing.final
class CreateAnnotationResponse(google.protobuf.message.Message):
    """The result of a call to `AnnotationService_CreateAnnotation`."""

    DESCRIPTOR: google.protobuf.descriptor.Descriptor

    ANNOTATION_FIELD_NUMBER: builtins.int
    @property
    def annotation(self) -> global___Annotation: ...
    def __init__(
        self,
        *,
        annotation: global___Annotation | None = ...,
    ) -> None: ...
    def HasField(self, field_name: typing.Literal["annotation", b"annotation"]) -> builtins.bool: ...
    def ClearField(self, field_name: typing.Literal["annotation", b"annotation"]) -> None: ...

global___CreateAnnotationResponse = CreateAnnotationResponse

@typing.final
class DeleteAnnotationRequest(google.protobuf.message.Message):
    """The request for a call to `AnnotationService_DeleteAnnotation`."""

    DESCRIPTOR: google.protobuf.descriptor.Descriptor

    ANNOTATION_ID_FIELD_NUMBER: builtins.int
    annotation_id: builtins.str
    def __init__(
        self,
        *,
        annotation_id: builtins.str = ...,
    ) -> None: ...
    def ClearField(self, field_name: typing.Literal["annotation_id", b"annotation_id"]) -> None: ...

global___DeleteAnnotationRequest = DeleteAnnotationRequest

@typing.final
class DeleteAnnotationResponse(google.protobuf.message.Message):
    """The response of a call to `AnnotationService_DeleteAnnotation`."""

    DESCRIPTOR: google.protobuf.descriptor.Descriptor

    def __init__(
        self,
    ) -> None: ...

global___DeleteAnnotationResponse = DeleteAnnotationResponse

@typing.final
class BatchDeleteAnnotationsRequest(google.protobuf.message.Message):
    DESCRIPTOR: google.protobuf.descriptor.Descriptor

    ANNOTATION_IDS_FIELD_NUMBER: builtins.int
    @property
    def annotation_ids(self) -> google.protobuf.internal.containers.RepeatedScalarFieldContainer[builtins.str]:
        """Limit of 1000 annotations per batch"""

    def __init__(
        self,
        *,
        annotation_ids: collections.abc.Iterable[builtins.str] | None = ...,
    ) -> None: ...
    def ClearField(self, field_name: typing.Literal["annotation_ids", b"annotation_ids"]) -> None: ...

global___BatchDeleteAnnotationsRequest = BatchDeleteAnnotationsRequest

@typing.final
class BatchDeleteAnnotationsResponse(google.protobuf.message.Message):
    DESCRIPTOR: google.protobuf.descriptor.Descriptor

    def __init__(
        self,
    ) -> None: ...

global___BatchDeleteAnnotationsResponse = BatchDeleteAnnotationsResponse

@typing.final
class GetAnnotationRequest(google.protobuf.message.Message):
    """The request for a call to `AnnotationService_GetAnnotation`."""

    DESCRIPTOR: google.protobuf.descriptor.Descriptor

    ANNOTATION_ID_FIELD_NUMBER: builtins.int
    annotation_id: builtins.str
    def __init__(
        self,
        *,
        annotation_id: builtins.str = ...,
    ) -> None: ...
    def ClearField(self, field_name: typing.Literal["annotation_id", b"annotation_id"]) -> None: ...

global___GetAnnotationRequest = GetAnnotationRequest

@typing.final
class GetAnnotationResponse(google.protobuf.message.Message):
    """The response of a call to `AnnotationService_GetAnnotation`."""

    DESCRIPTOR: google.protobuf.descriptor.Descriptor

    ANNOTATION_FIELD_NUMBER: builtins.int
    @property
    def annotation(self) -> global___Annotation: ...
    def __init__(
        self,
        *,
        annotation: global___Annotation | None = ...,
    ) -> None: ...
    def HasField(self, field_name: typing.Literal["annotation", b"annotation"]) -> builtins.bool: ...
    def ClearField(self, field_name: typing.Literal["annotation", b"annotation"]) -> None: ...

global___GetAnnotationResponse = GetAnnotationResponse

@typing.final
class ListAnnotationsRequest(google.protobuf.message.Message):
    """The request for a call to `AnnotationService_ListAnnotations` to retrieve annotations."""

    DESCRIPTOR: google.protobuf.descriptor.Descriptor

    PAGE_SIZE_FIELD_NUMBER: builtins.int
    PAGE_TOKEN_FIELD_NUMBER: builtins.int
    FILTER_FIELD_NUMBER: builtins.int
    ORGANIZATION_ID_FIELD_NUMBER: builtins.int
    ORDER_BY_FIELD_NUMBER: builtins.int
    page_size: builtins.int
    """The maximum number of annotations to return. The service may return fewer than this value.
    If unspecified, at most 50 annotations will be returned. The maximum value is 1000; values above
    1000 will be coerced to 1000. Optional.
    """
    page_token: builtins.str
    """A page token, received from a previous `ListAnnotations` call.
    Provide this to retrieve the subsequent page.
    When paginating, all other parameters provided to `ListAnnotations` must match
    the call that provided the page token. Optional.
    """
    filter: builtins.str
    """A [Common Expression Language (CEL)](https://github.com/google/cel-spec) filter string.
    Available fields to filter by are `annotation_id`, `start_time`, `end_time`,
    `created_date`, `modified_date`, `run_id`, `name`, `description`, `state`, `created_by_user_id`, `created_by_rule_condition_version_id`,
    `annotation_type`, `tag_name`, `report_id`, `asset_id`, `asset_name`, `pending`, and `assignee`.
    For further information about how to use CELs, please refer to [this guide](https://github.com/google/cel-spec/blob/master/doc/langdef.md#standard-definitions).
    For more information about the fields used for filtering, please refer to [this definition](/docs/api/grpc/protocol-buffers/annotations#annotation). Optional.
    """
    organization_id: builtins.str
    """This field is only required if your user belongs to multiple organizations."""
    order_by: builtins.str
    """How to order the retrieved annotations. Formatted as a comma-separated string i.e. "FIELD_NAME[ desc],...".
    Available fields to order_by are `created_date`, `modified_date`, `start_time`, and `end_time`.
    If left empty, items are ordered by `created_date` in ascending order (oldest-first).
    For more information about the format of this field, read [this](https://google.aip.dev/132#ordering)
    Example: "created_date desc,modified_date"
    """
    def __init__(
        self,
        *,
        page_size: builtins.int = ...,
        page_token: builtins.str = ...,
        filter: builtins.str = ...,
        organization_id: builtins.str = ...,
        order_by: builtins.str = ...,
    ) -> None: ...
    def ClearField(self, field_name: typing.Literal["filter", b"filter", "order_by", b"order_by", "organization_id", b"organization_id", "page_size", b"page_size", "page_token", b"page_token"]) -> None: ...

global___ListAnnotationsRequest = ListAnnotationsRequest

@typing.final
class ListAnnotationsResponse(google.protobuf.message.Message):
    """The result of a call to `AnnotationService_ListAnnotations`."""

    DESCRIPTOR: google.protobuf.descriptor.Descriptor

    ANNOTATIONS_FIELD_NUMBER: builtins.int
    NEXT_PAGE_TOKEN_FIELD_NUMBER: builtins.int
    next_page_token: builtins.str
    """Oops, we skipped to index 5! No reason for that; the indices between aren't reserved or anything."""
    @property
    def annotations(self) -> google.protobuf.internal.containers.RepeatedCompositeFieldContainer[global___Annotation]: ...
    def __init__(
        self,
        *,
        annotations: collections.abc.Iterable[global___Annotation] | None = ...,
        next_page_token: builtins.str = ...,
    ) -> None: ...
    def ClearField(self, field_name: typing.Literal["annotations", b"annotations", "next_page_token", b"next_page_token"]) -> None: ...

global___ListAnnotationsResponse = ListAnnotationsResponse

@typing.final
class UpdateAnnotationRequest(google.protobuf.message.Message):
    """The request for a call to `AnnotationService_UpdateAnnotation` to update an annotation."""

    DESCRIPTOR: google.protobuf.descriptor.Descriptor

    ANNOTATION_FIELD_NUMBER: builtins.int
    UPDATE_MASK_FIELD_NUMBER: builtins.int
    @property
    def annotation(self) -> global___Annotation:
        """The annotation to update."""

    @property
    def update_mask(self) -> google.protobuf.field_mask_pb2.FieldMask:
        """The list of fields to be updated. The fields available to be updated are `name`, `description`, `start_time`,
        `end_time`, `assigned_to_user_id`, `state`, and `tags`.
        Important Note: if `tags` is specified in the update mask and `annotation.tags` is an empty list then all associated tags on the annotation
        will be removed.
        """

    def __init__(
        self,
        *,
        annotation: global___Annotation | None = ...,
        update_mask: google.protobuf.field_mask_pb2.FieldMask | None = ...,
    ) -> None: ...
    def HasField(self, field_name: typing.Literal["annotation", b"annotation", "update_mask", b"update_mask"]) -> builtins.bool: ...
    def ClearField(self, field_name: typing.Literal["annotation", b"annotation", "update_mask", b"update_mask"]) -> None: ...

global___UpdateAnnotationRequest = UpdateAnnotationRequest

@typing.final
class UpdateAnnotationResponse(google.protobuf.message.Message):
    """The response of a call to `AnnotationService_UpdateAnnotation`."""

    DESCRIPTOR: google.protobuf.descriptor.Descriptor

    ANNOTATION_FIELD_NUMBER: builtins.int
    @property
    def annotation(self) -> global___Annotation: ...
    def __init__(
        self,
        *,
        annotation: global___Annotation | None = ...,
    ) -> None: ...
    def HasField(self, field_name: typing.Literal["annotation", b"annotation"]) -> builtins.bool: ...
    def ClearField(self, field_name: typing.Literal["annotation", b"annotation"]) -> None: ...

global___UpdateAnnotationResponse = UpdateAnnotationResponse
