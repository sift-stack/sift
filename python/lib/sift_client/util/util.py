from __future__ import annotations

from typing import TYPE_CHECKING, NamedTuple

if TYPE_CHECKING:
    from sift_client.resources import (
        AssetsAPIAsync,
        CalculatedChannelsAPIAsync,
        ChannelsAPIAsync,
        IngestionAPIAsync,
        PingAPIAsync,
        RulesAPIAsync,
        RunsAPIAsync,
        TestResultsAPIAsync,
    )


class AsyncAPIs(NamedTuple):
    """Simple accessor for the asynchronous APIs, still uses the SiftClient instance."""

    ping: PingAPIAsync
    """Instance of the Ping API for making asynchronous requests."""

    assets: AssetsAPIAsync
    """Instance of the Assets API for making asynchronous requests."""

    calculated_channels: CalculatedChannelsAPIAsync
    """Instance of the Calculated Channels API for making asynchronous requests."""

    channels: ChannelsAPIAsync
    """Instance of the Channels API for making asynchronous requests."""

    ingestion: IngestionAPIAsync
    """Instance of the Ingestion API for making asynchronous requests."""

    runs: RunsAPIAsync
    """Instance of the Runs API for making asynchronous requests."""

    rules: RulesAPIAsync
    """Instance of the Rules API for making asynchronous requests."""

    test_results: TestResultsAPIAsync
    """Instance of the Test Results API for making asynchronous requests."""
