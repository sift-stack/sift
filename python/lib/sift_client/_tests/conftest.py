"""Shared pytest fixtures for all tests."""

import os
from unittest.mock import MagicMock

import pytest

from sift_client import SiftClient, SiftConnectionConfig
from sift_client.util.util import AsyncAPIs


@pytest.fixture(autouse=True)
def _isolate_default_disk_cache_path(monkeypatch, tmp_path):
    """Redirect ``ChannelCache.DEFAULT_DISK_PATH`` to a per-test tmp dir.

    The channel data disk cache is **opt-out** — any test that triggers the
    lazy ``DataLowLevelClient`` init through ``ChannelsAPIAsync`` would
    otherwise create the real ``/tmp/sift-channel-data-cache`` directory and
    leak state across runs. Redirecting the default to ``tmp_path`` keeps
    every test self-contained without each test having to know that the disk
    tier is on by default.

    The override deliberately preserves the ``sift-channel-data-cache``
    suffix so ``TestChannelCacheClearDisk::test_default_path_constant_under_tmp``
    keeps validating the real shape of the constant.

    Importing ``ChannelCache`` here pulls in pandas, but only once per
    session — fixture body still runs per-test, just the monkeypatch.
    """
    from sift_client._internal.low_level_wrappers.data import ChannelCache

    monkeypatch.setattr(
        ChannelCache,
        "DEFAULT_DISK_PATH",
        str(tmp_path / "sift-channel-data-cache"),
    )


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
    client.resource_attributes = MagicMock()
    client.principal_attributes = MagicMock()
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


def pytest_configure(config: pytest.Config) -> None:
    """Pick a Sift plugin mode based on whether integration tests are running.

    Integration runs (``-m integration``) stay online with the default
    log-file pipeline enabled so CI exercises the JSONL write + import
    worker replay path that production users hit. Every other run defaults
    to ``--sift-disabled`` so unit tests don't need credentials.
    """
    is_integration_run = "integration" in (config.option.markexpr or "")
    if not is_integration_run:
        config.option.sift_disabled = True
