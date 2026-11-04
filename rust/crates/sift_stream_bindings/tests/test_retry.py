from datetime import timedelta

import pytest
from sift_stream_bindings import (
    DurationPy,
    IngestionConfigFormPy,
    RateLimitedErrorPy,
    RateLimitPolicyPy,
    RetryPolicyPy,
    SiftStreamBuilderPy,
)


class TestRetryComponents:
    """Test retry-related components."""

    def test_create_duration(self):
        """Test creating DurationPy."""
        duration = DurationPy(5, 0)
        assert duration is not None
        assert duration.secs == 5
        assert duration.nanos == 0

    def test_duration_to_timedelta(self):
        """Test converting DurationPy to a timedelta, dropping sub-microsecond nanoseconds."""
        assert DurationPy(2, 500_000_000).to_timedelta() == timedelta(seconds=2, milliseconds=500)
        assert DurationPy(0, 0).to_timedelta() == timedelta(0)
        assert DurationPy(0, 1_999).to_timedelta() == timedelta(microseconds=1)

    def test_duration_to_timedelta_bounds(self):
        """Test that timedelta.max converts and anything longer raises OverflowError."""
        max_secs = timedelta.max.days * 86_400 + timedelta.max.seconds
        assert DurationPy(max_secs, 999_999_000).to_timedelta() == timedelta.max

        for secs in (max_secs + 1, 2**63):
            with pytest.raises(OverflowError):
                DurationPy(secs, 0).to_timedelta()

    def test_duration_from_timedelta(self):
        """Test creating DurationPy from a timedelta."""
        duration = DurationPy.from_timedelta(timedelta(days=1, seconds=3, microseconds=250))
        assert duration.secs == 86_403
        assert duration.nanos == 250_000

        duration = DurationPy.from_timedelta(timedelta(0))
        assert duration.secs == 0
        assert duration.nanos == 0

    def test_duration_from_negative_timedelta(self):
        """Test that a negative timedelta is rejected."""
        with pytest.raises(ValueError):
            DurationPy.from_timedelta(timedelta(seconds=-1))

    def test_create_retry_policy(self):
        """Test creating RetryPolicyPy."""
        retry_policy = RetryPolicyPy(10, DurationPy(1, 0), DurationPy(10, 0), 2)
        assert retry_policy is not None
        assert retry_policy.max_attempts == 10
        assert retry_policy.initial_backoff.secs == 1
        assert retry_policy.initial_backoff.nanos == 0
        assert retry_policy.max_backoff.secs == 10
        assert retry_policy.max_backoff.nanos == 0
        assert retry_policy.backoff_multiplier == 2

    def test_create_rate_limit_policies(self):
        """Test creating each RateLimitPolicyPy variant."""
        assert repr(RateLimitPolicyPy.retry()) == "RateLimitPolicyPy.retry()"
        assert repr(RateLimitPolicyPy.default()) == "RateLimitPolicyPy.retry()"
        assert repr(RateLimitPolicyPy.return_error()) == "RateLimitPolicyPy.return_error()"

    def test_rate_limit_policy_builder(self):
        """Test that non-default settings are kept and shown, and defaults are omitted."""
        policy = RateLimitPolicyPy.retry().with_max_delay(DurationPy(2, 500))
        assert repr(policy) == (
            "RateLimitPolicyPy.retry().with_max_delay(DurationPy(secs=2, nanos=500))"
        )

        policy = policy.with_margin(DurationPy(0, 250_000_000))
        assert repr(policy) == (
            "RateLimitPolicyPy.retry()"
            ".with_max_delay(DurationPy(secs=2, nanos=500))"
            ".with_margin(DurationPy(secs=0, nanos=250000000))"
        )

        # Setting the defaults explicitly matches the default policy.
        policy = (
            RateLimitPolicyPy.retry()
            .with_max_delay(DurationPy(10, 0))
            .with_margin(DurationPy(0, 100_000_000))
        )
        assert repr(policy) == "RateLimitPolicyPy.retry()"

        # The builder methods do not turn return_error into a retrying policy.
        policy = RateLimitPolicyPy.return_error().with_max_delay(DurationPy(30, 0))
        assert repr(policy) == "RateLimitPolicyPy.return_error()"

    def test_builders_carry_rate_limit_policy(self):
        """Test that the builders default to retry and carry a configured policy forward."""
        builder = SiftStreamBuilderPy("localhost:50051", "api-key")
        assert repr(builder.rate_limit_policy) == "RateLimitPolicyPy.retry()"

        builder.rate_limit_policy = RateLimitPolicyPy.return_error()
        assert repr(builder.rate_limit_policy) == "RateLimitPolicyPy.return_error()"

        config_builder = builder.ingestion_config(
            IngestionConfigFormPy("asset", "client-key", [])
        )
        assert repr(config_builder.rate_limit_policy) == "RateLimitPolicyPy.return_error()"

        config_builder.rate_limit_policy = RateLimitPolicyPy.retry().with_max_delay(
            DurationPy(1, 0)
        )
        assert repr(config_builder.rate_limit_policy) == (
            "RateLimitPolicyPy.retry().with_max_delay(DurationPy(secs=1, nanos=0))"
        )

    def test_rate_limited_error(self):
        """Test that RateLimitedErrorPy carries the delay and is caught as a RuntimeError."""
        message = "[RateLimitedError] rate limited; retry after 2s"
        with pytest.raises(RuntimeError) as exc_info:
            raise RateLimitedErrorPy(message, DurationPy(2, 0))

        err = exc_info.value
        assert isinstance(err, RateLimitedErrorPy)
        assert str(err) == message
        assert err.retry_delay.secs == 2
        assert err.retry_delay.nanos == 0
        assert err.retry_delay.to_timedelta() == timedelta(seconds=2)

    def test_rate_limited_error_without_delay(self):
        """Test that RateLimitedErrorPy reports None when Sift provides no delay."""
        err = RateLimitedErrorPy("[RateLimitedError] rate limited")
        assert err.retry_delay is None
