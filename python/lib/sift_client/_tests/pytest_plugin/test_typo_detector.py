"""Tests for the unknown-setting warnings fired in ``pytest_configure``.

The plugin scans ``SIFT_*`` env vars and ``[tool.sift.pytest.*]`` keys at
session start and emits a ``SiftPytestPluginWarning`` for anything not
declared in the central ``_OPTIONS`` registry. A typo (`SIFT_REPORT_SERIALNUM`
instead of `SIFT_REPORT_SERIAL_NUMBER`) would otherwise silently no-op.
"""

from __future__ import annotations

from typing import TYPE_CHECKING, Callable

if TYPE_CHECKING:
    import pytest


class TestTypoDetector:
    def test_unknown_env_var_warns(
        self,
        pytester: pytest.Pytester,
        clear_sift_env: None,
        monkeypatch: pytest.MonkeyPatch,
        write_plugin_conftest: Callable[[], None],
    ) -> None:
        """An unknown ``SIFT_*`` env var emits a warning with a closest-match hint."""
        monkeypatch.setenv("SIFT_REPORT_SERIALNUM", "SN-1")  # missing underscore
        write_plugin_conftest()
        pytester.makepyfile("def test_runs(): pass")
        result = pytester.runpytest_subprocess("--sift-disabled")
        combined = "\n".join(result.outlines + result.errlines)
        assert "Unknown SIFT_* env var `SIFT_REPORT_SERIALNUM`" in combined, combined
        assert "did you mean `SIFT_REPORT_SERIAL_NUMBER`" in combined, combined

    def test_known_env_var_silent(
        self,
        pytester: pytest.Pytester,
        clear_sift_env: None,
        monkeypatch: pytest.MonkeyPatch,
        write_plugin_conftest: Callable[[], None],
    ) -> None:
        """Declared env vars (full and prefix-matched) don't warn."""
        monkeypatch.setenv("SIFT_REPORT_SERIAL_NUMBER", "SN-1")
        monkeypatch.setenv("SIFT_REPORT_METADATA_BUILD_ID", "v1.2.3")
        write_plugin_conftest()
        pytester.makepyfile("def test_runs(): pass")
        result = pytester.runpytest_subprocess("--sift-disabled")
        combined = "\n".join(result.outlines + result.errlines)
        assert "Unknown SIFT_*" not in combined, combined

    def test_unknown_toml_key_warns(
        self,
        pytester: pytest.Pytester,
        clear_sift_env: None,
        write_plugin_conftest: Callable[[], None],
    ) -> None:
        """An unknown ``[tool.sift.pytest.report]`` key warns with a suggestion."""
        write_plugin_conftest()
        pytester.makepyprojecttoml(
            """
            [tool.sift.pytest.report]
            serial_numbr = "SN-1"
            """
        )
        pytester.makepyfile("def test_runs(): pass")
        result = pytester.runpytest_subprocess("--sift-disabled")
        combined = "\n".join(result.outlines + result.errlines)
        assert "Unknown sift config key" in combined, combined
        assert "pytest.report.serial_numbr" in combined, combined
        assert "did you mean" in combined, combined
        assert "serial_number" in combined, combined

    def test_unknown_toml_outside_pytest_scope_silent(
        self,
        pytester: pytest.Pytester,
        clear_sift_env: None,
        write_plugin_conftest: Callable[[], None],
    ) -> None:
        """``[tool.sift.X]`` outside ``tool.sift.pytest`` is not the plugin's concern.

        Other Sift tools may use ``tool.sift.<other-subtree>`` (the build-time
        ``[tool.sift.extras]`` in this repo's own pyproject is one example);
        the detector intentionally only walks ``tool.sift.pytest``.
        """
        write_plugin_conftest()
        pytester.makepyprojecttoml(
            """
            [tool.sift.something_else]
            anything = "goes"
            """
        )
        pytester.makepyfile("def test_runs(): pass")
        result = pytester.runpytest_subprocess("--sift-disabled")
        combined = "\n".join(result.outlines + result.errlines)
        assert "Unknown sift config key" not in combined, combined

    def test_metadata_subtree_keys_are_user_defined(
        self,
        pytester: pytest.Pytester,
        clear_sift_env: None,
        write_plugin_conftest: Callable[[], None],
    ) -> None:
        """Keys under ``[tool.sift.pytest.report.metadata]`` don't trigger warnings."""
        write_plugin_conftest()
        pytester.makepyprojecttoml(
            """
            [tool.sift.pytest.report.metadata]
            anything_at_all = "value"
            another_thing   = 42
            """
        )
        pytester.makepyfile("def test_runs(): pass")
        result = pytester.runpytest_subprocess("--sift-disabled")
        combined = "\n".join(result.outlines + result.errlines)
        assert "Unknown sift config key" not in combined, combined
