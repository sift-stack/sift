"""Data import types for the Sift API."""

from __future__ import annotations

from datetime import datetime, timezone
from enum import Enum
from typing import TYPE_CHECKING, ClassVar

from google.protobuf.timestamp_pb2 import Timestamp
from pydantic import BaseModel, model_validator
from sift.data_imports.v2.data_imports_pb2 import Ch10Config as Ch10ConfigProto
from sift.data_imports.v2.data_imports_pb2 import CsvConfig as CsvConfigProto
from sift.data_imports.v2.data_imports_pb2 import CsvTimeColumn as CsvTimeColumnProto
from sift.data_imports.v2.data_imports_pb2 import DataImport as DataImportProto
from sift.data_imports.v2.data_imports_pb2 import DataImportStatus as DataImportStatusProto
from sift.data_imports.v2.data_imports_pb2 import (
    ParquetComplexTypesImportMode as ParquetComplexTypesImportModeProto,
)
from sift.data_imports.v2.data_imports_pb2 import ParquetConfig as ParquetConfigProto
from sift.data_imports.v2.data_imports_pb2 import (
    ParquetDataColumn as ParquetDataColumnProto,
)
from sift.data_imports.v2.data_imports_pb2 import (
    ParquetFlatDatasetConfig as ParquetFlatDatasetConfigProto,
)
from sift.data_imports.v2.data_imports_pb2 import (
    ParquetTimeColumn as ParquetTimeColumnProto,
)
from sift.data_imports.v2.data_imports_pb2 import TDMSConfig as TDMSConfigProto

from sift_client.sift_types._base import BaseType, MappingHelper, ModelCreateUpdateBase
from sift_client.sift_types.import_formats._base import TimeFormat

if TYPE_CHECKING:
    from sift_client.client import SiftClient
    from sift_client.sift_types.ingestion import ChannelConfig


class DataImportStatus(str, Enum):
    """Status of a data import."""

    PENDING = "PENDING"
    IN_PROGRESS = "IN_PROGRESS"
    SUCCEEDED = "SUCCEEDED"
    FAILED = "FAILED"

    def to_filter_str(self) -> str:
        """Convert to string representation."""
        return f"DATA_IMPORT_STATUS_{self.value}"

    @classmethod
    def from_proto(cls, proto_value: DataImportStatus) -> DataImportStatus:
        """Create from proto enum value."""
        mapping = {
            DataImportStatusProto.DATA_IMPORT_STATUS_PENDING: DataImportStatus.PENDING,
            DataImportStatusProto.DATA_IMPORT_STATUS_IN_PROGRESS: DataImportStatus.IN_PROGRESS,
            DataImportStatusProto.DATA_IMPORT_STATUS_SUCCEEDED: DataImportStatus.SUCCEEDED,
            DataImportStatusProto.DATA_IMPORT_STATUS_FAILED: DataImportStatus.FAILED,
        }
        if proto_value not in mapping:
            raise ValueError(f"Unknown DataImportStatus proto value: {proto_value}")
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


class _TimeColumn(BaseModel):
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


class CsvTimeColumn(_TimeColumn):
    """Time column configuration for CSV imports."""

    column_number: int

    def to_proto(self) -> CsvTimeColumnProto:
        """Convert to proto."""
        proto = CsvTimeColumnProto(
            column_number=self.column_number,
            format=self.format.to_proto(),
        )
        if self.relative_start_time is not None:
            proto.relative_start_time.CopyFrom(
                Timestamp(seconds=int(self.relative_start_time.timestamp()))
            )
        return proto

    @classmethod
    def from_proto(cls, proto: CsvTimeColumnProto) -> CsvTimeColumn:
        """Create from proto."""
        return cls(
            column_number=proto.column_number,
            format=TimeFormat.from_proto(proto.format),
            relative_start_time=(
                proto.relative_start_time.ToDatetime(tzinfo=timezone.utc)
                if proto.HasField("relative_start_time")
                else None
            ),
        )


class ParquetTimeColumn(_TimeColumn):
    """Time column configuration for Parquet imports."""

    path: str

    def to_proto(self) -> ParquetTimeColumnProto:
        """Convert to proto."""
        proto = ParquetTimeColumnProto(
            path=self.path,
            format=self.format.to_proto(),
        )
        if self.relative_start_time is not None:
            proto.relative_start_time.CopyFrom(
                Timestamp(seconds=int(self.relative_start_time.timestamp()))
            )
        return proto

    @classmethod
    def from_proto(cls, proto: ParquetTimeColumnProto) -> ParquetTimeColumn:
        """Create from proto."""
        return cls(
            path=proto.path,
            format=TimeFormat.from_proto(proto.format),
            relative_start_time=(
                proto.relative_start_time.ToDatetime(tzinfo=timezone.utc)
                if proto.HasField("relative_start_time")
                else None
            ),
        )


