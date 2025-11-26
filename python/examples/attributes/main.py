"""
Script to create, update, and delete ABAC resources:
- User Attributes (keys and values)
- Resource Attributes (keys, enum values, and attributes)
- Access Policies

This script can be copied and pasted into a Jupyter notebook or run as a standalone script.

⚠️ IMPORTANT: For multiple operations, use ABACClient context manager to avoid connection errors.

TROUBLESHOOTING "Stream removed" errors:
1. Use ABACClient context manager for multiple operations
2. Check that your BASE_URI is correct (should be hostname:port without https://)
3. Ensure SSL is properly configured (use_ssl: True)
4. If SSL errors occur, try setting cert_via_openssl: True in channel_config
5. Verify your API key is valid and has proper permissions
"""

import logging
import os
import uuid
from typing import Optional

import grpc
from dotenv import load_dotenv
from google.protobuf import field_mask_pb2

# Configure logging
logger = logging.getLogger(__name__)
logging.basicConfig(
    level=logging.INFO,
    format='%(asctime)s - %(name)s - %(levelname)s - %(message)s'
)

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
    ResourceAttribute,
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

# Configuration from environment variables
BASE_URI = os.getenv("SIFT_BASE_URI")
API_KEY = os.getenv("SIFT_API_KEY")

if not BASE_URI:
    raise ValueError("SIFT_BASE_URI environment variable must be set")
if not API_KEY:
    raise ValueError("SIFT_API_KEY environment variable must be set")

channel_config: SiftChannelConfig = {
    "apikey": API_KEY,
    "uri": BASE_URI,
    "use_ssl": True,  # Ensure SSL is enabled for production
    # Try enabling cert_via_openssl if you get SSL errors
    # "cert_via_openssl": True,
}


# ============================================================================
# UTILITY FUNCTIONS
# ============================================================================

def handle_grpc_error(e: grpc.RpcError, operation: str) -> None:
    """Handle gRPC errors with detailed diagnostics.
    
    Args:
        e: The gRPC error exception
        operation: Description of the operation that failed
    """
    status_code = e.code()
    details = e.details()
    
    logger.error(f"Error during {operation}")
    logger.error(f"   Status Code: {status_code}")
    logger.error(f"   Details: {details}")
    
    if status_code == grpc.StatusCode.INTERNAL:
        logger.error("⚠️  INTERNAL SERVER ERROR (500)")
        logger.error("   ✅ The request successfully reached the server")
        logger.error("   ❌ The server encountered an error while processing the request")
        logger.error("   📋 Use the error ID in the details message to check server logs")
    elif status_code == grpc.StatusCode.INVALID_ARGUMENT:
        logger.error("⚠️  INVALID ARGUMENT (400)")
        logger.error("   ❌ The request parameters are invalid")
        logger.error("   💡 Check that all required fields are provided and have correct types")
    elif status_code == grpc.StatusCode.UNAUTHENTICATED:
        logger.error("⚠️  UNAUTHENTICATED (401)")
        logger.error("   ❌ Your API key is invalid or missing")
    elif status_code == grpc.StatusCode.PERMISSION_DENIED:
        logger.error("⚠️  PERMISSION DENIED (403)")
        logger.error("   ❌ Your API key does not have permission for this operation")
    elif status_code == grpc.StatusCode.UNAVAILABLE:
        logger.error("⚠️  SERVICE UNAVAILABLE (503)")
        logger.error("   ❌ The service is temporarily unavailable")
        logger.error("   💡 Try again in a few moments")


def generate_uuid() -> str:
    """Generate a new UUID v4 string.
    
    Returns:
        A UUID string (e.g., "550e8400-e29b-41d4-a716-446655440000")
    """
    return str(uuid.uuid4())


def generate_uuid_short() -> str:
    """Generate a short UUID (without hyphens).
    
    Returns:
        A UUID string without hyphens (e.g., "550e8400e29b41d4a716446655440000")
    """
    return str(uuid.uuid4()).replace("-", "")


class ABACClient:
    """Context manager for reusing a gRPC channel across multiple ABAC operations.
    
    This prevents connection errors when making multiple RPC calls.
    
    Example:
        with ABACClient(channel_config) as client:
            key_id = create_user_attribute_key("department", "...", ..., client=client)
            create_user_attribute_value(key_id, "user_id", string_value="Engineering", client=client)
    """
    
    def __init__(self, config: SiftChannelConfig):
        self.config = config
        self.channel = None
        self.user_attributes_stub = None
        self.resource_attribute_stub = None
        self.policy_stub = None
    
    def __enter__(self):
        # use_sift_channel returns a grpc.Channel which supports context manager protocol
        self.channel = use_sift_channel(self.config)
        self.user_attributes_stub = UserAttributesServiceStub(self.channel)
        self.resource_attribute_stub = ResourceAttributeServiceStub(self.channel)
        self.policy_stub = PolicyServiceStub(self.channel)
        return self
    
    def __exit__(self, exc_type, exc_val, exc_tb):
        if self.channel:
            # gRPC channels should be closed explicitly
            self.channel.close()
        return False


