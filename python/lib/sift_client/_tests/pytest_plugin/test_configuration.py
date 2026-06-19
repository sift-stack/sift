"""Tests for the plugin's CLI/ini configuration surface.

Covers flag parsing, ini-key resolution, CLI-over-ini precedence, the
defaults that apply when nothing is set, and the marker-based gate that
governs the autouse fixtures. Credentials are tested in
``test_credentials.py``.
"""

from __future__ import annotations

import textwrap
from typing import TYPE_CHECKING, Callable

from sift_client._internal.pytest_plugin.options import (
    GRPC_URI_OPTION,
    resolved_settings,
)

if TYPE_CHECKING:
    from pathlib import Path

    import pytest


class TestResolvedSettings:
    """The audit snapshot helpers report value + source and redact the API key."""

    def test_resolve_with_source_env(self, monkeypatch: pytest.MonkeyPatch) -> None:
        monkeypatch.setenv("SIFT_GRPC_URI", "grpc.example:443")
        assert GRPC_URI_OPTION.resolve_with_source(None) == ("grpc.example:443", "env")

    def test_unset_resolves_to_default(self, monkeypatch: pytest.MonkeyPatch) -> None:
        monkeypatch.delenv("SIFT_GRPC_URI", raising=False)
        assert GRPC_URI_OPTION.resolve_with_source(None) == (None, "default")

    def test_resolved_settings_redacts_api_key(self, monkeypatch: pytest.MonkeyPatch) -> None:
        monkeypatch.setenv("SIFT_API_KEY", "super-secret")
        rows = {name: (value, source) for name, value, source in resolved_settings(None)}
        assert rows["api_key"] == ("***", "env")

    def test_resolved_settings_unset_api_key_is_default(
        self, monkeypatch: pytest.MonkeyPatch
    ) -> None:
        monkeypatch.delenv("SIFT_API_KEY", raising=False)
        rows = {name: (value, source) for name, value, source in resolved_settings(None)}
        assert rows["api_key"] == (None, "default")


