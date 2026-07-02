"""Access-control API namespace.

Access-control APIs configure who can access what in Sift. In these APIs, a
principal is the "who" and a resource is the "what" that access applies to.

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
    """Access-control APIs for configuring who can access what in Sift."""

    resource_attributes: ResourceAttributesAPI
    """Manage attributes on supported resources, such as assets, channels, and runs."""

    principal_attributes: PrincipalAttributesAPI
    """Manage attributes on principals such as users and user groups."""

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
    """Async access-control APIs for configuring who can access what in Sift."""

    resource_attributes: ResourceAttributesAPIAsync
    """Manage attributes on supported resources, such as assets, channels, and runs."""

    principal_attributes: PrincipalAttributesAPIAsync
    """Manage attributes on principals such as users and user groups."""

    def __init__(
        self,
        *,
        resource_attributes: ResourceAttributesAPIAsync,
        principal_attributes: PrincipalAttributesAPIAsync,
    ):
        """Initialize the namespace."""
        self.resource_attributes = resource_attributes
        self.principal_attributes = principal_attributes