class ParquetDataColumn(BaseModel):
    """Data column configuration for Parquet imports."""

    path: str
    channel_config: ChannelConfig

    def to_proto(self) -> ParquetDataColumnProto:
        """Convert to proto."""
        return ParquetDataColumnProto(
            path=self.path,
            channel_config=self.channel_config._to_config_proto(),
        )

    @classmethod
    def from_proto(cls, proto: ParquetDataColumnProto) -> ParquetDataColumn:
        """Create from proto."""
        from sift_client.sift_types.ingestion import ChannelConfig

        return cls(
            path=proto.path,
            channel_config=ChannelConfig._from_proto(proto.channel_config),
        )


class ParquetFlatDatasetConfig(BaseModel):
    """Flat dataset configuration for Parquet imports."""

    time_column: ParquetTimeColumn
    data_columns: list[ParquetDataColumn]

    def to_proto(self) -> ParquetFlatDatasetConfigProto:
        """Convert to proto."""
        return ParquetFlatDatasetConfigProto(
            time_column=self.time_column.to_proto(),
            data_columns=[col.to_proto() for col in self.data_columns],
        )

    @classmethod
    def from_proto(cls, proto: ParquetFlatDatasetConfigProto) -> ParquetFlatDatasetConfig:
        """Create from proto."""
        return cls(
            time_column=ParquetTimeColumn.from_proto(proto.time_column),
            data_columns=[ParquetDataColumn.from_proto(col) for col in proto.data_columns],
        )


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


class CsvConfig(DataImportConfigBase):
    """Configuration for CSV data imports."""

    first_data_row: int
    time_column: CsvTimeColumn
    data_columns: dict[int, ChannelConfig]
    num_rows: int | None = None

    _to_proto_helpers: ClassVar[dict[str, MappingHelper]] = {
        "time_column": MappingHelper(
            proto_attr_path="time_column",
            converter=lambda tc: tc.to_proto(),
        ),
        "data_columns": MappingHelper(
            proto_attr_path="data_columns",
            converter=lambda cols: {k: v._to_config_proto() for k, v in cols.items()},
        ),
    }

    @model_validator(mode="after")
    def _validate_csv_config(self):
        """Validate CSV configuration."""
        if not self.data_columns:
            raise ValueError("data_columns cannot be empty")
        if self.first_data_row < 1:
            raise ValueError("first_data_row must be >= 1")
        return self

    def to_proto(self) -> CsvConfigProto:
        """Convert to proto."""
        proto = CsvConfigProto(
            asset_name=self.asset_name,
            run_name=self.run_name or "",
            run_id=self.run_id or "",
            first_data_row=self.first_data_row,
            time_column=self.time_column.to_proto(),
            data_columns={k: v._to_config_proto() for k, v in self.data_columns.items()},
        )
        if self.num_rows is not None:
            proto.num_rows = self.num_rows
        return proto

    @classmethod
    def from_proto(cls, proto: CsvConfigProto) -> CsvConfig:
        """Create from proto."""
        from sift_client.sift_types.ingestion import ChannelConfig

        return cls(
            asset_name=proto.asset_name,
            run_name=proto.run_name if proto.run_name else None,
            run_id=proto.run_id if proto.run_id else None,
            first_data_row=proto.first_data_row,
            time_column=CsvTimeColumn.from_proto(proto.time_column),
            data_columns={
                k: ChannelConfig._from_proto(v) for k, v in proto.data_columns.items()
            },
            num_rows=proto.num_rows if proto.HasField("num_rows") else None,
        )


class Ch10Config(DataImportConfigBase):
    """Configuration for CH10 data imports."""

    scale_values: bool = False

    def to_proto(self) -> Ch10ConfigProto:
        """Convert to proto."""
        return Ch10ConfigProto(
            asset_name=self.asset_name,
            run_name=self.run_name or "",
            scale_values=self.scale_values,
        )

    @classmethod
    def from_proto(cls, proto: Ch10ConfigProto) -> Ch10Config:
        """Create from proto."""
        return cls(
            asset_name=proto.asset_name,
            run_name=proto.run_name if proto.run_name else None,
            scale_values=proto.scale_values,
        )


class TDMSConfig(DataImportConfigBase):
    """Configuration for TDMS data imports."""

    start_time_override: datetime | None = None
    file_size: int | None = None

    _to_proto_helpers: ClassVar[dict[str, MappingHelper]] = {
        "start_time_override": MappingHelper(
            proto_attr_path="start_time_override",
            converter=lambda dt: Timestamp(seconds=int(dt.timestamp())),
        ),
    }

    def to_proto(self) -> TDMSConfigProto:
        """Convert to proto."""
        proto = TDMSConfigProto(
            asset_name=self.asset_name,
            run_name=self.run_name or "",
            run_id=self.run_id or "",
        )
        if self.start_time_override is not None:
            proto.start_time_override.CopyFrom(
                Timestamp(seconds=int(self.start_time_override.timestamp()))
            )
        if self.file_size is not None:
            proto.file_size = self.file_size
        return proto

    @classmethod
    def from_proto(cls, proto: TDMSConfigProto) -> TDMSConfig:
        """Create from proto."""
        return cls(
            asset_name=proto.asset_name,
            run_name=proto.run_name if proto.run_name else None,
            run_id=proto.run_id if proto.run_id else None,
            start_time_override=(
                proto.start_time_override.ToDatetime(tzinfo=timezone.utc)
                if proto.HasField("start_time_override")
                else None
            ),
            file_size=proto.file_size if proto.HasField("file_size") else None,
        )


