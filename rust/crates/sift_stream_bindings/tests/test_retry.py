from sift_stream_bindings import (
    DurationPy,
    IngestionConfigFormPy,
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

        policy = RateLimitPolicyPy.retry_if_delay_at_most(DurationPy(2, 500))
        assert (
            repr(policy)
            == "RateLimitPolicyPy.retry_if_delay_at_most(DurationPy(secs=2, nanos=500))"
        )

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

        config_builder.rate_limit_policy = RateLimitPolicyPy.retry_if_delay_at_most(
            DurationPy(1, 0)
        )
        assert repr(config_builder.rate_limit_policy) == (
            "RateLimitPolicyPy.retry_if_delay_at_most(DurationPy(secs=1, nanos=0))"
        )