# ============================================================================
# USER ATTRIBUTES
# ============================================================================

def create_user_attribute_key(
    name: str,
    description: str,
    value_type: UserAttributeValueType,
    organization_id: Optional[str] = None,
    client: Optional[ABACClient] = None,
) -> str:
    """Create a user attribute key.

    Args:
        name: Name of the attribute key (e.g., "department")
        description: Description of the attribute key
        value_type: Type of value (USER_ATTRIBUTE_VALUE_TYPE_STRING, BOOLEAN, or NUMBER)
        organization_id: Optional organization ID
        client: Optional ABACClient instance for reusing channel. If None, creates a new channel.

    Returns:
        The created user_attribute_key_id
    """
    if client:
        stub = client.user_attributes_stub
        request = CreateUserAttributeKeyRequest(
            name=name,
            description=description,
            type=value_type,
            organization_id=organization_id,
        )
        try:
            response = stub.CreateUserAttributeKey(request)
            logger.info(f"Created user attribute key: {response.user_attribute_key.name} (ID: {response.user_attribute_key.user_attribute_key_id})")
            return response.user_attribute_key.user_attribute_key_id
        except grpc.RpcError as e:
            handle_grpc_error(e, "create_user_attribute_key")
            raise
    else:
        # Create a new channel for this call
        with use_sift_channel(channel_config) as channel:
            stub = UserAttributesServiceStub(channel)
            request = CreateUserAttributeKeyRequest(
                name=name,
                description=description,
                type=value_type,
                organization_id=organization_id,
            )
            try:
                response = stub.CreateUserAttributeKey(request)
                logger.info(f"Created user attribute key: {response.user_attribute_key.name} (ID: {response.user_attribute_key.user_attribute_key_id})")
                return response.user_attribute_key.user_attribute_key_id
            except grpc.RpcError as e:
                handle_grpc_error(e, "create_user_attribute_key")
                raise


def list_user_attribute_keys(
    organization_id: Optional[str] = None,
    include_archived: bool = False,
    client: Optional[ABACClient] = None,
):
    """List all user attribute keys."""
    try:
        if client:
            stub = client.user_attributes_stub
            request = ListUserAttributeKeysRequest(
                organization_id=organization_id,
                include_archived=include_archived,
            )
            response = stub.ListUserAttributeKeys(request)
        else:
            with use_sift_channel(channel_config) as channel:
                stub = UserAttributesServiceStub(channel)
                request = ListUserAttributeKeysRequest(
                    organization_id=organization_id,
                    include_archived=include_archived,
                )
                response = stub.ListUserAttributeKeys(request)
        
        logger.info(f"Found {len(response.user_attribute_keys)} user attribute keys:")
        for key in response.user_attribute_keys:
            logger.info(f"  - {key.name} (ID: {key.user_attribute_key_id}, Type: {key.type})")
        return response.user_attribute_keys
    except grpc.RpcError as e:
        handle_grpc_error(e, "list_user_attribute_keys")
        raise


def create_user_attribute_value(
    user_attribute_key_id: str,
    user_id: str,
    string_value: Optional[str] = None,
    number_value: Optional[float] = None,
    boolean_value: Optional[bool] = None,
    organization_id: Optional[str] = None,
    client: Optional[ABACClient] = None,
) -> str:
    """Create a user attribute value.

    Args:
        user_attribute_key_id: The ID of the user attribute key
        user_id: The user ID to assign the attribute to
        string_value: String value (if key type is STRING)
        number_value: Number value (if key type is NUMBER)
        boolean_value: Boolean value (if key type is BOOLEAN)
        organization_id: Optional organization ID
        client: Optional ABACClient instance for reusing channel. If None, creates a new channel.

    Returns:
        The created user_attribute_value_id
    """
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

    try:
        if client:
            stub = client.user_attributes_stub
            response = stub.CreateUserAttributeValue(request)
        else:
            with use_sift_channel(channel_config) as channel:
                stub = UserAttributesServiceStub(channel)
                response = stub.CreateUserAttributeValue(request)
        
        logger.info(f"Created user attribute value for user {user_id}: {response.user_attribute_value.user_attribute_value_id}")
        return response.user_attribute_value.user_attribute_value_id
    except grpc.RpcError as e:
        handle_grpc_error(e, "create_user_attribute_value")
        raise


