"""Pytest tests for the Policies API.

These tests demonstrate and validate the usage of the Policies API including:
- Basic policy operations (create, get, list, update, archive)
- Filtering and searching
- Error handling and edge cases
"""

from datetime import datetime, timezone

import pytest

from sift_client.resources import PoliciesAPI, PoliciesAPIAsync
from sift_client.sift_types import Policy

pytestmark = pytest.mark.integration


def test_client_binding(sift_client):
    """Test that policies API is properly registered on the client."""
    assert sift_client.policies
    assert isinstance(sift_client.policies, PoliciesAPI)
    assert sift_client.async_.policies
    assert isinstance(sift_client.async_.policies, PoliciesAPIAsync)


@pytest.fixture(scope="session")
def test_timestamp():
    """Setup a test timestamp for the session."""
    timestamp = datetime.now(timezone.utc)
    return timestamp


@pytest.fixture(scope="session")
def test_timestamp_str(test_timestamp):
    """Setup a test timestamp string for the session."""
    return test_timestamp.isoformat()


@pytest.fixture(scope="session")
def test_policy(sift_client, test_timestamp_str):
    """Setup a test policy for the session."""
    policy = sift_client.policies.create(
        name=f"test_policy_{test_timestamp_str}",
        cedar_policy='permit(principal, action, resource) when { principal.department == "Engineering" };',
        description="Test policy",
    )
    yield policy
    # Cleanup: archive the policy
    try:
        sift_client.policies.archive(policy.id_)
    except Exception:
        pass


class TestPolicies:
    """Tests for Policies API."""

    def test_create(self, sift_client, test_timestamp_str):
        """Test creating a policy."""
        policy = sift_client.policies.create(
            name=f"test_create_{test_timestamp_str}",
            cedar_policy="permit(principal, action, resource);",
            description="Test policy",
        )

        assert isinstance(policy, Policy)
        assert policy.id_ is not None
        assert policy.name == f"test_create_{test_timestamp_str}"
        assert "permit" in policy.cedar_policy

        # Cleanup
        sift_client.policies.archive(policy.id_)

    def test_get(self, sift_client, test_policy):
        """Test getting a policy by ID."""
        policy = sift_client.policies.get(test_policy.id_)

        assert isinstance(policy, Policy)
        assert policy.id_ == test_policy.id_
        assert policy.name == test_policy.name

    def test_list(self, sift_client):
        """Test listing policies."""
        policies = sift_client.policies.list(limit=10)

        assert isinstance(policies, list)
        assert all(isinstance(p, Policy) for p in policies)

    def test_list_with_filter(self, sift_client, test_policy):
        """Test listing policies with filtering."""
        policies = sift_client.policies.list(name=test_policy.name, limit=10)

        assert len(policies) >= 1
        assert policies[0].id_ == test_policy.id_

    def test_update(self, sift_client, test_timestamp_str):
        """Test updating a policy."""
        policy = sift_client.policies.create(
            name=f"test_update_{test_timestamp_str}",
            cedar_policy="permit(principal, action, resource);",
        )

        updated_policy = sift_client.policies.update(
            policy,
            {
                "name": f"test_updated_{test_timestamp_str}",
                "description": "Updated description",
            },
        )

        assert updated_policy.name == f"test_updated_{test_timestamp_str}"
        assert updated_policy.description == "Updated description"

        # Cleanup
        sift_client.policies.archive(updated_policy.id_)

    def test_archive(self, sift_client, test_timestamp_str):
        """Test archiving a policy."""
        policy = sift_client.policies.create(
            name=f"test_archive_{test_timestamp_str}",
            cedar_policy="permit(principal, action, resource);",
        )

        archived_policy = sift_client.policies.archive(policy.id_)

        assert archived_policy.is_archived is True
