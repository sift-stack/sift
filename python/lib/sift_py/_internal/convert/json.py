from __future__ import annotations

import json
from abc import ABC, abstractmethod
from typing import Any


class AsJson(ABC):
    """
    Utility sub-types that require custom-serialization meant to be used in conjunction with the
    `to_json` function. Sub-types should implement `as_json` which should return the object that
    you want passed to `json.dumps`.
    """

    @abstractmethod
    def as_json(self) -> Any:
        pass


def to_json(value: Any) -> str:
    """
    Serializes `value` to a JSON string uses the `AsJson.as_json` implementation of the type.
    """
    return json.dumps(value, default=lambda x: x.as_json())
