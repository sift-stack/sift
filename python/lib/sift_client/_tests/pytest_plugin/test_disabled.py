"""Tests for ``--sift-disabled`` mode.

Disabled mode skips Sift entirely. Autouse fixtures yield stub objects so
test code that calls ``step.measure(...)`` keeps working without any Sift
configuration; ``measure*`` evaluates bounds locally and returns the real
pass/fail boolean. Nothing reaches Sift and no log file is written.
"""

from __future__ import annotations

from typing import TYPE_CHECKING, Callable

if TYPE_CHECKING:
    from pathlib import Path

    import pytest


class TestDisabledMode:
    def test_in_bounds_passes_out_of_bounds_fails(
        self,
        pytester: pytest.Pytester,
        clear_sift_env: None,
        write_plugin_conftest: Callable[[], None],
    ) -> None:
        """Stub measure* evaluates bounds locally; pass/fail matches the real plugin."""
        write_plugin_conftest()
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
        result = pytester.runpytest_subprocess("--sift-disabled")
        result.assert_outcomes(passed=6)

    def test_disabled_does_not_require_credentials(
        self,
        pytester: pytest.Pytester,
        clear_sift_env: None,
        write_plugin_conftest: Callable[[], None],
    ) -> None:
        """Disabled mode never reads SIFT_* env vars; runs cleanly without them."""
        write_plugin_conftest()
        pytester.makepyfile("def test_runs(step): step.measure(name='v', value=1.0)")
        result = pytester.runpytest_subprocess("--sift-disabled")
        result.assert_outcomes(passed=1)

    def test_disabled_via_env_var(
        self,
        pytester: pytest.Pytester,
        clear_sift_env: None,
        write_plugin_conftest: Callable[[], None],
        monkeypatch: pytest.MonkeyPatch,
    ) -> None:
        """``SIFT_DISABLED=1`` triggers disabled mode without the CLI flag."""
        write_plugin_conftest()
        pytester.makepyfile("def test_runs(step): step.measure(name='v', value=1.0)")
        monkeypatch.setenv("SIFT_DISABLED", "1")
        result = pytester.runpytest_subprocess()
        result.assert_outcomes(passed=1)

    def test_disabled_supersedes_offline(
        self,
        pytester: pytest.Pytester,
        clear_sift_env: None,
        write_plugin_conftest: Callable[[], None],
    ) -> None:
        """``--sift-disabled`` wins when combined with ``--sift-offline``.

        Disabled is the "skip Sift entirely" hammer; passing it alongside
        offline shouldn't error. The session runs without credentials, without
        a log file, and without the offline-mode replay machinery.
        """
        write_plugin_conftest()
        pytester.makepyfile(
            """
            def test_runs(step):
                assert step.measure(name="v", value=5.0, bounds={"max": 10.0}) is True
            """
        )
        result = pytester.runpytest_subprocess("--sift-disabled", "--sift-offline")
        result.assert_outcomes(passed=1)

    def test_disabled_yields_stub_fixtures(
        self,
        pytester: pytest.Pytester,
        clear_sift_env: None,
        write_plugin_conftest: Callable[[], None],
    ) -> None:
        """`report_context` / `step` / `module_substep` are real instances backed by a simulate client."""
        write_plugin_conftest()
        pytester.makepyfile(
            """
            from sift_client.util.test_results import ReportContext
            from sift_client.util.test_results.context_manager import NewStep

            def test_types(step, report_context, module_substep):
                assert isinstance(report_context, ReportContext)
                assert report_context.is_simulated is True
                assert report_context.report.is_simulated is True
                assert step.current_step.is_simulated is True
                assert isinstance(step, NewStep)
                assert isinstance(module_substep, NewStep)
            """
        )
        result = pytester.runpytest_subprocess("--sift-disabled")
        result.assert_outcomes(passed=1)

    def test_disabled_writes_no_log_file_even_when_path_pinned(
        self,
        pytester: pytest.Pytester,
        tmp_path: Path,
        clear_sift_env: None,
        write_plugin_conftest: Callable[[], None],
    ) -> None:
        """Disabled mode skips the log-file pipeline even when a path is pinned."""
        log_path = tmp_path / "should-not-exist.jsonl"
        write_plugin_conftest()
        pytester.makepyfile("def test_runs(step): step.measure(name='v', value=1.0)")
        result = pytester.runpytest_subprocess("--sift-disabled", f"--sift-log-file={log_path}")
        result.assert_outcomes(passed=1)
        assert not log_path.exists(), f"log file unexpectedly created at {log_path}"

    def test_disabled_skips_client_has_connection_and_sift_client(
        self,
        pytester: pytest.Pytester,
        clear_sift_env: None,
    ) -> None:
        """Disabled mode never resolves ``client_has_connection`` or ``sift_client``.

        The plugin's ``report_context`` short-circuits to the stub before
        consulting either fixture. Overrides that raise on resolution stay
        un-triggered, so the inner test passes cleanly.
        """
        pytester.makeconftest(
            """
            import pytest

            pytest_plugins = ["sift_client.pytest_plugin"]


            @pytest.fixture(scope="session")
            def sift_client():
                raise AssertionError("sift_client should not resolve in disabled mode")


            @pytest.fixture(scope="session")
            def client_has_connection():
                raise AssertionError(
                    "client_has_connection should not resolve in disabled mode"
                )
            """
        )
        pytester.makepyfile(
            """
            def test_runs(step):
                assert step.measure(name="v", value=5.0, bounds={"max": 10.0}) is True
            """
        )
        result = pytester.runpytest_subprocess("--sift-disabled")
        result.assert_outcomes(passed=1)
