from __future__ import annotations

from abc import ABC
from datetime import datetime  # noqa: TC003
from enum import Enum
from typing import Union

from pydantic import BaseModel, model_validator
from sift.common.type.v1.channel_config_pb2 import ChannelConfig as ChannelConfigProto
from sift.common.type.v1.channel_enum_type_pb2 import ChannelEnumType as ChannelEnumTypeProto
from sift.data_imports.v2.data_imports_pb2 import (
    DATA_TYPE_KEY_CSV,
    DATA_TYPE_KEY_HDF5,
    DATA_TYPE_KEY_PARQUET_FLATDATASET,
    DATA_TYPE_KEY_PARQUET_SINGLE_CHANNEL_PER_ROW,
    DATA_TYPE_KEY_TDMS,
    DATA_TYPE_KEY_ULOG,
    PARQUET_COMPLEX_TYPES_IMPORT_MODE_BOTH,
    PARQUET_COMPLEX_TYPES_IMPORT_MODE_BYTES,
    PARQUET_COMPLEX_TYPES_IMPORT_MODE_IGNORE,
    PARQUET_COMPLEX_TYPES_IMPORT_MODE_STRING,
    TDMS_COMPLEX_COMPONENT_IMAGINARY,
    TDMS_COMPLEX_COMPONENT_REAL,
    TDMS_COMPLEX_COMPONENT_UNSPECIFIED,
    TDMS_FALLBACK_METHOD_FAIL_ON_ERROR,
    TDMS_FALLBACK_METHOD_IGNORE_ERROR,
    TDMS_FALLBACK_METHOD_UNSPECIFIED,
    ULOG_PARSE_ERROR_POLICY_FAIL_ON_ERROR,
    ULOG_PARSE_ERROR_POLICY_IGNORE_ERROR,
)
from sift.data_imports.v2.data_imports_pb2 import CsvConfig as CsvConfigProto
from sift.data_imports.v2.data_imports_pb2 import CsvTimeColumn as CsvTimeColumnProto
from sift.data_imports.v2.data_imports_pb2 import Hdf5Config as Hdf5ConfigProto
from sift.data_imports.v2.data_imports_pb2 import Hdf5DataConfig as Hdf5DataConfigProto
from sift.data_imports.v2.data_imports_pb2 import ParquetConfig as ParquetConfigProto
from sift.data_imports.v2.data_imports_pb2 import ParquetDataColumn as ParquetDataColumnProto
from sift.data_imports.v2.data_imports_pb2 import (
    ParquetFlatDatasetConfig as ParquetFlatDatasetConfigProto,
)
from sift.data_imports.v2.data_imports_pb2 import (
    ParquetSingleChannelPerRowConfig as ParquetSingleChannelPerRowConfigProto,
)
from sift.data_imports.v2.data_imports_pb2 import (
    ParquetSingleChannelPerRowMultiChannelConfig as ParquetSingleChannelPerRowMultiChannelConfigProto,
)
from sift.data_imports.v2.data_imports_pb2 import (
    ParquetSingleChannelPerRowSingleChannelConfig as ParquetSingleChannelPerRowSingleChannelConfigProto,
)
from sift.data_imports.v2.data_imports_pb2 import ParquetTimeColumn as ParquetTimeColumnProto
from sift.data_imports.v2.data_imports_pb2 import TDMSConfig as TDMSConfigProto
from sift.data_imports.v2.data_imports_pb2 import TdmsDataConfig as TdmsDataConfigProto
from sift.data_imports.v2.data_imports_pb2 import TimeFormat as TimeFormatProto
from sift.data_imports.v2.data_imports_pb2 import UlogConfig as UlogConfigProto
from sift.data_imports.v2.data_imports_pb2 import UlogDataConfig as UlogDataConfigProto

from sift_client._internal.util.timestamp import to_pb_timestamp
from sift_client.sift_types.channel import ChannelDataType


