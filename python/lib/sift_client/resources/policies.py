from __future__ import annotations

from typing import TYPE_CHECKING

from sift_client._internal.low_level_wrappers.policies import PoliciesLowLevelClient
from sift_client.resources._base import ResourceBase
from sift_client.util import cel_utils as cel

if TYPE_CHECKING:
    from sift_client.client import SiftClient
    from sift_client.sift_types.policies import Policy, PolicyUpdate


class PoliciesAPIAsync(ResourceBase):
    """High-level API for interacting with policies."""

    def __init__(self, sift_client: SiftClient):
        """Initialize the PoliciesAPI.

        Args:
            sift_client: The Sift client to use.
        """
        super().__init__(sift_client)
        self._low_level_client = PoliciesLowLevelClient(grpc_client=self.client.grpc_client)

    async def create(
        self,
        name: str,
        cedar_policy: str,
        description: str | None = None,
        version_notes: str | None = None,
    ) -> Policy:
        """Create a new policy.

        Args:
            name: The name of the policy.
            cedar_policy: The Cedar policy string.
            description: Optional description.
            version_notes: Optional version notes.

        Returns:
            The created Policy.
        """
        policy = await self._low_level_client.create_policy(
            name=name,
            cedar_policy=cedar_policy,
            description=description,
            version_notes=version_notes,
        )
        return self._apply_client_to_instance(policy)

    async def get(self, policy_id: str) -> Policy:
        """Get a policy by ID.

        Args:
            policy_id: The policy ID.

        Returns:
            The Policy.
        """
        policy = await self._low_level_client.get_policy(policy_id)
        return self._apply_client_to_instance(policy)

    async def list(
        self,
        *,
        name: str | None = None,
        name_contains: str | None = None,
        organization_id: str | None = None,
        include_archived: bool = False,
        filter_query: str | None = None,
        order_by: str | None = None,
        limit: int | None = None,
    ) -> list[Policy]:
        """List policies with optional filtering.

        Args:
            name: Exact name of the policy.
            name_contains: Partial name of the policy.
            organization_id: Filter by organization ID.
            include_archived: If True, include archived policies in results.
            filter_query: Explicit CEL query to filter policies.
            order_by: How to order the retrieved policies.
            limit: How many policies to retrieve. If None, retrieves all matches.

        Returns:
            A list of Policies that match the filter.
        """
        filter_parts = []
        if name:
            filter_parts.append(cel.eq("name", name))
        if name_contains:
            filter_parts.append(cel.contains("name", name_contains))
        if organization_id:
            filter_parts.append(cel.eq("organization_id", organization_id))
        if not include_archived:
            filter_parts.append(cel.eq("is_archived", False))

        if filter_query:
            filter_parts.append(cel.raw(filter_query))

        query_filter = cel.and_(*filter_parts) if filter_parts else None

        policies = await self._low_level_client.list_all_policies(
            query_filter=query_filter,
            order_by=order_by,
            include_archived=include_archived,
            max_results=limit,
        )
        return self._apply_client_to_instances(policies)

    async def update(
        self,
        policy: str | Policy,
        update: PolicyUpdate | dict,
        version_notes: str | None = None,
    ) -> Policy:
        """Update a policy.

        Args:
            policy: The Policy or policy ID to update.
            update: Updates to apply to the policy.
            version_notes: Optional version notes for the update.

        Returns:
            The updated Policy.
        """
        updated_policy = await self._low_level_client.update_policy(policy, update, version_notes)
        return self._apply_client_to_instance(updated_policy)

    async def archive(self, policy_id: str) -> Policy:
        """Archive a policy.

        Args:
            policy_id: The policy ID to archive.

        Returns:
            The archived Policy.
        """
        policy = await self._low_level_client.archive_policy(policy_id)
        return self._apply_client_to_instance(policy)

