"""
@generated by mypy-protobuf.  Do not edit manually!
isort:skip_file
"""

import builtins
import collections.abc
import google.protobuf.descriptor
import google.protobuf.internal.containers
import google.protobuf.internal.enum_type_wrapper
import google.protobuf.message
import google.protobuf.timestamp_pb2
import sift.common.type.v1.channel_config_pb2
import sys
import typing

if sys.version_info >= (3, 10):
    import typing as typing_extensions
else:
    import typing_extensions

DESCRIPTOR: google.protobuf.descriptor.FileDescriptor

class _TimeFormat:
    ValueType = typing.NewType("ValueType", builtins.int)
    V: typing_extensions.TypeAlias = ValueType

class _TimeFormatEnumTypeWrapper(google.protobuf.internal.enum_type_wrapper._EnumTypeWrapper[_TimeFormat.ValueType], builtins.type):
    DESCRIPTOR: google.protobuf.descriptor.EnumDescriptor
    TIME_FORMAT_UNSPECIFIED: _TimeFormat.ValueType  # 0
    TIME_FORMAT_RELATIVE_NANOSECONDS: _TimeFormat.ValueType  # 1
    TIME_FORMAT_RELATIVE_MICROSECONDS: _TimeFormat.ValueType  # 2
    TIME_FORMAT_RELATIVE_MILLISECONDS: _TimeFormat.ValueType  # 3
    TIME_FORMAT_RELATIVE_SECONDS: _TimeFormat.ValueType  # 4
    TIME_FORMAT_RELATIVE_MINUTES: _TimeFormat.ValueType  # 5
    TIME_FORMAT_RELATIVE_HOURS: _TimeFormat.ValueType  # 6
    TIME_FORMAT_ABSOLUTE_RFC3339: _TimeFormat.ValueType  # 10
    TIME_FORMAT_ABSOLUTE_DATETIME: _TimeFormat.ValueType  # 11
    TIME_FORMAT_ABSOLUTE_UNIX_SECONDS: _TimeFormat.ValueType  # 12
    TIME_FORMAT_ABSOLUTE_UNIX_MILLISECONDS: _TimeFormat.ValueType  # 13
    TIME_FORMAT_ABSOLUTE_UNIX_MICROSECONDS: _TimeFormat.ValueType  # 14
    TIME_FORMAT_ABSOLUTE_UNIX_NANOSECONDS: _TimeFormat.ValueType  # 15

class TimeFormat(_TimeFormat, metaclass=_TimeFormatEnumTypeWrapper): ...

TIME_FORMAT_UNSPECIFIED: TimeFormat.ValueType  # 0
TIME_FORMAT_RELATIVE_NANOSECONDS: TimeFormat.ValueType  # 1
TIME_FORMAT_RELATIVE_MICROSECONDS: TimeFormat.ValueType  # 2
TIME_FORMAT_RELATIVE_MILLISECONDS: TimeFormat.ValueType  # 3
TIME_FORMAT_RELATIVE_SECONDS: TimeFormat.ValueType  # 4
TIME_FORMAT_RELATIVE_MINUTES: TimeFormat.ValueType  # 5
TIME_FORMAT_RELATIVE_HOURS: TimeFormat.ValueType  # 6
TIME_FORMAT_ABSOLUTE_RFC3339: TimeFormat.ValueType  # 10
TIME_FORMAT_ABSOLUTE_DATETIME: TimeFormat.ValueType  # 11
TIME_FORMAT_ABSOLUTE_UNIX_SECONDS: TimeFormat.ValueType  # 12
TIME_FORMAT_ABSOLUTE_UNIX_MILLISECONDS: TimeFormat.ValueType  # 13
TIME_FORMAT_ABSOLUTE_UNIX_MICROSECONDS: TimeFormat.ValueType  # 14
TIME_FORMAT_ABSOLUTE_UNIX_NANOSECONDS: TimeFormat.ValueType  # 15
global___TimeFormat = TimeFormat

class _DataImportStatus:
    ValueType = typing.NewType("ValueType", builtins.int)
    V: typing_extensions.TypeAlias = ValueType

