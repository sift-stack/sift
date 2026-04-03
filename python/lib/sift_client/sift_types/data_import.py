from __future__ import annotations

from datetime import datetime  # noqa: TC003
from enum import Enum

from pydantic import BaseModel, ConfigDict, model_validator
from sift.common.type.v1.channel_config_pb2 import ChannelConfig as ChannelConfigProto
from sift.data_imports.v2.data_imports_pb2 import (
    DATA_TYPE_KEY_CH10,
    DATA_TYPE_KEY_CSV,
    DATA_TYPE_KEY_HDF5,
    DATA_TYPE_KEY_PARQUET_FLATDATASET,
    DATA_TYPE_KEY_PARQUET_SINGLE_CHANNEL_PER_ROW,
    DATA_TYPE_KEY_TDMS,
)
from sift.data_imports.v2.data_imports_pb2 import CsvConfig as CsvConfigProto
from sift.data_imports.v2.data_imports_pb2 import CsvTimeColumn as CsvTimeColumnProto
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


class CsvTimeColumn(BaseModel):
    """Time column configuration for CSV imports.

    Attributes:
        column: The 1-indexed column number of the time column.
        format: The time format used in this column.
        relative_start_time: Required when using a relative time format.
    """

    model_config = ConfigDict(frozen=True)

    column: int
    format: TimeFormat
    relative_start_time: datetime | None = None

    def _to_proto(self) -> CsvTimeColumnProto:
        proto = CsvTimeColumnProto(
            column_number=self.column,
            format=self.format.value,
        )
        if self.relative_start_time is not None:
            proto.relative_start_time.CopyFrom(to_pb_timestamp(self.relative_start_time))
        return proto

    @model_validator(mode="after")
    def _check_relative_start_time(self) -> CsvTimeColumn:
        if self.format.name.startswith("RELATIVE_") and self.relative_start_time is None:
            raise ValueError(
                f"'relative_start_time' is required when using a relative time format ({self.format.name})."
            )
        return self


class CsvDataColumn(BaseModel):
    """A data column definition for CSV imports.

    Attributes:
        column: The 1-indexed column number.
        name: Channel name.
        data_type: The data type of the channel values.
        units: Optional units string.
        description: Optional channel description.
    """

    model_config = ConfigDict(frozen=True)

    column: int
    name: str
    data_type: ChannelDataType
    units: str = ""
    description: str = ""


class CsvImportConfig(BaseModel):
    """Configuration for importing a CSV file.

    Attributes:
        asset_name: Name of the asset to import data into.
        run_name: Name for the run. Ignored if ``run_id`` is set.
        run_id: ID of an existing run to append data to.
        first_data_row: The first row containing data (1-indexed). Defaults to 2 to skip a header row.
        time_column: Time column configuration.
        data_columns: List of data column definitions.
    """

    model_config = ConfigDict(frozen=True)

    asset_name: str
    run_name: str | None = None
    run_id: str | None = None
    first_data_row: int = 2
    time_column: CsvTimeColumn
    data_columns: list[CsvDataColumn]

    def _to_proto(self) -> CsvConfigProto:
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
