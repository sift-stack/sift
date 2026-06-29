"""Access-control API namespace.

Use ``client.access_control`` for synchronous APIs and
``client.async_.access_control`` for asynchronous APIs.
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
    """Namespace for access-control APIs."""

    resource_attributes: ResourceAttributesAPI
    """Manage ABAC attributes for assets, channels, and runs."""

    principal_attributes: PrincipalAttributesAPI
    """Manage ABAC attributes for users and user groups."""

    def __init__(
        self,
        *,
        resource_attributes: ResourceAttributesAPI,
        principal_attributes: PrincipalAttributesAPI,
    ):
        """Initialize the namespace."""
        self.resource_attributes = resource_attributes
        self.principal_attributes = principal_attributes


class AccessControlAPIAsync:
    """Namespace for async access-control APIs."""

    resource_attributes: ResourceAttributesAPIAsync
    """Manage ABAC attributes for assets, channels, and runs."""

    principal_attributes: PrincipalAttributesAPIAsync
    """Manage ABAC attributes for users and user groups."""

    def __init__(
        self,
        *,
        resource_attributes: ResourceAttributesAPIAsync,
        principal_attributes: PrincipalAttributesAPIAsync,
    ):
        """Initialize the namespace."""
        self.resource_attributes = resource_attributes
        self.principal_attributes = principal_attributes