class _DataImportStatusEnumTypeWrapper(google.protobuf.internal.enum_type_wrapper._EnumTypeWrapper[_DataImportStatus.ValueType], builtins.type):
    DESCRIPTOR: google.protobuf.descriptor.EnumDescriptor
    DATA_IMPORT_STATUS_UNSPECIFIED: _DataImportStatus.ValueType  # 0
    DATA_IMPORT_STATUS_PENDING: _DataImportStatus.ValueType  # 1
    DATA_IMPORT_STATUS_IN_PROGRESS: _DataImportStatus.ValueType  # 2
    DATA_IMPORT_STATUS_SUCCEEDED: _DataImportStatus.ValueType  # 3
    DATA_IMPORT_STATUS_FAILED: _DataImportStatus.ValueType  # 4

class DataImportStatus(_DataImportStatus, metaclass=_DataImportStatusEnumTypeWrapper): ...

DATA_IMPORT_STATUS_UNSPECIFIED: DataImportStatus.ValueType  # 0
DATA_IMPORT_STATUS_PENDING: DataImportStatus.ValueType  # 1
DATA_IMPORT_STATUS_IN_PROGRESS: DataImportStatus.ValueType  # 2
DATA_IMPORT_STATUS_SUCCEEDED: DataImportStatus.ValueType  # 3
DATA_IMPORT_STATUS_FAILED: DataImportStatus.ValueType  # 4
global___DataImportStatus = DataImportStatus

@typing.final
class CreateDataImportFromUrlRequest(google.protobuf.message.Message):
    DESCRIPTOR: google.protobuf.descriptor.Descriptor

    URL_FIELD_NUMBER: builtins.int
    CSV_CONFIG_FIELD_NUMBER: builtins.int
    CH10_CONFIG_FIELD_NUMBER: builtins.int
    TDMS_CONFIG_FIELD_NUMBER: builtins.int
    url: builtins.str
    """The url to import. HTTP and S3 urls are supported.
    If you need to import non-public S3 objects, please contact Sift to set that up.
    """
    @property
    def csv_config(self) -> global___CsvConfig: ...
    @property
    def ch10_config(self) -> global___Ch10Config: ...
    @property
    def tdms_config(self) -> global___TDMSConfig: ...
    def __init__(
        self,
        *,
        url: builtins.str = ...,
        csv_config: global___CsvConfig | None = ...,
        ch10_config: global___Ch10Config | None = ...,
        tdms_config: global___TDMSConfig | None = ...,
    ) -> None: ...
    def HasField(self, field_name: typing.Literal["ch10_config", b"ch10_config", "csv_config", b"csv_config", "tdms_config", b"tdms_config"]) -> builtins.bool: ...
    def ClearField(self, field_name: typing.Literal["ch10_config", b"ch10_config", "csv_config", b"csv_config", "tdms_config", b"tdms_config", "url", b"url"]) -> None: ...

global___CreateDataImportFromUrlRequest = CreateDataImportFromUrlRequest

@typing.final
class CreateDataImportFromUrlResponse(google.protobuf.message.Message):
    DESCRIPTOR: google.protobuf.descriptor.Descriptor

    DATA_IMPORT_ID_FIELD_NUMBER: builtins.int
    data_import_id: builtins.str
    def __init__(
        self,
        *,
        data_import_id: builtins.str = ...,
    ) -> None: ...
    def ClearField(self, field_name: typing.Literal["data_import_id", b"data_import_id"]) -> None: ...

global___CreateDataImportFromUrlResponse = CreateDataImportFromUrlResponse

@typing.final
class GetDataImportRequest(google.protobuf.message.Message):
    DESCRIPTOR: google.protobuf.descriptor.Descriptor

    DATA_IMPORT_ID_FIELD_NUMBER: builtins.int
    data_import_id: builtins.str
    def __init__(
        self,
        *,
        data_import_id: builtins.str = ...,
    ) -> None: ...
    def ClearField(self, field_name: typing.Literal["data_import_id", b"data_import_id"]) -> None: ...

global___GetDataImportRequest = GetDataImportRequest