def archive_user_attribute_keys(
    user_attribute_key_ids: list[str],
    client: Optional[ABACClient] = None,
):
    """Archive user attribute keys."""
    try:
        request = ArchiveUserAttributeKeysRequest(user_attribute_key_ids=user_attribute_key_ids)
        if client:
            stub = client.user_attributes_stub
            stub.ArchiveUserAttributeKeys(request)
        else:
            with use_sift_channel(channel_config) as channel:
                stub = UserAttributesServiceStub(channel)
                stub.ArchiveUserAttributeKeys(request)
        logger.info(f"Archived {len(user_attribute_key_ids)} user attribute key(s)")
    except grpc.RpcError as e:
        handle_grpc_error(e, "archive_user_attribute_keys")
        raise


def archive_user_attribute_values(
    user_attribute_value_ids: list[str],
    client: Optional[ABACClient] = None,
):
    """Archive user attribute values."""
    try:
        request = ArchiveUserAttributeValuesRequest(user_attribute_value_ids=user_attribute_value_ids)
        if client:
            stub = client.user_attributes_stub
            stub.ArchiveUserAttributeValues(request)
        else:
            with use_sift_channel(channel_config) as channel:
                stub = UserAttributesServiceStub(channel)
                stub.ArchiveUserAttributeValues(request)
        logger.info(f"Archived {len(user_attribute_value_ids)} user attribute value(s)")
    except grpc.RpcError as e:
        handle_grpc_error(e, "archive_user_attribute_values")
        raise


# ============================================================================
# RESOURCE ATTRIBUTES
# ============================================================================

def create_resource_attribute_key(
    display_name: str,
    description: str,
    key_type: ResourceAttributeKeyType,
    organization_id: Optional[str] = None,
    initial_enum_values: Optional[list] = None,
    client: Optional[ABACClient] = None,
) -> str:
    """Create a resource attribute key.

    Args:
        display_name: Display name of the key
        description: Description of the key
        key_type: Type of key (RESOURCE_ATTRIBUTE_KEY_TYPE_ENUM, BOOLEAN, or NUMBER)
        organization_id: Optional organization ID
        initial_enum_values: Optional list of enum values to create with the key (for ENUM type)
        client: Optional ABACClient instance for reusing channel. If None, creates a new channel.

    Returns:
        The created resource_attribute_key_id
    """
    # Convert enum to int value
    if isinstance(key_type, int):
        enum_int_value = key_type
    else:
        try:
            enum_int_value = int(key_type)
        except (ValueError, TypeError):
            raise ValueError(f"Invalid key_type: {key_type}. Must be an integer or ResourceAttributeKeyType enum constant.")
    
    # Validate the enum value
    if enum_int_value == 0:
        raise ValueError(
            f"Invalid key_type: {key_type} (value: {enum_int_value}). "
            f"RESOURCE_ATTRIBUTE_KEY_TYPE_UNSPECIFIED (0) is not allowed. "
            f"Use one of:\n"
            f"  - ResourceAttributeKeyType.RESOURCE_ATTRIBUTE_KEY_TYPE_ENUM (1)\n"
            f"  - ResourceAttributeKeyType.RESOURCE_ATTRIBUTE_KEY_TYPE_BOOLEAN (2)\n"
            f"  - ResourceAttributeKeyType.RESOURCE_ATTRIBUTE_KEY_TYPE_NUMBER (3)"
        )
    
    # Create the request with individual fields (matching the updated proto)
    request = CreateResourceAttributeKeyRequest(
        display_name=display_name,
        description=description,
        type=enum_int_value,
    )
    # Set organization_id only if provided (it's optional)
    if organization_id:
        request.organization_id = organization_id
    
    # Verify the type is set correctly
    if int(request.type) != enum_int_value:
        raise ValueError(
            f"Type mismatch: Expected {enum_int_value}, but request has {int(request.type)}"
        )
    
    if initial_enum_values:
        request.initial_enum_values.extend(initial_enum_values)

    try:
        if client:
            stub = client.resource_attribute_stub
            response = stub.CreateResourceAttributeKey(request)
        else:
            with use_sift_channel(channel_config) as channel:
                stub = ResourceAttributeServiceStub(channel)
                response = stub.CreateResourceAttributeKey(request)
        
        logger.info(f"Created resource attribute key: {response.resource_attribute_key.display_name} (ID: {response.resource_attribute_key.resource_attribute_key_id})")
        return response.resource_attribute_key.resource_attribute_key_id
    except grpc.RpcError as e:
        handle_grpc_error(e, "create_resource_attribute_key")
        raise


