"""CH10 import format configuration."""

from __future__ import annotations

from sift.data_imports.v2.data_imports_pb2 import Ch10Config as Ch10ConfigProto

from sift_client.sift_types.import_formats._base import DataImportConfigBase


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