@typing.final
class GetDataImportResponse(google.protobuf.message.Message):
    DESCRIPTOR: google.protobuf.descriptor.Descriptor

    DATA_IMPORT_FIELD_NUMBER: builtins.int
    @property
    def data_import(self) -> global___DataImport: ...
    def __init__(
        self,
        *,
        data_import: global___DataImport | None = ...,
    ) -> None: ...
    def HasField(self, field_name: typing.Literal["data_import", b"data_import"]) -> builtins.bool: ...
    def ClearField(self, field_name: typing.Literal["data_import", b"data_import"]) -> None: ...

global___GetDataImportResponse = GetDataImportResponse

@typing.final
class CreateDataImportFromUploadRequest(google.protobuf.message.Message):
    DESCRIPTOR: google.protobuf.descriptor.Descriptor

    CSV_CONFIG_FIELD_NUMBER: builtins.int
    CH10_CONFIG_FIELD_NUMBER: builtins.int
    TDMS_CONFIG_FIELD_NUMBER: builtins.int
    @property
    def csv_config(self) -> global___CsvConfig: ...
    @property
    def ch10_config(self) -> global___Ch10Config: ...
    @property
    def tdms_config(self) -> global___TDMSConfig: ...
    def __init__(
        self,
        *,
        csv_config: global___CsvConfig | None = ...,
        ch10_config: global___Ch10Config | None = ...,
        tdms_config: global___TDMSConfig | None = ...,
    ) -> None: ...
    def HasField(self, field_name: typing.Literal["ch10_config", b"ch10_config", "csv_config", b"csv_config", "tdms_config", b"tdms_config"]) -> builtins.bool: ...
    def ClearField(self, field_name: typing.Literal["ch10_config", b"ch10_config", "csv_config", b"csv_config", "tdms_config", b"tdms_config"]) -> None: ...

global___CreateDataImportFromUploadRequest = CreateDataImportFromUploadRequest

@typing.final
class CreateDataImportFromUploadResponse(google.protobuf.message.Message):
    DESCRIPTOR: google.protobuf.descriptor.Descriptor

    UPLOAD_URL_FIELD_NUMBER: builtins.int
    DATA_IMPORT_ID_FIELD_NUMBER: builtins.int
    upload_url: builtins.str
    data_import_id: builtins.str
    def __init__(
        self,
        *,
        upload_url: builtins.str = ...,
        data_import_id: builtins.str = ...,
    ) -> None: ...
    def ClearField(self, field_name: typing.Literal["data_import_id", b"data_import_id", "upload_url", b"upload_url"]) -> None: ...

global___CreateDataImportFromUploadResponse = CreateDataImportFromUploadResponse

