"""
@generated by mypy-protobuf.  Do not edit manually!
isort:skip_file
"""

import builtins
import collections.abc
import google.protobuf.any_pb2
import google.protobuf.descriptor
import google.protobuf.internal.containers
import google.protobuf.message
import google.protobuf.timestamp_pb2
import sift.calculated_channels.v1.calculated_channels_pb2
import sift.common.type.v1.channel_bit_field_element_pb2
import sift.common.type.v1.channel_data_type_pb2
import sift.common.type.v1.channel_enum_type_pb2
import typing

DESCRIPTOR: google.protobuf.descriptor.FileDescriptor

@typing.final
class GetDataRequest(google.protobuf.message.Message):
    DESCRIPTOR: google.protobuf.descriptor.Descriptor

    QUERIES_FIELD_NUMBER: builtins.int
    START_TIME_FIELD_NUMBER: builtins.int
    END_TIME_FIELD_NUMBER: builtins.int
    SAMPLE_MS_FIELD_NUMBER: builtins.int
    PAGE_SIZE_FIELD_NUMBER: builtins.int
    PAGE_TOKEN_FIELD_NUMBER: builtins.int
    sample_ms: builtins.int
    """The rate to sample the returned data at. The data is sampled using [LTTB](https://github.com/sveinn-steinarsson/flot-downsample)
    which will return one point approximately every sample_ms milliseconds that retains the shape of the raw data.
    Sampling is only supported for numeric data types, if sample_ms is provided for non-numeric data, it will be
    ignored and the full-fidelity data will be returned.
    """
    page_size: builtins.int
    """The maximum number of channel values to return.
    The service may return fewer than this value.
    If unspecified, at most 10,000 values will be returned.
    The maximum value is 100,000; values above 100,000 will be coerced to 100,000.
    For variable data types (i.e. string channels), at most page_size elements
    will be read, or 1MB, whichever occurs first.
    """
    page_token: builtins.str
    """A page token, received from a previous `GetData` call.
    Provide this to retrieve the subsequent page.
    When paginating, all other parameters provided to `GetData` must match
    the call that provided the page token.
    """
    @property
    def queries(self) -> google.protobuf.internal.containers.RepeatedCompositeFieldContainer[global___Query]: ...
    @property
    def start_time(self) -> google.protobuf.timestamp_pb2.Timestamp:
        """Required. The starting timestamp of the data to retrieve. This is an inclusive bound."""

    @property
    def end_time(self) -> google.protobuf.timestamp_pb2.Timestamp:
        """Required. The end timestamp of the data to retrieve. This is an exclusive bound."""

    def __init__(
        self,
        *,
        queries: collections.abc.Iterable[global___Query] | None = ...,
        start_time: google.protobuf.timestamp_pb2.Timestamp | None = ...,
        end_time: google.protobuf.timestamp_pb2.Timestamp | None = ...,
        sample_ms: builtins.int = ...,
        page_size: builtins.int = ...,
        page_token: builtins.str = ...,
    ) -> None: ...
    def HasField(self, field_name: typing.Literal["end_time", b"end_time", "start_time", b"start_time"]) -> builtins.bool: ...
    def ClearField(self, field_name: typing.Literal["end_time", b"end_time", "page_size", b"page_size", "page_token", b"page_token", "queries", b"queries", "sample_ms", b"sample_ms", "start_time", b"start_time"]) -> None: ...

global___GetDataRequest = GetDataRequest

@typing.final
class Query(google.protobuf.message.Message):
    DESCRIPTOR: google.protobuf.descriptor.Descriptor

    CHANNEL_FIELD_NUMBER: builtins.int
    CALCULATED_CHANNEL_FIELD_NUMBER: builtins.int
    @property
    def channel(self) -> global___ChannelQuery: ...
    @property
    def calculated_channel(self) -> global___CalculatedChannelQuery: ...
    def __init__(
        self,
        *,
        channel: global___ChannelQuery | None = ...,
        calculated_channel: global___CalculatedChannelQuery | None = ...,
    ) -> None: ...
    def HasField(self, field_name: typing.Literal["calculated_channel", b"calculated_channel", "channel", b"channel", "query", b"query"]) -> builtins.bool: ...
    def ClearField(self, field_name: typing.Literal["calculated_channel", b"calculated_channel", "channel", b"channel", "query", b"query"]) -> None: ...
    def WhichOneof(self, oneof_group: typing.Literal["query", b"query"]) -> typing.Literal["channel", "calculated_channel"] | None: ...

