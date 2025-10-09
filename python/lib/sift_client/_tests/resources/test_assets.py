"""Pytest tests for the Assets API.

These tests demonstrate and validate the usage of the Assets API including:
- Basic asset operations (get, list, find)
- Asset filtering and searching
- Asset updates and archiving
- Error handling and edge cases
"""

import pytest

from sift_client import SiftClient
from sift_client.resources import AssetsAPI, AssetsAPIAsync
from sift_client.sift_types import Asset

pytestmark = pytest.mark.integration


def test_client_binding(sift_client):
    assert sift_client.assets
    assert isinstance(sift_client.assets, AssetsAPI)
    assert sift_client.async_.assets
    assert isinstance(sift_client.async_.assets, AssetsAPIAsync)


@pytest.fixture
def assets_api_async(sift_client: SiftClient):
    """Get the async assets API instance."""
    return sift_client.async_.assets


@pytest.fixture
def assets_api_sync(sift_client: SiftClient):
    """Get the synchronous assets API instance."""
    return sift_client.assets


@pytest.fixture
def test_asset(assets_api_sync):
    assets = assets_api_sync.list_(limit=1)
    assert assets
    assert len(assets) >= 1
    return assets[0]


class TestAssetsAPIAsync:
    """Test suite for the async Assets API functionality."""

    class TestList:
        """Tests for the async list_ method."""

        @pytest.mark.asyncio
        async def test_basic_list(self, assets_api_async):
            """Test basic asset listing functionality."""
            assets = await assets_api_async.list_(limit=5)

            # Verify we get a list
            assert isinstance(assets, list)
            assert assets

            # If we have assets, verify their structure
            asset = assets[0]
            assert isinstance(asset, Asset)
            assert asset.id_ is not None
            assert asset.name is not None

        @pytest.mark.asyncio
        async def test_list_with_name_filter(self, assets_api_async):
            """Test asset listing with name filtering."""
            # First get some assets to work with
            all_assets = await assets_api_async.list_(limit=10)

            if all_assets:
                # Use the first asset's name for filtering
                test_asset_name = all_assets[0].name
                filtered_assets = await assets_api_async.list_(name=test_asset_name)

                # Should find at least one asset with exact name match
                assert isinstance(filtered_assets, list)
                assert len(filtered_assets) >= 1

                # All returned assets should have the exact name
                for asset in filtered_assets:
                    assert asset.name == test_asset_name

        @pytest.mark.asyncio
        async def test_list_with_name_contains_filter(self, assets_api_async):
            """Test asset listing with name contains filtering."""
            # Test with a common substring that might exist in asset names
            assets = await assets_api_async.list_(name_contains="test", limit=5)

            assert isinstance(assets, list)

            # If we found assets, verify they contain the substring
            for asset in assets:
                assert "test" in asset.name.lower()

        @pytest.mark.asyncio
        async def test_list_with_limit(self, assets_api_async):
            """Test asset listing with different limits."""
            # Test with limit of 1
            assets_1 = await assets_api_async.list_(limit=1)
            assert isinstance(assets_1, list)
            assert len(assets_1) <= 1

            # Test with limit of 3
            assets_3 = await assets_api_async.list_(limit=3)
            assert isinstance(assets_3, list)
            assert len(assets_3) <= 3

        @pytest.mark.asyncio
        async def test_list_include_archived(self, assets_api_async):
            """Test asset listing with archived assets included."""
            # Test without archived assets (default)
            assets_active = await assets_api_async.list_(limit=5, include_archived=False)
            assert isinstance(assets_active, list)

            # Test with archived assets included
            assets_all = await assets_api_async.list_(limit=5, include_archived=True)
            assert isinstance(assets_all, list)

            # Should have at least as many assets when including archived
            assert len(assets_all) >= len(assets_active)

    class TestGet:
        """Tests for the async get method."""

        @pytest.mark.asyncio
        async def test_get_by_name(self, assets_api_async, test_asset):
            """Test getting a specific asset by name."""
            retrieved_asset = await assets_api_async.get(name=test_asset.name)

            assert retrieved_asset is not None
            assert retrieved_asset.id_ == test_asset.id_
            assert retrieved_asset.name == test_asset.name

        @pytest.mark.asyncio
        async def test_get_by_id(self, assets_api_async, test_asset):
            """Test getting a specific asset by ID."""
            retrieved_asset = await assets_api_async.get(asset_id=test_asset.id_)

            assert retrieved_asset is not None
            assert retrieved_asset.id_ == test_asset.id_

        @pytest.mark.asyncio
        async def test_get_without_params_raises_error(self, assets_api_async):
            """Test that getting an asset without parameters raises an error."""
            with pytest.raises(ValueError, match="Either asset_id or name must be provided"):
                await assets_api_async.get()

        @pytest.mark.asyncio
        async def test_get_nonexistent_asset_raises_error(self, assets_api_async):
            """Test that getting a non-existent asset raises an error."""
            with pytest.raises(ValueError, match="No asset found"):
                await assets_api_async.get(name="nonexistent-asset-name-12345")

    class TestFind:
        """Tests for the async find method."""

        @pytest.mark.asyncio
        async def test_find_asset(self, assets_api_async, test_asset):
            """Test finding a single asset."""
            # Find the same asset by name
            found_asset = await assets_api_async.find(name=test_asset.name)

            assert found_asset is not None
            assert found_asset.id_ == test_asset.id_

        @pytest.mark.asyncio
        async def test_find_nonexistent_asset(self, assets_api_async):
            """Test finding a non-existent asset returns None."""
            found_asset = await assets_api_async.find(name="nonexistent-asset-name-12345")
            assert found_asset is None

        @pytest.mark.asyncio
        async def test_find_multiple_raises_error(self, assets_api_async):
            """Test finding multiple assets raises an error."""
            with pytest.raises(ValueError, match="Multiple"):
                await assets_api_async.find(name_contains="a")


class TestAssetsAPISync:
    """Test suite for the synchronous Assets API functionality.

    Only includes a single test for basic sync generation. No specific sync behavior difference tests are needed.
    """

    class TestList:
        """Tests for the sync list_ method."""

        def test_basic_list(self, assets_api_sync):
            """Test basic synchronous asset listing functionality."""
            assets = assets_api_sync.list_(limit=5)

            # Verify we get a list
            assert isinstance(assets, list)
            assert assets
            assert isinstance(assets[0], Asset)
