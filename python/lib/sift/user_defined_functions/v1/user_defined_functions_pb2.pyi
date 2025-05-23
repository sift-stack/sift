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
import sift.calculated_channels.v2.calculated_channels_pb2
import sift.common.type.v1.user_defined_functions_pb2
import sift.rules.v1.rules_pb2
import typing

DESCRIPTOR: google.protobuf.descriptor.FileDescriptor

@typing.final
class GetUserDefinedFunctionRequest(google.protobuf.message.Message):
    """The request for a call to `UserDefinedFunctionService_GetUserDefinedFunction` to retrieve the latest version of a user defined function.
    If `user_defined_function_id` is provided then all other arguments will be ignored. The argument `user_defined_function_id`
    should not be used together with `name`.
    """

    DESCRIPTOR: google.protobuf.descriptor.Descriptor

    USER_DEFINED_FUNCTION_ID_FIELD_NUMBER: builtins.int
    NAME_FIELD_NUMBER: builtins.int
    user_defined_function_id: builtins.str
    name: builtins.str
    def __init__(
        self,
        *,
        user_defined_function_id: builtins.str = ...,
        name: builtins.str = ...,
    ) -> None: ...
    def ClearField(self, field_name: typing.Literal["name", b"name", "user_defined_function_id", b"user_defined_function_id"]) -> None: ...

global___GetUserDefinedFunctionRequest = GetUserDefinedFunctionRequest

@typing.final
class GetUserDefinedFunctionResponse(google.protobuf.message.Message):
    DESCRIPTOR: google.protobuf.descriptor.Descriptor

    USER_DEFINED_FUNCTION_FIELD_NUMBER: builtins.int
    @property
    def user_defined_function(self) -> sift.common.type.v1.user_defined_functions_pb2.UserDefinedFunction: ...
    def __init__(
        self,
        *,
        user_defined_function: sift.common.type.v1.user_defined_functions_pb2.UserDefinedFunction | None = ...,
    ) -> None: ...
    def HasField(self, field_name: typing.Literal["user_defined_function", b"user_defined_function"]) -> builtins.bool: ...
    def ClearField(self, field_name: typing.Literal["user_defined_function", b"user_defined_function"]) -> None: ...

global___GetUserDefinedFunctionResponse = GetUserDefinedFunctionResponse

@typing.final
class GetUserDefinedFunctionVersionRequest(google.protobuf.message.Message):
    """The request for a call to `UserDefinedFunctionService_GetUserDefinedFunctionVersion` to retrieve a specific version of a user defined function."""

    DESCRIPTOR: google.protobuf.descriptor.Descriptor

    USER_DEFINED_FUNCTION_VERSION_ID_FIELD_NUMBER: builtins.int
    user_defined_function_version_id: builtins.str
    def __init__(
        self,
        *,
        user_defined_function_version_id: builtins.str = ...,
    ) -> None: ...
    def ClearField(self, field_name: typing.Literal["user_defined_function_version_id", b"user_defined_function_version_id"]) -> None: ...

global___GetUserDefinedFunctionVersionRequest = GetUserDefinedFunctionVersionRequest

@typing.final
class GetUserDefinedFunctionVersionResponse(google.protobuf.message.Message):
    DESCRIPTOR: google.protobuf.descriptor.Descriptor

    USER_DEFINED_FUNCTION_FIELD_NUMBER: builtins.int
    @property
    def user_defined_function(self) -> sift.common.type.v1.user_defined_functions_pb2.UserDefinedFunction: ...
    def __init__(
        self,
        *,
        user_defined_function: sift.common.type.v1.user_defined_functions_pb2.UserDefinedFunction | None = ...,
    ) -> None: ...
    def HasField(self, field_name: typing.Literal["user_defined_function", b"user_defined_function"]) -> builtins.bool: ...
    def ClearField(self, field_name: typing.Literal["user_defined_function", b"user_defined_function"]) -> None: ...

global___GetUserDefinedFunctionVersionResponse = GetUserDefinedFunctionVersionResponse