class TestIniConfiguration:
    """`addini` keys configure the plugin via pyproject.toml / pytest.ini."""

    def test_ini_log_file_disabled(
        self,
        pytester: pytest.Pytester,
        write_probe_conftest: Callable[[str], None],
    ) -> None:
        write_probe_conftest(
            """
            from sift_client._internal.pytest_plugin.report import log_file_enabled
            print("ENABLED:", log_file_enabled(config))
            """,
        )
        pytester.makepyprojecttoml(
            """
            [tool.pytest.ini_options]
            sift_log_file = false
            """
        )
        pytester.makepyfile("def test_noop(): pass")
        result = pytester.runpytest_subprocess("-s", "--co")
        result.stdout.fnmatch_lines(["ENABLED: False"])

    def test_python_false_disables_log_file(
        self,
        pytester: pytest.Pytester,
        write_probe_conftest: Callable[[str], None],
    ) -> None:
        """`config.option.sift_log_file = False` disables logging.

        Conftests use this pattern (see lib/sift_client/_tests/util/conftest.py)
        to opt their subtree out of log-file mode.
        """
        write_probe_conftest(
            """
            config.option.sift_log_file = False
            from sift_client._internal.pytest_plugin.report import log_file_enabled
            print("ENABLED:", log_file_enabled(config))
            """,
        )
        pytester.makepyfile("def test_noop(): pass")
        result = pytester.runpytest_subprocess("-s", "--co")
        result.stdout.fnmatch_lines(["ENABLED: False"])

    def test_ini_output_dir(
        self,
        pytester: pytest.Pytester,
        tmp_path: Path,
        write_probe_conftest: Callable[[str], None],
    ) -> None:
        out_dir = tmp_path / "artifacts"
        write_probe_conftest(
            """
            from sift_client._internal.pytest_plugin.options import OUTPUT_DIR_OPTION
            print("RESOLVED:", OUTPUT_DIR_OPTION.resolve(config))
            """,
        )
        pytester.makepyprojecttoml(
            f"""
            [tool.pytest.ini_options]
            sift_output_dir = "{out_dir}"
            """
        )
        pytester.makepyfile("def test_noop(): pass")
        result = pytester.runpytest_subprocess("-s", "--co")
        result.stdout.fnmatch_lines([f"RESOLVED: {out_dir}"])

    def test_ini_offline_true(
        self,
        pytester: pytest.Pytester,
        write_probe_conftest: Callable[[str], None],
    ) -> None:
        write_probe_conftest(
            """
            from sift_client._internal.pytest_plugin.modes import is_offline
            print("OFFLINE:", is_offline(config))
            """,
        )
        pytester.makepyprojecttoml(
            """
            [tool.pytest.ini_options]
            sift_offline = true
            """
        )
        pytester.makepyfile("def test_noop(): pass")
        result = pytester.runpytest_subprocess("-s", "--co")
        result.stdout.fnmatch_lines(["OFFLINE: True"])

    def test_ini_disabled_true(
        self,
        pytester: pytest.Pytester,
        write_probe_conftest: Callable[[str], None],
    ) -> None:
        write_probe_conftest(
            """
            from sift_client._internal.pytest_plugin.modes import is_disabled
            print("DISABLED:", is_disabled(config))
            """,
        )
        pytester.makepyprojecttoml(
            """
            [tool.pytest.ini_options]
            sift_disabled = true
            """
        )
        pytester.makepyfile("def test_noop(): pass")
        result = pytester.runpytest_subprocess("-s", "--co")
        result.stdout.fnmatch_lines(["DISABLED: True"])

    def test_ini_git_metadata_false(
        self,
        pytester: pytest.Pytester,
        write_probe_conftest: Callable[[str], None],
    ) -> None:
        write_probe_conftest(
            """
            print("INI_GIT:", config.getini("sift_git_metadata"))
            """,
        )
        pytester.makepyprojecttoml(
            """
            [tool.pytest.ini_options]
            sift_git_metadata = false
            """
        )
        pytester.makepyfile("def test_noop(): pass")
        result = pytester.runpytest_subprocess("-s", "--co")
        result.stdout.fnmatch_lines(["INI_GIT: False"])

    def test_cli_overrides_ini(
        self,
        pytester: pytest.Pytester,
        write_probe_conftest: Callable[[str], None],
    ) -> None:
        """A CLI flag takes precedence over the matching ini key."""
        write_probe_conftest(
            """
            from sift_client._internal.pytest_plugin.report import log_file_enabled
            print("ENABLED:", log_file_enabled(config))
            """,
        )
        pytester.makepyprojecttoml(
            """
            [tool.pytest.ini_options]
            sift_log_file = true
            """
        )
        pytester.makepyfile("def test_noop(): pass")
        result = pytester.runpytest_subprocess("-s", "--co", "--no-sift-log-file")
        result.stdout.fnmatch_lines(["ENABLED: False"])

    def test_cli_offline_flag(
        self,
        pytester: pytest.Pytester,
        write_probe_conftest: Callable[[str], None],
    ) -> None:
        """The ``--sift-offline`` CLI flag flips the resolver to True."""
        write_probe_conftest(
            """
            from sift_client._internal.pytest_plugin.modes import is_offline
            print("OFFLINE:", is_offline(config))
            """,
        )
        pytester.makepyfile("def test_noop(): pass")
        result = pytester.runpytest_subprocess("-s", "--co", "--sift-offline")
        result.stdout.fnmatch_lines(["OFFLINE: True"])

    def test_cli_disabled_flag(
        self,
        pytester: pytest.Pytester,
        write_probe_conftest: Callable[[str], None],
    ) -> None:
        """The ``--sift-disabled`` CLI flag flips the resolver to True."""
        write_probe_conftest(
            """
            from sift_client._internal.pytest_plugin.modes import is_disabled
            print("DISABLED:", is_disabled(config))
            """,
        )
        pytester.makepyfile("def test_noop(): pass")
        result = pytester.runpytest_subprocess("-s", "--co", "--sift-disabled")
        result.stdout.fnmatch_lines(["DISABLED: True"])

    def test_cli_no_git_metadata_flag(
        self,
        pytester: pytest.Pytester,
        write_probe_conftest: Callable[[str], None],
    ) -> None:
        """The ``--no-sift-git-metadata`` CLI flag flips git_metadata to False.

        Guards the negation flag's ``dest`` binding: the flag name doesn't match
        the ini key, so a broken ``dest`` would silently fall back to the ini
        default and pass every other test in this file.
        """
        write_probe_conftest(
            """
            print("CLI_GIT:", config.getoption("sift_git_metadata"))
            """,
        )
        pytester.makepyfile("def test_noop(): pass")
        result = pytester.runpytest_subprocess("-s", "--co", "--no-sift-git-metadata")
        result.stdout.fnmatch_lines(["CLI_GIT: False"])

    def test_defaults_when_neither_set(
        self,
        pytester: pytest.Pytester,
        write_probe_conftest: Callable[[str], None],
    ) -> None:
        write_probe_conftest(
            """
            from sift_client._internal.pytest_plugin.modes import is_disabled, is_offline
            from sift_client._internal.pytest_plugin.report import log_file_enabled
            print("ENABLED:", log_file_enabled(config))
            print("OFFLINE:", is_offline(config))
            print("DISABLED:", is_disabled(config))
            print("INI_GIT:", config.getini("sift_git_metadata"))
            """,
        )
        pytester.makepyfile("def test_noop(): pass")
        result = pytester.runpytest_subprocess("-s", "--co")
        result.stdout.fnmatch_lines(
            [
                "ENABLED: True",
                "OFFLINE: False",
                "DISABLED: False",
                "INI_GIT: True",
            ]
        )


# A session-scoped `report_context` stub for the autouse-gate tests. Overrides
# the plugin's real `report_context` so the inner pytest sessions don't try to
# talk to a Sift backend; the gate tests only need to observe whether `step`
# resolves to a real value or to None.
_GATE_INNER_CONFTEST = textwrap.dedent(
    """
    from unittest.mock import MagicMock

    import pytest

    pytest_plugins = ["sift_client.pytest_plugin"]


    @pytest.fixture(scope="session")
    def report_context():
        yield MagicMock()
    """
)