def create_resource_attribute_enum_value(
    resource_attribute_key_id: str,
    display_name: str,
    description: Optional[str] = None,
    client: Optional[ABACClient] = None,
) -> str:
    """Create a resource attribute enum value.

    Args:
        resource_attribute_key_id: The ID of the resource attribute key
        display_name: Display name of the enum value
        description: Optional description
        client: Optional ABACClient instance for reusing channel. If None, creates a new channel.

    Returns:
        The created resource_attribute_enum_value_id
    """
    enum_value = ResourceAttributeEnumValue(
        resource_attribute_key_id=resource_attribute_key_id,
        display_name=display_name,
        description=description or "",
    )
    request = CreateResourceAttributeEnumValueRequest(resource_attribute_enum_value=enum_value)
    
    try:
        if client:
            stub = client.resource_attribute_stub
            response = stub.CreateResourceAttributeEnumValue(request)
        else:
            with use_sift_channel(channel_config) as channel:
                stub = ResourceAttributeServiceStub(channel)
                response = stub.CreateResourceAttributeEnumValue(request)
        
        logger.info(f"Created enum value: {response.resource_attribute_enum_value.display_name} (ID: {response.resource_attribute_enum_value.resource_attribute_enum_value_id})")
        return response.resource_attribute_enum_value.resource_attribute_enum_value_id
    except grpc.RpcError as e:
        handle_grpc_error(e, "create_resource_attribute_enum_value")
        raise


def create_resource_attribute(
    resource_attribute_key_id: str,
    entity_id: str,
    entity_type: ResourceAttributeEntityType,
    resource_attribute_enum_value_id: Optional[str] = None,
    boolean_value: Optional[bool] = None,
    number_value: Optional[float] = None,
    client: Optional[ABACClient] = None,
) -> str:
    """Create a resource attribute (assigns attribute to an entity).

    Args:
        resource_attribute_key_id: The ID of the resource attribute key
        entity_id: The ID of the entity (asset or channel)
        entity_type: Type of entity (RESOURCE_ATTRIBUTE_ENTITY_TYPE_ASSET or CHANNEL)
        resource_attribute_enum_value_id: Enum value ID (if key type is ENUM)
        boolean_value: Boolean value (if key type is BOOLEAN)
        number_value: Number value (if key type is NUMBER)
        client: Optional ABACClient instance for reusing channel. If None, creates a new channel.

    Returns:
        The created resource_attribute_id
    """
    # Create the ResourceAttribute message with only the fields needed for creation
    # Note: resource_attribute_id, organization_id, created_date, created_by_user_id
    # are server-generated (OUTPUT_ONLY) and should NOT be set in the request.
    # The server will populate these from the request context.
    resource_attr = ResourceAttribute()
    resource_attr.resource_attribute_key_id = resource_attribute_key_id
    resource_attr.entity.entity_id = entity_id
    resource_attr.entity.entity_type = entity_type

    if resource_attribute_enum_value_id is not None:
        resource_attr.resource_attribute_enum_value_id = resource_attribute_enum_value_id
    elif boolean_value is not None:
        resource_attr.boolean_value = boolean_value
    elif number_value is not None:
        resource_attr.number_value = number_value
    else:
        raise ValueError("Must provide one of: resource_attribute_enum_value_id, boolean_value, or number_value")

    # Create the request object
    request = CreateResourceAttributeRequest(resource_attribute=resource_attr)

    try:
        if client:
            stub = client.resource_attribute_stub
            response = stub.CreateResourceAttribute(request)
        else:
            with use_sift_channel(channel_config) as channel:
                stub = ResourceAttributeServiceStub(channel)
                response = stub.CreateResourceAttribute(request)
        
        logger.info(f"Created resource attribute for entity {entity_id}: {response.resource_attribute.resource_attribute_id}")
        return response.resource_attribute.resource_attribute_id
    except grpc.RpcError as e:
        handle_grpc_error(e, "create_resource_attribute")
        raise


def update_resource_attribute_key(
    resource_attribute_key_id: str,
    display_name: Optional[str] = None,
    description: Optional[str] = None,
    client: Optional[ABACClient] = None,
):
    """Update a resource attribute key.

    Args:
        resource_attribute_key_id: The ID of the key to update
        display_name: New display name (optional)
        description: New description (optional)
        client: Optional ABACClient instance for reusing channel. If None, creates a new channel.
    """
    try:
        get_request = GetResourceAttributeKeyRequest(resource_attribute_key_id=resource_attribute_key_id)
        
        if client:
            stub = client.resource_attribute_stub
            current_key = stub.GetResourceAttributeKey(get_request).resource_attribute_key
        else:
            with use_sift_channel(channel_config) as channel:
                stub = ResourceAttributeServiceStub(channel)
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
        
        if client:
            stub = client.resource_attribute_stub
            response = stub.UpdateResourceAttributeKey(request)
        else:
            with use_sift_channel(channel_config) as channel:
                stub = ResourceAttributeServiceStub(channel)
                response = stub.UpdateResourceAttributeKey(request)
        
        logger.info(f"Updated resource attribute key: {response.resource_attribute_key.display_name}")
    except grpc.RpcError as e:
        handle_grpc_error(e, "update_resource_attribute_key")
        raise