class TimeFormat(Enum):
    """Supported time formats for data import columns."""

    RELATIVE_NANOSECONDS = TimeFormatProto.TIME_FORMAT_RELATIVE_NANOSECONDS
    RELATIVE_MICROSECONDS = TimeFormatProto.TIME_FORMAT_RELATIVE_MICROSECONDS
    RELATIVE_MILLISECONDS = TimeFormatProto.TIME_FORMAT_RELATIVE_MILLISECONDS
    RELATIVE_SECONDS = TimeFormatProto.TIME_FORMAT_RELATIVE_SECONDS
    RELATIVE_MINUTES = TimeFormatProto.TIME_FORMAT_RELATIVE_MINUTES
    RELATIVE_HOURS = TimeFormatProto.TIME_FORMAT_RELATIVE_HOURS
    ABSOLUTE_RFC3339 = TimeFormatProto.TIME_FORMAT_ABSOLUTE_RFC3339
    ABSOLUTE_DATETIME = TimeFormatProto.TIME_FORMAT_ABSOLUTE_DATETIME
    ABSOLUTE_UNIX_SECONDS = TimeFormatProto.TIME_FORMAT_ABSOLUTE_UNIX_SECONDS
    ABSOLUTE_UNIX_MILLISECONDS = TimeFormatProto.TIME_FORMAT_ABSOLUTE_UNIX_MILLISECONDS
    ABSOLUTE_UNIX_MICROSECONDS = TimeFormatProto.TIME_FORMAT_ABSOLUTE_UNIX_MICROSECONDS
    ABSOLUTE_UNIX_NANOSECONDS = TimeFormatProto.TIME_FORMAT_ABSOLUTE_UNIX_NANOSECONDS


class DataTypeKey(Enum):
    """Supported file types and layouts for data import detection."""

    CSV = "csv"
    PARQUET_FLATDATASET = "parquet_flatdataset"
    PARQUET_SINGLE_CHANNEL_PER_ROW = "parquet_single_channel_per_row"
    TDMS = "tdms"
    HDF5_ONE_D = "hdf5_one_d"
    HDF5_TWO_D = "hdf5_two_d"
    HDF5_COMPOUND = "hdf5_compound"
    ULOG = "ulog"


DATA_TYPE_KEY_TO_PROTO = {
    DataTypeKey.CSV: DATA_TYPE_KEY_CSV,
    DataTypeKey.PARQUET_FLATDATASET: DATA_TYPE_KEY_PARQUET_FLATDATASET,
    DataTypeKey.PARQUET_SINGLE_CHANNEL_PER_ROW: DATA_TYPE_KEY_PARQUET_SINGLE_CHANNEL_PER_ROW,
    DataTypeKey.TDMS: DATA_TYPE_KEY_TDMS,
    DataTypeKey.HDF5_ONE_D: DATA_TYPE_KEY_HDF5,
    DataTypeKey.HDF5_TWO_D: DATA_TYPE_KEY_HDF5,
    DataTypeKey.HDF5_COMPOUND: DATA_TYPE_KEY_HDF5,
    DataTypeKey.ULOG: DATA_TYPE_KEY_ULOG,
}


EXTENSION_TO_DATA_TYPE_KEY: dict[str, DataTypeKey] = {
    ".csv": DataTypeKey.CSV,
    ".tdms": DataTypeKey.TDMS,
    ".ulg": DataTypeKey.ULOG,
}


class TimeColumnBase(BaseModel, ABC):
    """Base class for time column configurations.

    Attributes:
        format: The time format used in this column.
        relative_start_time: Required when using a relative time format.
    """

    format: TimeFormat
    relative_start_time: datetime | None = None

    @model_validator(mode="after")
    def _check_relative_start_time(self) -> TimeColumnBase:
        if (
            self.format is not None
            and self.format.name.startswith("RELATIVE_")
            and self.relative_start_time is None
        ):
            raise ValueError(
                f"'relative_start_time' is required when using a relative time format ({self.format.name})."
            )
        return self


class DataColumnBase(BaseModel, ABC):
    """Base class for data column definitions.

    Attributes:
        name: Channel name.
        data_type: The data type of the channel values.
        units: Optional units string.
        description: Optional channel description.
    """

    name: str
    data_type: ChannelDataType
    units: str = ""
    description: str = ""


class ImportConfigBase(BaseModel, ABC):
    """Base class for all import configurations.

    Attributes:
        asset_name: Name of the asset to import data into.
        run_name: Name for the run. Ignored if ``run_id`` is set.
        run_id: ID of an existing run to append data to.
    """

    asset_name: str
    run_name: str | None = None
    run_id: str | None = None


class CsvTimeColumn(TimeColumnBase):
    """Time column configuration for CSV imports.

    Attributes:
        column: The 1-indexed column number of the time column.
    """

    column: int

    def _to_proto(self) -> CsvTimeColumnProto:
        proto = CsvTimeColumnProto(
            column_number=self.column,
            format=self.format.value,
        )
        if self.relative_start_time is not None:
            proto.relative_start_time.CopyFrom(to_pb_timestamp(self.relative_start_time))
        return proto


class CsvDataColumn(DataColumnBase):
    """A data column definition for CSV imports.

    Attributes:
        column: The 1-indexed column number.
    """

    column: int


