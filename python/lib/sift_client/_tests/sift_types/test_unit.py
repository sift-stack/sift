"""Tests for sift_types.Unit model."""

from sift.unit.v2.unit_pb2 import Unit as UnitProto

from sift_client.sift_types.unit import Unit


class TestUnit:
    """Unit tests for the Unit model."""

    def test_from_proto_maps_unit_id_and_abbreviated_name(self):
        """_from_proto maps proto unit_id -> id_ and abbreviated_name -> name."""
        unit = Unit._from_proto(UnitProto(unit_id="unit123", abbreviated_name="volts"))

        assert unit.id_ == "unit123"
        assert unit.name == "volts"

    def test_to_proto_maps_id_and_name(self):
        """_to_proto maps id_ -> proto unit_id and name -> abbreviated_name."""
        proto = Unit(id_="unit123", name="volts")._to_proto()

        assert proto.unit_id == "unit123"
        assert proto.abbreviated_name == "volts"

    def test_str_returns_name(self):
        """str(unit) returns the unit name."""
        assert str(Unit(name="volts")) == "volts"