@typing.final
class CsvConfig(google.protobuf.message.Message):
    DESCRIPTOR: google.protobuf.descriptor.Descriptor

    @typing.final
    class DataColumnsEntry(google.protobuf.message.Message):
        DESCRIPTOR: google.protobuf.descriptor.Descriptor

        KEY_FIELD_NUMBER: builtins.int
        VALUE_FIELD_NUMBER: builtins.int
        key: builtins.int
        @property
        def value(self) -> sift.common.type.v1.channel_config_pb2.ChannelConfig: ...
        def __init__(
            self,
            *,
            key: builtins.int = ...,
            value: sift.common.type.v1.channel_config_pb2.ChannelConfig | None = ...,
        ) -> None: ...
        def HasField(self, field_name: typing.Literal["value", b"value"]) -> builtins.bool: ...
        def ClearField(self, field_name: typing.Literal["key", b"key", "value", b"value"]) -> None: ...

    ASSET_NAME_FIELD_NUMBER: builtins.int
    RUN_NAME_FIELD_NUMBER: builtins.int
    RUN_ID_FIELD_NUMBER: builtins.int
    FIRST_DATA_ROW_FIELD_NUMBER: builtins.int
    TIME_COLUMN_FIELD_NUMBER: builtins.int
    DATA_COLUMNS_FIELD_NUMBER: builtins.int
    asset_name: builtins.str
    run_name: builtins.str
    run_id: builtins.str
    """The id of the run to add this data to. If set, `run_name` is ignored."""
    first_data_row: builtins.int
    """The first row to start reading as data. Can be used to skip header rows.
    The first row in the file is 1.
    """
    @property
    def time_column(self) -> global___CsvTimeColumn: ...
    @property
    def data_columns(self) -> google.protobuf.internal.containers.MessageMap[builtins.int, sift.common.type.v1.channel_config_pb2.ChannelConfig]:
        """A map from column number (1-indexed) to the channel configuration for that column."""

    def __init__(
        self,
        *,
        asset_name: builtins.str = ...,
        run_name: builtins.str = ...,
        run_id: builtins.str = ...,
        first_data_row: builtins.int = ...,
        time_column: global___CsvTimeColumn | None = ...,
        data_columns: collections.abc.Mapping[builtins.int, sift.common.type.v1.channel_config_pb2.ChannelConfig] | None = ...,
    ) -> None: ...
    def HasField(self, field_name: typing.Literal["_time_column", b"_time_column", "time_column", b"time_column"]) -> builtins.bool: ...
    def ClearField(self, field_name: typing.Literal["_time_column", b"_time_column", "asset_name", b"asset_name", "data_columns", b"data_columns", "first_data_row", b"first_data_row", "run_id", b"run_id", "run_name", b"run_name", "time_column", b"time_column"]) -> None: ...
    def WhichOneof(self, oneof_group: typing.Literal["_time_column", b"_time_column"]) -> typing.Literal["time_column"] | None: ...

global___CsvConfig = CsvConfig

@typing.final
class CsvTimeColumn(google.protobuf.message.Message):
    DESCRIPTOR: google.protobuf.descriptor.Descriptor

    COLUMN_NUMBER_FIELD_NUMBER: builtins.int
    FORMAT_FIELD_NUMBER: builtins.int
    RELATIVE_START_TIME_FIELD_NUMBER: builtins.int
    column_number: builtins.int
    """The column number (1-indexed) of the time column."""
    format: global___TimeFormat.ValueType
    @property
    def relative_start_time(self) -> google.protobuf.timestamp_pb2.Timestamp: ...
    def __init__(
        self,
        *,
        column_number: builtins.int = ...,
        format: global___TimeFormat.ValueType = ...,
        relative_start_time: google.protobuf.timestamp_pb2.Timestamp | None = ...,
    ) -> None: ...
    def HasField(self, field_name: typing.Literal["_relative_start_time", b"_relative_start_time", "relative_start_time", b"relative_start_time"]) -> builtins.bool: ...
    def ClearField(self, field_name: typing.Literal["_relative_start_time", b"_relative_start_time", "column_number", b"column_number", "format", b"format", "relative_start_time", b"relative_start_time"]) -> None: ...
    def WhichOneof(self, oneof_group: typing.Literal["_relative_start_time", b"_relative_start_time"]) -> typing.Literal["relative_start_time"] | None: ...

global___CsvTimeColumn = CsvTimeColumn

@typing.final
class DetectConfigRequest(google.protobuf.message.Message):
    DESCRIPTOR: google.protobuf.descriptor.Descriptor

    DATA_FIELD_NUMBER: builtins.int
    data: builtins.bytes
    def __init__(
        self,
        *,
        data: builtins.bytes = ...,
    ) -> None: ...
    def ClearField(self, field_name: typing.Literal["data", b"data"]) -> None: ...

global___DetectConfigRequest = DetectConfigRequest

@typing.final
class DetectConfigResponse(google.protobuf.message.Message):
    DESCRIPTOR: google.protobuf.descriptor.Descriptor

    CSV_CONFIG_FIELD_NUMBER: builtins.int
    @property
    def csv_config(self) -> global___CsvConfig: ...
    def __init__(
        self,
        *,
        csv_config: global___CsvConfig | None = ...,
    ) -> None: ...
    def HasField(self, field_name: typing.Literal["csv_config", b"csv_config"]) -> builtins.bool: ...
    def ClearField(self, field_name: typing.Literal["csv_config", b"csv_config"]) -> None: ...

