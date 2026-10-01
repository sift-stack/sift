"""``archive_on_create``: four config surfaces, explicit false, and the log.

The plugin creates the report, then archives it. An explicit ``false`` from a
higher-precedence surface overrides a ``true`` from a lower one, so a shared
TOML default can archive dev runs while production sets the env var to false.
"""

from __future__ import annotations

from typing import TYPE_CHECKING, Callable
from unittest.mock import MagicMock

import pytest

from sift_client._internal.pytest_plugin.options import ARCHIVE_ON_CREATE_OPTION
from sift_client.errors import SiftWarning
from sift_client.pytest_plugin import SiftPytestPluginWarning

if TYPE_CHECKING:
    from pathlib import Path


def _print_archive_probe() -> str:
    return """
    from sift_client._internal.pytest_plugin.options import ARCHIVE_ON_CREATE_OPTION
    value, source = ARCHIVE_ON_CREATE_OPTION.resolve_with_source(config)
    print(f"ARCHIVE: {value} {source}")
    """


class TestArchiveOnCreateResolution:
    """Precedence is env, then CLI, then ini, then TOML. False is a real value."""

    def test_env_false_is_false_not_a_string(self, monkeypatch: pytest.MonkeyPatch) -> None:
        monkeypatch.setenv("SIFT_REPORT_ARCHIVE_ON_CREATE", "false")
        assert ARCHIVE_ON_CREATE_OPTION.resolve_with_source(None) == (False, "env")

    def test_env_true(self, monkeypatch: pytest.MonkeyPatch) -> None:
        monkeypatch.setenv("SIFT_REPORT_ARCHIVE_ON_CREATE", "true")
        assert ARCHIVE_ON_CREATE_OPTION.resolve_with_source(None) == (True, "env")

    def test_invalid_env_warns_and_stays_unset(self, monkeypatch: pytest.MonkeyPatch) -> None:
        monkeypatch.setenv("SIFT_REPORT_ARCHIVE_ON_CREATE", "maybe")
        with pytest.warns(SiftPytestPluginWarning, match="expected true or false"):
            assert ARCHIVE_ON_CREATE_OPTION.resolve_with_source(None) == (None, "default")

    def test_unset_is_default(
        self,
        pytester: pytest.Pytester,
        monkeypatch: pytest.MonkeyPatch,
        write_probe_conftest: Callable[[str], None],
    ) -> None:
        monkeypatch.delenv("SIFT_REPORT_ARCHIVE_ON_CREATE", raising=False)
        write_probe_conftest(_print_archive_probe())
        pytester.makepyfile("def test_noop(): pass")
        result = pytester.runpytest_subprocess("-s", "--co")
        result.stdout.fnmatch_lines(["ARCHIVE: None default"])

    def test_toml_true(
        self,
        pytester: pytest.Pytester,
        monkeypatch: pytest.MonkeyPatch,
        write_probe_conftest: Callable[[str], None],
    ) -> None:
        monkeypatch.delenv("SIFT_REPORT_ARCHIVE_ON_CREATE", raising=False)
        write_probe_conftest(_print_archive_probe())
        pytester.makepyprojecttoml(
            """
            [tool.sift.pytest.report]
            archive_on_create = true
            """
        )
        pytester.makepyfile("def test_noop(): pass")
        result = pytester.runpytest_subprocess("-s", "--co")
        result.stdout.fnmatch_lines(["ARCHIVE: True toml"])

    def test_ini_false_overrides_toml_true(
        self,
        pytester: pytest.Pytester,
        monkeypatch: pytest.MonkeyPatch,
        write_probe_conftest: Callable[[str], None],
    ) -> None:
        monkeypatch.delenv("SIFT_REPORT_ARCHIVE_ON_CREATE", raising=False)
        write_probe_conftest(_print_archive_probe())
        pytester.makepyprojecttoml(
            """
            [tool.pytest.ini_options]
            sift_archive_on_create = false

            [tool.sift.pytest.report]
            archive_on_create = true
            """
        )
        pytester.makepyfile("def test_noop(): pass")
        result = pytester.runpytest_subprocess("-s", "--co")
        result.stdout.fnmatch_lines(["ARCHIVE: False ini"])

    def test_cli_true_overrides_ini_false(
        self,
        pytester: pytest.Pytester,
        monkeypatch: pytest.MonkeyPatch,
        write_probe_conftest: Callable[[str], None],
    ) -> None:
        monkeypatch.delenv("SIFT_REPORT_ARCHIVE_ON_CREATE", raising=False)
        write_probe_conftest(_print_archive_probe())
        pytester.makepyprojecttoml(
            """
            [tool.pytest.ini_options]
            sift_archive_on_create = false
            """
        )
        pytester.makepyfile("def test_noop(): pass")
        result = pytester.runpytest_subprocess("-s", "--co", "--sift-archive-on-create")
        result.stdout.fnmatch_lines(["ARCHIVE: True cli"])

    def test_env_false_overrides_cli_true(
        self,
        pytester: pytest.Pytester,
        monkeypatch: pytest.MonkeyPatch,
        write_probe_conftest: Callable[[str], None],
    ) -> None:
        """Production can force false even when the flag and the TOML say true."""
        monkeypatch.setenv("SIFT_REPORT_ARCHIVE_ON_CREATE", "false")
        write_probe_conftest(_print_archive_probe())
        pytester.makepyprojecttoml(
            """
            [tool.sift.pytest.report]
            archive_on_create = true
            """
        )
        pytester.makepyfile("def test_noop(): pass")
        result = pytester.runpytest_subprocess("-s", "--co", "--sift-archive-on-create")
        result.stdout.fnmatch_lines(["ARCHIVE: False env"])


