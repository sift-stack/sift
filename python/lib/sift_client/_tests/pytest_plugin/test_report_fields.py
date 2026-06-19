"""Tests for [tool.sift.pytest.report] and the report-content env-var overrides.

Report-content fields are configured under ``[tool.sift.pytest.report]`` in
pyproject.toml and overridden per-run via ``SIFT_REPORT_*`` env vars. These
tests drive offline-mode inner sessions and inspect the JSONL
``CreateTestReport`` line, which serializes every report field with its proto
type intact.
"""

from __future__ import annotations

import json
from typing import TYPE_CHECKING, Callable

from google.protobuf import json_format
from sift.metadata.v1.metadata_pb2 import MetadataValue

from sift_client._tests.pytest_plugin._step_status_capture import run_jsonl
from sift_client.util.metadata import metadata_proto_to_dict

if TYPE_CHECKING:
    from pathlib import Path

    import pytest


def _create_report_dict(log_text: str) -> dict:
    """Parse the JSON payload from the ``[CreateTestReport:...]`` log line."""
    for line in log_text.splitlines():
        if line.startswith("[CreateTestReport:"):
            return json.loads(line[line.index("{") :])
    raise AssertionError(f"no CreateTestReport line in log:\n{log_text}")


def _metadata_pairs(report: dict) -> dict[str, str | float | bool]:
    """Unwrap the report's JSON metadata map into a ``{key: value}`` dict.

    Each entry is the JSON form of a ``MetadataValue`` proto, so parse it back
    into the proto and reuse the canonical ``metadata_proto_to_dict`` converter
    rather than hand-walking the value slots.
    """
    protos = [json_format.ParseDict(entry, MetadataValue()) for entry in report.get("metadata", [])]
    return metadata_proto_to_dict(protos)


