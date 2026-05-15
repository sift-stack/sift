"""Unit tests for the pytest plugin using pytester.

These tests run inner pytest sessions in isolated tmp directories so the host
session's plugin is unaffected. They cover the modes, error paths, and the
no-op sibling plugin without requiring a live Sift backend.
"""

from __future__ import annotations

import textwrap

import pytest

pytest_plugins = ["pytester"]


def _write_inner_conftest(pytester: pytest.Pytester, body: str) -> None:
    pytester.makeconftest(textwrap.dedent(body))


class TestOfflineMode:
    def test_offline_runs_without_network(
        self, pytester: pytest.Pytester, monkeypatch: pytest.MonkeyPatch
    ) -> None:
        """Offline mode constructs the client locally and never pings."""
        for name in ("SIFT_API_KEY", "SIFT_GRPC_URI", "SIFT_REST_URI"):
            monkeypatch.delenv(name, raising=False)
        _write_inner_conftest(
            pytester,
            """
            pytest_plugins = ["sift_client.pytest_plugin"]
            """,
        )
        pytester.makepyfile(
            """
            def test_in_bounds(step):
                assert step.measure(name="v", value=5.0, bounds={"min": 4.8, "max": 5.2})

            def test_out_of_bounds(step):
                assert step.measure(name="v", value=10.0, bounds={"max": 5.2}) is False
            """
        )
        result = pytester.runpytest("--sift-offline")
        result.assert_outcomes(passed=2)


class TestOnlineMode:
    def test_online_ping_failure_aborts(self, pytester: pytest.Pytester) -> None:
        """Online mode with an unreachable ping aborts the session via UsageError."""
        _write_inner_conftest(
            pytester,
            """
            import pytest
            from unittest.mock import MagicMock

            pytest_plugins = ["sift_client.pytest_plugin"]


            @pytest.fixture(scope="session")
            def sift_client():
                client = MagicMock()
                client.ping.ping.side_effect = ConnectionError("unreachable")
                return client
            """,
        )
        pytester.makepyfile(
            """
            def test_should_not_run():
                assert True
            """
        )
        result = pytester.runpytest()
        # UsageError surfaces as a non-zero exit; the test never runs.
        assert result.ret != 0
        combined = "\n".join(result.outlines + result.errlines)
        assert "Sift ping failed" in combined, combined


class TestBadConfig:
    def test_missing_env_vars_named_in_error(
        self, pytester: pytest.Pytester, monkeypatch: pytest.MonkeyPatch
    ) -> None:
        """The default sift_client fixture names missing env vars in its error."""
        for name in ("SIFT_API_KEY", "SIFT_GRPC_URI", "SIFT_REST_URI"):
            monkeypatch.delenv(name, raising=False)
        _write_inner_conftest(
            pytester,
            """
            pytest_plugins = ["sift_client.pytest_plugin"]
            """,
        )
        pytester.makepyfile(
            """
            def test_should_not_run():
                assert True
            """
        )
        result = pytester.runpytest()
        assert result.ret != 0
        combined = "\n".join(result.outlines + result.errlines)
        for var in ("SIFT_API_KEY", "SIFT_GRPC_URI", "SIFT_REST_URI"):
            assert var in combined, combined


class TestNoopPlugin:
    def test_in_bounds_passes_out_of_bounds_fails(
        self, pytester: pytest.Pytester
    ) -> None:
        """Shim measure* evaluates bounds locally; pass/fail matches the real plugin."""
        _write_inner_conftest(
            pytester,
            """
            pytest_plugins = ["sift_client.pytest_plugin_noop"]
            """,
        )
        pytester.makepyfile(
            """
            def test_passes_in_bounds(step):
                assert step.measure(name="v", value=5.0, bounds={"min": 4.8, "max": 5.2})

            def test_fails_out_of_bounds(step):
                assert step.measure(name="v", value=99.0, bounds={"max": 5.2}) is False

            def test_substep_and_report_outcome(step):
                with step.substep(name="inner") as inner:
                    assert inner.report_outcome(name="ok", result=True) is True

            def test_string_bounds(step):
                assert step.measure(name="fw", value="1.0", bounds="1.0") is True
                assert step.measure(name="fw", value="1.0", bounds="2.0") is False

            def test_measure_avg(step):
                assert step.measure_avg(
                    name="bus", values=[4.97, 5.01, 5.03], bounds={"min": 4.9, "max": 5.1}
                ) is True

            def test_measure_all_outlier(step):
                assert step.measure_all(
                    name="p", values=[10.1, 10.2, 99.9], bounds={"max": 11.0}
                ) is False
            """
        )
        result = pytester.runpytest()
        result.assert_outcomes(passed=6)

    def test_no_network_calls_in_noop(self, pytester: pytest.Pytester) -> None:
        """The noop plugin runs without ever loading the real Sift plugin module."""
        _write_inner_conftest(
            pytester,
            """
            pytest_plugins = ["sift_client.pytest_plugin_noop"]
            """,
        )
        pytester.makepyfile(
            """
            import sys

            def test_real_plugin_not_loaded():
                assert "sift_client.pytest_plugin" not in sys.modules
            """
        )
        # Run in a subprocess so the outer test runner's sys.modules don't
        # bleed in (the outer runner loads the real plugin via the test
        # conftest's `pytest_plugins` declaration).
        result = pytester.runpytest_subprocess()
        result.assert_outcomes(passed=1)


class TestLogFileFlag:
    def test_no_log_file_incompatible_with_offline(
        self, pytester: pytest.Pytester
    ) -> None:
        """--no-sift-log-file + --sift-offline is a usage error."""
        _write_inner_conftest(
            pytester,
            """
            import pytest
            from unittest.mock import MagicMock

            pytest_plugins = ["sift_client.pytest_plugin"]


            @pytest.fixture(scope="session")
            def sift_client():
                return MagicMock()
            """,
        )
        pytester.makepyfile(
            """
            def test_should_not_run():
                assert True
            """
        )
        result = pytester.runpytest("--sift-offline", "--no-sift-log-file")
        assert result.ret != 0
        combined = "\n".join(result.outlines + result.errlines)
        assert "incompatible with --sift-offline" in combined, combined
