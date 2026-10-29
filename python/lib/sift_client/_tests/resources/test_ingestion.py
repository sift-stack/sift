"""Tests for streaming client configuration and rate-limit error propagation."""

from datetime import timedelta
from unittest.mock import AsyncMock, MagicMock

import pytest

from sift_client.resources.ingestion import (
    AutoRegisterStreamingClient,
    IngestionAPIAsync,
    IngestionConfigStreamingClient,
    StreamingMode,
)

sift_stream_bindings = pytest.importorskip("sift_stream_bindings")


@pytest.mark.asyncio
@pytest.mark.parametrize(
    "factory_name",
    ["create_ingestion_config_streaming_client", "create_auto_register_streaming_client"],
)
@pytest.mark.parametrize("streaming_mode", [None, StreamingMode.LIVE_WITH_BACKUPS])
async def test_streaming_mode_default_and_opt_in(
    mock_client, monkeypatch, factory_name, streaming_mode
):
    build_stream = AsyncMock()
    monkeypatch.setattr(
        "sift_client._internal.low_level_wrappers.ingestion._build_sift_stream_instance",
        build_stream,
    )
    monkeypatch.setattr("sift_client.resources.ingestion._build_sift_stream_instance", build_stream)
    monkeypatch.setattr(
        sift_stream_bindings,
        "SiftStreamAutoRegisterPy",
        MagicMock(from_stream=AsyncMock()),
    )
    api = IngestionAPIAsync(mock_client)
    kwargs = {} if streaming_mode is None else {"streaming_mode": streaming_mode}

    await getattr(api, factory_name)(ingestion_config=MagicMock(), **kwargs)

    build_stream.assert_awaited_once()
    assert build_stream.call_args.kwargs["streaming_mode"] == (
        StreamingMode.LIVE_ONLY if streaming_mode is None else streaming_mode
    )


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