@typing.final
class GetUserDefinedFunctionDependentsRequest(google.protobuf.message.Message):
    """The request for a call to `UserDefinedFunctionService_GetUserDefinedFunctionDependents` to retrieve versions of user defined functions."""

    DESCRIPTOR: google.protobuf.descriptor.Descriptor

    PAGE_SIZE_FIELD_NUMBER: builtins.int
    USER_DEFINED_FUNCTION_ID_FIELD_NUMBER: builtins.int
    USER_DEFINED_FUNCTION_NAME_FIELD_NUMBER: builtins.int
    USER_DEFINED_FUNCTION_VERSION_ID_FIELD_NUMBER: builtins.int
    page_size: builtins.int
    user_defined_function_id: builtins.str
    user_defined_function_name: builtins.str
    user_defined_function_version_id: builtins.str
    def __init__(
        self,
        *,
        page_size: builtins.int = ...,
        user_defined_function_id: builtins.str = ...,
        user_defined_function_name: builtins.str = ...,
        user_defined_function_version_id: builtins.str = ...,
    ) -> None: ...
    def HasField(self, field_name: typing.Literal["user_defined_function", b"user_defined_function", "user_defined_function_id", b"user_defined_function_id", "user_defined_function_name", b"user_defined_function_name", "user_defined_function_version_id", b"user_defined_function_version_id"]) -> builtins.bool: ...
    def ClearField(self, field_name: typing.Literal["page_size", b"page_size", "user_defined_function", b"user_defined_function", "user_defined_function_id", b"user_defined_function_id", "user_defined_function_name", b"user_defined_function_name", "user_defined_function_version_id", b"user_defined_function_version_id"]) -> None: ...
    def WhichOneof(self, oneof_group: typing.Literal["user_defined_function", b"user_defined_function"]) -> typing.Literal["user_defined_function_id", "user_defined_function_name", "user_defined_function_version_id"] | None: ...

global___GetUserDefinedFunctionDependentsRequest = GetUserDefinedFunctionDependentsRequest

@typing.final
class GetUserDefinedFunctionDependentsResponse(google.protobuf.message.Message):
    DESCRIPTOR: google.protobuf.descriptor.Descriptor

    USER_DEFINED_FUNCTIONS_FIELD_NUMBER: builtins.int
    NEXT_PAGE_TOKEN_USER_DEFINED_FUNCTION_FIELD_NUMBER: builtins.int
    CALCULATED_CHANNELS_FIELD_NUMBER: builtins.int
    NEXT_PAGE_TOKEN_CALCULATED_CHANNEL_FIELD_NUMBER: builtins.int
    RULES_FIELD_NUMBER: builtins.int
    NEXT_PAGE_TOKEN_RULE_FIELD_NUMBER: builtins.int
    next_page_token_user_defined_function: builtins.str
    next_page_token_calculated_channel: builtins.str
    next_page_token_rule: builtins.str
    @property
    def user_defined_functions(self) -> google.protobuf.internal.containers.RepeatedCompositeFieldContainer[sift.common.type.v1.user_defined_functions_pb2.UserDefinedFunction]: ...
    @property
    def calculated_channels(self) -> google.protobuf.internal.containers.RepeatedCompositeFieldContainer[sift.calculated_channels.v2.calculated_channels_pb2.CalculatedChannel]: ...
    @property
    def rules(self) -> google.protobuf.internal.containers.RepeatedCompositeFieldContainer[sift.rules.v1.rules_pb2.Rule]: ...
    def __init__(
        self,
        *,
        user_defined_functions: collections.abc.Iterable[sift.common.type.v1.user_defined_functions_pb2.UserDefinedFunction] | None = ...,
        next_page_token_user_defined_function: builtins.str = ...,
        calculated_channels: collections.abc.Iterable[sift.calculated_channels.v2.calculated_channels_pb2.CalculatedChannel] | None = ...,
        next_page_token_calculated_channel: builtins.str = ...,
        rules: collections.abc.Iterable[sift.rules.v1.rules_pb2.Rule] | None = ...,
        next_page_token_rule: builtins.str = ...,
    ) -> None: ...
    def ClearField(self, field_name: typing.Literal["calculated_channels", b"calculated_channels", "next_page_token_calculated_channel", b"next_page_token_calculated_channel", "next_page_token_rule", b"next_page_token_rule", "next_page_token_user_defined_function", b"next_page_token_user_defined_function", "rules", b"rules", "user_defined_functions", b"user_defined_functions"]) -> None: ...

global___GetUserDefinedFunctionDependentsResponse = GetUserDefinedFunctionDependentsResponse