class TestReportFields:
    def test_toml_resolves_every_field(
        self,
        pytester: pytest.Pytester,
        tmp_path: Path,
        clear_sift_env: None,
        write_plugin_conftest: Callable[[], None],
    ) -> None:
        """Every report-content field resolves from ``[tool.sift.pytest.report]``."""
        out_dir = tmp_path / "sift-out"
        write_plugin_conftest()
        pytester.makepyprojecttoml(
            """
            [tool.sift.pytest.report]
            test_case        = "case-from-toml"
            test_system_name = "rig-7"
            system_operator  = "ci-bot"
            serial_number    = "SN-001"
            part_number      = "PN-9000"
            """
        )
        pytester.makepyfile("def test_one(step): pass")
        result = pytester.runpytest_subprocess("--sift-offline", f"--sift-output-dir={out_dir}")
        log_path = run_jsonl(out_dir)
        result.assert_outcomes(passed=1)
        report = _create_report_dict(log_path.read_text())
        assert report["testCase"] == "case-from-toml"
        assert report["testSystemName"] == "rig-7"
        assert report["systemOperator"] == "ci-bot"
        assert report["serialNumber"] == "SN-001"
        assert report["partNumber"] == "PN-9000"

    def test_test_case_template_renders(
        self,
        pytester: pytest.Pytester,
        tmp_path: Path,
        clear_sift_env: None,
        write_plugin_conftest: Callable[[], None],
    ) -> None:
        """``test_case`` accepts the same template placeholders as ``name``."""
        out_dir = tmp_path / "sift-out"
        write_plugin_conftest()
        pytester.makepyprojecttoml(
            """
            [tool.sift.pytest.report]
            test_case = "case-{rootdir}-{count}"
            """
        )
        pytester.makepyfile("def test_one(step): pass")
        result = pytester.runpytest_subprocess("--sift-offline", f"--sift-output-dir={out_dir}")
        log_path = run_jsonl(out_dir)
        result.assert_outcomes(passed=1)
        report = _create_report_dict(log_path.read_text())
        assert report["testCase"].startswith("case-"), report["testCase"]
        assert report["testCase"].endswith("-1"), report["testCase"]

    def test_default_target_single_test_is_function(
        self,
        pytester: pytest.Pytester,
        tmp_path: Path,
        clear_sift_env: None,
        write_plugin_conftest: Callable[[], None],
    ) -> None:
        """One test collected -> default test_case is the project-anchored function nodeid.

        Derivation is from the collected items, so it doesn't depend on flag
        order or which path form was typed; the value is anchored to the
        rootdir (project) name.
        """
        out_dir = tmp_path / "sift-out"
        write_plugin_conftest()
        pytester.makepyfile(test_demo="def test_one(step): pass")
        result = pytester.runpytest_subprocess("--sift-offline", f"--sift-output-dir={out_dir}")
        log_path = run_jsonl(out_dir)
        result.assert_outcomes(passed=1)
        report = _create_report_dict(log_path.read_text())
        assert report["testCase"] == f"{pytester.path.name}/test_demo.py::test_one", report[
            "testCase"
        ]

    def test_default_target_single_test_strips_param(
        self,
        pytester: pytest.Pytester,
        tmp_path: Path,
        clear_sift_env: None,
        write_plugin_conftest: Callable[[], None],
    ) -> None:
        """A parametrized single test drops the ``[param]`` suffix from the key."""
        out_dir = tmp_path / "sift-out"
        write_plugin_conftest()
        pytester.makepyfile(
            test_demo=(
                "import pytest\n@pytest.mark.parametrize('v', [12])\ndef test_p(step, v): pass\n"
            )
        )
        result = pytester.runpytest_subprocess("--sift-offline", f"--sift-output-dir={out_dir}")
        log_path = run_jsonl(out_dir)
        result.assert_outcomes(passed=1)
        report = _create_report_dict(log_path.read_text())
        assert report["testCase"] == f"{pytester.path.name}/test_demo.py::test_p", report[
            "testCase"
        ]

    def test_default_target_single_file(
        self,
        pytester: pytest.Pytester,
        tmp_path: Path,
        clear_sift_env: None,
        write_plugin_conftest: Callable[[], None],
    ) -> None:
        """Multiple tests in one file -> the default target is that file (anchored)."""
        out_dir = tmp_path / "sift-out"
        write_plugin_conftest()
        pytester.makepyfile(test_demo="def test_a(step): pass\ndef test_b(step): pass")
        result = pytester.runpytest_subprocess("--sift-offline", f"--sift-output-dir={out_dir}")
        log_path = run_jsonl(out_dir)
        result.assert_outcomes(passed=2)
        report = _create_report_dict(log_path.read_text())
        assert report["testCase"] == f"{pytester.path.name}/test_demo.py", report["testCase"]

    def test_default_target_multiple_files_common_dir(
        self,
        pytester: pytest.Pytester,
        tmp_path: Path,
        clear_sift_env: None,
        write_plugin_conftest: Callable[[], None],
    ) -> None:
        """Tests across several files -> the default target is their common directory (anchored)."""
        out_dir = tmp_path / "sift-out"
        write_plugin_conftest()
        suite = pytester.mkdir("suite")
        (suite / "test_a.py").write_text("def test_a(step): pass\n")
        (suite / "test_b.py").write_text("def test_b(step): pass\n")
        result = pytester.runpytest_subprocess("--sift-offline", f"--sift-output-dir={out_dir}")
        log_path = run_jsonl(out_dir)
        result.assert_outcomes(passed=2)
        report = _create_report_dict(log_path.read_text())
        assert report["testCase"] == f"{pytester.path.name}/suite", report["testCase"]

    def test_default_target_whole_tree_is_project(
        self,
        pytester: pytest.Pytester,
        tmp_path: Path,
        clear_sift_env: None,
        write_plugin_conftest: Callable[[], None],
    ) -> None:
        """Tests spanning the rootdir -> the default target is the bare project name."""
        out_dir = tmp_path / "sift-out"
        write_plugin_conftest()
        # Two files directly under rootdir -> common path is rootdir itself.
        pytester.makepyfile(test_a="def test_a(step): pass", test_b="def test_b(step): pass")
        result = pytester.runpytest_subprocess("--sift-offline", f"--sift-output-dir={out_dir}")
        log_path = run_jsonl(out_dir)
        result.assert_outcomes(passed=2)
        report = _create_report_dict(log_path.read_text())
        assert report["testCase"] == pytester.path.name, report["testCase"]

    def test_env_overrides_toml(
        self,
        pytester: pytest.Pytester,
        tmp_path: Path,
        monkeypatch: pytest.MonkeyPatch,
        clear_sift_env: None,
        write_plugin_conftest: Callable[[], None],
    ) -> None:
        """An env var wins over a value set in ``[tool.sift.pytest.report]``."""
        out_dir = tmp_path / "sift-out"
        monkeypatch.setenv("SIFT_REPORT_SYSTEM_OPERATOR", "env-wins")
        write_plugin_conftest()
        pytester.makepyprojecttoml(
            """
            [tool.sift.pytest.report]
            system_operator = "ci-bot"
            """
        )
        pytester.makepyfile("def test_one(step): pass")
        result = pytester.runpytest_subprocess("--sift-offline", f"--sift-output-dir={out_dir}")
        log_path = run_jsonl(out_dir)
        result.assert_outcomes(passed=1)
        report = _create_report_dict(log_path.read_text())
        assert report["systemOperator"] == "env-wins"

    def test_metadata_table_typed_values(
        self,
        pytester: pytest.Pytester,
        tmp_path: Path,
        clear_sift_env: None,
        write_plugin_conftest: Callable[[], None],
    ) -> None:
        """``[tool.sift.pytest.report.metadata]`` keeps TOML types end-to-end."""
        out_dir = tmp_path / "sift-out"
        write_plugin_conftest()
        pytester.makepyprojecttoml(
            """
            [tool.sift.pytest.report.metadata]
            build_id = "v1.2.3"
            lane     = 2
            verbose  = true
            """
        )
        pytester.makepyfile("def test_one(step): pass")
        result = pytester.runpytest_subprocess("--sift-offline", f"--sift-output-dir={out_dir}")
        log_path = run_jsonl(out_dir)
        result.assert_outcomes(passed=1)
        pairs = _metadata_pairs(_create_report_dict(log_path.read_text()))
        assert pairs.get("build_id") == "v1.2.3"
        # Ints and floats share the proto's numeric slot.
        assert pairs.get("lane") == 2
        assert pairs.get("verbose") is True
        # Auto-recorded keys still present alongside the typed entries.
        assert "pytest_command" in pairs

    def test_loader_warns_on_bad_toml(
        self,
        tmp_path: Path,
        recwarn: pytest.WarningsRecorder,
    ) -> None:
        """A malformed pyproject.toml emits a warning and the loader returns ``{}``.

        pytest itself aborts the session when its own ``pyproject.toml`` is
        unparseable, so the loader's graceful warning path only matters when
        the file is reachable via the loader's own discovery (e.g. an upward
        walk in a monorepo). Exercise the loader directly here.
        """
        from types import SimpleNamespace

        from sift_client._internal.pyproject_config import load_tool_sift

        bad = tmp_path / "pyproject.toml"
        bad.write_text('[tool.sift]\ngrpc_uri = "unterminated\n')
        fake_config = SimpleNamespace(inipath=bad, rootpath=tmp_path)

        result = load_tool_sift(fake_config)  # type: ignore[arg-type]

        assert result == {}
        messages = [str(w.message) for w in recwarn.list]
        assert any("[tool.sift]" in m and "Failed to read" in m for m in messages), messages
