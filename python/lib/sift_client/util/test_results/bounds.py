from __future__ import annotations

from typing import TYPE_CHECKING

import numpy as np
import pandas as pd

from sift_client.sift_types.test_report import (
    NumericBounds,
    TestMeasurement,
    TestMeasurementCreate,
    TestMeasurementType,
    TestMeasurementUpdate,
)

if TYPE_CHECKING:
    from numpy.typing import NDArray


def to_numpy_array(
    values: list[float | int] | NDArray[np.float64] | pd.Series,
) -> NDArray[np.float64]:
    """Normalize a list / ndarray / pandas Series into a numpy array.

    Shared by ``measure_avg`` and ``measure_all`` in both the real and
    stub step implementations so the accepted input types stay in sync.
    """
    if isinstance(values, list):
        return np.array(values)
    if isinstance(values, np.ndarray):
        return values
    if isinstance(values, pd.Series):
        return values.to_numpy()
    raise ValueError(f"Invalid value type: {type(values)}")


def out_of_bounds_mask(
    arr: NDArray[np.float64],
    bounds: dict[str, float] | NumericBounds,
) -> NDArray[np.bool_]:
    """Return a boolean mask selecting elements of ``arr`` that violate ``bounds``.

    Raises ``ValueError`` when ``bounds`` has neither ``min`` nor ``max`` set.
    """
    if isinstance(bounds, dict):
        bounds = NumericBounds(min=bounds.get("min"), max=bounds.get("max"))
    mask: NDArray[np.bool_] | None = None
    if bounds.min is not None:
        mask = arr < bounds.min
    if bounds.max is not None:
        above = arr > bounds.max
        mask = mask | above if mask is not None else above
    if mask is None:
        raise ValueError("No bounds provided")
    return mask


def all_within_bounds(
    arr: NDArray[np.float64],
    bounds: dict[str, float] | NumericBounds,
) -> bool:
    """Return True when every element of ``arr`` is within ``bounds``."""
    return bool(arr[out_of_bounds_mask(arr, bounds)].size == 0)


def assign_value_to_measurement(
    measurement: TestMeasurement | TestMeasurementCreate | TestMeasurementUpdate,
    value: float | str | bool,
) -> None:
    """Resolve value type from a given value and assign it to a measurement.

    Args:
        measurement: The measurement to assign the value to.
        value: The value to resolve and assign to the measurement.
    """
    if isinstance(value, bool):
        measurement.boolean_value = value
        measurement.measurement_type = TestMeasurementType.BOOLEAN
    elif isinstance(value, float) or isinstance(value, int):
        measurement.numeric_value = float(value)
        measurement.measurement_type = TestMeasurementType.DOUBLE
    elif isinstance(value, str):
        measurement.string_value = value
        measurement.measurement_type = TestMeasurementType.STRING
    else:
        raise ValueError(f"Invalid value type: {type(value)}")


def value_passes_bounds(
    value: float | str | bool,
    bounds: dict[str, float] | NumericBounds | str | bool | None,
) -> bool:
    """Evaluate a value against bounds without recording a measurement.

    Used by consumers that need pass/fail semantics matching the real plugin but
    do not transmit a measurement (e.g. ``--sift-disabled`` mode in the pytest
    plugin).
    """
    if bounds is None:
        return True
    if isinstance(bounds, dict):
        bounds = NumericBounds(min=bounds.get("min"), max=bounds.get("max"))
    if isinstance(bounds, bool):
        if isinstance(value, str):
            return str(value).lower() == str(bounds).lower()
        return bool(value) == bounds
    if isinstance(bounds, str):
        if not (isinstance(value, str) or isinstance(value, bool)):
            raise ValueError("Value must be a string if bounds provided is a string")
        if isinstance(value, bool):
            return str(value).lower() == str(bounds).lower()
        return value == bounds
    # NumericBounds
    try:
        if bounds.min is not None and bounds.min > value:  # type: ignore[operator]
            return False
        if bounds.max is not None and bounds.max < value:  # type: ignore[operator]
            return False
    except TypeError:
        raise TypeError(
            f"Value must be a float or int to evaluate numeric bounds but gave {type(value)}"
        ) from None
    return True


def evaluate_measurement_bounds(
    measurement: TestMeasurement | TestMeasurementCreate | TestMeasurementUpdate,
    value: float | str | bool,
    bounds: dict[str, float] | NumericBounds | str | bool | None,
) -> bool:
    """Update a measurement with the resolved bounds type and result of evaluating the given value against those bounds.

    Args:
        measurement: The measurement to update.
        value: The value to evaluate the bounds of.
        bounds: The bounds to evaluate the value against. Either a dictionary with "min" and "max" keys, a NumericBounds object, a string, a boolean, or None.

    Returns:
        True if the value is within the bounds, False otherwise.
    """
    assign_value_to_measurement(measurement, value)
    if bounds is None:
        return bool(measurement.passed)

    if isinstance(bounds, dict):
        bounds = NumericBounds(min=bounds.get("min"), max=bounds.get("max"))
    if isinstance(bounds, str) and not isinstance(bounds, bool):
        measurement.string_expected_value = bounds
    elif isinstance(bounds, NumericBounds):
        measurement.numeric_bounds = bounds

    measurement.passed = value_passes_bounds(value, bounds)
    return bool(measurement.passed)
