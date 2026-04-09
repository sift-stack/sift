from __future__ import annotations

from abc import ABC
from datetime import datetime  # noqa: TC003
from enum import Enum
from typing import Union

from pydantic import BaseModel, model_validator
from sift.common.type.v1.channel_config_pb2 import ChannelConfig as ChannelConfigProto
from sift.data_imports.v2.data_imports_pb2 import (
    DATA_TYPE_KEY_CH10,
    DATA_TYPE_KEY_CSV,
    DATA_TYPE_KEY_HDF5,
    DATA_TYPE_KEY_PARQUET_FLATDATASET,
    DATA_TYPE_KEY_PARQUET_SINGLE_CHANNEL_PER_ROW,
    DATA_TYPE_KEY_TDMS,
    PARQUET_COMPLEX_TYPES_IMPORT_MODE_BOTH,
    PARQUET_COMPLEX_TYPES_IMPORT_MODE_BYTES,
    PARQUET_COMPLEX_TYPES_IMPORT_MODE_IGNORE,
    PARQUET_COMPLEX_TYPES_IMPORT_MODE_STRING,
)
from sift.data_imports.v2.data_imports_pb2 import Ch10Config as Ch10ConfigProto
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
from sift.data_imports.v2.data_imports_pb2 import TimeFormat as TimeFormatProto

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
    """Supported file types for data import detection."""

    CSV = DATA_TYPE_KEY_CSV
    PARQUET_FLATDATASET = DATA_TYPE_KEY_PARQUET_FLATDATASET
    PARQUET_SINGLE_CHANNEL_PER_ROW = DATA_TYPE_KEY_PARQUET_SINGLE_CHANNEL_PER_ROW
    TDMS = DATA_TYPE_KEY_TDMS
    CH10 = DATA_TYPE_KEY_CH10
    HDF5 = DATA_TYPE_KEY_HDF5


EXTENSION_TO_DATA_TYPE_KEY: dict[str, DataTypeKey] = {
    ".csv": DataTypeKey.CSV,
    ".tdms": DataTypeKey.TDMS,
    ".ch10": DataTypeKey.CH10,
    ".h5": DataTypeKey.HDF5,
    ".hdf5": DataTypeKey.HDF5,
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
        if self.format.name.startswith("RELATIVE_") and self.relative_start_time is None:
            raise ValueError(
                f"'relative_start_time' is required when using a relative time format ({self.format.name})."
            )
        return self


class CsvTimeColumn(TimeColumnBase):
    """Time column configuration for CSV imports.

    Attributes:
        column: The 1-indexed column number of the time column.
        format: The time format used in this column.
        relative_start_time: Required when using a relative time format.
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


class CsvDataColumn(BaseModel):
    """A data column definition for CSV imports.

    Attributes:
        column: The 1-indexed column number.
        name: Channel name.
        data_type: The data type of the channel values.
        units: Optional units string.
        description: Optional channel description.
    """

    column: int
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
        format: The time format used in this column.
        relative_start_time: Required when using a relative time format.
    """

    path: str
    format: TimeFormat = TimeFormat.ABSOLUTE_UNIX_NANOSECONDS

    def _to_proto(self) -> ParquetTimeColumnProto:
        if not self.path:
            raise ValueError("ParquetTimeColumn.path must be set before importing.")
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

        fmt = TimeFormat(proto.format) if proto.format else TimeFormat.ABSOLUTE_UNIX_NANOSECONDS
        return cls(
            path=proto.path or "",
            format=fmt,
            relative_start_time=relative_start_time,
        )


class ParquetDataColumn(BaseModel):
    """A data column definition for Parquet flat dataset imports.

    Attributes:
        path: The column path in the Parquet schema.
        name: Channel name.
        data_type: The data type of the channel values.
        units: Optional units string.
        description: Optional channel description.
    """

    path: str
    name: str
    data_type: ChannelDataType
    units: str = ""
    description: str = ""


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


class ParquetSingleChannelConfig(BaseModel):
    """Configuration for a single-channel Parquet single-channel-per-row import.

    Attributes:
        data_path: The column path containing channel data.
        name: Channel name.
        data_type: The data type of the channel values.
        units: Optional units string.
        description: Optional channel description.
    """

    data_path: str
    name: str
    data_type: ChannelDataType
    units: str = ""
    description: str = ""


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

    Exactly one of ``single_channel`` or ``multi_channel`` must be set.

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

    def _to_proto(self) -> ParquetConfigProto:
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


class Ch10ImportConfig(ImportConfigBase):
    """Configuration for importing a CH10 file.

    Attributes:
        scale_values: Whether to apply EU (engineering unit) scaling to channel values.
    """

    scale_values: bool = False

    def _to_proto(self) -> Ch10ConfigProto:
        return Ch10ConfigProto(
            asset_name=self.asset_name,
            run_name=self.run_name or "",
            scale_values=self.scale_values,
        )


class TdmsImportConfig(ImportConfigBase):
    """Configuration for importing a TDMS file.

    Attributes:
        start_time_override: Override the ``wf_start_time`` metadata field for all channels.
            Useful when waveform channels have ``wf_increment`` but no ``wf_start_time``.
        file_size: The file size in bytes. Required if the file has truncated chunks.
    """

    start_time_override: datetime | None = None
    file_size: int | None = None

    def _to_proto(self) -> TDMSConfigProto:
        proto = TDMSConfigProto(
            asset_name=self.asset_name,
            run_name=self.run_name or "",
            run_id=self.run_id or "",
        )
        if self.start_time_override is not None:
            proto.start_time_override.CopyFrom(to_pb_timestamp(self.start_time_override))
        if self.file_size is not None:
            proto.file_size = self.file_size
        return proto


class Hdf5DataColumn(BaseModel):
    """A dataset mapping for HDF5 imports.

    Each entry maps a time/value dataset pair to a channel.

    Attributes:
        time_dataset: HDF5 path to the time dataset.
        time_index: Column index within the time dataset. Defaults to 0.
        value_dataset: HDF5 path to the value dataset.
        value_index: Column index within the value dataset. Defaults to 0.
        name: Channel name.
        data_type: The data type of the channel values.
        units: Optional units string.
        description: Optional channel description.
        time_field: For compound dataset types, the field name to use for time.
        value_field: For compound dataset types, the field name to use for value.
    """

    time_dataset: str
    time_index: int = 0
    value_dataset: str
    value_index: int = 0
    name: str
    data_type: ChannelDataType
    units: str = ""
    description: str = ""
    time_field: str | None = None
    value_field: str | None = None


class Hdf5ImportConfig(ImportConfigBase):
    """Configuration for importing an HDF5 file.

    Attributes:
        data: List of dataset mappings, each pairing a time and value dataset to a channel.
        time_format: The time format used across all time datasets.
        relative_start_time: Required when using a relative time format.
    """

    data: list[Hdf5DataColumn]
    time_format: TimeFormat
    relative_start_time: datetime | None = None

    @model_validator(mode="after")
    def _check_relative_start_time(self) -> Hdf5ImportConfig:
        if self.time_format.name.startswith("RELATIVE_") and self.relative_start_time is None:
            raise ValueError(
                f"'relative_start_time' is required when using a relative time format ({self.time_format.name})."
            )
        return self

    def _to_proto(self) -> Hdf5ConfigProto:
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


ImportConfig = Union[
    CsvImportConfig,
    ParquetFlatDatasetImportConfig,
    ParquetSingleChannelPerRowImportConfig,
    Ch10ImportConfig,
    TdmsImportConfig,
    Hdf5ImportConfig,
]
