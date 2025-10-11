"""Shared pytest fixtures for all tests."""

import os
from unittest.mock import MagicMock

import pytest

from sift_client import SiftClient
from sift_client.transport import SiftConnectionConfig
from sift_client.util.util import AsyncAPIs


@pytest.fixture(scope="session")
def sift_client() -> SiftClient:
    """Create a SiftClient instance for testing.

    This fixture is shared across all test files and is session-scoped
    to avoid creating multiple client instances.
    """
    grpc_url = os.getenv("SIFT_GRPC_URI", "localhost:50051")
    rest_url = os.getenv("SIFT_REST_URI", "localhost:8080")
    api_key = os.getenv("SIFT_API_KEY", "")

    return SiftClient(
        connection_config=SiftConnectionConfig(
            api_key=api_key,
            grpc_url=grpc_url,
            rest_url=rest_url,
            use_ssl=False,
        )
    )


@pytest.fixture
def mock_client():
    """Create a mock SiftClient for unit testing."""
    client = MagicMock(spec=SiftClient)
    # Configure the mock to have the necessary API attributes
    client.assets = MagicMock()
    client.runs = MagicMock()
    client.channels = MagicMock()
    client.calculated_channels = MagicMock()
    client.rules = MagicMock()
    client.test_results = MagicMock()
    client.async_ = MagicMock(spec=AsyncAPIs)
    client.async_.ingestion = MagicMock()
    return client
