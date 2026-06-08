"""Read step status sequences from a Sift offline-mode log file.

The contract suite drives each scenario through an inner pytester session
run with ``--sift-offline``, which causes the real plugin + ``ReportContext``
to write every test-result API call to a JSONL log. This module parses
that log into a per-step status timeline that ``test_pass_fail.py`` asserts
against, with no test-only ``ReportContext`` fake required.
"""

from __future__ import annotations

import json
from dataclasses import dataclass, field
from typing import TYPE_CHECKING

from sift_client._internal.low_level_wrappers._test_results_log import iter_log_data_lines
from sift_client.sift_types.test_report import TestStatus

if TYPE_CHECKING:
    from pathlib import Path


@dataclass
class CapturedStep:
    step_id: str
    name: str
    step_path: str
    parent_step_id: str | None
    statuses: list[TestStatus] = field(default_factory=list)
    error_messages: list[str] = field(default_factory=list)
    # ``startTime`` from the create entry; ``endTime`` is the latest seen across
    # create/update entries. Both are RFC3339 strings.
    start_time: str | None = None
    end_time: str | None = None


_PROTO_STATUS_NAMES = {
    "TEST_STATUS_UNSPECIFIED": TestStatus.UNSPECIFIED,
    "TEST_STATUS_DRAFT": TestStatus.DRAFT,
    "TEST_STATUS_PASSED": TestStatus.PASSED,
    "TEST_STATUS_FAILED": TestStatus.FAILED,
    "TEST_STATUS_ABORTED": TestStatus.ABORTED,
    "TEST_STATUS_ERROR": TestStatus.ERROR,
    "TEST_STATUS_IN_PROGRESS": TestStatus.IN_PROGRESS,
    "TEST_STATUS_SKIPPED": TestStatus.SKIPPED,
}


def _status(name: str | None) -> TestStatus:
    if name is None:
        return TestStatus.UNSPECIFIED
    return _PROTO_STATUS_NAMES.get(name, TestStatus.UNSPECIFIED)


def parse_log(log_path: Path) -> dict[str, CapturedStep]:
    """Parse the offline log into ``{step_id: CapturedStep}``.

    Walks the JSONL file in order, building a ``CapturedStep`` for each
    ``CreateTestStep`` entry and appending the new status from each
    ``UpdateTestStep`` entry.
    """
    steps: dict[str, CapturedStep] = {}
    for request_type, response_id, json_str in iter_log_data_lines(log_path):
        payload = json.loads(json_str)
        test_step = payload.get("testStep", {})
        error_message = test_step.get("errorInfo", {}).get("errorMessage")
        if request_type == "CreateTestStep" and response_id:
            steps[response_id] = CapturedStep(
                step_id=response_id,
                name=test_step.get("name", ""),
                step_path=test_step.get("stepPath", ""),
                parent_step_id=test_step.get("parentStepId") or None,
                statuses=[_status(test_step.get("status"))],
                error_messages=[error_message] if error_message else [],
                start_time=test_step.get("startTime"),
                end_time=test_step.get("endTime"),
            )
        elif request_type == "UpdateTestStep":
            step_id = test_step.get("testStepId")
            new_status = test_step.get("status")
            if step_id and step_id in steps and new_status is not None:
                steps[step_id].statuses.append(_status(new_status))
                if error_message:
                    steps[step_id].error_messages.append(error_message)
                if test_step.get("endTime") is not None:
                    steps[step_id].end_time = test_step.get("endTime")
    return steps


_active_log: Path | None = None
_cached: dict[str, CapturedStep] | None = None


def set_log(path: Path) -> None:
    """Point subsequent queries at a new log file. Clears the parse cache."""
    global _active_log, _cached
    _active_log = path
    _cached = None


def _steps() -> dict[str, CapturedStep]:
    global _cached
    if _cached is None:
        if _active_log is None or not _active_log.exists():
            _cached = {}
        else:
            _cached = parse_log(_active_log)
    return _cached


def steps_by_name(name: str) -> list[CapturedStep]:
    return [s for s in _steps().values() if s.name == name]


def test_step(name: str) -> CapturedStep | None:
    """The step the autouse ``step`` fixture creates for the test function.

    Multiple steps can share a name (e.g. when the makereport hook records an
    inline step for a collection-time skip on top of the autouse step). The
    autouse step is the shallowest by path depth.
    """
    matches = steps_by_name(name)
    if not matches:
        return None
    return min(matches, key=lambda s: s.step_path.count("."))


def final_status(name: str) -> TestStatus | None:
    step = test_step(name)
    return step.statuses[-1] if step and step.statuses else None


def final_error_message(name: str) -> str | None:
    step = test_step(name)
    return step.error_messages[-1] if step and step.error_messages else None


def log_events(log_path: Path) -> list[tuple[str, str, TestStatus]]:
    """Ordered ``(request_type, step_name, status)`` tuples as they appear in the log.

    Unlike ``load_steps`` (which collapses each step to its final state), this
    preserves write order, so tests can assert *when* a step resolved relative to
    other entries — e.g. that a container's terminal ``UpdateTestStep`` precedes a
    later sibling's ``CreateTestStep`` (proof it closed mid-session, not at the
    end). ``UpdateTestStep`` entries carry only an id, so the name is resolved
    from the preceding ``CreateTestStep``.
    """
    if not log_path.exists():
        return []
    id_to_name: dict[str, str] = {}
    events: list[tuple[str, str, TestStatus]] = []
    for request_type, response_id, json_str in iter_log_data_lines(log_path):
        test_step = json.loads(json_str).get("testStep", {})
        status = _status(test_step.get("status"))
        if request_type == "CreateTestStep" and response_id:
            name = test_step.get("name", "")
            id_to_name[response_id] = name
            events.append((request_type, name, status))
        elif request_type == "UpdateTestStep":
            events.append((request_type, id_to_name.get(test_step.get("testStepId"), ""), status))
    return events


def load_steps(log_path: Path) -> list[dict]:
    """Load the offline log as a list of step records keyed by hierarchy fields.

    Each record has ``id``, ``name``, ``parent_step_id``, ``step_path``, the
    shape ``test_hierarchy.py`` expects for its ``_by_name`` and
    ``_ancestor_names`` walkers. Returns an empty list if the log was never
    created (e.g. every item in the inner session was ``sift_exclude``-d, so
    the plugin's ``report_context`` fixture never fired).
    """
    if not log_path.exists():
        return []
    return [
        {
            "id": s.step_id,
            "name": s.name,
            "parent_step_id": s.parent_step_id,
            "step_path": s.step_path,
            "statuses": s.statuses,
            "start_time": s.start_time,
            "end_time": s.end_time,
        }
        for s in parse_log(log_path).values()
    ]
