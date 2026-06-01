from __future__ import annotations

from typing import TYPE_CHECKING

from sift_client._internal.low_level_wrappers.units import UnitsLowLevelClient
from sift_client.resources._base import ResourceBase
from sift_client.util import cel_utils as cel

if TYPE_CHECKING:
    import re

    from sift_client.client import SiftClient
    from sift_client.sift_types.unit import Unit


class UnitsAPIAsync(ResourceBase):
    """High-level API for interacting with units."""

    def __init__(self, sift_client: SiftClient):
        """Initialize the UnitsAPI.

        Args:
            sift_client: The Sift client to use.
        """
        super().__init__(sift_client)
        self._low_level_client = UnitsLowLevelClient(grpc_client=self.client.grpc_client)

    async def list_(
        self,
        *,
        name: str | None = None,
        name_contains: str | None = None,
        name_regex: str | re.Pattern | None = None,
        names: list[str] | None = None,
        unit_ids: list[str] | None = None,
        filter_query: str | None = None,
        order_by: str | None = None,
        limit: int | None = None,
        page_size: int | None = None,
    ) -> list[Unit]:
        """List units with optional filtering.

        Args:
            name: Exact name of the unit.
            name_contains: Partial name of the unit.
            name_regex: Regular expression string to filter units by name.
            names: List of unit names to filter by.
            unit_ids: List of unit IDs to filter by.
            filter_query: Explicit CEL query to filter units.
            order_by: How to order the retrieved units.
            limit: How many units to retrieve. If None, retrieves all matches.
            page_size: Number of results to fetch per request. Lower this if you hit gRPC
                message size limits on responses. If None, uses the server default.

        Returns:
            A list of Units that matches the filter.
        """
        # Build CEL filter
        filter_parts = [
            *self._build_name_cel_filters(
                name=name, names=names, name_contains=name_contains, name_regex=name_regex
            ),
            *self._build_common_cel_filters(
                filter_query=filter_query,
            ),
        ]

        if unit_ids:
            filter_parts.append(cel.in_("unit_id", unit_ids))

        query_filter = cel.and_(*filter_parts) if filter_parts else None

        units = await self._low_level_client.list_all_units(
            query_filter=query_filter,
            order_by=order_by,
            max_results=limit,
            **({"page_size": page_size} if page_size is not None else {}),
        )
        return self._apply_client_to_instances(units)

    async def find(self, **kwargs) -> Unit | None:
        """Find a single unit matching the given query. Takes the same arguments as `list`. If more than one unit is
        found, raises an error.

        Args:
            **kwargs: Keyword arguments to pass to `list`.

        Returns:
            The Unit found or None.
        """
        units = await self.list_(**kwargs)
        if len(units) > 1:
            raise ValueError("Multiple units found for query")
        elif len(units) == 1:
            return units[0]
        return None

    async def create(self, name: str) -> Unit:
        """Create a new unit.

        If a unit with the same name already exists, it is returned instead of creating a duplicate.

        Args:
            name: The name of the unit.

        Returns:
            The created Unit.
        """
        created_unit = await self._low_level_client.create_unit(name=name)
        return self._apply_client_to_instance(created_unit)

    async def find_or_create(self, names: list[str]) -> list[Unit]:
        """Find units by name or create them if they don't exist.

        Args:
            names: List of unit names to find or create.

        Returns:
            List of Units that were found or created.
        """
        units = await self.list_(names=names)
        existing_unit_names = {unit.name for unit in units}
        for name in names:
            if name not in existing_unit_names:
                units.append(await self.create(name))
        return units