class TestArchiveOnCreateReport:
    """The setting archives after create, including through the offline log."""

    def test_archive_failure_warns_and_continues(self) -> None:
        from sift_client.util.test_results import ReportContext

        report = MagicMock()
        report.archive.side_effect = RuntimeError("down")
        client = MagicMock()
        client.test_results.create.return_value = report
        with pytest.warns(SiftWarning, match="Could not archive"):
            context = ReportContext(client, name="n", log_file=False, archive_on_create=True)
        assert context.report is report
        report.archive.assert_called_once()

    def test_offline_log_and_footer_record_archive(
        self,
        pytester: pytest.Pytester,
        tmp_path: Path,
        clear_sift_env: None,
        write_plugin_conftest: Callable[[], None],
    ) -> None:
        from sift_client._tests.pytest_plugin._step_status_capture import run_jsonl

        out_dir = tmp_path / "sift-out"
        write_plugin_conftest()
        pytester.makepyprojecttoml(
            """
            [tool.sift.pytest.report]
            archive_on_create = true
            """
        )
        pytester.makepyfile("def test_one(step): pass")
        result = pytester.runpytest_subprocess("--sift-offline", f"--sift-output-dir={out_dir}")
        result.assert_outcomes(passed=1)
        log_text = run_jsonl(out_dir).read_text()
        create_at = log_text.index("[CreateTestReport:")
        archive_at = log_text.index('"isArchived":true')
        assert create_at < archive_at
        result.stdout.fnmatch_lines(["*· archived*"])

    def test_env_false_skips_archive(
        self,
        pytester: pytest.Pytester,
        tmp_path: Path,
        clear_sift_env: None,
        monkeypatch: pytest.MonkeyPatch,
        write_plugin_conftest: Callable[[], None],
    ) -> None:
        from sift_client._tests.pytest_plugin._step_status_capture import run_jsonl

        monkeypatch.setenv("SIFT_REPORT_ARCHIVE_ON_CREATE", "false")
        out_dir = tmp_path / "sift-out"
        write_plugin_conftest()
        pytester.makepyprojecttoml(
            """
            [tool.sift.pytest.report]
            archive_on_create = true
            """
        )
        pytester.makepyfile("def test_one(step): pass")
        result = pytester.runpytest_subprocess("--sift-offline", f"--sift-output-dir={out_dir}")
        result.assert_outcomes(passed=1)
        log_text = run_jsonl(out_dir).read_text()
        assert '"isArchived":true' not in log_text
        result.stdout.no_fnmatch_line("*· archived*")
