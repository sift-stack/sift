import pytest


def pytest_addoption(parser: pytest.Parser) -> None:
    existing_options = [opt.names() for opt in parser._anonymous.options]
    # Flatten the list of lists into a single list of strings
    flat_options = [item for sublist in existing_options for item in sublist]
    if not any("--sift-test-results-log-file" in name for name in flat_options):
        parser.addoption("--sift-test-results-log-file", action="store_true", default="false")


def pytest_configure(config: pytest.Config) -> None:
    """Configure the pytest configuration to disable the Sift test results log file."""
    config.option.sift_test_results_log_file = False
