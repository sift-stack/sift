from __future__ import annotations

from datetime import datetime, timezone
from typing import TYPE_CHECKING

from google.protobuf import field_mask_pb2
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
        # Get the corresponding proto class
        proto_cls = self._get_proto_class()
        proto_msg = proto_cls()

        # Get all fields except cedar_policy (we'll handle it manually)
        data = self.model_dump(
            exclude_unset=True, exclude_none=True, exclude={"cedar_policy"}
        )
        self._build_proto_and_paths(proto_msg, data)

        # Set policy configuration manually
        proto_msg.configuration.cedar_policy = self.cedar_policy

        return proto_msg


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

    def to_proto_with_mask(self) -> tuple[PolicyProto, field_mask_pb2.FieldMask]:
        """Convert to proto with field mask, handling policy configuration."""
        # Get the corresponding proto class
        proto_cls = self._get_proto_class()
        proto_msg = proto_cls()

        # Get all fields except cedar_policy (we'll handle it manually)
        data = self.model_dump(
            exclude_unset=True, exclude_none=True, exclude={"cedar_policy"}
        )
        paths = self._build_proto_and_paths(proto_msg, data)

        # Set resource ID
        self._add_resource_id_to_proto(proto_msg)

        # If cedar_policy is being updated, set it in the configuration
        if self.cedar_policy is not None:
            proto_msg.configuration.cedar_policy = self.cedar_policy
            if "configuration.cedar_policy" not in paths:
                paths.append("configuration.cedar_policy")

        mask = field_mask_pb2.FieldMask(paths=paths)
        return proto_msg, mask
