from __future__ import annotations

from datetime import datetime, timezone
from typing import TYPE_CHECKING

from sift.policies.v1.policies_pb2 import (
    CreatePolicyRequest as CreatePolicyRequestProto,
)
from sift.policies.v1.policies_pb2 import (
    Policy as PolicyProto,
)

from sift_client.sift_types._base import BaseType, ModelCreate, ModelUpdate

if TYPE_CHECKING:
    from sift_client.client import SiftClient


class Policy(BaseType[PolicyProto, "Policy"]):
    """Model representing a Policy."""

    name: str
    description: str | None
    organization_id: str
    created_by_user_id: str
    modified_by_user_id: str
    created_date: datetime
    modified_date: datetime
    cedar_policy: str  # Policy configuration Cedar policy string
    policy_version_id: str
    archived_date: datetime | None
    is_archived: bool
    version: int | None
    version_notes: str | None
    generated_change_message: str | None

    @classmethod
    def _from_proto(cls, proto: PolicyProto, sift_client: SiftClient | None = None) -> Policy:
        return cls(
            id_=proto.policy_id,
            proto=proto,
            name=proto.name,
            description=proto.description if proto.HasField("description") else None,
            organization_id=proto.organization_id,
            created_by_user_id=proto.created_by_user_id,
            modified_by_user_id=proto.modified_by_user_id,
            created_date=proto.created_date.ToDatetime(tzinfo=timezone.utc),
            modified_date=proto.modified_date.ToDatetime(tzinfo=timezone.utc),
            cedar_policy=proto.configuration.cedar_policy,
            policy_version_id=proto.policy_version_id,
            archived_date=(
                proto.archived_date.ToDatetime(tzinfo=timezone.utc)
                if proto.HasField("archived_date")
                else None
            ),
            is_archived=proto.is_archived,
            version=proto.version if proto.HasField("version") else None,
            version_notes=proto.version_notes if proto.HasField("version_notes") else None,
            generated_change_message=(
                proto.generated_change_message
                if proto.HasField("generated_change_message")
                else None
            ),
            _client=sift_client,
        )


class PolicyCreate(ModelCreate[CreatePolicyRequestProto]):
    """Create model for Policy."""

    name: str
    description: str | None = None
    cedar_policy: str
    version_notes: str | None = None

    def _get_proto_class(self) -> type[CreatePolicyRequestProto]:
        return CreatePolicyRequestProto

    def to_proto(self) -> CreatePolicyRequestProto:
        """Convert to proto, handling policy configuration."""
        proto = super().to_proto()
        # Set policy configuration
        proto.configuration.cedar_policy = self.cedar_policy
        return proto


class PolicyUpdate(ModelUpdate[PolicyProto]):
    """Update model for Policy."""

    name: str | None = None
    description: str | None = None
    cedar_policy: str | None = None
    version_notes: str | None = None

    def _get_proto_class(self) -> type[PolicyProto]:
        return PolicyProto

    def _add_resource_id_to_proto(self, proto_msg: PolicyProto):
        if self._resource_id is None:
            raise ValueError("Resource ID must be set before adding to proto")
        proto_msg.policy_id = self._resource_id

    def to_proto_with_mask(self) -> tuple[PolicyProto, any]:
        """Convert to proto with field mask, handling policy configuration."""
        proto, mask = super().to_proto_with_mask()
        # If cedar_policy is being updated, set it in the configuration
        if self.cedar_policy is not None:
            proto.configuration.cedar_policy = self.cedar_policy
            if "configuration.cedar_policy" not in mask.paths:
                mask.paths.append("configuration.cedar_policy")
        return proto, mask

