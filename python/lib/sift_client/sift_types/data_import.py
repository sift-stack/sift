"""Data import types for the Sift client.

This module provides Pydantic models for data import configurations and related types.
"""

from __future__ import annotations

from datetime import datetime, timezone
from enum import Enum
from typing import TYPE_CHECKING

from google.protobuf.timestamp_pb2 import Timestamp
from pydantic import ConfigDict, Field
from sift.data_imports.v2.data_imports_pb2 import (
    CsvConfig as CsvConfigProto,
)
from sift.data_imports.v2.data_imports_pb2 import (
    CsvTimeColumn as CsvTimeColumnProto,
)
from sift.data_imports.v2.data_imports_pb2 import (
    DataImport as DataImportProto,
)
from sift.data_imports.v2.data_imports_pb2 import (
    ParquetTimeColumn as ParquetTimeColumnProto,
)
from sift.data_imports.v2.data_imports_pb2 import (
    DataImportStatus as DataImportStatusProto,
)
from sift.data_imports.v2.data_imports_pb2 import (
    ParquetComplexTypesImportMode as ParquetComplexTypesImportModeProto,
)
from sift.data_imports.v2.data_imports_pb2 import (
    TimeFormat as TimeFormatProto,
)

from sift_client.sift_types._base import BaseType, ModelCreate
from sift_client.sift_types.ingestion import ChannelConfig

if TYPE_CHECKING:
    from sift_client.client import SiftClient


class TimeColumn(ModelCreate):
    """Common time column configuration for data imports.
    
    This can be converted to CSV or Parquet specific time column configurations.
    """

    model_config = ConfigDict(frozen=False)

    format: TimeFormat
    relative_start_time: datetime | None = None

    def _get_proto_class(self):
        raise NotImplementedError("TimeColumn is a base class, use to_csv_time_column or to_parquet_time_column")

    def to_csv_time_column(self, column_number: int) -> "CsvTimeColumn":
        """Convert to CSV time column configuration.
        
        Args:
            column_number: The column number (1-indexed) for the CSV time column.
            
        Returns:
            A CsvTimeColumn instance.
        """
        return CsvTimeColumn(
            column_number=column_number,
            format=self.format,
            relative_start_time=self.relative_start_time,
        )

    def to_parquet_time_column(self, path: str) -> "ParquetTimeColumn":
        """Convert to Parquet time column configuration.
        
        Args:
            path: The path to the time column in the Parquet file.
            
        Returns:
            A ParquetTimeColumn instance.
        """
        return ParquetTimeColumn(
            path=path,
            format=self.format,
            relative_start_time=self.relative_start_time,
        )


