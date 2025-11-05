"""
Script to create, update, and delete ABAC resources:
- User Attributes (keys and values)
- Resource Attributes (keys, enum values, and attributes)
- Access Policies

This script can be copied and pasted into a Jupyter notebook or run as a standalone script.
"""

import os
from typing import Optional

from dotenv import load_dotenv
from google.protobuf import field_mask_pb2

# User Attributes imports
from sift.user_attributes.v1.user_attributes_pb2 import (
    ArchiveUserAttributeKeysRequest,
    ArchiveUserAttributeValuesRequest,
    CreateUserAttributeKeyRequest,
    CreateUserAttributeValueRequest,
    GetUserAttributeKeyRequest,
    GetUserAttributeValueRequest,
    ListUserAttributeKeysRequest,
    ListUserAttributeValuesRequest,
    UserAttributeValueType,
)
from sift.user_attributes.v1.user_attributes_pb2_grpc import UserAttributesServiceStub

# Resource Attributes imports
from sift.resource_attribute.v1.resource_attribute_pb2 import (
    ArchiveResourceAttributeKeyRequest,
    ArchiveResourceAttributeRequest,
    CreateResourceAttributeEnumValueRequest,
    CreateResourceAttributeKeyRequest,
    CreateResourceAttributeRequest,
    GetResourceAttributeKeyRequest,
    GetResourceAttributeRequest,
    ListResourceAttributeKeysRequest,
    ListResourceAttributesRequest,
    ResourceAttributeEntityIdentifier,
    ResourceAttributeEntityType,
    ResourceAttributeEnumValue,
    ResourceAttributeKey,
    ResourceAttributeKeyType,
    UpdateResourceAttributeKeyRequest,
)
from sift.resource_attribute.v1.resource_attribute_pb2_grpc import ResourceAttributeServiceStub

# Policies imports
from sift.policies.v1.policies_pb2 import (
    ArchivePolicyRequest,
    CreatePolicyRequest,
    GetPolicyRequest,
    ListPoliciesRequest,
    PolicyConfiguration,
    UpdatePolicyRequest,
)
from sift.policies.v1.policies_pb2_grpc import PolicyServiceStub

from sift_py.grpc.transport import SiftChannelConfig, use_sift_channel

# Load environment variables
load_dotenv()

API_KEY = os.getenv("SIFT_API_KEY") or os.getenv("api_key")
BASE_URI = os.getenv("BASE_URI") or os.getenv("BASE_URI", "localhost:50051")

if not API_KEY:
    raise ValueError("Please set SIFT_API_KEY or api_key environment variable")
if not BASE_URI:
    raise ValueError("Please set BASE_URI environment variable")

channel_config: SiftChannelConfig = {
    "apikey": API_KEY,
    "uri": BASE_URI,
}


# ============================================================================
# USER ATTRIBUTES
# ============================================================================

def create_user_attribute_key(
    name: str,
    description: str,
    value_type: UserAttributeValueType,
    organization_id: Optional[str] = None,
) -> str:
    """Create a user attribute key.

    Args:
        name: Name of the attribute key (e.g., "department")
        description: Description of the attribute key
        value_type: Type of value (USER_ATTRIBUTE_VALUE_TYPE_STRING, BOOLEAN, or NUMBER)
        organization_id: Optional organization ID

    Returns:
        The created user_attribute_key_id
    """
    with use_sift_channel(channel_config) as channel:
        stub = UserAttributesServiceStub(channel)
        request = CreateUserAttributeKeyRequest(
            name=name,
            description=description,
            type=value_type,
            organization_id=organization_id,
        )
        response = stub.CreateUserAttributeKey(request)
        print(f"Created user attribute key: {response.user_attribute_key.name} (ID: {response.user_attribute_key.user_attribute_key_id})")
        return response.user_attribute_key.user_attribute_key_id


def list_user_attribute_keys(organization_id: Optional[str] = None, include_archived: bool = False):
    """List all user attribute keys."""
    with use_sift_channel(channel_config) as channel:
        stub = UserAttributesServiceStub(channel)
        request = ListUserAttributeKeysRequest(
            organization_id=organization_id,
            include_archived=include_archived,
        )
        response = stub.ListUserAttributeKeys(request)
        print(f"Found {len(response.user_attribute_keys)} user attribute keys:")
        for key in response.user_attribute_keys:
            print(f"  - {key.name} (ID: {key.user_attribute_key_id}, Type: {key.type})")
        return response.user_attribute_keys


