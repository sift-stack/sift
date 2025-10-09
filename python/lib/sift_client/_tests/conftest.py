"""Shared pytest fixtures for all tests."""

import os

import pytest

from sift_client import SiftClient


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
        api_key=api_key,
        grpc_url=grpc_url,
        rest_url=rest_url,
    )
