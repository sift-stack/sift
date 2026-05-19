"""Tests for ``--sift-offline`` mode.

Offline mode routes every create/update through the JSONL log file without
contacting Sift. The session-start ping is skipped, the import worker is not
spawned, and missing ``SIFT_*`` env vars are tolerated (placeholders are
filled). Offline + ``--sift-test-results-log-file=none`` is rejected as a
usage error since the log file is the sole sink in this mode.
"""

from __future__ import annotations

from typing import TYPE_CHECKING, Callable

if TYPE_CHECKING:
    import pytest


class TestOfflineMode:
    def test_offline_runs_without_network(
        self,
        pytester: pytest.Pytester,
        clear_sift_env: None,
        write_plugin_conftest: Callable[[], None],
    ) -> None:
        """Offline mode constructs the client locally and never pings."""
        write_plugin_conftest()
        pytester.makepyfile(
            """
            def test_in_bounds(step):
                assert step.measure(name="v", value=5.0, bounds={"min": 4.8, "max": 5.2})

            def test_out_of_bounds(step):
                assert step.measure(name="v", value=10.0, bounds={"max": 5.2}) is False
            """
        )
        result = pytester.runpytest_subprocess("--sift-offline")
        result.assert_outcomes(passed=2)

    def test_log_file_none_incompatible_with_offline(
        self,
        pytester: pytest.Pytester,
        write_plugin_conftest: Callable[[], None],
    ) -> None:
        """``--sift-test-results-log-file=none`` + ``--sift-offline`` is a usage error."""
        write_plugin_conftest()
        pytester.makepyfile("def test_should_not_run(): pass")
        result = pytester.runpytest_subprocess(
            "--sift-offline", "--sift-test-results-log-file=none"
        )
        assert result.ret != 0
        combined = "\n".join(result.outlines + result.errlines)
        assert "incompatible with --sift-offline" in combined, combined
