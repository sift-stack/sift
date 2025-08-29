from __future__ import annotations

from typing import TYPE_CHECKING

from sift_client.resources._base import ResourceBase

if TYPE_CHECKING:
    from sift_client.sift_types.asset import Asset


class MockClassAsync(ResourceBase):
    """Mock async class docstring."""

    def __init__(self, client=None):
        super().__init__(client)

    async def async_method(self, param1: str, *, param2: int = 0) -> str:
        """Mock async method docstring.

        Args:
            param1: Param1 docstring.
            param2: Param2 docstring.

        Returns:
            A string.
        """
        return f"{param1}:{param2}"

    def sync_method(self, param1: str, **kwargs) -> Asset | None:
        """Mock sync method docstring."""
        return None

    @property
    async def async_prop(self) -> str:
        """Mock async property docstring."""
        return "value"

    @property
    def sync_prop(self) -> int:
        """Mock sync property docstring."""
        return 42


class SecondMockClass:
    """Class doesn't have a sync version generated so shouldn't be present."""

    def shouldnt_be_in_gen_stubs(self):
        """Shouldn't be in gen stubs since it isn't called by generator."""
        return