global___DetectConfigResponse = DetectConfigResponse

@typing.final
class Ch10Config(google.protobuf.message.Message):
    DESCRIPTOR: google.protobuf.descriptor.Descriptor

    ASSET_NAME_FIELD_NUMBER: builtins.int
    RUN_NAME_FIELD_NUMBER: builtins.int
    SCALE_VALUES_FIELD_NUMBER: builtins.int
    asset_name: builtins.str
    run_name: builtins.str
    scale_values: builtins.bool
    def __init__(
        self,
        *,
        asset_name: builtins.str = ...,
        run_name: builtins.str = ...,
        scale_values: builtins.bool = ...,
    ) -> None: ...
    def ClearField(self, field_name: typing.Literal["asset_name", b"asset_name", "run_name", b"run_name", "scale_values", b"scale_values"]) -> None: ...

global___Ch10Config = Ch10Config

@typing.final
class TDMSConfig(google.protobuf.message.Message):
    DESCRIPTOR: google.protobuf.descriptor.Descriptor

    ASSET_NAME_FIELD_NUMBER: builtins.int
    RUN_NAME_FIELD_NUMBER: builtins.int
    START_TIME_OVERRIDE_FIELD_NUMBER: builtins.int
    asset_name: builtins.str
    run_name: builtins.str
    @property
    def start_time_override(self) -> google.protobuf.timestamp_pb2.Timestamp:
        """Override the wf_start_time metadata field for all channels.
        Useful if your waveform channels have wf_increment but no wf_start_time (Veristand is guilty of this).
        """

    def __init__(
        self,
        *,
        asset_name: builtins.str = ...,
        run_name: builtins.str = ...,
        start_time_override: google.protobuf.timestamp_pb2.Timestamp | None = ...,
    ) -> None: ...
    def HasField(self, field_name: typing.Literal["start_time_override", b"start_time_override"]) -> builtins.bool: ...
    def ClearField(self, field_name: typing.Literal["asset_name", b"asset_name", "run_name", b"run_name", "start_time_override", b"start_time_override"]) -> None: ...

global___TDMSConfig = TDMSConfig

@typing.final
class DataImport(google.protobuf.message.Message):
    DESCRIPTOR: google.protobuf.descriptor.Descriptor

    DATA_IMPORT_ID_FIELD_NUMBER: builtins.int
    SOURCE_URL_FIELD_NUMBER: builtins.int
    STATUS_FIELD_NUMBER: builtins.int
    ERROR_MESSAGE_FIELD_NUMBER: builtins.int
    CREATED_DATE_FIELD_NUMBER: builtins.int
    MODIFIED_DATE_FIELD_NUMBER: builtins.int
    CSV_CONFIG_FIELD_NUMBER: builtins.int
    CH10_CONFIG_FIELD_NUMBER: builtins.int
    TDMS_CONFIG_FIELD_NUMBER: builtins.int
    data_import_id: builtins.str
    source_url: builtins.str
    status: global___DataImportStatus.ValueType
    error_message: builtins.str
    @property
    def created_date(self) -> google.protobuf.timestamp_pb2.Timestamp: ...
    @property
    def modified_date(self) -> google.protobuf.timestamp_pb2.Timestamp: ...
    @property
    def csv_config(self) -> global___CsvConfig: ...
    @property
    def ch10_config(self) -> global___Ch10Config: ...
    @property
    def tdms_config(self) -> global___TDMSConfig: ...
    def __init__(
        self,
        *,
        data_import_id: builtins.str = ...,
        source_url: builtins.str = ...,
        status: global___DataImportStatus.ValueType = ...,
        error_message: builtins.str = ...,
        created_date: google.protobuf.timestamp_pb2.Timestamp | None = ...,
        modified_date: google.protobuf.timestamp_pb2.Timestamp | None = ...,
        csv_config: global___CsvConfig | None = ...,
        ch10_config: global___Ch10Config | None = ...,
        tdms_config: global___TDMSConfig | None = ...,
    ) -> None: ...
    def HasField(self, field_name: typing.Literal["ch10_config", b"ch10_config", "created_date", b"created_date", "csv_config", b"csv_config", "modified_date", b"modified_date", "tdms_config", b"tdms_config"]) -> builtins.bool: ...
    def ClearField(self, field_name: typing.Literal["ch10_config", b"ch10_config", "created_date", b"created_date", "csv_config", b"csv_config", "data_import_id", b"data_import_id", "error_message", b"error_message", "modified_date", b"modified_date", "source_url", b"source_url", "status", b"status", "tdms_config", b"tdms_config"]) -> None: ...

