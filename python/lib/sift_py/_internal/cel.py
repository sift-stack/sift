"""
Utilities to interact with APIs that have a CEL-based interface.
"""

from typing import Iterable


def cel_in(field: str, values: Iterable[str]) -> str:
    """
    Produces a list membership CEL expression. Example:

    ```python
    > print(cel_in("name", ["foo", "bar"]))
    name in ["foo", "bar"]
    ```
    """
    items = ",".join([f'"{val}"' for val in values])
    return f"{field} in [{items}]"
