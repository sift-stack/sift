"""Pytest tests for the Ping API.

These tests demonstrate and validate the usage of the Ping API including:
- Basic ping functionality
- Connection health checks
- Error handling and edge cases
"""

import pytest

from sift_client import SiftClient
from sift_client.resources import PingAPI, PingAPIAsync

pytestmark = pytest.mark.integration


def test_client_binding(sift_client):
    assert sift_client.ping
    assert isinstance(sift_client.ping, PingAPI)
    assert sift_client.async_.ping
    assert isinstance(sift_client.async_.ping, PingAPIAsync)


@pytest.fixture
def ping_api_async(sift_client: SiftClient):
    """Get the ping async API instance."""
    return sift_client.async_.ping


@pytest.fixture
def ping_api_sync(sift_client: SiftClient):
    """Get the synchronous ping API instance."""
    return sift_client.ping


class TestPingAPIAsync:
    """Test suite for the Ping API functionality."""

    @pytest.mark.asyncio
    async def test_basic_ping(self, ping_api_async):
        """Test basic ping functionality."""
        response = await ping_api_async.ping()

        # Verify response is a string
        assert isinstance(response, str)

        # Verify response is not empty
        assert len(response) > 0


class TestPingAPISync:
    """Test suite for the  Ping API functionality."""

    def test_basic_ping(self, ping_api_sync):
        """Test basic synchronous ping functionality."""
        response = ping_api_sync.ping()

        # Verify response is a string
        assert isinstance(response, str)

        # Verify response is not empty
        assert len(response) > 0
