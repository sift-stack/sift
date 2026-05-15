"""Shared state for the step-status characterization suite.

The outer test in ``test_step_status_states.py`` runs inner pytest sessions
via ``pytester``. The inner session installs a fake ``sift_client`` (see
``_INNER_CONFTEST_SRC`` in that file) which records every step status
write into this module's ``CAPTURED_STEPS`` dict so the outer test can
assert on what the plugin produced.

This lives in its own module (rather than inside the test file) because
the inner ``conftest.py`` runs in a fresh pytester tmp dir and needs an
importable, package-reachable handle to the same dict object.
"""

from __future__ import annotations

from dataclasses import dataclass, field
from typing import TYPE_CHECKING

if TYPE_CHECKING:
    from sift_client.sift_types.test_report import TestStatus


@dataclass
class CapturedStep:
    step_id: str
    name: str
    step_path: str
    parent_step_id: str | None
    statuses: list[TestStatus] = field(default_factory=list)


CAPTURED_STEPS: dict[str, CapturedStep] = {}


def reset() -> None:
    CAPTURED_STEPS.clear()


def steps_by_name(name: str) -> list[CapturedStep]:
    return [s for s in CAPTURED_STEPS.values() if s.name == name]


def test_step(name: str) -> CapturedStep | None:
    """The step the autouse ``step`` fixture creates for the test function.

    There can be a deeper step with the same name when the ``makereport``
    hook also records one (e.g. ``pytest.skip()`` inside the test body, or
    an ``xfail`` mark). The autouse step is the shallowest of those, so
    pick by step_path depth.
    """
    matches = [s for s in CAPTURED_STEPS.values() if s.name == name]
    if not matches:
        return None
    return min(matches, key=lambda s: s.step_path.count("."))


def child_steps(parent: CapturedStep) -> list[CapturedStep]:
    return [s for s in CAPTURED_STEPS.values() if s.parent_step_id == parent.step_id]


def final_status(name: str) -> TestStatus | None:
    step = test_step(name)
    if step is None or not step.statuses:
        return None
    return step.statuses[-1]