def list_resource_attribute_keys(
    organization_id: Optional[str] = None,
    include_archived: bool = False,
    client: Optional[ABACClient] = None,
):
    """List all resource attribute keys."""
    try:
        request = ListResourceAttributeKeysRequest(
            organization_id=organization_id,
            include_archived=include_archived,
        )
        
        if client:
            stub = client.resource_attribute_stub
            response = stub.ListResourceAttributeKeys(request)
        else:
            with use_sift_channel(channel_config) as channel:
                stub = ResourceAttributeServiceStub(channel)
                response = stub.ListResourceAttributeKeys(request)
        
        logger.info(f"Found {len(response.resource_attribute_keys)} resource attribute keys:")
        for key in response.resource_attribute_keys:
            logger.info(f"  - {key.display_name} (ID: {key.resource_attribute_key_id}, Type: {key.type})")
        return response.resource_attribute_keys
    except grpc.RpcError as e:
        handle_grpc_error(e, "list_resource_attribute_keys")
        raise


def archive_resource_attribute_key(
    resource_attribute_key_id: str,
    client: Optional[ABACClient] = None,
):
    """Archive a resource attribute key."""
    try:
        request = ArchiveResourceAttributeKeyRequest(resource_attribute_key_id=resource_attribute_key_id)
        if client:
            stub = client.resource_attribute_stub
            stub.ArchiveResourceAttributeKey(request)
        else:
            with use_sift_channel(channel_config) as channel:
                stub = ResourceAttributeServiceStub(channel)
                stub.ArchiveResourceAttributeKey(request)
        logger.info(f"Archived resource attribute key: {resource_attribute_key_id}")
    except grpc.RpcError as e:
        handle_grpc_error(e, "archive_resource_attribute_key")
        raise


def archive_resource_attribute(
    resource_attribute_id: str,
    client: Optional[ABACClient] = None,
):
    """Archive a resource attribute."""
    try:
        request = ArchiveResourceAttributeRequest(resource_attribute_id=resource_attribute_id)
        if client:
            stub = client.resource_attribute_stub
            stub.ArchiveResourceAttribute(request)
        else:
            with use_sift_channel(channel_config) as channel:
                stub = ResourceAttributeServiceStub(channel)
                stub.ArchiveResourceAttribute(request)
        logger.info(f"Archived resource attribute: {resource_attribute_id}")
    except grpc.RpcError as e:
        handle_grpc_error(e, "archive_resource_attribute")
        raise


# ============================================================================
# ACCESS POLICIES
# ============================================================================

def create_policy(
    name: str,
    cedar_policy: str,
    description: Optional[str] = None,
    version_notes: Optional[str] = None,
    client: Optional[ABACClient] = None,
) -> str:
    """Create an access policy.

    Args:
        name: Name of the policy
        cedar_policy: Cedar policy string (see https://docs.cedarpolicy.com/policies/syntax-policy.html)
        description: Optional description
        version_notes: Optional version notes
        client: Optional ABACClient instance for reusing channel. If None, creates a new channel.

    Returns:
        The created policy_id
    """
    config = PolicyConfiguration(cedar_policy=cedar_policy)
    request = CreatePolicyRequest(
        name=name,
        description=description or "",
        configuration=config,
        version_notes=version_notes,
    )
    
    try:
        if client:
            stub = client.policy_stub
            response = stub.CreatePolicy(request)
        else:
            with use_sift_channel(channel_config) as channel:
                stub = PolicyServiceStub(channel)
                response = stub.CreatePolicy(request)
        
        logger.info(f"Created policy: {response.policy.name} (ID: {response.policy.policy_id})")
        return response.policy.policy_id
    except grpc.RpcError as e:
        handle_grpc_error(e, "create_policy")
        raise