def create_user_attribute_value(
    user_attribute_key_id: str,
    user_id: str,
    string_value: Optional[str] = None,
    number_value: Optional[float] = None,
    boolean_value: Optional[bool] = None,
    organization_id: Optional[str] = None,
) -> str:
    """Create a user attribute value.

    Args:
        user_attribute_key_id: The ID of the user attribute key
        user_id: The user ID to assign the attribute to
        string_value: String value (if key type is STRING)
        number_value: Number value (if key type is NUMBER)
        boolean_value: Boolean value (if key type is BOOLEAN)
        organization_id: Optional organization ID

    Returns:
        The created user_attribute_value_id
    """
    with use_sift_channel(channel_config) as channel:
        stub = UserAttributesServiceStub(channel)
        request = CreateUserAttributeValueRequest(
            user_attribute_key_id=user_attribute_key_id,
            user_id=user_id,
            organization_id=organization_id,
        )
        if string_value is not None:
            request.string_value = string_value
        elif number_value is not None:
            request.number_value = number_value
        elif boolean_value is not None:
            request.boolean_value = boolean_value
        else:
            raise ValueError("Must provide one of: string_value, number_value, or boolean_value")

        response = stub.CreateUserAttributeValue(request)
        print(f"Created user attribute value for user {user_id}: {response.user_attribute_value.user_attribute_value_id}")
        return response.user_attribute_value.user_attribute_value_id


def archive_user_attribute_keys(user_attribute_key_ids: list[str]):
    """Archive user attribute keys."""
    with use_sift_channel(channel_config) as channel:
        stub = UserAttributesServiceStub(channel)
        request = ArchiveUserAttributeKeysRequest(user_attribute_key_ids=user_attribute_key_ids)
        stub.ArchiveUserAttributeKeys(request)
        print(f"Archived {len(user_attribute_key_ids)} user attribute key(s)")


def archive_user_attribute_values(user_attribute_value_ids: list[str]):
    """Archive user attribute values."""
    with use_sift_channel(channel_config) as channel:
        stub = UserAttributesServiceStub(channel)
        request = ArchiveUserAttributeValuesRequest(user_attribute_value_ids=user_attribute_value_ids)
        stub.ArchiveUserAttributeValues(request)
        print(f"Archived {len(user_attribute_value_ids)} user attribute value(s)")


# ============================================================================
# RESOURCE ATTRIBUTES
# ============================================================================

def create_resource_attribute_key(
    display_name: str,
    description: str,
    key_type: ResourceAttributeKeyType,
    organization_id: Optional[str] = None,
    initial_enum_values: Optional[list] = None,
) -> str:
    """Create a resource attribute key.

    Args:
        display_name: Display name of the key
        description: Description of the key
        key_type: Type of key (RESOURCE_ATTRIBUTE_KEY_TYPE_ENUM, BOOLEAN, or NUMBER)
        organization_id: Optional organization ID
        initial_enum_values: Optional list of enum values to create with the key (for ENUM type)

    Returns:
        The created resource_attribute_key_id
    """
    with use_sift_channel(channel_config) as channel:
        stub = ResourceAttributeServiceStub(channel)
        key = ResourceAttributeKey(
            display_name=display_name,
            description=description,
            type=key_type,
        )
        request = CreateResourceAttributeKeyRequest(resource_attribute_key=key)
        if initial_enum_values:
            request.initial_enum_values.extend(initial_enum_values)

        response = stub.CreateResourceAttributeKey(request)
        print(f"Created resource attribute key: {response.resource_attribute_key.display_name} (ID: {response.resource_attribute_key.resource_attribute_key_id})")
        return response.resource_attribute_key.resource_attribute_key_id


def create_resource_attribute_enum_value(
    resource_attribute_key_id: str,
    display_name: str,
    description: Optional[str] = None,
) -> str:
    """Create a resource attribute enum value.

    Args:
        resource_attribute_key_id: The ID of the resource attribute key
        display_name: Display name of the enum value
        description: Optional description

    Returns:
        The created resource_attribute_enum_value_id
    """
    with use_sift_channel(channel_config) as channel:
        stub = ResourceAttributeServiceStub(channel)
        enum_value = ResourceAttributeEnumValue(
            resource_attribute_key_id=resource_attribute_key_id,
            display_name=display_name,
            description=description or "",
        )
        request = CreateResourceAttributeEnumValueRequest(resource_attribute_enum_value=enum_value)
        response = stub.CreateResourceAttributeEnumValue(request)
        print(f"Created enum value: {response.resource_attribute_enum_value.display_name} (ID: {response.resource_attribute_enum_value.resource_attribute_enum_value_id})")
        return response.resource_attribute_enum_value.resource_attribute_enum_value_id


