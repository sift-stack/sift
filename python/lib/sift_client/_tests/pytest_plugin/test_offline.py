"""Tests for ``--sift-offline`` mode.

Offline mode routes every create/update through the JSONL log file without
contacting Sift. The session-start ping is skipped, the import worker is not
spawned, and missing ``SIFT_*`` env vars are tolerated (placeholders are
filled). Offline + ``--sift-log-file=none`` is rejected as a
usage error since the log file is the sole sink in this mode.
"""

from __future__ import annotations

from typing import TYPE_CHECKING, Callable

if TYPE_CHECKING:
    from pathlib import Path

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
        """``--sift-log-file=none`` + ``--sift-offline`` is a usage error."""
        write_plugin_conftest()
        pytester.makepyfile("def test_should_not_run(): pass")
        result = pytester.runpytest_subprocess("--sift-offline", "--sift-log-file=none")
        assert result.ret != 0
        combined = "\n".join(result.outlines + result.errlines)
        assert "incompatible with --sift-offline" in combined, combined

    def test_offline_yields_real_fixtures(
        self,
        pytester: pytest.Pytester,
        clear_sift_env: None,
        write_plugin_conftest: Callable[[], None],
    ) -> None:
        """Offline mode runs a real ReportContext; entities still report `is_simulated=True` because the log-file path synthesizes responses prior to replay."""
        write_plugin_conftest()
        pytester.makepyfile(
            """
            from sift_client.util.test_results import ReportContext
            from sift_client.util.test_results.context_manager import NewStep

            def test_types(step, report_context):
                assert isinstance(report_context, ReportContext)
                assert isinstance(step, NewStep)
                assert report_context.client._simulate is False
                # log-file mode synthesizes responses, so entities are flagged simulated.
                assert report_context.is_simulated is True
                assert step.current_step.is_simulated is True
            """
        )
        result = pytester.runpytest_subprocess("--sift-offline")
        result.assert_outcomes(passed=1)

    def test_offline_writes_jsonl_to_pinned_log_file(
        self,
        pytester: pytest.Pytester,
        tmp_path: Path,
        clear_sift_env: None,
        write_plugin_conftest: Callable[[], None],
    ) -> None:
        """Offline mode populates the pinned JSONL file with create/update entries."""
        log_path = tmp_path / "run.jsonl"
        write_plugin_conftest()
        pytester.makepyfile(
            """
            def test_one(step):
                assert step.measure(
                    name="v", value=5.0, bounds={"min": 4.8, "max": 5.2}
                ) is True
            """
        )
        result = pytester.runpytest_subprocess("--sift-offline", f"--sift-log-file={log_path}")
        result.assert_outcomes(passed=1)
        assert log_path.exists(), f"offline mode did not create {log_path}"
        content = log_path.read_text()
        assert content.strip(), "log file is empty"
        # Each non-empty line is ``[Operation:uuid] {json}``. A successful
        # session produces at least the report create + step create lines.
        lines = [line for line in content.splitlines() if line.strip()]
        assert any(line.startswith("[CreateTestReport:") for line in lines), content
        assert any(line.startswith("[CreateTestStep:") for line in lines), content

    def test_offline_skips_client_has_connection(
        self,
        pytester: pytest.Pytester,
        clear_sift_env: None,
    ) -> None:
        """Offline mode never resolves ``client_has_connection``.

        Override the fixture to raise on resolution. If the override is
        invoked, the session aborts. If it isn't, the inner test passes
        cleanly, which confirms the offline path skipped the ping check.
        """
        pytester.makeconftest(
            """
            import pytest

            pytest_plugins = ["sift_client.pytest_plugin"]


            @pytest.fixture(scope="session")
            def client_has_connection():
                raise AssertionError(
                    "client_has_connection should not resolve in offline mode"
                )
            """
        )
        pytester.makepyfile("def test_runs(step): pass")
        result = pytester.runpytest_subprocess("--sift-offline")
        result.assert_outcomes(passed=1)