@typing.final
class GetUserDefinedFunctionVersionsRequest(google.protobuf.message.Message):
    """The request for a call to `UserDefinedFunctionService_GetUserDefinedFunctionVersions` to retrieve versions of user defined functions."""

    DESCRIPTOR: google.protobuf.descriptor.Descriptor

    USER_DEFINED_FUNCTION_VERSION_IDS_FIELD_NUMBER: builtins.int
    @property
    def user_defined_function_version_ids(self) -> google.protobuf.internal.containers.RepeatedScalarFieldContainer[builtins.str]: ...
    def __init__(
        self,
        *,
        user_defined_function_version_ids: collections.abc.Iterable[builtins.str] | None = ...,
    ) -> None: ...
    def ClearField(self, field_name: typing.Literal["user_defined_function_version_ids", b"user_defined_function_version_ids"]) -> None: ...

global___GetUserDefinedFunctionVersionsRequest = GetUserDefinedFunctionVersionsRequest

@typing.final
class GetUserDefinedFunctionVersionsResponse(google.protobuf.message.Message):
    DESCRIPTOR: google.protobuf.descriptor.Descriptor

    USER_DEFINED_FUNCTIONS_FIELD_NUMBER: builtins.int
    @property
    def user_defined_functions(self) -> google.protobuf.internal.containers.RepeatedCompositeFieldContainer[sift.common.type.v1.user_defined_functions_pb2.UserDefinedFunction]: ...
    def __init__(
        self,
        *,
        user_defined_functions: collections.abc.Iterable[sift.common.type.v1.user_defined_functions_pb2.UserDefinedFunction] | None = ...,
    ) -> None: ...
    def ClearField(self, field_name: typing.Literal["user_defined_functions", b"user_defined_functions"]) -> None: ...

global___GetUserDefinedFunctionVersionsResponse = GetUserDefinedFunctionVersionsResponse

@typing.final
class CreateUserDefinedFunctionRequest(google.protobuf.message.Message):
    DESCRIPTOR: google.protobuf.descriptor.Descriptor

    NAME_FIELD_NUMBER: builtins.int
    DESCRIPTION_FIELD_NUMBER: builtins.int
    EXPRESSION_FIELD_NUMBER: builtins.int
    FUNCTION_INPUTS_FIELD_NUMBER: builtins.int
    USER_NOTES_FIELD_NUMBER: builtins.int
    name: builtins.str
    description: builtins.str
    expression: builtins.str
    user_notes: builtins.str
    @property
    def function_inputs(self) -> google.protobuf.internal.containers.RepeatedCompositeFieldContainer[sift.common.type.v1.user_defined_functions_pb2.FunctionInput]: ...
    def __init__(
        self,
        *,
        name: builtins.str = ...,
        description: builtins.str | None = ...,
        expression: builtins.str = ...,
        function_inputs: collections.abc.Iterable[sift.common.type.v1.user_defined_functions_pb2.FunctionInput] | None = ...,
        user_notes: builtins.str | None = ...,
    ) -> None: ...
    def HasField(self, field_name: typing.Literal["_description", b"_description", "_user_notes", b"_user_notes", "description", b"description", "user_notes", b"user_notes"]) -> builtins.bool: ...
    def ClearField(self, field_name: typing.Literal["_description", b"_description", "_user_notes", b"_user_notes", "description", b"description", "expression", b"expression", "function_inputs", b"function_inputs", "name", b"name", "user_notes", b"user_notes"]) -> None: ...
    @typing.overload
    def WhichOneof(self, oneof_group: typing.Literal["_description", b"_description"]) -> typing.Literal["description"] | None: ...
    @typing.overload
    def WhichOneof(self, oneof_group: typing.Literal["_user_notes", b"_user_notes"]) -> typing.Literal["user_notes"] | None: ...

global___CreateUserDefinedFunctionRequest = CreateUserDefinedFunctionRequest