global___Query = Query

@typing.final
class ChannelQuery(google.protobuf.message.Message):
    DESCRIPTOR: google.protobuf.descriptor.Descriptor

    CHANNEL_ID_FIELD_NUMBER: builtins.int
    RUN_ID_FIELD_NUMBER: builtins.int
    channel_id: builtins.str
    """channel_id is the uuid of the channel."""
    run_id: builtins.str
    """Optional.
    If set, only data associated with the specified run is returned.
    If set to the empty string, only non-run data is returned.
    If unset, all run / non-run data is returned.
    """
    def __init__(
        self,
        *,
        channel_id: builtins.str = ...,
        run_id: builtins.str | None = ...,
    ) -> None: ...
    def HasField(self, field_name: typing.Literal["_run_id", b"_run_id", "run_id", b"run_id"]) -> builtins.bool: ...
    def ClearField(self, field_name: typing.Literal["_run_id", b"_run_id", "channel_id", b"channel_id", "run_id", b"run_id"]) -> None: ...
    def WhichOneof(self, oneof_group: typing.Literal["_run_id", b"_run_id"]) -> typing.Literal["run_id"] | None: ...

global___ChannelQuery = ChannelQuery

@typing.final
class CalculatedChannelQuery(google.protobuf.message.Message):
    DESCRIPTOR: google.protobuf.descriptor.Descriptor

    CHANNEL_KEY_FIELD_NUMBER: builtins.int
    EXPRESSION_FIELD_NUMBER: builtins.int
    RUN_ID_FIELD_NUMBER: builtins.int
    MODE_FIELD_NUMBER: builtins.int
    channel_key: builtins.str
    run_id: builtins.str
    """Optional.
    If set, only data for the specified run is returned
    If set to the empty string, only non-run data is returned.
    If unset, all run / non-run data is returned.
    """
    mode: sift.calculated_channels.v1.calculated_channels_pb2.ExpressionMode.ValueType
    """Optional. If unset, will default to EXPRESSION_MODE_CALCULATED_CHANNELS."""
    @property
    def expression(self) -> sift.calculated_channels.v1.calculated_channels_pb2.ExpressionRequest: ...
    def __init__(
        self,
        *,
        channel_key: builtins.str = ...,
        expression: sift.calculated_channels.v1.calculated_channels_pb2.ExpressionRequest | None = ...,
        run_id: builtins.str | None = ...,
        mode: sift.calculated_channels.v1.calculated_channels_pb2.ExpressionMode.ValueType | None = ...,
    ) -> None: ...
    def HasField(self, field_name: typing.Literal["_mode", b"_mode", "_run_id", b"_run_id", "expression", b"expression", "mode", b"mode", "run_id", b"run_id"]) -> builtins.bool: ...
    def ClearField(self, field_name: typing.Literal["_mode", b"_mode", "_run_id", b"_run_id", "channel_key", b"channel_key", "expression", b"expression", "mode", b"mode", "run_id", b"run_id"]) -> None: ...
    @typing.overload
    def WhichOneof(self, oneof_group: typing.Literal["_mode", b"_mode"]) -> typing.Literal["mode"] | None: ...
    @typing.overload
    def WhichOneof(self, oneof_group: typing.Literal["_run_id", b"_run_id"]) -> typing.Literal["run_id"] | None: ...

global___CalculatedChannelQuery = CalculatedChannelQuery

