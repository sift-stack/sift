"""Tests for the assets low-level wrapper."""

from unittest.mock import AsyncMock, MagicMock

import pytest
from sift.assets.v1 import assets_pb2

from sift_client._internal.low_level_wrappers.assets import AssetsLowLevelClient


def _client_with_stub(stub: MagicMock) -> AssetsLowLevelClient:
    grpc = MagicMock()
    grpc.get_stub.return_value = stub
    return AssetsLowLevelClient(grpc)


class TestArchiveAsset:
    @pytest.mark.asyncio
    async def test_returns_archived_run_ids(self):
        stub = MagicMock()
        stub.ArchiveAsset = AsyncMock(
            return_value=assets_pb2.ArchiveAssetResponse(archived_run_ids=["run-1", "run-2"])
        )
        client = _client_with_stub(stub)

        archived = await client.archive_asset("asset-1", archive_runs=True)

        assert archived == ["run-1", "run-2"]

    @pytest.mark.asyncio
    async def test_returns_empty_when_no_runs_archived(self):
        stub = MagicMock()
        stub.ArchiveAsset = AsyncMock(return_value=assets_pb2.ArchiveAssetResponse())
        client = _client_with_stub(stub)

        archived = await client.archive_asset("asset-1")

        assert list(archived) == []

    @pytest.mark.asyncio
    async def test_request_carries_asset_id_and_archive_runs(self):
        stub = MagicMock()
        stub.ArchiveAsset = AsyncMock(return_value=assets_pb2.ArchiveAssetResponse())
        client = _client_with_stub(stub)

        await client.archive_asset("asset-1", archive_runs=True)

        request = stub.ArchiveAsset.call_args[0][0]
        assert request.asset_id == "asset-1"
        assert request.archive_runs is True