class CsvImportConfig(ImportConfigBase):
    """Configuration for importing a CSV file.

    Attributes:
        first_data_row: The first row containing data (1-indexed). Defaults to 2 to skip a header row.
        time_column: Time column configuration.
        data_columns: List of data column definitions.
    """

    first_data_row: int = 2
    time_column: CsvTimeColumn
    data_columns: list[CsvDataColumn]

    def __getitem__(self, name: str) -> CsvDataColumn:
        """Look up a data column by channel name.

        Example::

            config["temperature"].data_type = ChannelDataType.FLOAT
        """
        for dc in self.data_columns:
            if dc.name == name:
                return dc
        raise KeyError(f"No data column named '{name}'")

    def _to_proto(self) -> CsvConfigProto:
        if not self.data_columns:
            raise ValueError("Config has no data columns. Add at least one before importing.")
        return CsvConfigProto(
            asset_name=self.asset_name,
            run_name=self.run_name or "",
            run_id=self.run_id or "",
            first_data_row=self.first_data_row,
            time_column=self.time_column._to_proto(),
            data_columns={
                dc.column: ChannelConfigProto(
                    name=dc.name,
                    data_type=dc.data_type.value,
                    units=dc.units,
                    description=dc.description,
                )
                for dc in self.data_columns
            },
        )

    @classmethod
    def _from_proto(cls, proto: CsvConfigProto) -> CsvImportConfig:
        """Create from a proto CsvConfig (e.g. from DetectConfig response)."""
        relative_start_time = None
        if proto.time_column.HasField("relative_start_time"):
            from datetime import timezone

            relative_start_time = proto.time_column.relative_start_time.ToDatetime(
                tzinfo=timezone.utc
            )
        time_column = CsvTimeColumn(
            column=proto.time_column.column_number,
            format=TimeFormat(proto.time_column.format),
            relative_start_time=relative_start_time,
        )
        data_columns = [
            CsvDataColumn(
                column=col_num,
                name=ch_cfg.name,
                data_type=ChannelDataType(ch_cfg.data_type),
                units=ch_cfg.units,
                description=ch_cfg.description,
            )
            for col_num, ch_cfg in proto.data_columns.items()
        ]
        return cls(
            asset_name=proto.asset_name,
            run_name=proto.run_name or None,
            run_id=proto.run_id or None,
            first_data_row=proto.first_data_row or 2,
            time_column=time_column,
            data_columns=data_columns,
        )


class ParquetComplexTypesImportMode(Enum):
    """Controls how complex Parquet types (maps, lists, structs) are imported."""

    IGNORE = PARQUET_COMPLEX_TYPES_IMPORT_MODE_IGNORE
    BOTH = PARQUET_COMPLEX_TYPES_IMPORT_MODE_BOTH
    STRING = PARQUET_COMPLEX_TYPES_IMPORT_MODE_STRING
    BYTES = PARQUET_COMPLEX_TYPES_IMPORT_MODE_BYTES


class ParquetTimeColumn(TimeColumnBase):
    """Time column configuration for Parquet imports.

    Attributes:
        path: The column path in the Parquet schema (e.g. ``"timestamp"``).
        format: The time format. Optional at construction so that
            ``detect_config`` / ``import_from_path`` can apply the standard
            precedence chain (caller-passed ``time_format`` > server-detected
            format > ``ABSOLUTE_UNIX_NANOSECONDS`` fallback). Importing
            without a format set raises ``ValueError``.
    """

    path: str
    format: TimeFormat | None = None  # type: ignore[assignment]

    def _to_proto(self) -> ParquetTimeColumnProto:
        if not self.path:
            raise ValueError("ParquetTimeColumn.path must be set before importing.")
        if self.format is None:
            raise ValueError(
                "ParquetTimeColumn.format must be set before importing. "
                "Pass time_format to detect_config/import_from_path, or set "
                "config.time_column.format explicitly."
            )
        proto = ParquetTimeColumnProto(
            path=self.path,
            format=self.format.value,
        )
        if self.relative_start_time is not None:
            proto.relative_start_time.CopyFrom(to_pb_timestamp(self.relative_start_time))
        return proto

    @classmethod
    def _from_proto(cls, proto: ParquetTimeColumnProto) -> ParquetTimeColumn:
        relative_start_time = None
        if proto.HasField("relative_start_time"):
            from datetime import timezone

            relative_start_time = proto.relative_start_time.ToDatetime(tzinfo=timezone.utc)

        fmt = TimeFormat(proto.format) if proto.format else None
        return cls(
            path=proto.path or "",
            format=fmt,
            relative_start_time=relative_start_time,
        )