@typing.final
class GetDataResponse(google.protobuf.message.Message):
    DESCRIPTOR: google.protobuf.descriptor.Descriptor

    NEXT_PAGE_TOKEN_FIELD_NUMBER: builtins.int
    DATA_FIELD_NUMBER: builtins.int
    next_page_token: builtins.str
    @property
    def data(self) -> google.protobuf.internal.containers.RepeatedCompositeFieldContainer[google.protobuf.any_pb2.Any]:
        """data contains the result of the supplied queries.
        Be aware that each query can generate multiple data responses.
        For example, if run_id is omitted from a ChannelQuery, the query returns
        data for all runs containing that channel. Channel data for each run is
        returned in a separate data object.
        Possible message types:
          sift.data.v2.DoubleValues
          sift.data.v2.FloatValues
          sift.data.v2.StringValues
          sift.data.v2.EnumValues
          sift.data.v2.BitFieldValues
          sift.data.v2.BoolValues
          sift.data.v2.Int32Values
          sift.data.v2.Int64Values
          sift.data.v2.Uint32Values
          sift.data.v2.Uint64Values
        """

    def __init__(
        self,
        *,
        next_page_token: builtins.str = ...,
        data: collections.abc.Iterable[google.protobuf.any_pb2.Any] | None = ...,
    ) -> None: ...
    def ClearField(self, field_name: typing.Literal["data", b"data", "next_page_token", b"next_page_token"]) -> None: ...

global___GetDataResponse = GetDataResponse

@typing.final
class Metadata(google.protobuf.message.Message):
    DESCRIPTOR: google.protobuf.descriptor.Descriptor

    @typing.final
    class Asset(google.protobuf.message.Message):
        DESCRIPTOR: google.protobuf.descriptor.Descriptor

        ASSET_ID_FIELD_NUMBER: builtins.int
        NAME_FIELD_NUMBER: builtins.int
        asset_id: builtins.str
        name: builtins.str
        def __init__(
            self,
            *,
            asset_id: builtins.str = ...,
            name: builtins.str = ...,
        ) -> None: ...
        def ClearField(self, field_name: typing.Literal["asset_id", b"asset_id", "name", b"name"]) -> None: ...

    @typing.final
    class Run(google.protobuf.message.Message):
        DESCRIPTOR: google.protobuf.descriptor.Descriptor

        RUN_ID_FIELD_NUMBER: builtins.int
        NAME_FIELD_NUMBER: builtins.int
        run_id: builtins.str
        """The run_id that was sent with the data during ingestion (if any).
        Note that this may be different from the run_id that was requested in the query.
        """
        name: builtins.str
        def __init__(
            self,
            *,
            run_id: builtins.str = ...,
            name: builtins.str = ...,
        ) -> None: ...
        def ClearField(self, field_name: typing.Literal["name", b"name", "run_id", b"run_id"]) -> None: ...

    @typing.final
    class Channel(google.protobuf.message.Message):
        DESCRIPTOR: google.protobuf.descriptor.Descriptor

        @typing.final
        class Unit(google.protobuf.message.Message):
            DESCRIPTOR: google.protobuf.descriptor.Descriptor

            NAME_FIELD_NUMBER: builtins.int
            ABBREVIATED_NAME_FIELD_NUMBER: builtins.int
            name: builtins.str
            abbreviated_name: builtins.str
            def __init__(
                self,
                *,
                name: builtins.str = ...,
                abbreviated_name: builtins.str = ...,
            ) -> None: ...
            def ClearField(self, field_name: typing.Literal["abbreviated_name", b"abbreviated_name", "name", b"name"]) -> None: ...

        CHANNEL_ID_FIELD_NUMBER: builtins.int
        NAME_FIELD_NUMBER: builtins.int
        UNIT_FIELD_NUMBER: builtins.int
        ENUM_TYPES_FIELD_NUMBER: builtins.int
        BIT_FIELD_ELEMENTS_FIELD_NUMBER: builtins.int
        channel_id: builtins.str
        """For channel queries, this will contain the requested backing channel id.
        For calculated channel queries, this will contain the requested channel key.
        """
        name: builtins.str
        @property
        def unit(self) -> global___Metadata.Channel.Unit: ...
        @property
        def enum_types(self) -> google.protobuf.internal.containers.RepeatedCompositeFieldContainer[sift.common.type.v1.channel_enum_type_pb2.ChannelEnumType]: ...
        @property
        def bit_field_elements(self) -> google.protobuf.internal.containers.RepeatedCompositeFieldContainer[sift.common.type.v1.channel_bit_field_element_pb2.ChannelBitFieldElement]: ...
        def __init__(
            self,
            *,
            channel_id: builtins.str = ...,
            name: builtins.str = ...,
            unit: global___Metadata.Channel.Unit | None = ...,
            enum_types: collections.abc.Iterable[sift.common.type.v1.channel_enum_type_pb2.ChannelEnumType] | None = ...,
            bit_field_elements: collections.abc.Iterable[sift.common.type.v1.channel_bit_field_element_pb2.ChannelBitFieldElement] | None = ...,
        ) -> None: ...
        def HasField(self, field_name: typing.Literal["unit", b"unit"]) -> builtins.bool: ...
        def ClearField(self, field_name: typing.Literal["bit_field_elements", b"bit_field_elements", "channel_id", b"channel_id", "enum_types", b"enum_types", "name", b"name", "unit", b"unit"]) -> None: ...

    DATA_TYPE_FIELD_NUMBER: builtins.int
    SAMPLED_MS_FIELD_NUMBER: builtins.int
    ASSET_FIELD_NUMBER: builtins.int
    RUN_FIELD_NUMBER: builtins.int
    CHANNEL_FIELD_NUMBER: builtins.int
    data_type: sift.common.type.v1.channel_data_type_pb2.ChannelDataType.ValueType
    sampled_ms: builtins.int
    @property
    def asset(self) -> global___Metadata.Asset: ...
    @property
    def run(self) -> global___Metadata.Run: ...
    @property
    def channel(self) -> global___Metadata.Channel: ...
    def __init__(
        self,
        *,
        data_type: sift.common.type.v1.channel_data_type_pb2.ChannelDataType.ValueType = ...,
        sampled_ms: builtins.int = ...,
        asset: global___Metadata.Asset | None = ...,
        run: global___Metadata.Run | None = ...,
        channel: global___Metadata.Channel | None = ...,
    ) -> None: ...
    def HasField(self, field_name: typing.Literal["_channel", b"_channel", "_run", b"_run", "asset", b"asset", "channel", b"channel", "run", b"run"]) -> builtins.bool: ...
    def ClearField(self, field_name: typing.Literal["_channel", b"_channel", "_run", b"_run", "asset", b"asset", "channel", b"channel", "data_type", b"data_type", "run", b"run", "sampled_ms", b"sampled_ms"]) -> None: ...
    @typing.overload
    def WhichOneof(self, oneof_group: typing.Literal["_channel", b"_channel"]) -> typing.Literal["channel"] | None: ...
    @typing.overload
    def WhichOneof(self, oneof_group: typing.Literal["_run", b"_run"]) -> typing.Literal["run"] | None: ...

