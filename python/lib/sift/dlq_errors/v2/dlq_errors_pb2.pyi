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
import typing

DESCRIPTOR: google.protobuf.descriptor.FileDescriptor

@typing.final
class ListDlqErrorsRequest(google.protobuf.message.Message):
    DESCRIPTOR: google.protobuf.descriptor.Descriptor

    PAGE_SIZE_FIELD_NUMBER: builtins.int
    PAGE_TOKEN_FIELD_NUMBER: builtins.int
    FILTER_FIELD_NUMBER: builtins.int
    ORDER_BY_FIELD_NUMBER: builtins.int
    page_size: builtins.int
    """The maximum number of errors values to return.
    The service may return fewer than this value.
    If unspecified, at most 50 values will be returned.
    The maximum value is 1000; values above 1000 will be coerced to 1000.
    """
    page_token: builtins.str
    """A page token, received from a previous `ListDlqErrors` call.
    Provide this to retrieve the subsequent page.

    When paginating, all other parameters provided to `ListDlqErrors` must match
    the call that provided the page token.
    """
    filter: builtins.str
    """A Common Expression Language (CEL, https://github.com/google/cel-spec) filter string.
    Available tag variables are:
    - min_timestamp
    - max_timestamp
    - asset_id
    """
    order_by: builtins.str
    """How to order the retrieved campaigns. Formatted as a comma-separated string i.e. "FIELD_NAME[ desc],...".
    Available fields to order_by are `min_timestamp`, `max_timestamp`, and `asset_id`.
    If left empty, items are ordered by `min_timestamp` in ascending order (oldest-first).
    For more information about the format of this field, read [this](https://google.aip.dev/132#ordering)
    Example: "asset_id desc,min_timestamp"
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

global___ListDlqErrorsRequest = ListDlqErrorsRequest

@typing.final
class ErrorSummary(google.protobuf.message.Message):
    DESCRIPTOR: google.protobuf.descriptor.Descriptor

    COUNT_FIELD_NUMBER: builtins.int
    ASSET_NAME_FIELD_NUMBER: builtins.int
    MIN_TIMESTAMP_FIELD_NUMBER: builtins.int
    MAX_TIMESTAMP_FIELD_NUMBER: builtins.int
    DLQ_PARQUET_FILE_ID_FIELD_NUMBER: builtins.int
    CREATED_DATE_FIELD_NUMBER: builtins.int
    MODIFIED_DATE_FIELD_NUMBER: builtins.int
    count: builtins.int
    asset_name: builtins.str
    dlq_parquet_file_id: builtins.str
    @property
    def min_timestamp(self) -> google.protobuf.timestamp_pb2.Timestamp: ...
    @property
    def max_timestamp(self) -> google.protobuf.timestamp_pb2.Timestamp: ...
    @property
    def created_date(self) -> google.protobuf.timestamp_pb2.Timestamp: ...
    @property
    def modified_date(self) -> google.protobuf.timestamp_pb2.Timestamp: ...
    def __init__(
        self,
        *,
        count: builtins.int = ...,
        asset_name: builtins.str | None = ...,
        min_timestamp: google.protobuf.timestamp_pb2.Timestamp | None = ...,
        max_timestamp: google.protobuf.timestamp_pb2.Timestamp | None = ...,
        dlq_parquet_file_id: builtins.str = ...,
        created_date: google.protobuf.timestamp_pb2.Timestamp | None = ...,
        modified_date: google.protobuf.timestamp_pb2.Timestamp | None = ...,
    ) -> None: ...
    def HasField(self, field_name: typing.Literal["_asset_name", b"_asset_name", "asset_name", b"asset_name", "created_date", b"created_date", "max_timestamp", b"max_timestamp", "min_timestamp", b"min_timestamp", "modified_date", b"modified_date"]) -> builtins.bool: ...
    def ClearField(self, field_name: typing.Literal["_asset_name", b"_asset_name", "asset_name", b"asset_name", "count", b"count", "created_date", b"created_date", "dlq_parquet_file_id", b"dlq_parquet_file_id", "max_timestamp", b"max_timestamp", "min_timestamp", b"min_timestamp", "modified_date", b"modified_date"]) -> None: ...
    def WhichOneof(self, oneof_group: typing.Literal["_asset_name", b"_asset_name"]) -> typing.Literal["asset_name"] | None: ...

global___ErrorSummary = ErrorSummary

@typing.final
class ListDlqErrorsResponse(google.protobuf.message.Message):
    DESCRIPTOR: google.protobuf.descriptor.Descriptor

    ERROR_SUMMARIES_FIELD_NUMBER: builtins.int
    NEXT_PAGE_TOKEN_FIELD_NUMBER: builtins.int
    next_page_token: builtins.str
    @property
    def error_summaries(self) -> google.protobuf.internal.containers.RepeatedCompositeFieldContainer[global___ErrorSummary]: ...
    def __init__(
        self,
        *,
        error_summaries: collections.abc.Iterable[global___ErrorSummary] | None = ...,
        next_page_token: builtins.str = ...,
    ) -> None: ...
    def ClearField(self, field_name: typing.Literal["error_summaries", b"error_summaries", "next_page_token", b"next_page_token"]) -> None: ...

global___ListDlqErrorsResponse = ListDlqErrorsResponse
