"""Tests that rate-limit errors from SiftStream reach callers of the streaming clients unchanged."""

from datetime import timedelta
from unittest.mock import AsyncMock, MagicMock

import pytest

from sift_client.resources.ingestion import (
    AutoRegisterStreamingClient,
    IngestionConfigStreamingClient,
)

sift_stream_bindings = pytest.importorskip("sift_stream_bindings")


def _rate_limited_error():
    return sift_stream_bindings.RateLimitedErrorPy(
        "[RateLimitedError] rate limited",
        sift_stream_bindings.DurationPy.from_timedelta(timedelta(seconds=2, milliseconds=500)),
    )


class TestRateLimitedErrorPropagation:
    @pytest.mark.asyncio
    async def test_ingestion_config_client_attach_run(self):
        low_level_client = MagicMock()
        low_level_client.attach_run = AsyncMock(side_effect=_rate_limited_error())
        client = IngestionConfigStreamingClient(MagicMock(), low_level_client)

        with pytest.raises(sift_stream_bindings.RateLimitedErrorPy) as exc_info:
            await client.attach_run("run-id")

        assert exc_info.value.retry_delay.to_timedelta() == timedelta(seconds=2, milliseconds=500)

    @pytest.mark.asyncio
    async def test_auto_register_client_send(self):
        stream = MagicMock()
        stream.send = AsyncMock(side_effect=_rate_limited_error())
        client = AutoRegisterStreamingClient(MagicMock(), stream)

        with pytest.raises(sift_stream_bindings.RateLimitedErrorPy) as exc_info:
            await client.send(MagicMock())

        assert exc_info.value.retry_delay.to_timedelta() == timedelta(seconds=2, milliseconds=500)