@typing.final
class CreateUserDefinedFunctionResponse(google.protobuf.message.Message):
    DESCRIPTOR: google.protobuf.descriptor.Descriptor

    USER_DEFINED_FUNCTION_FIELD_NUMBER: builtins.int
    @property
    def user_defined_function(self) -> sift.common.type.v1.user_defined_functions_pb2.UserDefinedFunction: ...
    def __init__(
        self,
        *,
        user_defined_function: sift.common.type.v1.user_defined_functions_pb2.UserDefinedFunction | None = ...,
    ) -> None: ...
    def HasField(self, field_name: typing.Literal["user_defined_function", b"user_defined_function"]) -> builtins.bool: ...
    def ClearField(self, field_name: typing.Literal["user_defined_function", b"user_defined_function"]) -> None: ...

global___CreateUserDefinedFunctionResponse = CreateUserDefinedFunctionResponse

@typing.final
class ValidateUserDefinedFunctionRequest(google.protobuf.message.Message):
    DESCRIPTOR: google.protobuf.descriptor.Descriptor

    EXPRESSION_FIELD_NUMBER: builtins.int
    FUNCTION_INPUTS_FIELD_NUMBER: builtins.int
    expression: builtins.str
    @property
    def function_inputs(self) -> google.protobuf.internal.containers.RepeatedCompositeFieldContainer[sift.common.type.v1.user_defined_functions_pb2.FunctionInput]: ...
    def __init__(
        self,
        *,
        expression: builtins.str = ...,
        function_inputs: collections.abc.Iterable[sift.common.type.v1.user_defined_functions_pb2.FunctionInput] | None = ...,
    ) -> None: ...
    def ClearField(self, field_name: typing.Literal["expression", b"expression", "function_inputs", b"function_inputs"]) -> None: ...

global___ValidateUserDefinedFunctionRequest = ValidateUserDefinedFunctionRequest

@typing.final
class ValidateUserDefinedFunctionResponse(google.protobuf.message.Message):
    DESCRIPTOR: google.protobuf.descriptor.Descriptor

    @typing.final
    class ErrorValidatingUserDefinedFunctionResult(google.protobuf.message.Message):
        DESCRIPTOR: google.protobuf.descriptor.Descriptor

        ERROR_MESSAGE_FIELD_NUMBER: builtins.int
        error_message: builtins.str
        def __init__(
            self,
            *,
            error_message: builtins.str = ...,
        ) -> None: ...
        def ClearField(self, field_name: typing.Literal["error_message", b"error_message"]) -> None: ...

    @typing.final
    class SuccessValidatingUserDefinedFunctionResult(google.protobuf.message.Message):
        DESCRIPTOR: google.protobuf.descriptor.Descriptor

        USER_DEFINED_FUNCTION_FIELD_NUMBER: builtins.int
        @property
        def user_defined_function(self) -> sift.common.type.v1.user_defined_functions_pb2.UserDefinedFunction: ...
        def __init__(
            self,
            *,
            user_defined_function: sift.common.type.v1.user_defined_functions_pb2.UserDefinedFunction | None = ...,
        ) -> None: ...
        def HasField(self, field_name: typing.Literal["user_defined_function", b"user_defined_function"]) -> builtins.bool: ...
        def ClearField(self, field_name: typing.Literal["user_defined_function", b"user_defined_function"]) -> None: ...

    ERROR_FIELD_NUMBER: builtins.int
    SUCCESS_FIELD_NUMBER: builtins.int
    @property
    def error(self) -> global___ValidateUserDefinedFunctionResponse.ErrorValidatingUserDefinedFunctionResult: ...
    @property
    def success(self) -> global___ValidateUserDefinedFunctionResponse.SuccessValidatingUserDefinedFunctionResult: ...
    def __init__(
        self,
        *,
        error: global___ValidateUserDefinedFunctionResponse.ErrorValidatingUserDefinedFunctionResult | None = ...,
        success: global___ValidateUserDefinedFunctionResponse.SuccessValidatingUserDefinedFunctionResult | None = ...,
    ) -> None: ...
    def HasField(self, field_name: typing.Literal["error", b"error", "result", b"result", "success", b"success"]) -> builtins.bool: ...
    def ClearField(self, field_name: typing.Literal["error", b"error", "result", b"result", "success", b"success"]) -> None: ...
    def WhichOneof(self, oneof_group: typing.Literal["result", b"result"]) -> typing.Literal["error", "success"] | None: ...

global___ValidateUserDefinedFunctionResponse = ValidateUserDefinedFunctionResponse

