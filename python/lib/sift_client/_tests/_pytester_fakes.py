"""Test doubles for the pytester-driven pytest-plugin tests.

The fake ``ReportContext`` is a drop-in for the real one that records every
step creation to a JSON file at session exit. Used by ``test_pytest_plugin_parametrize.py``
to assert the step tree produced by an inner pytester pytest run.
"""

from __future__ import annotations

import json
from typing import TYPE_CHECKING, Any
from unittest.mock import MagicMock

if TYPE_CHECKING:
    from pathlib import Path


class FakeStep:
    def __init__(self, id_: str, name: str, parent_step_id: str | None, step_path: str) -> None:
        self.id_ = id_
        self.name = name
        self.parent_step_id = parent_step_id
        self.step_path = step_path
        self.status: Any = None
        self.description: Any = None
        self.error_info: Any = None

    def update(self, fields: dict[str, Any]) -> None:
        for k, v in fields.items():
            setattr(self, k, v)


class FakeReport:
    def __init__(self) -> None:
        self.id_ = "report-id"

    def update(self, fields: dict[str, Any]) -> None:
        pass


class FakeReportContext:
    _counter = 0

    def __init__(self, steps_file: Path) -> None:
        self.steps_file = steps_file
        self.report = FakeReport()
        self.client = MagicMock()
        self.step_stack: list[FakeStep] = []
        self.step_number_at_depth: dict[int, int] = {}
        self.open_step_results: dict[str, bool] = {}
        self.any_failures = False
        self.log_file: Path | None = None
        self.steps: list[dict[str, Any]] = []

    def __enter__(self) -> FakeReportContext:
        return self

    def __exit__(self, *_: Any) -> None:
        self.steps_file.write_text(json.dumps(self.steps))

    def new_step(
        self,
        name: str,
        description: str | None = None,
        assertion_as_fail_not_error: bool = True,
        metadata: dict[str, Any] | None = None,
    ) -> Any:
        # Reuse the real NewStep machinery — it talks to this fake via the
        # methods below.
        from sift_client.util.test_results.context_manager import NewStep

        return NewStep(
            self,  # type: ignore[arg-type]
            name=name,
            description=description,
            assertion_as_fail_not_error=assertion_as_fail_not_error,
            metadata=metadata,
        )

    def get_next_step_path(self) -> str:
        top = self.step_stack[-1] if self.step_stack else None
        path = top.step_path if top else ""
        next_n = self.step_number_at_depth.get(len(self.step_stack), 0) + 1
        prefix = f"{path}." if path else ""
        return f"{prefix}{next_n}"

    def create_step(
        self,
        name: str,
        description: str | None = None,
        metadata: dict[str, Any] | None = None,
    ) -> FakeStep:
        type(self)._counter += 1
        step_path = self.get_next_step_path()
        parent = self.step_stack[-1] if self.step_stack else None
        step = FakeStep(
            id_=f"step-{type(self)._counter}",
            name=name,
            parent_step_id=parent.id_ if parent else None,
            step_path=step_path,
        )
        self.step_number_at_depth[len(self.step_stack)] = (
            self.step_number_at_depth.get(len(self.step_stack), 0) + 1
        )
        self.step_stack.append(step)
        self.open_step_results[step.step_path] = True
        self.steps.append(
            {
                "id": step.id_,
                "name": name,
                "parent_step_id": step.parent_step_id,
                "step_path": step_path,
            }
        )
        return step

    def record_step_outcome(self, outcome: bool, step: FakeStep) -> None:
        if not outcome:
            self.open_step_results[step.step_path] = False
            self.any_failures = True

    def resolve_and_propagate_step_result(self, step: FakeStep, error_info: Any = None) -> bool:
        result = self.open_step_results.get(step.step_path, True)
        if error_info:
            result = False
        return result

    def exit_step(self, step: FakeStep) -> None:
        self.step_number_at_depth[len(self.step_stack)] = 0
        stack_top = self.step_stack.pop()
        self.open_step_results.pop(step.step_path)
        if stack_top.id_ != step.id_:
            raise ValueError("popped step was not the top of the stack")
