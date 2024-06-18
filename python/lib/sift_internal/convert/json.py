from abc import ABC, abstractmethod

import json


class AsJson(ABC):
    """
    Sub-types of this class are expected to implement `as_json`
    which returns a valid JSON string.
    """

    @abstractmethod
    def as_json(self) -> str:
        pass


class RemoveNullEncoder(json.JSONEncoder):
    """
    JSON encoder to remove null values
    """

    def default(self, o):
        if isinstance(o, dict):
            return {k: v for k, v in o.items() if v is not None}
        return super().default(o)