@typing.final
class UpdateUserDefinedFunctionRequest(google.protobuf.message.Message):
    """The request for a call to `UserDefinedFunctionService_UpdateUserDefinedFunction` to update a user defined function. Updating a user
    defined function creates a new version of the user defined function, leaving the previous untouched. If no update is deemed necessary, then the
    the current version is returned. If name is changed then only name will be changed. If archive date is changed then only archive date will be changed.
    To archive user defined function, specify `archived_date` in the `update mask` as well as a non-null value for `archived_date` in the
    `user_defined_function` object. To unarchive a user defined function, specify `archived_date` in the `update mask` and a `null` value for `archived_date`
    in the `user_defined_function` object.
    """

    DESCRIPTOR: google.protobuf.descriptor.Descriptor

    USER_DEFINED_FUNCTION_FIELD_NUMBER: builtins.int
    UPDATE_MASK_FIELD_NUMBER: builtins.int
    @property
    def user_defined_function(self) -> sift.common.type.v1.user_defined_functions_pb2.UserDefinedFunction:
        """The user defined function to update."""

    @property
    def update_mask(self) -> google.protobuf.field_mask_pb2.FieldMask:
        """The list of fields to be updated. The fields available to be updated are `name`, `archived_date`, `description`, `expression`, and `function_inputs`.
         -- `name` can't be updated if the function has ever had any dependencies
         -- `function_inputs` inputs can't be updated if the function has any dependents (functions or calculated channels with a dependency on this function).
         -- `expression` can't be updated if the function has dependents and the expression changes the output type.
        """

    def __init__(
        self,
        *,
        user_defined_function: sift.common.type.v1.user_defined_functions_pb2.UserDefinedFunction | None = ...,
        update_mask: google.protobuf.field_mask_pb2.FieldMask | None = ...,
    ) -> None: ...
    def HasField(self, field_name: typing.Literal["update_mask", b"update_mask", "user_defined_function", b"user_defined_function"]) -> builtins.bool: ...
    def ClearField(self, field_name: typing.Literal["update_mask", b"update_mask", "user_defined_function", b"user_defined_function"]) -> None: ...

global___UpdateUserDefinedFunctionRequest = UpdateUserDefinedFunctionRequest

@typing.final
class UpdateUserDefinedFunctionResponse(google.protobuf.message.Message):
    DESCRIPTOR: google.protobuf.descriptor.Descriptor

    USER_DEFINED_FUNCTION_FIELD_NUMBER: builtins.int
    @property
    def user_defined_function(self) -> sift.common.type.v1.user_defined_functions_pb2.UserDefinedFunction: ...
    def __init__(
        self,
        *,
        user_defined_function: sift.common.type.v1.user_defined_functions_pb2.UserDefinedFunction | None = ...,
    ) -> None: ...
    def HasField(self, field_name: typing.Literal["user_defined_function", b"user_defined_function"]) -> builtins.bool: ...
    def ClearField(self, field_name: typing.Literal["user_defined_function", b"user_defined_function"]) -> None: ...

global___UpdateUserDefinedFunctionResponse = UpdateUserDefinedFunctionResponse

@typing.final
class CheckUpdatableFieldsRequest(google.protobuf.message.Message):
    DESCRIPTOR: google.protobuf.descriptor.Descriptor

    USER_DEFINED_FUNCTION_ID_FIELD_NUMBER: builtins.int
    UPDATE_MASK_FIELD_NUMBER: builtins.int
    user_defined_function_id: builtins.str
    @property
    def update_mask(self) -> google.protobuf.field_mask_pb2.FieldMask: ...
    def __init__(
        self,
        *,
        user_defined_function_id: builtins.str = ...,
        update_mask: google.protobuf.field_mask_pb2.FieldMask | None = ...,
    ) -> None: ...
    def HasField(self, field_name: typing.Literal["update_mask", b"update_mask"]) -> builtins.bool: ...
    def ClearField(self, field_name: typing.Literal["update_mask", b"update_mask", "user_defined_function_id", b"user_defined_function_id"]) -> None: ...

global___CheckUpdatableFieldsRequest = CheckUpdatableFieldsRequest

