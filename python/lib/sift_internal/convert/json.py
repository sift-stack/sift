from abc import ABC, abstractmethod


class AsJson(ABC):
    """
    Sub-types of this class are expected to implement `as_json`
    which returns a valid JSON string.
    """

    @abstractmethod
    def as_json(self) -> str:
        pass
