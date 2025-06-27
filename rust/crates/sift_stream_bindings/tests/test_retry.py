from sift_stream_bindings import (
    DurationPy,
    RecoveryStrategyPy,
    RetryPolicyPy,
)


class TestRetryComponents:
    """Test retry-related components."""

    def test_create_duration(self):
        """Test creating DurationPy."""
        duration = DurationPy(5, 0)
        assert duration is not None
        assert duration.secs == 5
        assert duration.nanos == 0

    def test_recovery_strategy_retry_only(self):
        """Test creating RecoveryStrategyPy."""
        recovery_strategy = RecoveryStrategyPy.retry_only(RetryPolicyPy.default())
        assert recovery_strategy is not None
        assert recovery_strategy.strategy_type == "RetryOnly"
        assert recovery_strategy.max_buffer_size is None
        assert recovery_strategy.backups_dir is None
        assert recovery_strategy.max_backups_file_size is None

    def test_recovery_strategy_retry_with_in_memory_backups(self):
        """Test creating RecoveryStrategyPy."""
        recovery_strategy = RecoveryStrategyPy.retry_with_in_memory_backups(
            RetryPolicyPy.default(), 100
        )
        assert recovery_strategy is not None
        assert recovery_strategy.strategy_type == "RetryWithInMemoryBackups"
        assert recovery_strategy.max_buffer_size == 100
        assert recovery_strategy.backups_dir is None
        assert recovery_strategy.max_backups_file_size is None

    def test_recovery_strategy_retry_with_disk_backups(self):
        """Test creating RecoveryStrategyPy."""
        recovery_strategy = RecoveryStrategyPy.retry_with_disk_backups(
            RetryPolicyPy.default(), "test_backups", 100
        )
        assert recovery_strategy is not None
        assert recovery_strategy.strategy_type == "RetryWithDiskBackups"
        assert recovery_strategy.backups_dir == "test_backups"
        assert recovery_strategy.max_backups_file_size == 100
        assert recovery_strategy.max_buffer_size is None

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