class ParquetDataColumn(DataColumnBase):
    """A data column definition for Parquet flat dataset imports.

    Attributes:
        path: The column path in the Parquet schema.
    """

    path: str


class ParquetFlatDatasetImportConfig(ImportConfigBase):
    """Configuration for importing a Parquet file with a flat dataset layout.

    Each column in the file maps to a separate channel.

    Attributes:
        time_column: Time column configuration.
        data_columns: List of data column definitions.
        footer_offset: Byte offset where the Parquet footer begins. Populated
            automatically when using ``detect_config``.
        footer_length: Length of the Parquet footer in bytes. Populated
            automatically when using ``detect_config``.
        complex_types_import_mode: How to handle complex Parquet types.
    """

    time_column: ParquetTimeColumn
    data_columns: list[ParquetDataColumn]
    footer_offset: int = 0
    footer_length: int = 0
    complex_types_import_mode: ParquetComplexTypesImportMode = ParquetComplexTypesImportMode.IGNORE

    def __getitem__(self, name: str) -> ParquetDataColumn:
        """Look up a data column by channel name.

        Example::

            config["temperature"].data_type = ChannelDataType.FLOAT
        """
        for dc in self.data_columns:
            if dc.name == name:
                return dc
        raise KeyError(f"No data column named '{name}'")

    def _to_proto(self) -> ParquetConfigProto:
        if not self.data_columns:
            raise ValueError("Config has no data columns. Add at least one before importing.")
        flat_dataset = ParquetFlatDatasetConfigProto(
            time_column=self.time_column._to_proto(),
            data_columns=[
                ParquetDataColumnProto(
                    path=dc.path,
                    channel_config=ChannelConfigProto(
                        name=dc.name,
                        data_type=dc.data_type.value,
                        units=dc.units,
                        description=dc.description,
                    ),
                )
                for dc in self.data_columns
            ],
        )
        return ParquetConfigProto(
            asset_name=self.asset_name,
            run_name=self.run_name or "",
            run_id=self.run_id or "",
            flat_dataset=flat_dataset,
            footer_offset=self.footer_offset,
            footer_length=self.footer_length,
            complex_types_import_mode=self.complex_types_import_mode.value,
        )

    @classmethod
    def _from_proto(
        cls,
        proto: ParquetConfigProto,
        footer_offset: int = 0,
        footer_length: int = 0,
    ) -> ParquetFlatDatasetImportConfig:
        """Create from a proto ParquetConfig with a flat_dataset config."""
        fd = proto.flat_dataset
        time_column = ParquetTimeColumn._from_proto(fd.time_column)
        data_columns = [
            ParquetDataColumn(
                path=dc.path,
                name=dc.channel_config.name,
                data_type=ChannelDataType(dc.channel_config.data_type),
                units=dc.channel_config.units,
                description=dc.channel_config.description,
            )
            for dc in fd.data_columns
        ]
        mode = proto.complex_types_import_mode
        return cls(
            asset_name=proto.asset_name,
            run_name=proto.run_name or None,
            run_id=proto.run_id or None,
            time_column=time_column,
            data_columns=data_columns,
            footer_offset=footer_offset or proto.footer_offset,
            footer_length=footer_length or proto.footer_length,
            complex_types_import_mode=ParquetComplexTypesImportMode(mode)
            if mode
            else ParquetComplexTypesImportMode.IGNORE,
        )


class ParquetSingleChannelConfig(DataColumnBase):
    """Configuration for a single-channel Parquet single-channel-per-row import.

    Attributes:
        data_path: The column path containing channel data.
    """

    data_path: str


class ParquetMultiChannelConfig(BaseModel):
    """Configuration for a multi-channel Parquet single-channel-per-row import.

    Attributes:
        name_path: The column path that identifies the channel name per row.
        data_path: The column path containing channel data.
    """

    name_path: str
    data_path: str


