"""Tests for the plugin's CLI/ini configuration surface.

Covers flag parsing, ini-key resolution, CLI-over-ini precedence, the
defaults that apply when nothing is set, and the marker-based gate that
governs the autouse fixtures. Credentials are tested in
``test_credentials.py``.
"""

from __future__ import annotations

import textwrap
from typing import TYPE_CHECKING, Callable

if TYPE_CHECKING:
    from pathlib import Path

    import pytest


class TestIniConfiguration:
    """`addini` keys configure the plugin via pyproject.toml / pytest.ini."""

    def test_ini_log_file_none(
        self,
        pytester: pytest.Pytester,
        write_probe_conftest: Callable[[str], None],
    ) -> None:
        write_probe_conftest(
            """
            from sift_client.pytest_plugin import _resolve_log_file
            print("RESOLVED:", _resolve_log_file(config))
            """,
        )
        pytester.makepyprojecttoml(
            """
            [tool.pytest.ini_options]
            sift_log_file = "none"
            """
        )
        pytester.makepyfile("def test_noop(): pass")
        result = pytester.runpytest_subprocess("-s", "--co")
        result.stdout.fnmatch_lines(["RESOLVED: None"])

    def test_python_false_disables_log_file(
        self,
        pytester: pytest.Pytester,
        write_probe_conftest: Callable[[str], None],
    ) -> None:
        """`config.option.sift_log_file = False` disables logging.

        Conftests use this pattern (see lib/sift_client/_tests/util/conftest.py)
        to opt their subtree out of log-file mode. Regression test for the
        resolver case where Python `False` was previously confused with `None`
        and silently kept the temp-file default.
        """
        write_probe_conftest(
            """
            config.option.sift_log_file = False
            from sift_client.pytest_plugin import _resolve_log_file
            print("RESOLVED:", _resolve_log_file(config))
            """,
        )
        pytester.makepyfile("def test_noop(): pass")
        result = pytester.runpytest_subprocess("-s", "--co")
        result.stdout.fnmatch_lines(["RESOLVED: None"])

    def test_ini_log_file_path(
        self,
        pytester: pytest.Pytester,
        tmp_path: Path,
        write_probe_conftest: Callable[[str], None],
    ) -> None:
        log_path = tmp_path / "sift-run.jsonl"
        write_probe_conftest(
            """
            from sift_client.pytest_plugin import _resolve_log_file
            print("RESOLVED:", _resolve_log_file(config))
            """,
        )
        pytester.makepyprojecttoml(
            f"""
            [tool.pytest.ini_options]
            sift_log_file = "{log_path}"
            """
        )
        pytester.makepyfile("def test_noop(): pass")
        result = pytester.runpytest_subprocess("-s", "--co")
        result.stdout.fnmatch_lines([f"RESOLVED: {log_path}"])

    def test_ini_offline_true(
        self,
        pytester: pytest.Pytester,
        write_probe_conftest: Callable[[str], None],
    ) -> None:
        write_probe_conftest(
            """
            from sift_client.pytest_plugin import _is_offline
            print("OFFLINE:", _is_offline(config))
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
            from sift_client.pytest_plugin import _is_disabled
            print("DISABLED:", _is_disabled(config))
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
        tmp_path: Path,
        write_probe_conftest: Callable[[str], None],
    ) -> None:
        """A CLI flag takes precedence over the matching ini key."""
        cli_path = tmp_path / "cli-wins.jsonl"
        write_probe_conftest(
            """
            from sift_client.pytest_plugin import _resolve_log_file
            print("RESOLVED:", _resolve_log_file(config))
            """,
        )
        pytester.makepyprojecttoml(
            """
            [tool.pytest.ini_options]
            sift_log_file = "none"
            """
        )
        pytester.makepyfile("def test_noop(): pass")
        result = pytester.runpytest_subprocess(
            "-s", "--co", f"--sift-log-file={cli_path}"
        )
        result.stdout.fnmatch_lines([f"RESOLVED: {cli_path}"])

    def test_cli_offline_flag(
        self,
        pytester: pytest.Pytester,
        write_probe_conftest: Callable[[str], None],
    ) -> None:
        """The ``--sift-offline`` CLI flag flips the resolver to True."""
        write_probe_conftest(
            """
            from sift_client.pytest_plugin import _is_offline
            print("OFFLINE:", _is_offline(config))
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
            from sift_client.pytest_plugin import _is_disabled
            print("DISABLED:", _is_disabled(config))
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
            from sift_client.pytest_plugin import (
                _is_disabled,
                _is_offline,
                _resolve_log_file,
            )
            print("RESOLVED:", _resolve_log_file(config))
            print("OFFLINE:", _is_offline(config))
            print("DISABLED:", _is_disabled(config))
            print("INI_GIT:", config.getini("sift_git_metadata"))
            """,
        )
        pytester.makepyfile("def test_noop(): pass")
        result = pytester.runpytest_subprocess("-s", "--co")
        result.stdout.fnmatch_lines(
            [
                "RESOLVED: True",
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