class TimeFormat(str, Enum):
    """Time format for data imports."""

    RELATIVE_NANOSECONDS = "RELATIVE_NANOSECONDS"
    RELATIVE_MICROSECONDS = "RELATIVE_MICROSECONDS"
    RELATIVE_MILLISECONDS = "RELATIVE_MILLISECONDS"
    RELATIVE_SECONDS = "RELATIVE_SECONDS"
    RELATIVE_MINUTES = "RELATIVE_MINUTES"
    RELATIVE_HOURS = "RELATIVE_HOURS"
    ABSOLUTE_RFC3339 = "ABSOLUTE_RFC3339"
    ABSOLUTE_DATETIME = "ABSOLUTE_DATETIME"
    ABSOLUTE_UNIX_SECONDS = "ABSOLUTE_UNIX_SECONDS"
    ABSOLUTE_UNIX_MILLISECONDS = "ABSOLUTE_UNIX_MILLISECONDS"
    ABSOLUTE_UNIX_MICROSECONDS = "ABSOLUTE_UNIX_MICROSECONDS"
    ABSOLUTE_UNIX_NANOSECONDS = "ABSOLUTE_UNIX_NANOSECONDS"

    def to_proto(self) -> int:
        """Convert to proto enum value."""
        mapping = {
            TimeFormat.RELATIVE_NANOSECONDS: TimeFormatProto.TIME_FORMAT_RELATIVE_NANOSECONDS,
            TimeFormat.RELATIVE_MICROSECONDS: TimeFormatProto.TIME_FORMAT_RELATIVE_MICROSECONDS,
            TimeFormat.RELATIVE_MILLISECONDS: TimeFormatProto.TIME_FORMAT_RELATIVE_MILLISECONDS,
            TimeFormat.RELATIVE_SECONDS: TimeFormatProto.TIME_FORMAT_RELATIVE_SECONDS,
            TimeFormat.RELATIVE_MINUTES: TimeFormatProto.TIME_FORMAT_RELATIVE_MINUTES,
            TimeFormat.RELATIVE_HOURS: TimeFormatProto.TIME_FORMAT_RELATIVE_HOURS,
            TimeFormat.ABSOLUTE_RFC3339: TimeFormatProto.TIME_FORMAT_ABSOLUTE_RFC3339,
            TimeFormat.ABSOLUTE_DATETIME: TimeFormatProto.TIME_FORMAT_ABSOLUTE_DATETIME,
            TimeFormat.ABSOLUTE_UNIX_SECONDS: TimeFormatProto.TIME_FORMAT_ABSOLUTE_UNIX_SECONDS,
            TimeFormat.ABSOLUTE_UNIX_MILLISECONDS: TimeFormatProto.TIME_FORMAT_ABSOLUTE_UNIX_MILLISECONDS,
            TimeFormat.ABSOLUTE_UNIX_MICROSECONDS: TimeFormatProto.TIME_FORMAT_ABSOLUTE_UNIX_MICROSECONDS,
            TimeFormat.ABSOLUTE_UNIX_NANOSECONDS: TimeFormatProto.TIME_FORMAT_ABSOLUTE_UNIX_NANOSECONDS,
        }
        return mapping[self]

    @classmethod
    def from_proto(cls, proto_value: int) -> TimeFormat:
        """Create from proto enum value."""
        mapping = {
            TimeFormatProto.TIME_FORMAT_RELATIVE_NANOSECONDS: TimeFormat.RELATIVE_NANOSECONDS,
            TimeFormatProto.TIME_FORMAT_RELATIVE_MICROSECONDS: TimeFormat.RELATIVE_MICROSECONDS,
            TimeFormatProto.TIME_FORMAT_RELATIVE_MILLISECONDS: TimeFormat.RELATIVE_MILLISECONDS,
            TimeFormatProto.TIME_FORMAT_RELATIVE_SECONDS: TimeFormat.RELATIVE_SECONDS,
            TimeFormatProto.TIME_FORMAT_RELATIVE_MINUTES: TimeFormat.RELATIVE_MINUTES,
            TimeFormatProto.TIME_FORMAT_RELATIVE_HOURS: TimeFormat.RELATIVE_HOURS,
            TimeFormatProto.TIME_FORMAT_ABSOLUTE_RFC3339: TimeFormat.ABSOLUTE_RFC3339,
            TimeFormatProto.TIME_FORMAT_ABSOLUTE_DATETIME: TimeFormat.ABSOLUTE_DATETIME,
            TimeFormatProto.TIME_FORMAT_ABSOLUTE_UNIX_SECONDS: TimeFormat.ABSOLUTE_UNIX_SECONDS,
            TimeFormatProto.TIME_FORMAT_ABSOLUTE_UNIX_MILLISECONDS: TimeFormat.ABSOLUTE_UNIX_MILLISECONDS,
            TimeFormatProto.TIME_FORMAT_ABSOLUTE_UNIX_MICROSECONDS: TimeFormat.ABSOLUTE_UNIX_MICROSECONDS,
            TimeFormatProto.TIME_FORMAT_ABSOLUTE_UNIX_NANOSECONDS: TimeFormat.ABSOLUTE_UNIX_NANOSECONDS,
        }
        return mapping.get(proto_value, TimeFormat.RELATIVE_NANOSECONDS)


class DataImportStatus(str, Enum):
    """Status of a data import."""

    PENDING = "PENDING"
    IN_PROGRESS = "IN_PROGRESS"
    SUCCEEDED = "SUCCEEDED"
    FAILED = "FAILED"

    @classmethod
    def from_proto(cls, proto_value: int) -> DataImportStatus:
        """Create from proto enum value."""
        mapping = {
            DataImportStatusProto.DATA_IMPORT_STATUS_PENDING: DataImportStatus.PENDING,
            DataImportStatusProto.DATA_IMPORT_STATUS_IN_PROGRESS: DataImportStatus.IN_PROGRESS,
            DataImportStatusProto.DATA_IMPORT_STATUS_SUCCEEDED: DataImportStatus.SUCCEEDED,
            DataImportStatusProto.DATA_IMPORT_STATUS_FAILED: DataImportStatus.FAILED,
        }
        return mapping.get(proto_value, DataImportStatus.PENDING)


