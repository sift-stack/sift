from __future__ import annotations

from typing import NamedTuple

from sift_client.resources import (
    AssetsAPIAsync,
    CalculatedChannelsAPIAsync,
    PingAPIAsync,
    RunsAPIAsync,
)


class AsyncAPIs(NamedTuple):
    """Simple accessor for the asynchronous APIs, still uses the SiftClient instance."""

    ping: PingAPIAsync
    """Instance of the Ping API for making asynchronous requests."""

    assets: AssetsAPIAsync
    """Instance of the Assets API for making asynchronous requests."""

    calculated_channels: CalculatedChannelsAPIAsync
    """Instance of the Calculated Channels API for making asynchronous requests."""

    runs: RunsAPIAsync
    """Instance of the Runs API for making asynchronous requests."""