class ParquetSingleChannelPerRowImportConfig(ImportConfigBase):
    """Configuration for importing a Parquet file where each row represents
    a single channel's data point.

    Exactly one of ``single_channel`` or ``multi_channel`` must be set before
    importing. When returned by ``detect_config()``, neither field is populated
    and must be filled in before passing the config to ``import_from_path()``.

    Attributes:
        time_column: Time column configuration.
        single_channel: Set when the entire file contains data for one channel.
        multi_channel: Set when each row identifies its channel via a name column.
        footer_offset: Byte offset where the Parquet footer begins. Populated
            automatically when using ``detect_config``.
        footer_length: Length of the Parquet footer in bytes. Populated
            automatically when using ``detect_config``.
        complex_types_import_mode: How to handle complex Parquet types.
    """

    time_column: ParquetTimeColumn
    single_channel: ParquetSingleChannelConfig | None = None
    multi_channel: ParquetMultiChannelConfig | None = None
    footer_offset: int = 0
    footer_length: int = 0
    complex_types_import_mode: ParquetComplexTypesImportMode = ParquetComplexTypesImportMode.IGNORE

    @model_validator(mode="after")
    def _check_channel_config(self) -> ParquetSingleChannelPerRowImportConfig:
        if self.single_channel is not None and self.multi_channel is not None:
            raise ValueError(
                "Exactly one of 'single_channel' or 'multi_channel' must be set, not both."
            )
        return self

    def _to_proto(self) -> ParquetConfigProto:
        if self.single_channel is None and self.multi_channel is None:
            raise ValueError(
                "Either 'single_channel' or 'multi_channel' must be set before importing. "
                "If this config was returned by detect_config(), set one of these fields "
                "to specify the channel layout."
            )
        scpr = ParquetSingleChannelPerRowConfigProto(
            time_column=self.time_column._to_proto(),
        )
        if self.single_channel is not None:
            sc = self.single_channel
            scpr.single_channel.CopyFrom(
                ParquetSingleChannelPerRowSingleChannelConfigProto(
                    data_path=sc.data_path,
                    channel=ChannelConfigProto(
                        name=sc.name,
                        data_type=sc.data_type.value,
                        units=sc.units,
                        description=sc.description,
                    ),
                )
            )
        elif self.multi_channel is not None:
            scpr.multi_channel.CopyFrom(
                ParquetSingleChannelPerRowMultiChannelConfigProto(
                    name_path=self.multi_channel.name_path,
                    data_path=self.multi_channel.data_path,
                )
            )
        return ParquetConfigProto(
            asset_name=self.asset_name,
            run_name=self.run_name or "",
            run_id=self.run_id or "",
            single_channel_per_row=scpr,
            footer_offset=self.footer_offset,
            footer_length=self.footer_length,
            complex_types_import_mode=self.complex_types_import_mode.value,
        )

    @classmethod
    def _from_proto(
        cls,
        proto: ParquetConfigProto,
        footer_offset: int = 0,
        footer_length: int = 0,
    ) -> ParquetSingleChannelPerRowImportConfig:
        """Create from a proto ParquetConfig with a single_channel_per_row config."""
        scpr = proto.single_channel_per_row

        time_column = ParquetTimeColumn._from_proto(scpr.time_column)

        single_channel = None
        multi_channel = None
        if scpr.HasField("single_channel"):
            sc = scpr.single_channel
            single_channel = ParquetSingleChannelConfig(
                data_path=sc.data_path,
                name=sc.channel.name,
                data_type=ChannelDataType(sc.channel.data_type),
                units=sc.channel.units,
                description=sc.channel.description,
            )
        elif scpr.HasField("multi_channel"):
            mc = scpr.multi_channel
            multi_channel = ParquetMultiChannelConfig(
                name_path=mc.name_path,
                data_path=mc.data_path,
            )

        mode = proto.complex_types_import_mode
        return cls(
            asset_name=proto.asset_name,
            run_name=proto.run_name or None,
            run_id=proto.run_id or None,
            time_column=time_column,
            single_channel=single_channel,
            multi_channel=multi_channel,
            footer_offset=footer_offset or proto.footer_offset,
            footer_length=footer_length or proto.footer_length,
            complex_types_import_mode=ParquetComplexTypesImportMode(mode)
            if mode
            else ParquetComplexTypesImportMode.IGNORE,
        )


class TdmsFallbackMethod(Enum):
    """Controls handling of TDMS channels without timing information
    during auto-detection.

    Auto-detection runs when you call ``detect_tdms_config`` or when
    you import a ``TdmsImportConfig`` with empty ``data``.

    A channel has timing information when the group defines an
    ``xchannel``, the first channel is a ``TimeStamp`` type, or the
    channel has waveform properties (``wf_start_offset`` +
    ``wf_increment``).

    Use ``IGNORE_ERROR`` when a file mixes timeseries and
    non-timeseries data (e.g., binary blobs, spectra) and you want the
    non-timeseries channels skipped. Alternatively, build
    ``TdmsImportConfig.data`` explicitly to import only the valid
    timeseries channels.
    """

    FAIL_ON_ERROR = TDMS_FALLBACK_METHOD_FAIL_ON_ERROR
    """Raise if any channel lacks timing information."""

    IGNORE_ERROR = TDMS_FALLBACK_METHOD_IGNORE_ERROR
    """Skip channels that lack timing information."""


