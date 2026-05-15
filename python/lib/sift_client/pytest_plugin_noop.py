"""No-op variant of the Sift pytest plugin.

Wire this in instead of ``sift_client.pytest_plugin`` to keep test code that
calls ``step.measure(...)``, ``step.substep(...)``, etc. working without any
Sift configuration. Bounds are evaluated locally so pass/fail outcomes match
the real plugin exactly; nothing is sent to Sift and no log file is written.

Typical usage in a project's top-level conftest:

    import os
    pytest_plugins = [
        "sift_client.pytest_plugin"
        if os.getenv("SIFT_ENABLED")
        else "sift_client.pytest_plugin_noop"
    ]

The fixture names (``report_context``, ``step``, ``module_substep``,
``client_has_connection``) and method signatures match the real plugin's so
swapping plugins requires no test-code changes.
"""

from __future__ import annotations

from contextlib import AbstractContextManager
from typing import TYPE_CHECKING, Any, Generator

import pytest

from sift_client.util.test_results.bounds import value_passes_bounds

if TYPE_CHECKING:
    from datetime import datetime

    import numpy as np
    import pandas as pd
    from numpy.typing import NDArray

    from sift_client.sift_types.channel import Channel
    from sift_client.sift_types.test_report import NumericBounds


class _NoopTestStep:
    """Stub for ``TestStep`` exposing only the attributes user code touches."""

    description: str | None = None
    status: Any = None

    def update(self, *_args: Any, **_kwargs: Any) -> None:
        return None


class _NoopReport:
    """Stub for ``TestReport`` exposing only ``.update(...)``."""

    def update(self, *_args: Any, **_kwargs: Any) -> None:
        return None


class _NoopStep(AbstractContextManager):
    """Step shim that mirrors ``NewStep``'s public surface without any I/O."""

    def __init__(self, name: str, description: str | None = None) -> None:
        self.name = name
        self.current_step = _NoopTestStep()
        self.current_step.description = description

    def __enter__(self) -> _NoopStep:
        return self

    def __exit__(self, exc_type, exc_value, tb) -> bool:
        return False

    def measure(
        self,
        *,
        name: str,
        value: float | str | bool | int,
        bounds: dict[str, float] | NumericBounds | str | bool | None = None,
        timestamp: datetime | None = None,
        unit: str | None = None,
        description: str | None = None,
        metadata: dict[str, str | float | bool] | None = None,
        channel_names: list[str] | list[Channel] | None = None,
    ) -> bool:
        return value_passes_bounds(value, bounds)

    def measure_avg(
        self,
        *,
        name: str,
        values: list[float | int] | NDArray[np.float64] | pd.Series,
        bounds: dict[str, float] | NumericBounds,
        timestamp: datetime | None = None,
        unit: str | None = None,
        description: str | None = None,
        metadata: dict[str, str | float | bool] | None = None,
        channel_names: list[str] | list[Channel] | None = None,
    ) -> bool:
        import numpy as np
        import pandas as pd

        if isinstance(values, list):
            arr = np.array(values)
        elif isinstance(values, np.ndarray):
            arr = values
        elif isinstance(values, pd.Series):
            arr = values.to_numpy()
        else:
            raise ValueError(f"Invalid value type: {type(values)}")
        return value_passes_bounds(float(np.mean(arr)), bounds)

    def measure_all(
        self,
        *,
        name: str,
        values: list[float | int] | NDArray[np.float64] | pd.Series,
        bounds: dict[str, float] | NumericBounds,
        timestamp: datetime | None = None,
        unit: str | None = None,
        description: str | None = None,
        metadata: dict[str, str | float | bool] | None = None,
        channel_names: list[str] | list[Channel] | None = None,
    ) -> bool:
        import numpy as np
        import pandas as pd

        from sift_client.sift_types.test_report import NumericBounds as _NumericBounds

        if isinstance(values, list):
            arr = np.array(values)
        elif isinstance(values, np.ndarray):
            arr = values
        elif isinstance(values, pd.Series):
            arr = values.to_numpy()
        else:
            raise ValueError(f"Invalid value type: {type(values)}")

        nb = bounds
        if isinstance(nb, dict):
            nb = _NumericBounds(min=bounds.get("min"), max=bounds.get("max"))  # type: ignore[union-attr]
        mask = None
        if nb.min is not None:
            mask = arr < nb.min
        if nb.max is not None:
            above = arr > nb.max
            mask = mask | above if mask is not None else above
        if mask is None:
            raise ValueError("No bounds provided")
        return bool(arr[mask].size == 0)

    def report_outcome(self, name: str, result: bool, reason: str | None = None) -> bool:
        return result

    def substep(
        self,
        name: str,
        description: str | None = None,
        metadata: dict[str, str | float | bool] | None = None,
    ) -> _NoopStep:
        return _NoopStep(name=name, description=description)


class _NoopReportContext:
    """Report context shim exposing only what user code or autouse hooks touch."""

    def __init__(self) -> None:
        self.report = _NoopReport()

    def new_step(
        self,
        name: str,
        description: str | None = None,
        assertion_as_fail_not_error: bool = True,
        metadata: dict[str, str | float | bool] | None = None,
    ) -> _NoopStep:
        return _NoopStep(name=name, description=description)


@pytest.fixture(scope="session")
def client_has_connection() -> bool:
    """Always-false reachability signal under the no-op plugin."""
    return False


@pytest.fixture(scope="session", autouse=True)
def report_context() -> Generator[_NoopReportContext, None, None]:
    """No-op report context."""
    yield _NoopReportContext()


@pytest.fixture(autouse=True)
def step(request: pytest.FixtureRequest) -> Generator[_NoopStep, None, None]:
    """Per-test no-op step. Bounds-checked ``measure*`` calls still return real booleans."""
    name = str(request.node.name)
    docstring = request.node.obj.__doc__ if hasattr(request.node, "obj") else None
    with _NoopStep(name=name, description=docstring) as new_step:
        yield new_step


@pytest.fixture(scope="module", autouse=True)
def module_substep(request: pytest.FixtureRequest) -> Generator[_NoopStep, None, None]:
    """Per-module no-op step."""
    name = str(request.node.name)
    docstring = request.node.obj.__doc__ if hasattr(request.node, "obj") else None
    with _NoopStep(name=name, description=docstring) as new_step:
        yield new_step