global___Metadata = Metadata

@typing.final
class DoubleValue(google.protobuf.message.Message):
    DESCRIPTOR: google.protobuf.descriptor.Descriptor

    TIMESTAMP_FIELD_NUMBER: builtins.int
    VALUE_FIELD_NUMBER: builtins.int
    value: builtins.float
    @property
    def timestamp(self) -> google.protobuf.timestamp_pb2.Timestamp: ...
    def __init__(
        self,
        *,
        timestamp: google.protobuf.timestamp_pb2.Timestamp | None = ...,
        value: builtins.float = ...,
    ) -> None: ...
    def HasField(self, field_name: typing.Literal["timestamp", b"timestamp"]) -> builtins.bool: ...
    def ClearField(self, field_name: typing.Literal["timestamp", b"timestamp", "value", b"value"]) -> None: ...

global___DoubleValue = DoubleValue

@typing.final
class DoubleValues(google.protobuf.message.Message):
    DESCRIPTOR: google.protobuf.descriptor.Descriptor

    METADATA_FIELD_NUMBER: builtins.int
    VALUES_FIELD_NUMBER: builtins.int
    @property
    def metadata(self) -> global___Metadata: ...
    @property
    def values(self) -> google.protobuf.internal.containers.RepeatedCompositeFieldContainer[global___DoubleValue]: ...
    def __init__(
        self,
        *,
        metadata: global___Metadata | None = ...,
        values: collections.abc.Iterable[global___DoubleValue] | None = ...,
    ) -> None: ...
    def HasField(self, field_name: typing.Literal["metadata", b"metadata"]) -> builtins.bool: ...
    def ClearField(self, field_name: typing.Literal["metadata", b"metadata", "values", b"values"]) -> None: ...

