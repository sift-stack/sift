"""CSV import format configuration."""

from __future__ import annotations

from datetime import timezone
from typing import ClassVar

from google.protobuf.timestamp_pb2 import Timestamp
from pydantic import model_validator
from sift.data_imports.v2.data_imports_pb2 import CsvConfig as CsvConfigProto
from sift.data_imports.v2.data_imports_pb2 import CsvTimeColumn as CsvTimeColumnProto

from sift_client.sift_types._base import MappingHelper
from sift_client.sift_types.import_formats._base import (
    DataImportConfigBase,
    TimeColumn,
    TimeFormat,
)
from sift_client.sift_types.ingestion import ChannelConfig  # noqa: TC001


class CsvTimeColumn(TimeColumn):
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
            data_columns={k: ChannelConfig._from_proto(v) for k, v in proto.data_columns.items()},
            num_rows=proto.num_rows if proto.HasField("num_rows") else None,
        )
