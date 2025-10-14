"""CEL (Common Expression Language) utilities for generating CEL expressions.

This module provides helper functions to generate CEL expressions for building filters commonly used in Sift.
"""

from __future__ import annotations

import re
from datetime import datetime, timedelta
from typing import Any


def in_(field: str, vals: list[str]) -> str:
    """Generates a CEL expression that checks for `field` membership in `vals`.

    Args:
        field: The field name to check
        vals: List of string values to check membership against

    Returns:
        A CEL expression string or empty string if vals is empty
    """
    if not vals:
        return ""

    quoted_vals = [f"'{val}'" for val in vals]
    return f"{field} in [{','.join(quoted_vals)}]"


def parens(expr: str) -> str:
    """Wraps the given expression in parentheses.

    Args:
        expr: The expression to wrap in parentheses

    Returns:
        A CEL expression string with parentheses
    """
    return f"({expr})"


def equals(key: str, value: Any) -> str:
    """Generates a CEL expression that checks for equality.

    Args:
        key: The field name
        value: The value to compare against

    Returns:
        A CEL expression string
    """
    if value is None:
        return equals_null(key)
    elif isinstance(value, str):
        return f"{key} == '{value}'"
    elif isinstance(value, bool):
        return f"{key} == {str(value).lower()}"
    else:
        return f"{key} == {value}"


def equals_all(values: dict[str, Any]) -> str:
    """Generates a CEL expression that checks for equality of all key-value pairs.

    Args:
        values: Dictionary of field names and values to check for equality

    Returns:
        A CEL expression string with all equality checks joined by AND
    """
    clauses = [equals(key, value) for key, value in values.items()]
    return and_(*clauses)


def equals_any(values: dict[str, Any]) -> str:
    """Generates a CEL expression that checks for equality of any key-value pairs.

    Args:
        values: Dictionary of field names and values to check for equality

    Returns:
        A CEL expression string with all equality checks joined by OR
    """
    clauses = [equals(key, value) for key, value in values.items()]
    return or_(*clauses)


def equals_double(key: str, value: Any) -> str:
    """Generates a CEL expression that checks for equality with a double value.

    Args:
        key: The field name
        value: The value to compare against as a double

    Returns:
        A CEL expression string
    """
    if value is None:
        return f"{key} == null"
    return f"{key} == double({value})"


def equals_null(key: str) -> str:
    """Generates a CEL expression that checks for equality with null.

    Args:
        key: The field name

    Returns:
        A CEL expression string
    """
    return f"{key} == null"


def and_(*clauses: str) -> str:
    """Generates a CEL expression that joins all clauses with an AND operator.

    Args:
        *clauses: Variable number of CEL expression strings

    Returns:
        A CEL expression string with all clauses joined by AND
    """
    if not clauses:
        return ""
    if len(clauses) == 1:
        return clauses[0]
    return " && ".join(clauses)


def or_(*clauses: str) -> str:
    """Generates a CEL expression that joins all clauses with an OR operator.

    Args:
        *clauses: Variable number of CEL expression strings

    Returns:
        A CEL expression string with all clauses joined by OR
    """
    if not clauses:
        return ""
    if len(clauses) == 1:
        return clauses[0]
    return " || ".join(clauses)


def not_(clause: str) -> str:
    """Generates a CEL expression that negates the given clause.

    Args:
        clause: The CEL expression to negate

    Returns:
        A negated CEL expression string
    """
    return f"!({clause})"


def contains(field: str, value: str) -> str:
    """Generates a CEL expression that checks whether a string field contains a given value.

    Args:
        field: The field name
        value: The substring to check for

    Returns:
        A CEL expression string
    """
    return f"{field}.contains('{value}')"


def match(field: str, query: str | re.Pattern) -> str:
    """Generates a CEL expression that checks for a match on the specified field.

    Args:
        field: The field name
        query: The regex pattern to match against

    Returns:
        A CEL expression string
    """
    if isinstance(query, re.Pattern):
        query = str(query.pattern)
    # Double-escape any backslashes that already exist in the regex
    escaped_regex = query.replace("\\", "\\\\")
    return f"{field}.matches('{escaped_regex}')"


def greater_than(field: str, value: int | float | datetime | timedelta) -> str:
    """Generates a CEL expression that checks whether a numeric or datetime field is greater than a given value.

    Args:
        field: The field name
        value: The value to compare against

    Returns:
        A CEL expression string
    """
    if isinstance(value, datetime):
        as_string = f"timestamp('{value.isoformat()}')"
    elif isinstance(value, timedelta):
        as_string = f"duration('{value.total_seconds()}s')"
    else:
        as_string = str(value)
    return f"{field} > {as_string}"


def less_than(field: str, value: int | float | datetime | timedelta) -> str:
    """Generates a CEL expression that checks whether a numeric or datetime field is less than a given value.

    Args:
        field: The field name
        value: The value to compare against

    Returns:
        A CEL expression string
    """
    if isinstance(value, datetime):
        as_string = f"timestamp('{value.isoformat()}')"
    elif isinstance(value, timedelta):
        as_string = f"duration('{value.total_seconds()}s')"
    else:
        as_string = str(value)
    return f"{field} < {as_string}"