global___DoubleValues = DoubleValues

@typing.final
class StringValue(google.protobuf.message.Message):
    DESCRIPTOR: google.protobuf.descriptor.Descriptor

    TIMESTAMP_FIELD_NUMBER: builtins.int
    VALUE_FIELD_NUMBER: builtins.int
    value: builtins.str
    @property
    def timestamp(self) -> google.protobuf.timestamp_pb2.Timestamp: ...
    def __init__(
        self,
        *,
        timestamp: google.protobuf.timestamp_pb2.Timestamp | None = ...,
        value: builtins.str = ...,
    ) -> None: ...
    def HasField(self, field_name: typing.Literal["timestamp", b"timestamp"]) -> builtins.bool: ...
    def ClearField(self, field_name: typing.Literal["timestamp", b"timestamp", "value", b"value"]) -> None: ...

global___StringValue = StringValue

@typing.final
class StringValues(google.protobuf.message.Message):
    DESCRIPTOR: google.protobuf.descriptor.Descriptor

    METADATA_FIELD_NUMBER: builtins.int
    VALUES_FIELD_NUMBER: builtins.int
    @property
    def metadata(self) -> global___Metadata: ...
    @property
    def values(self) -> google.protobuf.internal.containers.RepeatedCompositeFieldContainer[global___StringValue]: ...
    def __init__(
        self,
        *,
        metadata: global___Metadata | None = ...,
        values: collections.abc.Iterable[global___StringValue] | None = ...,
    ) -> None: ...
    def HasField(self, field_name: typing.Literal["metadata", b"metadata"]) -> builtins.bool: ...
    def ClearField(self, field_name: typing.Literal["metadata", b"metadata", "values", b"values"]) -> None: ...

global___StringValues = StringValues

@typing.final
class EnumValue(google.protobuf.message.Message):
    DESCRIPTOR: google.protobuf.descriptor.Descriptor

    TIMESTAMP_FIELD_NUMBER: builtins.int
    VALUE_FIELD_NUMBER: builtins.int
    value: builtins.int
    @property
    def timestamp(self) -> google.protobuf.timestamp_pb2.Timestamp: ...
    def __init__(
        self,
        *,
        timestamp: google.protobuf.timestamp_pb2.Timestamp | None = ...,
        value: builtins.int = ...,
    ) -> None: ...
    def HasField(self, field_name: typing.Literal["timestamp", b"timestamp"]) -> builtins.bool: ...
    def ClearField(self, field_name: typing.Literal["timestamp", b"timestamp", "value", b"value"]) -> None: ...

global___EnumValue = EnumValue

@typing.final
class EnumValues(google.protobuf.message.Message):
    DESCRIPTOR: google.protobuf.descriptor.Descriptor

    METADATA_FIELD_NUMBER: builtins.int
    VALUES_FIELD_NUMBER: builtins.int
    @property
    def metadata(self) -> global___Metadata: ...
    @property
    def values(self) -> google.protobuf.internal.containers.RepeatedCompositeFieldContainer[global___EnumValue]: ...
    def __init__(
        self,
        *,
        metadata: global___Metadata | None = ...,
        values: collections.abc.Iterable[global___EnumValue] | None = ...,
    ) -> None: ...
    def HasField(self, field_name: typing.Literal["metadata", b"metadata"]) -> builtins.bool: ...
    def ClearField(self, field_name: typing.Literal["metadata", b"metadata", "values", b"values"]) -> None: ...

global___EnumValues = EnumValues

@typing.final
class BitFieldValue(google.protobuf.message.Message):
    DESCRIPTOR: google.protobuf.descriptor.Descriptor

    TIMESTAMP_FIELD_NUMBER: builtins.int
    VALUE_FIELD_NUMBER: builtins.int
    value: builtins.int
    @property
    def timestamp(self) -> google.protobuf.timestamp_pb2.Timestamp: ...
    def __init__(
        self,
        *,
        timestamp: google.protobuf.timestamp_pb2.Timestamp | None = ...,
        value: builtins.int = ...,
    ) -> None: ...
    def HasField(self, field_name: typing.Literal["timestamp", b"timestamp"]) -> builtins.bool: ...
    def ClearField(self, field_name: typing.Literal["timestamp", b"timestamp", "value", b"value"]) -> None: ...