def create_policies(
    policies: list[dict],
    client: Optional[ABACClient] = None,
) -> dict[str, str]:
    """Create multiple policies from a list of policy definitions.
    
    Args:
        policies: List of policy dictionaries. Each dictionary should contain:
            - 'name' (str, required): Name of the policy
            - 'cedar_policy' (str, required): Cedar policy string
            - 'description' (str, optional): Description of the policy
            - 'version_notes' (str, optional): Version notes
        client: Optional ABACClient instance for reusing channel. If None, creates a new channel.
    
    Returns:
        Dictionary mapping policy names to their created policy_ids
    
    Example:
        policies = [
            {
                'name': 'Engineering Policy',
                'cedar_policy': 'permit(principal, action, resource) when { principal.department == "Engineering" };',
                'description': 'Policy for engineering department'
            },
            {
                'name': 'Production Access',
                'cedar_policy': 'permit(principal, action, resource) when { resource.environment == "production" };',
                'description': 'Policy for production environment access'
            }
        ]
        policy_ids = create_policies(policies)
        # Returns: {'Engineering Policy': 'policy-id-1', 'Production Access': 'policy-id-2'}
    """
    policy_ids = {}
    
    # Use ABACClient if provided, otherwise create one for all policies
    if client:
        for policy_data in policies:
            name = policy_data.get('name')
            if not name:
                raise ValueError("Each policy must have a 'name' field")
            
            cedar_policy = policy_data.get('cedar_policy')
            if not cedar_policy:
                raise ValueError(f"Policy '{name}' must have a 'cedar_policy' field")
            
            policy_id = create_policy(
                name=name,
                cedar_policy=cedar_policy,
                description=policy_data.get('description'),
                version_notes=policy_data.get('version_notes'),
                client=client
            )
            policy_ids[name] = policy_id
    else:
        # Create a client for the batch operation
        with ABACClient(channel_config) as batch_client:
            for policy_data in policies:
                name = policy_data.get('name')
                if not name:
                    raise ValueError("Each policy must have a 'name' field")
                
                cedar_policy = policy_data.get('cedar_policy')
                if not cedar_policy:
                    raise ValueError(f"Policy '{name}' must have a 'cedar_policy' field")
                
                policy_id = create_policy(
                    name=name,
                    cedar_policy=cedar_policy,
                    description=policy_data.get('description'),
                    version_notes=policy_data.get('version_notes'),
                    client=batch_client
                )
                policy_ids[name] = policy_id
    
    logger.info(f"✅ Successfully created {len(policy_ids)} policies")
    return policy_ids


# ============================================================================
# EXAMPLE POLICY DEFINITIONS
# ============================================================================