class ParquetComplexTypesImportMode(str, Enum):
    """Import mode for complex types in Parquet files."""

    IGNORE = "IGNORE"
    BOTH = "BOTH"
    STRING = "STRING"
    BYTES = "BYTES"

    def to_proto(self) -> int:
        """Convert to proto enum value."""
        mapping = {
            ParquetComplexTypesImportMode.IGNORE: ParquetComplexTypesImportModeProto.PARQUET_COMPLEX_TYPES_IMPORT_MODE_IGNORE,
            ParquetComplexTypesImportMode.BOTH: ParquetComplexTypesImportModeProto.PARQUET_COMPLEX_TYPES_IMPORT_MODE_BOTH,
            ParquetComplexTypesImportMode.STRING: ParquetComplexTypesImportModeProto.PARQUET_COMPLEX_TYPES_IMPORT_MODE_STRING,
            ParquetComplexTypesImportMode.BYTES: ParquetComplexTypesImportModeProto.PARQUET_COMPLEX_TYPES_IMPORT_MODE_BYTES,
        }
        return mapping[self]

    @classmethod
    def from_proto(cls, proto_value: int) -> ParquetComplexTypesImportMode:
        """Create from proto enum value."""
        mapping = {
            ParquetComplexTypesImportModeProto.PARQUET_COMPLEX_TYPES_IMPORT_MODE_IGNORE: ParquetComplexTypesImportMode.IGNORE,
            ParquetComplexTypesImportModeProto.PARQUET_COMPLEX_TYPES_IMPORT_MODE_BOTH: ParquetComplexTypesImportMode.BOTH,
            ParquetComplexTypesImportModeProto.PARQUET_COMPLEX_TYPES_IMPORT_MODE_STRING: ParquetComplexTypesImportMode.STRING,
            ParquetComplexTypesImportModeProto.PARQUET_COMPLEX_TYPES_IMPORT_MODE_BYTES: ParquetComplexTypesImportMode.BYTES,
        }
        return mapping.get(
            proto_value, ParquetComplexTypesImportMode.IGNORE
        )


class CsvTimeColumn(ModelCreate[CsvTimeColumnProto]):
    """Configuration for a CSV time column."""

    model_config = ConfigDict(frozen=False)

    column_number: int = 0
    format: TimeFormat
    relative_start_time: datetime | None = None

    @classmethod
    def from_time_column(cls, time_column: TimeColumn, column_number: int) -> CsvTimeColumn:
        """Create from a common TimeColumn.
        
        Args:
            time_column: The common time column configuration.
            column_number: The column number (1-indexed) for the CSV time column.
            
        Returns:
            A CsvTimeColumn instance.
        """
        return cls(
            column_number=column_number,
            format=time_column.format,
            relative_start_time=time_column.relative_start_time,
        )

    def _get_proto_class(self) -> type[CsvTimeColumnProto]:
        return CsvTimeColumnProto

    def to_proto(self) -> CsvTimeColumnProto:
        """Convert to proto."""
        proto = CsvTimeColumnProto(
            column_number=self.column_number,
            format=self.format.to_proto(),
        )
        if self.relative_start_time is not None:
            timestamp = Timestamp()
            timestamp.FromDatetime(self.relative_start_time)
            proto.relative_start_time.CopyFrom(timestamp)
        return proto

    @classmethod
    def from_proto(cls, proto: CsvTimeColumnProto) -> CsvTimeColumn:
        """Create from proto."""
        relative_start_time = None
        if proto.HasField("relative_start_time"):
            relative_start_time = proto.relative_start_time.ToDatetime(tzinfo=timezone.utc)
        return cls(
            column_number=proto.column_number,
            format=TimeFormat.from_proto(proto.format),
            relative_start_time=relative_start_time,
        )


class ParquetTimeColumn(ModelCreate[ParquetTimeColumnProto]):
    """Configuration for a Parquet time column."""

    model_config = ConfigDict(frozen=False)

    path: str
    format: TimeFormat
    relative_start_time: datetime | None = None

    @classmethod
    def from_time_column(cls, time_column: TimeColumn, path: str) -> ParquetTimeColumn:
        """Create from a common TimeColumn.
        
        Args:
            time_column: The common time column configuration.
            path: The path to the time column in the Parquet file.
            
        Returns:
            A ParquetTimeColumn instance.
        """
        return cls(
            path=path,
            format=time_column.format,
            relative_start_time=time_column.relative_start_time,
        )

    def _get_proto_class(self) -> type[ParquetTimeColumnProto]:
        return ParquetTimeColumnProto

    def to_proto(self) -> ParquetTimeColumnProto:
        """Convert to proto."""
        proto = ParquetTimeColumnProto(
            path=self.path,
            format=self.format.to_proto(),
        )
        if self.relative_start_time is not None:
            timestamp = Timestamp()
            timestamp.FromDatetime(self.relative_start_time)
            proto.relative_start_time.CopyFrom(timestamp)
        return proto

    @classmethod
    def from_proto(cls, proto: ParquetTimeColumnProto) -> ParquetTimeColumn:
        """Create from proto."""
        relative_start_time = None
        if proto.HasField("relative_start_time"):
            relative_start_time = proto.relative_start_time.ToDatetime(tzinfo=timezone.utc)
        return cls(
            path=proto.path,
            format=TimeFormat.from_proto(proto.format),
            relative_start_time=relative_start_time,
        )