global___BitFieldValue = BitFieldValue

@typing.final
class BitFieldElementValues(google.protobuf.message.Message):
    DESCRIPTOR: google.protobuf.descriptor.Descriptor

    NAME_FIELD_NUMBER: builtins.int
    VALUES_FIELD_NUMBER: builtins.int
    name: builtins.str
    @property
    def values(self) -> google.protobuf.internal.containers.RepeatedCompositeFieldContainer[global___BitFieldValue]: ...
    def __init__(
        self,
        *,
        name: builtins.str = ...,
        values: collections.abc.Iterable[global___BitFieldValue] | None = ...,
    ) -> None: ...
    def ClearField(self, field_name: typing.Literal["name", b"name", "values", b"values"]) -> None: ...

global___BitFieldElementValues = BitFieldElementValues

@typing.final
class BitFieldValues(google.protobuf.message.Message):
    DESCRIPTOR: google.protobuf.descriptor.Descriptor

    METADATA_FIELD_NUMBER: builtins.int
    VALUES_FIELD_NUMBER: builtins.int
    @property
    def metadata(self) -> global___Metadata: ...
    @property
    def values(self) -> google.protobuf.internal.containers.RepeatedCompositeFieldContainer[global___BitFieldElementValues]: ...
    def __init__(
        self,
        *,
        metadata: global___Metadata | None = ...,
        values: collections.abc.Iterable[global___BitFieldElementValues] | None = ...,
    ) -> None: ...
    def HasField(self, field_name: typing.Literal["metadata", b"metadata"]) -> builtins.bool: ...
    def ClearField(self, field_name: typing.Literal["metadata", b"metadata", "values", b"values"]) -> None: ...

global___BitFieldValues = BitFieldValues

@typing.final
class BoolValue(google.protobuf.message.Message):
    DESCRIPTOR: google.protobuf.descriptor.Descriptor

    TIMESTAMP_FIELD_NUMBER: builtins.int
    VALUE_FIELD_NUMBER: builtins.int
    value: builtins.bool
    @property
    def timestamp(self) -> google.protobuf.timestamp_pb2.Timestamp: ...
    def __init__(
        self,
        *,
        timestamp: google.protobuf.timestamp_pb2.Timestamp | None = ...,
        value: builtins.bool = ...,
    ) -> None: ...
    def HasField(self, field_name: typing.Literal["timestamp", b"timestamp"]) -> builtins.bool: ...
    def ClearField(self, field_name: typing.Literal["timestamp", b"timestamp", "value", b"value"]) -> None: ...

global___BoolValue = BoolValue

@typing.final
class BoolValues(google.protobuf.message.Message):
    DESCRIPTOR: google.protobuf.descriptor.Descriptor

    METADATA_FIELD_NUMBER: builtins.int
    VALUES_FIELD_NUMBER: builtins.int
    @property
    def metadata(self) -> global___Metadata: ...
    @property
    def values(self) -> google.protobuf.internal.containers.RepeatedCompositeFieldContainer[global___BoolValue]: ...
    def __init__(
        self,
        *,
        metadata: global___Metadata | None = ...,
        values: collections.abc.Iterable[global___BoolValue] | None = ...,
    ) -> None: ...
    def HasField(self, field_name: typing.Literal["metadata", b"metadata"]) -> builtins.bool: ...
    def ClearField(self, field_name: typing.Literal["metadata", b"metadata", "values", b"values"]) -> None: ...

global___BoolValues = BoolValues