class TdmsComplexComponent(Enum):
    """Selects which component to import from complex-valued TDMS data."""

    REAL = TDMS_COMPLEX_COMPONENT_REAL
    IMAGINARY = TDMS_COMPLEX_COMPONENT_IMAGINARY


class TdmsDataColumn(DataColumnBase):
    """Per-channel configuration for TDMS imports.

    Attributes:
        group_name: The TDMS group name.
        channel_name: The TDMS channel name.
        time_channel_name: Explicit time channel. If unset, assumes waveform properties.
        scaled: Whether to import scaled or raw values. Defaults to True.
        complex_component: Which component to import for complex types. Defaults to real.
    """

    group_name: str
    channel_name: str
    time_channel_name: str | None = None
    scaled: bool | None = None
    complex_component: TdmsComplexComponent | None = None
    enum_types: dict[str, int] | None = None


class TdmsImportConfig(ImportConfigBase):
    """Configuration for importing a TDMS file.

    Attributes:
        start_time_override: Override the ``wf_start_time`` metadata field for all channels.
            Useful when waveform channels have ``wf_increment`` but no ``wf_start_time``.
        data: Per-channel configurations. If empty, ingests everything using the fallback method.
        fallback_method: How to handle channels with missing timing information.
        time_format: Time format for time channels not using the TDMS timestamp type.
        relative_start_time: Relative start time for channels using a non-standard time channel.
        import_file_properties: If true, imports TDMS file properties as run metadata.
    """

    start_time_override: datetime | None = None
    data: list[TdmsDataColumn] = []
    fallback_method: TdmsFallbackMethod = TdmsFallbackMethod.FAIL_ON_ERROR
    time_format: TimeFormat | None = None
    relative_start_time: datetime | None = None
    import_file_properties: bool = False

    def __getitem__(self, name: str) -> TdmsDataColumn:
        """Look up a data column by channel name."""
        for d in self.data:
            if d.name == name:
                return d
        raise KeyError(f"No data column named '{name}'")

    def _to_proto(self) -> TDMSConfigProto:
        proto = TDMSConfigProto(
            asset_name=self.asset_name,
            run_name=self.run_name or "",
            run_id=self.run_id or "",
            import_file_properties=self.import_file_properties,
        )
        if self.start_time_override is not None:
            proto.start_time_override.CopyFrom(to_pb_timestamp(self.start_time_override))
        proto.fallback_method = self.fallback_method.value
        if self.time_format is not None:
            proto.time_format = self.time_format.value
        if self.relative_start_time is not None:
            proto.relative_start_time.CopyFrom(to_pb_timestamp(self.relative_start_time))
        for d in self.data:
            channel_config = ChannelConfigProto(
                name=d.name,
                data_type=d.data_type.value,
                units=d.units,
                description=d.description,
            )
            if d.enum_types:
                channel_config.enum_types.extend(
                    ChannelEnumTypeProto(name=name, key=key) for name, key in d.enum_types.items()
                )
            entry = TdmsDataConfigProto(
                group_name=d.group_name,
                channel_name=d.channel_name,
                channel_config=channel_config,
            )
            if d.time_channel_name is not None:
                entry.time_channel_name = d.time_channel_name
            if d.scaled is not None:
                entry.scaled = d.scaled
            if d.complex_component is not None:
                entry.complex_component = d.complex_component.value
            proto.data.append(entry)
        return proto

    @classmethod
    def _from_proto(cls, proto: TDMSConfigProto) -> TdmsImportConfig:
        """Create from a proto TDMSConfig (e.g. from DetectConfig response)."""
        start_time_override = None
        if proto.HasField("start_time_override"):
            from datetime import timezone

            start_time_override = proto.start_time_override.ToDatetime(tzinfo=timezone.utc)

        relative_start_time = None
        if proto.HasField("relative_start_time"):
            from datetime import timezone

            relative_start_time = proto.relative_start_time.ToDatetime(tzinfo=timezone.utc)

        data = []
        for d in proto.data:
            ch = d.channel_config
            complex_component = None
            if d.complex_component and d.complex_component != TDMS_COMPLEX_COMPONENT_UNSPECIFIED:
                complex_component = TdmsComplexComponent(d.complex_component)
            enum_types = {e.name: e.key for e in ch.enum_types} if ch.enum_types else None
            data.append(
                TdmsDataColumn(
                    group_name=d.group_name,
                    channel_name=d.channel_name,
                    name=ch.name,
                    data_type=ChannelDataType(ch.data_type),
                    units=ch.units,
                    description=ch.description,
                    time_channel_name=d.time_channel_name
                    if d.HasField("time_channel_name")
                    else None,
                    scaled=d.scaled if d.HasField("scaled") else None,
                    complex_component=complex_component,
                    enum_types=enum_types,
                )
            )

        fallback_method = TdmsFallbackMethod.FAIL_ON_ERROR
        if proto.fallback_method and proto.fallback_method != TDMS_FALLBACK_METHOD_UNSPECIFIED:
            fallback_method = TdmsFallbackMethod(proto.fallback_method)

        time_format = None
        if proto.HasField("time_format"):
            time_format = TimeFormat(proto.time_format)

        return cls(
            asset_name=proto.asset_name,
            run_name=proto.run_name or None,
            run_id=proto.run_id or None,
            start_time_override=start_time_override,
            data=data,
            fallback_method=fallback_method,
            time_format=time_format,
            relative_start_time=relative_start_time,
            import_file_properties=proto.import_file_properties,
        )


