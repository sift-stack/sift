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
        """Test creating RecoveryStrategyPy with retry only."""
        recovery_strategy = RecoveryStrategyPy.retry_only(RetryPolicyPy.default())
        assert recovery_strategy is not None

    def test_recovery_strategy_retry_with_backups(self):
        """Test creating RecoveryStrategyPy with disk backups."""
        from sift_stream_bindings import DiskBackupPolicyPy, RollingFilePolicyPy

        disk_policy = DiskBackupPolicyPy(
            backups_dir="test_backups",
            max_backup_file_size=1024 * 1024,
            rolling_file_policy=RollingFilePolicyPy(max_file_count=5),
            retain_backups=True,
        )
        recovery_strategy = RecoveryStrategyPy.retry_with_backups(
            RetryPolicyPy.default(), disk_policy
        )
        assert recovery_strategy is not None

    def test_recovery_strategy_default(self):
        """Test creating default RecoveryStrategyPy."""
        recovery_strategy = RecoveryStrategyPy.default()
        assert recovery_strategy is not None

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
