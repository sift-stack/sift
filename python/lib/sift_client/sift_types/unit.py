from __future__ import annotations

from typing import TYPE_CHECKING

from sift.unit.v2.unit_pb2 import Unit as UnitProto

from sift_client.sift_types._base import BaseType

if TYPE_CHECKING:
    from sift_client.client import SiftClient


class Unit(BaseType[UnitProto, "Unit"]):
    """Model of the Sift Unit."""

    name: str

    @classmethod
    def _from_proto(cls, proto: UnitProto, sift_client: SiftClient | None = None) -> Unit:
        return cls(
            id_=proto.unit_id,
            proto=proto,
            name=proto.abbreviated_name,
            _client=sift_client,
        )

    def _to_proto(self) -> UnitProto:
        """Convert to protobuf message."""
        return UnitProto(
            unit_id=self.id_ or "",
            abbreviated_name=self.name,
        )

    def __str__(self) -> str:
        return self.name