class Hdf5DataColumn(DataColumnBase):
    """A dataset mapping for HDF5 imports.

    Each entry maps a time/value dataset pair to a channel.

    Attributes:
        time_dataset: HDF5 path to the time dataset.
        time_index: Column index within the time dataset. Defaults to 0.
        value_dataset: HDF5 path to the value dataset.
        value_index: Column index within the value dataset. Defaults to 0.
        time_field: For compound dataset types, the field name to use for time.
        value_field: For compound dataset types, the field name to use for value.
    """

    time_dataset: str
    time_index: int = 0
    value_dataset: str
    value_index: int = 0
    time_field: str | None = None
    value_field: str | None = None


class Hdf5ImportConfig(ImportConfigBase):
    """Configuration for importing an HDF5 file.

    Attributes:
        data: List of dataset mappings, each pairing a time and value dataset to a channel.
        time_format: The time format used across all time datasets. Always
            left unset by ``detect_config``: HDF5 timestamps aren't self-describing,
            so the caller must set this before importing. Importing without it
            raises ``ValueError``.
        relative_start_time: Required when using a relative time format.
    """

    data: list[Hdf5DataColumn]
    time_format: TimeFormat | None = None
    relative_start_time: datetime | None = None

    def __getitem__(self, name: str) -> Hdf5DataColumn:
        """Look up a data column by channel name.

        Example::

            config["temperature"].data_type = ChannelDataType.FLOAT
        """
        for dc in self.data:
            if dc.name == name:
                return dc
        raise KeyError(f"No data column named '{name}'")

    @model_validator(mode="after")
    def _check_relative_start_time(self) -> Hdf5ImportConfig:
        if (
            self.time_format is not None
            and self.time_format.name.startswith("RELATIVE_")
            and self.relative_start_time is None
        ):
            raise ValueError(
                f"'relative_start_time' is required when using a relative time format ({self.time_format.name})."
            )
        return self

    def _to_proto(self) -> Hdf5ConfigProto:
        if self.time_format is None:
            raise ValueError(
                "time_format is required to import HDF5 files. Set "
                "config.time_format explicitly (e.g. TimeFormat.ABSOLUTE_UNIX_NANOSECONDS "
                "or TimeFormat.ABSOLUTE_DATETIME) before importing."
            )
        proto = Hdf5ConfigProto(
            asset_name=self.asset_name,
            run_name=self.run_name or "",
            run_id=self.run_id or "",
            time_format=self.time_format.value,
            data=[
                Hdf5DataConfigProto(
                    time_dataset=d.time_dataset,
                    time_index=d.time_index,
                    value_dataset=d.value_dataset,
                    value_index=d.value_index,
                    channel_config=ChannelConfigProto(
                        name=d.name,
                        data_type=d.data_type.value,
                        units=d.units,
                        description=d.description,
                    ),
                    time_field=d.time_field,
                    value_field=d.value_field,
                )
                for d in self.data
            ],
        )
        if self.relative_start_time is not None:
            proto.relative_start_time.CopyFrom(to_pb_timestamp(self.relative_start_time))
        return proto


class UlogParseErrorPolicy(Enum):
    """How the importer handles recoverable ULog parse errors.

    Recoverable errors include a truncated final record, a data record
    referencing an unbound message id, and garbage bytes skipped to the next
    sync marker. The policy is enforced server-side at import time.
    """

    FAIL_ON_ERROR = ULOG_PARSE_ERROR_POLICY_FAIL_ON_ERROR
    """Fail the import on any recoverable parse error."""

    IGNORE_ERROR = ULOG_PARSE_ERROR_POLICY_IGNORE_ERROR
    """Import the records that parsed; skipped records are logged."""


