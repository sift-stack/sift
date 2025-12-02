"""Base classes and enums for data import configurations."""

from __future__ import annotations

from datetime import datetime
from enum import Enum

from pydantic import BaseModel, model_validator
from sift.data_imports.v2.data_imports_pb2 import (
    ParquetComplexTypesImportMode as ParquetComplexTypesImportModeProto,
)
from sift.data_imports.v2.data_imports_pb2 import TimeFormat as TimeFormatProto

from sift_client.sift_types._base import ModelCreateUpdateBase


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

    def to_filter_str(self) -> str:
        """Convert to string representation."""
        return f"TIME_FORMAT_{self.value}"

    def is_relative(self) -> bool:
        """Return True if this is a relative time format."""
        return self.value.startswith("RELATIVE_")

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
        if proto_value not in mapping:
            raise ValueError(f"Unknown TimeFormat proto value: {proto_value}")
        return mapping[proto_value]


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
        if proto_value not in mapping:
            raise ValueError(f"Unknown ParquetComplexTypesImportMode proto value: {proto_value}")
        return mapping[proto_value]


class TimeColumn(BaseModel):
    """Base class for time column configuration."""

    format: TimeFormat
    relative_start_time: datetime | None = None

    @model_validator(mode="after")
    def _validate_relative_start_time(self):
        """Validate that relative_start_time is provided for relative formats."""
        if self.format.is_relative() and self.relative_start_time is None:
            raise ValueError(
                f"relative_start_time is required for relative time format {self.format}"
            )
        if not self.format.is_relative() and self.relative_start_time is not None:
            raise ValueError(
                f"relative_start_time should not be provided for absolute time format {self.format}"
            )
        return self


class DataImportConfigBase(ModelCreateUpdateBase):
    """Base configuration for data imports with common fields."""

    asset_name: str
    run_name: str | None = None
    run_id: str | None = None

    @model_validator(mode="after")
    def _validate_run_fields(self):
        """Validate that only one of run_name or run_id is provided."""
        if self.run_name and self.run_id:
            raise ValueError("Only one of run_name or run_id should be provided, not both")
        return self