def get_example_policies() -> list[dict]:
    """Get example policy definitions for common ABAC scenarios.
    
    Returns:
        List of policy dictionaries ready to use with create_policies()
    """
    return [
        # ========== CHANNEL VIEW ACCESS (Scenario 5: External Sharing) ==========
        {
            'name': 'Internal Users View Internal/Partner Channels',
            'cedar_policy': '''permit (
    principal in SiftApp::UserGroup::"engineering",
    action == SiftApp::Action::"view",
    resource is SiftApp::Channel
)
when
{
    principal has external_user &&
    principal.external_user == false &&
    resource has sensitivity_level &&
    (resource.sensitivity_level == "internal" || resource.sensitivity_level == "partner_sharable")
};''',
            'description': 'Internal users can view internal and partner-sharable channels'
        },
        {
            'name': 'External Users View Shared Channels',
            'cedar_policy': '''permit (
    principal,
    action == SiftApp::Action::"view",
    resource is SiftApp::Channel
)
when
{
    principal has external_user &&
    principal.external_user == true &&
    principal has organization &&
    resource has external_sharing &&
    resource.external_sharing.contains(principal.organization)
};''',
            'description': 'External users can ONLY view channels explicitly shared with their organization'
        },
        {
            'name': 'Deny External Users Confidential Channels',
            'cedar_policy': '''forbid (
    principal,
    action,
    resource is SiftApp::Channel
)
when
{
    principal has external_user &&
    principal.external_user == true &&
    resource has sensitivity_level &&
    resource.sensitivity_level == "confidential"
};''',
            'description': 'Deny external users from accessing confidential channels'
        },
        {
            'name': 'Deny External Users Write Operations',
            'cedar_policy': '''forbid (
    principal,
    action in [SiftApp::Action::"edit", SiftApp::Action::"delete", SiftApp::Action::"update", SiftApp::Action::"ingest", SiftApp::Action::"update-metadata"],
    resource
)
when
{
    principal has external_user &&
    principal.external_user == true
};''',
            'description': 'External users can never write, edit, or modify data'
        },
        
        # ========== RUN WRITE PROTECTION (Scenario 1) ==========
        {
            'name': 'Deny Overwrite Protected Data Runs',
            'cedar_policy': '''forbid (
    principal,
    action in [SiftApp::Action::"edit", SiftApp::Action::"delete", SiftApp::Action::"update", SiftApp::Action::"ingest"],
    resource
)
when
{
    resource has protected_data_overwrite &&
    resource.protected_data_overwrite == true
};''',
            'description': 'Deny data overwrite on runs where protected_data_overwrite flag is true'
        },
        {
            'name': 'Deny Overwrite Ingested/Approved/Archived Runs',
            'cedar_policy': '''forbid (
    principal,
    action in [SiftApp::Action::"edit", SiftApp::Action::"delete", SiftApp::Action::"update", SiftApp::Action::"ingest"],
    resource
)
when
{
    resource has status &&
    (resource.status == "ingested" || 
     resource.status == "approved" || 
     resource.status == "archived")
};''',
            'description': 'Deny data overwrite on runs that have been ingested, approved, or archived'
        },
        {
            'name': 'Allow Editing Draft Runs',
            'cedar_policy': '''permit (
    principal in SiftApp::UserGroup::"engineering",
    action in [SiftApp::Action::"edit", SiftApp::Action::"update"],
    resource
)
when
{
    resource has status &&
    resource.status == "draft"
};''',
            'description': 'Allow editing runs in draft status'
        },
        {
            'name': 'Engineering Read All Runs',
            'cedar_policy': '''permit (
    principal in SiftApp::UserGroup::"engineering",
    action == SiftApp::Action::"view",
    resource is SiftApp::Run
);''',
            'description': 'Allow read access to all runs for engineering'
        },
        {
            'name': 'Engineering Ingest Runs',
            'cedar_policy': '''permit (
    principal in SiftApp::UserGroup::"engineering",
    action == SiftApp::Action::"ingest",
    resource is SiftApp::Run
);''',
            'description': 'Allow ingesting runs for engineering users (unless forbidden by other policies)'
        },
        
        # ========== TEAM-BASED CHANNEL OWNERSHIP (Scenario 3) ==========
        {
            'name': 'Team Members Update Team-Owned Channels',
            'cedar_policy': '''permit (
    principal,
    action == SiftApp::Action::"update-metadata",
    resource
)
when
{
    resource has owning_team &&
    principal has team_memberships &&
    principal.team_memberships.contains(resource.owning_team)
};''',
            'description': 'Only team members can update metadata of team-owned channels'
        },
        {
            'name': 'Deny Non-Team Members Update Team Channels',
            'cedar_policy': '''forbid (
    principal,
    action == SiftApp::Action::"update-metadata",
    resource
)
when
{
    resource has owning_team &&
    (!(principal has team_memberships) ||
     !principal.team_memberships.contains(resource.owning_team))
};''',
            'description': 'Deny metadata updates to team-owned channels if user is not a member'
        },
        {
            'name': 'Engineering Update Channels Without Team',
            'cedar_policy': '''permit (
    principal in SiftApp::UserGroup::"engineering",
    action == SiftApp::Action::"update-metadata",
    resource is SiftApp::Channel
)
when
{
    !(resource has owning_team)
};''',
            'description': 'Allow metadata updates to channels without an owning team for all engineering users'
        },
        
        # ========== MISSION RUN APPROVAL WORKFLOW (Scenario 4) ==========
        {
            'name': 'Users with Approval Authority Can Approve Runs',
            'cedar_policy': '''permit (
    principal,
    action == SiftApp::Action::"approve",
    resource is SiftApp::Run
)
when
{
    principal has can_approve_runs &&
    principal.can_approve_runs == true &&
    principal has approval_authority
};''',
            'description': 'Only users with approval authority can approve runs'
        },
        {
            'name': 'Deny Use Unapproved Runs in Analysis',
            'cedar_policy': '''forbid (
    principal,
    action == SiftApp::Action::"use-in-analysis",
    resource
)
when
{
    resource has approval_required &&
    resource.approval_required == true &&
    resource has approval_status &&
    resource.approval_status != "approved"
};''',
            'description': 'Deny use of runs in analysis if approval is required but not approved'
        },
        {
            'name': 'Engineering Can Use Approved Runs in Analysis',
            'cedar_policy': '''permit (
    principal in SiftApp::UserGroup::"engineering",
    action == SiftApp::Action::"use-in-analysis",
    resource is SiftApp::Run
)
when
{
    resource has approval_status &&
    resource.approval_status == "approved"
};''',
            'description': 'Allow use of approved runs in analysis'
        },
        {
            'name': 'Engineering Can Use Non-Approval-Required Runs',
            'cedar_policy': '''permit (
    principal in SiftApp::UserGroup::"engineering",
    action == SiftApp::Action::"use-in-analysis",
    resource is SiftApp::Run
)
when
{
    resource has approval_required &&
    resource.approval_required == false
};''',
            'description': 'Allow use of runs that don\'t require approval'
        },
    ]