class UlogDataColumn(DataColumnBase):
    """A channel to import from a ULog file.

    Attributes:
        channel: The full ULog channel name, formed from the message name, its
            multi-instance index, and the field (e.g. ``"sensor_accel_0.x"``).
            This selects the source field; the inherited ``name`` is the Sift
            channel name it is imported as and defaults to ``channel``.
    """

    channel: str

    @model_validator(mode="before")
    @classmethod
    def _default_name_to_channel(cls, data: object) -> object:
        # The Sift name defaults to the ULog channel key, matching the
        # import-all default, so a selection can be built from `channel` alone.
        if isinstance(data, dict) and not data.get("name") and data.get("channel"):
            data["name"] = data["channel"]
        return data


class UlogImportConfig(ImportConfigBase):
    """Configuration for importing a PX4 ULog (``.ulg``) file.

    ULog files are self-describing, so ``detect_config`` enumerates every
    channel from the embedded schema and you supply no column mapping. Inspect
    the detected ``data`` and drop, rename, or retype channels before importing.

    Attributes:
        data: Channels to import. Empty imports every detected channel with its
            defaults; if non-empty, only the listed channels are imported.
        relative_start_time: Log-start UTC. ULog timestamps are relative to a
            boot clock; the timeline is anchored to absolute time by the log's
            GPS fix when present, otherwise by this value. The import fails if
            neither is available, so set this for logs without a GPS fix.
        info_keys: Info (``I``/``M``) message keys to import as run metadata,
            stored as ``info.<key>``. Requires ``run_name`` or ``run_id``.
        param_keys: Parameter (``P``) names to import as run metadata, stored
            as ``param.<name>``. Requires ``run_name`` or ``run_id``.
        parse_error_policy: How to handle recoverable parse errors. Defaults to
            failing the import.
    """

    data: list[UlogDataColumn] = []
    relative_start_time: datetime | None = None
    info_keys: list[str] = []
    param_keys: list[str] = []
    parse_error_policy: UlogParseErrorPolicy = UlogParseErrorPolicy.FAIL_ON_ERROR

    def __getitem__(self, name: str) -> UlogDataColumn:
        """Look up a data column by Sift channel name.

        Example::

            config["sensor_accel_0.x"].data_type = ChannelDataType.DOUBLE
        """
        for dc in self.data:
            if dc.name == name:
                return dc
        raise KeyError(f"No data column named '{name}'")

    def _to_proto(self) -> UlogConfigProto:
        proto = UlogConfigProto(
            asset_name=self.asset_name,
            run_name=self.run_name or "",
            run_id=self.run_id or "",
            info_keys=self.info_keys,
            param_keys=self.param_keys,
            parse_error_policy=self.parse_error_policy.value,
        )
        if self.relative_start_time is not None:
            proto.relative_start_time.CopyFrom(to_pb_timestamp(self.relative_start_time))
        for dc in self.data:
            proto.data.append(
                UlogDataConfigProto(
                    channel=dc.channel,
                    channel_config=ChannelConfigProto(
                        name=dc.name,
                        data_type=dc.data_type.value,
                        units=dc.units,
                        description=dc.description,
                    ),
                )
            )
        return proto

    @classmethod
    def _from_proto(cls, proto: UlogConfigProto) -> UlogImportConfig:
        """Create from a proto UlogConfig (e.g. from a GetDataImport response)."""
        relative_start_time = None
        if proto.HasField("relative_start_time"):
            from datetime import timezone

            relative_start_time = proto.relative_start_time.ToDatetime(tzinfo=timezone.utc)

        parse_error_policy = UlogParseErrorPolicy.FAIL_ON_ERROR
        if proto.parse_error_policy == ULOG_PARSE_ERROR_POLICY_IGNORE_ERROR:
            parse_error_policy = UlogParseErrorPolicy.IGNORE_ERROR

        data = [
            UlogDataColumn(
                channel=d.channel,
                name=d.channel_config.name,
                data_type=ChannelDataType(d.channel_config.data_type),
                units=d.channel_config.units,
                description=d.channel_config.description,
            )
            for d in proto.data
        ]
        return cls(
            asset_name=proto.asset_name,
            run_name=proto.run_name or None,
            run_id=proto.run_id or None,
            data=data,
            relative_start_time=relative_start_time,
            info_keys=list(proto.info_keys),
            param_keys=list(proto.param_keys),
            parse_error_policy=parse_error_policy,
        )


ImportConfig = Union[
    CsvImportConfig,
    ParquetFlatDatasetImportConfig,
    ParquetSingleChannelPerRowImportConfig,
    TdmsImportConfig,
    Hdf5ImportConfig,
    UlogImportConfig,
]