global___DataImport = DataImport

@typing.final
class ListDataImportsRequest(google.protobuf.message.Message):
    DESCRIPTOR: google.protobuf.descriptor.Descriptor

    PAGE_SIZE_FIELD_NUMBER: builtins.int
    PAGE_TOKEN_FIELD_NUMBER: builtins.int
    FILTER_FIELD_NUMBER: builtins.int
    ORDER_BY_FIELD_NUMBER: builtins.int
    page_size: builtins.int
    """The maximum number of data imports to return. The service may return fewer than this value.
    If unspecified, at most 50 data imports will be returned. The maximum value is 1000; values above
    1000 will be coerced to 1000. Optional.
    """
    page_token: builtins.str
    """A page token, received from a previous `ListDataImports` call.
    Provide this to retrieve the subsequent page.
    When paginating, all other parameters provided to `ListDataImports` must match
    the call that provided the page token. Optional.
    """
    filter: builtins.str
    """A [Common Expression Language (CEL)](https://github.com/google/cel-spec) filter string.
    Available fields to filter by are `data_import_id`, `source_url`, `status`.
    For further information about how to use CELs, please refer to [this guide](https://github.com/google/cel-spec/blob/master/doc/langdef.md#standard-definitions).
    """
    order_by: builtins.str
    """How to order the retrieved data imports. Formatted as a comma-separated string i.e. "FIELD_NAME[ desc],...".
    Available fields to order_by are `created_date` and `modified_date`.
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
        order_by: builtins.str = ...,
    ) -> None: ...
    def ClearField(self, field_name: typing.Literal["filter", b"filter", "order_by", b"order_by", "page_size", b"page_size", "page_token", b"page_token"]) -> None: ...

global___ListDataImportsRequest = ListDataImportsRequest

@typing.final
class ListDataImportsResponse(google.protobuf.message.Message):
    DESCRIPTOR: google.protobuf.descriptor.Descriptor

    DATA_IMPORTS_FIELD_NUMBER: builtins.int
    NEXT_PAGE_TOKEN_FIELD_NUMBER: builtins.int
    next_page_token: builtins.str
    @property
    def data_imports(self) -> google.protobuf.internal.containers.RepeatedCompositeFieldContainer[global___DataImport]: ...
    def __init__(
        self,
        *,
        data_imports: collections.abc.Iterable[global___DataImport] | None = ...,
        next_page_token: builtins.str = ...,
    ) -> None: ...
    def ClearField(self, field_name: typing.Literal["data_imports", b"data_imports", "next_page_token", b"next_page_token"]) -> None: ...

global___ListDataImportsResponse = ListDataImportsResponse

@typing.final
class RetryDataImportRequest(google.protobuf.message.Message):
    DESCRIPTOR: google.protobuf.descriptor.Descriptor

    DATA_IMPORT_ID_FIELD_NUMBER: builtins.int
    data_import_id: builtins.str
    """data_import_id is the id of the data import to retry.
    You can only retry an import that is a "url" based import (created with CreateDataImportFromUrl) and is in a failed state.
    """
    def __init__(
        self,
        *,
        data_import_id: builtins.str = ...,
    ) -> None: ...
    def ClearField(self, field_name: typing.Literal["data_import_id", b"data_import_id"]) -> None: ...

global___RetryDataImportRequest = RetryDataImportRequest

@typing.final
class RetryDataImportResponse(google.protobuf.message.Message):
    DESCRIPTOR: google.protobuf.descriptor.Descriptor

    def __init__(
        self,
    ) -> None: ...

global___RetryDataImportResponse = RetryDataImportResponse