@typing.final
class FloatValue(google.protobuf.message.Message):
    DESCRIPTOR: google.protobuf.descriptor.Descriptor

    TIMESTAMP_FIELD_NUMBER: builtins.int
    VALUE_FIELD_NUMBER: builtins.int
    value: builtins.float
    @property
    def timestamp(self) -> google.protobuf.timestamp_pb2.Timestamp: ...
    def __init__(
        self,
        *,
        timestamp: google.protobuf.timestamp_pb2.Timestamp | None = ...,
        value: builtins.float = ...,
    ) -> None: ...
    def HasField(self, field_name: typing.Literal["timestamp", b"timestamp"]) -> builtins.bool: ...
    def ClearField(self, field_name: typing.Literal["timestamp", b"timestamp", "value", b"value"]) -> None: ...

global___FloatValue = FloatValue

@typing.final
class FloatValues(google.protobuf.message.Message):
    DESCRIPTOR: google.protobuf.descriptor.Descriptor

    METADATA_FIELD_NUMBER: builtins.int
    VALUES_FIELD_NUMBER: builtins.int
    @property
    def metadata(self) -> global___Metadata: ...
    @property
    def values(self) -> google.protobuf.internal.containers.RepeatedCompositeFieldContainer[global___FloatValue]: ...
    def __init__(
        self,
        *,
        metadata: global___Metadata | None = ...,
        values: collections.abc.Iterable[global___FloatValue] | None = ...,
    ) -> None: ...
    def HasField(self, field_name: typing.Literal["metadata", b"metadata"]) -> builtins.bool: ...
    def ClearField(self, field_name: typing.Literal["metadata", b"metadata", "values", b"values"]) -> None: ...

global___FloatValues = FloatValues

@typing.final
class Int32Value(google.protobuf.message.Message):
    DESCRIPTOR: google.protobuf.descriptor.Descriptor

    TIMESTAMP_FIELD_NUMBER: builtins.int
    VALUE_FIELD_NUMBER: builtins.int
    value: builtins.int
    @property
    def timestamp(self) -> google.protobuf.timestamp_pb2.Timestamp: ...
    def __init__(
        self,
        *,
        timestamp: google.protobuf.timestamp_pb2.Timestamp | None = ...,
        value: builtins.int = ...,
    ) -> None: ...
    def HasField(self, field_name: typing.Literal["timestamp", b"timestamp"]) -> builtins.bool: ...
    def ClearField(self, field_name: typing.Literal["timestamp", b"timestamp", "value", b"value"]) -> None: ...

global___Int32Value = Int32Value

@typing.final
class Int32Values(google.protobuf.message.Message):
    DESCRIPTOR: google.protobuf.descriptor.Descriptor

    METADATA_FIELD_NUMBER: builtins.int
    VALUES_FIELD_NUMBER: builtins.int
    @property
    def metadata(self) -> global___Metadata: ...
    @property
    def values(self) -> google.protobuf.internal.containers.RepeatedCompositeFieldContainer[global___Int32Value]: ...
    def __init__(
        self,
        *,
        metadata: global___Metadata | None = ...,
        values: collections.abc.Iterable[global___Int32Value] | None = ...,
    ) -> None: ...
    def HasField(self, field_name: typing.Literal["metadata", b"metadata"]) -> builtins.bool: ...
    def ClearField(self, field_name: typing.Literal["metadata", b"metadata", "values", b"values"]) -> None: ...

global___Int32Values = Int32Values

@typing.final
class Uint32Value(google.protobuf.message.Message):
    DESCRIPTOR: google.protobuf.descriptor.Descriptor

    TIMESTAMP_FIELD_NUMBER: builtins.int
    VALUE_FIELD_NUMBER: builtins.int
    value: builtins.int
    @property
    def timestamp(self) -> google.protobuf.timestamp_pb2.Timestamp: ...
    def __init__(
        self,
        *,
        timestamp: google.protobuf.timestamp_pb2.Timestamp | None = ...,
        value: builtins.int = ...,
    ) -> None: ...
    def HasField(self, field_name: typing.Literal["timestamp", b"timestamp"]) -> builtins.bool: ...
    def ClearField(self, field_name: typing.Literal["timestamp", b"timestamp", "value", b"value"]) -> None: ...

global___Uint32Value = Uint32Value

