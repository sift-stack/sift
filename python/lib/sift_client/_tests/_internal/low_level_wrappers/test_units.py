"""Tests for the Units low-level wrapper."""

from unittest.mock import AsyncMock, MagicMock

import pytest
from sift.unit.v2.unit_pb2 import CreateUnitResponse, ListUnitsResponse
from sift.unit.v2.unit_pb2 import Unit as UnitProto

from sift_client._internal.low_level_wrappers.units import UnitsLowLevelClient


@pytest.mark.asyncio
async def test_create_unit_rejects_empty_name():
    """create_unit raises before making a request when name is empty."""
    client = UnitsLowLevelClient(grpc_client=MagicMock())

    with pytest.raises(ValueError, match="name must be provided"):
        await client.create_unit("")


@pytest.mark.asyncio
async def test_create_unit_returns_created_unit_proto():
    """create_unit unwraps the response and returns the unit proto (unit_id + abbreviated_name)."""
    stub = MagicMock()
    stub.CreateUnit = AsyncMock(
        return_value=CreateUnitResponse(unit=UnitProto(unit_id="u1", abbreviated_name="volts"))
    )
    grpc_client = MagicMock()
    grpc_client.get_stub.return_value = stub

    unit = await UnitsLowLevelClient(grpc_client).create_unit("volts")

    assert unit.unit_id == "u1"
    assert unit.abbreviated_name == "volts"


@pytest.mark.asyncio
async def test_list_all_units_follows_pagination():
    """list_all_units concatenates units across pages until the page token is empty."""
    stub = MagicMock()
    stub.ListUnits = AsyncMock(
        side_effect=[
            ListUnitsResponse(
                units=[UnitProto(unit_id="u1", abbreviated_name="volts")],
                next_page_token="page2",
            ),
            ListUnitsResponse(
                units=[UnitProto(unit_id="u2", abbreviated_name="amps")],
                next_page_token="",
            ),
        ]
    )
    grpc_client = MagicMock()
    grpc_client.get_stub.return_value = stub

    units = await UnitsLowLevelClient(grpc_client).list_all_units()

    assert [(u.unit_id, u.abbreviated_name) for u in units] == [
        ("u1", "volts"),
        ("u2", "amps"),
    ]
    assert stub.ListUnits.await_count == 2