@typing.final
class CheckUpdatableFieldsResponse(google.protobuf.message.Message):
    DESCRIPTOR: google.protobuf.descriptor.Descriptor

    @typing.final
    class DisallowedFieldsEntry(google.protobuf.message.Message):
        DESCRIPTOR: google.protobuf.descriptor.Descriptor

        KEY_FIELD_NUMBER: builtins.int
        VALUE_FIELD_NUMBER: builtins.int
        key: builtins.str
        value: builtins.str
        def __init__(
            self,
            *,
            key: builtins.str = ...,
            value: builtins.str = ...,
        ) -> None: ...
        def ClearField(self, field_name: typing.Literal["key", b"key", "value", b"value"]) -> None: ...

    ALLOWED_FIELDS_FIELD_NUMBER: builtins.int
    DISALLOWED_FIELDS_FIELD_NUMBER: builtins.int
    @property
    def allowed_fields(self) -> google.protobuf.field_mask_pb2.FieldMask: ...
    @property
    def disallowed_fields(self) -> google.protobuf.internal.containers.ScalarMap[builtins.str, builtins.str]:
        """Disallowed Fields will contain the update fields as the key and the reason for rejection as the value."""

    def __init__(
        self,
        *,
        allowed_fields: google.protobuf.field_mask_pb2.FieldMask | None = ...,
        disallowed_fields: collections.abc.Mapping[builtins.str, builtins.str] | None = ...,
    ) -> None: ...
    def HasField(self, field_name: typing.Literal["allowed_fields", b"allowed_fields"]) -> builtins.bool: ...
    def ClearField(self, field_name: typing.Literal["allowed_fields", b"allowed_fields", "disallowed_fields", b"disallowed_fields"]) -> None: ...

global___CheckUpdatableFieldsResponse = CheckUpdatableFieldsResponse

