"""Tests for sift_types.Unit model."""

from unittest.mock import MagicMock

import pytest
from sift.unit.v2.unit_pb2 import Unit as UnitProto

from sift_client.sift_types.unit import Unit


@pytest.fixture
def mock_unit(mock_client):
    """Create a mock Unit instance for testing."""
    unit = Unit(
        proto=MagicMock(),
        id_="test_unit_id",
        name="volts",
    )
    unit._apply_client_to_instance(mock_client)
    return unit


class TestUnit:
    """Unit tests for Unit model - tests properties and methods."""

    def test_unit_properties(self, mock_unit):
        """Test that Unit properties are accessible."""
        assert mock_unit.id_ == "test_unit_id"
        assert mock_unit.name == "volts"

    def test_unit_str(self, mock_unit):
        """Test Unit string representation."""
        assert str(mock_unit) == "volts"

    def test_unit_from_proto(self):
        """Test that Unit maps unit_id -> id_ and abbreviated_name -> name."""
        proto = UnitProto(unit_id="unit123", abbreviated_name="amps")
        unit = Unit._from_proto(proto)

        assert unit.id_ == "unit123"
        assert unit.name == "amps"

    def test_unit_to_proto(self):
        """Test that Unit converts back to proto with name on abbreviated_name."""
        unit = Unit(id_="unit123", name="amps")
        proto = unit._to_proto()

        assert proto.unit_id == "unit123"
        assert proto.abbreviated_name == "amps"

    def test_unit_without_client_raises_error(self):
        """Test that accessing client without setting it raises an error."""
        unit = Unit(id_="test_unit_id", name="volts")

        with pytest.raises(AttributeError, match="Sift client not set"):
            _ = unit.client