@typing.final
class Uint32Values(google.protobuf.message.Message):
    DESCRIPTOR: google.protobuf.descriptor.Descriptor

    METADATA_FIELD_NUMBER: builtins.int
    VALUES_FIELD_NUMBER: builtins.int
    @property
    def metadata(self) -> global___Metadata: ...
    @property
    def values(self) -> google.protobuf.internal.containers.RepeatedCompositeFieldContainer[global___Uint32Value]: ...
    def __init__(
        self,
        *,
        metadata: global___Metadata | None = ...,
        values: collections.abc.Iterable[global___Uint32Value] | None = ...,
    ) -> None: ...
    def HasField(self, field_name: typing.Literal["metadata", b"metadata"]) -> builtins.bool: ...
    def ClearField(self, field_name: typing.Literal["metadata", b"metadata", "values", b"values"]) -> None: ...

global___Uint32Values = Uint32Values

@typing.final
class Int64Value(google.protobuf.message.Message):
    DESCRIPTOR: google.protobuf.descriptor.Descriptor

    TIMESTAMP_FIELD_NUMBER: builtins.int
    VALUE_FIELD_NUMBER: builtins.int
    value: builtins.int
    @property
    def timestamp(self) -> google.protobuf.timestamp_pb2.Timestamp: ...
    def __init__(
        self,
        *,
        timestamp: google.protobuf.timestamp_pb2.Timestamp | None = ...,
        value: builtins.int = ...,
    ) -> None: ...
    def HasField(self, field_name: typing.Literal["timestamp", b"timestamp"]) -> builtins.bool: ...
    def ClearField(self, field_name: typing.Literal["timestamp", b"timestamp", "value", b"value"]) -> None: ...

global___Int64Value = Int64Value

@typing.final
class Int64Values(google.protobuf.message.Message):
    DESCRIPTOR: google.protobuf.descriptor.Descriptor

    METADATA_FIELD_NUMBER: builtins.int
    VALUES_FIELD_NUMBER: builtins.int
    @property
    def metadata(self) -> global___Metadata: ...
    @property
    def values(self) -> google.protobuf.internal.containers.RepeatedCompositeFieldContainer[global___Int64Value]: ...
    def __init__(
        self,
        *,
        metadata: global___Metadata | None = ...,
        values: collections.abc.Iterable[global___Int64Value] | None = ...,
    ) -> None: ...
    def HasField(self, field_name: typing.Literal["metadata", b"metadata"]) -> builtins.bool: ...
    def ClearField(self, field_name: typing.Literal["metadata", b"metadata", "values", b"values"]) -> None: ...

global___Int64Values = Int64Values

@typing.final
class Uint64Value(google.protobuf.message.Message):
    DESCRIPTOR: google.protobuf.descriptor.Descriptor

    TIMESTAMP_FIELD_NUMBER: builtins.int
    VALUE_FIELD_NUMBER: builtins.int
    value: builtins.int
    @property
    def timestamp(self) -> google.protobuf.timestamp_pb2.Timestamp: ...
    def __init__(
        self,
        *,
        timestamp: google.protobuf.timestamp_pb2.Timestamp | None = ...,
        value: builtins.int = ...,
    ) -> None: ...
    def HasField(self, field_name: typing.Literal["timestamp", b"timestamp"]) -> builtins.bool: ...
    def ClearField(self, field_name: typing.Literal["timestamp", b"timestamp", "value", b"value"]) -> None: ...

global___Uint64Value = Uint64Value

@typing.final
class Uint64Values(google.protobuf.message.Message):
    DESCRIPTOR: google.protobuf.descriptor.Descriptor

    METADATA_FIELD_NUMBER: builtins.int
    VALUES_FIELD_NUMBER: builtins.int
    @property
    def metadata(self) -> global___Metadata: ...
    @property
    def values(self) -> google.protobuf.internal.containers.RepeatedCompositeFieldContainer[global___Uint64Value]: ...
    def __init__(
        self,
        *,
        metadata: global___Metadata | None = ...,
        values: collections.abc.Iterable[global___Uint64Value] | None = ...,
    ) -> None: ...
    def HasField(self, field_name: typing.Literal["metadata", b"metadata"]) -> builtins.bool: ...
    def ClearField(self, field_name: typing.Literal["metadata", b"metadata", "values", b"values"]) -> None: ...

global___Uint64Values = Uint64Values
