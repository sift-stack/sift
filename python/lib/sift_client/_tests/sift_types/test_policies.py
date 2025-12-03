"""Tests for sift_types.policies models."""

from datetime import datetime, timezone

import pytest
from sift.policies.v1.policies_pb2 import Policy as PolicyProto
from sift.policies.v1.policies_pb2 import PolicyConfiguration

from sift_client._internal.util.timestamp import to_pb_timestamp
from sift_client.sift_types.policies import Policy, PolicyCreate, PolicyUpdate


@pytest.fixture
def mock_policy(mock_client):
    """Create a mock Policy instance for testing."""
    now = datetime.now(timezone.utc)
    proto = PolicyProto(
        policy_id="test_policy_id",
        name="Engineering Access",
        description="Allow engineering department access",
        organization_id="test_org_id",
        created_by_user_id="user1",
        modified_by_user_id="user1",
        created_date=to_pb_timestamp(now),
        modified_date=to_pb_timestamp(now),
        configuration=PolicyConfiguration(
            cedar_policy='permit(principal, action, resource) when { principal.department == "Engineering" };'
        ),
        policy_version_id="test_version_id",
        is_archived=False,
    )
    policy = Policy._from_proto(proto, mock_client)
    return policy


class TestPolicyCreate:
    """Unit tests for PolicyCreate model."""

    def test_policy_create_basic(self):
        """Test basic PolicyCreate instantiation."""
        create = PolicyCreate(
            name="Engineering Access",
            cedar_policy='permit(principal, action, resource) when { principal.department == "Engineering" };',
        )

        assert create.name == "Engineering Access"
        assert "Engineering" in create.cedar_policy

    def test_policy_create_with_description(self):
        """Test PolicyCreate with description."""
        create = PolicyCreate(
            name="Engineering Access",
            cedar_policy='permit(principal, action, resource) when { principal.department == "Engineering" };',
            description="Allow engineering department access",
        )

        assert create.description == "Allow engineering department access"

    def test_policy_create_with_version_notes(self):
        """Test PolicyCreate with version notes."""
        create = PolicyCreate(
            name="Engineering Access",
            cedar_policy='permit(principal, action, resource) when { principal.department == "Engineering" };',
            version_notes="Initial version",
        )

        assert create.version_notes == "Initial version"

    def test_policy_create_to_proto(self):
        """Test that PolicyCreate converts to proto correctly."""
        create = PolicyCreate(
            name="Engineering Access",
            cedar_policy='permit(principal, action, resource) when { principal.department == "Engineering" };',
            description="Allow engineering department access",
        )
        proto = create.to_proto()

        assert proto.name == "Engineering Access"
        assert proto.description == "Allow engineering department access"
        assert proto.configuration.cedar_policy == create.cedar_policy


class TestPolicyUpdate:
    """Unit tests for PolicyUpdate model."""

    def test_policy_update_basic(self):
        """Test basic PolicyUpdate instantiation."""
        update = PolicyUpdate(name="New Name")

        assert update.name == "New Name"
        assert update.description is None
        assert update.cedar_policy is None

    def test_policy_update_to_proto_with_mask(self):
        """Test that PolicyUpdate converts to proto with field mask correctly."""
        update = PolicyUpdate(
            name="New Name",
            description="New description",
            cedar_policy="permit(principal, action, resource);",
        )
        update.resource_id = "test_policy_id"
        proto, mask = update.to_proto_with_mask()

        assert proto.policy_id == "test_policy_id"
        assert proto.name == "New Name"
        assert proto.description == "New description"
        assert proto.configuration.cedar_policy == "permit(principal, action, resource);"
        assert "name" in mask.paths
        assert "description" in mask.paths
        assert "configuration.cedar_policy" in mask.paths


class TestPolicy:
    """Unit tests for Policy model."""

    def test_policy_properties(self, mock_policy):
        """Test that Policy properties are accessible."""
        assert mock_policy.id_ == "test_policy_id"
        assert mock_policy.name == "Engineering Access"
        assert mock_policy.description == "Allow engineering department access"
        assert mock_policy.organization_id == "test_org_id"
        assert mock_policy.created_by_user_id == "user1"
        assert mock_policy.modified_by_user_id == "user1"
        assert mock_policy.created_date is not None
        assert mock_policy.created_date.tzinfo == timezone.utc
        assert mock_policy.modified_date is not None
        assert mock_policy.modified_date.tzinfo == timezone.utc
        assert "Engineering" in mock_policy.cedar_policy
        assert mock_policy.policy_version_id == "test_version_id"
        assert mock_policy.is_archived is False

    def test_policy_from_proto(self, mock_client):
        """Test Policy creation from proto."""
        now = datetime.now(timezone.utc)
        proto = PolicyProto(
            policy_id="test_policy_id",
            name="Engineering Access",
            organization_id="test_org_id",
            created_by_user_id="user1",
            modified_by_user_id="user1",
            created_date=to_pb_timestamp(now),
            modified_date=to_pb_timestamp(now),
            configuration=PolicyConfiguration(cedar_policy="permit(principal, action, resource);"),
            policy_version_id="test_version_id",
            is_archived=False,
        )

        policy = Policy._from_proto(proto, mock_client)

        assert policy.id_ == "test_policy_id"
        assert policy.name == "Engineering Access"
        assert policy.cedar_policy == "permit(principal, action, resource);"

    def test_policy_without_client_raises_error(self):
        """Test that accessing client without setting it raises an error."""
        now = datetime.now(timezone.utc)
        proto = PolicyProto(
            policy_id="test_policy_id",
            name="Engineering Access",
            organization_id="test_org_id",
            created_by_user_id="user1",
            modified_by_user_id="user1",
            created_date=to_pb_timestamp(now),
            modified_date=to_pb_timestamp(now),
            configuration=PolicyConfiguration(cedar_policy="permit(principal, action, resource);"),
            policy_version_id="test_version_id",
            is_archived=False,
        )
        policy = Policy._from_proto(proto, None)

        with pytest.raises(AttributeError, match="Sift client not set"):
            _ = policy.client
