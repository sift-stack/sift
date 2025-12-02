"""TDMS import format configuration."""

from __future__ import annotations

from datetime import datetime, timezone
from typing import ClassVar

from google.protobuf.timestamp_pb2 import Timestamp
from sift.data_imports.v2.data_imports_pb2 import TDMSConfig as TDMSConfigProto

from sift_client.sift_types._base import MappingHelper
from sift_client.sift_types.import_formats._base import DataImportConfigBase


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
