"""Tests for how `_build_sift_stream_instance` configures the SiftStream builders."""

from datetime import timedelta
from unittest.mock import AsyncMock, MagicMock

import pytest

from sift_client._internal.low_level_wrappers.ingestion import _build_sift_stream_instance
from sift_client.resources.ingestion import StreamingMode

sift_stream_bindings = pytest.importorskip("sift_stream_bindings")

MODE_BUILDER_METHODS = {
    StreamingMode.LIVE_ONLY: "live_only",
    StreamingMode.LIVE_WITH_BACKUPS: "live_with_backups",
    StreamingMode.FILE_BACKUP: "file_backup",
}


@pytest.fixture
def config_builder(monkeypatch):
    """Replace `SiftStreamBuilderPy` with a mock and return the config builder it produces."""
    sift_builder = MagicMock()
    config_builder = sift_builder.ingestion_config.return_value
    for method in MODE_BUILDER_METHODS.values():
        getattr(config_builder, method).return_value.build = AsyncMock()

    monkeypatch.setattr(
        sift_stream_bindings, "SiftStreamBuilderPy", MagicMock(return_value=sift_builder)
    )
    monkeypatch.setattr(sift_stream_bindings, "is_tracing_initialized", lambda: True)
    return config_builder


async def _build(**kwargs):
    await _build_sift_stream_instance(
        api_key="api-key",
        grpc_uri="localhost:50051",
        ingestion_config_form=MagicMock(),
        **kwargs,
    )


class TestStreamingMode:
    @pytest.mark.asyncio
    async def test_defaults_to_live_only(self, config_builder):
        await _build()

        config_builder.live_only.return_value.build.assert_awaited_once()
        config_builder.live_with_backups.assert_not_called()
        config_builder.file_backup.assert_not_called()

    @pytest.mark.asyncio
    async def test_backups_are_explicit_opt_in(self, config_builder):
        backup_policy = sift_stream_bindings.DiskBackupPolicyPy.default()
        retry_policy = sift_stream_bindings.RetryPolicyPy.default()

        await _build(
            streaming_mode=StreamingMode.LIVE_WITH_BACKUPS,
            disk_backup_policy=backup_policy,
            retry_policy=retry_policy,
            checkpoint_interval_seconds=30,
        )

        mode_builder = config_builder.live_with_backups.return_value
        mode_builder.build.assert_awaited_once()
        assert mode_builder.disk_backup_policy is backup_policy
        assert mode_builder.retry_policy is retry_policy
        assert mode_builder.checkpoint_interval.to_timedelta() == timedelta(seconds=30)
        config_builder.live_only.assert_not_called()
        config_builder.file_backup.assert_not_called()


@pytest.mark.parametrize("streaming_mode", list(MODE_BUILDER_METHODS))
class TestMetricsStreamingInterval:
    @pytest.mark.asyncio
    async def test_defaults_to_500_ms(self, config_builder, streaming_mode):
        await _build(streaming_mode=streaming_mode)

        mode_builder = getattr(config_builder, MODE_BUILDER_METHODS[streaming_mode]).return_value
        assert mode_builder.metrics_streaming_interval.to_timedelta() == timedelta(milliseconds=500)
        mode_builder.build.assert_awaited_once()

    @pytest.mark.asyncio
    async def test_custom_interval(self, config_builder, streaming_mode):
        await _build(
            streaming_mode=streaming_mode,
            metrics_streaming_interval=timedelta(seconds=2, milliseconds=250),
        )

        mode_builder = getattr(config_builder, MODE_BUILDER_METHODS[streaming_mode]).return_value
        assert mode_builder.metrics_streaming_interval.to_timedelta() == timedelta(
            seconds=2, milliseconds=250
        )

    @pytest.mark.asyncio
    async def test_none_disables_metrics_streaming(self, config_builder, streaming_mode):
        await _build(streaming_mode=streaming_mode, metrics_streaming_interval=None)

        mode_builder = getattr(config_builder, MODE_BUILDER_METHODS[streaming_mode]).return_value
        assert mode_builder.metrics_streaming_interval is None
        mode_builder.build.assert_awaited_once()

    @pytest.mark.asyncio
    @pytest.mark.parametrize("interval", [timedelta(0), timedelta(seconds=-1)])
    async def test_rejects_non_positive_interval(self, config_builder, streaming_mode, interval):
        with pytest.raises(ValueError, match="greater than zero"):
            await _build(streaming_mode=streaming_mode, metrics_streaming_interval=interval)

        mode_builder = getattr(config_builder, MODE_BUILDER_METHODS[streaming_mode]).return_value
        mode_builder.build.assert_not_awaited()


class TestRateLimitPolicy:
    @pytest.mark.asyncio
    async def test_sets_policy_on_config_builder(self, config_builder):
        policy = sift_stream_bindings.RateLimitPolicyPy.return_error()

        await _build(rate_limit_policy=policy)

        assert config_builder.rate_limit_policy is policy

    @pytest.mark.asyncio
    async def test_leaves_default_when_none(self, config_builder):
        await _build()

        assert not isinstance(
            config_builder.rate_limit_policy, sift_stream_bindings.RateLimitPolicyPy
        )