class TestAutouseGate:
    """`sift_include` / `sift_exclude` markers and the `sift_autouse` ini gate."""

    def test_default_ini_true_activates(self, pytester: pytest.Pytester) -> None:
        """Plugin default (ini absent) keeps the autouse fixtures active."""
        pytester.makeconftest(_GATE_INNER_CONFTEST)
        pytester.makepyfile(
            """
            def test_inner(step):
                assert step is not None
            """
        )
        result = pytester.runpytest_subprocess()
        result.assert_outcomes(passed=1)

    def test_default_ini_false_skips(self, pytester: pytest.Pytester) -> None:
        """`sift_autouse = false` makes the autouse fixtures no-op by default."""
        pytester.makeconftest(_GATE_INNER_CONFTEST)
        pytester.makepyprojecttoml(
            """
            [tool.pytest.ini_options]
            sift_autouse = false
            """
        )
        pytester.makepyfile(
            """
            def test_inner(step):
                assert step is None
            """
        )
        result = pytester.runpytest_subprocess()
        result.assert_outcomes(passed=1)

    def test_sift_include_marker_forces_on(self, pytester: pytest.Pytester) -> None:
        """`@pytest.mark.sift_include` overrides ini-false to enable the gate."""
        pytester.makeconftest(_GATE_INNER_CONFTEST)
        pytester.makepyprojecttoml(
            """
            [tool.pytest.ini_options]
            sift_autouse = false
            """
        )
        pytester.makepyfile(
            """
            import pytest

            @pytest.mark.sift_include
            def test_inner(step):
                assert step is not None
            """
        )
        result = pytester.runpytest_subprocess()
        result.assert_outcomes(passed=1)

    def test_sift_exclude_marker_forces_off(self, pytester: pytest.Pytester) -> None:
        """`@pytest.mark.sift_exclude` overrides ini-true to disable the gate."""
        pytester.makeconftest(_GATE_INNER_CONFTEST)
        pytester.makepyfile(
            """
            import pytest

            @pytest.mark.sift_exclude
            def test_inner(step):
                assert step is None
            """
        )
        result = pytester.runpytest_subprocess()
        result.assert_outcomes(passed=1)

    def test_exclude_beats_include(self, pytester: pytest.Pytester) -> None:
        """When both markers are present, `sift_exclude` wins (safer default)."""
        pytester.makeconftest(_GATE_INNER_CONFTEST)
        pytester.makepyfile(
            """
            import pytest

            @pytest.mark.sift_include
            @pytest.mark.sift_exclude
            def test_inner(step):
                assert step is None
            """
        )
        result = pytester.runpytest_subprocess()
        result.assert_outcomes(passed=1)

    def test_module_pytestmark_inherits(self, pytester: pytest.Pytester) -> None:
        """Module-level `pytestmark = pytest.mark.sift_include` covers every test in the module."""
        pytester.makeconftest(_GATE_INNER_CONFTEST)
        pytester.makepyprojecttoml(
            """
            [tool.pytest.ini_options]
            sift_autouse = false
            """
        )
        pytester.makepyfile(
            """
            import pytest

            pytestmark = pytest.mark.sift_include

            def test_inner_a(step):
                assert step is not None

            def test_inner_b(step):
                assert step is not None
            """
        )
        result = pytester.runpytest_subprocess()
        result.assert_outcomes(passed=2)

    def test_bulk_apply_via_conftest_hook(self, pytester: pytest.Pytester) -> None:
        """A subtree opts in via `pytest_collection_modifyitems`; siblings stay off.

        Regression test for this repo's wiring pattern: the project default is
        autouse-off, the integration subtree's conftest bulk-applies
        `sift_include`, and sibling subtrees remain disabled. Verifies the
        per-directory mechanism works in a single pytest invocation.
        """
        pytester.makeconftest(_GATE_INNER_CONFTEST)
        pytester.makepyprojecttoml(
            """
            [tool.pytest.ini_options]
            sift_autouse = false
            """
        )
        included = pytester.mkdir("included_subtree")
        (included / "conftest.py").write_text(
            textwrap.dedent(
                """
                from pathlib import Path

                import pytest

                _HERE = Path(__file__).parent


                def pytest_collection_modifyitems(config, items):
                    for item in items:
                        try:
                            item.path.relative_to(_HERE)
                        except ValueError:
                            continue
                        item.add_marker(pytest.mark.sift_include)
                """
            )
        )
        (included / "test_included.py").write_text(
            "def test_included(step):\n    assert step is not None\n"
        )
        untouched = pytester.mkdir("untouched_subtree")
        (untouched / "test_untouched.py").write_text(
            "def test_untouched(step):\n    assert step is None\n"
        )
        result = pytester.runpytest_subprocess()
        result.assert_outcomes(passed=2)
