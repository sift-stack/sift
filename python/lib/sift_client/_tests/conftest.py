"""Shared pytest fixtures for all tests."""

import os
from unittest.mock import MagicMock

import pytest

from sift_client import SiftClient, SiftConnectionConfig
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
    # If the URL contains localhost, don't use SSL.  Most likely running tests or local development.
    use_ssl = not ("localhost" in grpc_url or "localhost" in rest_url)

    client = SiftClient(
        connection_config=SiftConnectionConfig(
            api_key=api_key,
            grpc_url=grpc_url,
            rest_url=rest_url,
            use_ssl=use_ssl,
        )
    )

    return client


@pytest.fixture
def mock_client():
    """Create a mock SiftClient for unit testing."""
    client = MagicMock(spec=SiftClient)
    # Configure the mock to have the necessary API attributes
    client.assets = MagicMock()
    client.reports = MagicMock()
    client.runs = MagicMock()
    client.channels = MagicMock()
    client.calculated_channels = MagicMock()
    client.rules = MagicMock()
    client.tags = MagicMock()
    client.test_results = MagicMock()
    client.file_attachments = MagicMock()
    client.jobs = MagicMock()
    client.async_ = MagicMock(spec=AsyncAPIs)
    client.async_.ingestion = MagicMock()
    return client


@pytest.fixture(scope="session")
def nostromo_asset(sift_client):
    return sift_client.assets.find(name="NostromoLV426")


@pytest.fixture(scope="session")
def nostromo_run(nostromo_asset):
    return nostromo_asset.runs[0]


@pytest.fixture(scope="session")
def test_tag(sift_client):
    tag = sift_client.tags.find_or_create(names=["test"])[0]
    assert tag is not None
    return tag


@pytest.fixture(scope="session")
def ci_pytest_tag(sift_client):
    tag = sift_client.tags.find_or_create(names=["sift-client-pytest"])[0]
    assert tag is not None
    return tag


from sift_client.util.test_results import (
    client_has_connection,  # noqa: F401
    pytest_runtest_makereport,  # noqa: F401
)
from sift_client.util.test_results import (
    module_substep_check_connection as module_substep,  # noqa: F401
)
from sift_client.util.test_results import (
    report_context_check_connection as report_context,  # noqa: F401
)
from sift_client.util.test_results import (
    step_check_connection as step,  # noqa: F401
)
