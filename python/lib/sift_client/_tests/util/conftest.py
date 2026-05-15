import pytest


def pytest_configure(config: pytest.Config) -> None:
    """Configure the pytest configuration to disable the Sift test results log file."""
    config.option.sift_test_results_log_file = False