def list_policies(
    include_archived: bool = False,
    client: Optional[ABACClient] = None,
):
    """List all policies."""
    try:
        request = ListPoliciesRequest(include_archived=include_archived)
        
        if client:
            stub = client.policy_stub
            response = stub.ListPolicies(request)
        else:
            with use_sift_channel(channel_config) as channel:
                stub = PolicyServiceStub(channel)
                response = stub.ListPolicies(request)
        
        logger.info(f"Found {len(response.policies)} policies:")
        for policy in response.policies:
            logger.info(f"  - {policy.name} (ID: {policy.policy_id}, Archived: {policy.is_archived})")
        return response.policies
    except grpc.RpcError as e:
        handle_grpc_error(e, "list_policies")
        raise


def update_policy(
    policy_id: str,
    name: Optional[str] = None,
    description: Optional[str] = None,
    cedar_policy: Optional[str] = None,
    version_notes: Optional[str] = None,
    client: Optional[ABACClient] = None,
):
    """Update a policy.

    Args:
        policy_id: The ID of the policy to update
        name: New name (optional)
        description: New description (optional)
        cedar_policy: New Cedar policy string (optional)
        version_notes: Optional version notes for the update
        client: Optional ABACClient instance for reusing channel. If None, creates a new channel.
    """
    try:
        get_request = GetPolicyRequest(policy_id=policy_id)
        
        if client:
            stub = client.policy_stub
            current_policy = stub.GetPolicy(get_request).policy
        else:
            with use_sift_channel(channel_config) as channel:
                stub = PolicyServiceStub(channel)
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
        
        if client:
            stub = client.policy_stub
            response = stub.UpdatePolicy(request)
        else:
            with use_sift_channel(channel_config) as channel:
                stub = PolicyServiceStub(channel)
                response = stub.UpdatePolicy(request)
        
        logger.info(f"Updated policy: {response.policy.name}")
    except grpc.RpcError as e:
        handle_grpc_error(e, "update_policy")
        raise


def archive_policy(
    policy_id: str,
    client: Optional[ABACClient] = None,
):
    """Archive a policy."""
    try:
        request = ArchivePolicyRequest(policy_id=policy_id)
        if client:
            stub = client.policy_stub
            response = stub.ArchivePolicy(request)
        else:
            with use_sift_channel(channel_config) as channel:
                stub = PolicyServiceStub(channel)
                response = stub.ArchivePolicy(request)
        logger.info(f"Archived policy: {response.policy.name} (ID: {response.policy.policy_id})")
    except grpc.RpcError as e:
        handle_grpc_error(e, "archive_policy")
        raise


# ============================================================================
# EXAMPLE USAGE
# ============================================================================

def example_create_all_policies():
    """Example: Create all 16 example policies at once.
    
    This demonstrates how to use create_policies() with the example policy definitions.
    
    Usage:
        policy_ids = example_create_all_policies()
        # Returns a dict mapping policy names to their IDs
    """
    # Get the example policies
    policies = get_example_policies()
    
    # Create all policies using ABACClient for efficient connection management
    with ABACClient(channel_config) as client:
        policy_ids = create_policies(policies, client=client)
    
    # Log the results
    logger.info("="*60)
    logger.info("Created Policies Summary:")
    logger.info("="*60)
    for name, policy_id in policy_ids.items():
        logger.info(f"  {name}: {policy_id}")
    
    return policy_ids


def example_create_custom_policies():
    """Example: Create custom policies from a list.
    
    This shows how to create your own policy definitions.
    
    Usage:
        policy_ids = example_create_custom_policies()
    """
    custom_policies = [
        {
            'name': 'My Custom Policy 1',
            'cedar_policy': 'permit(principal, action, resource) when { principal.department == "Engineering" };',
            'description': 'Custom policy for engineering department',
            'version_notes': 'Initial version'
        },
        {
            'name': 'My Custom Policy 2',
            'cedar_policy': 'permit(principal, action, resource) when { resource.environment == "production" };',
            'description': 'Custom policy for production access'
        }
    ]
    
    # Create the policies
    policy_ids = create_policies(custom_policies)
    
    return policy_ids


if __name__ == "__main__":
    logger.info("ABAC Resources Management Script")
    logger.info("=" * 50)
    logger.info("\nThis script provides functions to manage:")
    logger.info("1. User Attributes (keys and values)")
    logger.info("2. Resource Attributes (keys, enum values, and attributes)")
    logger.info("3. Access Policies")
    logger.info("\n⚠️  IMPORTANT: For multiple operations, use ABACClient to reuse the channel:")
    logger.info("\n# Example: Create all example policies")
    logger.info("from abac_resources.main import example_create_all_policies")
    logger.info("policy_ids = example_create_all_policies()")
    logger.info("\n# Or create custom policies")
    logger.info("from abac_resources.main import create_policies, get_example_policies")
    logger.info("policies = get_example_policies()  # Get all 16 example policies")
    logger.info("policy_ids = create_policies(policies)  # Create them all")