class CsvConfig(ModelCreate[CsvConfigProto]):
    """Configuration for CSV data imports."""

    model_config = ConfigDict(frozen=False)

    asset_name: str
    run_name: str | None
    run_id: str | None
    first_data_row: int = 1
    time_column: CsvTimeColumn | None = Field(None, description="Configuration for the time column")
    data_columns: dict[int, ChannelConfig] = Field(
        default_factory=dict,
    )
    max_rows: int | None

    def _get_proto_class(self) -> type[CsvConfigProto]:
        return CsvConfigProto

    def to_proto(self) -> CsvConfigProto:
        """Convert to proto."""
        proto = CsvConfigProto(
            asset_name=self.asset_name,
            run_name=self.run_name,
            first_data_row=self.first_data_row,
        )
        if self.run_id is not None:
            proto.run_id = self.run_id
        if self.time_column is not None:
            proto.time_column.CopyFrom(self.time_column.to_proto())
        if self.data_columns:
            for col_num, channel_config in self.data_columns.items():
                proto.data_columns[col_num].CopyFrom(channel_config)
        if self.num_rows is not None:
            proto.num_rows = self.num_rows
        return proto

class DataImport(BaseType[DataImportProto, "DataImport"]):
    """Model of a Sift data import."""

    data_import_id: str
    source_url: str | None = None
    status: DataImportStatus
    error_message: str | None = None
    created_date: datetime
    modified_date: datetime
    csv_config: CsvConfig | None = None
    ch10_config: Ch10Config | None = None
    tdms_config: TDMSConfig | None = None
    parquet_config: ParquetConfig | None = None
    run_id: str | None = None
    report_id: str | None = None
    asset_id: str | None = None
    data_start_time: datetime | None = None
    data_stop_time: datetime | None = None

    @classmethod
    def _from_proto(
        cls, proto: DataImportProto, sift_client: SiftClient | None = None
    ) -> DataImport:
        """Create from proto."""
        csv_config = None
        if proto.HasField("csv_config"):
            csv_config = CsvConfig.from_proto(proto.csv_config)

        ch10_config = None
        if proto.HasField("ch10_config"):
            ch10_config = Ch10Config.from_proto(proto.ch10_config)

        tdms_config = None
        if proto.HasField("tdms_config"):
            tdms_config = TDMSConfig.from_proto(proto.tdms_config)

        parquet_config = None
        if proto.HasField("parquet_config"):
            parquet_config = ParquetConfig.from_proto(proto.parquet_config)

        data_start_time = None
        if proto.HasField("data_start_time"):
            data_start_time = proto.data_start_time.ToDatetime(tzinfo=timezone.utc)

        data_stop_time = None
        if proto.HasField("data_stop_time"):
            data_stop_time = proto.data_stop_time.ToDatetime(tzinfo=timezone.utc)

        return cls(
            proto=proto,
            id_=proto.data_import_id,
            data_import_id=proto.data_import_id,
            source_url=proto.source_url if proto.source_url else None,
            status=DataImportStatus.from_proto(proto.status),
            error_message=proto.error_message if proto.error_message else None,
            created_date=proto.created_date.ToDatetime(tzinfo=timezone.utc),
            modified_date=proto.modified_date.ToDatetime(tzinfo=timezone.utc),
            csv_config=csv_config,
            ch10_config=ch10_config,
            tdms_config=tdms_config,
            parquet_config=parquet_config,
            run_id=proto.run_id if proto.HasField("run_id") else None,
            report_id=proto.report_id if proto.HasField("report_id") else None,
            asset_id=proto.asset_id if proto.HasField("asset_id") else None,
            data_start_time=data_start_time,
            data_stop_time=data_stop_time,
            _client=sift_client,
        )