@typing.final
class ListUserDefinedFunctionsRequest(google.protobuf.message.Message):
    """The request for a call to `UserDefinedFunctionService_ListUserDefinedFunctions` to retrieve lateset versions of user defined functions."""

    DESCRIPTOR: google.protobuf.descriptor.Descriptor

    PAGE_SIZE_FIELD_NUMBER: builtins.int
    PAGE_TOKEN_FIELD_NUMBER: builtins.int
    FILTER_FIELD_NUMBER: builtins.int
    ORDER_BY_FIELD_NUMBER: builtins.int
    page_size: builtins.int
    """The maximum number of user defined functions to return. The service may return fewer than this value.
    If unspecified, at most 50 user defined functions will be returned. The maximum value is 1000; values above
    1000 will be coerced to 1000. Optional.
    """
    page_token: builtins.str
    """A page token, received from a previous `ListUserDefinedFunctions` call.
    Provide this to retrieve the subsequent page.
    When paginating, all other parameters provided to `ListUserDefinedFunctions` must match
    the call that provided the page token. Optional.
    """
    filter: builtins.str
    """A [Common Expression Language (CEL)](https://github.com/google/cel-spec) filter string.
    There are currently no available fields.
    For further information about how to use CELs, please refer to [this guide](https://github.com/google/cel-spec/blob/master/doc/langdef.md#standard-definitions).
    """
    order_by: builtins.str
    """How to order the retrieved user defined functions. Formatted as a comma-separated string i.e. "FIELD_NAME[ desc],...".
    Available fields to order_by are `created_date`, `modified_date`, and `name`.
    If left empty, items are ordered by `name` in ascending order (alphabetical).
    For more information about the format of this field, read [this](https://google.aip.dev/132#ordering)
    Example: "created_date desc,modified_date".
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

global___ListUserDefinedFunctionsRequest = ListUserDefinedFunctionsRequest

@typing.final
class ListUserDefinedFunctionsResponse(google.protobuf.message.Message):
    """The response of a call to `UserDefinedFunctionService_ListUserDefinedFunctions`."""

    DESCRIPTOR: google.protobuf.descriptor.Descriptor

    USER_DEFINED_FUNCTIONS_FIELD_NUMBER: builtins.int
    NEXT_PAGE_TOKEN_FIELD_NUMBER: builtins.int
    next_page_token: builtins.str
    @property
    def user_defined_functions(self) -> google.protobuf.internal.containers.RepeatedCompositeFieldContainer[sift.common.type.v1.user_defined_functions_pb2.UserDefinedFunction]: ...
    def __init__(
        self,
        *,
        user_defined_functions: collections.abc.Iterable[sift.common.type.v1.user_defined_functions_pb2.UserDefinedFunction] | None = ...,
        next_page_token: builtins.str = ...,
    ) -> None: ...
    def ClearField(self, field_name: typing.Literal["next_page_token", b"next_page_token", "user_defined_functions", b"user_defined_functions"]) -> None: ...

global___ListUserDefinedFunctionsResponse = ListUserDefinedFunctionsResponse

@typing.final
class ListUserDefinedFunctionVersionsRequest(google.protobuf.message.Message):
    """The request for a call to `UserDefinedFunctionService_ListUserDefinedFunctionVersions` to retrieve versions of user defined function.
    If `user_defined_function_id` is provided then `name` is ignored.
    """

    DESCRIPTOR: google.protobuf.descriptor.Descriptor

    USER_DEFINED_FUNCTION_ID_FIELD_NUMBER: builtins.int
    NAME_FIELD_NUMBER: builtins.int
    PAGE_SIZE_FIELD_NUMBER: builtins.int
    PAGE_TOKEN_FIELD_NUMBER: builtins.int
    FILTER_FIELD_NUMBER: builtins.int
    ORDER_BY_FIELD_NUMBER: builtins.int
    user_defined_function_id: builtins.str
    name: builtins.str
    page_size: builtins.int
    """The maximum number of user defined function versions to return. The service may return fewer than this value.
    If unspecified, at most 50 user defined function versions will be returned. The maximum value is 1000; values above
    1000 will be coerced to 1000. Optional.
    """
    page_token: builtins.str
    """A page token, received from a previous `ListUserDefinedFunctions` call.
    Provide this to retrieve the subsequent page.
    When paginating, all other parameters provided to `ListUserDefinedFunctions` must match
    the call that provided the page token. Optional.
    """
    filter: builtins.str
    """A [Common Expression Language (CEL)](https://github.com/google/cel-spec) filter string.
    There are currently no available fields.
    For further information about how to use CELs, please refer to [this guide](https://github.com/google/cel-spec/blob/master/doc/langdef.md#standard-definitions).
    """
    order_by: builtins.str
    """How to order the retrieved user defined functions. Formatted as a comma-separated string i.e. "FIELD_NAME[ desc],...".
    Available fields to order_by are `created_date`, `modified_date`, `name`, and `version`.
    If left empty, items are ordered by `name` in ascending order (oldest-first).
    For more information about the format of this field, read [this](https://google.aip.dev/132#ordering)
    Example: "version desc,name".
    """
    def __init__(
        self,
        *,
        user_defined_function_id: builtins.str = ...,
        name: builtins.str = ...,
        page_size: builtins.int = ...,
        page_token: builtins.str = ...,
        filter: builtins.str = ...,
        order_by: builtins.str = ...,
    ) -> None: ...
    def ClearField(self, field_name: typing.Literal["filter", b"filter", "name", b"name", "order_by", b"order_by", "page_size", b"page_size", "page_token", b"page_token", "user_defined_function_id", b"user_defined_function_id"]) -> None: ...

global___ListUserDefinedFunctionVersionsRequest = ListUserDefinedFunctionVersionsRequest

@typing.final
class ListUserDefinedFunctionVersionsResponse(google.protobuf.message.Message):
    """The response of a call to `UserDefinedFunctionService_ListUserDefinedFunctionVersions`."""

    DESCRIPTOR: google.protobuf.descriptor.Descriptor

    USER_DEFINED_FUNCTIONS_FIELD_NUMBER: builtins.int
    NEXT_PAGE_TOKEN_FIELD_NUMBER: builtins.int
    next_page_token: builtins.str
    @property
    def user_defined_functions(self) -> google.protobuf.internal.containers.RepeatedCompositeFieldContainer[sift.common.type.v1.user_defined_functions_pb2.UserDefinedFunction]: ...
    def __init__(
        self,
        *,
        user_defined_functions: collections.abc.Iterable[sift.common.type.v1.user_defined_functions_pb2.UserDefinedFunction] | None = ...,
        next_page_token: builtins.str = ...,
    ) -> None: ...
    def ClearField(self, field_name: typing.Literal["next_page_token", b"next_page_token", "user_defined_functions", b"user_defined_functions"]) -> None: ...

global___ListUserDefinedFunctionVersionsResponse = ListUserDefinedFunctionVersionsResponse
