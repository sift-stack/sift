"""Access-control APIs.

Groups the ABAC sub-APIs (resource and principal attributes); roles, policies, and
user groups will live here as they are added. The namespace is exposed on the client
as ``client.access_control`` (and ``client.async_.access_control``).
"""

from __future__ import annotations

from typing import TYPE_CHECKING

if TYPE_CHECKING:
    from sift_client.resources.access_control.principal_attributes import (
        PrincipalAttributesAPIAsync,
    )
    from sift_client.resources.access_control.resource_attributes import (
        ResourceAttributesAPIAsync,
    )
    from sift_client.resources.sync_stubs import (
        PrincipalAttributesAPI,
        ResourceAttributesAPI,
    )


class AccessControlAPI:
    """Access-control namespace. Groups the ABAC APIs; roles, policies, and user groups
    will live here as they are added.
    """

    resource_attributes: ResourceAttributesAPI
    """Attribute keys assigned to entities (assets, channels, runs)."""

    principal_attributes: PrincipalAttributesAPI
    """Attribute keys assigned to principals (users, user groups)."""

    def __init__(
        self,
        *,
        resource_attributes: ResourceAttributesAPI,
        principal_attributes: PrincipalAttributesAPI,
    ):
        """Initialize the access-control namespace with its sub-APIs."""
        self.resource_attributes = resource_attributes
        self.principal_attributes = principal_attributes


class AccessControlAPIAsync:
    """Asynchronous counterpart to `AccessControlAPI`."""

    resource_attributes: ResourceAttributesAPIAsync
    """Attribute keys assigned to entities (assets, channels, runs)."""

    principal_attributes: PrincipalAttributesAPIAsync
    """Attribute keys assigned to principals (users, user groups)."""

    def __init__(
        self,
        *,
        resource_attributes: ResourceAttributesAPIAsync,
        principal_attributes: PrincipalAttributesAPIAsync,
    ):
        """Initialize the access-control namespace with its sub-APIs."""
        self.resource_attributes = resource_attributes
        self.principal_attributes = principal_attributes
