"""Parquet import format configuration."""

from __future__ import annotations

from datetime import timezone
from typing import TYPE_CHECKING, ClassVar

from google.protobuf.timestamp_pb2 import Timestamp
from pydantic import BaseModel
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

from sift_client.sift_types._base import MappingHelper
from sift_client.sift_types.import_formats._base import (
    DataImportConfigBase,
    ParquetComplexTypesImportMode,
    TimeColumn,
    TimeFormat,
)

if TYPE_CHECKING:
    from sift_client.sift_types.ingestion import ChannelConfig


class ParquetTimeColumn(TimeColumn):
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
