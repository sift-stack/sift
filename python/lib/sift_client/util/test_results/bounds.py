from __future__ import annotations

from sift_client.sift_types.test_report import (
    NumericBounds,
    TestMeasurement,
    TestMeasurementCreate,
    TestMeasurementType,
    TestMeasurementUpdate,
)


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
    if isinstance(bounds, bool):
        if isinstance(value, str):
            measurement.passed = str(value).lower() == str(bounds).lower()
        else:
            measurement.passed = bool(value) == bounds
        return bool(measurement.passed)
    elif isinstance(bounds, str):
        if not (isinstance(value, str) or isinstance(value, bool)):
            raise ValueError("Value must be a string if bounds provided is a string")
        measurement.string_expected_value = bounds
        if isinstance(value, bool):
            measurement.passed = str(value).lower() == str(bounds).lower()
        else:
            measurement.passed = value == bounds
    elif isinstance(bounds, NumericBounds):
        measurement.numeric_bounds = bounds
        measurement.passed = True
        float_value = float(value)
        if measurement.numeric_bounds.min is not None:
            measurement.passed = (
                measurement.passed and measurement.numeric_bounds.min <= float_value
            )
        if measurement.numeric_bounds.max is not None:
            measurement.passed = (
                measurement.passed and measurement.numeric_bounds.max >= float_value
            )
    return bool(measurement.passed)