def create_resource_attribute(
    resource_attribute_key_id: str,
    entity_id: str,
    entity_type: ResourceAttributeEntityType,
    resource_attribute_enum_value_id: Optional[str] = None,
    boolean_value: Optional[bool] = None,
    number_value: Optional[float] = None,
) -> str:
    """Create a resource attribute (assigns attribute to an entity).

    Args:
        resource_attribute_key_id: The ID of the resource attribute key
        entity_id: The ID of the entity (asset or channel)
        entity_type: Type of entity (RESOURCE_ATTRIBUTE_ENTITY_TYPE_ASSET or CHANNEL)
        resource_attribute_enum_value_id: Enum value ID (if key type is ENUM)
        boolean_value: Boolean value (if key type is BOOLEAN)
        number_value: Number value (if key type is NUMBER)

    Returns:
        The created resource_attribute_id
    """
    with use_sift_channel(channel_config) as channel:
        stub = ResourceAttributeServiceStub(channel)
        entity = ResourceAttributeEntityIdentifier(
            entity_id=entity_id,
            entity_type=entity_type,
        )
        request = CreateResourceAttributeRequest()
        request.resource_attribute.resource_attribute_key_id = resource_attribute_key_id
        request.resource_attribute.entity.CopyFrom(entity)

        if resource_attribute_enum_value_id is not None:
            request.resource_attribute.resource_attribute_enum_value_id = resource_attribute_enum_value_id
        elif boolean_value is not None:
            request.resource_attribute.boolean_value = boolean_value
        elif number_value is not None:
            request.resource_attribute.number_value = number_value
        else:
            raise ValueError("Must provide one of: resource_attribute_enum_value_id, boolean_value, or number_value")

        response = stub.CreateResourceAttribute(request)
        print(f"Created resource attribute for entity {entity_id}: {response.resource_attribute.resource_attribute_id}")
        return response.resource_attribute.resource_attribute_id


def update_resource_attribute_key(
    resource_attribute_key_id: str,
    display_name: Optional[str] = None,
    description: Optional[str] = None,
):
    """Update a resource attribute key.

    Args:
        resource_attribute_key_id: The ID of the key to update
        display_name: New display name (optional)
        description: New description (optional)
    """
    with use_sift_channel(channel_config) as channel:
        stub = ResourceAttributeServiceStub(channel)
        # First get the current key
        get_request = GetResourceAttributeKeyRequest(resource_attribute_key_id=resource_attribute_key_id)
        current_key = stub.GetResourceAttributeKey(get_request).resource_attribute_key

        # Update fields
        if display_name is not None:
            current_key.display_name = display_name
        if description is not None:
            current_key.description = description

        # Create update mask
        update_mask = field_mask_pb2.FieldMask()
        if display_name is not None:
            update_mask.paths.append("display_name")
        if description is not None:
            update_mask.paths.append("description")

        request = UpdateResourceAttributeKeyRequest(
            resource_attribute_key=current_key,
            update_mask=update_mask,
        )
        response = stub.UpdateResourceAttributeKey(request)
        print(f"Updated resource attribute key: {response.resource_attribute_key.display_name}")


def list_resource_attribute_keys(organization_id: Optional[str] = None, include_archived: bool = False):
    """List all resource attribute keys."""
    with use_sift_channel(channel_config) as channel:
        stub = ResourceAttributeServiceStub(channel)
        request = ListResourceAttributeKeysRequest(
            organization_id=organization_id,
            include_archived=include_archived,
        )
        response = stub.ListResourceAttributeKeys(request)
        print(f"Found {len(response.resource_attribute_keys)} resource attribute keys:")
        for key in response.resource_attribute_keys:
            print(f"  - {key.display_name} (ID: {key.resource_attribute_key_id}, Type: {key.type})")
        return response.resource_attribute_keys


def archive_resource_attribute_key(resource_attribute_key_id: str):
    """Archive a resource attribute key."""
    with use_sift_channel(channel_config) as channel:
        stub = ResourceAttributeServiceStub(channel)
        request = ArchiveResourceAttributeKeyRequest(resource_attribute_key_id=resource_attribute_key_id)
        stub.ArchiveResourceAttributeKey(request)
        print(f"Archived resource attribute key: {resource_attribute_key_id}")


def archive_resource_attribute(resource_attribute_id: str):
    """Archive a resource attribute."""
    with use_sift_channel(channel_config) as channel:
        stub = ResourceAttributeServiceStub(channel)
        request = ArchiveResourceAttributeRequest(resource_attribute_id=resource_attribute_id)
        stub.ArchiveResourceAttribute(request)
        print(f"Archived resource attribute: {resource_attribute_id}")


# ============================================================================
# ACCESS POLICIES
# ============================================================================

def create_policy(
    name: str,
    cedar_policy: str,
    description: Optional[str] = None,
    version_notes: Optional[str] = None,
) -> str:
    """Create an access policy.

    Args:
        name: Name of the policy
        cedar_policy: Cedar policy string (see https://docs.cedarpolicy.com/policies/syntax-policy.html)
        description: Optional description
        version_notes: Optional version notes

    Returns:
        The created policy_id
    """
    with use_sift_channel(channel_config) as channel:
        stub = PolicyServiceStub(channel)
        config = PolicyConfiguration(cedar_policy=cedar_policy)
        request = CreatePolicyRequest(
            name=name,
            description=description or "",
            configuration=config,
            version_notes=version_notes,
        )
        response = stub.CreatePolicy(request)
        print(f"Created policy: {response.policy.name} (ID: {response.policy.policy_id})")
        return response.policy.policy_id