class ParquetConfig(DataImportConfigBase):
    """Configuration for Parquet data imports."""

    flat_dataset: ParquetFlatDatasetConfig | None = None
    footer_offset: int
    footer_length: int
    complex_types_import_mode: ParquetComplexTypesImportMode

    _to_proto_helpers: ClassVar[dict[str, MappingHelper]] = {
        "flat_dataset": MappingHelper(
            proto_attr_path="flat_dataset",
            converter=lambda fd: fd.to_proto(),
        ),
        "complex_types_import_mode": MappingHelper(
            proto_attr_path="complex_types_import_mode",
            converter=lambda mode: mode.to_proto(),
        ),
    }

    def to_proto(self) -> ParquetConfigProto:
        """Convert to proto."""
        proto = ParquetConfigProto(
            asset_name=self.asset_name,
            run_name=self.run_name or "",
            run_id=self.run_id or "",
            footer_offset=self.footer_offset,
            footer_length=self.footer_length,
            complex_types_import_mode=self.complex_types_import_mode.to_proto(),
        )
        if self.flat_dataset is not None:
            proto.flat_dataset.CopyFrom(self.flat_dataset.to_proto())
        return proto

    @classmethod
    def from_proto(cls, proto: ParquetConfigProto) -> ParquetConfig:
        """Create from proto."""
        return cls(
            asset_name=proto.asset_name,
            run_name=proto.run_name if proto.run_name else None,
            run_id=proto.run_id if proto.run_id else None,
            flat_dataset=(
                ParquetFlatDatasetConfig.from_proto(proto.flat_dataset)
                if proto.HasField("flat_dataset")
                else None
            ),
            footer_offset=proto.footer_offset,
            footer_length=proto.footer_length,
            complex_types_import_mode=ParquetComplexTypesImportMode.from_proto(
                proto.complex_types_import_mode
            ),
        )


class DataImport(BaseType[DataImportProto, "DataImport"]):
    """A data import in the Sift system.

    Data imports represent the process of importing data files into Sift.
    """

    # Required fields
    created_date: datetime
    modified_date: datetime
    status: DataImportStatus

    # Optional fields
    source_url: str | None = None
    error_message: str | None = None
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
    def _from_proto(cls, proto: DataImportProto, sift_client: SiftClient | None = None) -> DataImport:
        """Create from proto."""
        return cls(
            proto=proto,
            id_=proto.data_import_id,
            created_date=proto.created_date.ToDatetime(tzinfo=timezone.utc),
            modified_date=proto.modified_date.ToDatetime(tzinfo=timezone.utc),
            status=DataImportStatus.from_proto(proto.status),
            source_url=proto.source_url if proto.source_url else None,
            error_message=proto.error_message if proto.error_message else None,
            csv_config=(
                CsvConfig.from_proto(proto.csv_config) if proto.HasField("csv_config") else None
            ),
            ch10_config=(
                Ch10Config.from_proto(proto.ch10_config)
                if proto.HasField("ch10_config")
                else None
            ),
            tdms_config=(
                TDMSConfig.from_proto(proto.tdms_config)
                if proto.HasField("tdms_config")
                else None
            ),
            parquet_config=(
                ParquetConfig.from_proto(proto.parquet_config)
                if proto.HasField("parquet_config")
                else None
            ),
            run_id=proto.run_id if proto.HasField("run_id") else None,
            report_id=proto.report_id if proto.HasField("report_id") else None,
            asset_id=proto.asset_id if proto.HasField("asset_id") else None,
            data_start_time=(
                proto.data_start_time.ToDatetime(tzinfo=timezone.utc)
                if proto.HasField("data_start_time")
                else None
            ),
            data_stop_time=(
                proto.data_stop_time.ToDatetime(tzinfo=timezone.utc)
                if proto.HasField("data_stop_time")
                else None
            ),
            _client=sift_client,
        )

    @property
    def is_pending(self) -> bool:
        """Return True if the data import is pending."""
        return self.status == DataImportStatus.PENDING

    @property
    def is_in_progress(self) -> bool:
        """Return True if the data import is in progress."""
        return self.status == DataImportStatus.IN_PROGRESS

    @property
    def is_succeeded(self) -> bool:
        """Return True if the data import has succeeded."""
        return self.status == DataImportStatus.SUCCEEDED

    @property
    def is_failed(self) -> bool:
        """Return True if the data import has failed."""
        return self.status == DataImportStatus.FAILED

    def refresh(self) -> DataImport:
        """Refresh this data import with the latest data from the API.

        Returns:
            The updated DataImport object.
        """
        updated_import = self.client.data_imports.get(self._id_or_error)
        self._update(updated_import)
        return self

    def retry(self) -> DataImport:
        """Retry this data import if it failed.

        Only works for URL-based imports that are in a failed state.

        Returns:
            The updated DataImport object.
        """
        updated_import = self.client.data_imports.retry(self)
        self._update(updated_import)
        return self