def list_policies(include_archived: bool = False):
    """List all policies."""
    with use_sift_channel(channel_config) as channel:
        stub = PolicyServiceStub(channel)
        request = ListPoliciesRequest(include_archived=include_archived)
        response = stub.ListPolicies(request)
        print(f"Found {len(response.policies)} policies:")
        for policy in response.policies:
            print(f"  - {policy.name} (ID: {policy.policy_id}, Archived: {policy.is_archived})")
        return response.policies


def update_policy(
    policy_id: str,
    name: Optional[str] = None,
    description: Optional[str] = None,
    cedar_policy: Optional[str] = None,
    version_notes: Optional[str] = None,
):
    """Update a policy.

    Args:
        policy_id: The ID of the policy to update
        name: New name (optional)
        description: New description (optional)
        cedar_policy: New Cedar policy string (optional)
        version_notes: Optional version notes for the update
    """
    with use_sift_channel(channel_config) as channel:
        stub = PolicyServiceStub(channel)
        # First get the current policy
        get_request = GetPolicyRequest(policy_id=policy_id)
        current_policy = stub.GetPolicy(get_request).policy

        # Update fields
        update_mask = field_mask_pb2.FieldMask()
        if name is not None:
            current_policy.name = name
            update_mask.paths.append("name")
        if description is not None:
            current_policy.description = description
            update_mask.paths.append("description")
        if cedar_policy is not None:
            current_policy.configuration.cedar_policy = cedar_policy
            update_mask.paths.append("configuration")

        request = UpdatePolicyRequest(
            policy=current_policy,
            update_mask=update_mask,
            version_notes=version_notes,
        )
        response = stub.UpdatePolicy(request)
        print(f"Updated policy: {response.policy.name}")


def archive_policy(policy_id: str):
    """Archive a policy."""
    with use_sift_channel(channel_config) as channel:
        stub = PolicyServiceStub(channel)
        request = ArchivePolicyRequest(policy_id=policy_id)
        response = stub.ArchivePolicy(request)
        print(f"Archived policy: {response.policy.name} (ID: {response.policy.policy_id})")


# ============================================================================
# EXAMPLE USAGE
# ============================================================================

if __name__ == "__main__":
    print("ABAC Resources Management Script")
    print("=" * 50)
    print("\nThis script provides functions to manage:")
    print("1. User Attributes (keys and values)")
    print("2. Resource Attributes (keys, enum values, and attributes)")
    print("3. Access Policies")
    print("\nExample usage:")
    print("\n# Create a user attribute key")
    print('key_id = create_user_attribute_key(')
    print('    name="department",')
    print('    description="User department",')
    print('    value_type=UserAttributeValueType.USER_ATTRIBUTE_VALUE_TYPE_STRING')
    print(')')
    print("\n# Create a user attribute value")
    print('create_user_attribute_value(')
    print('    user_attribute_key_id=key_id,')
    print('    user_id="my_user",')
    print('    string_value="Engineering"')
    print(')')
    print("\n# Create a resource attribute key with enum type")
    print('resource_key_id = create_resource_attribute_key(')
    print('    display_name="environment",')
    print('    description="Deployment environment",')
    print('    key_type=ResourceAttributeKeyType.RESOURCE_ATTRIBUTE_KEY_TYPE_ENUM')
    print(')')
    print("\n# Create an enum value")
    print('enum_value_id = create_resource_attribute_enum_value(')
    print('    resource_attribute_key_id=resource_key_id,')
    print('    display_name="production"')
    print(')')
    print("\n# Create a resource attribute")
    print('create_resource_attribute(')
    print('    resource_attribute_key_id=resource_key_id,')
    print('    entity_id="asset_id",')
    print('    entity_type=ResourceAttributeEntityType.RESOURCE_ATTRIBUTE_ENTITY_TYPE_ASSET,')
    print('    resource_attribute_enum_value_id=enum_value_id')
    print(')')
    print("\n# Create a policy")
    print('policy_id = create_policy(')
    print('    name="Example Policy",')
    print('    cedar_policy=\'permit(principal, action, resource) when { principal.department == "Engineering" };\'')
    print(')')
    print("\n# List all resources")
    print("list_user_attribute_keys()")
    print("list_resource_attribute_keys()")
    print("list_policies()")
    print("\n# Archive resources")
    print('archive_user_attribute_keys([key_id])')
    print('archive_resource_attribute_key(resource_key_id)')
    print('archive_policy(policy_id)')

